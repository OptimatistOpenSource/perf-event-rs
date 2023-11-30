pub fn cpu_workload() {
    for _ in 0..1000000 {
        std::hint::black_box(0);
    }
}

pub fn mem_workload() {
    std::hint::black_box(vec![0_u8; 10000000]);
}
