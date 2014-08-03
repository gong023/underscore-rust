#!/bin/bash
mkdir -p compiled/lib
rustc --crate-type=lib src/lib/lib.rs --out-dir compiled/lib
rustc --test src/test/test.rs --out-dir compiled -L compiled/lib
./compiled/test
