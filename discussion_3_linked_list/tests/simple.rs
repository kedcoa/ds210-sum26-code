use linked_list::LinkedList;
use malloc::MALLOC;

#[test]
fn test_len() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    assert_eq!(l.len(), 3);
}

#[test]
fn test_get() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    unsafe {
        assert_eq!((*l.get(0)).value, 3);
        assert_eq!((*l.get(1)).value, 2);
        assert_eq!((*l.get(2)).value, 1);
    }
}

#[test]
#[should_panic]
fn test_get_out_of_bounds() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    l.get(3);
}

#[test]
fn test_push_back() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_back(1);
    l.push_back(2);
    l.push_back(3);
    unsafe {
        assert_eq!((*l.get(0)).value, 1);
        assert_eq!((*l.get(1)).value, 2);
        assert_eq!((*l.get(2)).value, 3);
    }
}
