// Copyright (c) 2023-2024 Optimatist Technology Co., Ltd. All rights reserved.
// DO NOT ALTER OR REMOVE COPYRIGHT NOTICES OR THIS FILE HEADER.
//
// This file is part of perf-event-rs.
//
// Perf-event-rs is free software: you can redistribute it and/or modify it under the terms of the GNU Lesser General Public License
// as published by the Free Software Foundation, either version 3 of the License, or (at your option) any later version.
//
// Perf-event-rs is distributed in the hope that it will be useful, but WITHOUT ANY WARRANTY; without even
// the implied warranty of MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE. See the GNU Lesser General Public License for more details.
//
// You should have received a copy of the GNU Lesser General Public License along with Perf-event-rs. If not,
// see <https://www.gnu.org/licenses/>.

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
