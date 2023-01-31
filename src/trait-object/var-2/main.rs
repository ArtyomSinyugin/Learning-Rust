use design_pattern::Post;
fn main () {
    let mut post = Post::new();
    
    post
        .add_text("I ate a salad for lunch today");
    //assert_eq!("", post.content());  // для варианта с типаж-объектами

    let post = post
        .request_review();
    //assert_eq!("", post.content());  // для варианта с типаж-объектами

    let mut post = post.reject();

    post.add_text(". Valeria!");

    let post = post.request_review();
    let post = post.approve();
    let post = post.approve();
    //post.add_text("Wow");

    assert_eq!("I ate a salad for lunch today. Valeria!", post.content());
}