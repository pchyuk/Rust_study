// 3.5. 제어 흐름문

fn main() {

// if 표현식

    let number = 3;

    if number < 5 {
        println!("condition was true"); // 이 줄이 실행
    } else {
        println!("condition was false");
    }


    let number = 7;

    if number < 5 {
        println!("condition was true");
    } else {
        println!("condition was false"); // 이 줄이 실행
    }


    // if number { <- 여기가 문제임
    //     println!("number was three");
    // }

    // 위의 코드는 에러를 발생시킨다.
    // if 조건식의 결과는 bool형이어야 한다.


    // 수정한 코드
    let number = 3;

    if number != 0 {
        println!("number was something other than zero");
    }

// else if로 여러 조건식 다루기

    let number = 6;

    if number % 4 == 0 {
        println!("number is divisible by 4");
    } else if number % 3 == 0 {
        println!("number is divisible by 3"); // 여기가 실행
    } else if number % 2 == 0 {
        println!("number is divisible by 2");
    } else {
        println!("number is not divisible by 4, 3, or 2");
    }

// let 구문에서 if 사용하기

    let condition = true;
    let number = if condition { 5 } else { 6 };

    println!("The value of number is: {number}");

    // let condition = true;

    // let number = if condition { 5 } else { "six" }; <- 여기서 에러 발생
        // if문은 정수값인데 else문은 문자열이다.
        // 변수가 가질 수 있는 타입은 오직 하나이다.
        // Rust는 컴파일 시점에 number 변수의 타입이 무엇인지 확실히 알 필요가 있다.
        // 그래야 컴파일 시점에 number가 사용되는 모든 곳에서 해당 타입이 유효한지 검증할 수 있다. 
        // 러스트에서는 number의 타입이 런타임에 정의되도록 할 수 없다.

    // println!("The value of number is: {number}");


// 반복문을 이용한 반복
    
    // loop로 코드 반복하기
    loop {
        // println!("again!"); <- 무한 반복. Ctrl + C로 빠져나올 수 있다.
    }


    // 반복문에서 값 반환하기
    let mut counter = 0;

    let result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {result}"); // 결과: 20


    // 루프 라벨로 여러 반복문 사이에 모호함 없애기
    let mut count = 0;
    'counting_up: loop {
        println!("count = {count}");
        let mut remaining = 10;

        loop {
            println!("remaining = {remaining}");
            if remaining == 9 {
                break;
            }
            if count == 2 {
                break 'counting_up;
            }
            remaining -= 1;
        }

        count += 1;
    }
    println!("End count = {count}");


    // while을 이용한 조건 반복문
    let mut number = 3;

    while number != 0 {
        println!("{number}!");

        number -= 1;
    }

    println!("LIFTOFF!!!");


    // for를 이용한 컬렉션에 대한 반복문
    let a = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("the value is: {}", a[index]);

        index += 1;
    }

    // 좀 더 간편한 대안
    let a = [10, 20, 30, 40, 50];

    for element in a {
        println!("the value is: {element}");
    }

    // for 반복문을 이용한 카운트다운 구현
    for number in (1..4).rev() {
        println!("{number}!");
    }
    println!("LIFTOFF!!!");
}