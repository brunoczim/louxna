use std::{collections::HashMap, path::PathBuf, sync::Mutex};

use mlua::{FromLuaMulti, Function, IntoLua, Lua, LuaOptions, Table};
pub use mlua::{MultiValue, StdLib, Value};

use crate::{
    Error,
    error::ErrorInner,
    id::{
        module::{ModId, ModIdPartsRef, ModPath},
        package::PkgId,
    },
    module::Module,
    package::{Package, PkgConfig},
};

#[derive(Debug)]
pub struct Runtime {
    _lua: Lua,
    require_fn: Function,
    main_pkg_config: PkgConfig,
}

impl Runtime {
    pub fn new(main_pkg: Package) -> Result<Self, Error> {
        let lua_stdlib = StdLib::JIT
            | StdLib::TABLE
            | StdLib::STRING
            | StdLib::MATH
            | StdLib::PACKAGE
            | StdLib::BIT;
        let lua = Lua::new_with(lua_stdlib, LuaOptions::new())
            .map_err(ErrorInner::InitLua)?;

        let mut require_fn_builder =
            RequireFnBuilder::new(main_pkg.config.id());
        for module in main_pkg.modules.into_values() {
            require_fn_builder.register(module)?;
        }

        let require_fn = require_fn_builder
            .with_isolated_modules(main_pkg.config.isolated_modules)
            .finish(&lua)?;

        lua.globals()
            .set("require", require_fn.clone())
            .map_err(|source| ErrorInner::SetLuaRequire { source })?;

        Ok(Self { _lua: lua, require_fn, main_pkg_config: main_pkg.config })
    }

    pub async fn load(root_dir: impl Into<PathBuf>) -> Result<Self, Error> {
        let main_pkg = Package::load(root_dir).await?;
        Self::new(main_pkg)
    }

    pub fn require_parametric<T>(
        &self,
        mod_id: &ModId,
        args: impl IntoLua,
    ) -> Result<T, Error>
    where
        T: FromLuaMulti,
    {
        let output: T = self
            .require_fn
            .call((mod_id, args))
            .map_err(|source| ErrorInner::CallLuaRequire { source })?;
        Ok(output)
    }

    pub fn require<T>(&self, mod_id: &ModId) -> Result<T, Error>
    where
        T: FromLuaMulti,
    {
        self.require_parametric(mod_id, Value::Nil)
    }

    pub fn run_as_binary(&self, args: Vec<String>) -> Result<i32, Error> {
        let mod_id = ModId::from_parts(ModIdPartsRef::External {
            pkg_id: &self.main_pkg_config.id(),
            mod_path: ModPath::ROOT,
        });
        let status: Option<i32> = self.require_parametric(&mod_id, args)?;
        Ok(status.unwrap_or(0))
    }
}

fn make_env(
    lua: &Lua,
    isolate: bool,
    arguments: Option<Table>,
) -> Result<Table, Error> {
    let env = lua
        .create_table()
        .map_err(|source| ErrorInner::FailedToCreateLuaEnv { source })?;
    env.set("arguments", arguments)
        .map_err(|source| ErrorInner::FailedToSetLuaEnvArguments { source })?;

    let metatable = lua
        .create_table()
        .map_err(|source| ErrorInner::FailedToCreateLuaEnvMeta { source })?;
    metatable
        .set("__index", lua.globals())
        .map_err(|source| ErrorInner::FailedToSetLuaEnvMetaIndex { source })?;
    if !isolate {
        metatable.set("__newindex", lua.globals()).map_err(|source| {
            ErrorInner::FailedToSetLuaEnvMetaNewIndex { source }
        })?;
    }
    env.set_metatable(Some(metatable))
        .map_err(|source| ErrorInner::FailedToSetLuaEnvMeta { source })?;

    Ok(env)
}

#[derive(Debug)]
struct RequireEntry {
    module: Module,
    loaded: Mutex<Option<Value>>,
}

#[derive(Debug)]
struct RequireFnBuilder {
    pkg_id: Box<PkgId>,
    isolate: bool,
    modules: HashMap<Box<ModPath>, RequireEntry>,
}

impl RequireFnBuilder {
    pub fn new(pkg_id: Box<PkgId>) -> Self {
        Self { pkg_id, isolate: false, modules: HashMap::new() }
    }

    pub fn with_isolated_modules(mut self, isolate: bool) -> Self {
        self.isolate = isolate;
        self
    }

    pub fn register(&mut self, module: Module) -> Result<(), Error> {
        self.modules.insert(
            module.id.parts().mod_path().to_owned(),
            RequireEntry { module, loaded: Mutex::new(None) },
        );
        Ok(())
    }

    pub fn finish(self, lua: &Lua) -> Result<Function, Error> {
        let closure = move |lua: &Lua,
                            (mod_id, arguments): (
            Box<ModId>,
            Option<Table>,
        )|
              -> Result<Value, mlua::Error> {
            let entry = match mod_id.parts() {
                ModIdPartsRef::External { pkg_id, mod_path } => self
                    .modules
                    .get(mod_path)
                    .filter(|_| pkg_id.compatible_with(&self.pkg_id))
                    .ok_or(ErrorInner::UnknownModule { mod_id })
                    .map_err(mlua::Error::external)?,
                ModIdPartsRef::Internal { mod_path } => self
                    .modules
                    .get(mod_path)
                    .ok_or(ErrorInner::UnknownModule { mod_id })
                    .map_err(mlua::Error::external)?,
            };
            let mut loaded = entry.loaded.lock().unwrap();
            let value = match loaded.clone() {
                Some(value) if arguments.is_none() => value,
                _ => {
                    let env = make_env(lua, self.isolate, arguments)
                        .map_err(mlua::Error::external)?;
                    let value: Value =
                        lua.load(&entry.module).set_environment(env).eval()?;
                    *loaded = Some(value.clone());
                    value
                },
            };
            Ok(value)
        };
        let function = lua
            .create_function(closure)
            .map_err(|source| ErrorInner::CreateLuaRequire { source })?;
        Ok(function)
    }
}

#[cfg(test)]
mod test {
    use std::collections::HashMap;

    use anyhow::Result;
    use mlua::Table;

    use crate::{
        id::{
            module::{ModId, ModPath},
            package::{PkgName, PkgNamespace},
        },
        module::Module,
        package::{Package, PkgConfig},
        runtime::Runtime,
    };

    fn make_package() -> Result<Package> {
        Ok(Package {
            config: PkgConfig {
                name: PkgName::new("foobar")?.to_owned(),
                namespace: PkgNamespace::new("owner")?.to_owned(),
                isolated_modules: false,
            },
            modules: {
                let mut mods = HashMap::new();
                mods.insert(
                    ModPath::ROOT.to_owned(),
                    Module {
                        path: "src/init.lua".into(),
                        id: ModId::new("owner/foobar::")?.to_owned(),
                        content: b"return arguments == nil".into(),
                    },
                );
                mods.insert(
                    ModPath::new("mymod")?.to_owned(),
                    Module {
                        path: "src/mymod.lua".into(),
                        id: ModId::new("owner/foobar::mymod")?.to_owned(),
                        content: b"return { x = 3 }".into(),
                    },
                );
                mods.insert(
                    ModPath::new("mymod.submod")?.to_owned(),
                    Module {
                        path: "src/mymod/submod.lua".into(),
                        id: ModId::new("owner/foobar::mymod.submod")?
                            .to_owned(),
                        content: b"return { name = 'bar' }".into(),
                    },
                );
                mods.insert(
                    ModPath::new("badrand")?.to_owned(),
                    Module {
                        path: "src/badrand.lua".into(),
                        id: ModId::new("owner/foobar::badrand")?.to_owned(),
                        content: b"return math.random(0, ~0 >> 1)".into(),
                    },
                );
                mods
            },
        })
    }

    #[test]
    fn require_root() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: bool =
            runtime.require(ModId::from_internal(ModPath::ROOT))?;
        assert_eq!(output, true);
        Ok(())
    }

    #[test]
    fn require_non_root() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: Table = runtime.require(ModId::new("mymod")?)?;
        let field: i32 = output.get("x")?;
        assert_eq!(field, 3);
        Ok(())
    }

    #[test]
    fn require_non_root_nested() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: Table = runtime.require(ModId::new("mymod.submod")?)?;
        let field: String = output.get("name")?;
        assert_eq!(field, "bar");
        Ok(())
    }

    #[test]
    fn require_root_external_minimal() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: bool = runtime.require(ModId::new("foobar::")?)?;
        assert_eq!(output, true);
        Ok(())
    }

    #[test]
    fn require_non_root_external_minimal() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: Table = runtime.require(ModId::new("foobar::mymod")?)?;
        let field: i32 = output.get("x")?;
        assert_eq!(field, 3);
        Ok(())
    }

    #[test]
    fn require_non_root_nested_external_minimal() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: Table =
            runtime.require(ModId::new("foobar::mymod.submod")?)?;
        let field: String = output.get("name")?;
        assert_eq!(field, "bar");
        Ok(())
    }

    #[test]
    fn require_root_external_namespaced() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: bool = runtime.require(ModId::new("owner/foobar::")?)?;
        assert_eq!(output, true);
        Ok(())
    }

    #[test]
    fn require_non_root_external_namespaced() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: Table =
            runtime.require(ModId::new("owner/foobar::mymod")?)?;
        let field: i32 = output.get("x")?;
        assert_eq!(field, 3);
        Ok(())
    }

    #[test]
    fn require_non_root_nested_external_namespaced() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: Table =
            runtime.require(ModId::new("owner/foobar::mymod.submod")?)?;
        let field: String = output.get("name")?;
        assert_eq!(field, "bar");
        Ok(())
    }

    #[test]
    fn require_root_few_args() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output: bool = runtime.require_parametric(
            ModId::from_internal(ModPath::ROOT),
            vec![1_i32, 2, 3],
        )?;
        assert_eq!(output, false);
        Ok(())
    }

    #[test]
    fn require_rand_no_args_caches() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output_first: i32 = runtime.require(ModId::new("badrand")?)?;
        let output_second: i32 = runtime.require(ModId::new("badrand")?)?;
        assert_eq!(output_first, output_second);
        Ok(())
    }

    #[test]
    fn require_root_works_diff_args_dont_cache() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output_first: i32 = runtime.require(ModId::new("badrand")?)?;
        let output_second: i32 = runtime
            .require_parametric(ModId::new("badrand")?, Vec::<i32>::new())?;
        assert_ne!(output_first, output_second);
        Ok(())
    }

    #[test]
    fn require_root_works_same_present_args_dont_cache() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output_first: i32 = runtime
            .require_parametric(ModId::new("badrand")?, Vec::<i32>::new())?;
        let output_second: i32 = runtime
            .require_parametric(ModId::new("badrand")?, Vec::<i32>::new())?;
        assert_ne!(output_first, output_second);
        Ok(())
    }

    #[test]
    fn require_root_works_no_args_after_args_uses_cache() -> Result<()> {
        let main_pkg = make_package()?;
        let runtime = Runtime::new(main_pkg)?;
        let output_first: i32 = runtime
            .require_parametric(ModId::new("badrand")?, Vec::<i32>::new())?;
        let output_second: i32 = runtime.require(ModId::new("badrand")?)?;
        assert_eq!(output_first, output_second);
        Ok(())
    }
}
