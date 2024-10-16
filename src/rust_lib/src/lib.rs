use anyhow::Result;
use ffi::XY;

#[cxx::bridge]
mod ffi {

    struct XY {
        pub x: f64,
        pub y: f64,
    }

    extern "Rust" {
        type MessageContainer;

        fn rust_cxx_square(i: i32) -> i32;

        fn rust_cxx_build_message_container(message: &str) -> Box<MessageContainer>;

        fn rust_cxx_print_message(container: &Box<MessageContainer>);

        fn rust_cxx_rotate(point: XY, radians: f64) -> XY;

        fn rust_cxx_wow(message: &str) -> Result<String>;

        fn rust_cxx_http_get(url: &str, body: &str) -> Result<String>;
    }
}

pub struct MessageContainer {
    pub message: String,
}

pub fn rust_cxx_square(i: i32) -> i32 {
    i * i
}

pub fn rust_cxx_build_message_container(message: &str) -> Box<MessageContainer> {
    Box::new(MessageContainer {
        message: message.into(),
    })
}

pub fn rust_cxx_print_message(container: &Box<MessageContainer>) {
    println!("Message: {}", container.message);
}

pub fn rust_cxx_rotate(point: XY, radians: f64) -> XY {
    XY {
        x: (point.x * f64::cos(radians) - point.y * f64::sin(radians)),
        y: (point.x * f64::sin(radians) + point.y * f64::cos(radians)),
    }
}

pub fn rust_cxx_wow(message: &str) -> Result<String> {
    Ok(format!("{}. wow.", message))
}

pub fn rust_cxx_http_get(url: &str, body: &str) -> Result<String> {
    let client = reqwest::blocking::Client::new();
    let mut res = client.get(url).body(body.to_string().clone()).send()?;

    eprintln!("Response: {:?} {}", res.version(), res.status());
    eprintln!("Headers: {:#?}\n", res.headers());

    let mut buf = Vec::new();
    res.copy_to(&mut buf)?;

    Ok(std::str::from_utf8(buf.as_slice()).unwrap().to_string())
}
