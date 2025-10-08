pub mod cli;
mod config;
pub mod generate;
pub mod live;
pub mod models;
mod templates;
pub mod validate;

rust_i18n::i18n!("locales");

fn main() {
    cli::run();
}
