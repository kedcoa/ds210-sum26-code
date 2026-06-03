use malloc::MALLOC;
use super::simple_linked_list::LinkedList;

impl<T> LinkedList<T> {
    pub fn remove_head(&mut self) {
        if self.head.is_null() {
            panic!("Index out of bounds");
        }
        unsafe {
            let old_head = self.head;
            self.head = (*old_head).next;
            MALLOC.free(old_head as *mut u8);
        }
    }

    pub fn remove(&mut self, i: usize) {
        todo!("write code to remove the element at index i. What if i > len()? What if i == 0? What if it is somewhere in the middle?")
    }
}
