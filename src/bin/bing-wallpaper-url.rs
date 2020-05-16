#[tokio::main]
async fn main() -> Result<(), bing_wallpaper_url::Error> {
    let url = bing_wallpaper_url::get_url().await?;
    println!("{}", url);
    Ok(())
}

