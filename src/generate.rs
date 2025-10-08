use std::fs;

use crate::{config::OutputConfig, models, templates::CVTemplateManager};
use color_eyre::{eyre::eyre, Result};
use headless_chrome::{types::PrintToPdfOptions, Browser};
use std::io::Write;
use tempfile::NamedTempFile;

#[derive(Debug)]
pub enum ExportFormat {
    Html,
    Json,
    Pdf,
}

impl TryFrom<String> for ExportFormat {
    type Error = &'static str;

    fn try_from(value: String) -> Result<Self, Self::Error> {
        match value.trim().to_lowercase().as_str() {
            "html" => Ok(ExportFormat::Html),
            "json" => Ok(ExportFormat::Json),
            "pdf" => Ok(ExportFormat::Pdf),
            _ => Err("Not a valid export format!"),
        }
    }
}

impl ExportFormat {
    pub fn as_str(&self) -> &'static str {
        match self {
            ExportFormat::Html => "html",
            ExportFormat::Json => "json",
            ExportFormat::Pdf => "pdf",
        }
    }
}

pub fn generate<P: AsRef<std::path::Path>>(
    resume: models::Resume,
    template: P,
    filename_out: P,
    format: ExportFormat,
    config: OutputConfig,
) -> Result<()> {
    match format {
        ExportFormat::Html => fs::write(filename_out, render_to_html(resume, template, config)?)?,
        ExportFormat::Json => fs::write(filename_out, serde_json::to_string_pretty(&resume)?)?,
        ExportFormat::Pdf => generate_to_pdf(resume, template, filename_out, config)?,
    };
    Ok(())
}

pub fn render_to_html<P: AsRef<std::path::Path>>(
    resume: models::Resume,
    template: P,
    config: OutputConfig,
) -> Result<String> {
    let template_manager = CVTemplateManager::from_template_name_or_path(
        template
            .as_ref()
            .to_str()
            .ok_or(eyre!("Error converting path to str"))?,
        config,
    )?;

    template_manager.render(&resume)
}

pub fn generate_to_pdf<P: AsRef<std::path::Path>>(
    resume: models::Resume,
    template: P,
    filename_out: P,
    config: OutputConfig,
) -> Result<()> {
    let html = render_to_html(resume, template, config)?;
    let browser = Browser::default().map_err(|e| eyre!(Box::new(e)))?;
    let tab = browser.new_tab().map_err(|e| eyre!(Box::new(e)))?;
    let mut tempfile = NamedTempFile::with_suffix(".html")?;
    write!(tempfile, "{}", html)?;

    tab.navigate_to(format!("file://{}", &tempfile.path().display()).as_str())
        .map_err(|e| eyre!(Box::new(e)))?;

    tab.wait_for_element("#readyMarker")
        .map_err(|e| eyre!(Box::new(e)))?;

    let pdf = tab
        .print_to_pdf(Some(PrintToPdfOptions {
            landscape: Some(false),
            display_header_footer: Some(false),
            print_background: Some(true),
            scale: Some(1.0),
            paper_width: Some(8.27),
            paper_height: Some(11.69),
            prefer_css_page_size: Some(true),
            generate_document_outline: Some(true),
            generate_tagged_pdf: Some(true),
            ..Default::default()
        }))
        .map_err(|e| eyre!(Box::new(e)))?;
    fs::write(filename_out, &pdf)?;
    Ok(())
}
