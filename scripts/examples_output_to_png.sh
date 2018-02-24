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

cargo run --example mutable_struct | ./aha/aha > examples/mutable_struct.html
./wkhtmltox/bin/wkhtmltoimage --user-style-sheet scripts/style.css --zoom 1.3 examples/mutable_struct.html assets/mutable_struct.png
rm examples/mutable_struct.html

cargo run --example multiple_inputs | ./aha/aha > examples/multiple_inputs.html
./wkhtmltox/bin/wkhtmltoimage --user-style-sheet scripts/style.css --zoom 1.3 examples/multiple_inputs.html assets/multiple_inputs.png
rm examples/multiple_inputs.html

cargo run --example multiple_outputs | ./aha/aha > examples/multiple_outputs.html
./wkhtmltox/bin/wkhtmltoimage --user-style-sheet scripts/style.css --zoom 1.3 examples/multiple_outputs.html assets/multiple_outputs.png
rm examples/multiple_outputs.html

