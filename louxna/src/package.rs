use std::path::PathBuf;

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    Error,
    error::ErrorInner,
    id::package::{PkgId, PkgIdPartsRef, PkgName},
};

pub const PKG_CONFIG_STEM: &str = "pkg.lx.toml";

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
pub struct PkgConfig {
    pub name: PkgName,
}

impl PkgConfig {
    pub async fn load(root_dir: impl Into<PathBuf>) -> Result<Self, Error> {
        let mut path = root_dir.into();
        path.push(PKG_CONFIG_STEM);
        let content = fs::read(&path).await.map_err(|source| {
            ErrorInner::ReadPkgConfig { path: path.clone(), source }
        })?;
        let this = toml::de::from_slice(&content).map_err(|source| {
            ErrorInner::ParsePkgConfig { path: path.clone(), source }
        })?;
        Ok(this)
    }

    pub async fn save(
        &self,
        root_dir: impl Into<PathBuf>,
    ) -> Result<(), Error> {
        let mut path = root_dir.into();
        path.push(PKG_CONFIG_STEM);
        let content = toml::ser::to_string_pretty(self).map_err(|source| {
            ErrorInner::EmitPkgConfig { path: path.clone(), source }
        })?;
        fs::write(&path, content).await.map_err(|source| {
            ErrorInner::WritePkgConfig { path: path.clone(), source }
        })?;
        Ok(())
    }

    pub fn id(&self) -> PkgId {
        PkgId::from_parts(PkgIdPartsRef::Minimal(self.name.as_ref()))
    }
}
