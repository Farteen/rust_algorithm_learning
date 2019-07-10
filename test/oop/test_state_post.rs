//pub struct Post {
//    state: Option<Box<dyn State>>,
//    content: String,
//}

//
//impl Post {
//    pub fn new() -> Post {
//        Post {
//            state: Some(Box::new(Draft{})),
//            content: String::new(),
//        }
//    }
//}
//
//traitd State {
//    fn content<'a>(&self, post: &'a Post) -> &'a str;
//}
//
//struct Draft {}
//
//impl State for Draft {
//
//}