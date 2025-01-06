mod game; // game 모듈 추가
mod user; // user 모듈 추가
mod utils; // utils 모듈 추가

use std::io::{self}; // Write 제거

fn main() {
    // 게임 초기화
    let mut game = game::Game::new();

    println!("\n*** [게임 전적 검색기] ***");

    loop {
        // 회원가입 또는 로그인 선택
        println!("\n1. 회원가입 / 2. 로그인 / 3. 종료");
        let mut choice = String::new(); // choice 변수를 선언하고 초기화
        io::stdin().read_line(&mut choice).expect("입력 오류"); // choice 변수에 사용자 입력을 받음
        let choice = choice.trim(); // choice 변수의 앞뒤 공백을 제거

        match choice { // choice 변수의 값에 따라 분기
            "1" => {
                // 회원가입
                println!("\n회원가입하려는 플레이어의 이름을 입력하세요:");
                let mut username = String::new(); // username 변수를 선언하고 초기화
                io::stdin().read_line(&mut username).expect("입력 오류"); // username 변수에 사용자 입력을 받음
                let username = username.trim(); // username 변수의 앞뒤 공백을 제거

                println!("\n비밀번호를 입력하세요:");
                let mut password = String::new(); // password 변수를 선언하고 초기화
                io::stdin().read_line(&mut password).expect("입력 오류"); // password 변수에 사용자 입력을 받음
                let password = password.trim(); // password 변수의 앞뒤 공백을 제거

                if game.register(username, password) { // game 객체의 register 메서드를 호출하고 반환값에 따라 분기
                    println!("\n회원가입 성공! 이제 로그인하세요.\n");
                } else {
                    println!("\n회원가입 실패. 이미 존재하는 사용자입니다.\n");
                }
            }
            "2" => {
                // 로그인
                println!("\n로그인하려는 플레이어의 이름을 입력하세요:");
                let mut username = String::new(); // username 변수를 선언하고 초기화
                io::stdin().read_line(&mut username).expect("입력 오류"); // username 변수에 사용자 입력을 받음
                let username = username.trim(); // username 변수의 앞뒤 공백을 제거

                println!("\n비밀번호를 입력하세요:");
                let mut password = String::new(); // password 변수를 선언하고 초기화
                io::stdin().read_line(&mut password).expect("입력 오류"); // password 변수에 사용자 입력을 받음
                let password = password.trim(); // password 변수의 앞뒤 공백을 제거

                // 로그인 처리
                if let Some(user) = game.login(username, password) { // game 객체의 login 메서드를 호출하고 반환값에 따라 분기
                    println!("\n환영합니다, {}!\n", user.username); // user 객체의 username 필드 출력

                    // 간략한 전적 출력
                    game.display_statistics(&user); // game 객체의 display_statistics 메서드를 호출

                    // 메인 게임 루프
                    loop {
                        println!("\n메뉴를 선택하세요:");
                        println!("1. 상대 플레이어별 vs 전적 보기  / 2. 나의 세부 전적 보기  / 3. 내 전적 추가하기  / 4. 로그아웃");
                        let mut choice = String::new(); // choice 변수를 선언하고 초기화
                        io::stdin().read_line(&mut choice).expect("입력 오류"); // choice 변수에 사용자 입력을 받음
                        let choice = choice.trim(); // choice 변수의 앞뒤 공백을 제거

                        match choice { // choice 변수의 값에 따라 분기
                            "1" => {
                                println!("\n전적을 확인할 플레이어 이름을 입력하세요:");
                                let mut opponent = String::new(); // opponent 변수를 선언하고 초기화
                                io::stdin().read_line(&mut opponent).expect("입력 오류"); // opponent 변수에 사용자 입력을 받음
                                let opponent = opponent.trim(); // opponent 변수의 앞뒤 공백을 제거
                                game.display_opponent_statistics(&user, opponent); // game 객체의 display_opponent_statistics 메서드를 호출
                            }
                            "2" => {
                                game.display_detailed_statistics(&user); // game 객체의 display_detailed_statistics 메서드를 호출
                            }
                            "3" => {
                                println!("\n전적을 기록할 상대 플레이어 이름과 결과를 입력하세요 (예: player2 W):");
                                let mut record = String::new(); // record 변수를 선언하고 초기화
                                io::stdin().read_line(&mut record).expect("입력 오류"); // record 변수에 사용자 입력을 받음
                                let record = record.trim(); // record 변수의 앞뒤 공백을 제거
                                game.record_result(&user, record); // game 객체의 record_result 메서드를 호출
                            }
                            "4" => break, // 반복문 종료
                            _ => println!("\n잘못된 선택입니다. 다시 시도하세요."), // 그 외의 경우
                        }
                    }
                } else {
                    println!("\n로그인 실패.\n");
                }
            }
            "3" => {
                println!("\n프로그램을 종료합니다.\n");
                break; // 반복문 종료
            }
            _ => println!("\n잘못된 선택입니다. 다시 시도하세요."), // 그 외의 경우
        }
    }
}