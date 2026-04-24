use mlua::{Lua, UserData};

use crate::mainapi::SharedEngineAPI;

pub trait LuaType: UserData {
    fn make_global_table(_: &Lua, _: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        return Result::Ok(());
    }
    fn wipe(_: &Lua) -> mlua::Result<()> {
        return mlua::Result::Ok(());
    }
    fn make_doc() -> String {
        return String::new();
    }
}
