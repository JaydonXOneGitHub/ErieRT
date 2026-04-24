use std::path::Path;

use mlua::UserData;

use crate::{luaapi::{ApiResult, BlockingWebRequest, ErieRT, Json, Listener, LuaFileContents, LuaRedirect, LuaResponse, LuaType, Match, Promise, Reloadable, Server, StateType, WebRequest, WebRequestMetadata}, mainapi::SharedEngineAPI};

pub struct Api;

impl UserData for Api {}

impl LuaType for Api {
    fn make_global_table(lua: &mlua::Lua, _: Option<mlua::Table>, engine_api: SharedEngineAPI) -> mlua::Result<()> {
        ErieRT::make_global_table(lua, Option::None, engine_api.clone())?;

        let res = lua.globals().get::<mlua::Table>("ErieRT");

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let eriert = Option::Some(res.unwrap());

        WebRequest::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        BlockingWebRequest::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        Promise::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        Match::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        WebRequestMetadata::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        Json::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        ApiResult::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        Listener::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        Server::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        StateType::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        LuaFileContents::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        Reloadable::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        LuaResponse::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        LuaRedirect::make_global_table(lua, eriert.clone(), engine_api.clone())?;
        
        return Result::Ok(());
    }

    fn wipe(lua: &mlua::Lua) -> mlua::Result<()> {
        WebRequest::wipe(lua)?;
        BlockingWebRequest::wipe(lua)?;
        WebRequestMetadata::wipe(lua)?;
        Promise::wipe(lua)?;
        Match::wipe(lua)?;
        WebRequestMetadata::wipe(lua)?;
        ApiResult::wipe(lua)?;
        Listener::wipe(lua)?;
        Server::wipe(lua)?;
        StateType::wipe(lua)?;
        LuaFileContents::wipe(lua)?;
        Reloadable::wipe(lua)?;
        LuaRedirect::wipe(lua)?;
        ErieRT::wipe(lua)?;
        return Result::Ok(());
    }

    fn make_doc() -> String {
        let docs = vec![
            WebRequest::make_doc(),
            Match::make_doc(),
            Promise::make_doc(),
            WebRequestMetadata::make_doc(),
            Json::make_doc(),
            BlockingWebRequest::make_doc(),
            ApiResult::make_doc(),
            Listener::make_doc(),
            Server::make_doc(),
            StateType::make_doc(),
            LuaFileContents::make_doc(),
            LuaResponse::make_doc(),
            Reloadable::make_doc(),
            LuaRedirect::make_doc(),
            ErieRT::make_doc(),
        ];
        return docs.join("\n\n\n\n\n");
    }
}

impl Api {
    pub fn save_doc(path: impl AsRef<Path>) -> std::io::Result<()> {
        let doc = Self::make_doc();
        return std::fs::write(path, doc);
    }
}
