# bing-wallpaper-url

A Rust crate for getting the url of Bing wallpaper.

## Usage of library

```rust
#[tokio::main]
async fn main() -> Result<(), bing_wallpaper_url::Error> {
    let url = bing_wallpaper_url::get_url().await?;
    println!("{}", url); //http://www.bing.com/hpwp/...
    Ok(())
}
```

## Usage of executable

```console
$ bing-wallpaper-url
https://www.bing.com/hpwp/9a5f9bbcc3f0298de8ef5c947510f8dc
```
