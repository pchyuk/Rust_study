// 예제 13-12: 반복자의 next 메서드를 (직접) 호출하기
#[test]
fn iterator_demonstration() {
    let v1 = vec![1, 2, 3]; // 벡터 생성

    let mut v1_iter = v1.iter(); // 반복자 생성, 가변(mut)으로 선언하여 next 메서드를 호출할 수 있도록 함
    // for 루프를 사용할 때는 v1_iter를 가변으로 만들 필요가 없는데,
    // 루프가 v1_iter의 소유권을 갖고 내부적으로 가변으로 만들기 때문이다.

    assert_eq!(v1_iter.next(), Some(&1)); // next 메서드를 호출하여 첫 번째 요소를 가져옴
    assert_eq!(v1_iter.next(), Some(&2)); // next 메서드를 호출하여 두 번째 요소를 가져옴
    assert_eq!(v1_iter.next(), Some(&3)); // next 메서드를 호출하여 세 번째 요소를 가져옴
    assert_eq!(v1_iter.next(), None); // next 메서드를 호출하여 더 이상 요소가 없음을 확인
}

// 예제 13-13: sum 메서드를 호출하여 반복자의 모든 아이템에 대한 합계 구하기
#[test]
fn iterator_sum() {
    let v1 = vec![1, 2, 3]; // 벡터 생성

    let v1_iter = v1.iter(); // 반복자 생성

    let total: i32 = v1_iter.sum(); // sum 메서드를 호출하여 반복자의 모든 아이템에 대한 합계를 구함
    // sum은 반복자를 소유하여 호출하므로,
    // sum을 호출한 이후에는는 v1_iter를 사용할 수 없다.

    assert_eq!(total, 6); // 합계가 6인지 확인
}

// 예제 13-16: shoe_size를 캡처하는 클로저로 filter 메서드 사용하기
#[derive(PartialEq, Debug)]
struct Shoe {
    size: u32,
    style: String,
}

fn shoes_in_size(shoes: Vec<Shoe>, shoe_size: u32) -> Vec<Shoe> { // shoes_in_size 함수 정의
    // filter 메서드를 호출하여 캡처한 shoe_size와 같은 크기의 신발만 모아서 새로운 벡터를 생성
    shoes.into_iter().filter(|s| s.size == shoe_size).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn filters_by_size() {
        let shoes = vec![
            Shoe {
                size: 10,
                style: String::from("sneaker"),
            },
            Shoe {
                size: 13,
                style: String::from("sandal"),
            },
            Shoe {
                size: 10,
                style: String::from("boot"),
            },
        ];

        let in_my_size = shoes_in_size(shoes, 10);

        assert_eq!(
            in_my_size,
            vec![
                Shoe {
                    size: 10,
                    style: String::from("sneaker")
                },
                Shoe {
                    size: 10,
                    style: String::from("boot")
                },
            ]
        );
    }
}