use crate::models::Resume;
use crate::templates;
use crate::templates::CVTemplateManager;
use crate::validate;

use color_eyre::eyre::Result;
use color_eyre::owo_colors::OwoColorize;
use colored::Colorize;
use log::{error, info, trace};
use notify::RecursiveMode;
use notify_debouncer_full::notify::*;
use smol::channel;
use smol::fs::canonicalize;
use smol::future;
use smol::Timer;
use std::path::Path;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use tide::http::mime;

pub fn live(file_in: &Path, template_path: &Path, locale: &str, tags: &[&str]) {
    let mut logger = colog::default_builder();
    logger.init();

    let executor = smol::Executor::new();

    let rendered_resume = Arc::new(Mutex::new(String::new()));

    future::block_on(executor.run(async {
        executor
            .spawn(async_watch(
                file_in,
                template_path,
                rendered_resume.clone(),
                locale,
                tags,
            ))
            .detach();

        executor
            .spawn(http_server(rendered_resume.clone()))
            .await
            .unwrap();
    }));
}

fn watcher() -> notify::Result<(RecommendedWatcher, channel::Receiver<notify::Event>)> {
    let (tx, rx) = smol::channel::unbounded();

    let watcher = recommended_watcher(move |res| {
        if let Ok(event) = res {
            future::block_on(async {
                tx.send(event).await.unwrap();
            })
        } else {
            println!("Failure in watcher")
        }
    })?;

    Ok((watcher, rx))
}

async fn async_watch<P: AsRef<Path>>(
    resume_path: P,
    template_path: P,
    rendered_resume: Arc<Mutex<String>>,
    locale: &str,
    tags: &[&str],
) -> Result<()> {
    let absolute_resume_path = canonicalize(resume_path).await?;
    let absolute_template_path = canonicalize(template_path).await?;

    println!("{}", "Validating resume...".italic().blue());
    let mut resume = validate::parse_and_resolve_resume(&absolute_resume_path, locale, tags)
        .inspect_err(|e| error!("{}", e))?;

    println!("{}", "Loading templates...".italic().blue());
    let mut template_manager =
        templates::CVTemplateManager::from_template_path(&absolute_template_path)
            .inspect_err(|e| error!("{}", e,))?;

    println!("{}", "Rendering resume...".italic().blue());
    {
        let mut rr = rendered_resume.lock().unwrap();
        *rr = template_manager.render(&resume).unwrap();
    }

    println!("{}", "Starting file watcher...".italic().blue());
    let (mut watcher, rx) = watcher()?;

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    watcher.watch(&absolute_resume_path, RecursiveMode::Recursive)?;
    watcher.watch(&absolute_template_path, RecursiveMode::Recursive)?;

    while let Ok(event) = rx.recv().await {
        if let Err(e) = handle_watch_event(
            event,
            &mut template_manager,
            &mut resume,
            &absolute_resume_path,
            &absolute_template_path,
            rendered_resume.clone(),
            locale,
            tags,
        ) {
            eprintln!("{}\n{}", "Error while rendering the resume.".red(), e)
        };
    }

    Ok(())
}

fn handle_watch_event<P: AsRef<Path>>(
    event: Event,
    template_manager: &mut CVTemplateManager,
    resume: &mut Resume,
    resume_path: P,
    template_path: P,
    rendered_resume: Arc<Mutex<String>>,
    locale: &str,
    tags: &[&str],
) -> Result<()> {
    if !event.kind.is_modify() {
        trace!("Event ignored");
        return Ok(());
    }

    let Some(path_changed) = event.paths.first() else {
        trace!("Event ignored");
        return Ok(());
    };

    trace!("Event has path: {}", path_changed.display());
    if path_changed.starts_with(&template_path) {
        trace!("Event is template change");
        if path_changed.ends_with("styles.hbs") {
            template_manager.update_styles()?;
        } else if path_changed.ends_with("main.hbs") {
            template_manager.update_main()?;
        }
    } else if path_changed.starts_with(&resume_path) {
        trace!("Event is resume change");
        *resume = validate::parse_and_resolve_resume(&resume_path, locale, tags)?;
    } else {
        trace!("Event path is strange");
        return Ok(());
    }

    let mut rendered = rendered_resume.lock().unwrap();
    *rendered = template_manager.render(resume)?;

    Ok(())
}

async fn http_server(rendered_resume: Arc<Mutex<String>>) -> std::io::Result<()> {
    let mut app = tide::new();
    app.at("/")
        .get(move |r| handle_request(r, rendered_resume.clone()));

    Timer::after(Duration::from_millis(50)).await;
    println!(
        "{} {}",
        "Live resume available at: ".italic(),
        "http:://127.0.0.1:3030".bold().blue()
    );

    app.listen("127.0.0.1:3030").await?;
    Ok(())
}

async fn handle_request(
    req: tide::Request<()>,
    rendered_resume: Arc<Mutex<String>>,
) -> tide::Result<tide::Response> {
    info!("{} at {}", req.method().bold(), "/".blue());

    Ok(tide::Response::builder(200)
        .body(rendered_resume.lock().unwrap().clone())
        .content_type(mime::HTML)
        .build())
}
