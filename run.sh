#!/bin/bash
export init_dir=$(pwd)
mkdir target
mkdir target/debug
ln -s "${init_dir}/assets/" "${init_dir}/target/debug/resources"
cargo run