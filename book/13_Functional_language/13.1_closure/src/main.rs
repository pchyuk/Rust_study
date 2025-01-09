use std::thread;

#[derive(Debug, PartialEq, Copy, Clone)]
enum ShirtColor {
    Red,
    Blue,
}

struct Inventory {
    shirts: Vec<ShirtColor>,
}

impl Inventory {
    fn giveaway(&self, user_preference: Option<ShirtColor>) -> ShirtColor { // Option<ShirtColor> 타입의 user_preference 매개변수
        user_preference.unwrap_or_else(|| self.most_stocked()) // user_preference가 Some일 경우 unwrap_or_else의 첫 번째 클로저가 실행되지 않음
    }

    fn most_stocked(&self) -> ShirtColor { // 가장 많이 재고가 있는 색을 반환
        let mut num_red = 0;
        let mut num_blue = 0;

        for color in &self.shirts {
            match color {
                ShirtColor::Red => num_red += 1,
                ShirtColor::Blue => num_blue += 1,
            }
        }

        if num_red > num_blue {
            ShirtColor::Red
        } else {
            ShirtColor::Blue
        }
    }
}

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

fn main() {
// 예제 13-1: 셔츠 회사 증정 상황
    // let store = Inventory {
    //     shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
    // };

    // let user_pref1 = Some(ShirtColor::Red);
    // let giveaway1 = store.giveaway(user_pref1);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref1, giveaway1
    // );

    // let user_pref2 = None;
    // let giveaway2 = store.giveaway(user_pref2);
    // println!(
    //     "The user with preference {:?} gets {:?}",
    //     user_pref2, giveaway2
    // );

    // let expensive_closuer = |num: u32| -> u32 { // expensive_closure 클로저
    //     println!("calculating slowly...");
    //     thread::sleep(Duration::from_secs(2)); // 2초 동안 대기
    //     num // num 반환
    // }

// 예제 13.2: 클로저에 매개변수와 반환 값의 타입을 추가적으로 명시하기
    // fn  add_one_v1   (x: u32) -> u32 { x + 1 }  // add_one_v1 함수,   함수 정의(클로저가 아님)
    // let add_one_v2 = |x: u32| -> u32 { x + 1 }; // add_one_v2 클로저, 모든 것이 명시된 클로저 정의
    // let add_one_v3 = |x|             { x + 1 }; // add_one_v3 클로저, 타입 명시를 제거 (타입 추론을 사용한 클로저 정의)
    // let add_one_v4 = |x|               x + 1  ; // add_one_v4 클로저, 중괄호를 제거한 클로저 정의 (딱 하나의 표현식일 때만 가능)

// 예제 13-3: 두 개의 다른 타입에 대해 타입이 추론되는 클로저 호출 시도하기기
    // let example_closure = |x| x; // 클로저 정의

    // let s = example_closure(String::from("hello")); // 클로저 호출
    // let n = example_closure(5); // 에러 발생 - 위의 클로저에서 String 타입을 반환했는데, 여기서는 i32 타입을 반환하려고 하기 때문

// 예제 13-4: 불변 참조자를 캡쳐하는 클로저의 정의와 호출
    // let list = vec![1, 2, 3]; // 벡터 생성
    // println!("Before defining closure: {:?}", list);

    // let only_borrows = || println!("From closure: {:?}", list); // 클로저 정의

    // println!("Before calling closure: {:?}", list);
    // only_borrows(); // 클로저 호출
    // println!("After calling closure: {:?}", list);

// 예제 13-5: 가변 참조자를 캡쳐하는 클로저의 정의와 호출
    // let mut list = vec![1, 2, 3]; // 가변 참조자를 사용하기 위해 list를 가변으로 선언
    // println!("Before defining closure: {:?}", list);

    // let mut borrows_mutably = || list.push(7); // 클로저 정의

    // borrows_mutably(); // 클로저 호출
    // println!("After calling closure: {:?}", list);

// 예제 13-6: 스레드에 대한 클로저가 list의 소유권을 갖도록 move 사용하기
    // let list = vec![1, 2, 3]; // 벡터 생성
    // println!("Before defining closure: {:?}", list);

    // thread::spawn(move || println!("From thread: {:?}", list)) // move 키워드를 사용하여 list의 소유권을 클로저로 이동
    //     .join()
    //     .unwrap();

// 예제 13-7: sort_by_key를 사용하여 너비로 사각형 정렬하기
    // let mut list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // list.sort_by_key(|r| r.width); // sort_by_key 메서드를 사용하여 너비로 정렬
    // println!("{:#?}", list);

// 예제 13-8: FnOnce 클로저를 sort_by_key에 사용 시도하기 (컴파일 안됨)
    // let mut list = [
    //     Rectangle { width: 10, height: 1 },
    //     Rectangle { width: 3, height: 5 },
    //     Rectangle { width: 7, height: 12 },
    // ];

    // let mut sort_operations = vec![];
    // let value = String::from("by key called");

    // list.sort_by_key(|r| {
    //     sort_operations.push(value);
    //     r.width
    // });
    // println!("{:#?}", list);

// 예제 13-9: FnMut 클로저를 sort_by_key에 사용하는 것은 허용됩니다.
    let mut list = [
        Rectangle { width: 10, height: 1 },
        Rectangle { width: 3, height: 5 },
        Rectangle { width: 7, height: 12 },
    ];

    let mut num_sort_operations = 0; // 정렬에 사용된 연산 횟수를 저장할 변수
    list.sort_by_key(|r| { // sort_by_key 메서드를 사용하여 너비로 정렬
        num_sort_operations += 1; // num_sort_operations 변수를 증가
        r.width // 너비를 반환
    });
    println!("{:#?}, sorted in {num_sort_operations} operations", list); // 정렬된 리스트와 정렬에 사용된 연산 횟수 출력
}