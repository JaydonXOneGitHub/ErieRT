use std::sync::Arc;

use eriemacros::make_extension;
use eriert_api::mainapi::SharedEngineAPI;

make_extension!(entry);

pub fn entry_main(_lua: Arc<mlua::Lua>, _engine_api: SharedEngineAPI) -> mlua::Result<()> {
    return Result::Ok(());
}