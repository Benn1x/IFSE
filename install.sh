#!/bin/sh
# this is teh install script
# execute command: cargo build --release
echo "Start Compiling ifs..."
cargo build --release
echo "Compiling Done!"
# copy the binary to /usr/local/bin
echo "Start Copying ifs to /usr/local/bin"
mv target/release/ifs /usr/local/bin
echo "Copying Done!"
echo "ifs is installed successfully!"
# q: write a commit message that discribes the changes
