use crate::rigidbodyobjects::motion::collisions::datastructures::linearqueue::LinearQueue;
use crate::rigidbodyobjects::rigidbodies::RigidBody;

#[derive(Debug)]
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

    pub fn push(&mut self, item: RigidBody) {
        &self.elements.enqueue(item)
    }

    pub fn pop(&mut self) -> Result<RigidBody, E> {
        &self.elements.pop().unwrap()
    }
}
