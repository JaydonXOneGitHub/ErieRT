use mlua::{Lua, UserData};

pub trait SharedStateLuaType<T: Clone>: UserData {
    fn make_global_table(_: &Lua, _state: T) -> mlua::Result<()> {
        return Result::Ok(());
    }
    fn wipe(_: &Lua) -> mlua::Result<()> {
        return mlua::Result::Ok(());
    }
    fn make_doc() -> String {
        return String::new();
    }
}
