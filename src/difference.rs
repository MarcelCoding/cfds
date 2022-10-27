use cloudflare::endpoints::dns::{DnsContent, DnsRecord};
use domain::master::entry::MasterRecord;
use domain::rdata::ZoneRecordData;

use crate::difference::State::{Outdated, Present};

#[derive(Debug)]
pub(crate) struct Difference<'a> {
  pub(crate) to_create: Vec<&'a MasterRecord>,
  pub(crate) to_update: Vec<(&'a DnsRecord, &'a MasterRecord)>,
  pub(crate) to_delete: Vec<&'a DnsRecord>,
}

enum State {
  Present,
  Outdated,
}

impl<'a> Difference<'a> {
  pub(crate) fn find(records: &'a Vec<MasterRecord>, cf_records: &'a Vec<DnsRecord>) -> Self {
    let mut to_create = Vec::new();
    let mut to_update = Vec::new();
    let mut to_delete = Vec::new();
    for x in cf_records {
      to_delete.push(x);
    }

    for record in records {
      let mut found = false;

      for (i, cf_record) in cf_records.iter().enumerate().rev() {
        if let Some(state) = State::from(record, cf_record) {
          to_delete.remove(i);

          if let Outdated = state {
            to_update.push((cf_record, record));
            found = true;
            break;
          }
        }
      }

      if !found {
        to_create.push(record);
      }
    }

    Difference { to_create, to_update, to_delete }
  }
}

impl State {
  fn from(record: &MasterRecord, cf_record: &DnsRecord) -> Option<Self> {
    if record.owner().to_string() != cf_record.name {
      return None;
    }

    match record.data() {
      ZoneRecordData::A(data) => {
        if let DnsContent::A { content } = &cf_record.content {
          return if &data.addr() == content {
            Some(Present)
          } else {
            Some(Outdated)
          };
        }
      }
      ZoneRecordData::Aaaa(data) => {
        if let DnsContent::AAAA { content } = &cf_record.content {
          return if &data.addr() == content {
            Some(Present)
          } else {
            Some(Outdated)
          };
        }
      }
      ZoneRecordData::Cname(_) => unimplemented!("record type"),
      ZoneRecordData::Hinfo(_) => unimplemented!("record type"),
      ZoneRecordData::Mb(_) => unimplemented!("record type"),
      ZoneRecordData::Md(_) => unimplemented!("record type"),
      ZoneRecordData::Mf(_) => unimplemented!("record type"),
      ZoneRecordData::Minfo(_) => unimplemented!("record type"),
      ZoneRecordData::Mr(_) => unimplemented!("record type"),
      ZoneRecordData::Mx(_) => unimplemented!("record type"),
      ZoneRecordData::Ns(_) => unimplemented!("record type"),
      ZoneRecordData::Ptr(_) => unimplemented!("record type"),
      ZoneRecordData::Soa(_) => unimplemented!("record type"),
      ZoneRecordData::Txt(data) => {
        if let DnsContent::TXT { content } = &cf_record.content {
          return if &data.to_string() == content {
            Some(Present)
          } else {
            Some(Outdated)
          };
        }
      }
      ZoneRecordData::Srv(_) => unimplemented!("record type"),
      ZoneRecordData::Aaaa(_) => unimplemented!("record type"),
      ZoneRecordData::Dnskey(_) => unimplemented!("record type"),
      ZoneRecordData::Rrsig(_) => unimplemented!("record type"),
      ZoneRecordData::Nsec(_) => unimplemented!("record type"),
      ZoneRecordData::Ds(_) => unimplemented!("record type"),
      ZoneRecordData::Dname(_) => unimplemented!("record type"),
      ZoneRecordData::Nsec3(_) => unimplemented!("record type"),
      ZoneRecordData::Nsec3param(_) => unimplemented!("record type"),
      ZoneRecordData::Cdnskey(_) => unimplemented!("record type"),
      ZoneRecordData::Cds(_) => unimplemented!("record type"),
      ZoneRecordData::Other(_) => unimplemented!("record type"),
      _ => unimplemented!("record type")
    }

    None
  }
}
