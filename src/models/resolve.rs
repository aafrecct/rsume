use crate::models::core;
use crate::models::source;
use color_eyre::Result;
pub trait Resolve<T> {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<T>>;
    fn include_by_tags(&self, include_tags: &[&str]) -> bool;
}

impl Resolve<core::Resume> for source::SourceResume {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Resume>> {
        Ok(Some(core::Resume {
            basics: self.basics.resolve(locale, include_tags)?.unwrap(),
            education: self.education.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            work_experience: self.work_experience.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            volunteer_work: self.volunteer_work.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            asociations: self.asociations.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            awards: self.awards.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            certificates: self.certificates.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            publications: self.publications.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            skills: self.skills.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            languages: self.languages.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            interests: self.interests.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            projects: self.projects.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            references: self.references.map(|o| {
                o.into_iter()
                    .filter_map(|i| i.resolve(locale, include_tags).ok()?)
                    .collect()
            }),
            meta: self.meta,
        }))
    }

    fn include_by_tags(&self, _include_tags: &[&str]) -> bool {
        true
    }
}

impl Resolve<core::BasicInfo> for source::SourceBasicInfo {
    fn resolve(self, locale: &str, _include_tags: &[&str]) -> Result<Option<core::BasicInfo>> {
        Ok(Some(core::BasicInfo {
            name: self.name,
            surname: self.surname,
            label: self.label.try_into_inner(locale)?,
            email: self.email,
            phone: self.phone,
            address: self.address,
            zipcode: self.zipcode,
            location: self.location.try_into_inner(locale)?,
            summary: self.summary.map(|s| s.try_into_inner(locale)).transpose()?,
            image: self.image,
            profiles: self.profiles,
        }))
    }

    fn include_by_tags(&self, _include_tags: &[&str]) -> bool {
        true
    }
}

impl Resolve<core::Asociation> for source::SourceAsociation {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Asociation>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Asociation {
            name: self.name.try_into_inner(locale)?,
            role: self.role.try_into_inner(locale)?,
            joined: self.joined,
            left: self.left,
            highlights: self
                .highlights
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
        }))
    }

    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Award> for source::SourceAward {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Award>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Award {
            title: self.title.try_into_inner(locale)?,
            date: self.date,
            awarder: self.awarder.try_into_inner(locale)?,
            summary: self.summary.map(|s| s.try_into_inner(locale)).transpose()?,
        }))
    }

    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Certificate> for source::SourceCertificate {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Certificate>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Certificate {
            name: self.name.try_into_inner(locale)?,
            issuer: self.issuer.try_into_inner(locale)?,
            date: self.date,
            url: self.url,
        }))
    }

    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Education> for source::SourceEducation {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Education>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Education {
            institution: self.institution.try_into_inner(locale)?,
            location: self
                .location
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
            title: self.title.try_into_inner(locale)?,
            level: self.level.try_into_inner(locale)?,
            grade_average: self.grade_average,
            start_date: self.start_date,
            end_date: self.end_date,
            highlights: self
                .highlights
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
        }))
    }

    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Interest> for source::SourceInterest {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Interest>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Interest {
            name: self.name.try_into_inner(locale)?,
            keywords: self
                .keywords
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
        }))
    }

    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Language> for source::SourceLanguage {
    fn resolve(self, locale: &str, _include_tags: &[&str]) -> Result<Option<core::Language>> {
        Ok(Some(core::Language {
            language: self.language.try_into_inner(locale)?,
            fluency: self.fluency.try_into_inner(locale)?,
        }))
    }

    fn include_by_tags(&self, _include_tags: &[&str]) -> bool {
        true
    }
}

impl Resolve<core::Project> for source::SourceProject {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Project>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Project {
            name: self.name.try_into_inner(locale)?,
            description: self
                .description
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
            url: self.url,
            keywords: self
                .keywords
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
        }))
    }

    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Publication> for source::SourcePublication {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Publication>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Publication {
            name: self.name.try_into_inner(locale)?,
            description: self.description.try_into_inner(locale)?,
            url: self.url,
            release_date: self.release_date,
            publisher: self.publisher,
            summary: self.summary.map(|s| s.try_into_inner(locale)).transpose()?,
        }))
    }
    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Reference> for source::SourceReference {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Reference>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Reference {
            name: self.name,
            description: self
                .description
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
            contact: self.contact,
            quote: self.quote.map(|s| s.try_into_inner(locale)).transpose()?,
            url: self.url,
        }))
    }
    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Skillset> for source::SourceSkillSet {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::Skillset>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::Skillset {
            name: self.name.try_into_inner(locale)?,
            items: self
                .items
                .into_iter()
                .filter_map(|o| o.resolve(locale, include_tags).transpose())
                .collect::<Result<Vec<core::Skill>>>()?,
        }))
    }
    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::Skill> for source::SourceSkill {
    fn resolve(self, locale: &str, _include_tags: &[&str]) -> Result<Option<core::Skill>> {
        Ok(Some(core::Skill {
            name: self.name.try_into_inner(locale)?,
            proficiency: self.proficiency,
        }))
    }
    fn include_by_tags(&self, _include_tags: &[&str]) -> bool {
        true
    }
}

impl Resolve<core::VolunteerWork> for source::SourceVolunteerWork {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::VolunteerWork>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::VolunteerWork {
            title: self.title.try_into_inner(locale)?,
            organization: self
                .organization
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
            organization_url: self.organization_url,
            location: self
                .location
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
            start_date: self.start_date,
            end_date: self.end_date,
            highlights: self
                .highlights
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
        }))
    }
    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

impl Resolve<core::WorkExperience> for source::SourceWorkExperience {
    fn resolve(self, locale: &str, include_tags: &[&str]) -> Result<Option<core::WorkExperience>> {
        if !self.include_by_tags(include_tags) {
            return Ok(None);
        }

        Ok(Some(core::WorkExperience {
            organization: self.organization.try_into_inner(locale)?,
            organization_url: self.organization_url,
            position: self.position.try_into_inner(locale)?,
            description: self
                .description
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
            job_type: self.job_type,
            experience: self.experience,
            location: self
                .location
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
            start_date: self.start_date,
            end_date: self.end_date,
            highlights: self
                .highlights
                .map(|s| s.try_into_inner(locale))
                .transpose()?,
        }))
    }
    fn include_by_tags(&self, include_tags: &[&str]) -> bool {
        include_by_tags(self.tags.as_deref(), include_tags)
    }
}

fn include_by_tags(tag_list: Option<&[String]>, include_tags: &[&str]) -> bool {
    if include_tags.is_empty() {
        true
    } else if let Some(tags) = tag_list {
        include_tags
            .iter()
            .any(|i_tag| tags.iter().any(|l_tag| l_tag == *i_tag))
    } else {
        false
    }
}
