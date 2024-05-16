#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_cxx_square(i: i32) -> i32;
    }
}

pub fn rust_cxx_square(i: i32) -> i32 {
    i*i
}