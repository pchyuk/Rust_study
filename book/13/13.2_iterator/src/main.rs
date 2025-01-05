pub trait Iterator { // 반복자 트레이트 정의, 딱 하나의 메서드 정의를 요구
    type Item; // 연관 타입(associated type)

    fn next(&mut self) -> Option<Self::Item>; // next 메서드

    // 기본 구현이 있는 메서드는 생략략
}

fn main() {
// 예제 13-10: 반복자 생성하기, 예제 13-11: for 루프에서 반복자 사용하기
    // let v1 = vec![1, 2, 3]; // 벡터 생성

    // let v1_iter = v1.iter(); // 반복자 생성

    // for val in v1_iter { // 반복자를 사용하여 벡터의 값을 출력
    //     println!("Got: {}", val);
    // }

// 예제 13-14: 반복자 어댑터 map을 호출하여 새로운 반복자 생성하기
    // let v1: Vec<i32> = vec![1, 2, 3]; // 벡터 생성

    // v1.iter().map(|x| x + 1); // map 메서드를 호출하여 새로운 반복자 생성
    // // 위 코드는 경고를 발생시킨다.
    // // map 메서드는 반복자 어댑터이므로, 실제로 아무것도 수행하지 않는다.
    // // 넘겨진 클로저는 호출되지 않는다.

// 예제 13-15: map을 호출하여 새로운 반복자를 생성한 다음,
// collect 메서드를 호출하여 이 반복자를 소비하고 새로운 벡터 생성하기
    let v1: Vec<i32> = vec![1, 2, 3]; // 벡터 생성

    let v2: Vec<_> = v1.iter().map(|x| x + 1).collect(); // map 메서드를 호출하여 새로운 반복자 생성, collect 메서드를 호출하여 새로운 벡터 생성

    assert_eq!(v2, vec![2, 3, 4]); // 새로운 벡터가 [2, 3, 4]인지 확인
}