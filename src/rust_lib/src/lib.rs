use anyhow::Result;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_cxx_square(i: i32) -> i32;

        fn rust_cxx_wow(lol: &str) -> Result<String>;
    }
}

pub fn rust_cxx_square(i: i32) -> i32 {
    i*i
}

pub fn rust_cxx_wow(lol: &str) -> Result<String> {
    Ok(format!("{}. wow.", lol))
}