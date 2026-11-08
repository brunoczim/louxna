use std::{borrow::Borrow, fmt, mem, str::FromStr};

use serde::{Deserialize, Serialize};

use crate::{Error, error::ErrorInner};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PkgName {
    content: str,
}

impl PkgName {
    pub fn new(content: &str) -> Result<&Self, Error> {
        if !content
            .starts_with(|ch: char| ch.is_ascii_alphabetic() || ch == '_')
        {
            Err(ErrorInner::InvalidPkgNameStr {
                input: content.to_owned(),
                path: Default::default(),
            })?
        }
        if !content.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Err(ErrorInner::InvalidPkgNameStr {
                input: content.to_owned(),
                path: Default::default(),
            })?
        }
        unsafe { Ok(Self::new_unchecked(content)) }
    }

    pub fn new_boxed(content: Box<str>) -> Result<Box<Self>, Error> {
        PkgName::new(&content)?;
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

impl Clone for Box<PkgName> {
    fn clone(&self) -> Self {
        Self::from(&**self)
    }
}

impl From<&'_ PkgName> for Box<PkgName> {
    fn from(value: &'_ PkgName) -> Self {
        unsafe {
            PkgName::new_boxed_unchecked(Box::<str>::from(value.content()))
        }
    }
}

impl FromStr for Box<PkgName> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PkgName::new(s).map(Box::from)
    }
}

impl<'a> TryFrom<&'a str> for &'a PkgName {
    type Error = Error;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        PkgName::new(content)
    }
}

impl<'a> TryFrom<Box<str>> for Box<PkgName> {
    type Error = Error;

    fn try_from(content: Box<str>) -> Result<Self, Self::Error> {
        PkgName::new_boxed(content)
    }
}

impl<'a> TryFrom<String> for Box<PkgName> {
    type Error = Error;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        Self::try_from(content.into_boxed_str())
    }
}

impl ToOwned for PkgName {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        Box::from(self)
    }
}

impl Borrow<str> for PkgName {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl AsRef<str> for PkgName {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl AsRef<Self> for PkgName {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for PkgName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl PartialEq<str> for PkgName {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<PkgName> for str {
    fn eq(&self, other: &PkgName) -> bool {
        self == other.content()
    }
}

impl Serialize for PkgName {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.content())
    }
}

impl<'de> Deserialize<'de> for Box<PkgName> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Box<PkgName>;

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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PkgNamespace {
    content: str,
}

impl PkgNamespace {
    pub fn new(content: &str) -> Result<&Self, Error> {
        if !content
            .starts_with(|ch: char| ch.is_ascii_alphabetic() || ch == '_')
        {
            Err(ErrorInner::InvalidPkgNamespaceStr {
                input: content.to_owned(),
                path: Default::default(),
            })?
        }
        if !content.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Err(ErrorInner::InvalidPkgNamespaceStr {
                input: content.to_owned(),
                path: Default::default(),
            })?
        }
        unsafe { Ok(Self::new_unchecked(content)) }
    }

    pub fn new_boxed(content: Box<str>) -> Result<Box<Self>, Error> {
        PkgNamespace::new(&content)?;
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

impl Clone for Box<PkgNamespace> {
    fn clone(&self) -> Self {
        Self::from(&**self)
    }
}

impl From<&'_ PkgNamespace> for Box<PkgNamespace> {
    fn from(value: &'_ PkgNamespace) -> Self {
        unsafe {
            PkgNamespace::new_boxed_unchecked(Box::<str>::from(value.content()))
        }
    }
}

impl FromStr for Box<PkgNamespace> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PkgNamespace::new(s).map(Box::from)
    }
}

impl<'a> TryFrom<&'a str> for &'a PkgNamespace {
    type Error = Error;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        PkgNamespace::new(content)
    }
}

impl<'a> TryFrom<Box<str>> for Box<PkgNamespace> {
    type Error = Error;

    fn try_from(content: Box<str>) -> Result<Self, Self::Error> {
        PkgNamespace::new_boxed(content)
    }
}

impl<'a> TryFrom<String> for Box<PkgNamespace> {
    type Error = Error;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        Self::try_from(content.into_boxed_str())
    }
}

impl ToOwned for PkgNamespace {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        Box::from(self)
    }
}

impl Borrow<str> for PkgNamespace {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl AsRef<str> for PkgNamespace {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl AsRef<Self> for PkgNamespace {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for PkgNamespace {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl PartialEq<str> for PkgNamespace {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<PkgNamespace> for str {
    fn eq(&self, other: &PkgNamespace) -> bool {
        self == other.content()
    }
}

impl Serialize for PkgNamespace {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.content())
    }
}

impl<'de> Deserialize<'de> for Box<PkgNamespace> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Box<PkgNamespace>;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkgIdPartsRef<'a> {
    Minimal { name: &'a PkgName },
    Namespaced { namespace: &'a PkgNamespace, name: &'a PkgName },
}

impl<'a> PkgIdPartsRef<'a> {
    pub fn name(&self) -> &'a PkgName {
        match self {
            Self::Minimal { name } => name,
            Self::Namespaced { name, .. } => name,
        }
    }

    pub fn namespace(&self) -> Option<&'a PkgNamespace> {
        match self {
            Self::Minimal { .. } => None,
            Self::Namespaced { namespace, .. } => Some(namespace),
        }
    }
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[repr(transparent)]
pub struct PkgId {
    content: str,
}

impl PkgId {
    pub fn new(content: &str) -> Result<&Self, Error> {
        if let Some((namespace, name)) = content.split_once('/') {
            PkgNamespace::new(namespace).map_err(|error| {
                error.replacing(ErrorInner::InvalidPkgIdStr {
                    input: content.to_owned(),
                    path: Default::default(),
                })
            })?;
            PkgName::new(name).map_err(|error| {
                error.replacing(ErrorInner::InvalidPkgIdStr {
                    input: content.to_owned(),
                    path: Default::default(),
                })
            })?;
        } else {
            PkgName::new(content).map_err(|error| {
                error.replacing(ErrorInner::InvalidPkgIdStr {
                    input: content.to_owned(),
                    path: Default::default(),
                })
            })?;
        }
        unsafe { Ok(Self::new_unchecked(content)) }
    }

    pub fn new_boxed(content: Box<str>) -> Result<Box<Self>, Error> {
        PkgName::new(&content)?;
        unsafe { Ok(Self::new_boxed_unchecked(content)) }
    }

    pub const unsafe fn new_unchecked(content: &str) -> &Self {
        unsafe { mem::transmute(content) }
    }

    pub const unsafe fn new_boxed_unchecked(content: Box<str>) -> Box<Self> {
        unsafe { mem::transmute(content) }
    }

    pub fn from_parts(parts: PkgIdPartsRef<'_>) -> Box<Self> {
        match parts {
            PkgIdPartsRef::Minimal { name } => {
                let buf = format!("{}", name);
                unsafe { Self::new_boxed_unchecked(buf.into_boxed_str()) }
            },
            PkgIdPartsRef::Namespaced { namespace, name } => {
                let buf = format!("{}/{}", namespace, name);
                unsafe { Self::new_boxed_unchecked(buf.into_boxed_str()) }
            },
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn parts<'a>(&'a self) -> PkgIdPartsRef<'a> {
        if let Some((namespace, name)) = self.content().split_once('/') {
            PkgIdPartsRef::Namespaced {
                namespace: unsafe { PkgNamespace::new_unchecked(namespace) },
                name: unsafe { PkgName::new_unchecked(name) },
            }
        } else {
            PkgIdPartsRef::Minimal {
                name: unsafe { PkgName::new_unchecked(self.content()) },
            }
        }
    }

    pub fn compatible_with(&self, other: &Self) -> bool {
        match (self.parts(), other.parts()) {
            (
                PkgIdPartsRef::Namespaced {
                    namespace: l_namespace,
                    name: l_name,
                },
                PkgIdPartsRef::Namespaced {
                    namespace: r_namespace,
                    name: r_name,
                },
            ) => l_namespace == r_namespace && l_name == r_name,

            (
                PkgIdPartsRef::Namespaced { name: l_name, .. },
                PkgIdPartsRef::Minimal { name: r_name },
            )
            | (
                PkgIdPartsRef::Minimal { name: l_name },
                PkgIdPartsRef::Namespaced { name: r_name, .. },
            )
            | (
                PkgIdPartsRef::Minimal { name: l_name },
                PkgIdPartsRef::Minimal { name: r_name },
            ) => l_name == r_name,
        }
    }
}

impl Clone for Box<PkgId> {
    fn clone(&self) -> Self {
        Self::from(&**self)
    }
}

impl From<&'_ PkgId> for Box<PkgId> {
    fn from(value: &'_ PkgId) -> Self {
        unsafe { PkgId::new_boxed_unchecked(Box::<str>::from(value.content())) }
    }
}

impl FromStr for Box<PkgId> {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        PkgId::new(s).map(Box::from)
    }
}

impl<'a> TryFrom<&'a str> for &'a PkgId {
    type Error = Error;

    fn try_from(content: &'a str) -> Result<Self, Self::Error> {
        PkgId::new(content)
    }
}

impl<'a> TryFrom<Box<str>> for Box<PkgId> {
    type Error = Error;

    fn try_from(content: Box<str>) -> Result<Self, Self::Error> {
        PkgId::new_boxed(content)
    }
}

impl<'a> TryFrom<String> for Box<PkgId> {
    type Error = Error;

    fn try_from(content: String) -> Result<Self, Self::Error> {
        Self::try_from(content.into_boxed_str())
    }
}

impl ToOwned for PkgId {
    type Owned = Box<Self>;

    fn to_owned(&self) -> Self::Owned {
        Box::from(self)
    }
}

impl Borrow<str> for PkgId {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl AsRef<str> for PkgId {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl AsRef<Self> for PkgId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl fmt::Display for PkgId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl PartialEq<str> for PkgId {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<PkgId> for str {
    fn eq(&self, other: &PkgId) -> bool {
        self == other.content()
    }
}

impl Serialize for PkgId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(self.content())
    }
}

impl<'de> Deserialize<'de> for Box<PkgId> {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        struct Visitor;

        impl<'de> serde::de::Visitor<'de> for Visitor {
            type Value = Box<PkgId>;

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

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::id::package::{PkgId, PkgIdPartsRef, PkgName, PkgNamespace};

    #[test]
    fn pkg_name_should_accept_alphabetic() -> Result<()> {
        PkgName::new("foobar")?;
        Ok(())
    }

    #[test]
    fn pkg_name_should_accept_numeric_not_at_start() -> Result<()> {
        PkgName::new("foobar123")?;
        Ok(())
    }

    #[test]
    fn pkg_name_should_accept_underscore() -> Result<()> {
        PkgName::new("_abc")?;
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_accept_numeric_at_start() -> Result<()> {
        let result = PkgName::new("0abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_accept_special() -> Result<()> {
        let result = PkgName::new("~abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_accept_unicode() -> Result<()> {
        let result = PkgName::new("abç");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_empty() -> Result<()> {
        let result = PkgName::new("");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_namespace_should_accept_alphabetic() -> Result<()> {
        PkgNamespace::new("foobar")?;
        Ok(())
    }

    #[test]
    fn pkg_namespace_should_accept_numeric_not_at_start() -> Result<()> {
        PkgNamespace::new("foobar123")?;
        Ok(())
    }

    #[test]
    fn pkg_namespace_should_accept_underscore() -> Result<()> {
        PkgNamespace::new("_abc")?;
        Ok(())
    }

    #[test]
    fn pkg_namespace_should_not_accept_numeric_at_start() -> Result<()> {
        let result = PkgNamespace::new("0abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_namespace_should_not_accept_special() -> Result<()> {
        let result = PkgNamespace::new("~abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_namespace_should_not_accept_unicode() -> Result<()> {
        let result = PkgNamespace::new("abç");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_namespace_should_not_empty() -> Result<()> {
        let result = PkgNamespace::new("");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_accept_alphabetic() -> Result<()> {
        PkgId::new("foobar")?;
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_accept_numeric_not_at_start() -> Result<()> {
        PkgId::new("foobar123")?;
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_accept_underscore() -> Result<()> {
        PkgId::new("_abc")?;
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_numeric_at_start() -> Result<()> {
        let result = PkgId::new("0abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_special() -> Result<()> {
        let result = PkgId::new("~abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_unicode() -> Result<()> {
        let result = PkgId::new("abç");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_empty() -> Result<()> {
        let result = PkgId::new("");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_be_constructible_from_parts() -> Result<()> {
        let name = PkgName::new("foobar")?;
        let parts_input = PkgIdPartsRef::Minimal { name };
        let id = PkgId::from_parts(parts_input);
        let parts_output = id.parts();
        assert_eq!(parts_input, parts_output);
        assert_eq!(id.content(), "foobar");
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_should_accept_namespace() -> Result<()> {
        PkgId::new("myorg/foobar")?;
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_should_not_accept_empty_namespace() -> Result<()> {
        let result = PkgId::new("/foobar");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_should_be_constructible_from_parts() -> Result<()> {
        let name = PkgName::new("foobar")?;
        let namespace = PkgNamespace::new("myorg")?;
        let parts_input = PkgIdPartsRef::Namespaced { namespace, name };
        let id = PkgId::from_parts(parts_input);
        let parts_output = id.parts();
        assert_eq!(parts_input, parts_output);
        assert_eq!(id.content(), "myorg/foobar");
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_compatible_with_namespaced() -> Result<()> {
        let lhs = PkgId::new("bcz/foo")?;
        let rhs = PkgId::new("bcz/foo")?;
        assert!(lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_not_compatible_with_namespaced_on_diff_namespace()
    -> Result<()> {
        let lhs = PkgId::new("bcy/foo")?;
        let rhs = PkgId::new("bcz/foo")?;
        assert!(!lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_not_compatible_with_namespaced_on_diff_name()
    -> Result<()> {
        let lhs = PkgId::new("bcz/bar")?;
        let rhs = PkgId::new("bcz/foo")?;
        assert!(!lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_compatible_with_minimal() -> Result<()> {
        let lhs = PkgId::new("foo")?;
        let rhs = PkgId::new("foo")?;
        assert!(lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_not_compatible_with_minimal() -> Result<()> {
        let lhs = PkgId::new("foo")?;
        let rhs = PkgId::new("foy")?;
        assert!(!lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_compatible_with_minimal() -> Result<()> {
        let lhs = PkgId::new("bcz/foo")?;
        let rhs = PkgId::new("foo")?;
        assert!(lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_compatible_with_namespaced() -> Result<()> {
        let lhs = PkgId::new("foo")?;
        let rhs = PkgId::new("bcz/foo")?;
        assert!(lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_namespaced_not_compatible_with_minimal_on_diff_name() -> Result<()>
    {
        let lhs = PkgId::new("bcz/foy")?;
        let rhs = PkgId::new("foo")?;
        assert!(!lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_not_compatible_with_namespaced_on_diff_name() -> Result<()>
    {
        let lhs = PkgId::new("foo")?;
        let rhs = PkgId::new("bcz/foy")?;
        assert!(!lhs.compatible_with(rhs), "left: {lhs:#?}\nright: {rhs:#?}");
        Ok(())
    }
}
