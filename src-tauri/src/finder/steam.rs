use keyvalues_parser::Vdf;
use std::{fs, path::PathBuf};

pub const STEAM_CONFIG_DIR: &'static str = "C:\\Program Files (x86)\\Steam\\config";

#[derive(Default, Clone, Debug)]
pub struct LibraryFolders {
    pub paths: Vec<PathBuf>,
    pub discovered: bool,
}

impl LibraryFolders {
    pub fn new() -> Self {
        return Self {
            paths: vec![],
            discovered: false,
        };
    }

    pub fn discover(&mut self) -> Option<&mut Self> {
        let steam_config_dir = PathBuf::from(STEAM_CONFIG_DIR);
        let libraryfolders_vdf_path = steam_config_dir.join("libraryfolders.vdf");

        if libraryfolders_vdf_path.is_file() {
            let vdf_text = fs::read_to_string(&libraryfolders_vdf_path).ok()?;
            let value = Vdf::parse(&vdf_text).ok()?.value;
            let obj = value.get_obj()?;

            let library_folders: Vec<_> = obj
                .iter()
                .filter(|(key, values)| key.parse::<u32>().is_ok() && values.len() == 1)
                .filter_map(|(_, values)| {
                    let library_folder_string = values
                        .get(0)?
                        .get_obj()?
                        .get("path")?
                        .get(0)?
                        .get_str()?
                        .to_string();

                    let library_folder = PathBuf::from(library_folder_string).join("steamapps");

                    return Some(library_folder);
                })
                .collect();

            self.paths = library_folders;
        }

        self.discovered = true;

        return Some(self);
    }
}

pub struct SteamInstallFinder {
    pub library_folders: LibraryFolders,
}

impl SteamInstallFinder {
    pub fn new() -> Self {
        let mut me = Self {
            library_folders: LibraryFolders::new(),
        };

        me.library_folders
            .discover()
            .expect("Failed to discover Steam library folders!");

        return me;
    }

    pub fn find_ksp2_dir(&mut self) -> Option<PathBuf> {
        for library_folder in self.library_folders.clone().paths {
            let ksp2_dir = library_folder.join("common").join("Kerbal Space Program 2");

            if ksp2_dir.is_dir() {
                let dir_contents = fs::read_dir(&ksp2_dir).unwrap();
                let mut ksp2_executable_found = false;

                for file in dir_contents {
                    if file.is_ok() {
                        let file_path = file.unwrap().path();

                        if file_path.is_file()
                            && file_path
                                .as_os_str()
                                .to_str()
                                .unwrap()
                                .contains("KSP2_x64.exe")
                        {
                            ksp2_executable_found = true;
                            break;
                        }
                    }
                }

                if !ksp2_executable_found {
                    return None;
                }

                return Some(ksp2_dir);
            }
        }

        return None;
    }
}
