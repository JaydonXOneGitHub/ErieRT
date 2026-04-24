use std::sync::Arc;

use axum::{Router, routing::MethodRouter};
use mlua::UserData;
use tokio::net::TcpListener;

use crate::{
    luaapi::{
        Listener,
        LuaType,
        StateType
    }, macros::macros::{add_docs, wipe_type}, mainapi::{
        SharedEngineAPI, delete_from_endpoint, get_from_endpoint, head_from_endpoint, patch_from_endpoint, post_from_endpoint, put_from_endpoint
    }
};

pub struct ServerEndpoint {
    pub endpoint: String,
    pub state_type: StateType,
    pub callback: mlua::RegistryKey
}

pub struct Server {
    router: Option<Router>,
    listener: Option<TcpListener>
}

impl Server {
    pub fn new(listener: &mut Listener) -> Self {
        return Self {
            router: Option::Some(Router::new()),
            listener: listener.take_listener()
        };
    }

    pub fn lua_new(_: &mlua::Lua, listener: mlua::Value) -> mlua::Result<Self> {
        if !listener.is_userdata() {
            return Result::Err(mlua::Error::RuntimeError(
                "Argument \'listener\' is not of type \"userdata\"!".into()
            ));
        }

        let userdata = listener.as_userdata().unwrap();

        let res = userdata.borrow_mut::<Listener>();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let mut listener = res.ok().unwrap();

        let me = Self::new(listener.as_mut_self());

        return Result::Ok(me);
    }
}

impl Server {
    fn handler(
        &mut self,
        lua: &mlua::Lua,
        endpoint: (mlua::Value, mlua::Value),
        callback: impl FnOnce(&mlua::Lua, Arc<ServerEndpoint>) -> mlua::Result<MethodRouter>
    ) -> mlua::Result<()> {
        let res = self.apply_method(lua, endpoint);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let endpoint = res.ok().unwrap();

        let endpoint = Arc::new(endpoint);

        let res = callback(lua, endpoint.clone());

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let method = res.unwrap();

        match self.router.take() {
            Option::Some(router) => {
                self.router = Option::Some(
                    router.route(&endpoint.endpoint, method)
                );
            },
            _ => {
                return Result::Err(mlua::Error::RuntimeError(
                    "Server's router has been used!".into()
                ));
            }
        }

        return Result::Ok(());
    }

    fn apply_method(
        &mut self, lua: &mlua::Lua,
        (endpoint, callback): (mlua::Value, mlua::Value)
    ) -> mlua::Result<ServerEndpoint> {
        if !endpoint.is_string() {
            return Result::Err(mlua::Error::RuntimeError(
                format!("Argument \'{}\' is not of type \"{}\"!", "endpoint", "String")
            ));
        }

        if !callback.is_function() {
            return Result::Err(mlua::Error::RuntimeError(
                format!("Argument \'{}\' is not of type \"{}\"!", "callback", "function")
            ));
        }

        let res = endpoint.as_string().unwrap().to_str();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let endpoint = res.ok().unwrap().to_string();
        let callback = callback.as_function().unwrap().clone();

        let res = lua.create_registry_value(callback);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        return Result::Ok(ServerEndpoint {
            endpoint: endpoint,
            state_type: StateType::Stateless,
            callback: res.ok().unwrap()
        });
    }
}

impl Server {
    pub fn get(
        &mut self,
        lua: &mlua::Lua,
        endpoint: (mlua::Value, mlua::Value)
    ) -> mlua::Result<()> {
        return self.handler(lua, endpoint, get_from_endpoint);
    }

    pub fn post(
        &mut self,
        lua: &mlua::Lua,
        endpoint: (mlua::Value, mlua::Value)
    ) -> mlua::Result<()> {
        return self.handler(lua, endpoint, post_from_endpoint);
    }

    pub fn head(
        &mut self,
        lua: &mlua::Lua,
        endpoint: (mlua::Value, mlua::Value)
    ) -> mlua::Result<()> {
        return self.handler(lua, endpoint, head_from_endpoint);
    }

    pub fn patch(
        &mut self,
        lua: &mlua::Lua,
        endpoint: (mlua::Value, mlua::Value)
    ) -> mlua::Result<()> {
        return self.handler(lua, endpoint, patch_from_endpoint);
    }

    pub fn delete(
        &mut self,
        lua: &mlua::Lua,
        endpoint: (mlua::Value, mlua::Value)
    ) -> mlua::Result<()> {
        return self.handler(lua, endpoint, delete_from_endpoint);
    }

    pub fn put(
        &mut self,
        lua: &mlua::Lua,
        endpoint: (mlua::Value, mlua::Value)
    ) -> mlua::Result<()> {
        return self.handler(lua, endpoint, put_from_endpoint);
    }

    pub async fn run(&mut self) -> mlua::Result<()> {
        let listener = self.listener.take();
        let router = self.router.take();

        if listener.is_none() || router.is_none() {
            return Result::Err(mlua::Error::RuntimeError(
                "Server instance already used!".into()
            ));
        }

        let res = axum::serve(listener.unwrap(), router.unwrap()).await;

        return res.map_err(|e| mlua::Error::RuntimeError(e.to_string()));
    }
}

impl UserData for Server {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("get", |lua, this, values: (mlua::Value, mlua::Value)| {
            return this.get(lua, values);
        });
        methods.add_method_mut("post", |lua, this, values: (mlua::Value, mlua::Value)| {
            return this.post(lua, values);
        });
        methods.add_method_mut("patch", |lua, this, values: (mlua::Value, mlua::Value)| {
            return this.patch(lua, values);
        });
        methods.add_method_mut("delete", |lua, this, values: (mlua::Value, mlua::Value)| {
            return this.delete(lua, values);
        });
        methods.add_method_mut("put", |lua, this, values: (mlua::Value, mlua::Value)| {
            return this.put(lua, values);
        });
        methods.add_method_mut("head", |lua, this, values: (mlua::Value, mlua::Value)| {
            return this.head(lua, values);
        });
        methods.add_async_method_mut("run", |_, mut this, ()| async move {
            return this.run().await;
        });
    }
}

impl LuaType for Server {
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

        return globals.set("Server", table);
    }

    add_docs!(server);

    wipe_type!(Server);
}
