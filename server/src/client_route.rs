use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub struct ClientFiles {
    pub wasm: Vec<u8>,
    pub js: String,
    pub html: String,
    pub snippet: String,
    pub wasm_name: String,
    pub js_name: String,
    pub snippet_name: String,
}

pub async fn files() -> anyhow::Result<ClientFiles> {
    let base_path = Path::new("/home/birchmd/rust/zk-map-reduce/client/dist/");
    let files: io::Result<Vec<PathBuf>> = std::fs::read_dir(base_path)?
        .map(|entry| entry.map(|x| x.path()))
        .collect();
    let files = files?;

    let wasm_path = find_with_extension(&files, "wasm")
        .ok_or_else(|| anyhow::Error::msg("Wasm file not found"))?;
    let wasm_name: String =
        file_name(wasm_path).ok_or_else(|| anyhow::Error::msg("Cannot extract Wasm name"))?;
    let wasm = read_bytes(wasm_path).await?;

    let js_path =
        find_with_extension(&files, "js").ok_or_else(|| anyhow::Error::msg("JS file not found"))?;
    let js_name: String =
        file_name(js_path).ok_or_else(|| anyhow::Error::msg("Cannot extract JS name"))?;
    let js = read_text(js_path).await?;

    let html_path = find_with_extension(&files, "html")
        .ok_or_else(|| anyhow::Error::msg("HTML file not found"))?;
    let html = read_text(html_path).await?;

    let snippet_dir = std::fs::read_dir(base_path.join("snippets"))?
        .find_map(|entry| {
            let entry = entry.ok()?;
            if entry.file_name().to_str()?.starts_with("zkmr") {
                Some(entry.path())
            } else {
                None
            }
        })
        .ok_or_else(|| anyhow::Error::msg("snippet file not found"))?;
    let snippet_path = snippet_dir.join("inline0.js");
    let snippet = read_text(&snippet_path).await?;
    let snippet_name = format!(
        "snippets/{}/inline0.js",
        snippet_dir
            .file_name()
            .and_then(|x| x.to_str())
            .unwrap_or("")
    );

    let result = ClientFiles {
        wasm,
        js,
        html,
        snippet,
        wasm_name,
        js_name,
        snippet_name,
    };
    Ok(result)
}

fn find_with_extension<'a>(files: &'a [PathBuf], ext: &str) -> Option<&'a PathBuf> {
    files
        .iter()
        .find(|p| p.extension().map(|x| x == ext).unwrap_or(false))
}

fn file_name(path: &Path) -> Option<String> {
    path.file_name().and_then(|n| n.to_str()).map(Into::into)
}

async fn read_bytes(path: &Path) -> anyhow::Result<Vec<u8>> {
    let result = tokio::fs::read(path).await?;
    Ok(result)
}

async fn read_text(path: &Path) -> anyhow::Result<String> {
    let result = tokio::fs::read_to_string(path).await?;
    Ok(result)
}
