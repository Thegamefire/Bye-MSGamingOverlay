use sysuri::{UriScheme, register, parse_args, is_registered};
use std::env;

fn main() {    if let Some(uri) = parse_args() {
    println!("[Bye MSG] Opened with URI: {}", uri);
} else {
    if !is_registered("ms-gamingoverlay").is_ok_and(|v| v) {
        println!("[Bye MSG] Registering ms-gamingoverlay ...");
        register_uri();
        println!("[Bye MSG] Registered ms-gamingoverlay ...");
    } else {
        println!("[Bye MSG] ms-gamingoverlay URI already registered!");
    }
}
}


fn register_uri() {
    let exe = env::current_exe().unwrap();

    let scheme = UriScheme::new(
        "ms-gamingoverlay",
        "Microsoft Gaming Overlay",
        exe
    );

    register(&scheme).unwrap();
}
