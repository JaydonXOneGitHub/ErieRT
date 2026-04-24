use std::path::Path;

use mlua::UserData;

use crate::{luaapi::LuaType, macros::macros::{add_docs, wipe_type}, mainapi::{FileContents, SharedEngineAPI}};

#[derive(Clone)]
pub struct LuaFileContents {
    pub inner: Option<FileContents>
}

impl LuaFileContents {
    pub fn read(path: impl AsRef<Path>, engine_api: SharedEngineAPI) -> Self {
        return Self {
            inner: Option::Some(FileContents::read(path, engine_api))
        };
    }

    fn get_proper_string(path: mlua::Value) -> mlua::Result<String> {
        if !path.is_string() {
            return Result::Err(mlua::Error::RuntimeError(
                format!("Argument \'{}\' is not of type \"{}\"!", "path", "String")
            ));
        }

        let res = path.as_string().unwrap().to_str();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let path = res.ok().unwrap().to_string();

        return Result::Ok(path);
    }

    pub fn lua_read(path: mlua::Value, engine_api: SharedEngineAPI) -> mlua::Result<Self> {
        let res = Self::get_proper_string(path);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let path = res.unwrap();

        return Result::Ok(Self::read(&path, engine_api));
    }

    pub fn lua_read_as_bytes(path: mlua::Value, engine_api: SharedEngineAPI) -> mlua::Result<Self> {
        let res = Self::get_proper_string(path);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let path = res.unwrap();

        let res = engine_api.read(&path);

        if let Result::Err(err) = res {
            return Result::Err(mlua::Error::RuntimeError(err.to_string()));
        }

        let bytes = res.unwrap();

        return Result::Ok(Self {
            inner: Option::Some(FileContents::Bytes(bytes))
        });
    }

    pub fn lua_read_as_string(path: mlua::Value, engine_api: SharedEngineAPI) -> mlua::Result<Self> {
        let res = Self::get_proper_string(path);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let path = res.unwrap();

        let res = engine_api.read_to_string(&path);

        if let Result::Err(err) = res {
            return Result::Err(mlua::Error::RuntimeError(err.to_string()));
        }

        let plain_text = res.unwrap();

        return Result::Ok(Self {
            inner: Option::Some(FileContents::Plain(plain_text))
        });
    }
}

impl LuaFileContents {
    pub fn as_mut_self(&mut self) -> &mut Self {
        return self;
    }
}

impl UserData for LuaFileContents {}

impl LuaType for LuaFileContents {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, engine_api: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let engine_api_2 = engine_api.clone();
        let res = lua.create_function(move |_, path: mlua::Value| {
            return Self::lua_read(path, engine_api_2.clone());
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("read", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let engine_api_2 = engine_api.clone();
        let res = lua.create_function(move |_, path: mlua::Value| {
            return Self::lua_read_as_bytes(path, engine_api_2.clone());
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("readAsBytes", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let engine_api_2 = engine_api.clone();
        let res = lua.create_function(move |_, path: mlua::Value| {
            return Self::lua_read_as_string(path, engine_api_2.clone());
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("readAsString", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("FileContents", table);
    }

    add_docs!(filecontents);

    wipe_type!(FileContents);
}
