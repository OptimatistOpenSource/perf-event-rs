use std::thread;
use std::time::Duration;

pub fn cpu_workload() {
    for _ in 0..1000000 {
        std::hint::black_box(0);
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
