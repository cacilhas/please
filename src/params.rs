use clap::{ArgAction, Parser, Subcommand};

use crate::{vendors::PlsCommand, Vendor};


#[derive(Debug, Parser)]
#[command(
    author = "Montegasppα Cacilhας <montegasppa@cacilhas.info>",
    about = "A unified interface package manager for many OSes.",
    name = "please",
)]
pub struct Params {
    #[arg(short, long = "dry-run", action = ArgAction::SetTrue, help = "dry run (do not actually execute commands)")]
    pub dry_run: bool,
    #[arg(short, long, action = ArgAction::SetTrue, help = "assume yes for all prompts")]
    pub yes: bool,
    #[arg(short, long, help = "set the installer command")]
    pub vendor: Option<Vendor>,
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, Subcommand)]
pub enum Cmd {
    #[command(about = "install package(s)")]
    Install {
        #[arg(name = "package(s)", help = "package(s) to be installed")]
        args: Vec<String>,
    },
    #[command(about = "remove package(s)")]
    Remove {
        #[arg(name = "package(s)", help = "package(s) to be removed")]
        args: Vec<String>,
    },
    #[command(about = "upgrade package(s)")]
    Upgrade {
        #[arg(name = "package(s)", help = "package(s) to be upgraded")]
        args: Vec<String>,
    },
    #[command(about = "search for package(s)")]
    Search {
        #[arg(name = "query", help = "text to be searched")]
        args: String,
    },
    #[command(about = "get info for a package")]
    Info {
        #[arg(name = "package", help = "package for which to get info")]
        args: String,
    },
    #[command(about = "update database")]
    Update,
    #[command(about = "list installed packages")]
    List,
}

impl Cmd {
    pub fn args(&self) -> String {
        match self {
            Cmd::Install { args } => args.join(" "),
            Cmd::Remove { args } => args.join(" "),
            Cmd::Upgrade { args } => args.join(" "),
            Cmd::Search { args } => args.to_string(),
            Cmd::Info { args } => args.to_string(),
            _ => String::new(),
        }
    }
}

impl From<&Cmd> for PlsCommand {
    fn from(value: &Cmd) -> Self {
        match value {
            Cmd::Install {..} => PlsCommand::Install,
            Cmd::Remove {..} => PlsCommand::Remove,
            Cmd::Upgrade {args} if args.is_empty() => PlsCommand::UpgradeAll,
            Cmd::Upgrade {..} => PlsCommand::Upgrade,
            Cmd::Search {..} => PlsCommand::Search,
            Cmd::Info {..} => PlsCommand::Info,
            Cmd::Update => PlsCommand::Update,
            Cmd::List => PlsCommand::List,
        }
    }
}
