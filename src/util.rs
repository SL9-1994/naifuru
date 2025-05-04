use std::path::Path;

pub fn read_text(path: &Path) -> Result<String, std::io::Error> {
    let config: String = std::fs::read_to_string(path)?;

    Ok(config)
}
