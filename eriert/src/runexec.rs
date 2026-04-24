use std::{fs::File, path::Path, sync::Arc};

use eriert_api::mainapi::{FSHandler, SharedEngineAPI};
use mlua::Lua;
use zip::ZipArchive;

use crate::{
    enginestate::EngineState, 
    projecthandler::execute_project
};

pub async fn run_archive(lua: Arc<Lua>, file: &String, is_app: bool) -> mlua::Result<()> {
    let res = File::open(file);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let archive_file = res.unwrap();

    let res = ZipArchive::new(archive_file);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let archive = res.unwrap();

    let fshandler = FSHandler::Archive(archive);

    let engine_state = EngineState::new(fshandler, is_app);

    let engine_api = SharedEngineAPI::create(engine_state);

    let path: &Path = file.as_ref();

    let file_name = path.file_prefix().expect("File ends in \"..\" for some reason!");

    let file_name = file_name.to_str().expect("Unable to convert &OsStr to &str");

    let file_name = format!("res://{}.json", file_name);

    let res = execute_project(lua, Option::Some(&file_name), engine_api).await;

    return res;
}