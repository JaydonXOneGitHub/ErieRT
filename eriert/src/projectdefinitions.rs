use serde::{Deserialize, Serialize};

#[derive(Deserialize, Serialize)]
pub struct Project {
    pub name: String,
    pub exec: Exec,
    pub extensions: Vec<Extension>
}

#[derive(Deserialize, Serialize)]
pub struct Exec {
    pub script: String
}

#[derive(Deserialize, Serialize)]
pub struct Extension {
    pub windows_path: String,
    pub linux_path: String,
    pub macos_path: String,
    pub entry: String
}

impl Extension {
    pub fn get_lib_path(&self) -> Option<&String> {
        use os_info::get as os_info_get;
        use os_info::Type as OSType;

        let info = os_info_get();
        
        return match info.os_type() {
            OSType::Linux => Option::Some(&self.linux_path),
            OSType::Macos => Option::Some(&self.macos_path),
            OSType::Windows => Option::Some(&self.windows_path),
            _ => Option::None
        };
    }
}