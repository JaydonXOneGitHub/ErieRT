use mlua::UserData;
use tokio::net::{TcpListener, ToSocketAddrs};

use crate::{luaapi::LuaType, macros::macros::{add_docs, wipe_type}, mainapi::SharedEngineAPI};

pub struct Listener {
    inner: Option<TcpListener>
}

impl Listener {
    pub fn new<A: ToSocketAddrs>(address: A) -> Result<Self, mlua::Error> {
        let handle = tokio::runtime::Handle::current();

        let res = tokio::task::block_in_place(move || {
            return handle.block_on(async {
                return TcpListener::bind(address).await;
            });
        });

        if let Result::Err(err) = res {
            return Result::Err(mlua::Error::RuntimeError(err.to_string()));
        }

        return Result::Ok(Self {
            inner: Option::Some(res.ok().unwrap())
        });
    }

    pub fn lua_new(_: &mlua::Lua, address: mlua::Value) -> Result<Self, mlua::Error> {
        if !address.is_string() {
            return Result::Err(mlua::Error::RuntimeError(
                "Argument \'address\' is not of type \"String\"!".into()
            ));
        }

        let res = address.as_string().unwrap().to_str();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let address_value = res.ok().unwrap();

        let s = address_value.to_string();

        return Self::new(&s);
    }
}

impl Listener {
    pub fn take_listener(&mut self) -> Option<TcpListener> {
        return self.inner.take();
    }

    pub fn as_self(&self) -> &Self {
        return self;
    }

    pub fn as_mut_self(&mut self) -> &mut Self {
        return self;
    }
}

impl UserData for Listener {

}

impl LuaType for Listener {
    fn make_global_table(lua: &mlua::Lua, globals: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = lua.create_function(Self::lua_new);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let func = res.unwrap();

        let res = table.set("new", func);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let globals = globals.unwrap_or(lua.globals().clone());

        return globals.set("Listener", table);
    }

    add_docs!(listener);

    wipe_type!(Listener);
}
