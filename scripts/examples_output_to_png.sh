#!/bin/bash

OUT_DIR='assets'
declare -a examples=("mutable_struct" "multiple_inputs")

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

for example in "${examples[@]}"
do
	cargo run --example $example | ./aha/aha > tmp.html
	./wkhtmltox/bin/wkhtmltoimage --user-style-sheet scripts/style.css --zoom 1.3 --quality 85 tmp.html "assets/"$example".png"
	rm tmp.html
done