use color_eyre::eyre::{eyre, Result};
use font_kit::source::SystemSource;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct OutputConfig {
    pub icon_font_path: PathBuf,
    pub icon_font_format: String,
}

pub fn get_icon_font() -> Result<(PathBuf, String)> {
    let font_collection = SystemSource::new().select_family_by_name("remixicon")?;
    let font_kit::handle::Handle::Path {
        path: font_path,
        font_index: _,
    } = font_collection
        .fonts()
        .first()
        .ok_or(eyre!("No fonts in font collection"))?
    else {
        return Err(eyre!("Font file for remixicon not found"));
    };
    let extension = font_path
        .extension()
        .ok_or(eyre!("Font file has no extension"))?
        .to_str()
        .ok_or(eyre!("Error converting OsString to String."))?;

    Ok((font_path.to_path_buf(), font_format(extension).to_string()))
}

pub fn load_config() -> Result<OutputConfig> {
    let (icon_font_path, icon_font_format) = get_icon_font()?;
    Ok(OutputConfig {
        icon_font_path,
        icon_font_format,
    })
}

pub fn font_format(extension: &str) -> &'static str {
    match extension {
        "ttf" => "truetype",
        "woff" => "woff",
        "woff2" => "woff2",
        _ => "opentype",
    }
}
