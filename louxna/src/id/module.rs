use std::{
    borrow::Borrow,
    ffi::{OsStr, OsString},
    fmt,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{Error, error::ErrorInner, id::package::PkgIdRef};

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "&str", into = "&str")]
pub struct ModSegmentRef<'a> {
    content: &'a str,
}

impl<'a> ModSegmentRef<'a> {
    pub fn content(self) -> &'a str {
        self.content
    }

    pub fn to_owned(self) -> ModSegment {
        ModSegment { content: self.content.into() }
    }
}

impl fmt::Display for ModSegmentRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<str> for ModSegmentRef<'_> {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for ModSegmentRef<'_> {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl PartialEq<str> for ModSegmentRef<'_> {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<ModSegmentRef<'_>> for str {
    fn eq(&self, other: &ModSegmentRef<'_>) -> bool {
        self == other.content()
    }
}

impl<'a> TryFrom<&'a str> for ModSegmentRef<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if !value.starts_with(|ch: char| ch.is_ascii_alphabetic() || ch == '_')
        {
            Err(ErrorInner::InvalidModSegmentStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        }
        if !value.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Err(ErrorInner::InvalidModSegmentStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        }
        Ok(Self { content: value })
    }
}

impl<'a> TryFrom<&'a OsStr> for ModSegmentRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        let Some(as_str) = value.to_str() else {
            Err(ErrorInner::InvalidModSegmentOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        };
        Self::try_from(as_str).map_err(|error| {
            error.replacing(ErrorInner::InvalidModSegmentOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })
        })
    }
}

impl<'a> TryFrom<&'a OsString> for ModSegmentRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for ModSegmentRef<'a> {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> From<ModSegmentRef<'a>> for &'a str {
    fn from(value: ModSegmentRef<'a>) -> Self {
        value.content()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(try_from = "&str", into = "String")]
pub struct ModSegment {
    content: Box<str>,
}

impl ModSegment {
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn as_ref(&self) -> ModSegmentRef<'_> {
        ModSegmentRef { content: self.content() }
    }
}

impl fmt::Display for ModSegment {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<Self> for ModSegment {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for ModSegment {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for ModSegment {
    fn borrow(&self) -> &str {
        self.content()
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

impl PartialEq<ModSegmentRef<'_>> for ModSegment {
    fn eq(&self, other: &ModSegmentRef<'_>) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<ModSegment> for ModSegmentRef<'_> {
    fn eq(&self, other: &ModSegment) -> bool {
        other.as_ref() == *self
    }
}

impl<'a> TryFrom<&'a str> for ModSegment {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        ModSegmentRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsStr> for ModSegment {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        ModSegmentRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsString> for ModSegment {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for ModSegment {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl From<ModSegment> for String {
    fn from(value: ModSegment) -> Self {
        value.content.into_string()
    }
}

impl FromStr for ModSegment {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "&str", into = "&str")]
pub struct ModPathRef<'a> {
    content: &'a str,
}

impl<'a> ModPathRef<'a> {
    pub const ROOT: Self = Self { content: "" };

    pub fn content(self) -> &'a str {
        self.content
    }

    pub fn to_owned(self) -> ModPath {
        ModPath { content: self.content.into() }
    }

    pub fn segments(
        self,
    ) -> impl Iterator<Item = ModSegmentRef<'a>> + Send + Sync {
        self.content.split('.').map(|content| ModSegmentRef { content })
    }
}

impl fmt::Display for ModPathRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<str> for ModPathRef<'_> {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for ModPathRef<'_> {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl PartialEq<str> for ModPathRef<'_> {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<ModPathRef<'_>> for str {
    fn eq(&self, other: &ModPathRef<'_>) -> bool {
        self == other.content()
    }
}

impl<'a> TryFrom<&'a str> for ModPathRef<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if value == &Self::ROOT {
            return Ok(Self::ROOT);
        }
        for segment in value.split(".") {
            ModSegmentRef::try_from(segment).map_err(|error| {
                error.replacing(ErrorInner::InvalidModPathStr {
                    input: value.to_owned(),
                    path: Default::default(),
                })
            })?;
        }
        Ok(Self { content: value })
    }
}

impl<'a> TryFrom<&'a OsStr> for ModPathRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        let Some(as_str) = value.to_str() else {
            Err(ErrorInner::InvalidModPathOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        };
        Self::try_from(as_str).map_err(|error| {
            error.replacing(ErrorInner::InvalidModPathOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })
        })
    }
}

impl<'a> TryFrom<&'a OsString> for ModPathRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for ModPathRef<'a> {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> From<ModPathRef<'a>> for &'a str {
    fn from(value: ModPathRef<'a>) -> Self {
        value.content()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(try_from = "&str", into = "String")]
pub struct ModPath {
    content: Box<str>,
}

impl ModPath {
    pub fn from_segments<'a>(
        segments: impl IntoIterator<Item = ModSegmentRef<'a>>,
    ) -> Self {
        let mut iter = segments.into_iter();
        let mut content = String::new();
        if let Some(first) = iter.next() {
            content.push_str(first.content());
            for segment in iter {
                content.push('.');
                content.push_str(segment.content());
            }
        }
        Self { content: content.into_boxed_str() }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn as_ref<'a>(&'a self) -> ModPathRef<'a> {
        ModPathRef { content: self.content() }
    }

    pub fn segments<'a>(
        &'a self,
    ) -> impl Iterator<Item = ModSegmentRef<'a>> + Send + Sync + 'a {
        self.as_ref().segments()
    }
}

impl fmt::Display for ModPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<Self> for ModPath {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for ModPath {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for ModPath {
    fn borrow(&self) -> &str {
        self.content()
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

impl PartialEq<ModPathRef<'_>> for ModPath {
    fn eq(&self, other: &ModPathRef<'_>) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<ModPath> for ModPathRef<'_> {
    fn eq(&self, other: &ModPath) -> bool {
        other.as_ref() == *self
    }
}

impl<'a> TryFrom<&'a str> for ModPath {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        ModPathRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsStr> for ModPath {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        ModPathRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsString> for ModPath {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for ModPath {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl From<ModPath> for String {
    fn from(value: ModPath) -> Self {
        value.content.into_string()
    }
}

impl FromStr for ModPath {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ModIdPartsRef<'a> {
    External { pkg_id: PkgIdRef<'a>, mod_path: ModPathRef<'a> },
    Internal { mod_path: ModPathRef<'a> },
}

impl<'a> ModIdPartsRef<'a> {
    pub fn pkg_id(self) -> Option<PkgIdRef<'a>> {
        match self {
            Self::External { pkg_id, .. } => Some(pkg_id),
            Self::Internal { .. } => None,
        }
    }

    pub fn mod_path(self) -> ModPathRef<'a> {
        match self {
            Self::External { mod_path, .. } => mod_path,
            Self::Internal { mod_path } => mod_path,
        }
    }
}

#[derive(
    Debug,
    Clone,
    Copy,
    PartialEq,
    Eq,
    PartialOrd,
    Ord,
    Hash,
    Serialize,
    Deserialize,
)]
#[serde(try_from = "&str", into = "&str")]
pub struct ModIdRef<'a> {
    content: &'a str,
}

impl<'a> ModIdRef<'a> {
    pub fn content(self) -> &'a str {
        self.content
    }

    pub fn parts(self) -> ModIdPartsRef<'a> {
        match self.content().split_once('/') {
            Some((pkg_id, mod_path)) => ModIdPartsRef::External {
                pkg_id: PkgIdRef::from_raw(pkg_id),
                mod_path: ModPathRef { content: mod_path },
            },
            None => ModIdPartsRef::Internal {
                mod_path: ModPathRef { content: self.content() },
            },
        }
    }

    pub fn to_owned(self) -> ModId {
        ModId { content: self.content.into() }
    }
}

impl fmt::Display for ModIdRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<str> for ModIdRef<'_> {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for ModIdRef<'_> {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl PartialEq<str> for ModIdRef<'_> {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<ModIdRef<'_>> for str {
    fn eq(&self, other: &ModIdRef<'_>) -> bool {
        self == other.content()
    }
}

impl<'a> TryFrom<&'a str> for ModIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        let result = match value.split_once('/') {
            Some((pkg_id, mod_path)) => PkgIdRef::try_from(pkg_id)
                .and_then(|_| ModPathRef::try_from(mod_path))
                .map(|_| ()),
            None => ModPathRef::try_from(value).map(|_| ()),
        };
        result.map_err(|error| {
            error.replacing(ErrorInner::InvalidModIdStr {
                input: value.to_owned(),
                path: Default::default(),
            })
        })?;
        Ok(Self { content: value })
    }
}

impl<'a> TryFrom<&'a OsStr> for ModIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        let Some(as_str) = value.to_str() else {
            Err(ErrorInner::InvalidModIdOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        };
        Self::try_from(as_str).map_err(|error| {
            error.replacing(ErrorInner::InvalidModIdOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })
        })
    }
}

impl<'a> TryFrom<&'a OsString> for ModIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for ModIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> From<ModIdRef<'a>> for &'a str {
    fn from(value: ModIdRef<'a>) -> Self {
        value.content()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(try_from = "&str", into = "String")]
pub struct ModId {
    content: Box<str>,
}

impl ModId {
    pub fn from_parts(parts: ModIdPartsRef<'_>) -> Self {
        match parts {
            ModIdPartsRef::External { pkg_id, mod_path } => {
                let content =
                    format!("{}/{}", pkg_id.content(), mod_path.content());
                Self { content: content.into_boxed_str() }
            },
            ModIdPartsRef::Internal { mod_path } => {
                Self { content: mod_path.content().into() }
            },
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn as_ref<'a>(&'a self) -> ModIdRef<'a> {
        ModIdRef { content: self.content() }
    }

    pub fn parts<'a>(&'a self) -> ModIdPartsRef<'a> {
        self.as_ref().parts()
    }
}

impl fmt::Display for ModId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<Self> for ModId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for ModId {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for ModId {
    fn borrow(&self) -> &str {
        self.content()
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

impl PartialEq<ModIdRef<'_>> for ModId {
    fn eq(&self, other: &ModIdRef<'_>) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<ModId> for ModIdRef<'_> {
    fn eq(&self, other: &ModId) -> bool {
        other.as_ref() == *self
    }
}

impl<'a> TryFrom<&'a str> for ModId {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        ModIdRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsStr> for ModId {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        ModIdRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsString> for ModId {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for ModId {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl From<ModId> for String {
    fn from(value: ModId) -> Self {
        value.content.into_string()
    }
}

impl FromStr for ModId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::id::{
        module::{ModId, ModIdPartsRef, ModPath, ModPathRef, ModSegment},
        package::{PkgId, PkgIdRef},
    };

    #[test]
    fn mod_segment_should_accept_alphabetic() -> Result<()> {
        ModSegment::try_from("foobar")?;
        Ok(())
    }

    #[test]
    fn mod_segment_should_accept_numeric_not_at_start() -> Result<()> {
        ModSegment::try_from("foobar123")?;
        Ok(())
    }

    #[test]
    fn mod_segment_should_accept_underscore() -> Result<()> {
        ModSegment::try_from("_abc")?;
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_accept_numeric_at_start() -> Result<()> {
        let result = ModSegment::try_from("0abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_accept_special() -> Result<()> {
        let result = ModSegment::try_from("~abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_accept_unicode() -> Result<()> {
        let result = ModSegment::try_from("abç");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_segment_should_not_empty() -> Result<()> {
        let result = ModSegment::try_from("");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_empty_as_root() -> Result<()> {
        let segment = ModPath::try_from("")?;
        assert_eq!(segment, ModPathRef::ROOT);
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_single_segment() -> Result<()> {
        let segment = ModPath::try_from("music")?;
        assert_eq!(&segment, "music");
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_two_segments() -> Result<()> {
        let segment = ModPath::try_from("music.tempo")?;
        assert_eq!(&segment, "music.tempo");
        Ok(())
    }

    #[test]
    fn mod_path_should_accept_three_segments() -> Result<()> {
        let segment = ModPath::try_from("music.tempo.signature")?;
        assert_eq!(&segment, "music.tempo.signature");
        Ok(())
    }

    #[test]
    fn mod_path_should_not_accept_empty_segmeents() -> Result<()> {
        let result = ModPath::try_from("music..signature");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_path_should_not_accept_invalid_segments() -> Result<()> {
        let result = ModPath::try_from("music.abç.signature");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_external_root() -> Result<()> {
        let id = ModId::try_from("foobar/")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::External {
                pkg_id: PkgIdRef::try_from("foobar")?,
                mod_path: ModPathRef::ROOT,
            }
        );
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_external_non_root_path() -> Result<()> {
        let id = ModId::try_from("foobar/music.tuning")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::External {
                pkg_id: PkgIdRef::try_from("foobar")?,
                mod_path: ModPathRef::try_from("music.tuning")?,
            },
        );
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_internal_path() -> Result<()> {
        let id = ModId::try_from("music.tuning")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::Internal {
                mod_path: ModPathRef::try_from("music.tuning")?,
            },
        );
        Ok(())
    }

    #[test]
    fn mod_id_should_accept_internal_root() -> Result<()> {
        let id = ModId::try_from("")?;
        let parts = id.parts();
        assert_eq!(
            parts,
            ModIdPartsRef::Internal { mod_path: ModPathRef::ROOT },
        );
        Ok(())
    }
}
