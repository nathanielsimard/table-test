#!/bin/bash

mkdir -p target/cov
current_path=$(pwd)/target/cov
docker build -t table-test .
docker run -v $current_path:/opt/table-test/target/cov table-test:latest