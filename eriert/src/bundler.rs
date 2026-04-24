use std::{error::Error, fs::File, io::Write, path::Path};

use eriert_api::mainapi::{FSHandler, SharedEngineAPI};
use fs_extra::dir::CopyOptions;
use zip::{CompressionMethod, ZipWriter, write::FileOptions};

use crate::{enginestate::EngineState, projecthandler::read_project};
use dir_tree_obj::{Directory, FileBuffer};

pub const BUILD_DIR: &str = "build";
pub const PACK_DIR: &str = "pack";
pub const ARCHIVE_ROOT_FOLDER: &str = "res/";
pub const EXTENSION_DIR: &str = "extension";
pub const README_FILE: &str = "README.txt";

fn build_zip<R: Default>(
    dir: &Directory, 
    path: Option<&str>, 
    zip: &mut ZipWriter<File>,
    compression_method: CompressionMethod
) -> Result<R, Box<dyn Error>> {
    let options: FileOptions<'_, ()> = FileOptions::default();
    let options: FileOptions<'_, ()> = options.compression_method(compression_method);
    match dir {
        Directory::File { name, contents } => {
            println!("File name: {}", name);

            let full_path = match path.clone() {
                Option::Some(path) => format!("{}/{}", path, name),
                Option::None => name.clone()
            };

            let opt = full_path.split_at_checked(ARCHIVE_ROOT_FOLDER.len());

            let (_, read_path) = opt.expect("File did not start with \"res/\"!");

            let read_path: String = read_path.into();

            let contents = match contents {
                Option::Some(contents) => Option::Some(
                    FileBuffer::make((*contents).clone())
                ),
                Option::None => {
                    println!("{}", full_path);
                    let res = std::fs::read(&read_path);
                    Option::Some(FileBuffer::make(res.unwrap()))
                }
            };

            let contents_ref = contents.as_ref();
            
            let res = zip.start_file(full_path, options);

            if let Result::Err(err) = res {
                return Result::Err(Box::from(err));
            }

            let res = zip.write_all(contents_ref.unwrap().as_slice());

            if let Result::Err(err) = res {
                return Result::Err(Box::from(err));
            }
        },
        Directory::Folder { name, children } => {
            println!("Folder name: {}", name);

            let full_path = match path.clone() {
                Option::Some(path) => format!("{}/{}", path, name),
                Option::None => name.clone()
            };

            let res = zip.add_directory(&full_path, options);

            if let Result::Err(err) = res {
                return Result::Err(Box::from(err));
            }

            let opt = Option::Some(full_path);

            let path = opt.as_ref().map(|s| s.as_str());

            for child in children {
                let res = build_zip::<R>(child, path, zip, compression_method);

                if let Result::Err(err) = res {
                    return Result::Err(Box::from(err));
                }
            }
        },
        _ => {}
    }
    return Result::Ok(R::default());
}

fn filter_dir(dir: &Directory) -> bool {
    match dir {
        Directory::Folder { name, children: _ } => {
            return name != BUILD_DIR && 
                name != EXTENSION_DIR &&
                name != PACK_DIR;
        },
        Directory::Unimplemented => {
            return false;
        },
        Directory::File { name, contents: _ } => {
            return !name.ends_with(".ertpk") && 
                name != "lua_doc.lua" &&
                name != "README.md";
        }
    }
}

pub async fn execute_bundler(
    project_file: &String,
    dir: Option<&str>,
    compression_method: Option<CompressionMethod>
) -> mlua::Result<()> {
    let dir = dir.unwrap_or(PACK_DIR);

    if std::fs::exists(dir).expect("Ouput folder could not be verified!") {
        std::fs::remove_dir_all(dir).expect("Ouput folder removal failed!");
    }

    let exec_path = std::env::current_dir().unwrap();

    let engine_api = SharedEngineAPI::create(EngineState::new(FSHandler::Normal, false));

    let res = read_project(project_file, engine_api.clone());

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let project = res.unwrap();

    let res = std::fs::create_dir(dir);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let package_name: String = format!("{}.ertpk", project.name);

    let package_full_path = format!("{}/{}", dir, package_name);

    let res = File::create(&package_full_path);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let package_file = res.unwrap();

    let res = std::fs::read_dir(&exec_path);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let res = Directory::from_read_dir(res.unwrap(), false);

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let mut directory: Directory = res.unwrap();

    match &mut directory {
        Directory::Folder { name, children: _ } => {
            *name = String::from("res");
        },
        _ => {}
    }

    let directory = directory.filter(filter_dir).expect("Root failed filter!");

    let mut zip = ZipWriter::new(package_file);

    let res: Result<(), Box<dyn Error>> = build_zip(
        &directory, Option::None, &mut zip,
        compression_method.unwrap_or(CompressionMethod::Stored)
    );

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let res = zip.finish();

    if let Result::Err(err) = res {
        return Result::Err(mlua::Error::RuntimeError(err.to_string()));
    }

    let _ = res.unwrap();

    let res = std::fs::exists("extension");

    if res.expect("Extension folder unable to be validated!") {
        let copy_options = CopyOptions::new();

        let res = fs_extra::dir::copy(EXTENSION_DIR, dir, &copy_options);

        res.expect("Extension folder copy failed!");
    }

    return Result::Ok(());
}


pub async fn execute_builder(project_file: &String) -> mlua::Result<()> {
    let res = execute_bundler(
        project_file,
        Option::Some(BUILD_DIR),
        Option::Some(CompressionMethod::Deflated)
    ).await;

    if let Result::Err(err) = res {
        return Result::Err(err);
    }

    let current_exe = std::env::current_exe().expect(
        "Could not get current executable!"
    );

    // Doing this because Windows uses.exe
    let project_name: &Path = project_file.as_ref();
    #[cfg(target_os = "windows")]
    let mut project_name = project_name.file_prefix()
        .expect("File name is absent")
        .to_str()
        .map(|path| {
            return format!("{}/{}", BUILD_DIR, path);
        })
        .expect("&OsStr to &str failed");
    
    // Doing this because Linux doesn't have an application extension.
    #[cfg(not(target_os = "windows"))]
    let project_name = project_name.file_prefix()
        .expect("File name is absent")
        .to_str()
        .map(|path| {
            return format!("{}/{}", BUILD_DIR, path);
        })
        .expect("&OsStr to &str failed");

    #[cfg(target_os = "windows")]
    project_name.push_str(".exe");

    let res = std::fs::copy(&current_exe, &project_name).map_err(|e| {
        return mlua::Error::RuntimeError(
            e.to_string()
        );
    })
    .map(|_| ());

    return res;
}