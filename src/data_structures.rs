/// Collating all the code for a linear queue together.
pub mod linearqueue {
    
    use std::collections::LinkedList;

    /// Using a LinkedList to define the elements because it has associated methods that are more useful than the methods for vectors.
    #[derive(Debug, Clone)]
    pub struct LinearQueue<T> {
        elements: LinkedList<T>,
    }

    /// Standard operations on a queue such as enqueue, dequeue, and peek. Note that there is no `is_full()` function because the queue's possible size is unknown and can vary greatly.
    impl<T> LinearQueue<T> {
        pub fn new() -> Self {
            Self {
                elements: LinkedList::new(),
            }
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
    }

    /// Meta-programming allowing for the iteration through a linear queue.
    impl<T> Iterator for LinearQueue<T> {
        type Item = T;

        fn next(&mut self) -> Option<Self::Item> {
            self.dequeue()
        }
    }
}

// Similar definition for stacks but with a pointer.
pub mod Stack {
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
