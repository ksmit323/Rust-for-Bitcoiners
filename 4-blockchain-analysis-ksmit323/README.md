# Assignment 4

Implement Task 1 and 2 as described in the main.rs file.

I have shown how to create a custom `bitcoincore_rpc::Client` from `jsonrpc::client::Client`.
Have used `lazy_static` crate to create a global shared variable in rust.
Yes it's extremely hard to convince the compiler that you are using a global variable correctly,
so a crate has to be created to make your life easier.

## Adventure

Implement your own functions, take inspiration from `blockchain_analysis` directory.
Write as much functions you can, get comfortable with Rust syntax and reading the source code
of `bitcoin` and `bitcoincore_rpc` crates.
