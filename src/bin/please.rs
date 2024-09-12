use clap::Parser;
use eyre::Result;
use please_install::Params;


fn main() -> Result<()> {
    let args = Params::parse();
    println!("{:?}", args);

    Ok(())
}
