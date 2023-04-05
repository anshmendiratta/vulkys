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
            pointer: Result<u32, _>,
        }
    }

    pub fn push(mut self, item: RigidBody) {
        &self.elements.enqueue(item)
    }

    pub fn pop(mut self) {
        &self.elements.dequeue()
    }
}
