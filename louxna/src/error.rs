use std::{error::Error as StdError, ffi::OsString, fmt, mem, path::PathBuf};

use thiserror::Error;
use tokio::io;

use crate::id::module::{ModId, ModPath};

#[derive(Debug, Error)]
#[error(transparent)]
pub struct Error {
    inner: Box<ErrorInner>,
}

impl Error {
    pub fn multi(errors: impl IntoIterator<Item = Self>) -> Result<(), Self> {
        let errors = errors.into_iter();
        let (low, _) = errors.size_hint();
        let mut collection = Vec::with_capacity(low);

        for mut error in errors {
            if let ErrorInner::Multi(multi) = &mut *error.inner {
                let mut to_flatten = mem::take(&mut multi.inner).into_vec();
                collection.append(&mut to_flatten);
            } else {
                collection.push(error);
            }
        }

        if collection.is_empty() {
            Ok(())
        } else {
            let multi = ErrorMulti { inner: collection.into() };
            Err(ErrorInner::Multi(multi))?
        }
    }

    pub(crate) fn replace(&mut self, inner: ErrorInner) -> ErrorInner {
        mem::replace(&mut *self.inner, inner)
    }

    pub(crate) fn replacing(mut self, inner: ErrorInner) -> Self {
        self.replace(inner);
        self
    }
}

impl<E> From<E> for Error
where
    ErrorInner: From<E>,
{
    fn from(value: E) -> Self {
        Self { inner: Box::new(value.into()) }
    }
}

#[derive(Debug, Error)]
pub enum ErrorInner {
    #[error("multiple errors found")]
    Multi(#[from] ErrorMulti),
    #[error("failed to initialize Lua")]
    InitLua(#[source] mlua::Error),
    #[error("failed to read Lua module from path {}", path.display())]
    ReadLuaMod {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to initialize Lua")]
    EvalLuaMod {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to set Lua CLI arguments")]
    SetLuaCliArgs {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to create Lua require() function")]
    CreateLuaRequire {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to set Lua require() function")]
    SetLuaRequire {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to call Lua require() function")]
    CallLuaRequire {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to create Lua environment for module")]
    FailedToCreateLuaEnv {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to set arguments of Lua environment for module")]
    FailedToSetLuaEnvArguments {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to create metatable of Lua environment for module")]
    FailedToCreateLuaEnvMeta {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to set metatable __index of Lua environment for module")]
    FailedToSetLuaEnvMetaIndex {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to set metatable __newindex of Lua environment for module")]
    FailedToSetLuaEnvMetaNewIndex {
        #[source]
        source: mlua::Error,
    },
    #[error("failed to set metatable of Lua environment for module")]
    FailedToSetLuaEnvMeta {
        #[source]
        source: mlua::Error,
    },
    #[error("invalid package name {:?}{}", input, path)]
    InvalidPkgNameStr { input: String, path: ContextualPath },
    #[error("invalid package namespace {:?}{}", input, path)]
    InvalidPkgNamespaceStr { input: String, path: ContextualPath },
    #[error("invalid package identifier {:?}{}", input, path)]
    InvalidPkgIdStr { input: String, path: ContextualPath },
    #[error("invalid module segment {:?}{}", input, path)]
    InvalidModSegmentStr { input: String, path: ContextualPath },
    #[error("invalid module path {:?}{}", input, path)]
    InvalidModPathStr { input: String, path: ContextualPath },
    #[error("invalid module identifier {:?}{}", input, path)]
    InvalidModIdStr { input: String, path: ContextualPath },
    #[error(
        "invalid module identifier in OS string {:?}{}",
        input.display().to_string(),
        path
    )]
    InvalidModIdOsStr { input: OsString, path: ContextualPath },
    #[error("invalid module OS path {:?}", path.display().to_string())]
    InvalidModOsPath { path: PathBuf },
    #[error("duplicate module path {} inside the package {}", mod_path, path)]
    DuplicateModPath { mod_path: Box<ModPath>, path: ContextualPath },
    #[error("unknown module identifier {}", mod_id)]
    UnknownModule { mod_id: Box<ModId> },
    #[error("failed to read package config from path {}", path.display())]
    ReadPkgConfig {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error("failed to parse package config read from path {}", path.display())]
    ParsePkgConfig {
        path: PathBuf,
        #[source]
        source: toml::de::Error,
    },
    #[error("failed to write package config to path {}", path.display())]
    WritePkgConfig {
        path: PathBuf,
        #[source]
        source: io::Error,
    },
    #[error(
        "failed to emit package config to be written to path {}",
        path.display()
    )]
    EmitPkgConfig {
        path: PathBuf,
        #[source]
        source: toml::ser::Error,
    },
}

impl ErrorInner {
    pub fn set_contextual_path(&mut self, new_path: impl Into<ContextualPath>) {
        match self {
            Self::InvalidPkgNameStr { path, .. }
            | Self::InvalidModSegmentStr { path, .. }
            | Self::InvalidModPathStr { path, .. }
            | Self::DuplicateModPath { path, .. } => *path = new_path.into(),
            _ => (),
        }
    }

    pub fn with_contextual_path(
        mut self,
        new_path: impl Into<ContextualPath>,
    ) -> Self {
        self.set_contextual_path(new_path);
        self
    }
}

#[derive(Debug)]
pub struct ErrorMulti {
    inner: Box<[Error]>,
}

impl fmt::Display for ErrorMulti {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for error in &self.inner {
            writeln!(f, "- {error}")?;
            let mut next = error.source();
            while let Some(curr) = next {
                writeln!(f, "  Caused by: {curr}")?;
                next = curr.source();
            }
        }
        Ok(())
    }
}

impl StdError for ErrorMulti {}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct ContextualPath(pub Option<PathBuf>);

impl fmt::Display for ContextualPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if let Some(path) = &self.0 {
            write!(f, " (in path {})", path.display())?;
        }
        Ok(())
    }
}

impl<P> From<P> for ContextualPath
where
    P: Into<PathBuf>,
{
    fn from(value: P) -> Self {
        Self(Some(value.into()))
    }
}
