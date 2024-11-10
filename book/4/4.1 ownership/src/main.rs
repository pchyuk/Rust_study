// 4.1. 소유권이 뭔가요?

fn main() {
// 소유권 규칙
    // 1. 러스트에서, 각각의 값은 소유자(owner)가 정해져 있다.
    // 2. 한 값의 소유자는 동시에 여럿 존재할 수 없다.
    // 3. 소유자가 스코프 밖으로 벗어날 때, 값은 버려진다.(dropped)

// 변수의 스코프

    {                       // s는 아직 선언되지 않아서 여기서는 유효하지 않다.
        let s = "hello";    // 이 지점부터 s가 유효하다.

        // s로 어떤 작업을 한다.
    }                       // 이 스코프가 종료되었고, s가 더 이상 유효하지 않다.

    // 중요한 점은 2가지
        // 1. s가 스코프 "내에" 나타나면 유효하다.
        // 2. 유효기간은 스코프 "밖으로" 벗어나기 전까지 이다.

// String 타입

    let s = String::from("hello");

    s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가한다.

    println!("{}", s); // 이 줄이 `hello, world!`를 출력한다.

// 메모리와 할당
    {
        let s = String::from("hello"); // s는 이 지점부터 유효하다.

        // s를 가지고 무언가 한다.
    }                                  // 이 스코프가 종료되었고, s는 더 이상 유효하지 않다.


    // 여기부터는 오늘 점심(11일 월요일)에 이어서 할 것
}