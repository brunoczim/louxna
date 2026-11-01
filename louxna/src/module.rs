use std::{borrow::Cow, path::PathBuf};

use mlua::chunk::AsChunk;
use tokio::io;

use crate::id::module::ModId;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Module {
    pub path: PathBuf,
    pub id: Box<ModId>,
    pub content: Vec<u8>,
}

impl AsChunk for &'_ Module {
    fn name(&self) -> Option<String> {
        Some(format!("@{}", self.path.display()))
    }

    fn source<'a>(&self) -> io::Result<Cow<'a, [u8]>>
    where
        Self: 'a,
    {
        Ok(self.content.clone().into())
    }
}
