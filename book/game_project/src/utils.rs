use std::fs;
use std::io::{self, Write}; // Write 트레이트를 추가

pub fn read_player_names(file_path: &str) -> io::Result<Vec<String>> {
    // 파일의 내용을 문자열로 읽어옴
    let contents = fs::read_to_string(file_path)?;
    // 각 줄을 플레이어 이름으로 변환하여 벡터에 저장
    let player_names = contents.lines().map(String::from).collect();
    // 플레이어 이름 벡터를 반환
    Ok(player_names)
}

pub fn write_user_statistics(file_path: &str, username: &str, stats: &str) -> io::Result<()> {
    // 파일을 쓰기 모드로 열거나, 파일이 없으면 생성하고, 파일 끝에 내용을 추가
    let mut file = fs::OpenOptions::new()
        .write(true)
        .create(true)
        .append(true)
        .open(file_path)?;
    // 파일에 유저 이름과 전적을 기록
    writeln!(file, "{}: {}", username, stats)?;
    // 성공적으로 완료되었음을 반환
    Ok(())
}