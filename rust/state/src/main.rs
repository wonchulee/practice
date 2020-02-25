use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I have lunch");
    assert_eq!("", post.content());

    post.request_review();
    assert_eq!("", post.content());

    post.approve();
    assert_eq!("I have lunch", post.content());
}
