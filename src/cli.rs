use crate::generate;
use crate::live;
use crate::models;
use crate::validate;
use clap::{Args, Parser, Subcommand};
use schemars::schema_for;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand, Clone, Debug)]
#[command(version, about, long_about = None)]
enum Command {
    Schema,
    Validate(ValidateArgs),
    Generate(GenerateArgs),
    Live(LiveArgs),
}

#[derive(Args, Clone, Debug)]
#[group(required = true)]
struct ValidateArgs {
    #[arg(short = 'i', value_name = "FILE")]
    file_in: std::path::PathBuf,
    #[arg(short = 'l', long, value_name = "LOCALE")]
    locale: Option<String>,
}

#[derive(Args, Clone, Debug)]
struct GenerateArgs {
    #[arg(short = 'i', long = "in", value_name = "FILE")]
    file_in: std::path::PathBuf,
    #[arg(short = 'o', long = "out", value_name = "FILE")]
    file_out: Option<std::path::PathBuf>,
    #[arg(short = 't', long, value_name = "FILE")]
    template: std::path::PathBuf,
    #[arg(short = 'f', long = "format", value_name = "FORMAT")]
    format_out: Option<String>,
    #[arg(short, long, value_name = "LOCALE")]
    locale: Option<String>,
    #[arg(long, value_name = "TAGLIST", value_delimiter = ',', num_args = 0..)]
    tags: Vec<String>,
}

#[derive(Args, Clone, Debug)]
struct LiveArgs {
    #[arg(short = 'i', long = "in", value_name = "FILE")]
    file_in: std::path::PathBuf,
    #[arg(short = 't', long, value_name = "FILE")]
    template: std::path::PathBuf,
    #[arg(short = 'l', long, value_name = "LOCALE")]
    locale: Option<String>,
    #[arg(long, value_name = "TAGLIST", value_delimiter = ',', num_args = 0..)]
    tags: Vec<String>,
}

fn tags_to_ref(tags: &[String]) -> Vec<&str> {
    tags.iter().map(|s| s.as_str()).collect()
}

fn schema() {
    let schema = schema_for!(models::source::SourceResume);
    println!("{}", serde_json::to_string_pretty(&schema).unwrap());
}

fn validate(args: ValidateArgs) {
    let result = validate::parse_source_resume(args.file_in);
    match result {
        Ok(_) => {
            println!("Resume OK")
        }
        Err(error) => {
            println!("Error in resume: \n {:?}", error)
        }
    }
}

fn generate(args: GenerateArgs) {
    let format = generate::ExportFormat::try_from(args.format_out.unwrap_or("html".into()))
        .unwrap_or(generate::ExportFormat::Html);
    let filename_out = args
        .file_out
        .unwrap_or(format!("cv.{}", format.as_str()).into());
    let locale = args.locale.unwrap_or("en-US".to_string());
    rust_i18n::set_locale(locale.as_str());
    let tags = tags_to_ref(&args.tags);
    let resume = validate::parse_and_resolve_resume(args.file_in, &locale, tags.as_slice());

    let result =
        resume.map(|resume| generate::generate(resume, &args.template, &filename_out, format));

    match result {
        Ok(_) => println!("Resume generated at {}", &filename_out.display()),
        Err(error) => {
            println!("Error: \n {}", error)
        }
    }
}

fn live(args: LiveArgs) {
    let tags = tags_to_ref(&args.tags);
    let locale = args.locale.unwrap_or("en-US".to_string());
    rust_i18n::set_locale(locale.as_str());

    live::live(
        args.file_in.as_path(),
        args.template.as_path(),
        &locale,
        tags.as_slice(),
    );
}

pub fn run() {
    let cli = Cli::try_parse();

    match cli {
        Err(error) => error.print().expect("Unexpected Error!"),
        Ok(cli) => match cli.command {
            Command::Schema => schema(),
            Command::Validate(args) => validate(args),
            Command::Generate(args) => generate(args),
            Command::Live(args) => live(args),
        },
    }
}
