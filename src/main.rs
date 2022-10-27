use std::any::Any;
use std::path::PathBuf;

use clap::Parser;
use cloudflare::endpoints::dns::{DnsRecord, ListDnsRecords};
use cloudflare::framework::{Environment, HttpApiClient, HttpApiClientConfig};
use cloudflare::framework::apiclient::ApiClient;
use cloudflare::framework::auth::Credentials;
use cloudflare::framework::response::ApiSuccess;
use domain::base::Dname;
use domain::master::entry::MasterRecord;
use domain::master::reader::{Reader, ReaderItem};
use domain::rdata::ZoneRecordData;

use crate::cli::Cli;
use crate::difference::Difference;

mod cli;
mod difference;

fn main() -> anyhow::Result<()> {
  let args = Cli::parse();
  let records = load_records(args.zone_files)?;

  for record in &records {
    println!("{:?}", record);
  }

  let credentials = Credentials::UserAuthToken { token: args.token };
  let config = HttpApiClientConfig::default();
  let environment = Environment::Production;

  let client = HttpApiClient::new(credentials, config, environment)?;
  let data = ListDnsRecords { zone_identifier: "9fec59e065f2865406ea7d3e264a7e59", params: Default::default() };
  let response: ApiSuccess<Vec<DnsRecord>> = client.request(&data)?;
  let cf_records = response.result;

  for record in &cf_records {
    println!("{:?}", record);
  }

  let difference = Difference::find(&records, &cf_records);

  println!("to create");
  for x in difference.to_create {
    println!("{}", x);
  }

  println!("to update");
  for x in difference.to_update {
    println!("{:?}", x);
  }

  println!("to delete");
  for x in difference.to_delete {
    println!("{:?}", x);
  }

  Ok(())
}

fn load_records(zone_files: Vec<PathBuf>) -> anyhow::Result<Vec<MasterRecord>> {
  let mut records = Vec::new();

  for file in zone_files {
    let mut reader = Reader::open(file)?;

    // todo: no unwrap
    while let Some(item) = reader.next_record().unwrap() {
      match item {
        ReaderItem::Record(record) => records.push(record),
        ReaderItem::Include { .. } => unimplemented!("include entries"),
        ReaderItem::Control { .. } => unimplemented!("control entries"),
      }
    }
  }

  Ok(records)
}
