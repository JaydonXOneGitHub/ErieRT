use mlua::UserData;

use crate::{luaapi::LuaType, macros::macros::{add_docs, custom_lua_error, wipe_type}, mainapi::SharedEngineAPI};

pub struct Reloadable {
    path: String,
    engine_api: SharedEngineAPI
}

impl Reloadable {
    pub async fn reload(&self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let res = self.engine_api.read_to_string(&self.path);

        if let Result::Err(err) = res {
            return custom_lua_error!(err.to_string());
        }

        let contents = res.unwrap();

        let chunk = lua.load(contents);

        return chunk.eval_async().await;
    }
}

impl UserData for Reloadable {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_async_method("reload", |lua, this, ()| async move {
            return this.reload(&lua).await
        });
    }
}

impl LuaType for Reloadable {
    fn make_global_table(
        lua: &mlua::Lua, 
        globals: Option<mlua::Table>, 
        engine_api: crate::mainapi::SharedEngineAPI
    ) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let reloadable = res.unwrap();

        let engine_api_2 = engine_api.clone();

        let res = lua.create_function(move |_, path: mlua::Value| {
            if !path.is_string() {
                return custom_lua_error!(
                    "Argument \'path\' is not of type \"String\"!".into()
                );
            }

            let path = path.as_string().unwrap().to_string_lossy();

            return Result::Ok(Reloadable { 
                path: path,
                engine_api: engine_api_2.clone()
            });
        });

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = reloadable.set("load", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("Reloadable", reloadable);
    }

    add_docs!(reloadable);

    wipe_type!(Reloadable);
}