use std::{
    borrow::Borrow,
    ffi::{OsStr, OsString},
    fmt,
    str::FromStr,
};

use serde::{Deserialize, Serialize};

use crate::{Error, error::ErrorInner};

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
pub struct PkgNameRef<'a> {
    content: &'a str,
}

impl<'a> PkgNameRef<'a> {
    pub fn content(self) -> &'a str {
        self.content
    }

    pub fn to_owned(self) -> PkgName {
        PkgName { content: self.content.into() }
    }
}

impl fmt::Display for PkgNameRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<str> for PkgNameRef<'_> {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for PkgNameRef<'_> {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl PartialEq<str> for PkgNameRef<'_> {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<PkgNameRef<'_>> for str {
    fn eq(&self, other: &PkgNameRef<'_>) -> bool {
        self == other.content()
    }
}

impl<'a> TryFrom<&'a str> for PkgNameRef<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        if !value.starts_with(|ch: char| ch.is_ascii_alphabetic() || ch == '_')
        {
            Err(ErrorInner::InvalidPkgNameStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        }
        if !value.chars().all(|ch| ch.is_ascii_alphanumeric() || ch == '_') {
            Err(ErrorInner::InvalidPkgNameStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        }
        Ok(Self { content: value })
    }
}

impl<'a> TryFrom<&'a OsStr> for PkgNameRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        let Some(as_str) = value.to_str() else {
            Err(ErrorInner::InvalidPkgNameOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        };
        Self::try_from(as_str).map_err(|error| {
            error.replacing(ErrorInner::InvalidPkgNameOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })
        })
    }
}

impl<'a> TryFrom<&'a OsString> for PkgNameRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for PkgNameRef<'a> {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> From<PkgNameRef<'a>> for &'a str {
    fn from(value: PkgNameRef<'a>) -> Self {
        value.content()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(try_from = "&str", into = "String")]
pub struct PkgName {
    content: Box<str>,
}

impl PkgName {
    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn as_ref(&self) -> PkgNameRef<'_> {
        PkgNameRef { content: self.content() }
    }
}

impl fmt::Display for PkgName {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<Self> for PkgName {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for PkgName {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for PkgName {
    fn borrow(&self) -> &str {
        self.content()
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

impl PartialEq<PkgNameRef<'_>> for PkgName {
    fn eq(&self, other: &PkgNameRef<'_>) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<PkgName> for PkgNameRef<'_> {
    fn eq(&self, other: &PkgName) -> bool {
        other.as_ref() == *self
    }
}

impl<'a> TryFrom<&'a str> for PkgName {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        PkgNameRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsStr> for PkgName {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        PkgNameRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsString> for PkgName {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for PkgName {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl From<PkgName> for String {
    fn from(value: PkgName) -> Self {
        value.content.into_string()
    }
}

impl FromStr for PkgName {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PkgIdPartsRef<'a> {
    Minimal(PkgNameRef<'a>),
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
pub struct PkgIdRef<'a> {
    content: &'a str,
}

impl<'a> PkgIdRef<'a> {
    pub(super) fn from_raw(raw: &'a str) -> Self {
        Self { content: raw }
    }

    pub fn content(self) -> &'a str {
        self.content
    }

    pub fn to_owned(self) -> PkgId {
        PkgId { content: self.content.into() }
    }

    pub fn parts(self) -> PkgIdPartsRef<'a> {
        PkgIdPartsRef::Minimal(PkgNameRef { content: self.content() })
    }
}

impl fmt::Display for PkgIdRef<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<str> for PkgIdRef<'_> {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for PkgIdRef<'_> {
    fn borrow(&self) -> &str {
        self.content()
    }
}

impl PartialEq<str> for PkgIdRef<'_> {
    fn eq(&self, other: &str) -> bool {
        self.content() == other
    }
}

impl PartialEq<PkgIdRef<'_>> for str {
    fn eq(&self, other: &PkgIdRef<'_>) -> bool {
        self == other.content()
    }
}

impl<'a> TryFrom<&'a str> for PkgIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        PkgName::try_from(value).map_err(|error| {
            error.replacing(ErrorInner::InvalidPkgIdStr {
                input: value.to_owned(),
                path: Default::default(),
            })
        })?;
        Ok(Self { content: value })
    }
}

impl<'a> TryFrom<&'a OsStr> for PkgIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        let Some(as_str) = value.to_str() else {
            Err(ErrorInner::InvalidPkgIdOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })?
        };
        Self::try_from(as_str).map_err(|error| {
            error.replacing(ErrorInner::InvalidPkgIdOsStr {
                input: value.to_owned(),
                path: Default::default(),
            })
        })
    }
}

impl<'a> TryFrom<&'a OsString> for PkgIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for PkgIdRef<'a> {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> From<PkgIdRef<'a>> for &'a str {
    fn from(value: PkgIdRef<'a>) -> Self {
        value.content()
    }
}

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(try_from = "&str", into = "String")]
pub struct PkgId {
    content: Box<str>,
}

impl PkgId {
    pub fn from_parts(parts: PkgIdPartsRef<'_>) -> Self {
        match parts {
            PkgIdPartsRef::Minimal(name) => {
                Self { content: name.content().into() }
            },
        }
    }

    pub fn content(&self) -> &str {
        &self.content
    }

    pub fn as_ref(&self) -> PkgIdRef<'_> {
        PkgIdRef { content: self.content() }
    }

    pub fn parts<'a>(&'a self) -> PkgIdPartsRef<'a> {
        self.as_ref().parts()
    }
}

impl fmt::Display for PkgId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.content())
    }
}

impl AsRef<Self> for PkgId {
    fn as_ref(&self) -> &Self {
        self
    }
}

impl AsRef<str> for PkgId {
    fn as_ref(&self) -> &str {
        self.content()
    }
}

impl Borrow<str> for PkgId {
    fn borrow(&self) -> &str {
        self.content()
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

impl PartialEq<PkgIdRef<'_>> for PkgId {
    fn eq(&self, other: &PkgIdRef<'_>) -> bool {
        self.as_ref() == *other
    }
}

impl PartialEq<PkgId> for PkgIdRef<'_> {
    fn eq(&self, other: &PkgId) -> bool {
        other.as_ref() == *self
    }
}

impl<'a> TryFrom<&'a str> for PkgId {
    type Error = Error;

    fn try_from(value: &'a str) -> Result<Self, Self::Error> {
        PkgIdRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsStr> for PkgId {
    type Error = Error;

    fn try_from(value: &'a OsStr) -> Result<Self, Self::Error> {
        PkgIdRef::try_from(value).map(|r| r.to_owned())
    }
}

impl<'a> TryFrom<&'a OsString> for PkgId {
    type Error = Error;

    fn try_from(value: &'a OsString) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl<'a> TryFrom<&'a String> for PkgId {
    type Error = Error;

    fn try_from(value: &'a String) -> Result<Self, Self::Error> {
        Self::try_from(&**value)
    }
}

impl From<PkgId> for String {
    fn from(value: PkgId) -> Self {
        value.content.into_string()
    }
}

impl FromStr for PkgId {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Self::try_from(s)
    }
}

#[cfg(test)]
mod test {
    use anyhow::Result;

    use crate::id::package::{PkgId, PkgIdPartsRef, PkgName};

    #[test]
    fn pkg_name_should_accept_alphabetic() -> Result<()> {
        PkgName::try_from("foobar")?;
        Ok(())
    }

    #[test]
    fn pkg_name_should_accept_numeric_not_at_start() -> Result<()> {
        PkgName::try_from("foobar123")?;
        Ok(())
    }

    #[test]
    fn pkg_name_should_accept_underscore() -> Result<()> {
        PkgName::try_from("_abc")?;
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_accept_numeric_at_start() -> Result<()> {
        let result = PkgName::try_from("0abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_accept_special() -> Result<()> {
        let result = PkgName::try_from("~abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_accept_unicode() -> Result<()> {
        let result = PkgName::try_from("abç");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_name_should_not_empty() -> Result<()> {
        let result = PkgName::try_from("");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_accept_alphabetic() -> Result<()> {
        PkgId::try_from("foobar")?;
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_accept_numeric_not_at_start() -> Result<()> {
        PkgId::try_from("foobar123")?;
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_accept_underscore() -> Result<()> {
        PkgId::try_from("_abc")?;
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_numeric_at_start() -> Result<()> {
        let result = PkgId::try_from("0abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_special() -> Result<()> {
        let result = PkgId::try_from("~abc");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_unicode() -> Result<()> {
        let result = PkgId::try_from("abç");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_not_accept_empty() -> Result<()> {
        let result = PkgId::try_from("");
        assert!(result.is_err(), "expected error, got {result:#?}");
        Ok(())
    }

    #[test]
    fn pkg_id_minimal_should_be_constructible_from_parts() -> Result<()> {
        let name = PkgName::try_from("foobar")?;
        let parts_input = PkgIdPartsRef::Minimal(name.as_ref());
        let id = PkgId::from_parts(parts_input);
        let parts_output = id.parts();
        assert_eq!(parts_input, parts_output);
        assert_eq!(id.content(), "foobar");
        Ok(())
    }
}
