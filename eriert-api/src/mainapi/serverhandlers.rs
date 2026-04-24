use std::{sync::Arc};

use axum::{body::{Body, to_bytes}, extract::Request, http::HeaderMap, response::{IntoResponse, Response}, routing::MethodRouter};
use mlua::IntoLua;
use reqwest::{StatusCode};

use crate::{luaapi::{LuaResponse, ServerEndpoint, convert_to_lua}, macros::macros::make_handler_from_endpoint};

use serde_json::Value as JsonValue;

pub struct State {
    pub body: Option<JsonValue>,
    pub query: Option<JsonValue>,
    pub path: String,
    pub headers: Option<HeaderMap>
}

make_handler_from_endpoint!(get);
make_handler_from_endpoint!(post);
make_handler_from_endpoint!(delete);
make_handler_from_endpoint!(put);
make_handler_from_endpoint!(patch);
make_handler_from_endpoint!(head);

async fn request_to_state(req: Request<Body>) -> mlua::Result<State> {
    let headers = req.headers().clone();

    let query = req.uri().query();

    let path: String = req.uri().path().into();

    let query = query.map(|q| String::from(q));

    let query: Option<JsonValue> = serde_json::from_str(&query.unwrap_or_default()).ok();

    let body = req.into_body();

    let res = to_bytes(body, 1024 * 1024 * 10).await;

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let bytes = res.ok().unwrap();

    let res: Result<JsonValue, _> = serde_json::from_slice(&bytes);

    let body = match res {
        Ok(json) => Option::Some(json),
        Err(_) => Option::None
    };

    return Result::Ok(State {
        body: body,
        query: query,
        path: path.into(),
        headers: Option::Some(headers)
    });
}

async fn headers_to_lua(headers: Option<HeaderMap>, lua: &mlua::Lua) -> mlua::Result<mlua::Table> {
    let res = lua.create_table();

    if let Result::Err(err) = res {
        return Result::Err(err);
    }
    
    let lua_headers = res.ok().unwrap().to_owned();

    if let Option::Some(headers) = headers {
        for item in headers {
            match item.0 {
                Option::Some(header_name) => {
                    let res = item.1.to_str();

                    if let Result::Err(err) = res {
                        return Result::Err(
                            mlua::Error::RuntimeError(
                                err.to_string()
                            )
                        );
                    }

                    let res = lua_headers.set(header_name.as_str(), res.unwrap());

                    if let Result::Err(err) = res {
                        return Result::Err(err);
                    }
                },
                _ => {}
            }
        }
    }

    return Result::Ok(lua_headers);
}

async fn execute_state(
    state: State,
    lua: mlua::Lua,
    endpoint: Arc<ServerEndpoint>
) -> Response {
    let (body, query, headers, path) = 
        (state.body, state.query, state.headers, state.path);

    let lua_ref = &lua;
    
    let res = lua.create_table();

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let request = res.ok().unwrap().to_owned();

    let res = match body {
        Option::Some(body) => convert_to_lua(lua_ref, body),
        Option::None => Result::Ok(mlua::Value::Nil)
    };

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let body = res.unwrap();

    let res = match query {
        Option::Some(query) => convert_to_lua(lua_ref, query),
        Option::None => Result::Ok(mlua::Value::Nil)
    };

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let query = res.unwrap();

    let res = lua.create_string(path);

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let path = res.unwrap();

    let res = headers_to_lua(headers, lua_ref).await;

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let headers = res.unwrap();

    let res = request.set("query", query);

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let res = request.set("body", body);

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let res = request.set("headers", headers);

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let res = request.set("path", path);

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let res = LuaResponse::None.into_lua(&lua);

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let response = res.ok().unwrap().to_owned();

    let res = lua.registry_value::<mlua::Function>(&endpoint.callback);

    if let Result::Err(err) = res {
        return (StatusCode::BAD_REQUEST, err.to_string()).into_response();
    }

    let res = res.unwrap().call::<()>((request.clone(), response.clone()));

    return match res {
        Result::Ok(_) => {
            let r = response.as_userdata().unwrap().borrow::<LuaResponse>();
            let r = r.unwrap().clone();
            r.into_response()
        },
        Result::Err(err) => {
            (StatusCode::BAD_REQUEST, err.to_string()).into_response()
        }
    };
}