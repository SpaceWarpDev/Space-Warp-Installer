use crate::models::release::{ReleaseResponse, ReleaseZips};
use reqwest::Client;

pub const LATEST_RELEASE_URL: &'static str =
    "https://api.github.com/repos/SpaceWarpDev/SpaceWarp/releases/latest";

pub static USER_AGENT: &'static str = "SpaceWarp Installer v0";

pub async fn get_latest_release_data() -> ReleaseResponse {
    let client = Client::new();

    let res = client
        .get(LATEST_RELEASE_URL)
        .header(reqwest::header::USER_AGENT, USER_AGENT)
        .send()
        .await
        .expect("Failed to get API response from GitHub!")
        .text()
        .await
        .expect("Failed to get API response payload!");

    let json = serde_json::from_str::<ReleaseResponse>(&res)
        .expect("JSON was not correctly using the GitHub release schema!");

    return json;
}

pub async fn get_latest_release_zips() -> ReleaseZips {
    let json = get_latest_release_data().await;
    let mut zips = ReleaseZips::new();

    for asset in json.assets {
        if asset.content_type.eq("application/x-zip-compressed") {
            if asset.name.to_lowercase().contains("bepinex") {
                if zips.bepinex.is_none() {
                    zips.bepinex = Some(asset.browser_download_url);
                }
            } else {
                if zips.doorstop.is_none() {
                    zips.doorstop = Some(asset.browser_download_url);
                }
            }
        }
    }

    return zips;
}
