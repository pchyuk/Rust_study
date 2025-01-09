enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

fn main() {
    // let a = Cons(5, Box::new(Cons(10, Box::new(Nil))));
    // let b = Cons(3, Box::new(a));
    // // let c = Cons(4, Box::new(a)); // a를 두 번 소유하려고 하므로 컴파일되지 않는다.

    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    println!("count after creating a = {}", Rc::strong_count(&a));

    let b = Cons(3, Rc::clone(&a)); // Rc::clone을 호출하여 Rc<List>의 카운터를 증가시킨다.
    println!("count after creating b = {}", Rc::strong_count(&a));

    {
        let c = Cons(4, Rc::clone(&a));
        println!("count after creating c = {}", Rc::strong_count(&a));
    }

    println!("count after c goes out of scope = {}", Rc::strong_count(&a));

    // let c = Cons(4, Rc::clone(&a)); // a를 두 번 소유하려고 해도 컴파일된다.
    // a.clone()을 호출할 수도 있지만, Rc::clone은 러스트의 관습이다.
    // 대부분의 clone 구현체가 그러하듯 Rc::clone은 깊은 복사를 수행하지 않는다.
}