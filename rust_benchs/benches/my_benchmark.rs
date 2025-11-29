#![feature(test)]

extern crate test;

use std::{fs::File, time::Instant};

use rust_benchs::{eratostenes, fibonacci, quicksort};
use std::io::Write;
use test::{Bencher, black_box};

fn perform_benchmark<F: Fn() -> T, T>(b: &mut Bencher, f: F, name: &str) {
    println!("Benchmarking {}...", name);
    let mut times = Vec::new();

    b.iter(|| {
        let start = Instant::now();
        f();
        times.push(start.elapsed());
    });

    let filename = format!("benchmark_times_native_{}.json", name);
    let mut file = File::create(filename).unwrap();
    let nanosecs = times
        .iter()
        .map(|d: &std::time::Duration| d.as_nanos())
        .collect::<Vec<_>>();
    writeln!(file, "{:?}", nanosecs).unwrap();

    eprintln!("Benchmark {} done.", name);
}

fn perform_benchmark_2<F: Fn(&mut T), T: Clone>(b: &mut Bencher, f: F, name: &str, arg: T) {
    println!("Benchmarking {}...", name);
    let mut times = Vec::new();

    b.iter(|| {
        let mut copy = arg.clone();
        let start = Instant::now();
        f(&mut copy);
        times.push(start.elapsed());
    });

    let filename = format!("benchmark_times_native_{}.json", name);
    let mut file = File::create(filename).unwrap();
    let nanosecs = times
        .iter()
        .map(|d: &std::time::Duration| d.as_nanos())
        .collect::<Vec<_>>();
    writeln!(file, "{:?}", nanosecs).unwrap();

    eprintln!("Benchmark {} done.", name);
}

#[bench]
fn bench_all(b: &mut Bencher) {
    let tests_json = std::fs::read_to_string("../inputs/tests.json").unwrap();
    let tests: Vec<serde_json::Value> = serde_json::from_str(&tests_json).unwrap();

    for test in tests {
        let typ = test["type"].as_u64().unwrap();
        match typ {
            1 => {
                let fun = match test["fun"].as_str() {
                    Some("fibonacci") => fibonacci,
                    Some("eratostenes") => eratostenes,
                    _ => continue,
                };
                let arg = test["arg"].as_u64().unwrap();
                let name = test["name"].as_str().unwrap();
                perform_benchmark(b, || fun(arg as u32), name);
            }
            2 => {
                let fun = match test["fun"].as_str() {
                    Some("quicksort") => quicksort,
                    _ => continue,
                };
                let arg: Vec<i32> = serde_json::from_value(test["arg"].clone()).unwrap();
                let name = test["name"].as_str().unwrap();
                perform_benchmark_2(b, fun, name, &arg);
            }
            _ => continue,
        }
    }
}
