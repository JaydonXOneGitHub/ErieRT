#[macro_export]
macro_rules! make_extension {
    ($entry:ident) => {
        paste::paste! {
            #[unsafe(no_mangle)]
            pub extern "C" fn [<$entry>](
                _: *mut mlua::ffi::lua_State,
                lua_ptr: *const std::sync::Arc<mlua::Lua>, 
                engine_api_ptr: *const eriert_api::mainapi::SharedEngineAPI
            ) {

                let lua = unsafe {
                    let lua_ref = lua_ptr.as_ref().unwrap();
                    lua_ref.clone()
                };

                let engine_api = unsafe {
                    let engine_api_ref = engine_api_ptr.as_ref().unwrap();
                    engine_api_ref.clone()
                };

                let res = [<$entry _main>](lua, engine_api);

                if let Result::Err(err) = res {
                    eprintln!("Error: {}", err.to_string());
                }
            }
        }
    };
}