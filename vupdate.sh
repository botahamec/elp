git clone https://github.com/Botahamec/elp.git
cd elp
cargo build --release -v
sudo cp -R target/release /usr/bin/elp
sudo chmod u+x /usr/bin/elp
cd ..
rm -rf elp