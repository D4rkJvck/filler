#!/bin/bash

cd solution || exit 1
cargo build
cd ..

./linux_game_engine -f maps/map00 -p1 linux_robots/terminator -p2 solution/target/debug/filler
