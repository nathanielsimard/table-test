#!/bin/bash

if [ ! -d aha ]; then
	git clone https://github.com/theZiz/aha
	cd aha
	make
	cd ..
fi

if [ ! -d wkhtmltox ]; then
	wget https://github.com/wkhtmltopdf/wkhtmltopdf/releases/download/0.12.4/wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
	tar xvf wkhtmltox-0.12.4_linux-generic-amd64.tar.xz
	rm -rf wkhtmltox-0.12.4_linux-generic-amd64.tar.xz 
fi

cargo run --example change_name | ./aha/aha --black > examples/change_name.html
./wkhtmltox/bin/wkhtmltoimage examples/change_name.html examples/change_name.png

rm examples/change_name.html
