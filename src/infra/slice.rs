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

pub trait SliceExt<T> {
    unsafe fn follow_mem_ptr(&self) -> *const T;
}

impl<T> SliceExt<T> for &[T] {
    #[inline]
    unsafe fn follow_mem_ptr(&self) -> *const T {
        self.as_ptr().add(self.len())
    }
}

#[cfg(test)]
mod tests {
    use crate::infra::SliceExt;

    #[test]
    fn test_follow_mem_ptr_1() {
        let buf = [0, 1, 2, 3_u8];
        let slice = &buf[0..buf.len() - 1];
        let ptr = unsafe { slice.follow_mem_ptr() };
        assert_eq!(ptr, &buf[buf.len() - 1] as *const u8);
    }

    #[test]
    fn test_follow_mem_ptr_2() {
        let buf = [0, 1, 2, 3_u8];
        let slice = &buf[0..0];
        let ptr = unsafe { slice.follow_mem_ptr() };
        assert_eq!(ptr, &buf[0] as *const u8);
    }
}
