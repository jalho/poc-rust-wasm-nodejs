```
$ wasm-pack --version
wasm-pack 0.13.1

$ rustc --version
rustc 1.88.0 (6b00bc388 2025-06-23)

$ node --version
v22.15.0

$ musl-gcc --version
x86_64-linux-gnu-gcc (Debian 12.2.0-14+deb12u1) 12.2.0

$ bash run.sh
+ cd native-app
+ cargo build --release --target x86_64-unknown-linux-musl
    Finished `release` profile [optimized] target(s) in 0.01s
+ cd -
/poc-rust-wasm-nodejs
+ cd ffi-wasm
+ cargo build --release --target wasm32-unknown-unknown
    Finished `release` profile [optimized] target(s) in 0.01s
+ cd -
/poc-rust-wasm-nodejs
+ test -d ffi-wasm/pkg
+ rm -r ffi-wasm/pkg
+ wasm-pack build ffi-wasm --target nodejs
[INFO]: 🎯  Checking for the Wasm target...
[INFO]: 🌀  Compiling to Wasm...
    Finished `release` profile [optimized] target(s) in 0.01s
[INFO]: ⬇️  Installing wasm-bindgen...
[INFO]: Optimizing wasm binaries with `wasm-opt`...
[INFO]: Optional fields missing from Cargo.toml: 'description', 'repository', and 'license'. These are not necessary, but recommended
[INFO]: ✨   Done in 0.29s
[INFO]: 📦   Your wasm pkg is ready to publish at ffi-wasm/pkg.
++ realpath ./main.mts
+ ./target/x86_64-unknown-linux-musl/release/native-app /poc-rust-wasm-nodejs/main.mts
(node:63653) ExperimentalWarning: Type Stripping is an experimental feature and might change at any time
(Use `node --trace-warnings ...` to show where the warning was created)

DEBUG num_1 1
DEBUG log_entry LogEntry { __wbg_ptr: 1114128 }
DEBUG log_entry.nanos 1000000000n
DEBUG log_entry instanceof lib.LogEntry true
DEBUG timestamp_millis 1000n
DEBUG timestamp_micros 1000000n
DEBUG timestamp_rfc3339 1970-01-01T00:00:01+00:00
```
