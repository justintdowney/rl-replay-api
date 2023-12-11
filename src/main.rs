use relative_path::RelativePath;
use rl_replay_api::stat_collector::StatCollector;
use std::env::current_dir;

fn main() {
    let root = current_dir().unwrap();
    let relative_path = RelativePath::new("rr.replay");
    let full_path = relative_path.to_path(&root);
    println!("{:?}", full_path);
    let data = std::fs::read(full_path).unwrap();
    let parsing = boxcars::ParserBuilder::new(&data[..])
        .always_check_crc()
        .must_parse_network_data()
        .parse();
    let replay = parsing.unwrap();

    let collector = StatCollector::new();
    println!("collecting replay data");
    let replay_data = collector.analyze(&replay).unwrap();
    println!("done");
    println!("{}", serde_json::to_string_pretty(&replay_data).unwrap());
}
