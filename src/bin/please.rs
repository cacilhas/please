use std::{env, process::exit};

use clap::Parser;
use eyre::Result;
use please_install::{Cmd, Params, PlsCommand, Vendor};
use strum::IntoEnumIterator;


fn main() -> Result<()> {
    let params = Params::parse().config();
    let dry_run = params.dry_run;
    let assume_yes = params.yes;
    let args = params.cmd.args();
    let mut use_pager: Option<String> = None;

    if params.cmd == Cmd::ListVendors {
        for vendor in Vendor::iter() {
            if vendor.is_available() {
                println!("{:?}", vendor);
            }
        }
        return Ok(())
    }

    #[cfg(not(target_os = "windows"))]
    if let Cmd::Search { pager, paginate, .. } = params.cmd.clone() {
        if paginate {
            use_pager = pager
                .or_else(|| env::var("PAGER").ok())
                .or_else(|| Some("less".to_string()));
        }
    }

    let cmd: PlsCommand = (&params.cmd).into();
    let vendor = match params.vendor {
        Some(vendor) => vendor,
        None => Vendor::new()?,
    };
    #[cfg(target_os = "windows")]
    let su = false;
    #[cfg(not(target_os = "windows"))]
    let su = params.su;

    let status = vendor.execute(cmd, &args, assume_yes, su, dry_run, use_pager)?;
    exit(status);
}
