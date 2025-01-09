use std::ops::Deref;

impl<T> Deref for MyBox<T> { // Deref 트레이트를 구현한다.
    type Target = T; // 연관 타입을 정의한다.

    fn deref(&self) -> &Self::Target { // 역참조 연산자를 오버라이드한다.
        &self.0
    }
}

struct MyBox<T>(T); // 제네릭 타입 T를 받아들이는 MyBox 구조체를 정의한다.

impl<T> MyBox<T> { // MyBox 구조체에 대한 제네릭 타입 T를 받아들이는 new 메서드를 정의한다.
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn hello(name: &str) {
    println!("Hello, {name}!");
}

fn main() {
    // let x = 5;
    // // let y = &x; // 참조자를 만들어 x를 참조한다.
    // let y = Box::new(x); // 박스를 만들어 x를 박스 안에 넣는다. 이렇게 적어도 동일하게 작동한다.
    
    // // let y = MyBox::new(x); // MyBox를 만들어 x를 MyBox 안에 넣는다. 하지만 이렇게만 적으면 에러가 발생한다.
    // // MyBox는 역참조 될 수 없는데, 왜냐하면 이 타입에 그런 기능을 구현한 적이 없기 때문이다.
    // // 따라서 역참조 연산자를 사용하려면 Deref 트레이트를 구현해야 한다.

    // assert_eq!(5, x);
    // assert_eq!(5, *y);

    let m = MyBox::new(String::from("Rust"));
    hello(&m); // MyBox<String>을 참조자로 넘긴다. 역참조 강제 변환
    // MyBox<String> -> &String : MyBox<T>에 대한 Deref 트레이트를 구현했기 때문에 MyBox<String>을 &String으로 변환할 수 있다.
    // &String -> &str : 표준 라이브러리에 구현되어 있는 String의 Deref 트레이트가 &String(문자열)을 &str(문자열 슬라이스)로 변환한다.

    // hello(&(*m)[..]); // 역참조 연산자를 사용하여 MyBox<String>을 String으로 변환한다.
    // 러스트에 역참조 강제 변환이 구현되어 있지 않았다면, 코드는 위와 같이 작성해야 했을 것이다.

    // 역참조 강제 변환이 가변성과 상호작용하는 법

    // T: Deref<Target=U>일 때 &T에서 &U로
    // T: DerefMut<Target=U>일 때 &mut T에서 &mut U로
    // T: Deref<Target=U>일 때 &mut T에서 &U로

    // 불변 참조자 -> 불변 참조자 가능
    // 가변 -> 가변 가능
    // 가변 -> 불변 가능
    // 하지만 불변 -> 가변 은 불가능!
}
