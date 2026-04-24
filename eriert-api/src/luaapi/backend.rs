use std::time::Duration;

use mlua::UserData;

use crate::{luaapi::LuaType, macros::macros::add_docs, mainapi::SharedEngineAPI};

pub struct ErieRT;

impl ErieRT {
    fn set_timeout_internal(
        lua: &mlua::Lua,
        millisecond_time: mlua::Value,
        callback: mlua::Value,
        loop_amount: mlua::Value,
        wait: bool
    ) -> mlua::Result<()> {
        if !millisecond_time.is_integer() {
            return Result::Err(mlua::Error::RuntimeError(
                "Argument 'millisecondTime' is not of type \"integer\"!".into()
            ));
        }

        if !callback.is_function() {
            return Result::Err(mlua::Error::RuntimeError(
                "Argument 'callback' is not of type \"function\"!".into()
            ));
        }

        if !loop_amount.is_integer() && !loop_amount.is_nil() {
            return Result::Err(mlua::Error::RuntimeError(
                "Argument 'loop_amount' is not of type \"integer\"!".into()
            ));
        }

        let mut loop_amount = match loop_amount.as_i64() {
            Option::Some(time) => time.clone(),
            Option::None => 1
        };

        let res = lua.create_registry_value(callback);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let callback = res.unwrap();

        let lua = lua.clone();

        let thread = std::thread::spawn(move || {
            if loop_amount == 0 {
                loop {
                    let ms_time = millisecond_time.as_i64().unwrap() as u64;

                    std::thread::sleep(Duration::from_millis(ms_time));

                    let cb = lua.registry_value::<mlua::Function>(&callback).unwrap();

                    cb.call::<mlua::Value>(()).unwrap();
                }
            }

            while loop_amount > 0 {
                let ms_time = millisecond_time.as_i64().unwrap() as u64;

                std::thread::sleep(Duration::from_millis(ms_time));

                let cb = lua.registry_value::<mlua::Function>(&callback).unwrap();

                cb.call::<mlua::Value>(()).unwrap();

                loop_amount -= 1;
            }
        });

        if wait {
            let res = thread.join();

            if let Result::Err(_) = res {
                return Result::Err(mlua::Error::RuntimeError("Timeout panicked!".into()));
            }
        }

        return Result::Ok(());
    }

    pub async fn load(lua: mlua::Lua, path: mlua::Value, engine_api: SharedEngineAPI) -> mlua::Result<mlua::Value> {
        if !path.is_string() {
            return Result::Err(mlua::Error::RuntimeError(
                "Argument \'path\' is not of type \"String\"!".into()
            ));
        }

        let path = path.as_string().unwrap().to_string_lossy();

        let res = engine_api.read_to_string(&path);

        if let Result::Err(err) = res {
            return Result::Err(mlua::Error::RuntimeError(err.to_string()));
        }

        let contents = res.unwrap();

        let chunk = lua.load(contents);

        return chunk.eval_async().await;
    }

    pub fn set_timeout(
        lua: &mlua::Lua,
        millisecond_time: mlua::Value,
        callback: mlua::Value,
        loop_amount: mlua::Value
    ) -> mlua::Result<()> {
        return Self::set_timeout_internal(lua, millisecond_time, callback, loop_amount, false);
    }

    pub fn is_app(engine_api: SharedEngineAPI) -> mlua::Result<bool> {
        return engine_api.is_app().map_err(|err| mlua::Error::RuntimeError(err));
    }

    pub fn sleep(time_in_milliseconds: mlua::Value) -> mlua::Result<()> {
        if !time_in_milliseconds.is_integer() {
            return Result::Err(mlua::Error::RuntimeError(
                "Argument 'msTime' is not of type \"integer\"!".into()
            ));
        }

        let time_in_milliseconds = time_in_milliseconds.as_integer().unwrap();

        if time_in_milliseconds < 0 {
            return Result::Err(mlua::Error::RuntimeError(
                "Specified millisecond time is less than 0ms!".into()
            ));
        }

        std::thread::sleep(Duration::from_millis(time_in_milliseconds as u64));

        return Result::Ok(());
    }
}

impl UserData for ErieRT {

}

impl LuaType for ErieRT {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, engine_api: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = lua.create_function(|lua, (ms_time, callback, loop_amount): (mlua::Value, mlua::Value, mlua::Value)| {
            return Self::set_timeout(lua, ms_time, callback, loop_amount);
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("setTimeout", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = lua.create_function(|lua, ()| {
            return lua.gc_collect();
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("gcCollect", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = lua.create_function(|_, ms_time: mlua::Value| {
            return Self::sleep(ms_time);
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("sleep", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let engine_api_2 = engine_api.clone();
        let res = lua.create_function(move |_, ()| {
            let engine_api_2 = engine_api_2.clone();
            return Self::is_app(engine_api_2);
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("isApp", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let api = engine_api.clone();

        let res = lua.create_async_function(move |lua, path: mlua::Value| {
            let api = api.clone();
            async move {
                let api = api.clone();
                return Self::load(lua, path, api).await
            }
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("load", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let args: Vec<String> = std::env::args().collect();

        let res = table.set("args", args);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("ErieRT", table);
    }

    add_docs!(eriert);

    fn wipe(lua: &mlua::Lua) -> mlua::Result<()> {
        return lua.globals().set("ErieRT", mlua::Value::Nil);
    }
}
