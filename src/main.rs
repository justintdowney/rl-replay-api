#![feature(hash_set_entry)]
use boxcars::*;
use serde::{Deserialize, Serialize};
use subtr_actor::*;

use workspace::stat_collector::StatCollector;

fn main() {
    let data = std::fs::read("C:/Users/jtd/repos/rust/advanced-stat-parser-2/gg.replay").unwrap();
    let parsing = boxcars::ParserBuilder::new(&data[..])
        .always_check_crc()
        .must_parse_network_data()
        .parse();
    let replay = parsing.unwrap();

    let collector = StatCollector::new();
    println!("collecting replay data");
    let replay_data = collector.get_stat_data(&replay).unwrap();
    println!("done");
    println!("{}", serde_json::to_string_pretty(&replay_data).unwrap());
}
