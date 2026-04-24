use std::{ops::{Deref}, path::Path, sync::{Arc, Mutex}};

use crate::mainapi::{MutReference, Reference};

pub trait EngineAPI: Send + Sync + 'static {
    fn read(&mut self, path: &Path) -> Result<Vec<u8>, String>;
    fn read_to_string(&mut self, path: &Path) -> Result<String, String>;
    fn is_app(&self) -> bool;
    fn get_userdata_ref(&self, userdata: &mlua::AnyUserData) 
        -> Result<Reference, mlua::Error>;
    fn get_userdata_mut(&self, userdata: &mlua::AnyUserData) 
        -> Result<MutReference, mlua::Error>;
}

pub struct SharedEngineAPI {
    inner: Arc<Mutex<dyn EngineAPI>>
}

impl SharedEngineAPI {
    pub fn create<E: EngineAPI>(engine_api: E) -> Self {
        return Self {
            inner: Arc::new(Mutex::from(engine_api))
        };
    }
}

impl SharedEngineAPI {
    pub fn read<P: AsRef<Path>>(&self, path: P) -> Result<Vec<u8>, String> {
        return match self.inner.lock() {
            Result::Ok(mut engine_api) => engine_api.read(path.as_ref()).map_err(|e| e.to_string()),
            Result::Err(err) => Result::Err(err.to_string())
        };
    }

    pub fn read_to_string<P: AsRef<Path>>(&self, path: P) -> Result<String, String> {
        return match self.inner.lock() {
            Result::Ok(mut engine_api) => engine_api.read_to_string(path.as_ref()).map_err(|e| e.to_string()),
            Result::Err(err) => Result::Err(err.to_string())
        };
    }

    pub fn is_app(&self) -> Result<bool, String> {
        return self.inner.lock()
            .map(|e| e.is_app())
            .map_err(|err| err.to_string());
    }

    pub fn get_userdata_ref(&self, userdata: &mlua::AnyUserData) -> mlua::Result<Reference> {
        return match self.inner.lock() {
            Result::Ok(engine_api) => engine_api.get_userdata_ref(userdata),
            Result::Err(err) => Result::Err(mlua::Error::RuntimeError(err.to_string()))
        };
    }

    pub fn get_userdata_mut(&self, userdata: &mlua::AnyUserData) -> mlua::Result<MutReference> {
        return match self.inner.lock() {
            Result::Ok(engine_api) => engine_api.get_userdata_mut(userdata),
            Result::Err(err) => Result::Err(mlua::Error::RuntimeError(err.to_string()))
        };
    }
}

impl Clone for SharedEngineAPI {
    fn clone(&self) -> Self {
        return Self {
            inner: self.inner.clone()
        };
    }
}

impl Deref for SharedEngineAPI {
    type Target = Mutex<dyn EngineAPI>;
    fn deref(&self) -> &Self::Target {
        return self.inner.deref();
    }
}
