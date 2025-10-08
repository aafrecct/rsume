use serde::Serialize;

use crate::config;
use crate::models::Resume;

#[derive(Serialize, Debug)]
pub struct OutputResume<'a> {
    pub config: &'a config::OutputConfig,
    #[serde(flatten)]
    pub resume: &'a Resume,
}
