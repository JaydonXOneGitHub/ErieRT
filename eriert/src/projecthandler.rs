use std::{path::Path, sync::Arc};

use eriert_api::mainapi::SharedEngineAPI;
use libloading::Library;
use mlua::Lua;

pub type ErieExtensionFunc = unsafe extern "C" fn(
    *mut mlua::ffi::lua_State,
    *const Arc<mlua::Lua>, 
    *const SharedEngineAPI
);

use crate::{bundler::{EXTENSION_DIR, README_FILE}, internals::run_script, projectdefinitions::Project};

pub fn create_new_project(file: Option<&String>) -> mlua::Result<()> {
    let lua_code = include_str!("../templates/template.lua");

    let res = std::fs::write("main.lua", lua_code);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let res = std::fs::create_dir(EXTENSION_DIR);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let readme = include_str!("../templates/README.txt");

    let res = std::fs::write(README_FILE, readme);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let project = include_str!("../templates/erieproj.json");

    let res = serde_json::from_str::<Project>(project);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let mut project = res.unwrap();

    project.name = file.clone()
        .map(|s| {
            let path: &Path = s.as_ref();
            let file_prefix = path.file_prefix().expect("Filename is invalid!");
            let file_name = file_prefix.to_str().expect("&OsStr to &str failed");
            String::from(file_name)
        })
        .unwrap_or(String::from("erieproj"));

    let res = serde_json::to_string_pretty(&project);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let project = res.unwrap();

    let res = std::fs::write(
        file.map(|s| s.clone())
            .unwrap_or(String::from("./erieproj.json")), 
        &project
    );

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    return Result::Ok(());
}

pub fn read_project(project_file: &String, engine_api: SharedEngineAPI) -> mlua::Result<Project> {
    let res = engine_api.read_to_string(project_file);

    let res = res.map(|content| json5::from_str(&content))
        .map_err(|e| mlua::Error::RuntimeError(e.to_string()));

    return match res {
        Result::Ok(res) => {
            res.map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        },
        Result::Err(err) => Result::Err(err)
    };
}

pub async fn execute_project(lua: Arc<Lua>, project_file: Option<&String>, engine_api: SharedEngineAPI) -> mlua::Result<()> {
    let res = match project_file {
        Option::Some(project_file) => read_project(project_file, engine_api.clone()),
        Option::None => Result::Err(mlua::Error::RuntimeError("No project file given!".into()))
    };

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let project = res.ok().unwrap();

    return load_project(lua, project, engine_api).await;
}

pub async fn load_project(lua: Arc<Lua>, project: Project, engine_api: SharedEngineAPI) -> mlua::Result<()> {
    let libraries: Vec<(mlua::Result<Library>, String)> = project.extensions.iter().map(|extensions| {
        let lib = unsafe {
            let path = extensions
                .get_lib_path()
                .map(|s| s.clone())
                .unwrap_or(extensions.linux_path.clone());

            Library::new(&path)
                .map_err(|e| mlua::Error::RuntimeError(e.to_string()))
        };
        return (lib, extensions.entry.clone());
    }).collect();

    for (res, entry) in libraries.iter() {
        match res {
            Result::Ok(lib) => {
                let res = unsafe { lib.get::<ErieExtensionFunc>(format!("{}\0", entry).as_bytes()) };

                if let Result::Err(err) = res {
                    return Result::Err(mlua::Error::RuntimeError(err.to_string()));
                }

                let symbol = res.ok().unwrap();

                let lua_ptr = &*lua;

                let lua_ffi = lua.clone();
                let engine_api_ffi = engine_api.clone();

                let res: mlua::Result<()> = unsafe {
                    lua_ptr.exec_raw((), |lua| {
                        symbol(
                            lua,
                            &lua_ffi as *const Arc<mlua::Lua>,
                            &engine_api_ffi as *const SharedEngineAPI 
                        );
                    })
                };

                if let Result::Err(err) = res {
                    return Result::Err(err);
                }
            },
            Result::Err(err) => {
                return Result::Err(mlua::Error::RuntimeError(err.to_string()));
            }
        }
    }

    let res = run_script(lua, &project.exec.script, engine_api.clone()).await;
    drop(libraries);

    return res;
}
