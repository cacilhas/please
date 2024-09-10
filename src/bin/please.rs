use std::process::exit;

use clap::Parser;
use eyre::Result;
use please_install::{Params, PlsCommand, Vendor};


fn main() -> Result<()> {
    let params = Params::parse();
    let dry_run = params.dry_run;
    let assume_yes = params.yes;
    let args = params.cmd.args();
    let cmd: PlsCommand = (&params.cmd).into();
    let vendor = match params.vendor {
        Some(vendor) => vendor,
        None => Vendor::new()?,
    };
    #[cfg(target_os = "windows")]
    let su = false;
    #[cfg(not(target_os = "windows"))]
    let su = params.su;

    let status = vendor.execute(cmd, &args, assume_yes, su, dry_run)?;
    exit(status);
}
