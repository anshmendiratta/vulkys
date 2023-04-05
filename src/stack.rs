use crate::rigidbodyobjects::motion::collisions::dataStructures::linearqueue::LinearQueue;
use crate::rigidbodyobjects::rigidbodies::RigidBody;

pub struct Stack {
    elements: LinearQueue<RigidBody>,
    pointer: u32,
}

impl Stack {
    pub fn new() -> Self {
        Self {
            elements: LinearQueue::new(),
            pointer: 0,
        }
    }

    pub fn push(mut self, item: RigidBody) {
        &self.elements.enqueue(item)
    }

    pub fn pop(mut self) {
        if &self.elements.length() == 0 {
            panic!("Stack Underflow: Attempted to pop from a stack of length 0")
        } else {
            &self.elements.dequeue()
        }
    }
}
