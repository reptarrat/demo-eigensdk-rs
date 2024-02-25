#!/bin/bash

# Ensure all submodules are present
git submodule update --init --recursive

# hack to never cache. TODO: look into --no-cache flag
rm -rf /tmp/forge-cache

# Use forge to generate bindings
# See installation instructions here https://github.com/foundry-rs/foundry
# Using a version of foundry based on commit 05158b38792455c9365af188b9a525a0e93eb9d1
forge bind \
  --root crates/contracts/lib/eigenlayer-middleware \
  --bindings-path crates/contracts/eigenlayer-middleware-bindings \
  --crate-name eigenlayer-middleware-bindings \
  --skip-cargo-toml \
  --overwrite \
  --skip-extra-derives \
  --cache-path /tmp/forge-cache