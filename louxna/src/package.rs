use std::{
    collections::{HashMap, hash_map},
    path::PathBuf,
};

use serde::{Deserialize, Serialize};
use tokio::fs;

use crate::{
    Error,
    error::ErrorInner,
    id::{
        module::{ModId, ModIdPartsRef, ModPath, ModPathBuf, ModSegment},
        package::{PkgId, PkgIdPartsRef, PkgName, PkgNamespace},
    },
    module::Module,
};

pub const PKG_CONFIG_STEM: &str = "pkg.lx.toml";

#[derive(
    Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize,
)]
#[serde(rename_all = "kebab-case")]
pub struct PkgConfig {
    pub name: Box<PkgName>,
    pub namespace: Box<PkgNamespace>,
    #[serde(
        default,
        skip_serializing_if = "PkgConfig::skip_ser_isolated_modules"
    )]
    pub isolated_modules: bool,
}

impl PkgConfig {
    fn skip_ser_isolated_modules(v: &bool) -> bool {
        *v == false
    }

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

    pub fn id(&self) -> Box<PkgId> {
        PkgId::from_parts(PkgIdPartsRef::Namespaced {
            namespace: &self.namespace,
            name: &self.name,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Package {
    pub config: PkgConfig,
    pub modules: HashMap<Box<ModPath>, Module>,
}

impl Package {
    pub async fn load(root_dir: impl Into<PathBuf>) -> Result<Self, Error> {
        #[derive(Debug, Clone, PartialEq)]
        enum Op {
            LeaveValid,
            LeaveInvalid,
            Visit(PathBuf),
            ReadDir(PathBuf),
        }

        let root_dir = root_dir.into();
        let config = PkgConfig::load(&*root_dir).await?;
        let mut this = Self { config, modules: HashMap::new() };
        let pkg_id = this.config.id();

        let mut src_dir = root_dir;
        src_dir.push("src");

        let mut module_prefix = ModPathBuf::new();
        let mut invalid_count = 0;
        let mut ops = vec![Op::ReadDir(src_dir)];
        let mut errors = Vec::<Error>::new();

        while let Some(op) = ops.pop() {
            match op {
                Op::LeaveValid => {
                    module_prefix.pop();
                },
                Op::LeaveInvalid => {
                    invalid_count -= 1;
                },
                Op::Visit(path) => {
                    let metadata = match fs::metadata(&path).await {
                        Ok(x) => x,
                        Err(source) => {
                            let error = ErrorInner::ReadLuaMod {
                                source,
                                path: path.clone(),
                            };
                            let error = error.into();
                            errors.push(error);
                            continue;
                        },
                    };

                    if metadata.is_dir() {
                        if path.extension().is_none()
                            && let Some(file_name) = path.file_name()
                            && let Some(name_as_str) = file_name.to_str()
                            && let Ok(seg) = ModSegment::new(name_as_str)
                        {
                            module_prefix.push(seg);
                            ops.push(Op::LeaveValid);
                        } else {
                            invalid_count += 1;
                            ops.push(Op::LeaveInvalid);
                        }

                        ops.push(Op::ReadDir(path));
                    } else {
                        if path.extension().is_none_or(|ext| ext != "lua") {
                            continue;
                        }

                        if invalid_count > 0 {
                            let error = ErrorInner::InvalidModOsPath {
                                path: path.to_owned(),
                            };
                            errors.push(error.into());
                            continue;
                        }

                        let module_segment = match path
                            .file_stem()
                            .and_then(|s| s.to_str())
                            .and_then(|s| ModSegment::new(s).ok())
                        {
                            Some(x) => x,
                            None => {
                                let error = ErrorInner::InvalidModOsPath {
                                    path: path.to_owned(),
                                };
                                errors.push(error.into());
                                continue;
                            },
                        };

                        let content = match fs::read(&path).await {
                            Ok(x) => x,
                            Err(source) => {
                                let error = ErrorInner::ReadLuaMod {
                                    source,
                                    path: path.clone(),
                                };
                                errors.push(error.into());
                                continue;
                            },
                        };

                        let mod_path = if module_segment == "init" {
                            module_prefix.clone()
                        } else {
                            module_prefix.join(module_segment)
                        };

                        let mod_id_parts = ModIdPartsRef::External {
                            mod_path: &mod_path,
                            pkg_id: &pkg_id,
                        };
                        let id = ModId::from_parts(mod_id_parts).to_owned();

                        let module = Module { path, content, id };

                        match this.modules.entry(mod_path.into()) {
                            hash_map::Entry::Occupied(_) => {
                                let error = ErrorInner::DuplicateModPath {
                                    mod_path: module
                                        .id
                                        .parts()
                                        .mod_path()
                                        .to_owned(),
                                    path: module.path.into(),
                                };
                                errors.push(error.into());
                            },
                            hash_map::Entry::Vacant(entry) => {
                                entry.insert(module);
                            },
                        }
                    }
                },

                Op::ReadDir(path) => {
                    let mut reader = match fs::read_dir(&path).await {
                        Ok(x) => x,
                        Err(source) => {
                            let error = ErrorInner::ReadLuaMod {
                                source,
                                path: path.clone(),
                            };
                            errors.push(error.into());
                            continue;
                        },
                    };

                    loop {
                        match reader.next_entry().await {
                            Ok(None) => break,
                            Ok(Some(entry)) => {
                                ops.push(Op::Visit(entry.path()));
                            },
                            Err(source) => {
                                let error = ErrorInner::ReadLuaMod {
                                    source,
                                    path: path.clone(),
                                };
                                errors.push(error.into());
                                break;
                            },
                        }
                    }
                },
            }
        }

        Error::multi(errors)?;

        Ok(this)
    }
}
