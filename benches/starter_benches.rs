use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn process_replay_stat_collector(c: &mut Criterion) {
    let root = std::env::current_dir().unwrap();
    let relative_path = relative_path::RelativePath::new("rr.replay");
    let full_path = relative_path.to_path(&root);
    println!("{:?}", full_path);
    let data = std::fs::read(full_path).unwrap();
    let parsing = boxcars::ParserBuilder::new(black_box(&data))
        .always_check_crc()
        .must_parse_network_data()
        .parse()
        .unwrap();

    c.bench_function("parsing bench", |b| {
        b.iter(|| {
            let collector = rl_replay_api::stat_collector::StatCollector::new();

            collector.analyze(black_box(&parsing)).unwrap();
        })
    });
}

fn process_replay_base_collector(c: &mut Criterion) {
    let root = std::env::current_dir().unwrap();
    let relative_path = relative_path::RelativePath::new("rr.replay");
    let full_path = relative_path.to_path(&root);
    println!("{:?}", full_path);
    let data = std::fs::read(full_path).unwrap();
    let parsing = boxcars::ParserBuilder::new(black_box(&data))
        .always_check_crc()
        .must_parse_network_data()
        .parse()
        .unwrap();

    c.bench_function("parsing bench replaycollector", |b| {
        b.iter(|| {
            let collector = subtr_actor::ReplayDataCollector::new();

            collector
                .get_replay_data(black_box(&parsing))
                .unwrap()
                .as_json()
                .unwrap();
        })
    });
}

criterion_group! {
    name = collectors;
    config = Criterion::default().significance_level(0.1).sample_size(80);
    targets = process_replay_stat_collector, process_replay_base_collector
}

criterion_main!(collectors);
