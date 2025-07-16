//编译到linux
cross build --release --target x86_64-unknown-linux-musl
cargo run --target x86_64-pc-windows-msvc