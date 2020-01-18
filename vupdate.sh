#!/bin/bash
cd elp
cargo build --release -v
sudo cp -R target/elp /usr/bin/elp
sudo chmod u+x /usr/bin/elp
cd ..
rm -rf elp