mod blog;

use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I ate a salad for lunch today");
    assert_eq!("", post.content());
    // println!("After adding text: {}", post.content());

    post.request_review();
    assert_eq!("", post.content());
    // println!("After request review: {}", post.content());

    post.approve();
    assert_eq!("I ate a salad for lunch today", post.content());
    // println!("After approval: {}", post.content());
}