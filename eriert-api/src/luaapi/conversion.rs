use mlua::{IntoLua, Lua, Value as LuaValue};
use serde_json::{Map, Number, Value as JsonValue};

fn convert_to_lua_number(number: Number) -> mlua::Result<LuaValue> {
    return if number.is_f64() {
        Result::Ok(LuaValue::Number(number.as_f64().unwrap()))
    } else if number.is_i64() {
        Result::Ok(LuaValue::Integer(number.as_i64().unwrap()))
    } else {
        Result::Err(mlua::Error::RuntimeError("Number type invalid".into()))
    };
}

fn convert_to_lua_array(lua: &Lua, arr: Vec<JsonValue>) -> mlua::Result<LuaValue> {
    let res = lua.create_table();

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let table = res.ok().unwrap();

    for (i, item) in arr.into_iter().enumerate() {
        let res = convert_to_lua(lua, item);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let value = res.ok().unwrap();

        let res = table.set(i + 1, value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }
    }

    return Result::Ok(LuaValue::Table(table));
}

fn convert_to_lua_table(lua: &Lua, map: Map<String, JsonValue>) -> mlua::Result<LuaValue> {
    let res = lua.create_table();

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let table = res.ok().unwrap();

    for (k, v) in map {
        let res = convert_to_lua(lua, v);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let value = res.ok().unwrap();

        let res = table.set(k, value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }
    }

    return Result::Ok(LuaValue::Table(table));
}

pub fn convert_to_lua(lua: &Lua, json_value: JsonValue) -> mlua::Result<LuaValue> {
    return match json_value {
        JsonValue::Bool(bool) => Result::Ok(LuaValue::Boolean(bool)),
        JsonValue::String(str) => str.into_lua(lua),
        JsonValue::Number(number) => convert_to_lua_number(number),
        JsonValue::Array(arr) => convert_to_lua_array(lua, arr),
        JsonValue::Object(map) => convert_to_lua_table(lua, map),
        JsonValue::Null => Result::Ok(LuaValue::Nil)
    };
}

fn convert_table_to_json(lua: &Lua, table: mlua::Table) -> mlua::Result<JsonValue> {
    let mut map = serde_json::Map::new();

    for pair in table.pairs::<String, LuaValue>() {
        if let Result::Err(err) = pair {
            return Result::Err(err);
        }

        let (k, v) = pair.unwrap();

        let res = convert_to_json(lua, v);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        map.insert(k, res.unwrap());
    }

    Result::Ok(JsonValue::Object(map))
}

fn convert_lua_string_to_json(_: &Lua, lua_string: mlua::String) -> mlua::Result<JsonValue> {
    let res = lua_string.to_str();

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let str_slice = res.unwrap();

    return Result::Ok(JsonValue::String(str_slice.to_string()));
}

pub fn convert_to_json(lua: &Lua, json_value: LuaValue) -> mlua::Result<JsonValue> {
    return match json_value {
        LuaValue::Boolean(b) => Result::Ok(JsonValue::Bool(b)),
        LuaValue::Integer(i) => Result::Ok(JsonValue::Number(Number::from_i128(i as i128).unwrap())),
        LuaValue::Number(n) => Result::Ok(JsonValue::Number(Number::from_f64(n).unwrap())),
        LuaValue::String(s) => convert_lua_string_to_json(lua, s),
        LuaValue::Table(t) => convert_table_to_json(lua, t),
        _ => Result::Ok(JsonValue::Null)
    };
}

pub async fn convert_response_to_lua(lua: &Lua, response: reqwest::Response) -> Result<LuaValue, String> {
    let res: reqwest::Result<JsonValue> = response.json().await;

    if let Result::Err(err) = res {
        return Result::Err(format!("Couldn't deserialize JSON: {}", err.to_string()));
    }

    let value: JsonValue = res.ok().unwrap();

    let res = convert_to_lua(&lua, value);

    return res.map_err(|e| e.to_string());
}

pub fn convert_response_to_lua_blocking(lua: &Lua, response: reqwest::blocking::Response) -> Result<LuaValue, String> {
    let res: reqwest::Result<JsonValue> = response.json();

    if let Result::Err(err) = res {
        return Result::Err(format!("Couldn't deserialize JSON: {}", err.to_string()));
    }

    let value: JsonValue = res.ok().unwrap();

    let res = convert_to_lua(&lua, value);

    return res.map_err(|e| e.to_string());
}
