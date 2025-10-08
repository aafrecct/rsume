use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
    str,
};

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use serde_json::Value;

use color_eyre::eyre::{eyre, Result};
use rust_embed::Embed;
use rust_i18n::t;

use crate::{
    config::OutputConfig,
    models::{OutputResume, Resume},
};

#[derive(Embed)]
#[folder = "templates"]
#[exclude = "*.map"]
#[exclude = "*.scss"]
struct TemplateFiles;

pub struct CVTemplateManager<'a> {
    handlebars: Handlebars<'a>,
    template: Template,
    config: OutputConfig,
}

pub enum Template {
    FileTemplate { path: PathBuf },
    EmbeddedTemplate { name: String },
}

impl CVTemplateManager<'_> {
    pub fn from_template_name_or_path(template: &str, config: OutputConfig) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("localize", Box::new(localize));
        handlebars.register_helper("icon", Box::new(icon));

        let mut result = Self {
            handlebars,
            template: Self::template_from_str(template)?,
            config,
        };

        result.update_base()?;
        result.update_styles()?;
        result.update_main()?;

        Ok(result)
    }

    fn template_from_str(template: &str) -> Result<Template> {
        if template == "basic" {
            Ok(Template::EmbeddedTemplate {
                name: template.to_owned(),
            })
        } else {
            Self::template_from_template_path(PathBuf::from(template))
        }
    }

    fn template_from_template_path(template_path: PathBuf) -> Result<Template> {
        let is_dir = template_path.is_dir();
        if !is_dir {
            return Err(eyre!("Template path is not a directory."));
        }

        let main_path = template_path.join("main.hbs");
        let styles_path = template_path.join("styles.hbs");
        let has_main = main_path.is_file();
        let has_styles = styles_path.is_file();

        if !has_main {
            return Err(eyre!("Template path must contain `main.hbs`"));
        }

        if !has_styles {
            return Err(eyre!("Template path must contain `styles.hbs`"));
        }

        Ok(Template::FileTemplate {
            path: template_path,
        })
    }

    fn update_base(&mut self) -> Result<()> {
        let template_data = self.get_template_data_from_embedded("common", "style.css")?;
        self.handlebars.register_partial("base", template_data)?;
        Ok(())
    }

    pub fn update_styles(&mut self) -> Result<()> {
        self.load_template_file("styles", "styles.hbs", true)
    }

    pub fn update_main(&mut self) -> Result<()> {
        self.load_template_file("main", "main.hbs", true)
    }

    fn get_template_data_from_path(&self, path: &Path, file: &str) -> Result<String> {
        let mut full_template_path = path.to_path_buf();
        full_template_path.push(file);
        Ok(read_to_string(full_template_path)?)
    }

    fn get_template_data_from_embedded(&self, template_name: &str, file: &str) -> Result<String> {
        let mut path = PathBuf::from(template_name);
        path.push(file);

        let data = &TemplateFiles::get(path.to_str().ok_or(eyre!("Error getting embedded file"))?)
            .ok_or(eyre!("No such embedded file: {}", path.display()))?
            .data;
        Ok(String::from_utf8(data.to_vec())?)
    }

    fn load_template_file(
        &mut self,
        register_as: &str,
        file: &str,
        is_partial: bool,
    ) -> Result<()> {
        let template_data = match &self.template {
            Template::FileTemplate { path } => self.get_template_data_from_path(path, file)?,
            Template::EmbeddedTemplate { name } => {
                self.get_template_data_from_embedded(name, file)?
            }
        };

        if is_partial {
            self.handlebars
                .register_partial(register_as, template_data)?;
        } else {
            self.handlebars
                .register_template_string(register_as, template_data)?;
        }
        Ok(())
    }
    pub fn render(&self, resume: &Resume) -> Result<String> {
        let output_resume = OutputResume {
            resume,
            config: &self.config,
        };
        Ok(self.handlebars.render("main", &output_resume)?)
    }
}

pub fn localize(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let Some(param) = h.param(0) else {
        out.write("Nothing to localize")?;
        return Ok(());
    };

    let Value::String(s) = param.value() else {
        out.write("Localization error: not a string")?;
        return Ok(());
    };

    let param_serialized = t!(s.as_str());
    out.write(&param_serialized)?;
    Ok(())
}

pub fn icon(
    h: &Helper,
    _: &Handlebars,
    _: &Context,
    _rc: &mut RenderContext,
    out: &mut dyn Output,
) -> HelperResult {
    let Some(param) = h.param(0) else {
        out.write("No Icon")?;
        return Ok(());
    };

    let Value::String(s) = param.value() else {
        out.write("Icon name should be str")?;
        return Ok(());
    };

    out.write(format!("<i class=\"ri-{}-fill\"></i>", s.to_lowercase()).as_str())?;
    Ok(())
}
