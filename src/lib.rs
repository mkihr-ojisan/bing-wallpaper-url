use html_extractor::*;

/// Gets the url of Bing wallpaper.
///
/// # Example
/// ```
/// # #[tokio::main]
/// # async fn main() {
/// let url = bing_wallpaper_url::get_url().await.unwrap();
/// println!("{}", url);
/// # }
/// ```
pub async fn get_url() -> Result<String, Error> {
    let html = reqwest::get("https://www.bing.com/").await?.text().await?;
    let bing = BingTopPage::extract_from_str(&html)?;
    Ok(format!("https://www.bing.com{}", bing.wallpaper_url))
}
html_extractor! {
    BingTopPage {
        wallpaper_url: String = (attr["href"] of ".item.download > a"),
    }
}

#[derive(Debug)]
pub enum Error {
    Network(reqwest::Error),
    InvalidResponse(html_extractor::Error),
}
impl From<reqwest::Error> for Error {
    fn from(e: reqwest::Error) -> Self {
        Error::Network(e)
    }
}
impl From<html_extractor::Error> for Error {
    fn from(e: html_extractor::Error) -> Self {
        Error::InvalidResponse(e)
    }
}
