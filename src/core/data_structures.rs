pub mod linearqueue {
    use std::collections::LinkedList;

    #[derive(Debug, Clone, PartialEq)]
    pub struct LinearQueue<T> {
        elements: LinkedList<T>,
    }

    impl<T> LinearQueue<T> {
        pub fn new() -> Self {
            Self {
                elements: LinkedList::new(),
            }
        }
        pub fn from(elements: Vec<T>) -> Self {
            let mut llist: LinkedList<T> = LinkedList::new();
            for item in elements {
                llist.push_back(item)
            }

            Self { elements: llist }
        }
        pub fn enqueue(&mut self, item: T) {
            self.elements.push_back(item)
        }
        pub fn dequeue(&mut self) -> Option<T> {
            self.elements.pop_front()
        }
        pub fn peek(&self) -> Option<&T> {
            self.elements.back()
        }
        pub fn len<J>(&self) -> usize
        where
            J: crate::physics::rigidbodies::Updateable,
        {
            self.elements.len()
        }
    }

    impl<T> Iterator for LinearQueue<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.dequeue()
        }
    }
}

pub mod stack {
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

#[cfg(test)]
mod tests {
    use super::{linearqueue::LinearQueue, *};

    #[test]
    fn check_queue_add() {
        let mut l: LinearQueue<usize> = linearqueue::LinearQueue::new();
        l.enqueue(2 as usize);

        assert_eq!(l, linearqueue::LinearQueue::from(vec![2 as usize]))
    }
}
