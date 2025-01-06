use std::fs::{self, OpenOptions}; // OpenOptions 추가
use std::io::{self, Write}; // Write 트레이트 추가

#[derive(Clone)] // Clone 트레이트 추가
pub struct User { // User 구조체 추가
    pub username: String, // username 필드 추가
    password_hash: String, // password_hash 필드 추가
}

impl User { // User 구조체에 메서드 추가
    pub fn new(username: String, password: String) -> Self {
        let password_hash = Self::hash_password(&password); // User 구조체의 hash_password 메서드를 호출하여 비밀번호 해싱
        User { username, password_hash } // User 구조체의 인스턴스를 생성하고 반환
    }

    pub fn login(&self, password: &str) -> bool {
        self.password_hash == Self::hash_password(password) // 비밀번호 해싱 결과를 비교하여 로그인 성공 여부 반환
    }

    pub fn hash_password(password: &str) -> String {
        // 비밀번호 해싱 로직 구현
        format!("{:x}", md5::compute(password)) // md5 해시 함수를 사용하여 비밀번호 해싱
    }

    pub fn get_statistics(&self) -> String {
        // 유저의 전적 파일을 읽어와서 간략한 전적을 반환
        let file_path = format!("src/{}.txt", self.username); // 파일 경로 생성
        let contents = fs::read_to_string(&file_path).unwrap_or_else(|_| { // 파일의 내용을 문자열로 읽어옴
            // 파일이 없으면 빈 파일 생성
            OpenOptions::new() // 파일 옵션 생성
                .write(true) // 쓰기 권한 추가
                .create(true) // 파일이 없으면 생성
                .open(&file_path) // 파일 열기
                .expect("전적 파일을 생성할 수 없습니다."); // 파일 열기 실패 시 에러 메시지 출력
            String::new() // 빈 문자열 반환
        });
        let mut wins = 0;
        let mut losses = 0;
        let mut draws = 0;

        for line in contents.lines() { // 각 줄에 대해 반복
            let result = line.split_whitespace().nth(1).unwrap_or(""); // 공백으로 분리하여 결과 추출
            match result { // 결과에 따라 분기
                "W" => wins += 1, // 승리인 경우
                "L" => losses += 1, // 패배인 경우
                "D" => draws += 1, // 무승부인 경우
                _ => (), // 그 외의 경우
            }
        }

        format!("{}: {}W {}L {}D", self.username, wins, losses, draws) // 전적 문자열 반환
    }

    pub fn get_opponent_statistics(&self, opponent: &str) -> String {
        // 유저의 전적 파일을 읽어와서 상대 플레이어와의 전적을 반환
        let file_path = format!("src/{}.txt", self.username); // 파일 경로 생성
        let contents = fs::read_to_string(&file_path).unwrap_or_else(|_| { // 파일의 내용을 문자열로 읽어옴
            // 파일이 없으면 빈 파일 생성
            OpenOptions::new() // 파일 옵션 생성
                .write(true) // 쓰기 권한 추가
                .create(true) // 파일이 없으면 생성
                .open(&file_path) // 파일 열기
                .expect("전적 파일을 생성할 수 없습니다."); // 파일 열기 실패 시 에러 메시지 출력
            String::new() // 빈 문자열 반환
        });
        let mut wins = 0;
        let mut losses = 0;
        let mut draws = 0;

        for line in contents.lines() { // 각 줄에 대해 반복
            let parts: Vec<&str> = line.split_whitespace().collect(); // 공백으로 분리
            if parts.len() == 2 && parts[0] == opponent { // 상대 플레이어 이름이 일치하는 경우
                match parts[1] { // 결과에 따라 분기
                    "W" => wins += 1, // 승리인 경우
                    "L" => losses += 1, // 패배인 경우
                    "D" => draws += 1, // 무승부인 경우
                    _ => (), // 그 외의 경우
                }
            }
        }

        format!("vs {}: {}W {}L {}D", opponent, wins, losses, draws) // 전적 문자열 반환
    }

    pub fn get_detailed_statistics(&self) -> String {
        // 유저의 전적 파일을 읽어와서 세부 전적을 반환
        let file_path = format!("src/{}.txt", self.username); // 파일 경로 생성
        fs::read_to_string(&file_path).unwrap_or_else(|_| { // 파일의 내용을 문자열로 읽어옴
            // 파일이 없으면 빈 파일 생성
            OpenOptions::new() // 파일 옵션 생성
                .write(true) // 쓰기 권한 추가
                .create(true) // 파일이 없으면 생성
                .open(&file_path) // 파일 열기
                .expect("전적 파일을 생성할 수 없습니다."); // 파일 열기 실패 시 에러 메시지 출력
            String::new() // 빈 문자열 반환
        })
    }

    pub fn record_result(&self, opponent: &str, result: &str) {
        // 유저의 전적 파일에 결과를 기록
        let file_path = format!("src/{}.txt", self.username); // 파일 경로 생성
        let mut file = OpenOptions::new() // 파일 옵션 생성
            .write(true) // 쓰기 권한 추가
            .create(true) // 파일이 없으면 생성
            .append(true) // 파일 끝에 내용 추가
            .open(&file_path) // 파일 열기
            .expect("전적 파일을 열 수 없습니다."); // 파일 열기 실패 시 에러 메시지 출력

        writeln!(file, "{} {}", opponent, result).expect("전적 파일에 기록할 수 없습니다."); // 파일에 상대 플레이어 이름과 결과 기록

        // 상대 플레이어의 전적 파일에도 결과를 기록
        let opponent_file_path = format!("src/{}.txt", opponent); // 상대 플레이어의 파일 경로 생성
        let opponent_result = match result { // 결과에 따라 상대 플레이어의 결과 결정
            "W" => "L",
            "L" => "W",
            "D" => "D",
            _ => "",
        };

        let mut opponent_file = OpenOptions::new() // 파일 옵션 생성
            .write(true) // 쓰기 권한 추가
            .create(true) // 파일이 없으면 생성
            .append(true) // 파일 끝에 내용 추가
            .open(&opponent_file_path)
            .unwrap_or_else(|_| { // 파일 열기
                // 파일이 없으면 빈 파일 생성
                OpenOptions::new()
                    .write(true)
                    .create(true)
                    .open(&opponent_file_path)
                    .expect("상대 플레이어 전적 파일을 생성할 수 없습니다.")
            });

        writeln!(opponent_file, "{} {}", self.username, opponent_result).expect("상대 플레이어 전적 파일에 기록할 수 없습니다.");

        // 상대 플레이어가 player_list.txt에 없으면 추가
        let player_list_path = "src/player_list.txt";
        let player_names = fs::read_to_string(player_list_path).unwrap_or_else(|_| {
            // 파일이 없으면 빈 파일 생성
            OpenOptions::new()
                .write(true)
                .create(true)
                .open(player_list_path)
                .expect("플레이어 목록 파일을 생성할 수 없습니다.");
            String::new()
        });
        if !player_names.lines().any(|name| name == opponent) {
            let mut player_list_file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(player_list_path)
                .expect("플레이어 목록 파일을 열 수 없습니다.");
            writeln!(player_list_file, "{}", opponent).expect("플레이어 목록 파일에 기록할 수 없습니다.");
        }
    }
}