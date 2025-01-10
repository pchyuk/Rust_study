use gui::Draw;

struct SelectBox {
    width: i32,
    height: i32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // println!("SelectBox: width={}, height={}, options={:?}", self.width, self.height, self.options);
    }
}

use gui::{Button, Screen};

fn main() {
    let screen = Screen {
        // components: vec![
        //     Box::new(SelectBox {
        //         width: 75,
        //         height: 10,
        //         options: vec![
        //             String::from("Yes"),
        //             String::from("Maybe"),
        //             String::from("No"),
        //         ],
        //     }),
        //     Box::new(Button {
        //         width: 50,
        //         height: 10,
        //         label: String::from("OK"),
        //     }),
        // ],

        components: vec![Box::new(String::from("Hi"))],
    };
    
    screen.run();
}