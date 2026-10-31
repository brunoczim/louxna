use std::path::PathBuf;

use mlua::{Lua, LuaOptions};
pub use mlua::{StdLib, Value};

use tokio::fs;

use crate::{Error, error::ErrorInner};

#[derive(Debug)]
pub struct Runtime {
    lua: Lua,
    main_data: Vec<u8>,
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

        let main_path = root_dir.join("src/init.lua");
        let main_data = fs::read(&main_path).await.map_err(|source| {
            ErrorInner::ReadLuaMod { path: main_path.clone(), source }
        })?;

        Ok(Self { lua, main_data })
    }

    pub fn run(&self) -> Result<(), Error> {
        self.lua
            .load(&self.main_data)
            .exec()
            .map_err(|source| ErrorInner::EvalLuaMod { source })?;
        Ok(())
    }
}
