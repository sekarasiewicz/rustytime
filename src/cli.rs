// src/cli.rs
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "timey", version, about = "Rust time tracker")]
pub struct Cli {
    #[arg(long, default_value = "timey.db")]
    pub db: String,
    #[command(subcommand)]
    pub cmd: Command,
}

#[derive(Subcommand)]
pub enum Command {
    // Projects
    Project {
        #[command(subcommand)]
        cmd: ProjectCmd,
    },
    // Tasks
    Task {
        #[command(subcommand)]
        cmd: TaskCmd,
    },
    // Timer
    Start {
        task_id: String,
    },
    Stop,
    // Reports
    Report {
        #[command(subcommand)]
        cmd: ReportCmd,
    },
    // Export
    Export {
        format: ExportFormat,
        #[arg(long)]
        out: String,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum ProjectCmd {
    Add {
        name: String,
        #[arg(long)]
        desc: Option<String>,
    },
    List,
    Edit {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        desc: Option<String>,
    },
    Archive {
        id: String,
    },
    Delete {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum TaskCmd {
    Add {
        project_id: String,
        name: String,
        #[arg(long)]
        desc: Option<String>,
    },
    List {
        project_id: String,
    },
    Edit {
        id: String,
        #[arg(long)]
        name: Option<String>,
        #[arg(long)]
        desc: Option<String>,
    },
    Archive {
        id: String,
    },
    Delete {
        id: String,
    },
}

#[derive(Subcommand)]
pub enum ReportCmd {
    Daily {
        #[arg(long)]
        date: Option<String>,
    }, // YYYY-MM-DD
    Project {
        project_id: String,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
    Task {
        task_id: String,
        #[arg(long)]
        from: Option<String>,
        #[arg(long)]
        to: Option<String>,
    },
}

#[derive(clap::ValueEnum, Clone)]
pub enum ExportFormat {
    Csv,
    Json,
}
