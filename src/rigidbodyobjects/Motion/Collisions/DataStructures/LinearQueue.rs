use std::collections::LinkedList;

pub struct LinearQueue {
    elements: LinkedList<Collision>,
}

#[derive()]
impl LinearQueue {
    pub fn new() -> Self {
        LinkedList::new()
    }
    pub fn enqueue(&self, item: Collision) {
        &self.push(item)
    }

    pub fn dequeue(&self, item: Collision) -> Collision {
        &self.pop()
    }

    pub fn peek(&self) -> Collision {
        &self.last()
    }
}
