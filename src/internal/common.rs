use std::{
    fmt::Display,
    io::{stdout, Write},
};

pub fn error_and_exit(code: i32, msg: impl Display) -> ! {
    let _ = stdout().write_all(msg.to_string().as_bytes());
    std::process::exit(code);
}
