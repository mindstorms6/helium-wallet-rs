use crate::{
    cmd::{get_password, load_wallet, open_output_file, verify, Opts},
    format::{self, Format},
    pwhash::PWHash,
    result::Result,
    wallet::Wallet,
};
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Upgrade a wallet to the latest supported version of the given
/// format. The same password is used to decrypt the old and encrypt
/// the new wallet.
pub enum Cmd {
    Basic(Basic),
}

#[derive(Debug, StructOpt)]
/// Upgrade to the latest basic wallet format
pub struct Basic {
    #[structopt(short, long, default_value = "wallet.key")]
    /// Output file to store the key in
    output: PathBuf,

    #[structopt(long)]
    /// Overwrite an existing file
    force: bool,
}


impl Cmd {
    pub fn run(&self, opts: Opts) -> Result {
        match self {
            Cmd::Basic(cmd) => cmd.run(opts),
            
        }
    }
}

impl Basic {
    pub fn run(&self, opts: Opts) -> Result {
        let password = get_password(false)?;
        let wallet = load_wallet(opts.files)?;
        let keypair = wallet.decrypt(password.as_bytes())?;

        let format = format::Basic {
            pwhash: PWHash::argon2id13_default(),
        };
        let new_wallet = Wallet::encrypt(&keypair, password.as_bytes(), Format::Basic(format))?;
        let mut writer = open_output_file(&self.output, !self.force)?;
        new_wallet.write(&mut writer)?;
        verify::print_result(&new_wallet, true, opts.format)
    }
}


