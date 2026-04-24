use eriert_api::{luaapi::*, mainapi::{EngineAPI, FSHandler, MutReference, Reference}};

pub struct EngineState {
    fs_handler: FSHandler,
    is_app: bool
}

impl EngineState {
    pub fn new<FS: Into<FSHandler>>(fs_handler: FS, is_app: bool) -> Self {
        return Self {
            fs_handler: fs_handler.into(),
            is_app: is_app
        };
    }
}

impl EngineAPI for EngineState {
    fn read(&mut self, path: &std::path::Path) -> Result<Vec<u8>, String> {
        return self.fs_handler.read(path).map_err(|e| e.to_string());
    }

    fn read_to_string(&mut self, path: &std::path::Path) -> Result<String, String> {
        return self.fs_handler.read_to_string(path).map_err(|e| e.to_string());
    }

    fn is_app(&self) -> bool {
        return self.is_app;
    }

    fn get_userdata_ref(&self, userdata: &mlua::AnyUserData) 
        -> Result<Reference, mlua::Error> {
        if userdata.is::<LuaResponse>() {
            return userdata.borrow::<LuaResponse>()
                .map(|res| Reference::Response(res));
        }

        if userdata.is::<LuaRedirect>() {
            return userdata.borrow::<LuaRedirect>()
                .map(|redir| Reference::Redirect(redir));
        }

        if userdata.is::<LuaFileContents>() {
            return userdata.borrow::<LuaFileContents>()
                .map(|fc| Reference::FileContents(fc));
        }

        if userdata.is::<Server>() {
            return userdata.borrow::<Server>()
                .map(|server| Reference::Server(server));
        }

        if userdata.is::<Listener>() {
            return userdata.borrow::<Listener>()
                .map(|listener| Reference::Listener(listener));
        }

        if userdata.is::<Match>() {
            return userdata.borrow::<Match>()
                .map(|m| Reference::Match(m));
        }

        if userdata.is::<Promise>() {
            return userdata.borrow::<Promise>()
                .map(|p| Reference::Promise(p));
        }

        if userdata.is::<WebRequest>() {
            return userdata.borrow::<WebRequest>()
                .map(|wr| Reference::WebRequest(wr));
        }

        if userdata.is::<BlockingWebRequest>() {
            return userdata.borrow::<BlockingWebRequest>()
                .map(|wr| Reference::BlockingWebRequest(wr));
        }

        if userdata.is::<WebRequestMetadata>() {
            return userdata.borrow::<WebRequestMetadata>()
                .map(|wr| Reference::WebRequestMetadata(wr));
        }

        if userdata.is::<ApiResult>() {
            return userdata.borrow::<ApiResult>()
                .map(|ar| Reference::ApiResult(ar));
        }

        if userdata.is::<StateType>() {
            return userdata.borrow::<StateType>()
                .map(|st| Reference::StateType(st));
        }

        return Result::Ok(Reference::Nil);
    }

    fn get_userdata_mut(&self, userdata: &mlua::AnyUserData) 
        -> Result<MutReference, mlua::Error> {
        if userdata.is::<LuaResponse>() {
            return userdata.borrow_mut::<LuaResponse>()
                .map(|res| MutReference::Response(res));
        }

        if userdata.is::<LuaRedirect>() {
            return userdata.borrow_mut::<LuaRedirect>()
                .map(|redir| MutReference::Redirect(redir));
        }

        if userdata.is::<LuaFileContents>() {
            return userdata.borrow_mut::<LuaFileContents>()
                .map(|fc| MutReference::FileContents(fc));
        }

        if userdata.is::<Server>() {
            return userdata.borrow_mut::<Server>()
                .map(|server| MutReference::Server(server));
        }

        if userdata.is::<Listener>() {
            return userdata.borrow_mut::<Listener>()
                .map(|listener| MutReference::Listener(listener));
        }

        if userdata.is::<Match>() {
            return userdata.borrow_mut::<Match>()
                .map(|m| MutReference::Match(m));
        }

        if userdata.is::<Promise>() {
            return userdata.borrow_mut::<Promise>()
                .map(|p| MutReference::Promise(p));
        }

        if userdata.is::<WebRequest>() {
            return userdata.borrow_mut::<WebRequest>()
                .map(|wr| MutReference::WebRequest(wr));
        }

        if userdata.is::<BlockingWebRequest>() {
            return userdata.borrow_mut::<BlockingWebRequest>()
                .map(|wr| MutReference::BlockingWebRequest(wr));
        }

        if userdata.is::<WebRequestMetadata>() {
            return userdata.borrow_mut::<WebRequestMetadata>()
                .map(|wr| MutReference::WebRequestMetadata(wr));
        }

        if userdata.is::<ApiResult>() {
            return userdata.borrow_mut::<ApiResult>()
                .map(|ar| MutReference::ApiResult(ar));
        }

        if userdata.is::<StateType>() {
            return userdata.borrow_mut::<StateType>()
                .map(|st| MutReference::StateType(st));
        }

        return Result::Ok(MutReference::Nil);
    }
}
