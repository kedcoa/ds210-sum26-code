use std::ptr::null_mut;
use malloc::MALLOC;

pub struct LinkedListNode<T> {
    pub value: T,
    pub next: *mut LinkedListNode<T>
}
impl<T> LinkedListNode<T> {
    pub fn new(value: T, next: *mut LinkedListNode<T>) -> LinkedListNode<T> {
        return LinkedListNode {
            value: value,
            next: next,
        };
    }
}

pub struct LinkedList<T> {
    pub head: *mut LinkedListNode<T>
}
impl<T> LinkedList<T> {
    pub fn new() -> LinkedList<T> {
        return Self { head: null_mut() };
    }
    pub fn push_front(&mut self, value: T) {
        unsafe {
            let new_node = MALLOC.malloc(size_of::<LinkedListNode<T>>()) as *mut LinkedListNode<T>;
            std::ptr::write(new_node, LinkedListNode::new(value, self.head));
            self.head = new_node;
        }
    }
    pub fn len(&self) -> usize {
        let mut len = 0;
        let mut ptr = self.head;
        while !ptr.is_null() {
            len += 1;
            unsafe {
                ptr = (*ptr).next;
            }
        }
        return len;
    }
    pub fn get(&self, i: usize) -> *mut LinkedListNode<T> {
        todo!("write code to get the node at index i! What if i > self.len()?")
    }
    pub fn push_back(&mut self, value: T) {
        todo!("write code to push_back T to the end of the list!")
    }
}