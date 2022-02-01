use rand::Rng;
use std::collections::HashMap;
use std::time::{Duration, Instant};

fn vector_benchmark(vector: Vec<usize>, index: usize) -> Duration {
    let initial_time = Instant::now();
    for item in vector {
        if item == index {
            let _result = item;
        }
    }

    return initial_time.elapsed();
}

fn hashmap_benchmark(hashmap: HashMap<usize, usize>, index: &usize) -> Duration {
    let initial_time = Instant::now();
    let _result = hashmap.get(index).unwrap();

    return initial_time.elapsed();
}

fn benchmark(length: usize, index: usize) -> (Duration, Duration) {
    let vector: Vec<usize> = (0..length).collect();
    let hashmap: HashMap<usize, usize> = (0..length).map(|i| (i, i)).collect::<HashMap<_, _>>();

    return (
        vector_benchmark(vector, index),
        hashmap_benchmark(hashmap, &index),
    );
}

fn main() {
    let mut rng = rand::thread_rng();

    let lengths: Vec<usize> = vec![10, 100, 1_000, 10_000, 100_000, 1_000_000, 10_000_000];
    let samples: usize = 100;

    for length in lengths {
        let mut vector_benchmarks: Vec<Duration> = Vec::new();
        let mut hashmap_benchmarks: Vec<Duration> = Vec::new();
        for _ in 0..samples {
            let index = rng.gen_range(0..length);
            let (vector_time, hashmap_time) = benchmark(length, index);

            vector_benchmarks.push(vector_time);
            hashmap_benchmarks.push(hashmap_time);
        }

        // let sumation: Duration = vector_benchmarks.iter().sum();
        let mut vector_average_time: Duration = vector_benchmarks.iter().sum();
        let mut hashmap_average_time: Duration = hashmap_benchmarks.iter().sum();
        vector_average_time /= samples.try_into().unwrap();
        hashmap_average_time /= samples.try_into().unwrap();

        println!("Array/HashMap Length: {length}, vector average time: {vector_average_time:?}, hashmap average time: {hashmap_average_time:?}");
    }
}
