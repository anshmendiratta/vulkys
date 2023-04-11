pub mod LinearQueue;
use crate::rigidbodyobjects::motion::collisions::definition::Collision;
use std::collections::LinkedList;

mod LinearQueue { 
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

        pub fn dequeue(&self, item: Collision) -> Result<Collision> {
            &self.pop().unwrap()
        }

        pub fn peek(&self) -> Collision {
            &self.last()
        }
    }
}

use crate::rigidbodyobjects::motion::collisions::datastructures::linearqueue::LinearQueue;
use crate::rigidbodyobjects::rigidbodies::RigidBody;

#[derive(Debug)]
pub struct Stack {
    elements: LinearQueue<RigidBody>,
    pointer: u32,
}

mod Stack {
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
