pub mod cli;
pub mod generate;
pub mod l10n;
pub mod live;
pub mod models;
pub mod templates;
pub mod validate;

rust_i18n::i18n!("locales");

fn main() {
    cli::run();
}
