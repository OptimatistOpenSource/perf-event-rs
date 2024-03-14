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

use std::alloc::{alloc, Layout};
use std::ptr;

pub trait WrapBox<T> {
    #[inline]
    fn wrap_box(self) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(self)
    }
}

impl<T> WrapBox<T> for T {}

pub trait BoxSliceExt {
    unsafe fn uninit(len: usize) -> Self;
}

impl<T> BoxSliceExt for Box<[T]> {
    unsafe fn uninit(len: usize) -> Self {
        let layout = Layout::array::<u8>(len).unwrap();
        let ptr = unsafe { alloc(layout) };
        let slice = ptr::slice_from_raw_parts(ptr, len);
        unsafe { Self::from_raw(std::mem::transmute(slice)) }
    }
}
