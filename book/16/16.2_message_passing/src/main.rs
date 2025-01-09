use std::sync::mpsc;
use std::thread;
use std::time::Duration;

fn main() {
    let (tx, rx) = mpsc::channel(); // mpsc: 복수 생산자, 단일 소비자(multiple producer, single consumer)의 약자

    let tx1 = tx.clone(); 
    thread::spawn(move || {
        // let val = String::from("hi");
        // tx.send(val).unwrap(); // tx.send를 통해 채널에 val을 보냄
        // // println!("val is {}", val); // 이후 val 출력을 시도 - 에러!

        let vals = vec![
            String::from("hi"),
            String::from("from"),
            String::from("the"),
            String::from("thread"),
        ];

        for val in vals {
            tx1.send(val).unwrap(); // tx1.send를 통해 채널에 val을 보냄
            thread::sleep(Duration::from_secs(1)); // 1초 대기
        }
    });

    // let received = rx.recv().unwrap();
    // println!("Got: {}", received);

    thread::spawn(move || { // 새로운 스레드 생성, `move` 키워드를 사용하여 tx를 클로저로 이동
        let vals = vec![
            String::from("more"),
            String::from("messages"),
            String::from("for"),
            String::from("you"),
        ];

        for val in vals {
            tx.send(val).unwrap();
            thread::sleep(Duration::from_secs(1));
        }
    });

    for received in rx {
        println!("Got: {}", received);
    }
}