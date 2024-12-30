// 예제 4-5: 매개변수의 소유권을 되돌려주는 방법

fn main() {
    let s1 = String::from("hello");

    let (s2, len) = calculate_length(s1);

    println!("The length of '{}' is {}.", s2, len);
}

fn calculate_length(s: String) -> (String, usize) {
    let length = s.len(); // len()은 String의 길이를 반환합니다

    (s, length)
}