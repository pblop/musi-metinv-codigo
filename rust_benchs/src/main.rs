use std::io::Write;
use std::{fs::File, time::Instant};

use rand::Rng;
use rand::seq::IndexedRandom;
use rust_benchs::{
    eratostenes, fibonacci, mandelbrot, matrix_multiply, montecarlo, nqueens, output_matrix_size,
    quicksort,
};

const WARMUP_ITERS: u32 = 100;
const WARMUP_TIME: u32 = 30; // seconds

pub fn benchmark<T: Copy + std::ops::AddAssign + Default + std::fmt::Display>(
    executions: u64,
    runtime: &str,
    name: &str,
    fun: fn(T) -> T,
    arg: T,
) {
    let mut times = Vec::new();
    let mut accumulator: T = Default::default();
    let mut i = 0;

    eprintln!("Warming up {}...", name);
    let warmup_begin = Instant::now();
    while i < WARMUP_ITERS && warmup_begin.elapsed().as_secs() < WARMUP_TIME as u64 {
        let result = fun(arg);

        accumulator += result;
        i += 1;
    }
    eprintln!(
        "Warmup done in {} iterations, executing {} times...",
        i, executions
    );

    i = 0;
    for _ in 0..executions {
        let start = Instant::now();
        let result = fun(arg);
        let elapsed = start.elapsed();

        accumulator += result;
        times.push(elapsed.as_nanos());
        i += 1;
    }

    let filename = format!("benchmark_times_{}_{}.json", runtime, name);
    let mut file = File::create(filename).unwrap();
    writeln!(file, "{:?}", times).unwrap();

    eprintln!("Benchmarking of {} done.", name);
}

pub fn benchmark_2<T: Clone + std::ops::AddAssign + Default + std::fmt::Display>(
    executions: u64,
    runtime: &str,
    name: &str,
    fun: fn(&mut [T]),
    arg: Vec<T>,
) {
    let mut times = Vec::new();
    let mut accumulator: T = Default::default();
    let mut i = 0;

    eprintln!("Warming up {}...", name);
    let warmup_begin = Instant::now();
    while i < WARMUP_ITERS && warmup_begin.elapsed().as_secs() < WARMUP_TIME as u64 {
        let mut marg = arg.clone();
        fun(&mut marg);

        for item in 0..10 {
            accumulator += marg[item].clone();
        }
        i += 1;
    }
    eprintln!(
        "Warmup done in {} iterations, executing {} times...",
        i, executions
    );

    i = 0;
    for _ in 0..executions {
        let mut marg = arg.clone();
        let start = Instant::now();
        fun(&mut marg);
        let elapsed = start.elapsed();

        for item in 0..10 {
            accumulator += marg[item].clone();
        }
        times.push(elapsed.as_nanos());
        i += 1;
    }

    eprintln!("Accumulator after benchmark {} is {}", name, accumulator);

    let filename = format!("benchmark_times_{}_{}.json", runtime, name);
    let mut file = File::create(filename).unwrap();
    writeln!(file, "{:?}", times).unwrap();

    eprintln!("Benchmarking of {} done.", name);
}

pub fn benchmark_4(
    executions: u64,
    runtime: &str,
    name: &str,
    fun: fn(&[f64], &[f64], &mut [f64], usize),
    a: Vec<f64>,
    b: Vec<f64>,
    width: usize,
) {
    let mut times = Vec::new();
    let mut accumulator: f64 = 0.0;
    let mut i = 0;

    let mut rng = rand::rng();

    eprintln!("Warming up {}...", name);
    let warmup_begin = Instant::now();
    let mut mc = vec![0f64; output_matrix_size(width, a.len())];
    while i < WARMUP_ITERS && warmup_begin.elapsed().as_secs() < WARMUP_TIME as u64 {
        fun(&a, &b, &mut mc, width);

        for _ in 0..10 {
            // Get 10 random items to avoid optimizing away the computation
            accumulator += mc.choose(&mut rng).unwrap();
        }
        i += 1;
    }
    eprintln!(
        "Warmup done in {} iterations, executing {} times...",
        i, executions
    );

    i = 0;
    for _ in 0..executions {
        let start = Instant::now();
        fun(&a, &b, &mut mc, width);
        let elapsed = start.elapsed();

        for _ in 0..10 {
            // Get 10 random items to avoid optimizing away the computation
            accumulator += mc.choose(&mut rng).unwrap();
        }
        times.push(elapsed.as_nanos());
        i += 1;
    }

    eprintln!("Accumulator after benchmark {} is {}", name, accumulator);

    let filename = format!("benchmark_times_{}_{}.json", runtime, name);
    let mut file = File::create(filename).unwrap();
    writeln!(file, "{:?}", times).unwrap();

    eprintln!("Benchmarking of {} done.", name);
}

pub fn benchmark_5(
    executions: u64,
    runtime: &str,
    name: &str,
    fun: fn(u32, u32, u32) -> Vec<u32>,
    width: u32,
    height: u32,
    max_iter: u32,
) {
    let mut times = Vec::new();
    let mut accumulator: u32 = 0;
    let mut i = 0;

    let mut rng = rand::rng();

    eprintln!("Warming up {}...", name);
    let warmup_begin = Instant::now();
    while i < WARMUP_ITERS && warmup_begin.elapsed().as_secs() < WARMUP_TIME as u64 {
        let result = fun(width, height, max_iter);

        for _ in 0..10 {
            // Get 10 random items to avoid optimizing away the computation
            let index = rng.random_range(0..result.len());
            accumulator += result[index as usize];
        }
        for item in result {
            accumulator += item;
        }
        i += 1;
    }
    eprintln!(
        "Warmup done in {} iterations, executing {} times...",
        i, executions
    );

    i = 0;
    for _ in 0..executions {
        let start = Instant::now();
        let result = fun(width, height, max_iter);
        let elapsed = start.elapsed();

        for _ in 0..10 {
            // Get 10 random items to avoid optimizing away the computation
            let index = rng.random_range(0..result.len());
            accumulator += result[index as usize];
        }
        times.push(elapsed.as_nanos());
        i += 1;
    }

    eprintln!("Accumulator after benchmark {} is {}", name, accumulator);

    let filename = format!("benchmark_times_{}_{}.json", runtime, name);
    let mut file = File::create(filename).unwrap();
    writeln!(file, "{:?}", times).unwrap();

    eprintln!("Benchmarking of {} done.", name);
}

macro_rules! benchmark {
    ($duration:expr, $name:ident, $arg:expr) => {
        benchmark($duration, stringify!($name), $name, $arg);
    };
}

#[cfg(all(target_arch = "wasm32", target_env = "p2"))]
fn get_target_string() -> &'static str {
    "wasi"
}

#[cfg(not(all(target_arch = "wasm32", target_env = "p2")))]
fn get_target_string() -> &'static str {
    "native"
}

pub fn main() {
    let runtime_name = std::env::args()
        .nth(1)
        .unwrap_or_else(|| get_target_string().to_string());

    let tests_json = std::fs::read_dir("../inputs")
        .unwrap()
        .filter(|entry| entry.as_ref().unwrap().path().extension().unwrap() == "json")
        .map(|entry| std::fs::read_to_string(entry.unwrap().path()).unwrap())
        .collect::<Vec<_>>();

    println!("Loaded {} tests.", tests_json.len());

    for test_json in tests_json {
        let test = serde_json::from_str::<serde_json::Value>(&test_json).unwrap();
        let typ = test["type"].as_u64().unwrap();
        let executions = test["executions"].as_u64().unwrap();
        let name = test["name"].as_str().unwrap();
        match typ {
            1 => {
                let fun: fn(u32) -> u32 = match test["fun"].as_str() {
                    Some("fibonacci") => fibonacci,
                    Some("eratostenes") => eratostenes,
                    Some("nqueens") => nqueens,
                    _ => continue,
                };
                let arg = test["arg"].as_u64().unwrap();
                benchmark(executions, &runtime_name, name, fun, arg as u32);
            }
            2 => {
                let fun = match test["fun"].as_str() {
                    Some("quicksort") => quicksort,
                    _ => continue,
                };
                let arg: Vec<i32> = serde_json::from_value(test["arg"].clone()).unwrap();
                benchmark_2(executions, &runtime_name, name, fun, arg);
            }
            3 => {
                let fun: fn(f64) -> f64 = match test["fun"].as_str() {
                    Some("montecarlo") => montecarlo,
                    _ => continue,
                };
                let arg = test["arg"].as_f64().unwrap();
                benchmark(executions, &runtime_name, name, fun, arg);
            }
            4 => {
                let fun = match test["fun"].as_str() {
                    Some("matrix_multiply") => matrix_multiply,
                    _ => continue,
                };
                let a = serde_json::from_value::<Vec<f64>>(test["arg"]["a"].clone()).unwrap();
                let b = serde_json::from_value::<Vec<f64>>(test["arg"]["b"].clone()).unwrap();
                let width = test["arg"]["width"].as_u64().unwrap() as usize;
                benchmark_4(executions, &runtime_name, name, fun, a, b, width);
            }
            5 => {
                let fun = match test["fun"].as_str() {
                    Some("mandelbrot") => mandelbrot,
                    _ => continue,
                };
                let width = test["arg"]["width"].as_u64().unwrap() as u32;
                let height = test["arg"]["height"].as_u64().unwrap() as u32;
                let max_iter = test["arg"]["max_iter"].as_u64().unwrap() as u32;
                benchmark_5(
                    executions,
                    &runtime_name,
                    name,
                    fun,
                    width,
                    height,
                    max_iter,
                );
            }
            _ => continue,
        }
    }
}
