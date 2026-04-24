use axum::response::Redirect;
use mlua::{Lua, UserData};

use crate::{luaapi::LuaType, macros::macros::{add_docs, custom_lua_error, wipe_type}};

pub struct LuaRedirect {
    pub inner: Option<Redirect>
}

impl LuaRedirect {
    pub fn to(uri: mlua::Value) -> mlua::Result<Self> {
        return Self::make(uri, Redirect::to);
    }

    pub fn temporary(uri: mlua::Value) -> mlua::Result<Self> {
        return Self::make(uri, Redirect::temporary);
    }

    pub fn permanent(uri: mlua::Value) -> mlua::Result<Self> {
        return Self::make(uri, Redirect::permanent);
    }

    fn make<P: FnOnce(&str) -> Redirect>(uri: mlua::Value, callback: P) -> mlua::Result<Self> {
        if !uri.is_string() {
            return custom_lua_error!("Argument \'uri\' must be of type \"String\"!".into())
        }

        let res = uri.as_string().unwrap().to_str();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let uri = res.unwrap().to_string();

        let redirect = callback(&uri);

        return Result::Ok(Self {
            inner: Option::Some(redirect)
        });
    }
}

impl UserData for LuaRedirect {}

impl LuaType for LuaRedirect {
    fn make_global_table(lua: &Lua, globals: Option<mlua::Table>, _: crate::mainapi::SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = lua.create_function(|_, uri: mlua::Value| {
            return Self::to(uri);
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("to", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = lua.create_function(|_, uri: mlua::Value| {
            return Self::temporary(uri);
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("temporary", func);
        
        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = lua.create_function(|_, uri: mlua::Value| {
            return Self::permanent(uri);
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("permanent", func);
        
        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("Redirect", table);
    }

    add_docs!(luaredirect);

    wipe_type!(LuaRedirect);
}
