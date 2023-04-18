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

    impl<T> Iterator for LinearQueue<T> {
        type Item = T;
        
        fn next(&mut self) -> Option<Self::Item> {
            Some(*(*&self.elements.iter().next()).unwrap())
        }
    }

    // impl<T> IntoIterator for LinearQueue<T> {}
}

// TEST!
// use std::collections::LinkedList;

// pub struct LinearQueue<T: Copy> {
//     elements: LinkedList<T>,
// }

// impl<T: Copy> LinearQueue<T: C> {
//     pub fn new() -> Self {
//         Self {
//             elements: LinkedList::new()
//         }
//     }
    
//     pub fn enqueue(&mut self, item: T) {
//         *&self.elements.push_back(item)
//     }
    
//     pub fn dequeue(&mut self) -> T {
//         *&self.elements.pop_front().unwrap()
//     }
    
//     pub fn peek(&self) -> Option<&T> {
//         self.elements.back()
//     }
// }

// impl<T: Copy> Iterator for LinearQueue<T> {
//     type Item = T;
    
//     fn next(&mut self) -> Option<Self::Item> {
//         Some(*(*&self.elements.iter().next()).unwrap())
//     }
// }


// fn main() {
//     let mut v: LinearQueue<usize> = LinearQueue::new();
//     v.enqueue(1);
//     v.enqueue(2);
    
//     for element in v {
//         println!("{}", element)
//     }
// }


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