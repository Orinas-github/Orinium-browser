use anyhow::Context;

pub async fn load_local_file(path: &str) -> Result<Vec<u8>, anyhow::Error> {
    use tokio::fs::File;
    use tokio::io::AsyncReadExt;
    let mut file = File::open(path).await.context("Failed to open file")?;
    let mut contents = Vec::new();
    file.read_to_end(&mut contents).await.context("Failed to read file")?;
    Ok(contents)
}