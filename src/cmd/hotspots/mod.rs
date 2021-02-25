use crate::{
    cmd::{api_url, collect_addresses, print_json, Opts, OutputFormat},
    result::Result,
};
use helium_api::{Client, Hotspot};
// use prettytable::{format, Table};
use serde_json::json;
use structopt::StructOpt;

pub mod transfer;

#[derive(Debug, StructOpt)]
/// Display list of hotspots associated with wallet
/// or transfer a hotspot to another wallet
pub enum Cmd {
    List(List),
    Transfer(transfer::Transfer),
}

impl Cmd {
    pub fn run(self, opts: Opts) -> Result {
        match self {
            Self::List(cmd) => cmd.run(opts),
            Self::Transfer(cmd) => cmd.run(opts),
        }
    }
}

#[derive(Debug, StructOpt)]
/// Get the list of hotspots for one or more wallet addresses
pub struct List {
    /// Addresses to get hotspots for
    #[structopt(short = "a", long = "address")]
    addresses: Vec<String>,
}

impl List {
    pub fn run(&self, opts: Opts) -> Result {
        let client = Client::new_with_base_url(api_url());
        let mut results: Vec<(String, Result<Vec<Hotspot>>)> =
            Vec::with_capacity(self.addresses.len());
        for address in collect_addresses(opts.files, self.addresses.clone())? {
            results.push((address.to_string(), client.get_hotspots(&address)));
        }
        print_results(results, opts.format)
    }
}

fn print_results(results: Vec<(String, Result<Vec<Hotspot>>)>, format: OutputFormat) -> Result {
    match format {
        OutputFormat::Table => {
            
        }
        OutputFormat::Json => {
            let mut table = Vec::with_capacity(results.len());
            for (address, result) in results {
                let mut table_hotspots = vec![];
                if let Ok(hotspots) = result {
                    for hotspot in hotspots {
                        table_hotspots.push(json!({
                            "address": hotspot.address,
                            "name":  hotspot.name.unwrap_or_else(|| "unknown".to_string()),
                            "location": hotspot.location.unwrap_or_else(|| "uknnown".to_string()),
                            "city":
                                hotspot
                                    .geocode
                                    .short_city
                                .unwrap_or_else(|| "unknown".to_string()),
                            "state":
                                hotspot
                                    .geocode
                                    .short_state
                                .unwrap_or_else(|| "unknown".to_string())
                        }))
                    }
                };
                table.push(json!({
                    "address": address,
                    "hotspots": table_hotspots,
                }));
            }
            print_json(&table)
        }
    }
}
