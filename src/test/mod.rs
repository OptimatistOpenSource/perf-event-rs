use std::time::Duration;
use std::{fs, thread};

pub fn cpu_workload() {
    for _ in 0..1000000 {
        std::hint::black_box(rand::random::<usize>());
        std::hint::black_box(rand::random::<usize>());
    }
}

pub fn mem_workload() {
    let len = 999999999;
    let mut vec = vec![0; len];
    for _ in 0..9999 {
        let rand = rand::random::<usize>() % len;
        vec[rand] = rand;
        std::hint::black_box(&vec);
        thread::sleep(Duration::from_millis(1));
        std::hint::black_box(vec[rand]);
    }
}

pub fn read_file(path: &str) -> String {
    fs::read_to_string(path).unwrap()
}
