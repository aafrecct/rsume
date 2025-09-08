use color_eyre::{eyre::eyre, Result};
use schemars::JsonSchema;
use serde::Deserialize;
use std::collections::HashMap;
use url::Url;

use crate::models::core::{
    FlexibleDate, JobExperience, JobType, Location, Metadata, Profile, SkillProficiency,
};

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceResume {
    pub basics: SourceBasicInfo,
    pub asociations: Option<Vec<SourceAsociation>>,
    pub awards: Option<Vec<SourceAward>>,
    pub certificates: Option<Vec<SourceCertificate>>,
    pub education: Option<Vec<SourceEducation>>,
    pub interests: Option<Vec<SourceInterest>>,
    pub languages: Option<Vec<SourceLanguage>>,
    pub projects: Option<Vec<SourceProject>>,
    pub publications: Option<Vec<SourcePublication>>,
    pub references: Option<Vec<SourceReference>>,
    pub skills: Option<Vec<SourceSkillSet>>,
    pub volunteer_work: Option<Vec<SourceVolunteerWork>>,
    pub work_experience: Option<Vec<SourceWorkExperience>>,
    pub meta: Option<Metadata>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceBasicInfo {
    pub name: String,
    pub surname: String,
    pub label: Localized<String>,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub zipcode: Option<String>,
    pub location: Localized<Location>,
    pub summary: Option<Localized<String>>,
    pub image: Option<Url>,
    pub profiles: Option<Vec<Profile>>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceEducation {
    pub institution: Localized<String>,
    pub title: Localized<String>,
    pub level: Localized<String>,
    pub location: Option<Localized<Location>>,
    pub grade_average: Option<f32>,
    pub start_date: Option<FlexibleDate>,
    pub end_date: Option<FlexibleDate>,
    pub highlights: Option<Localized<Vec<String>>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceWorkExperience {
    pub position: Localized<String>,
    pub description: Option<Localized<String>>,
    pub organization: Localized<String>,
    pub organization_url: Option<Url>,
    pub job_type: Option<JobType>,
    pub experience: Option<JobExperience>,
    pub location: Option<Localized<Location>>,
    pub start_date: FlexibleDate,
    pub end_date: Option<FlexibleDate>,
    pub highlights: Option<Localized<Vec<String>>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceVolunteerWork {
    pub title: Localized<String>,
    pub organization: Option<Localized<String>>,
    pub organization_url: Option<Url>,
    pub location: Option<Localized<Location>>,
    pub start_date: Option<FlexibleDate>,
    pub end_date: Option<FlexibleDate>,
    pub highlights: Option<Localized<Vec<String>>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceSkillSet {
    pub name: Localized<String>,
    pub items: Vec<SourceSkill>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceSkill {
    pub name: Localized<String>,
    pub proficiency: Option<SkillProficiency>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceProject {
    pub name: Localized<String>,
    pub description: Option<Localized<String>>,
    pub url: Option<Url>,
    pub keywords: Option<Localized<Vec<String>>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourcePublication {
    pub name: Localized<String>,
    pub description: Localized<Option<String>>,
    pub url: Option<Url>,
    pub release_date: FlexibleDate,
    pub publisher: String,
    pub summary: Option<Localized<String>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceAward {
    pub title: Localized<String>,
    pub date: FlexibleDate,
    pub awarder: Localized<String>,
    pub summary: Option<Localized<String>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceAsociation {
    pub name: Localized<String>,
    pub role: Localized<String>,
    pub joined: FlexibleDate,
    pub left: Option<FlexibleDate>,
    pub highlights: Option<Localized<Vec<String>>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceCertificate {
    pub name: Localized<String>,
    pub issuer: Localized<String>,
    pub date: FlexibleDate,
    pub url: Url,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceLanguage {
    pub language: Localized<String>,
    pub fluency: Localized<String>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceInterest {
    pub name: Localized<String>,
    pub keywords: Option<Localized<Vec<String>>>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
pub struct SourceReference {
    pub name: String,
    pub description: Option<Localized<String>>,
    pub contact: Option<String>,
    pub quote: Option<Localized<String>>,
    pub url: Option<Url>,
    pub tags: Option<Tags>,
}

#[derive(Deserialize, JsonSchema, Debug)]
#[serde(untagged)]
pub enum Localized<T> {
    MultiLocale(HashMap<String, T>),
    Single(T),
}

impl<T> Localized<T> {
    pub fn try_into_inner(self, locale: &str) -> Result<T> {
        match self {
            Self::Single(inner) => Ok(inner),
            Self::MultiLocale(mut map) => map
                .remove(locale)
                .ok_or(eyre!("Locale missing: {}", locale)),
        }
    }
}

type Tags = Vec<String>;
