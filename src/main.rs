use error_chain::error_chain;
use std::fs::File;
use std::io::{self, Write};
use tempfile::Builder;

error_chain! {
    foreign_links {
        Io(std::io::Error);
        HttpRequest(reqwest::Error);
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    println!("Hello, world!");

    // Create a temporary directory
    let tmp_dir = Builder::new().prefix("example").tempdir()?;
    
    // Target URL
    let target = "http://www.rust-lang.org/logos/rust-logo-512x512.png";
    
    // Perform the HTTP GET request
    let response = reqwest::get(target).await?;
    
    // Extract file name from the URL
    let file_name = response
        .url()
        .path_segments()
        .and_then(|segments| segments.last())
        .and_then(|name| if name.is_empty() { None } else { Some(name) })
        .unwrap_or("tmp.bin");

    println!("File is being downloaded as {}", file_name);
    
    let file_path = tmp_dir.path().join(file_name);
    println!("File saved at {:?}", file_path);

    // Create a file in the temporary directory
    let mut dest = File::create(file_path)?;

    // Get the response body as bytes
    let content = response.bytes().await?;

    // Write content to the destination file
    io::copy(&mut content.as_ref(), &mut dest)?;
    
    Ok(())
}
