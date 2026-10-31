use std::{error::Error as StdError, fmt, mem, path::PathBuf};

use thiserror::Error;
use tokio::io;

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
