pub mod macros {
    macro_rules! make_handler_from_endpoint {
        ($method:ident) => {
            paste::paste! {
                pub fn [<$method _from_endpoint>](
                    lua: &mlua::Lua, endpoint: Arc<ServerEndpoint>
                ) -> mlua::Result<MethodRouter> {
                    let lua = lua.clone();
                    let endpoint = endpoint.clone();
                    return Result::Ok(axum::routing::$method(move |req: Request<Body>| async move {
                        let res = request_to_state(req).await;

                        if let Result::Err(err) = res {
                            return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
                        }

                        return execute_state(
                            res.ok().unwrap(), 
                            lua.clone(),
                            endpoint.clone()
                        ).await;
                    }));
                }
            }
        };
    }

    macro_rules! add_docs {
        ($file_path:expr) => {
            paste::paste! {
                fn make_doc() -> String {
                    let doc: &str = include_str!(concat!("../luadocs/", stringify!($file_path), ".lua"));
                    return doc.into();
                }
            }
        };
    }

    macro_rules! wipe_type {
        ($type:ident) => {
            paste::paste! {
                fn wipe(lua: &mlua::Lua) -> mlua::Result<()> {
                    let identifier = stringify!($type);

                    let res = lua.globals().get::<mlua::Table>("ErieRT");

                    if let Result::Err(err) = res {
                        return Result::Err(err);
                    }

                    let eriert = res.unwrap();

                    return eriert.set(identifier, mlua::Value::Nil);
                }
            }
        };
    }

    macro_rules! custom_lua_error {
        ($error:expr) => {
            Result::Err(mlua::Error::RuntimeError($error))
        };
    }

    pub(crate) use make_handler_from_endpoint;
    pub(crate) use wipe_type;
    pub(crate) use custom_lua_error;
    pub(crate) use add_docs;
}