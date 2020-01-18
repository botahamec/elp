#!/bin/bash
cd elp
cargo build --release -vv
sudo cp target/release/elp /usr/bin/elp
sudo chmod u+x /usr/bin/elp
cd ..
rm -rf elp