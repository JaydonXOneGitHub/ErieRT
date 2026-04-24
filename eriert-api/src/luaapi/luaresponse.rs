use axum::response::{IntoResponse, Redirect};
use mlua::UserData;
use reqwest::StatusCode;

use crate::{luaapi::{LuaFileContents, LuaRedirect, LuaType}, macros::macros::add_docs, mainapi::FileContents};

#[derive(Debug, Clone)]
pub enum LuaResponse {
    FileContents(FileContents),
    CodeAndBody(StatusCode, String),
    Redirect(Redirect),
    None
}

impl LuaResponse {
    fn assign_message(&mut self, table: &mlua::Table) -> mlua::Result<()> {
        let res = table.get::<i64>("code");

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let code = res.ok().unwrap();
        
        let res = table.get::<mlua::String>("message");

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let message = res.ok().unwrap();

        let res = message.to_str();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let message = res.ok().unwrap().to_string();

        let res = StatusCode::from_u16(code as u16);

        if let Result::Err(err) = res {
            return Result::Err(mlua::Error::RuntimeError(
                err.to_string()
            ));
        }

        let code = res.ok().unwrap();

        *self = Self::CodeAndBody(code, message);

        return Result::Ok(());
    }

    fn assign_json(&mut self, table: &mlua::Table) -> mlua::Result<()> {
        let res = table.get::<mlua::String>("json");

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let json = res.ok().unwrap();

        let res = json.to_str();

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let json = res.ok().unwrap().to_string();

        *self = Self::FileContents(FileContents::JSON(json));

        return Result::Ok(());
    }

    fn assign_from_table(&mut self, table: &mlua::Table) -> mlua::Result<()> {
        let res = table.contains_key("message");

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let has_message = res.unwrap();

        let res = table.contains_key("json");

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let has_json = res.unwrap();

        if has_message {
            let res = self.assign_message(table);

            if let Result::Err(err) = res {
                return Result::Err(err);
            }
        }      

        if has_json {
            let res = self.assign_json(table);

            if let Result::Err(err) = res {
                return Result::Err(err);
            }
        }        

        return Result::Ok(());
    }

    pub fn send(&mut self, value: mlua::Value) -> mlua::Result<()> {
        if value.is_userdata() {
            let userdata = value.as_userdata().unwrap();

            if let Result::Ok(mut fc) = userdata.borrow_mut::<LuaFileContents>() {
                match fc.inner.take() {
                    Option::Some(fc) => {
                        *self = Self::FileContents(fc);
                    },
                    _ => {}
                }
                
                return Result::Ok(());
            }

            if let Result::Ok(mut redirect) = userdata.borrow_mut::<LuaRedirect>() {
                match redirect.inner.take() {
                    Option::Some(redirect) => {
                        *self = Self::Redirect(redirect);
                    },
                    _ => {}
                }
                
                return Result::Ok(());
            }
        }
        else if value.is_table() {
            let table = value.as_table().unwrap();

            return self.assign_from_table(table);
        }
        return Result::Ok(());
    }
}

impl IntoResponse for LuaResponse {
    fn into_response(self) -> axum::response::Response {
        return match self {
            Self::FileContents(fc) => fc.into_response(),
            Self::CodeAndBody(code, body) => (code, body).into_response(),
            Self::Redirect(redirect) => redirect.into_response(),
            _ => {
                (
                    StatusCode::FORBIDDEN, 
                    String::from("Request unfulfilled!")
                ).into_response()
            }
        };
    }
}

impl UserData for LuaResponse {
    fn add_methods<M: mlua::UserDataMethods<Self>>(methods: &mut M) {
        methods.add_method_mut("send", |_, this, value: mlua::Value| {
            return this.send(value);
        });
    }
}

impl LuaType for LuaResponse {
    add_docs!(response);
}