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
    Function as LuaFunction
};

use crate::luaapi::{
    LuaType,
    convert_response_to_lua
};
use crate::macros::macros::{add_docs, wipe_type};
use crate::mainapi::SharedEngineAPI;

pub enum WebRequestType {
    Get,
    Put,
    Post,
    Delete,
    Patch,
    Head,
    Invalid
}

impl From<&str> for WebRequestType {
    fn from(value: &str) -> Self {
        let value: Self = if value.starts_with("GET") {
            Self::Get
        } else if value.starts_with("POST") {
            Self::Post
        } else if value.starts_with("PUT") {
            Self::Put
        } else if value.starts_with("PATCH") {
            Self::Patch
        } else if value.starts_with("DELETE") {
            Self::Delete
        } else if value.starts_with("HEAD") {
            Self::Head
        } else {
            Self::Invalid
        };

        return value;
    }
}

pub struct WebRequest {
    on_resolve: Option<LuaValue>,
    on_error: Option<LuaValue>,
    resolve_callback: Option<LuaFunction>,
    error_callback: Option<LuaFunction>,
    url: LuaValue,
    metadata: LuaValue,
    body: LuaValue,
}

impl WebRequest {
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
            on_resolve: Option::None,
            on_error: Option::None,
            resolve_callback: Option::None,
            error_callback: Option::None,
            url: url,
            metadata: metadata,
            body: body
        });
    }
}

impl WebRequest {
    pub fn on_resolve(&mut self, resolve_callback: LuaFunction) -> LuaResult<Self> {
        self.resolve_callback = resolve_callback.into();
        return Result::Ok(self.clone());
    }

    pub fn on_error(&mut self, error_callback: LuaFunction) -> LuaResult<Self> {
        self.error_callback = error_callback.into();
        return Result::Ok(self.clone());
    }

    pub fn pull(&self, lua: &Lua) -> LuaResult<()> {
        if !self.url.is_string() {
            return Result::Err(LuaError::RuntimeError(
                "Property \'url\' isn't of type \"String\".".into()
            ));
        }

        let promise_copy: Self = self.clone();
        let lua: Lua = lua.clone();

        tokio::task::spawn(async move {
            let res = promise_copy.make_request(lua).await;

            if let Result::Err(err) = res {
                println!("Error: {}", err);
            }
        });

        return Result::Ok(());
    }

    async fn make_request(self, lua: Lua) -> Result<(), String> {
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

        let request = res.ok().unwrap();

        let res = client.execute(request).await;

        return match res {
            Result::Err(err) => self.on_received_error(lua, Option::None, err).await,
            Result::Ok(response) => self.redirect(lua, response).await
        };
    }

    async fn redirect(self, lua: Lua, response: reqwest::Response) -> Result<(), String> {
        return if response.status().is_success() {
            self.on_received_success(lua, response).await
        } else {
            self.on_received_error(lua, response, Option::None).await
        };
    }

    async fn on_received_success(self, lua: Lua, response: reqwest::Response) -> Result<(), String> {
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

        match self.resolve_callback {
            Option::Some(callback) => {
                let res = callback.call::<LuaValue>(table);

                if let Result::Err(err) = res {
                    return Result::Err(err.to_string());
                }
            },
            Option::None => {
                println!("No resolve callback provided.");
            }
        };

        return Result::Ok(());
    }

    async fn on_received_error(
        self, lua: Lua,
        response: impl Into<Option<reqwest::Response>>,
        err: impl Into<Option<reqwest::Error>>
    ) -> Result<(), String> {
        let response = response.into();
        let err = err.into();

        if response.is_none() && err.is_some() {
            return self.execute_true_error(lua, err.unwrap());
        }

        if response.is_some() && err.is_none() {
            return self.return_http_code(lua, response.unwrap()).await;
        }

        return Result::Err("Clashing state detected.".into());
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

    async fn return_http_code(self, lua: Lua, response: reqwest::Response) -> Result<(), String> {
        let code = response.status().as_u16() as i64;

        let res = convert_response_to_lua(&lua, response).await;

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let value = res.ok().unwrap();

        let res = Self::create_or_get_table(&lua, &value);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let table = res.unwrap();

        let res = table.set("code", code);

        if let Result::Err(err) = res {
            return Result::Err(format!("Couldn't set HTTP code: {}", err.to_string()));
        }

        match self.error_callback {
            Option::Some(callback) => {
                let callback_res = callback.call::<LuaValue>(table);

                if let Result::Err(err) = callback_res {
                    return Result::Err(format!("Couldn't execute callback: {}", err.to_string()));
                }
            }
            Option::None => {
                println!("No error callback provided.");
            }
        };

        return Result::Ok(());
    }

    fn execute_true_error(self, lua: Lua, err: reqwest::Error) -> Result<(), String> {
        let res2 = lua.create_table();

        if let Result::Err(err2) = res2 {
            return Result::Err(format!("Couldn't make error table: {}", err2.to_string()));
        }

        let err_table = res2.ok().unwrap();

        let res = err_table.set("request_error", err.to_string().into_lua(&lua).unwrap());

        if let Result::Err(err) = res {
            return Result::Err(format!("Couldn't make error table: {}", err.to_string()));
        }

        match self.error_callback {
            Option::Some(callback) => {
                let callback_res = callback.call::<LuaValue>(err_table);

                if let Result::Err(err) = callback_res {
                    return Result::Err(format!("Couldn't execute callback: {}", err.to_string()));
                }
            }
            Option::None => {
                println!("No error callback provided.");
            }
        };

        return Result::Ok(());
    }
}

impl Clone for WebRequest {
    fn clone(&self) -> Self {
        return Self {
            on_resolve: self.on_resolve.clone(),
            on_error: self.on_error.clone(),
            resolve_callback: self.resolve_callback.clone(),
            error_callback: self.error_callback.clone(),
            url: self.url.clone(),
            metadata: self.metadata.clone(),
            body: self.body.clone()
        };
    }
}

impl UserData for WebRequest {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("onResolve",
            |_: &Lua, this: &mut Self, resolve_callback: LuaFunction| {
               return this.on_resolve(resolve_callback);
            }
        );
        methods.add_method_mut("onError",
            |_: &Lua, this: &mut Self, error_callback: LuaFunction| {
               return this.on_error(error_callback);
            }
        );
        methods.add_method("pull",
            |lua: &Lua, this: &Self, ()| {
                return this.pull(lua);
            }
        );
    }
}

impl LuaType for WebRequest {
    fn make_global_table(lua: &Lua, globals: Option<LuaTable>, _: SharedEngineAPI) -> LuaResult<()> {
        let res = lua.create_table();

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

        return globals.set("WebRequest", table);
    }

    wipe_type!(WebRequest);

    add_docs!(webrequest);
}
