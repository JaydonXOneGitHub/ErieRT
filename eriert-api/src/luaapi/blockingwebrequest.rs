use mlua::{
    IntoLua,
    Lua,
    UserData
};
use mlua::{
    Value as LuaValue,
    Table as LuaTable,
    Error as LuaError,
    Result as LuaResult,
    String as LuaString,
};

use crate::luaapi::{ApiResult, WebRequestType, convert_response_to_lua, LuaType};
use crate::macros::macros::{add_docs, wipe_type};
use crate::mainapi::SharedEngineAPI;

pub struct BlockingWebRequest {
    url: LuaValue,
    metadata: LuaValue,
    body: LuaValue,
}

impl BlockingWebRequest {
    pub fn make(lua: &Lua, url: LuaValue, metadata: LuaValue, body: LuaValue) -> LuaResult<Self> {
        if !url.is_string() {
            return Result::Err(LuaError::RuntimeError(
                "Argument 'url' isn't of type \"String\".".into()
            ));
        }

        let table: Option<&LuaTable> = metadata.as_table();

        if table.is_none() {
            return Result::Err(LuaError::RuntimeError(
                "Argument 'metadata' isn't of type \"table\".".into()
            ));
        }

        let table: &LuaTable = table.unwrap();

        let res = table.contains_key("http_method");

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let is_table = res.ok().unwrap();

        if !is_table {
            return Result::Err(LuaError::RuntimeError(
                "Metadata doesn't have property \"http_method\".".into()
            ));
        }

        let method: LuaValue = table.get("http_method").unwrap();

        if !method.is_string() {
            return Result::Err(LuaError::RuntimeError(
                "Property 'http_method' isn't of type \"String\".".into()
            ));
        }

        let body: LuaValue = if body.is_nil() {
            "{}".into_lua(lua).expect("String conversion failure!")
        } else { body };

        return Result::Ok(Self {
            url: url,
            metadata: metadata,
            body: body
        });
    }
}

impl BlockingWebRequest {
    pub fn pull(&self, lua: &Lua) -> LuaResult<ApiResult> {
        if !self.url.is_string() {
            return Result::Err(LuaError::RuntimeError(
                "Property \'url\' isn't of type \"String\".".into()
            ));
        }

        let promise_copy: Self = self.clone();
        let lua: Lua = lua.clone();

        eprintln!("Starting request!");

        let handle = tokio::runtime::Handle::current();

        let res = tokio::task::block_in_place(move || {
            return handle.block_on(async {
                eprintln!("Thread started");
                let result = promise_copy.make_request(lua).await;
                eprintln!("make_request returned");
                return result;
            });
        });

        eprintln!("Request wrapped up!");

        return res.map_err(|err| LuaError::RuntimeError(err));
    }

    async fn make_request(self, lua: Lua) -> Result<ApiResult, String> {
        let client = reqwest::Client::new();

        let http_method: String = {
            let metadata = self.metadata.as_table().unwrap();

            metadata.get::<LuaString>("http_method")
                .unwrap()
                .to_string_lossy()
        };

        let string_url: String = self.url.as_string().unwrap().to_string_lossy();

        let request_type: WebRequestType = WebRequestType::from(http_method.as_str());

        let mut builder = match request_type {
            WebRequestType::Get => client.get(string_url.as_str()),
            WebRequestType::Put => client.put(string_url.as_str()),
            WebRequestType::Post => client.post(string_url.as_str()),
            WebRequestType::Patch => client.patch(string_url.as_str()),
            WebRequestType::Delete => client.delete(string_url.as_str()),
            WebRequestType::Head => client.head(string_url.as_str()),
            _ => {
                return Result::Err(format!("Invalid header type: {}", http_method));
            }
        };

        let metadata = self.metadata.as_table().unwrap();

        let headers = match metadata.get::<LuaTable>("headers") {
            Result::Ok(headers) => headers,
            Result::Err(_) => {
                let res = lua.create_table();

                if let Result::Err(err) = res {
                    return Result::Err(err.to_string());
                }

                let headers = res.ok().unwrap();

                let res = metadata.set("headers", headers.clone());

                if let Result::Err(err) = res {
                    return Result::Err(err.to_string());
                }

                headers
            }
        };

        for pair in headers.pairs() {
            if let Result::Err(err) = pair {
                return Result::Err(format!("Error: {}", err.to_string()));
            }

            let (k, v): (LuaString, LuaString) = pair.unwrap();

            builder = builder.header(
                k.to_string_lossy(),
                v.to_string_lossy()
            );
        }

        let res = self.body.as_string().unwrap().to_str();

        if let Result::Err(err) = res {
            return Result::Err(err.to_string());
        }

        let string = res.unwrap().to_string();

        let res = serde_json::from_str::<serde_json::Value>(&string);

        if let Result::Err(err) = res {
            return Result::Err(err.to_string());
        }

        let value = res.unwrap();

        let res = builder
            .json(&value)
            .build();

        if let Result::Err(err) = res {
            return Result::Err(format!("Error: {}", err.to_string()));
        }

        let request: reqwest::Request = res.ok().unwrap();

        let res = client.execute(request).await;

        return match res {
            Result::Err(err) => Result::Err(err.to_string()),
            Result::Ok(response) => self.redirect(lua, response).await
        };
    }

    async fn redirect(self, lua: Lua, response: reqwest::Response) -> Result<ApiResult, String> {
        return if response.status().is_success() {
            self.on_received_success(lua, response).await
        } else {
            self.on_received_error(lua, response).await
        };
    }

    async fn on_received_success(self, lua: Lua, response: reqwest::Response) -> Result<ApiResult, String> {
        let code = response.status().as_u16() as i64;

        let res = convert_response_to_lua(&lua, response).await;

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let value = res.ok().unwrap();

        print!("{}", value.type_name());

        let res = Self::create_or_get_table(&lua, &value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = table.set("code", code);

        if let Result::Err(err) = res {
            return Result::Err(format!("Couldn't set HTTP code: {}", err.to_string()));
        }

        return Result::Ok(ApiResult::OK(LuaValue::Table(table)));
    }

    async fn on_received_error(
        self, lua: Lua,
        response: reqwest::Response
    ) -> Result<ApiResult, String> {
        let code = response.status().as_u16() as i64;

        let res = convert_response_to_lua(&lua, response).await;

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let value = res.ok().unwrap();

        print!("{}", value.type_name());

        let res = Self::create_or_get_table(&lua, &value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = table.set("code", code);

        if let Result::Err(err) = res {
            return Result::Err(format!("Couldn't set HTTP code: {}", err.to_string()));
        }

        return Result::Ok(ApiResult::Error(LuaValue::Table(table)));
    }

    fn create_or_get_table(lua: &Lua, value: &LuaValue) -> Result<LuaTable, String> {
        return match value.as_table() {
            Option::Some(table) => Result::Ok(table.clone()),
            Option::None => {
                let res = lua.create_table();

                if let Result::Err(err) = res {
                    return Result::Err(err.to_string());
                }

                let t = res.ok().unwrap();

                let res = t.set("value", value.clone());

                if let Result::Err(err) = res {
                    return Result::Err(err.to_string());
                }

                Result::Ok(t)
            }
        };
    }
}

impl Clone for BlockingWebRequest {
    fn clone(&self) -> Self {
        return Self {
            url: self.url.clone(),
            metadata: self.metadata.clone(),
            body: self.body.clone()
        };
    }
}

impl UserData for BlockingWebRequest {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method("pull",
            |lua: &Lua, this: &Self, ()| {
                return this.pull(lua);
            }
        );
    }
}

impl LuaType for BlockingWebRequest {
    fn make_global_table(lua: &Lua, globals: Option<mlua::Table>, _: SharedEngineAPI) -> LuaResult<()> {
        let res = lua.create_table();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.ok().unwrap();

        let res = lua.create_function(|lua, (url, metadata, body): (LuaValue, LuaValue, LuaValue)| {
            return Self::make(lua, url, metadata, body);
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

        return globals.set("BlockingWebRequest", table);
    }

    wipe_type!(BlockingWebRequest);

    add_docs!(blockingwebrequest);
}
