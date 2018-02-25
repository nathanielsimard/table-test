FROM akubera/rust-kcov:nightly

WORKDIR /opt/table-test
ADD ./ /opt/table-test/

RUN cargo install cargo-kcov

CMD "cargo" "kcov"
