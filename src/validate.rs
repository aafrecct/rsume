use crate::models::{core, resolve::Resolve, source};
use color_eyre::eyre::Result;

pub fn parse_and_resolve_resume<P: AsRef<std::path::Path>>(
    file_name: P,
    locale: &str,
    include_tags: &[&str],
) -> Result<core::Resume> {
    let source_resume = parse_source_resume(file_name)?;
    let mut core_resume = source_resume
        .resolve(locale, include_tags)?
        .expect("No resume found");
    core_resume.sort_fields();
    Ok(core_resume)
}

pub fn parse_source_resume<P: AsRef<std::path::Path>>(
    file_name: P,
) -> Result<source::SourceResume> {
    let file = std::fs::read_to_string(file_name)?;

    Ok(serde_yaml::from_str::<source::SourceResume>(&file)?)
}
