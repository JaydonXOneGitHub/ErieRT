use mlua::{
    UserData, Value as LuaValue
};

use crate::{luaapi::LuaType, macros::macros::{add_docs, wipe_type}, mainapi::SharedEngineAPI};

pub enum ApiResult {
    OK(LuaValue),
    Error(LuaValue)
}

impl UserData for ApiResult {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_function("ok", |_, value: LuaValue| {
            return Result::Ok(Self::OK(value));
        });
        methods.add_function("error", |_, error: LuaValue| {
            return Result::Ok(Self::Error(error));
        });
        methods.add_method("isOk", |_, this, ()| {
            return Result::Ok(match this {
                Self::OK(_) => true,
                _ => false
            });
        });
        methods.add_method("isError", |_, this, ()| {
            return Result::Ok(match this {
                Self::Error(_) => true,
                _ => false
            });
        });
        methods.add_method("getValue", |_, this, ()| {
            return Result::Ok(match this {
                Self::OK(value) => value.clone(),
                _ => LuaValue::Nil
            });
        });
        methods.add_method("getError", |_, this, ()| {
            return Result::Ok(match this {
                Self::Error(error) => error.clone(),
                _ => LuaValue::Nil
            });
        });
    }
}

impl LuaType for ApiResult {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = lua.create_function(|_, value: LuaValue| {
            return Result::Ok(Self::OK(value));
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("ok", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = lua.create_function(|_, error: LuaValue| {
            return Result::Ok(Self::Error(error));
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("error", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("Result", table);
    }

    wipe_type!(Result);

    add_docs!(apiresult);
}
