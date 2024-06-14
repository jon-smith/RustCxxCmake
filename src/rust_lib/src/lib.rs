use anyhow::Result;

#[cxx::bridge]
mod ffi {
    extern "Rust" {
        fn rust_cxx_square(i: i32) -> i32;

        fn rust_cxx_wow(lol: &str) -> Result<String>;

        fn rust_cxx_http_get(url: &str, body: &str) -> Result<String>;
    }
}

pub fn rust_cxx_square(i: i32) -> i32 {
    i*i
}

pub fn rust_cxx_wow(lol: &str) -> Result<String> {
    Ok(format!("{}. wow.", lol))
}

pub fn rust_cxx_http_get(url: &str, body: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let mut res = client.get(url)
    .body(body.to_string().clone())
    .send()?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let mut buf = Vec::new();
    res.copy_to(&mut buf)?;

    Ok(std::str::from_utf8(buf.as_slice()).unwrap().to_string())
}