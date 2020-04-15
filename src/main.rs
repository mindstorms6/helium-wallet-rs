use helium_wallet::{
    cmd::{balance, create, CmdRunner, hotspots, htlc, info, pay, verify, Opts},
    result::Result,
};
use std::process;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub struct Cli {
    #[structopt(flatten)]
    opts: Opts,

    #[structopt(flatten)]
    cmd: Cmd,
}

#[derive(Debug, StructOpt)]
pub enum Cmd {
    Info(info::Cmd),
    Verify(verify::Cmd),
    Balance(balance::Cmd),
    Hotspots(hotspots::Cmd),
    Create(create::Cmd),
    Pay(pay::Cmd),
    Htlc(htlc::Cmd),
}

fn main() {
    let cli = Cli::from_args();
    if let Err(e) = run(cli) {
        println!("error: {}", e);
        process::exit(1);
    }
}

fn run(cli: Cli) -> Result {
    let cmd: Box<dyn CmdRunner> = match cli.cmd {
        Cmd::Info(cmd) => Box::new(cmd),
        Cmd::Verify(cmd) => Box::new(cmd),
        Cmd::Balance(cmd) => Box::new(cmd),
        Cmd::Hotspots(cmd) => Box::new(cmd),
        Cmd::Create(cmd) => Box::new(cmd),
        Cmd::Pay(cmd) => Box::new(cmd),
        Cmd::Htlc(cmd) => Box::new(cmd),
    };
    cmd.run(cli.opts)
}
