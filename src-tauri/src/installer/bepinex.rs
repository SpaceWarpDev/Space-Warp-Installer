use crate::{
    installer::bepinex_loader::BepInExLoaderInstallManager, releases::get_latest_release_zips,
};
use std::{
    fs::{self, File},
    io,
    path::PathBuf,
};

pub struct BepInExInstallManager {
    pub ksp2_install_path: PathBuf,
    pub zip_url: Option<String>,
}

impl BepInExInstallManager {
    pub fn new(ksp2_install_path: PathBuf) -> Self {
        return BepInExInstallManager {
            ksp2_install_path,
            zip_url: None,
        };
    }

    pub async fn resolve(&mut self) -> Result<(), String> {
        let latest_release = get_latest_release_zips().await;

        if latest_release.bepinex.is_some() {
            self.zip_url = latest_release.bepinex;
        } else {
            return Err("No BepInEx release found!".to_string());
        }

        return Ok(());
    }

    pub async fn download(&mut self) -> Result<(), String> {
        if !self.ksp2_install_path.is_dir() {
            return Err("KSP2 install path is not a directory!".to_string());
        }

        if self.zip_url.clone().is_none() {
            self.resolve().await?;

            if self.zip_url.clone().is_none() {
                return Err("No valid SpaceWarp release found!".to_string());
            }
        }

        let files_in_dir = self.ksp2_install_path.read_dir().unwrap();

        let mut bepinex_installed = false;

        for file in files_in_dir {
            let file = file.unwrap();
            let file_name = file.file_name().into_string().unwrap();

            if file_name.contains("BepInEx") {
                bepinex_installed = true;
            }

            if file_name.contains("SpaceWarp") {
                return Err("SpaceWarp or another mod loader is already installed!".to_string());
            }
        }

        if !bepinex_installed {
            let mut bepinex = BepInExLoaderInstallManager::new(self.ksp2_install_path.clone());

            bepinex.download().await?;
        }

        let download_url = self
            .zip_url
            .clone()
            .expect("No valid SpaceWarp release found!");

        println!("Downloading from URL: {}", download_url);

        let response = reqwest::get(download_url)
            .await
            .expect("Could not download the SpaceWarp release!");

        let body = response
            .bytes()
            .await
            .expect("Could not read the SpaceWarp release!");

        let mut out_file = File::create(self.ksp2_install_path.join(".spacewarp_release.zip"))
            .expect("Could not create the SpaceWarp release file!");

        io::copy(&mut body.as_ref(), &mut out_file)
            .expect("Could not copy the SpaceWarp release to the file!");

        zip_extensions::read::zip_extract(
            &PathBuf::from(&self.ksp2_install_path.join(".spacewarp_release.zip")),
            &PathBuf::from(&self.ksp2_install_path),
        )
        .expect("Could not extract the SpaceWarp release!");

        fs::remove_file(&self.ksp2_install_path.join(".spacewarp_release.zip"))
            .expect("Could not delete the SpaceWarp release file!");
        
        if !&self.ksp2_install_path.join("SpaceWarp").join("Mods").exists() {
            fs::create_dir(&self.ksp2_install_path.join("SpaceWarp").join("Mods"))
                .expect("Could not create the Mods directory!");
        }

        return Ok(());
    }

    pub fn uninstall(&mut self) {
        fs::remove_dir_all(&self.ksp2_install_path.join("SpaceWarp"))
            .expect("Could not delete the SpaceWarp directory!");
        fs::remove_dir_all(&self.ksp2_install_path.join("BepInEx"))
            .expect("Could not delete the BepInEx directory!");

        fs::remove_file(&self.ksp2_install_path.join("winhttp.dll"))
            .expect("Could not delete the winhttp.dll file!");
        fs::remove_file(&self.ksp2_install_path.join("doorstop_config.ini"))
            .expect("Could not delete the doorstop_config.ini file!");
    }
}
