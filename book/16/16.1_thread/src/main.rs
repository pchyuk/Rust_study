use std::thread;
use std::time::Duration;

fn main() {
    // let handle = thread::spawn(|| {
    //     for i in 1..10 {
    //         println!("hi number {} from the spawned thread!", i);
    //         thread::sleep(Duration::from_millis(1));
    //     }
    // });

    // handle.join().unwrap(); // 여기에 join을 호출하면 메인 스레드는 handle 스레드가 종료될 때까지 기다린다.
    // // 출력값이 더 이상 교차하지 않는다.
    // // join이 호출되는 위치처럼 작은 세부 사항도 스레드가 동시에 실행되는지의 여부에 영향을 미칠 수 있다.

    // for i in 1..5 {
    //     println!("hi number {} from the main thread!", i);
    //     thread::sleep(Duration::from_millis(1));
    // }

    // 러스트는 메인 스레드가 종료되면 
    // 생성된 모든 스레드는 실행이 종료되었든 그렇지 않든 상관없이 멈추게 된다.
    // 따라서 spawn을 통해 만들어진 스레드에서 반복문이 10번을 다 돌기 전에 끝나게 된다.

    // 스레드들은 아마도 번갈아서 실행될 것이지만, 그것이 보장되지는 않는다.

    // 생성된 스레드가 실행되지 않거나, 전부 실행되지 않는 문제는 
    // thread::spawn의 반환값을 변수에 저장함으로써 해결할 수 있다.
    // thread::spawn의 반환값은 JoinHandle이라는 타입이다.

    // handle.join().unwrap();
    // handle.join()을 호출하면 메인 스레드는 handle 스레드가 종료될 때까지 기다린다.

    let v = vec![1, 2, 3];

    let handle = thread::spawn(move || {
        println!("Here's a vector: {:?}", v);
        // v를 사용하려고 하면 컴파일러는 에러를 발생시킨다.
        // println!이 v의 참조자만 필요로 하기 때문에, 클로저는 v를 빌리려고 한다.
        // 하지만 러스트는 생성된 스레드가 얼마나 오랫동안 실행될지 알 수 없으므로,
        // v의 수명을 보장할 수 없다.
    });

    // drop(v); // oh shit!
    // 만약 이 코드가 정상 동작한다면, 생성된 스레드가 전혀 실행되지 않고 즉시 백그라운드로 들어갔을 가능성이 있다.
    // main thread는 drop 함수를 사용하여 v를 즉시 버린다.
    // 그러면 생성된 스레드가 실행되기 시작할 때, v는 유효하지 않고, 그에 대한 참조자 또한 유효하지 않게 된다.
    
    // 해결법: 클로저(||) 앞에 move 키워드를 추가함으로써 클로저가 사용 중인 값의 소유권을 강제로 가지도록 한다.
    // 소유권 빌리는 거 아님.
    
    // drop은 당연히 쓰면 안되겠죠?
    // move와 drop을 같이 쓰면, 소유권이 아닌 다른 이유로 컴파일 에러가 발생한다.
    // 만일 클로저에 move를 추가하면, v를 클로저의 환경으로 이동시킬 것이고, 
    // 더 이상 메인 스레드에서 v에 대한 drop 호출을 할 수 없게 된다.

    handle.join().unwrap();

}
