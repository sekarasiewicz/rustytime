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
    let cli = Cli::parse();
    let pool = db::open_db(&cli.db).await?;

    match cli.cmd {
        Command::Project { cmd } => match cmd {
            ProjectCmd::Add { name, desc } => {
                let id = services::project::add(&pool, &name, desc.as_deref()).await?;
                println!("{id}");
            }
            ProjectCmd::List => { /* print rows */ }
            ProjectCmd::Edit { id, name, desc } => { /* update */ }
            ProjectCmd::Archive { id } => { /* set archived=1 */ }
            ProjectCmd::Delete { id } => { /* delete */ }
        },
        Command::Task { cmd } => { /* mirror projects */ }
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
