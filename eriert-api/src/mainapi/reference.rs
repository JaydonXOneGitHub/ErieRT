use mlua::UserDataRef;

use crate::luaapi::*;

pub enum Reference {
    Server(UserDataRef<Server>),
    FileContents(UserDataRef<LuaFileContents>),
    Listener(UserDataRef<Listener>),
    Reloadable(UserDataRef<Reloadable>),
    WebRequest(UserDataRef<WebRequest>),
    WebRequestMetadata(UserDataRef<WebRequestMetadata>),
    BlockingWebRequest(UserDataRef<BlockingWebRequest>),
    ApiResult(UserDataRef<ApiResult>),
    Promise(UserDataRef<Promise>),
    Match(UserDataRef<Match>),
    Redirect(UserDataRef<LuaRedirect>),
    Response(UserDataRef<LuaResponse>),
    StateType(UserDataRef<StateType>),
    Nil
}