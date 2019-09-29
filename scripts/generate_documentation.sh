#!/bin/bash

# this script meant to be run from the root of arrayfire-rust

cargo rustdoc -- --html-in-header ./scripts/mathjax.script

mdbook build tutorials-book && cp -r tutorials-book/book ./target/doc/arrayfire/
