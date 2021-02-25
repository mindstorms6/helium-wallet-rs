use crate::{
    cmd::{api_url, load_wallet, print_json, Opts, OutputFormat},
    result::Result,
    wallet::Wallet,
};
use helium_api::{Account, Client, Hnt, Hst};
// use prettytable::Table;
use qr2term::print_qr;
use serde_json::json;
use structopt::StructOpt;

/// Get wallet information
#[derive(Debug, StructOpt)]
pub struct Cmd {
    /// Display QR code for a given single wallet.
    #[structopt(long = "qr")]
    qr_code: bool,
}

impl Cmd {
    pub fn run(&self, opts: Opts) -> Result {
        let wallet = load_wallet(opts.files)?;
        if self.qr_code {
            let address = wallet.address()?;
            print_qr(&address)?;
            Ok(())
        } else {
            let client = Client::new_with_base_url(api_url());
            let account = client.get_account(&wallet.address()?)?;
            print_wallet(&wallet, &account, opts.format)
        }
    }
}

fn print_wallet(wallet: &Wallet, account: &Account, format: OutputFormat) -> Result {
    match format {
        OutputFormat::Table => {
            Ok(())
        }
        OutputFormat::Json => {
            let table = json!({
                // "sharded": wallet.is_sharded(),
                "pwhash": wallet.pwhash().to_string(),
                "account": account,
            });
            print_json(&table)
        }
    }
}
