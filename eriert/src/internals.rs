use std::{path::Path, sync::Arc};

use mlua::{FromLuaMulti, Lua};

use eriert_api::{
    luaapi::{
        Api,
        LuaType
    },
    mainapi::{
        FSHandler,
        SharedEngineAPI
    }
};
use crate::{
    bundler::{execute_builder, execute_bundler}, enginestate::EngineState, projecthandler::{
        create_new_project,
        execute_project
    }, runexec::run_archive
};

fn display_help() -> mlua::Result<()> {
    let help = include_str!("eriert.md");
    println!("{}", help);
    return Result::Ok(());
}

fn display_version() -> mlua::Result<()> {
    println!("ErieRT Version: 1.0.0");
    return Result::Ok(());
}

pub async fn execute_command(lua: Arc<Lua>, args: Vec<String>) -> mlua::Result<()> {
    let cmd = args.get(1);
    let file = args.get(2);
    let cmd = cmd.map(|s| s.as_str());

    return match (cmd, file) {
        (Option::Some("doc"), Option::Some(file)) => {
            Api::save_doc(file.as_str()).map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        },
        (Option::Some("run"), Option::Some(file)) => {
            let engine_state: SharedEngineAPI = SharedEngineAPI::create(EngineState::new(FSHandler::Normal, false));
            run_script::<()>(lua, file, engine_state).await
        },
        (Option::Some("help"), _) => display_help(),
        (Option::Some("version"), _) => display_version(),
        (Option::Some("new"), file) => create_new_project(file),
        (Option::Some("runproj"), file) => {
            let engine_state: SharedEngineAPI = SharedEngineAPI::create(EngineState::new(FSHandler::Normal, false));
            execute_project(lua, file, engine_state).await
        },
        (Option::Some("exec"), Option::Some(file)) => run_archive(lua, file, false).await,
        (Option::Some("pack"), Option::Some(file)) => {
            execute_bundler(file, Option::None, Option::None).await
        },
        (Option::Some("build"), Option::Some(file)) => {
            execute_builder(file).await
        },
        (Option::Some(cmd), Option::Some(file)) => {
            println!("Invalid options given. Use `eriert help` for list of options.");
            println!("CMD: {}, FILE: {}", cmd, file);
            Result::Ok(())
        },
        _ => {
            let exec = args.get(0).expect("Couldn't get first argument!");

            let path: &Path = exec.as_ref();

            let file_name = path.file_name().expect("File ends in \"..\" for some reason!");

            let file_name = file_name.to_str().expect("Unable to convert &OsStr to &str");

            let file_name = format!("{}.ertpk", file_name);

            if std::fs::exists(&file_name).expect("File couldn't be validated!") {
                run_archive(lua, &file_name, true).await
            }
            else {
                println!("Invalid options given. Use `eriert help` for list of options.");
                Result::Ok(())
            }
        }
    };
}

pub async fn run_script<T: FromLuaMulti>(
    lua: Arc<mlua::Lua>, 
    script_name: &String, 
    engine_state: SharedEngineAPI
) -> mlua::Result<T> {
    let lua_ref = &*lua;

    let res = Api::make_global_table(lua_ref, Option::None, engine_state.clone());

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let res = engine_state.read_to_string(script_name)
        .map_err(|e| mlua::Error::RuntimeError(e.to_string()));

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let script = res.unwrap();

    let chunk = lua.load(script).set_name(script_name);

    let res = chunk.eval_async().await;

    return res;
}
