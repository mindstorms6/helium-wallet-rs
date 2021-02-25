use crate::{
    cmd::{get_file_extension, get_password, get_seed_words, verify, Opts},
    format::{self, Format},
    keypair::{Keypair, Seed},
    mnemonic::mnemonic_to_entropy,
    pwhash::PWHash,
    result::Result,
    wallet::Wallet,
};
use std::{fs, io, path::PathBuf};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
/// Create a new wallet
pub enum Cmd {
    Basic(Basic),
    Sharded(Sharded),
}

#[derive(Debug, StructOpt)]
/// Create a new basic wallet
pub struct Basic {
    #[structopt(short, long, default_value = "wallet.key")]
    /// Output file to store the key in
    output: PathBuf,

    #[structopt(long)]
    /// Overwrite an existing file
    force: bool,

    #[structopt(long)]
    /// Use space separated seed words to create the wallet
    seed: bool,
}

#[derive(Debug, StructOpt)]
/// Create a new sharded wallet
pub struct Sharded {
    #[structopt(short, long, default_value = "wallet.key")]
    /// Output file to store the key in
    output: PathBuf,

    #[structopt(long)]
    /// Overwrite an existing file
    force: bool,

    #[structopt(short = "n", long = "shards", default_value = "5")]
    /// Number of shards to break the key into
    key_share_count: u8,

    #[structopt(short = "k", long = "required-shards", default_value = "3")]
    /// Number of shards required to recover the key
    recovery_threshold: u8,

    #[structopt(long)]
    /// Use space separated seed words to create the wallet
    seed: bool,
}

impl Cmd {
    pub fn run(&self, opts: Opts) -> Result {
        match self {
            Cmd::Basic(cmd) => cmd.run(opts),
            Cmd::Sharded(cmd) => cmd.run(opts),
        }
    }
}

impl Basic {
    pub fn run(&self, opts: Opts) -> Result {
        let seed_words = if self.seed {
            Some(get_seed_words()?)
        } else {
            None
        };
        let password = get_password(true)?;
        let keypair = gen_keypair(seed_words)?;
        let format = format::Basic {
            pwhash: PWHash::argon2id13_default(),
        };
        let wallet = Wallet::encrypt(&keypair, password.as_bytes(), Format::Basic(format))?;
        let mut writer = open_output_file(&self.output, !self.force)?;
        wallet.write(&mut writer)?;
        verify::print_result(&wallet, true, opts.format)
    }
}

impl Sharded {
    pub fn run(&self, opts: Opts) -> Result {
        let seed_words = if self.seed {
            Some(get_seed_words()?)
        } else {
            None
        };
        let password = get_password(true)?;

        let keypair = gen_keypair(seed_words)?;
        let format = format::Sharded {
            key_share_count: self.key_share_count,
            recovery_threshold: self.recovery_threshold,
            pwhash: PWHash::argon2id13_default(),
            key_shares: vec![],
        };
        let wallet = Wallet::encrypt(&keypair, password.as_bytes(), Format::Sharded(format))?;

        let extension = get_file_extension(&self.output);
        for (i, shard) in wallet.shards()?.iter().enumerate() {
            let mut filename = self.output.clone();
            let share_extension = format!("{}.{}", extension, (i + 1).to_string());
            filename.set_extension(share_extension);
            let mut writer = open_output_file(&filename, !self.force)?;
            shard.write(&mut writer)?;
        }
        verify::print_result(&wallet, true, opts.format)
    }
}

pub fn gen_keypair(seed_words: Option<Vec<String>>) -> Result<Keypair> {
    match seed_words {
        Some(words) => {
            let entropy = mnemonic_to_entropy(words)?;
            Ok(Keypair::gen_keypair_from_seed(&Seed(entropy)))
        }
        None => Ok(Keypair::gen_keypair()),
    }
}

fn open_output_file(filename: &PathBuf, create: bool) -> io::Result<fs::File> {
    fs::OpenOptions::new()
        .write(true)
        .create(true)
        .create_new(create)
        .open(filename)
}
