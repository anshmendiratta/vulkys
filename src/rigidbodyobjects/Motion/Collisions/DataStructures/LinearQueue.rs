// pub mod LinearQueue;
use std::collections::LinkedList;

pub struct LinearQueue<T> {
    elements: LinkedList<T>,
}

#[derive(length)]
impl LinearQueue {
    pub fn new() -> Self {
        LinkedList::new()
    }
    pub fn enqueue(&self, item: Collision) {
        &self.push(item)
    }

    pub fn dequeue(&self, item: Collision) -> Collision {
        &self.pop().unwrap()
    }

    pub fn peek(&self) -> Collision {
        &self.last()
    }
}
