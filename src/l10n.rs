use fluent_templates::static_loader;

static_loader! {
    pub static LOCALES = {
        locales: "./locales",
        fallback_language: "en-US"
    };
}
