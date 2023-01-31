pub struct Post {  // первое из двух решений реализации кода
    content: String,
}
impl Post {
    pub fn new() -> DraftPost {
        DraftPost {
            content: String::new(),
        }
    }
    pub fn content(&self) -> &str {
        &self.content
    }
}
pub struct DraftPost {
    content: String,
}
impl DraftPost {
    pub fn add_text(&mut self, text: &str) {
        self.content.push_str(text);
    }
    pub fn request_review(self)-> PendingReviewPost {
        PendingReviewPost {
            content: self.content,
        }  
    }
}
pub struct PendingReviewPost {
    content: String,
}
impl PendingReviewPost {
    pub fn approve(self)-> PendingReviewPost2 {
        PendingReviewPost2 {
                content: self.content,
            } 

    }
    pub fn reject(self)-> DraftPost {
        DraftPost {
            content: self.content,
        }       
    }
}
pub struct PendingReviewPost2 {
    content: String,
}
impl PendingReviewPost2 {
    pub fn approve(self)-> Post {
            Post {
                content: self.content,
            } 

    }
    pub fn reject(self)-> DraftPost {
        DraftPost {
            content: self.content,
        }       
    }
}
/*
pub struct Post {              // второй вариант с кодированием состояния с помощью типаж-объектов
    state: Option<Box<dyn State>>,
    content: String,
}
impl Post {
    pub fn new () -> Post {
        Post {
            state: Some(Box::new(Draft {})),
            content: String::new(),
        }
    }
    pub fn add_text(&mut self, text: &str) {
        let bool = self.state
            .as_ref()
            .unwrap()
            .check();
        if bool {
            self.content.push_str(text)
        } else {
            println!("Вы не! можете добавлять текст в этом состоянии");
        }
    }   
        //let state = self.state.take().unwrap(); // Хороший вариант решить задачу, но вопреки её требованиям
        //state.add_text(self, text);
        //self.state = Some(state);  // так-как take() замещает Some на None, нужно вернуть Some обратно
        // self.content.push_str(text); // первоначальное упрощение
    
    pub fn content(&self)-> &str {                           // отправляем content в State
        self.state.as_ref().unwrap().content(self)
    }
    pub fn request_review (&mut self) {
        match self.state.take() {              // take здесь пока самый непонятный элемент. Он забирает Some, и что? Some теперь находится в self.state?
            Some(s) => self.state = Some(s.request_review()),
            None => self.state = Some(Box::new(Draft {}))           
        }
        //if let Some(s) = self.state.take() { // take здесь пока самый непонятный элемент. Он забирает Some, и что? Some теперь находится в self.state?
        //    self.state = Some(s.request_review())
        //}
    }
    pub fn approve (&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.approve())
        }
    }
    pub fn reject (&mut self) {
        if let Some(s) = self.state.take() {
            self.state = Some(s.reject())
        }
    }
}

trait State {
    fn request_review(self: Box<Self>) -> Box<dyn State>;
    fn content<'a>(&self, post: &'a Post) -> &'a str {                // работаем с content
        ""
    }
    fn approve(self: Box<Self>) -> Box<dyn State>;
    fn reject(self: Box<Self>) -> Box<dyn State>;
    fn check(&self) -> bool {false}
}
struct Draft {
}

impl State for Draft {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        Box::new(PendingReview { count: 0 })
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn check(&self) -> bool {true}
}
struct PendingReview {
    count: u8,
}

impl State for PendingReview {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }  
    fn approve(self: Box<Self>) -> Box<dyn State> {
        match self.count {
            1 => Box::new(Published {}),
            _ => Box::new(PendingReview { count: 1 })
        }
        
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        Box::new(Draft {})
    }
}
struct Published {
}

impl State for Published {
    fn request_review(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn content<'a>(&self, post: &'a Post) -> &'a str {              // content возвращает срез строки
        &post.content
    }
    fn approve(self: Box<Self>) -> Box<dyn State> {
        self
    }
    fn reject(self: Box<Self>) -> Box<dyn State> {
        self
    }
} */