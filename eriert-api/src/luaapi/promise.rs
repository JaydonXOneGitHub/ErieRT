use mlua::{
    Function as LuaFunction, UserData, Value as LuaValue
};

use crate::{luaapi::LuaType, macros::macros::{add_docs, wipe_type}, mainapi::SharedEngineAPI};

#[derive(Clone)]
pub struct Promise {
    callback: LuaFunction,
    success_callback: Option<LuaFunction>,
    error_callback: Option<LuaFunction>
}

impl Promise {
    pub fn make(callback: LuaFunction) -> Self {
        return Self {
            callback: callback,
            success_callback: Option::None,
            error_callback: Option::None
        };
    }
}

impl Promise {
    pub fn on_success(&mut self, success: LuaValue) -> mlua::Result<()> {
        if success.type_name() != "function" {
            return Result::Err(mlua::Error::RuntimeError(
                "Callback wasn't of type function".into()
            ));
        }

        self.success_callback = success
            .as_function()
            .unwrap()
            .clone()
            .into();

        return Result::Ok(());
    }

    pub fn on_error(&mut self, error: LuaValue) -> mlua::Result<()> {
        if error.type_name() != "function" {
            return Result::Err(mlua::Error::RuntimeError(
                "Callback wasn't of type function".into()
            ));
        }

        self.error_callback = error
            .as_function()
            .unwrap()
            .clone()
            .into();

        return Result::Ok(());
    }

    pub fn pull(&self, lua: &mlua::Lua) -> mlua::Result<()> {
        let lua = lua.clone();
        let this = self.clone();

        tokio::task::spawn_blocking(move || {
            let table = lua.create_table().unwrap();

            let res = this.callback.call::<LuaValue>(());

            match res {
                Result::Ok(ok) => {
                    match this.success_callback {
                        Option::Some(c) => {
                            table.set("value", ok).unwrap();
                            c.call::<()>(table).unwrap()
                        },
                        Option::None => {}
                    }
                },
                Result::Err(err) => {
                    match this.error_callback {
                        Option::Some(c) => {
                            table.set("msg", err.to_string()).unwrap();
                            c.call::<()>(table).unwrap()
                        },
                        Option::None => {}
                    }
                }
            }
        });

        return Result::Ok(());
    }
}

impl UserData for Promise {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("onSuccess", |_, this, success: LuaValue| {
            return this.on_success(success);
        });

        methods.add_method_mut("onError", |_, this, error: LuaValue| {
            return this.on_error(error);
        });

        methods.add_method("pull", |lua: &mlua::Lua, this, ()| {
            return this.pull(lua);
        });
    }
}

impl LuaType for Promise {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.ok().unwrap();

        let res = lua.create_function(|_, callback: LuaFunction| {
            return Result::Ok(Self::make(callback));
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.ok().unwrap();

        let res = table.set("make", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("Promise", table);
    }

    wipe_type!(Promise);

    add_docs!(promise);
}