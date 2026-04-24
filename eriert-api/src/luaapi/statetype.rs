use mlua::UserData;

use crate::{luaapi::LuaType, macros::macros::{add_docs, wipe_type}, mainapi::SharedEngineAPI};

#[derive(Debug, Clone, Copy)]
pub enum StateType {
    Stateless,
    Body,
    Query,
    Headers,
    BodyAndHeaders,
    QueryAndHeaders,
}

impl UserData for StateType {

}

impl LuaType for StateType {
    fn make_global_table(lua: &mlua::Lua, table: Option<mlua::Table>, _: SharedEngineAPI) -> mlua::Result<()> {
        let res = match table.into() {
            Option::Some(table) => Result::Ok(table),
            _ => lua.create_table()
        };

        let table = res.ok().unwrap();

        let res = table.set("Stateless", Self::Stateless);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = table.set("Body", Self::Body);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let res = table.set("Query", Self::Query);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        return lua.globals().set("StateType", table);
    }

    wipe_type!(StateType);

    add_docs!(statetype);
}
