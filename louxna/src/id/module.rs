use std::{
    borrow::Borrow,
    fmt,
    mem,
    ops::{
        Deref,
        Index,
        Range,
        RangeFrom,
        RangeFull,
        RangeInclusive,
        RangeTo,
        RangeToInclusive,
    },
    str::FromStr,
};

use mlua::{FromLua, IntoLua, Lua, Value};
use serde::{Deserialize, Serialize};

use crate::{Error, error::ErrorInner, id::package::PkgId};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ModSegment {
    content: str,
}

impl ModSegment {
    pub fn new(content: &str) -> Result<&Self, Error> {
        if !content
            .starts_with(|ch: char| ch.is_ascii_alphabetic() || ch == '_')
        {
            Err(ErrorInner::InvalidModSegmentStr {
                input: content.to_owned(),
                path: Default::default(),
            })?
        }
        if !content.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Err(ErrorInner::InvalidModSegmentStr {
                input: content.to_owned(),
                path: Default::default(),
            })?
        }
        unsafe { Ok(Self::new_unchecked(content)) }
    }

    pub fn new_boxed(content: Box<str>) -> Result<Box<Self>, Error> {
        ModSegment::new(&content)?;
        unsafe { Ok(Self::new_boxed_unchecked(content)) }
    }

    pub const unsafe fn new_unchecked(content: &str) -> &Self {
        unsafe { mem::transmute(content) }
    }

    pub const unsafe fn new_boxed_unchecked(content: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(content) }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Clone for Box<ModSegment> {
    fn clone(&self) -> Self {
        Self::from(&**self)
    }
}

impl From<&'_ ModSegment> for Box<ModSegment> {
    fn from(value: &'_ ModSegment) -> Self {
        unsafe {
            ModSegment::new_boxed_unchecked(Box::<str>::from(value.content()))
        }
    }
}

impl FromStr for Box<ModSegment> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ModSegment::new(s).map(Box::from)
    }
}

impl<'a> TryFrom<&'a str> for &'a ModSegment {
    type Error = Error;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        ModSegment::new(content)
    }
}

impl<'a> TryFrom<Box<str>> for Box<ModSegment> {
    type Error = Error;

    fn try_from(content: Box<str>) -> Result<Self, Self::Error> {
        ModSegment::new_boxed(content)
    }
}

impl<'a> TryFrom<String> for Box<ModSegment> {
    type Error = Error;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        Self::try_from(content.into_boxed_str())
    }
}

impl ToOwned for ModSegment {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        Box::from(self)
    }
}

impl Borrow<str> for ModSegment {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl AsRef<str> for ModSegment {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl AsRef<Self> for ModSegment {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for ModSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl PartialEq<str> for ModSegment {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<ModSegment> for str {
    fn eq(&self, other: &ModSegment) -> bool {
        self == other.content()
    }
}

impl Serialize for ModSegment {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.content())
    }
}

impl<'de> Deserialize<'de> for Box<ModSegment> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Box<ModSegment>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("an alphanumeric identifier string")
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Box::try_from(v.to_owned()).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

pub trait ModPathIndex: fmt::Debug {
    type Output: ?Sized;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output>;
}

impl ModPathIndex for usize {
    type Output = ModSegment;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output> {
        path.segments().nth(*self)
    }
}

impl ModPathIndex for Range<usize> {
    type Output = ModPath;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output> {
        if self.start >= self.end {
            return Some(ModPath::ROOT);
        }

        let mut iter = path
            .content()
            .bytes()
            .enumerate()
            .filter(|(_, byte)| *byte == b'.')
            .map(|(i, _)| i);
        let mut pos = 0;
        let mut curr_segment = 0;
        loop {
            if curr_segment == self.start {
                break;
            }
            curr_segment += 1;
            let Some(i) = iter.next() else {
                if self.start == curr_segment {
                    return Some(ModPath::ROOT);
                }
                None?
            };
            pos = i;
        }
        let start = pos;
        loop {
            curr_segment += 1;
            if curr_segment == self.end {
                break;
            }
            let Some(i) = iter.next() else { None? };
            pos = i;
        }
        let end = pos;

        unsafe { Some(ModPath::new_unchecked(&path.content()[start .. end])) }
    }
}

impl ModPathIndex for RangeTo<usize> {
    type Output = ModPath;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output> {
        ModPathIndex::get(&(0 .. self.end), path)
    }
}

impl ModPathIndex for RangeFrom<usize> {
    type Output = ModPath;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output> {
        let mut iter = path
            .content()
            .bytes()
            .enumerate()
            .filter(|(_, byte)| *byte == b'.')
            .map(|(i, _)| i);
        let mut pos = 0;
        let mut curr_segment = 0;
        loop {
            if curr_segment == self.start {
                break;
            }
            curr_segment += 1;
            let Some(i) = iter.next() else {
                if self.start == curr_segment {
                    return Some(ModPath::ROOT);
                }
                None?
            };
            pos = i;
        }
        let start = pos;

        unsafe { Some(ModPath::new_unchecked(&path.content()[start ..])) }
    }
}

impl ModPathIndex for RangeToInclusive<usize> {
    type Output = ModPath;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output> {
        ModPathIndex::get(&(0 .. self.end.saturating_add(1)), path)
    }
}

impl ModPathIndex for RangeInclusive<usize> {
    type Output = ModPath;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output> {
        ModPathIndex::get(&(0 .. self.end().saturating_add(1)), path)
    }
}

impl ModPathIndex for RangeFull {
    type Output = ModPath;

    fn get<'a>(&self, path: &'a ModPath) -> Option<&'a Self::Output> {
        Some(path)
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ModPath {
    content: str,
}

impl ModPath {
    pub const ROOT: &'static Self = unsafe { Self::new_unchecked("") };

    pub fn new(content: &str) -> Result<&Self, Error> {
        if content == Self::ROOT {
            return Ok(Self::ROOT);
        }
        for segment in content.split(".") {
            ModSegment::new(segment).map_err(|error| {
                error.replacing(ErrorInner::InvalidModPathStr {
                    input: content.to_owned(),
                    path: Default::default(),
                })
            })?;
        }
        unsafe { Ok(Self::new_unchecked(content)) }
    }

    pub fn new_boxed(content: Box<str>) -> Result<Box<Self>, Error> {
        ModPath::new(&content)?;
        unsafe { Ok(Self::new_boxed_unchecked(content)) }
    }

    pub const unsafe fn new_unchecked(content: &str) -> &Self {
        unsafe { mem::transmute(content) }
    }

    pub const unsafe fn new_boxed_unchecked(content: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(content) }
    }

    pub fn from_segments(
        parts: impl IntoIterator<Item = impl AsRef<ModSegment>>,
    ) -> Box<Self> {
        let mut iter = parts.into_iter();
        let mut content = String::new();
        if let Some(first) = iter.next() {
            content.push_str(first.as_ref().content());
            for segment in iter {
                content.push('.');
                content.push_str(segment.as_ref().content());
            }
        }
        unsafe { Self::new_boxed_unchecked(content.into_boxed_str()) }
    }

    pub fn segments<'a>(
        &'a self,
    ) -> impl Iterator<Item = &'a ModSegment> + Send + Sync + 'a {
        self.content()
            .split('.')
            .map(|segment| unsafe { ModSegment::new_unchecked(segment) })
    }

    pub fn count_segments(&self) -> usize {
        self.content().bytes().filter(|ch| *ch == b'.').count()
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn get<'a, I>(&'a self, index: I) -> Option<&'a I::Output>
    where
        I: ModPathIndex,
    {
        index.get(self)
    }
}

impl Default for Box<ModPath> {
    fn default() -> Self {
        ModPath::ROOT.to_owned()
    }
}

impl Clone for Box<ModPath> {
    fn clone(&self) -> Self {
        Self::from(&**self)
    }
}

impl From<&'_ ModPath> for Box<ModPath> {
    fn from(value: &'_ ModPath) -> Self {
        unsafe {
            ModPath::new_boxed_unchecked(Box::<str>::from(value.content()))
        }
    }
}

impl FromStr for Box<ModPath> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ModPath::new(s).map(Box::from)
    }
}

impl<'a> TryFrom<&'a str> for &'a ModPath {
    type Error = Error;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        ModPath::new(content)
    }
}

impl<'a> TryFrom<Box<str>> for Box<ModPath> {
    type Error = Error;

    fn try_from(content: Box<str>) -> Result<Self, Self::Error> {
        ModPath::new_boxed(content)
    }
}

impl<'a> TryFrom<String> for Box<ModPath> {
    type Error = Error;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        Self::try_from(content.into_boxed_str())
    }
}

impl ToOwned for ModPath {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        Box::from(self)
    }
}

impl Borrow<str> for ModPath {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl AsRef<str> for ModPath {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl AsRef<Self> for ModPath {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for ModPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl PartialEq<str> for ModPath {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<ModPath> for str {
    fn eq(&self, other: &ModPath) -> bool {
        self == other.content()
    }
}

impl Serialize for ModPath {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.content())
    }
}

impl<'de> Deserialize<'de> for Box<ModPath> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Box<ModPath>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a text of dot-separated sequence of module segments",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Box::try_from(v.to_owned()).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl<I> Index<I> for ModPath
where
    I: ModPathIndex,
{
    type Output = <I as ModPathIndex>::Output;

    fn index(&self, index: I) -> &Self::Output {
        index.get(self).unwrap_or_else(|| {
            panic!(
                "module path segment-indexing {:?} is out of bounds {}",
                index,
                self.count_segments()
            )
        })
    }
}

pub trait ModPathPush: fmt::Debug + Sized {
    fn push_to(self, path: &mut ModPathBuf);

    fn join_to(self, path: &ModPathBuf) -> ModPathBuf {
        let mut cloned = path.clone();
        self.push_to(&mut cloned);
        cloned
    }
}

impl ModPathPush for &'_ ModSegment {
    fn push_to(self, path: &mut ModPathBuf) {
        if !path.is_empty() {
            path.inner.push('.');
        }
        path.inner.push_str(self.content());
    }
}

impl ModPathPush for &'_ ModPath {
    fn push_to(self, path: &mut ModPathBuf) {
        path.inner.push('.');
        path.inner.push_str(self.content());
    }
}

impl ModPathPush for &'_ ModPathBuf {
    fn push_to(self, path: &mut ModPathBuf) {
        path.push(&self[..]);
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ModPathBuf {
    inner: String,
}

impl ModPathBuf {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_capacity(cap: usize) -> Self {
        Self { inner: String::with_capacity(cap) }
    }

    pub fn is_empty(&self) -> bool {
        self.inner.is_empty()
    }

    pub fn first(&self) -> Option<&ModSegment> {
        unsafe {
            let pos = self.inner.bytes().position(|ch| ch == b'.')?;
            Some(ModSegment::new_unchecked(&self.inner[.. pos]))
        }
    }

    pub fn last(&self) -> Option<&ModSegment> {
        unsafe {
            let pos = self.inner.bytes().rposition(|ch| ch == b'.')?;
            Some(ModSegment::new_unchecked(&self.inner[pos + 1 ..]))
        }
    }

    pub fn push<P>(&mut self, extra: P)
    where
        P: ModPathPush,
    {
        extra.push_to(self)
    }

    pub fn pop(&mut self) -> bool {
        let Some(pos) = self.inner.bytes().position(|ch| ch == b'.') else {
            return false;
        };
        self.inner.truncate(pos);
        true
    }

    pub fn join<P>(&self, extra: P) -> Self
    where
        P: ModPathPush,
    {
        extra.join_to(self)
    }

    pub fn clear(&mut self) {
        self.inner.clear();
    }
}

impl Deref for ModPathBuf {
    type Target = ModPath;

    fn deref(&self) -> &Self::Target {
        unsafe { ModPath::new_unchecked(&self.inner) }
    }
}

impl From<Box<ModPath>> for ModPathBuf {
    fn from(value: Box<ModPath>) -> Self {
        unsafe {
            let content: Box<str> = mem::transmute(value);
            Self { inner: content.into() }
        }
    }
}

impl From<ModPathBuf> for Box<ModPath> {
    fn from(value: ModPathBuf) -> Self {
        unsafe {
            let content: Box<str> = value.inner.into_boxed_str();
            mem::transmute(content)
        }
    }
}

impl<'a> From<&'a ModPath> for ModPathBuf {
    fn from(value: &'a ModPath) -> Self {
        Self { inner: value.content().into() }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModIdPartsRef<'a> {
    External { pkg_id: &'a PkgId, mod_path: &'a ModPath },
    Internal { mod_path: &'a ModPath },
}

impl<'a> ModIdPartsRef<'a> {
    pub fn pkg_id(self) -> Option<&'a PkgId> {
        match self {
            Self::External { pkg_id, .. } => Some(pkg_id),
            Self::Internal { .. } => None,
        }
    }

    pub fn mod_path(self) -> &'a ModPath {
        match self {
            Self::External { mod_path, .. } => mod_path,
            Self::Internal { mod_path } => mod_path,
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct ModId {
    content: str,
}

impl ModId {
    pub const INTERNAL_ROOT: &'static Self = unsafe { Self::new_unchecked("") };

    pub fn new(content: &str) -> Result<&Self, Error> {
        let result = match content.split_once("::") {
            Some((pkg_id, mod_path)) => PkgId::new(pkg_id)
                .and_then(|_| ModPath::new(mod_path))
                .map(|_| ()),
            None => ModPath::new(content).map(|_| ()),
        };
        result.map_err(|error| {
            error.replacing(ErrorInner::InvalidModIdStr {
                input: content.to_owned(),
                path: Default::default(),
            })
        })?;
        unsafe { Ok(Self::new_unchecked(content)) }
    }

    pub fn new_boxed(content: Box<str>) -> Result<Box<Self>, Error> {
        ModId::new(&content)?;
        unsafe { Ok(Self::new_boxed_unchecked(content)) }
    }

    pub const unsafe fn new_unchecked(content: &str) -> &Self {
        unsafe { mem::transmute(content) }
    }

    pub const unsafe fn new_boxed_unchecked(content: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(content) }
    }

    pub fn from_internal(path: &ModPath) -> &Self {
        unsafe { Self::new_unchecked(path.content()) }
    }

    pub fn from_parts(parts: ModIdPartsRef<'_>) -> Box<Self> {
        match parts {
            ModIdPartsRef::External { pkg_id, mod_path } => {
                let content =
                    format!("{}::{}", pkg_id.content(), mod_path.content());
                unsafe { Self::new_boxed_unchecked(content.into_boxed_str()) }
            },
            ModIdPartsRef::Internal { mod_path } => unsafe {
                Self::new_boxed_unchecked(mod_path.content().into())
            },
        }
    }

    pub fn parts<'a>(&'a self) -> ModIdPartsRef<'a> {
        match self.content().split_once("::") {
            Some((pkg_id, mod_path)) => unsafe {
                ModIdPartsRef::External {
                    pkg_id: PkgId::new_unchecked(pkg_id),
                    mod_path: ModPath::new_unchecked(mod_path),
                }
            },
            None => unsafe {
                ModIdPartsRef::Internal {
                    mod_path: ModPath::new_unchecked(self.content()),
                }
            },
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }
}

impl Clone for Box<ModId> {
    fn clone(&self) -> Self {
        Self::from(&**self)
    }
}

impl From<&'_ ModId> for Box<ModId> {
    fn from(value: &'_ ModId) -> Self {
        unsafe { ModId::new_boxed_unchecked(Box::<str>::from(value.content())) }
    }
}

impl FromStr for Box<ModId> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        ModId::new(s).map(Box::from)
    }
}

impl<'a> TryFrom<&'a str> for &'a ModId {
    type Error = Error;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        ModId::new(content)
    }
}

impl<'a> TryFrom<Box<str>> for Box<ModId> {
    type Error = Error;

    fn try_from(content: Box<str>) -> Result<Self, Self::Error> {
        ModId::new_boxed(content)
    }
}

impl<'a> TryFrom<String> for Box<ModId> {
    type Error = Error;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        Self::try_from(content.into_boxed_str())
    }
}

impl From<ModIdPartsRef<'_>> for Box<ModId> {
    fn from(parts: ModIdPartsRef<'_>) -> Self {
        ModId::from_parts(parts)
    }
}

impl ToOwned for ModId {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        Box::from(self)
    }
}

impl Borrow<str> for ModId {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl AsRef<str> for ModId {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl AsRef<Self> for ModId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for ModId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl PartialEq<str> for ModId {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<ModId> for str {
    fn eq(&self, other: &ModId) -> bool {
        self == other.content()
    }
}

impl Serialize for ModId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.content())
    }
}

impl<'de> Deserialize<'de> for Box<ModId> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Box<ModId>;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str(
                    "a text of package id, '/', and dot-separated sequence of \
                     module segments",
                )
            }

            fn visit_str<E>(self, v: &str) -> Result<Self::Value, E>
            where
                E: serde::de::Error,
            {
                Box::try_from(v.to_owned()).map_err(E::custom)
            }
        }

        deserializer.deserialize_str(Visitor)
    }
}

impl FromLua for Box<ModId> {
    fn from_lua(value: Value, lua: &Lua) -> Result<Self, mlua::Error> {
        let stringified = Box::<str>::from_lua(value, lua)?;
        let this =
            ModId::new_boxed(stringified).map_err(mlua::Error::external)?;
        Ok(this)
    }
}

impl IntoLua for Box<ModId> {
    fn into_lua(self, lua: &Lua) -> Result<Value, mlua::Error> {
        (*self).into_lua(lua)
    }
}

impl IntoLua for &'_ Box<ModId> {
    fn into_lua(self, lua: &Lua) -> Result<Value, mlua::Error> {
        (**self).into_lua(lua)
    }
}

impl IntoLua for &'_ ModId {
    fn into_lua(self, lua: &Lua) -> Result<Value, mlua::Error> {
        lua.create_string(self.content())
            .and_then(|string| string.into_lua(lua))
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::id::{
        module::{ModId, ModIdPartsRef, ModPath, ModSegment},
        package::PkgId,
    };

    #[test]
    fn mod_segment_should_accept_alphabetic() -> Result<()> {
        ModSegment::new("foobar")?;
        Ok(())
    }

    #[test]
    fn mod_segment_should_accept_numeric_not_at_start() -> Result<()> {
        ModSegment::new("foobar123")?;
        Ok(())
    }

    #[test]
    fn mod_segment_should_accept_underscore() -> Result<()> {
        ModSegment::new("_abc")?;
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_accept_numeric_at_start() -> Result<()> {
        let result = ModSegment::new("0abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_accept_special() -> Result<()> {
        let result = ModSegment::new("~abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_accept_unicode() -> Result<()> {
        let result = ModSegment::new("abç");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_empty() -> Result<()> {
        let result = ModSegment::new("");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_empty_as_root() -> Result<()> {
        let segment = ModPath::new("")?;
        assert_eq!(segment, ModPath::ROOT);
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_single_segment() -> Result<()> {
        let segment = ModPath::new("music")?;
        assert_eq!(segment, "music");
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_two_segments() -> Result<()> {
        let segment = ModPath::new("music.tempo")?;
        assert_eq!(segment, "music.tempo");
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_three_segments() -> Result<()> {
        let segment = ModPath::new("music.tempo.signature")?;
        assert_eq!(segment, "music.tempo.signature");
        Ok(())
    }

    #[test]
    fn mod_path_should_not_accept_empty_segmeents() -> Result<()> {
        let result = ModPath::new("music..signature");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_path_should_not_accept_invalid_segments() -> Result<()> {
        let result = ModPath::new("music.abç.signature");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_external_root() -> Result<()> {
        let id = ModId::new("foobar::")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::External {
                pkg_id: PkgId::new("foobar")?,
                mod_path: ModPath::ROOT,
            }
        );
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_external_full() -> Result<()> {
        let id = ModId::new("bcz/foobar::bazz")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::External {
                pkg_id: PkgId::new("bcz/foobar")?,
                mod_path: ModPath::new("bazz")?,
            }
        );
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_external_non_root_path() -> Result<()> {
        let id = ModId::new("foobar::music.tuning")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::External {
                pkg_id: PkgId::new("foobar")?,
                mod_path: ModPath::new("music.tuning")?,
            },
        );
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_internal_path() -> Result<()> {
        let id = ModId::new("music.tuning")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::Internal { mod_path: ModPath::new("music.tuning")? },
        );
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_internal_root() -> Result<()> {
        let id = ModId::new("")?;
        let parts = id.parts();
        assert_eq!(parts, ModIdPartsRef::Internal { mod_path: ModPath::ROOT },);
        Ok(())
    }
}
