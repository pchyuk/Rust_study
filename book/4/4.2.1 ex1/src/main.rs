// 예제 4-6: 빌린 값을 수정해 보는 코드 

fn main() {
    // let s = String::from("hello");

    let mut s = String::from("hello");

    // change(&s);

    change(&mut s);
}

fn change(some_string: &mut String) {
    some_string.push_str(", world");
}