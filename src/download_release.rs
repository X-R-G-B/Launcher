// use url::Url;
use octocrab::{models::repos::Release, Error, models::repos::Asset};

async fn get_latest() -> Result<Release, Error> {
    let octo_inst = octocrab::instance();
    let releases = octo_inst
        .repos("X-R-G-B", "Artena")
        .releases()
        .get_latest()
        .await?;
    dbg!("Request API : SUCCESS");
    Ok(releases)
}

async fn download_assest(asset: &Asset) -> bool {
    true
}

pub async fn download_release() -> bool {
    let release = match get_latest().await {
        Ok(release) => release,
        Err(_) => {
            return false;
        },
    };
    for asset in &release.assets {
        download_assest(asset).await;
    }
    println!("{:?}", &release);
    true
}
