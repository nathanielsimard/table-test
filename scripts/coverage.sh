#!/bin/bash

current_path=$(pwd)
docker build -t table-test .
docker run -v $current_path:/opt/table-test/target table-test:latest