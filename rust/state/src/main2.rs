use blog::Post;

fn main() {
    let mut post = Post::new();

    post.add_text("I have lunch");

    let post = post.request_review();

    let post = post.approve();
    assert_eq!("I have lunch", post.content());
}
