#!/bin/bash
cd elp
cargo build --release
sudo cp -R target/release/elp /usr/bin/elp
sudo chmod u+x /usr/bin/elp
cd ..
rm -rf elp