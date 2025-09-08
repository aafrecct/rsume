use crate::models;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

#[derive(Serialize, Deserialize, JsonSchema, Debug)]
pub struct JsonResume {
    basics: models::BasicInfo,
    work: Option<Vec<JRWork>>,
    volunteer: Option<Vec<JRVolunteer>>,
    education: Option<Vec<JREducation>>,
    awards: Option<Vec<models::Award>>,
    certificates: Option<Vec<models::Certificate>>,
    publications: Option<Vec<models::Publication>>,
    skills: Option<Vec<JRSkill>>,
    languages: Option<Vec<models::Language>>,
    interests: Option<Vec<models::Interest>>,
    references: Option<Vec<models::Reference>>,
    projects: Option<Vec<models::Project>>,
    meta: Option<models::Metadata>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct JRWork {
    name: String,
    description: Option<String>,
    position: String,
    location: String,
    url: Option<Url>,
    title: String,
    start_date: models::FlexibleDate,
    end_date: Option<models::FlexibleDate>,
    experience: Option<String>,
    summary: Option<String>,
    highlights: Vec<String>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct JRVolunteer {
    organization: Option<String>,
    organization_url: Option<Url>,
    position: String,
    start_date: Option<models::FlexibleDate>,
    end_date: Option<models::FlexibleDate>,
    summary: Option<String>,
    highlights: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct JREducation {
    institution: String,
    url: Url,
    area: String,
    study_type: String,
    location: String,
    score: Option<f32>,
    start_date: Option<models::FlexibleDate>,
    end_date: Option<models::FlexibleDate>,
    highlights: Option<Vec<String>>,
}

#[derive(Serialize, Deserialize, JsonSchema, Clone, Debug)]
pub struct JRSkill {
    name: String,
    level: Option<String>,
    keywords: Option<Vec<String>>,
}

impl From<JsonResume> for models::Resume {
    fn from(value: JsonResume) -> Self {
        models::Resume {
            education: value.education.map(vec_into),
            work_experience: value.work.map(vec_into),
            volunteer_work: value.volunteer.map(vec_into),
            basics: value.basics,
            asociations: None,
            skills: value.skills.map(vec_into),
            awards: value.awards,
            certificates: value.certificates,
            publications: value.publications,
            languages: value.languages,
            interests: value.interests,
            references: value.references,
            projects: value.projects,
            meta: value.meta,
        }
    }
}

fn vec_into<Y: Clone, T: From<Y>>(vector: Vec<Y>) -> Vec<T> {
    vector.iter().map(|i| i.clone().into()).collect()
}

impl From<JRWork> for models::WorkExperience {
    fn from(value: JRWork) -> Self {
        models::WorkExperience {
            organization: value.name,
            organization_url: value.url,
            position: value.position,
            description: value.description,
            start_date: value.start_date,
            end_date: value.end_date,
            location: Some(models::Location {
                city: value.location,
                region: None,
                country: None,
            }),
            highlights: Some(value.highlights),
            experience: value.experience.map(models::JobExperience::Other),
            job_type: None,
        }
    }
}

impl From<JREducation> for models::Education {
    fn from(value: JREducation) -> Self {
        models::Education {
            institution: value.institution,
            title: value.area,
            level: value.study_type,
            grade_average: value.score,
            location: Some(models::Location {
                city: value.location,
                region: None,
                country: None,
            }),
            start_date: value.start_date,
            end_date: value.end_date,
            highlights: value.highlights,
        }
    }
}

impl From<JRVolunteer> for models::VolunteerWork {
    fn from(value: JRVolunteer) -> Self {
        models::VolunteerWork {
            title: value.position,
            organization: value.organization,
            organization_url: value.organization_url,
            location: None,
            start_date: value.start_date,
            end_date: value.end_date,
            highlights: value.highlights,
        }
    }
}

impl From<JRSkill> for models::Skillset {
    fn from(value: JRSkill) -> Self {
        models::Skillset {
            name: value.name.clone(),
            items: vec![models::Skill {
                name: value.name,
                proficiency: value.level.map(models::SkillProficiency::Other),
            }],
        }
    }
}
