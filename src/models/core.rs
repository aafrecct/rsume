use std::cmp::{Ordering, Reverse};

use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use url::Url;

static MONTHS: [&str; 12] = [
    "Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec",
];

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

#[derive(Clone, Serialize, Deserialize, PartialEq, Eq, Debug)]
#[serde(try_from = "String", into = "String")]
pub enum FlexibleDate {
    Now,
    Year(u16),
    Month(u16, u8),
    Day(u16, u8, u8),
}

impl FlexibleDate {
    fn as_tuple(&self) -> (bool, u16, u8, u8) {
        match self {
            FlexibleDate::Now => (true, 0, 0, 0),
            FlexibleDate::Year(y) => (false, *y, 0, 0),
            FlexibleDate::Month(y, m) => (false, *y, *m, 0),
            FlexibleDate::Day(y, m, d) => (false, *y, *m, *d),
        }
    }

    fn innner_cmp(&self, other: &Self) -> Ordering {
        use FlexibleDate as D;

        match (self, other) {
            (D::Now, D::Now) => Ordering::Equal,
            (D::Now, _) => Ordering::Greater,
            (_, D::Now) => Ordering::Less,
            (date_a, date_b) => date_a.as_tuple().cmp(&date_b.as_tuple()),
        }
    }
}

impl PartialOrd for FlexibleDate {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for FlexibleDate {
    fn cmp(&self, other: &Self) -> Ordering {
        self.innner_cmp(other)
    }
}

impl TryFrom<String> for FlexibleDate {
    type Error = String;

    fn try_from(mut value: String) -> Result<Self, Self::Error> {
        value.retain(|c| !c.is_whitespace());
        if value.to_lowercase() == "now" {
            return Ok(FlexibleDate::Now);
        }

        let parts: Vec<&str> = value.split('-').collect();
        match parts.len() {
            1 => Ok(FlexibleDate::Year(
                parts[0]
                    .parse::<u16>()
                    .or(Err(format!("Invalid year {}", parts[0])))?,
            )),
            2 => Ok(FlexibleDate::Month(
                parts[0]
                    .parse::<u16>()
                    .or(Err(format!("Invalid year {}", parts[0])))?,
                parts[1]
                    .parse::<u8>()
                    .or(Err(format!("Invalid month {}", parts[1])))?,
            )),
            3 => Ok(FlexibleDate::Day(
                parts[0]
                    .parse::<u16>()
                    .or(Err(format!("Invalid year {}", parts[0])))?,
                parts[1]
                    .parse::<u8>()
                    .or(Err(format!("Invalid month {}", parts[1])))?,
                parts[2]
                    .parse::<u8>()
                    .or(Err(format!("Invalid day {}", parts[2])))?,
            )),
            _ => Err(format!("Invalid date: {}", value)),
        }
    }
}

impl From<FlexibleDate> for String {
    fn from(value: FlexibleDate) -> String {
        match value {
            FlexibleDate::Now => "Now".to_string(),
            FlexibleDate::Year(year) => format!("{}", year),
            FlexibleDate::Month(year, month) => {
                format!("{} {}", MONTHS[(month - 1) as usize], year)
            }
            FlexibleDate::Day(year, month, day) => {
                format!("{} {} {}", day, MONTHS[(month - 1) as usize], year)
            }
        }
    }
}

impl JsonSchema for FlexibleDate {
    fn schema_name() -> String {
        "FlexibleDate".to_string()
    }

    fn json_schema(gen: &mut schemars::gen::SchemaGenerator) -> schemars::schema::Schema {
        use schemars::schema::*;
        let mut obj = SchemaObject {
            instance_type: Some(SingleOrVec::Single(InstanceType::String.into())),
            ..SchemaObject::default()
        };

        obj.string().pattern = Some(
            "^((([1-2][0-9]{3}-)?[0-1][0-9]-[0-3][0-9]|[1-2][0-9]{3}-)?[0-1][0-9]|[1-2][0-9]{3})|(now)$"
                .into(),
        );
        let obj = Schema::Object(obj);
        gen.definitions_mut()
            .insert("FlexibleDate".into(), obj.clone());

        let or = SchemaObject {
            reference: Some("#/definitions/FlexibleDate".into()),
            ..SchemaObject::default()
        };
        Schema::Object(or)
    }
}
