#!/bin/bash

rm -rf build.rs device.x lib.rs generic.rs features.toml src/

svd2rust -i LPC176x5x.svd \
  -s \
  --const_generic \
  --atomics --atomics_feature atomics \
  --impl_debug --impl_debug_feature debug \
  --feature_peripheral

echo "#![allow(unknown_lints)]" | cat - lib.rs > temp
mv temp lib.rs

form -i lib.rs -o src/
rm lib.rs

rm features.toml

cargo fmt
cargo check
