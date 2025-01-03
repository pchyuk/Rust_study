use std::env; // use를 사용해서 std::env 모듈을 스코프로 가져와서 args 함수를 사용할 수 있게 한다.
// use std::fs; // use를 사용해서 std::fs 모듈을 스코프로 가져와서 read_to_string 함수를 사용할 수 있게 한다.
use std::process; // use를 사용해서 std::process 모듈을 스코프로 가져와서 exit 함수를 사용할 수 있게 한다.
// use std::error::Error; // use를 사용해서 std::error::Error 모듈을 스코프로 가져와서 Error 트레이트를 사용할 수 있게 한다.

use minigrep::Config; // use를 사용해서 minigrep 모듈의 Config 구조체를 스코프로 가져온다.

fn main(){
    let args: Vec<String> = env::args().collect(); // env::args를 호출한 즉시 collect 메서드를 호출해서 결과를 Vec<String> 타입으로 변환한다.
    // // dbg!(args); // args 변수에 할당된 값을 출력한다.

    // let config = Config::new(&args); // parse_config 함수를 호출해서 query와 file_path 변수에 할당한다.
    // // let query = &args[1]; // query 변수에 args의 두 번째 요소를 할당한다.
    // let file_path = &args[2]; // file_path 변수에 args의 세 번째 요소를 할당한다.

    let config = Config::build(&args).unwrap_or_else(|err| { // build 함수를 호출해서 config 변수에 할당한다.
        // println!("Problem parsing arguments: {err}"); // 에러 메시지를 출력한다.
        eprintln!("Problem parsing arguments: {err}"); // 에러 메시지를 출력한다.
        process::exit(1); // 프로그램을 종료한다.
    });

    // println!("Searching for {}", config.query);
    // println!("In file {}", config.file_path);

    // run(config); // run 함수를 호출해서 config 변수를 전달한다.
    if let Err(e) = minigrep::run(config) { // run 함수를 호출해서 config 변수를 전달한다.
        // println!("Application error: {e}"); // 에러 메시지를 출력한다.
        eprintln!("Application error: {e}"); // 에러 메시지를 출력한다.
        process::exit(1); // 프로그램을 종료한다.
    }

    // // fs::read_to_string 함수를 호출해서 file_path에 있는 파일의 내용을 읽어온다.
    // let contents = fs::read_to_string(config.file_path).expect("Should have been able to read the file"); 

    // println!("With text:\n{contents}");

    // let (query, file_path) = parse_config(&args); // parse_config 함수를 호출해서 query와 file_path 변수에 할당한다.
}

// struct Config {
//     query: String,
//     file_path: String,
// }

// fn parse_config(args: &[String]) -> (&str, &str) { // parse_config 함수는 args 변수를 받아서 튜플을 반환한다.
//     let query = &args[1]; // query 변수에 args의 두 번째 요소를 할당한다.
//     let file_path = &args[2]; // file_path 변수에 args의 세 번째 요소를 할당한다.

//     (query, file_path) // query와 file_path 변수를 튜플로 반환한다.
// }

// impl Config {
//     fn build(args: &[String]) -> Result<Config, &'static str> { // build 함수는 args 변수를 받아서 Result<Config, &'static str> 타입으로 반환한다.
//         if args.len() < 3 { // args의 길이가 3보다 작으면 에러 메시지를 출력하고 종료한다.
//             return Err("not enough arguments");
//         }
//         let query = args[1].clone(); // query 변수에 args의 두 번째 요소를 할당한다.
//         let file_path = args[2].clone(); // file_path 변수에 args의 세 번째 요소를 할당한다.

//         Ok(Config { query, file_path }) // Config 구조체를 생성해서 Ok로 감싸서 반환한다.
//     }
// }

// fn run(config: Config) -> Result<(), Box<dyn Error>> { // run 함수는 Config 구조체를 받아서 Result<(), Box<dyn Error>> 타입으로 반환한다.
//     let contents = fs::read_to_string(config.file_path)?; // fs::read_to_string 함수를 호출해서 file_path에 있는 파일의 내용을 읽어온다.
    
//     println!("With text:\n{contents}");

//     Ok(()) // Ok로 감싸서 반환한다.
// }