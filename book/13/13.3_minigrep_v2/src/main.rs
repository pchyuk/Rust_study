use std::env;
use std::process;

use minigrep::Config;

fn main() {
// 예제 13-18: env::args의 반환 값을 Config::build로 넘기기
    let config = Config::build(env::args()).unwrap_or_else(|err| {
        eprintln!("Problem parsing arguments: {err}");
        process::exit(1);
    });

    // --생략--

    if let Err(e) = minigrep::run(config) {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}