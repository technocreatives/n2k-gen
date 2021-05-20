use embedded_hal_can::{Filter, Frame};
use log::*;
use n2k::BusError;
use n2k_messages::Pgns;
use std::{
    collections::HashSet,
    convert::{TryFrom, TryInto},
};
use structopt::StructOpt;

pub struct CanFrame {
    pub frame_id: u32,
    pub frame_body: [u8; 8],
}

pub struct Timestamp {
    pub seconds: u64,
    pub nanos: u64,
}
pub struct DumpEntry {
    pub timestamp: Timestamp,
    pub can_interface: String,
    pub can_frame: CanFrame,
}

struct CanDumpReceiver {
    entries: Vec<DumpEntry>,
    ctr: usize,
}

impl CanDumpReceiver {
    pub fn from_candump_file(dump_file: &str) -> Self {
        let dump = std::fs::read_to_string(dump_file).unwrap();
        Self::from_candump(dump.lines().map(|v| v.to_owned()).collect())
    }

    pub fn from_candump(lines: Vec<String>) -> Self {
        Self {
            entries: lines
                .iter()
                .filter_map(|s| {
                    let r = canutils::candump_parser::dump_entry(&s);
                    if let Err(e) = r.as_ref() {
                        error!("failed to parse CAN dump line: {:?}", e)
                    }
                    Some(r.ok()?.1)
                })
                .map(|d| DumpEntry {
                    timestamp: Timestamp {
                        seconds: d.timestamp().seconds,
                        nanos: d.timestamp().nanos,
                    },
                    can_interface: d.can_interface().to_owned(),
                    can_frame: CanFrame {
                        frame_id: d.can_frame().frame_id,
                        frame_body: d.can_frame().frame_body.to_be_bytes(),
                    },
                })
                .collect(),
            ctr: 0,
        }
    }

    pub fn from_n2kdump(dump_file: &str) -> Self {
        let dump = std::fs::read_to_string(dump_file).unwrap();
        let mut entries: Vec<_> = vec![];
        for line in dump.lines() {
            //Rx 200698 09 f1 0d 72 00 f8 17 28 ff 7f ff ff From:72 Rudder
            let tokens: Vec<_> = line.split(" ").collect();
            let timestamp: u32 = tokens[1].parse().unwrap();
            let frame_id: Vec<_> = tokens[2..][..4]
                .iter()
                .map(|v| u8::from_str_radix(v, 16).unwrap())
                .collect();
            let frame_data: Vec<_> = tokens[6..][..8]
                .iter()
                .map(|v| u8::from_str_radix(v, 16).unwrap())
                .collect();
            let frame_id = u32::from_be_bytes(frame_id.try_into().unwrap());

            entries.push(DumpEntry {
                timestamp: Timestamp {
                    seconds: (timestamp * 1000) as u64,
                    nanos: 0,
                },
                can_interface: "yes".to_string(),
                can_frame: CanFrame {
                    frame_id,
                    frame_body: frame_data.try_into().unwrap(),
                },
            });
        }
        Self { entries, ctr: 0 }
    }
}
impl embedded_hal_can::Interface for CanDumpReceiver {
    type Id = n2k::Id;
    type Frame = n2k::CanFrame;

    type Error = ();
    type Filter = MockFilter;
}
struct MockFilter {}

impl Filter for MockFilter {
    type Id = n2k::Id;

    fn from_id(_id: Self::Id) -> Self {
        panic!();
    }

    fn accept_all() -> Self {
        panic!();
    }

    fn from_mask(_mask: u32, _filter: u32) -> Self {
        panic!();
    }
}

impl embedded_hal_can::Receiver for CanDumpReceiver {
    fn receive(&mut self) -> nb::Result<Self::Frame, Self::Error> {
        if self.ctr >= self.entries.len() {
            return Err(nb::Error::WouldBlock);
        }
        let entry = &self.entries[self.ctr];
        self.ctr += 1;

        let id = n2k::Id::try_from(entry.can_frame.frame_id).unwrap();
        Ok(n2k::CanFrame::new(id, &entry.can_frame.frame_body))
    }

    fn set_filter(&mut self, filter: Self::Filter) {
        panic!();
    }

    fn clear_filter(&mut self) {
        panic!();
    }
}

#[derive(Debug)]
pub enum DumpFormat {
    CanDump,
    N2kDump,
}

impl std::str::FromStr for DumpFormat {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "CanDump" => DumpFormat::CanDump,
            "N2kDump" => DumpFormat::N2kDump,
            _ => return Err("invalid format".to_string()),
        })
    }
}
#[derive(StructOpt)]
struct Opts {
    pub dump_file: String,
    #[structopt(short, long, default_value = "CanDump")]
    pub format: DumpFormat,
    #[structopt(short, long)]
    pub show_unknown: bool,
}
fn main() {
    env_logger::init();
    let opts = Opts::from_args();
    //Some([127237].iter().cloned().collect())
    let receiver = match opts.format {
        DumpFormat::CanDump => CanDumpReceiver::from_candump_file(&opts.dump_file),
        DumpFormat::N2kDump => CanDumpReceiver::from_n2kdump(&opts.dump_file),
    };
    // let receiver = CanDumpReceiver::from_n2kdump(&opts.dump_file);
    let mut bus: n2k::Bus<_, n2k_messages::PgnRegistry> = n2k::Bus::new(receiver);

    loop {
        let result = bus.receive();
        if !matches!(result, Ok(None)) {
            match result {
                Err(nb::Error::Other(BusError::PgnError(n2k_messages::N2kError::UnknownPgn(
                    _,
                )))) => {
                    if opts.show_unknown {
                        println!("{:?}", &result);
                    }
                }
                r => {
                    println!("{:?}", &r);
                }
            };
        }
    }
}
