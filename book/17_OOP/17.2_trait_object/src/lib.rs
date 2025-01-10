
// Draw 트레이트의 정의
pub trait Draw {
    fn draw(&self);
}

// Draw 트레이트를 구현하는 트레이트 객체들의 벡터 components를 필드로 가지고 있는 Screen 구조체의 정의
pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

// 수정 -> Screen 구조체에 대한 제네릭 타입 T를 추가하고, T가 Draw 트레이트를 구현하도록 제약을 추가
// 제네릭과 트레이트 바운드를 사용한 Screen 구조체와 run 메서드의 대체 구현
// pub struct Screen<T: Draw> {
//     pub components: Vec<T>,
// }

// 각 컴포넌트에 대해 draw 메서드를 호출하는 Screen의 run 메서드
impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

// impl<T> Screen<T>
// where
//     T: Draw,
// {
//     pub fn run(&self) {
//         for component in self.components.iter() {
//             component.draw();
//         }
//     }
// }

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        // println!("Button: width={}, height={}, label={}", self.width, self.height, self.label);
    }
}