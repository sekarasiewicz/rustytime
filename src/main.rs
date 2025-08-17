mod cli;
mod db;
mod models;
mod services {
    pub mod export;
    pub mod project;
    pub mod report;
    pub mod task;
    pub mod timer;
    pub mod timeutil;
}

use clap::Parser;
use cli::{Cli, Command, ExportFormat, ProjectCmd, ReportCmd, TaskCmd};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv::dotenv().ok();
    let cli = Cli::parse();
    let (db_url, fs_path) = db::resolve_db_url(Some(&cli.db)); // or None if you removed --db
    let pool = db::open_db_resolved(&db_url, &fs_path).await?;

    match cli.cmd {
        Command::Project { cmd } => match cmd {
            ProjectCmd::Add { name, desc } => {
                let id = services::project::add(&pool, &name, desc.as_deref()).await?;
                println!("{id}");
            }
            ProjectCmd::List => {
                let projects = services::project::list(&pool).await?;
                for project in projects {
                    println!("id: {}, name: {}", project.id, project.name);
                }
            }
            ProjectCmd::Edit { id, name, desc } => { /* update */ }
            ProjectCmd::Archive { id } => { /* set archived=1 */ }
            ProjectCmd::Delete { id } => { /* delete */ }
        },
        Command::Task { cmd } => match cmd {
            TaskCmd::Add {
                project_id,
                name,
                desc,
            } => {}
            TaskCmd::List { project_id } => {}
            TaskCmd::Edit { id, name, desc } => {}
            TaskCmd::Archive { id } => {}
            TaskCmd::Delete { id } => {}
        },
        Command::Start { task_id } => {
            services::timer::start(&pool, &task_id).await?;
            println!("started {task_id}");
        }
        Command::Stop => {
            services::timer::stop(&pool).await?;
            println!("stopped");
        }
        Command::Report { cmd } => match cmd {
            ReportCmd::Daily { date } => { /* print (date,total) */ }
            ReportCmd::Project {
                project_id,
                from,
                to,
            } => { /* print secs */ }
            ReportCmd::Task { task_id, from, to } => { /* print secs */ }
        },
        Command::Export {
            format,
            out,
            from,
            to,
        } => {
            match format {
                ExportFormat::Json => {
                    services::export::export_json(&pool, &out, from.as_deref(), to.as_deref())
                        .await?
                }
                ExportFormat::Csv => { /* write CSV */ }
            }
            println!("exported -> {out}");
        }
    }
    Ok(())
}
