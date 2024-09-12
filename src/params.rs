use clap::{ArgAction, Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(about, author, name = "please", version)]
pub struct Params {

    /// configuration file
    #[arg(short, long, global = true)]
    pub config: Option<String>,

    /// dry run (do not actually execute commands)
    #[arg(short, long, global = true, action = ArgAction::SetTrue)]
    pub dry_run: bool,

    #[cfg(not(target_os = "windows"))]
    /// run as root (user must be sudoer)
    #[arg(short, long, global = true, action = ArgAction::SetTrue)]
    pub su: bool,

    /// set the installer backend
    #[arg(short, long, global = true)]
    pub vendor: Option<String>,  /* TODO: use dedicated struct */

    /// skip config file
    #[arg(short = 'x', long, global = true, action = ArgAction::SetTrue)]
    pub skip_config: bool,

    /// assume yes for all prompts
    #[arg(short, long, global = true, action = ArgAction::SetTrue)]
    pub yes: bool,

    #[command(subcommand)]
    pub cmd: Option<Cmd>,
}

#[derive(Clone, Debug, PartialEq, Subcommand)]
pub enum Cmd {
    /// get information for a package
    #[command()]
    Info {
        /// paginate results
        #[arg(short, long, action = ArgAction::SetTrue)]
        paginate: bool,

        /// package for which to get information
        #[arg(name = "PACKAGE")]
         package: String,

        #[arg(skip)]
        pager: Option<String>,
    },

    /// install package(s)
    #[command()]
    Install {
        /// package(s) to be installed
        #[arg(name = "PACKAGE")]
        packages: Vec<String>,
    },

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
    ListVendors {
        /// paginate results
        #[arg(short, long, action = ArgAction::SetTrue)]
        paginate: bool,

        #[arg(skip)]
        pager: Option<String>,
    },

    /// remove package(s)
    #[command()]
    Remove {
        /// package(s) to be removed
        #[arg(name = "PACKAGE")]
        packages: Vec<String>,
    },

    /// search for packages
    #[command()]
    Search {
        /// text to be searched
        #[arg(name = "QUERY")]
        query: Vec<String>,
    },

    /// Update system package database
    #[command()]
    Update,

    /// upgrade package(s)
    #[command()]
    Upgrade {
        /// package(s) to be upgraded
        #[arg(name = "PACKAGE")]
        packages: Vec<String>,
    }
}
