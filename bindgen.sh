#!/usr/bin/env bash

set -e

bindgen --with-derive-default wrapper.h > src/syscall/bindings.rs
