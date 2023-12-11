use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rl_replay_api::{constants::LARGE_BOOST_PADS, stat_collector::PickupHandler};

fn player_pickup_is_some(c: &mut Criterion) {
    let mut pickup_map = PickupHandler::new();
    let rb = boxcars::RigidBody {
        sleeping: false,
        location: boxcars::Vector3f {
            x: LARGE_BOOST_PADS[0].x,
            y: LARGE_BOOST_PADS[0].y,
            z: 0.0,
        },
        rotation: boxcars::Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        linear_velocity: None,
        angular_velocity: None,
    };

    c.bench_function("pickup is Some(BoostPadSize::Large)", |b| {
        b.iter(|| pickup_map.try_pickup(black_box(&rb)))
    });
}

fn player_pickup_is_none(c: &mut Criterion) {
    let mut pickup_map = PickupHandler::new();
    let rb = boxcars::RigidBody {
        sleeping: false,
        location: boxcars::Vector3f {
            x: 0.0,
            y: 0.0,
            z: 0.0,
        },
        rotation: boxcars::Quaternion {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 0.0,
        },
        linear_velocity: None,
        angular_velocity: None,
    };

    c.bench_function("pickup is None", |b| {
        b.iter(|| pickup_map.try_pickup(black_box(&rb)))
    });
}

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
    name = benches;
    config = Criterion::default().significance_level(0.1).sample_size(80);
    targets = player_pickup_is_none, player_pickup_is_some
}

criterion_group! {
    name = collectors;
    config = Criterion::default().significance_level(0.1).sample_size(80);
    targets = process_replay_stat_collector, process_replay_base_collector
}

criterion_main!(collectors);
