use crate::user::User; // User 구조체 추가
use crate::utils::{read_player_names, write_user_statistics}; // read_player_names 함수와 write_user_statistics 함수 추가
use std::collections::HashMap; // HashMap 추가
use std::fs::{self, OpenOptions}; // OpenOptions 추가
use std::io::Write; // Write 트레이트 추가

pub struct Game { // Game 구조체 추가
    players: HashMap<String, User>, // players 필드 추가
}

impl Game { // Game 구조체에 메서드 추가
    pub fn new() -> Self { // new 메서드 추가
        Game { // Game 구조체의 인스턴스를 생성하고 반환
            players: HashMap::new(), // players 필드를 빈 HashMap으로 초기화
        }
    }

    pub fn register(&mut self, username: &str, password: &str) -> bool { // register 메서드 추가
        // user_info.txt 파일이 없으면 생성
        let userinfo_path = "src/user_info.txt"; // userinfo_path 변수를 선언하고 초기화
        let mut userinfo_file = OpenOptions::new() // userinfo_file 변수를 선언하고 초기화
            .write(true) // 쓰기 권한 추가
            .create(true) // 파일이 없으면 생성
            .append(true) // 파일 끝에 내용 추가
            .open(userinfo_path) // 파일 열기
            .expect("user_info.txt 파일을 열 수 없습니다."); // 파일 열기 실패 시 에러 메시지 출력

        // user_info.txt 파일에서 사용자 이름이 있는지 확인
        let userinfo_contents = fs::read_to_string(userinfo_path).expect("user_info.txt 파일을 읽을 수 없습니다."); // 파일의 내용을 문자열로 읽어옴
        if userinfo_contents.lines().any(|line| line.split_whitespace().next() == Some(username)) { // 사용자 이름이 이미 존재하는지 확인
            return false; // 이미 존재하는 사용자
        }

        // 비밀번호 해싱
        let password_hash = User::hash_password(password); // User 구조체의 hash_password 메서드를 호출하여 비밀번호 해싱

        // user_info.txt 파일에 사용자 이름과 해싱된 비밀번호 저장
        writeln!(userinfo_file, "{} {}", username, password_hash).expect("user_info.txt 파일에 기록할 수 없습니다."); // 파일에 사용자 이름과 해싱된 비밀번호 기록

        // player_list.txt 파일이 없으면 생성
        let player_list_path = "src/player_list.txt"; // player_list_path 변수를 선언하고 초기화
        let player_names = fs::read_to_string(player_list_path).unwrap_or_else(|_| { // 파일의 내용을 문자열로 읽어옴
            // 파일이 없으면 빈 파일 생성
            OpenOptions::new() // 파일 옵션 생성
                .write(true) // 쓰기 권한 추가
                .create(true) // 파일이 없으면 생성
                .open(player_list_path) // 파일 열기
                .expect("플레이어 목록 파일을 생성할 수 없습니다."); // 파일 열기 실패 시 에러 메시지 출력
            String::new() // 빈 문자열 반환
        });

        // player_list.txt 파일에 사용자 이름이 없으면 추가
        if !player_names.lines().any(|name| name == username) { // 사용자 이름이 이미 존재하는지 확인
            let mut player_list_file = OpenOptions::new() // player_list_file 변수를 선언하고 초기화
                .write(true) // 쓰기 권한 추가
                .append(true) // 파일 끝에 내용 추가
                .open(player_list_path) // 파일 열기
                .expect("player_list.txt 파일을 열 수 없습니다."); // 파일 열기 실패 시 에러 메시지 출력
            writeln!(player_list_file, "{}", username).expect("player_list.txt 파일에 기록할 수 없습니다."); // 파일에 사용자 이름 기록
        }

        true // 회원가입 성공
    }

    pub fn login(&mut self, username: &str, password: &str) -> Option<User> { // login 메서드 추가
        // user_info.txt 파일에서 사용자 이름과 해싱된 비밀번호 확인
        let userinfo_path = "src/user_info.txt"; // userinfo_path 변수를 선언하고 초기화
        let userinfo_contents = fs::read_to_string(userinfo_path).expect("user_info.txt 파일을 읽을 수 없습니다."); // 파일의 내용을 문자열로 읽어옴
        for line in userinfo_contents.lines() { // 파일의 각 줄에 대해 반복
            let mut parts = line.split_whitespace(); // 공백으로 분리
            if let (Some(stored_username), Some(stored_password_hash)) = (parts.next(), parts.next()) { // 사용자 이름과 해싱된 비밀번호 추출
                if stored_username == username && stored_password_hash == User::hash_password(password) { // 사용자 이름과 비밀번호 일치 확인
                    // 로그인 성공
                    let user = User::new(username.to_string(), password.to_string()); // User 구조체의 인스턴스 생성
                    self.players.insert(username.to_string(), user.clone()); // players 필드에 사용자 추가
                    return Some(user); // User 구조체 반환
                }
            }
        }
        None // 로그인 실패
    }

    pub fn display_statistics(&self, user: &User) {
        // 유저의 전적 파일을 읽어와서 간략한 전적을 출력
        let stats = user.get_statistics(); // User 구조체의 get_statistics 메서드를 호출하여 전적을 가져옴
        println!("{}", stats); // 전적 출력
    }

    pub fn display_opponent_statistics(&self, user: &User, opponent: &str) {
        // 유저의 전적 파일을 읽어와서 상대 플레이어와의 전적을 출력
        let stats = user.get_opponent_statistics(opponent); // User 구조체의 get_opponent_statistics 메서드를 호출하여 상대 플레이어와의 전적을 가져옴
        println!("{}", stats); // 전적 출력
    }

    pub fn display_detailed_statistics(&self, user: &User) {
        // 유저의 전적 파일을 읽어와서 세부 전적을 출력
        let stats = user.get_detailed_statistics(); // User 구조체의 get_detailed_statistics 메서드를 호출하여 세부 전적을 가져옴
        println!("{}", stats); // 전적 출력
    }

    pub fn record_result(&self, user: &User, record: &str) {
        // 전적을 기록
        let parts: Vec<&str> = record.split_whitespace().collect(); // 공백으로 분리
        if parts.len() == 2 { // 길이가 2인 경우
            let opponent = parts[0]; // 상대 플레이어 이름
            let result = parts[1]; // 결과
            user.record_result(opponent, result); // User 구조체의 record_result 메서드를 호출하여 전적 기록
        } else { // 그 외의 경우
            println!("잘못된 입력 형식입니다.\n");
        }
    }
}