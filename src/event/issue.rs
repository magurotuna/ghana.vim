use crate::event::EventHandler;

pub trait Issue {
    fn list_issue(&self) {
        print!("Hello");
    }
}
