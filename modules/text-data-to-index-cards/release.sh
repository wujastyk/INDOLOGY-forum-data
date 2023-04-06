executable_name=${PWD##*/}
executable_name=${executable_name:-/}

cargo build --bin ${executable_name} --target-dir ./target --release --target x86_64-unknown-linux-musl
cp ./target/x86_64-unknown-linux-musl/release/${executable_name} .
