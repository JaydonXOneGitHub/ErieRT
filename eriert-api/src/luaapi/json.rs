use mlua::{IntoLua, UserData};

use crate::{luaapi::{LuaType, convert_to_json, convert_to_lua}, macros::macros::{add_docs, wipe_type}, mainapi::SharedEngineAPI};

pub struct Json;

impl Json {
    pub fn as_string(lua: &mlua::Lua, value: mlua::Value) -> mlua::Result<mlua::Value> {
        let res = convert_to_json(lua, value);

        let res = res.map(|json| serde_json::to_string::<serde_json::Value>(&json).expect("Failed to serialize!"));

        let res = res.map(|contents| contents.into_lua(lua));

        return match res {
            Result::Ok(res2) => {
                match res2 {
                    Result::Ok(value) => Result::Ok(value),
                    Result::Err(err) => Result::Err(err)
                }
            },
            Result::Err(err) => Result::Err(err)
        };
    }

    pub fn as_value(lua: &mlua::Lua, value: mlua::Value) -> mlua::Result<mlua::Value> {
        if !value.is_string() {
            return Result::Err(mlua::Error::RuntimeError("value is not a string".into()))
        }

        let res = value.as_string().unwrap().to_str();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let string = res.unwrap().to_string();

        let res = serde_json::from_str::<serde_json::Value>(&string);

        if let Result::Err(err) = res {
            return Result::Err(mlua::Error::RuntimeError(err.to_string()));
        }

        let value = res.unwrap();

        return convert_to_lua(lua, value);
    }

    pub fn jsonify_string(lua: &mlua::Lua, input_string: impl std::fmt::Display) -> mlua::Result<mlua::Value> {
        return format!("\"{}\"", input_string).into_lua(lua);
    }

    pub fn internal_jsonify_string(input_string: impl std::fmt::Display) -> String {
        return format!("\"{}\"", input_string);
    }
}

impl UserData for Json {}

impl LuaType for Json {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = lua.create_function(Self::as_string);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let as_string = res.unwrap();

        let res = table.set("asString", as_string);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = lua.create_function(Self::as_value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let as_value = res.unwrap();

        let res = table.set("asValue", as_value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = lua.create_function(|lua, value: mlua::Value| {
            let opt = value.as_string();

            if opt.is_none() {
                return Result::Err(mlua::Error::RuntimeError("Argument 'value' is not \"String\"!".into()));
            }

            let s = opt.unwrap();

            let res = s.to_str();

            if let Result::Err(err) = res {
                return Result::Err(err);
            }

            let display = res.unwrap().to_string();

            return Self::jsonify_string(lua, display);
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let as_value = res.unwrap();

        let res = table.set("jsonifyString", as_value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("Json", table);
    }

    add_docs!(json);

    wipe_type!(Json);
}
