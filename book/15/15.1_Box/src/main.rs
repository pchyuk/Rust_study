enum List {
    // Cons(i32, List),
    Cons(i32, Box<List>),
    Nil,
}

use crate::List::{Cons, Nil};

fn main() {
// 예제 12-1: 박스를 사용하여 i32 값을 힙에 저장하기
    
    // // 가장 직관적인 스마트 포인터는 박스(Box)이다.
    // // 박스는 스택이 아니라 힙에 데이터를 저장할 수 있도록 해준다.
    // // 스택에 남는 것은 힙 데이터를 가리키는 포인터이다.
    // let b = Box::new(5);
    // println!("b = {}", b);
    // // 박스 안에 있는 데이터는 마치 이 데이터가 스택에 있는 것처럼 접근 가능하다.
    // // b가 main의 끝에 도달하는 것처럼 어떤 박스가 스코프를 벗어날 때, 다른 어떤 소유된 값과 마찬가지로 할당은 해제될 것이다.

    // // 단일 값을 힙에 집어넣는 것은 그다지 유용하지 않다.

    // // 박스를 쓰지 않으면 허용되지 않을 타입을 박스로 정의하는 경우를 살펴보자.

    // let list = Cons(1, Cons(2, Cons(3, Nil)));
    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));

    // box 정리
    // 박스는 연결 리스트 같은 느낌이다.
    // 콘스 리스트와 같은 재귀 구조를 만들 때,
    // List의 크기를 알 수 없으므로 구조체의 사이즈가 무한히 커질 수 있지만,
    // 박스를 사용하면 포인털오써 주소값만을 저장하므로 얼마의 크기가 필요한지 크기를 확정할 수 있게 되고,
    // 더이상 구조체는 무한대의 크기를 가지지 않게 된다.

    // Box 는 Deref 트레이트를 구현하고 있기 때문에 스마트 포인터이다.
    // Deref와 Drop 트레이트가 매우 중요하다.
}