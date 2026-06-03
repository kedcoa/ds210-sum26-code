use crate::LinkedList;

impl<T> Drop for LinkedList<T> {
    fn drop(&mut self) {
        // todo: make sure the list does not leak any data!
    }
}