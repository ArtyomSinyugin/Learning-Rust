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
