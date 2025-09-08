use std::{
    fs::read_to_string,
    path::{Path, PathBuf},
};

use handlebars::{Context, Handlebars, Helper, HelperResult, Output, RenderContext};
use serde_json::Value;

use color_eyre::eyre::{eyre, Result};
use rust_i18n::t;
use serde::Serialize;

pub struct CVTemplateManager<'a> {
    handlebars: Handlebars<'a>,
    teplate_path: PathBuf,
}

impl CVTemplateManager<'_> {
    pub fn from_template_path(template_path: &Path) -> Result<Self> {
        let mut handlebars = Handlebars::new();
        handlebars.register_helper("localize", Box::new(localize));

        let mut result = CVTemplateManager {
            handlebars,
            teplate_path: template_path.to_owned(),
        };
        result.check_template()?;

        result.update_styles()?;
        result.update_main()?;

        Ok(result)
    }

    fn check_template(&self) -> Result<()> {
        let is_dir = self.teplate_path.is_dir();
        if !is_dir {
            return Err(eyre!("Template path is not a directory."));
        }

        let main_path = self.teplate_path.join("main.hbs");
        let styles_path = self.teplate_path.join("styles.hbs");
        let has_main = main_path.is_file();
        let has_styles = styles_path.is_file();

        if !has_main {
            return Err(eyre!("Template path must contain `main.hbs`"));
        }

        if !has_styles {
            return Err(eyre!("Template path must contain `styles.hbs`"));
        }

        Ok(())
    }

    pub fn update_styles(&mut self) -> Result<()> {
        let style_partial = read_to_string(self.teplate_path.join("styles.hbs"))?;
        self.handlebars
            .register_partial("styles", style_partial.as_str())?;
        Ok(())
    }

    pub fn update_main(&mut self) -> Result<()> {
        let main_template = read_to_string(self.teplate_path.join("main.hbs"))?;
        self.handlebars
            .register_template_string("main", main_template.as_str())?;
        Ok(())
    }

    pub fn render<T: Serialize>(&self, data: T) -> Result<String> {
        Ok(self.handlebars.render("main", &data)?)
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
