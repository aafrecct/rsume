use std::cmp::Reverse;

use crate::models::flexible_date::FlexibleDate;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Resume {
    pub basics: BasicInfo,
    pub asociations: Option<Vec<Asociation>>,
    pub awards: Option<Vec<Award>>,
    pub certificates: Option<Vec<Certificate>>,
    pub education: Option<Vec<Education>>,
    pub interests: Option<Vec<Interest>>,
    pub languages: Option<Vec<Language>>,
    pub projects: Option<Vec<Project>>,
    pub publications: Option<Vec<Publication>>,
    pub references: Option<Vec<Reference>>,
    pub skills: Option<Vec<Skillset>>,
    pub volunteer_work: Option<Vec<VolunteerWork>>,
    pub work_experience: Option<Vec<WorkExperience>>,
    pub meta: Option<Metadata>,
}

impl Resume {
    pub fn sort_fields(&mut self) {
        if let Some(ref mut education) = &mut self.education {
            education.sort_by_key(|i| Reverse(i.start_date.clone()));
        }

        if let Some(ref mut volunteer_work) = &mut self.volunteer_work {
            volunteer_work.sort_by_key(|i| Reverse(i.start_date.clone()));
        }

        if let Some(ref mut work_experience) = &mut self.work_experience {
            work_experience.sort_by_key(|i| Reverse(i.start_date.clone()));
        }
    }
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct BasicInfo {
    pub name: String,
    pub surname: String,
    pub label: String,
    pub email: String,
    pub phone: Option<String>,
    pub address: Option<String>,
    pub location: Location,
    pub zipcode: Option<String>,
    pub summary: Option<String>,
    pub image: Option<Url>,
    pub profiles: Option<Vec<Profile>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Profile {
    pub network: String,
    pub username: String,
    pub url: Url,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Education {
    pub institution: String,
    pub location: Option<Location>,
    pub title: String,
    pub level: String,
    pub grade_average: Option<f32>,
    pub start_date: Option<FlexibleDate>,
    pub end_date: Option<FlexibleDate>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct WorkExperience {
    pub organization: String,
    pub organization_url: Option<Url>,
    pub position: String,
    pub description: Option<String>,
    pub job_type: Option<JobType>,
    pub experience: Option<JobExperience>,
    pub location: Option<Location>,
    pub start_date: FlexibleDate,
    pub end_date: Option<FlexibleDate>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum JobType {
    FullTime,
    PartTime,
    Intern,
    Contractor,
    Freelancer,
    Other(String),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
#[serde(rename_all = "snake_case")]
pub enum JobExperience {
    Junior,
    MidLevel,
    Senior,
    Other(String),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct VolunteerWork {
    pub title: String,
    pub organization: Option<String>,
    pub organization_url: Option<Url>,
    pub location: Option<Location>,
    pub start_date: Option<FlexibleDate>,
    pub end_date: Option<FlexibleDate>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Skillset {
    pub name: String,
    pub items: Vec<Skill>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Skill {
    pub name: String,
    pub proficiency: Option<SkillProficiency>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub enum SkillProficiency {
    Aware,
    Novice,
    Intermediate,
    Advanced,
    Expert,
    Other(String),
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Project {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<Url>,
    pub keywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Publication {
    pub name: String,
    pub description: Option<String>,
    pub url: Option<Url>,
    pub release_date: FlexibleDate,
    pub publisher: String,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct Location {
    pub city: String,
    pub region: Option<String>,
    pub country: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Award {
    pub title: String,
    pub date: FlexibleDate,
    pub awarder: String,
    pub summary: Option<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Asociation {
    pub name: String,
    pub role: String,
    pub joined: FlexibleDate,
    pub left: Option<FlexibleDate>,
    pub highlights: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Certificate {
    pub name: String,
    pub issuer: String,
    pub date: FlexibleDate,
    pub url: Url,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Language {
    pub language: String,
    pub fluency: String,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Interest {
    pub name: String,
    pub keywords: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Reference {
    pub name: String,
    #[serde(alias = "reference")]
    pub description: Option<String>,
    pub contact: Option<String>,
    pub quote: Option<String>,
    pub url: Option<Url>,
}

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct Metadata {
    pub version: String,
    pub last_modified: FlexibleDate,
    pub canonical: Option<Url>,
}
