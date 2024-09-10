use std::{env, fmt::Display, fs::File, io::Read, path::PathBuf};
use clap::{ArgAction, Parser, Subcommand};
use eyre::{eyre, Result};
use toml::Table;
use crate::{vendors::PlsCommand, Vendor};


#[derive(Debug, Parser)]
#[command(
    author = "Montegasppα Cacilhας <montegasppa@cacilhas.info>",
    about = "A unified interface package manager for many OSes.",
    name = "please",
)]
pub struct Params {
    #[arg(short = 'x', long, action = ArgAction::SetTrue, help = "skip settings")]
    pub skip_settings: bool,
    #[arg(short, long, help = "configuration file")]
    pub config: Option<String>,
    #[arg(short, long = "dry-run", action = ArgAction::SetTrue, help = "dry run (do not actually execute commands)")]
    pub dry_run: bool,
    #[arg(short, long, action = ArgAction::SetTrue, help = "assume yes for all prompts")]
    pub yes: bool,
    #[cfg(not(target_os = "windows"))]
    #[arg(short, long, action = ArgAction::SetTrue, help = "run as root (user must be wheel)")]
    pub su: bool,
    #[arg(short, long, help = "set the installer command")]
    pub vendor: Option<Vendor>,
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Debug, PartialEq, Subcommand)]
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
    #[command(about = "list available vendors")]
    ListVendors,
}

impl Params {
    pub fn config(mut self) -> Self {
        if self.skip_settings {
            return self;
        }

        let config = match &self.config {
            Some(config) => PathBuf::from(config),
            None => {
                let config_home = match env::var("XDG_CONFIG_HOME") {
                    Ok(config_home) => PathBuf::from(config_home),
                    Err(_) => PathBuf::from(env!["HOME"]).join(".config"),
                };
                config_home.join("please.toml")
            }
        };
        if config.exists() {
            let _ = self.load(config);
        }

        self
    }

    fn load(&mut self, config: PathBuf) -> Result<()> {
        let mut file = File::open(config)?;
        let mut content = String::new();
        file.read_to_string(&mut content)?;
        let mut defaults: Table = content.parse()?;
        let cmd = self.cmd.to_string();
        let bind = defaults.clone();
        if let Some(value) = bind.get(cmd.as_str()).and_then(|value| value.as_table()) {
            for (k, v) in value.iter() {
                defaults.insert(k.to_string(), v.clone());
            }
        }

        if defaults.get("assume-yes").and_then(|yes| yes.as_bool()).unwrap_or_default() {
            self.yes = true;
        }
        if defaults.get("su").and_then(|yes| yes.as_bool()).unwrap_or_default() {
            self.su = true;
        }
        if let Some(vendor) = defaults.get("vendor").and_then(|vendor| vendor.as_str()) {
            let vendor: Vendor = vendor.try_into().map_err(|err: String| eyre![err])?;
            self.vendor = Some(vendor);
        }

        Ok(())
    }
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

impl Display for Cmd {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Cmd::Install { .. } => write!(f, "install"),
            Cmd::Remove { .. } => write!(f, "remove"),
            Cmd::Upgrade { .. } => write!(f, "upgrade"),
            Cmd::Search { .. } => write!(f, "search"),
            Cmd::Info { .. } => write!(f, "info"),
            Cmd::Update => write!(f, "update"),
            Cmd::List => write!(f, "list"),
            Cmd::ListVendors => write!(f, "list-vendors"),
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
            _ => PlsCommand::List,
        }
    }
}
