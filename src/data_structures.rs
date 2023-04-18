use crate::rigidbodies::*;
use crate::type_traits::*;
// use crate::rigidbodyobjects::motion::collisions::datastructures::linearqueue::LinearQueue;

pub mod linearqueue { 
    use crate::collision_definition::Collision;
    use std::collections::LinkedList;
    

    pub struct LinearQueue<T> {
        elements: LinkedList<T>,
    }
    
    impl<T> LinearQueue<T> {
        pub fn new() -> Self {
            Self {
                elements: LinkedList::new()
            }
        }
        
        pub fn enqueue(&mut self, item: T) {
            *&self.elements.push_back(item)
        }
        
        pub fn dequeue(&mut self, item: T) -> T {
            *&self.elements.pop_front().unwrap()
        }
        
        pub fn peek(&self) -> &Option<&T> {
            &self.elements.back()
        }
    }
}


pub mod Stack {
    use crate::{collision_definition::Collision, rigidbodies::RigidBody};

    #[derive(Debug)]
    pub struct Stack<T> {
        elements: Vec<T>,
        pointer: usize,
    }

    impl<T> Stack<T> {
        pub fn new() -> Self {
            Self {
                elements: Vec::new(),
                pointer: 0,
            }
        }

        pub fn push(&mut self, item: T) {
            self.pointer += 1;
            self.elements.push(item)
        }

        pub fn pop(&mut self) -> T {
            self.elements.remove(self.pointer)
        }
    }
}