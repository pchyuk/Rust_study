use std::io;

fn main() {
//     // let guess: u32 = "42".parse().expect("Not a number!");

//     let x = 2.0; // f64

//     let y: f32 = 3.0; // f32

// // 수치 연산

//     // 덧셈
//     let sum = 5 + 10;

//     // 뺄셈
//     let difference = 95.5 - 4.3

//     // 곱셈
//     let product = 4 * 30;

//     // 나눗셈
//     let quotient = 56.7 / 32.2;
//     let truncated = -5 / 3; // 결괏값은 -1입니다.

//     // 나머지 연산
//     let remainder = 43 % 5;


// // boolean 타입

//     let t = true;

//     let f: bool = false;

// // 문자 타입

//     let c = 'z';
//     let z: char = 'ℤ'; // 명시적인 타입 어노테이션
//     let heart_eyed_cat = '😻';

// // 복합 타입

//     // 튜플 타입
//     let tup: (i32, f64, u8) = (500, 6.4, 1);

//     let tup = (500, 6.4, 1);

//     let (x, y, z) = tup;

//     println!("The value of y is: {y}");


//     let x: (i32, f64, u8) = (500, 6.4, 1);

//     let five_hundred = x.0;

//     let six_point_four = x.1;

//     let one = x.2;

// // 배열 타입

//     let a = [1, 2, 3, 4, 5];

//     let months = ["January", "February", "March", "April", "May", "June", "July", "August", "September", "October", "November", "December"];

//     let a: [i32; 5] = [1, 2, 3, 4, 5];

//     let a = [3; 5]; // let a = [3, 3, 3, 3, 3];

// // 배열 요소에 접근

//     let a = [1, 2, 3, 4, 5];

//     let first = a[0];
//     let second = a[1];

// 유효하지 않은 배열 요소에 대한 접근

    let a = [1, 2, 3, 4, 5];

    println!("Please enter an array index");

    let mut index = String::new();

    io::stdin()
        .read_line(&mut index)
        .expect("Fail to read line");

    let index: usize = index
        .trim()
        .parse()
        .expect("Index entered was not a number");

    let element = a[index];

    println!("The value of the element at index {index} is: {element}");
}