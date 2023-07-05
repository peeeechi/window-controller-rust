#!/bin/bash -eu

cargo fix --allow-dirty \
  && cargo fmt \
  && cargo clean \
  && cargo build --target x86_64-pc-windows-gnu \
  && cargo doc \

