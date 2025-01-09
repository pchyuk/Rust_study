struct CustomSmartPointer {
    data: String,
}

impl Drop for CustomSmartPointer {
    fn drop(&mut self) {
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() {
    // let c = CustomSmartPointer {
    //     data: String::from("my stuff"),
    // }
    // let d = CustomSmartPointer {
    //     data: String::from("other stuff"),
    // };
    // println!("CustomSmartPointers created.");

    let c = CustomSmartPointer {
        data: String::from("some data"),
    };
    println!("CustomSmartPointer created.");
    // c.drop();
    // drop 함수를 명시적으로 호출하는 것은 허용되지 않는다.
    // 러스트가 main의 끝부분에서 그 값에 대한 drop 호출을 자동으로 할 것이기 때문이다.

    // 어떤 값에 대한 메모리 정리를 강제로 일찍 하길 원할 때는 std::mem::drop 함수를 사용한다.

    drop(c); // drop 함수를 호출하여 CustomSmartPointer 인스턴스를 수동으로 드롭한다.
    
    println!("CustomSmartPointer dropped before the end of main.");
}