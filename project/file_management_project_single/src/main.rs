fn main() {
    use std::fs; // 파일 시스템 조작을 위한 모듈
    use std::path::Path; // 파일 경로 조작을 위한 구조체
    use std::io; // 표준 입출력 작업을 위한 모듈

    println!("정리할 디렉토리의 이름(또는 경로)을 입력하세요:");

    let mut input = String::new(); // 입력을 String 타입으로 받을 변수를 선언

    io::stdin().read_line(&mut input).expect("입력 오류"); // 표준 입력에서 사용자 입력을 읽음

    let dir_path = input.trim(); // 입력받은 경로에서 개행 문자를 제거

    
    if !Path::new(dir_path).is_dir() { // 지정된 경로가 유효한 디렉토리인지 확인
        println!("유효하지 않은 디렉토리입니다.");
        return;
    }

    let entries = fs::read_dir(dir_path).expect("디렉토리 읽기 오류"); // 디렉토리 내의 모든 항목을 읽음

    // 파일 확장자에 따라 폴더를 생성하고 파일을 이동
    for entry in entries { // 반복자를 사용하여 디렉토리 내의 모든 항목에 대해 반복
        let entry = entry.expect("항목 읽기 오류"); // 항목을 읽음
        let path = entry.path(); // 항목의 경로를 가져와서 path 변수에 저장

        if path.is_file() { // 파일인지 확인
            if let Some(extension) = path.extension() { // 파일의 확장자를 가져옴
                let ext_str = extension.to_string_lossy(); // 확장자를 문자열로 변환
                
                let folder_path = format!("{}/{}", dir_path, ext_str); // 확장자에 해당하는 폴더 경로를 생성
                
                if !Path::new(&folder_path).exists() { // 폴더가 존재하지 않으면
                    fs::create_dir(&folder_path).expect("폴더 생성 오류"); // 폴더를 생성
                }

                // 파일을 해당 폴더로 이동
                let new_file_path = format!("{}/{}", folder_path, path.file_name().unwrap().to_string_lossy()); // 새 파일 경로를 생성
                fs::rename(&path, &new_file_path).expect("파일 이동 오류"); // 파일을 새 경로로 이동
            }
        }
    }

    println!("파일 정리가 완료되었습니다."); // 작업 완료 메시지를 출력
}