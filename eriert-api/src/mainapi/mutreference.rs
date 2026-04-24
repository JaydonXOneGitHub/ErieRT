use mlua::UserDataRefMut;

use crate::luaapi::*;

pub enum MutReference {
    Server(UserDataRefMut<Server>),
    FileContents(UserDataRefMut<LuaFileContents>),
    Listener(UserDataRefMut<Listener>),
    Reloadable(UserDataRefMut<Reloadable>),
    WebRequest(UserDataRefMut<WebRequest>),
    WebRequestMetadata(UserDataRefMut<WebRequestMetadata>),
    BlockingWebRequest(UserDataRefMut<BlockingWebRequest>),
    ApiResult(UserDataRefMut<ApiResult>),
    Promise(UserDataRefMut<Promise>),
    Match(UserDataRefMut<Match>),
    Redirect(UserDataRefMut<LuaRedirect>),
    Response(UserDataRefMut<LuaResponse>),
    StateType(UserDataRefMut<StateType>),
    Nil
}