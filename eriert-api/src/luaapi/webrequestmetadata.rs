use mlua::UserData;

use crate::{luaapi::LuaType, macros::macros::{add_docs, wipe_type}, mainapi::SharedEngineAPI};

pub struct WebRequestMetadata;

impl WebRequestMetadata {
    fn get(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        return Self::make(lua, "GET");
    }

    fn post(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        return Self::make(lua, "POST");
    }

    fn put(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        return Self::make(lua, "PUT");
    }

    fn patch(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        return Self::make(lua, "PATCH");
    }

    fn delete(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        return Self::make(lua, "DELETE");
    }

    fn head(lua: &mlua::Lua) -> mlua::Result<mlua::Function> {
        return Self::make(lua, "HEAD");
    }

    fn make(lua: &mlua::Lua, http_method: &str) -> mlua::Result<mlua::Function> {
        let res = lua.create_string(http_method);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let http_method: mlua::Value = mlua::Value::String(res.ok().unwrap());

        return lua.create_function(move |lua, headers: mlua::Value| {
            let res = lua.create_table();

            if let Result::Err(err) = res {
                return Result::Err(err);
            }

            let table = res.ok().unwrap();

            let res = table.set("http_method", http_method.clone());

            if let Result::Err(err) = res {
                return Result::Err(err);
            }

            let res = table.set("headers", headers);

            if let Result::Err(err) = res {
                return Result::Err(err);
            }

            return Result::Ok(table);
        });
    }
}

impl UserData for WebRequestMetadata {}

impl LuaType for WebRequestMetadata {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.ok().unwrap();

        let res = Self::get(lua);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.ok().unwrap();

        let res = table.set("get", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = Self::put(lua);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.ok().unwrap();

        let res = table.set("put", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = Self::patch(lua);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.ok().unwrap();

        let res = table.set("patch", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = Self::post(lua);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.ok().unwrap();

        let res = table.set("post", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = Self::delete(lua);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.ok().unwrap();

        let res = table.set("delete", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = Self::head(lua);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.ok().unwrap();

        let res = table.set("head", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("WebRequestMetadata", table);
    }

    wipe_type!(WebRequestMetadata);

    add_docs!(webrequestmetadata);
}
