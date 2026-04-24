use std::{error::Error, fs::File, io::Read, path::Path};

use mlua::UserData;
use zip::ZipArchive;

pub const VIRTUAL_FS_PREFIX: &str = "res://";

pub enum FSHandler {
    Archive(ZipArchive<File>),
    Normal
}

impl FSHandler {
    pub fn read<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<u8>, Box<dyn Error>> {
        let path_ref = path.as_ref();

        if path_ref.starts_with(VIRTUAL_FS_PREFIX) {
            return self.read_from_virtual_fs(path);
        }
        
        return std::fs::read(path).map_err(Box::from);
    }

    fn read_from_virtual_fs<P: AsRef<Path>>(&mut self, path: P) -> Result<Vec<u8>, Box<dyn Error>> {
        let res = path
            .as_ref()
            .as_os_str()
            .to_str()
            .ok_or_else(|| mlua::Error::RuntimeError(
                "Path could not become String!".into()
            ));

        if let Result::Err(err) = res {
            return Result::Err(Box::from(err));
        }

        let raw = res.unwrap();

        if !raw.starts_with(VIRTUAL_FS_PREFIX) {
            return Result::Err(Box::from(mlua::Error::RuntimeError(
                "Path specified is not a virtual resource path (starting with \"res://\")!".into()
            )));
        }

        let true_path = &raw[VIRTUAL_FS_PREFIX.len()..];

        match self {
            Self::Archive(archive) => {
                let true_path = format!("res/{}", true_path);
                
                let res = archive.by_path(&true_path)
                    .map_err(Box::from)
                    .map(|mut value| {
                        let mut buffer: Vec<u8> = Vec::new();
                        return value.read_to_end(&mut buffer).map(|_| buffer);
                    });

                return match res {
                    Result::Ok(res) => res.map_err(Box::from),
                    Result::Err(err) => Result::Err(err)
                };
            },
            _ => {
                return std::fs::read(true_path).map_err(Box::from);
            }
        }
    }

    pub fn read_to_string<P: AsRef<Path>>(&mut self, path: P) -> Result<String, Box<dyn Error>> {
        let res = self.read(path);

        if let Result::Err(err) = res {
            return Result::Err(err);
        }

        let contents = res.unwrap();

        return String::from_utf8(contents).map_err(Box::from);
    }
}

impl UserData for FSHandler {

}
