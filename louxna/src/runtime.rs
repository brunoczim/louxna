use std::path::PathBuf;

use mlua::{Lua, LuaOptions};
pub use mlua::{StdLib, Value};

use tokio::fs;

use crate::{
    Error,
    error::ErrorInner,
    id::module::{ModId, ModIdPartsRef, ModPath},
    module::Module,
    package::PkgConfig,
};

#[derive(Debug)]
pub struct Runtime {
    lua: Lua,
    main_module: Module,
}

impl Runtime {
    pub async fn new(root_dir: impl Into<PathBuf>) -> Result<Self, Error> {
        let root_dir = root_dir.into();

        let lua_stdlib = StdLib::JIT
            | StdLib::TABLE
            | StdLib::STRING
            | StdLib::MATH
            | StdLib::PACKAGE
            | StdLib::BIT;
        let lua = Lua::new_with(lua_stdlib, LuaOptions::new())
            .map_err(ErrorInner::InitLua)?;

        let pkg_config = PkgConfig::load(&root_dir).await?;
        let main_path = root_dir.join("src/init.lua");
        let main_data = fs::read(&main_path).await.map_err(|source| {
            ErrorInner::ReadLuaMod { path: main_path.clone(), source }
        })?;
        let main_mod_path = ModPath::ROOT;
        let main_id = ModId::from_parts(ModIdPartsRef::External {
            pkg_id: pkg_config.id().as_ref(),
            mod_path: main_mod_path,
        });
        let main_module =
            Module { id: main_id, path: main_path, content: main_data };

        Ok(Self { lua, main_module })
    }

    pub fn run(&self) -> Result<(), Error> {
        self.lua
            .load(&self.main_module)
            .exec()
            .map_err(|source| ErrorInner::EvalLuaMod { source })?;
        Ok(())
    }
}
