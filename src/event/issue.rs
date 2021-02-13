use crate::event::EventHandler;

pub trait Issue {
    fn list_issue(&self);
}

impl Issue for EventHandler {
    fn list_issue(&self) {
        // able to access to self.client
        let _c = &self.client;
    }
}
