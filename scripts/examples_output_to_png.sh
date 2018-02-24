#!/bin/bash

git clone https://github.com/theZiz/aha
wget https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/0.12.4/wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
tar xvf wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
cd aha
make
cd ..
cargo run --example change_name | ./aha/aha --black > examples/change_name.html
./wkhtmltox/bin/wkhtmltoimage --width 500 examples/change_name.html examples/change_name.png

rm -rf aha
rm -rf wkhtmltox-0.12.4_linux-generic-amd64.tar.xz 
rm -rf wkhtmltox/
rm examples/change_name.html
