mod cmd;

use clap::{ArgAction, Parser};
use self::cmd::Cmd;


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
