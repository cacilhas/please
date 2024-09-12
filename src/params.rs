use std::{env, fmt::Display, fs::File, io::Read, path::PathBuf};
use clap::{ArgAction, Parser, Subcommand};
use eyre::{eyre, Result};
use toml::Table;
use crate::{vendors::PlsCommand, Vendor};


#[derive(Debug, Parser)]
#[command(about, author, name = "please", version)]
pub struct Params {
    /// skip settings
    #[arg(short = 'x', long, global = true, action = ArgAction::SetTrue)]
    pub skip_settings: bool,
    /// configuration file
    #[arg(short, long, global = true)]
    pub config: Option<String>,
    /// dry run (do not actually execute commands)
    #[arg(short, long, global = true, action = ArgAction::SetTrue)]
    pub dry_run: bool,
    /// assume yes for all prompts
    #[arg(short, long, global = true, action = ArgAction::SetTrue)]
    pub yes: bool,
    #[cfg(not(target_os = "windows"))]
    #[arg(short, long, global = true, action = ArgAction::SetTrue, help = "run as root (user must be sudoer)")]
    pub su: bool,
    /// set the installer command
    #[arg(short, long, global = true)]
    pub vendor: Option<Vendor>,
    #[command(subcommand)]
    pub cmd: Cmd,
}

#[derive(Clone, Debug, PartialEq, Subcommand)]
pub enum Cmd {
    /// install package(s)
    #[command()]
    Install {
        /// package(s) to be installed
        #[arg(name = "PACKAGE")]
        args: Vec<String>,
    },
    /// remove package(s)
    #[command()]
    Remove {
        /// package(s) to be removed
        #[arg(name = "PACKAGE")]
        args: Vec<String>,
    },
    /// upgrade package(s)
    #[command()]
    Upgrade {
        /// package(s) to be upgraded
        #[arg(name = "PACKAGE")]
        args: Vec<String>,
    },
    /// search for package(s)
    #[command()]
    Search {
        /// text to be searched
        #[arg(name = "QUERY")]
        args: String,
        /// paginate results
        #[arg(short, long, action = ArgAction::SetTrue)]
        paginate: bool,
        #[arg(skip)]
        pager: Option<String>,
    },
    /// get info for a package
    #[command()]
    Info {
        /// package for which to get info
        #[arg(name = "PACKAGE")]
        args: String,
    },
    /// update database
    #[command()]
    Update,
    /// list installed packages
    #[command()]
    List {
        /// paginate results
        #[arg(short, long, action = ArgAction::SetTrue)]
        paginate: bool,
        #[arg(skip)]
        pager: Option<String>,
    },
    /// list available vendors
    #[command()]
    ListVendors,
}

impl Params {
    pub fn config(mut self) -> Self {
        if self.skip_settings {
            return self;
        }

        #[cfg(target_os = "windows")]
        const XDG_CONFIG_HOME: &str = "APPDATA";
        #[cfg(not(target_os = "windows"))]
        const XDG_CONFIG_HOME: &str = "XDG_CONFIG_HOME";

        #[cfg(target_os = "windows")]
        const CONFIG_HOME: &str = "AppData";
        #[cfg(not(target_os = "windows"))]
        const CONFIG_HOME: &str = ".config";

        let config = match &self.config {
            Some(config) => PathBuf::from(config),
            None => {
                let config_home = match env::var(XDG_CONFIG_HOME) {
                    Ok(config_home) => PathBuf::from(config_home),
                    Err(_) => PathBuf::from(env!["HOME"]).join(CONFIG_HOME),
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

        match &self.cmd {
            Cmd::Search { args, .. } => {
                if defaults.get("pager").is_some() {
                    self.cmd = Cmd::Search {
                        pager: defaults
                            .get("pager")
                            .and_then(|pager| pager.as_str())
                            .map(|pager| pager.to_owned())
                            .filter(|pager| !pager.is_empty())
                            .map(|pager| pager.replace("$args", args)),
                        paginate: true,
                        args: args.to_owned(),
                    }
                }
            }
            Cmd::List { .. } => {
                if defaults.get("pager").is_some() {
                    self.cmd = Cmd::List {
                        pager: defaults
                            .get("pager")
                            .and_then(|pager| pager.as_str())
                            .map(|pager| pager.to_owned())
                            .filter(|pager| !pager.is_empty()),
                        paginate: true,
                    }
                }
            }
            _ => (),
        }

        if defaults.get("assume-yes").and_then(|yes| yes.as_bool()).unwrap_or_default() {
            self.yes = true;
        }
        if defaults.get("su").and_then(|yes| yes.as_bool()).unwrap_or_default() {
            self.su = true;
        }
        if self.vendor.is_none() {
            if let Some(vendor) = defaults.get("vendor").and_then(|vendor| vendor.as_str()) {
                let vendor: Vendor = vendor.try_into().map_err(|err: String| eyre![err])?;
                self.vendor = Some(vendor);
            }
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
            Cmd::Search { args, .. } => args.to_string(),
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
            Cmd::List { .. } => write!(f, "list"),
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
            Cmd::List { .. } => PlsCommand::List,
            _ => PlsCommand::List,
        }
    }
}
