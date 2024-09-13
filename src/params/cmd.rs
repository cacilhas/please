use clap::{ArgAction, Subcommand};
use eyre::Result;


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

impl Cmd {
    fn init(mut self, config: Option<String>) -> Result<Self> {
        Ok(self)
    }
}
