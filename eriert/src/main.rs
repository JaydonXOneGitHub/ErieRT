#![cfg_attr(target_os = "windows", windows_subsystem = "windows")]

use std::sync::Arc;
use mlua::Lua;


mod internals;
mod projecthandler;
mod projectdefinitions;
mod bundler;
mod enginestate;
mod runexec;

#[tokio::main]
async fn main() -> mlua::Result<()> {
    let args: Vec<String> = std::env::args().collect();

    let lua = Arc::new(Lua::new());

    return internals::execute_command(lua, args).await;
}
