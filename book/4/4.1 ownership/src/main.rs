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

    let mut s = String::from("hello");

    s.push_str(", world!"); // push_str()이 문자열에 리터럴을 추가한다.

    println!("{}", s); // 이 줄이 `hello, world!`를 출력한다.

// 메모리와 할당
    {
        let s = String::from("hello"); // s는 이 지점부터 유효하다.

        // s를 가지고 무언가 한다.
    }                                  // 이 스코프가 종료되었고, s는 더 이상 유효하지 않다.

    // 변수와 데이터 간 상호작용 방식: 이동
    let x = 5; // 5를 x에 바인딩하고,
    let y = x; // x값의 "복사본"을 만들어 y에 바인딩한다.

    // String으로 변경
    let s1 = String::from("hello"); // "hello"를 s1에 바인딩하고,
    let s2 = s1; // "hello" 값을 복사해서 s2에 바인딩한다??

    // NO!
    // s1과 s2는 "hello"가 저장되어 있는 공간의 주소를 저장한다. (포인터처럼 동작)

    // Rust는 변수가 스코프 밖으로 벗어날 때 
    // 자동으로 drop 함수를 호출하여 해당 변수가 사용하는 힙 메모리를 제거한다.
    // 그럼 만약 s1, s2처럼 두 포인터가 같은 곳을 가리키고 있다면? - 중복 해제(double free) 문제
    // 이는 메모리 안전성 버그 중 하나이며, 보안을 취약하게 만드는 메모리 손상의 원인이다.

    // 메모리 안전성을 보장하기 위해서 Rust는
    // let s2 = s1; 라인 뒤로는 s1이 더 이상 유효하지 않다고 판단한다.

    // println!("{}, world!", s1); // 컴파일 에러 - 유효하지 않은 참조자의 사용 감지

    // 얕은 복사, 깊은 복사?
    // Rust에서는 기존의 변수를 "무효화" 하기 때문에
    // 이를 복사가 아닌 "이동(move)"이라 한다.
    // 앞선 코드는 s1이 s2로 이동되었다 라고 표현한다. 

    // s2만이 유효하니, 스코프 밖으로 벗어났을 때 자신만 메모리를 해제할 것
    // 문제 해결!

    // 추가: Rust는 절대 자동으로 "깊은" 복사로 데이터를 복사하는 일이 없다.
    // 따라서, Rust가 자동으로 수행하는 모든 복사는 런타임 측면에서 효율적이라 할 수 있다.


    // 변수와 데이터 간 상호작용 방식: 클론
    let s1 = String::from("hello");
    let s2 = s1.clone(); 
    
    // String의 힙 데이터까지 깊이 복사하고 싶을 땐
    // 위와 같이 clone이라는 공용 메서드를 사용할 수 있다.

    println!("s1 = {}, s2 = {}", s1, s2);


    // 스택에만 저장되는 데이터: 복사
    let x = 5;
    let y = x;
    
    println!("x = {}, y = {}", x, y);

    // 아까 말한 대로라면 위 코드는 안 돌아가야 한다.
    // clone을 호출하지도 않았는데 x는 계속해서 유효하며 y로 이동되지도 않았다.

    // 이유: 정수형 등 컴파일 타임에 크기가 고정되는 타입은 모두 "스택에 저장"되기 때문이다.
    // 스택에 저장되니, 복사본을 빠르게 만들 수 있고,
    // 따라서 굳이 y를 생성한 후에 x를 무효화할 필요가 없다.
    // 이런 경우에는 깊은 보가와 얕은 복사 간에 차이가 없다.


    // Copy 가능한 타입
    // 1. 모든 정수형 타입(예: u32)
    // 2. true, false 값을 갖는 논리 자료형 bool
    // 3. 모든 부동 소수점 타입(예: f64)
    // 4. 문자 타입 char
    // 5. Copy 가능한 타입만으로 구성된 튜플
        // (i32, i32)는 Copy 가능하지만
        // (i32, String)은 불가능

// 소유권과 함수

    let s = String::from("hello");  // s가 스코프 안으로 들어온다.

    takes_ownership(s);             // s의 값이 함수로 이동된다...
                                    // ... 따라서 여기서는 더 이상 유효하지 않다.

    let x = 5;                      // x가 스코프 안으로 들어온다.

    makes_copy(x);                  // x가 함수로 이동될 것이지만,
                                    // i32는 Copy이므로 앞으로 계속 x를
                                    // 사용해도 좋다.
} // 여기서 x가 스코프 밖으로 벗어나고 s도 그렇게 된다. 
  // 그러나 s의 값이 이동되었으므로 별다른 일이 발생하지 않는다.

fn takes_ownership(some_string: String) { // some_string이 스코프 안으로 들어온다.
    println!("{}", some_string);
} // 여기서 some_string이 스코프 밖으로 벗어나고 `drop`이 호출된다.
  // 메모리가 해제된다.

fn makes_copy(some_integer: i32) { // some_integer가 스코프 안으로 들어온다.
    println!("{}", some_integer);
} // 여기서 some_integer가 스코프 밖으로 벗어난다. 별다른 일이 발생하지 않는다.