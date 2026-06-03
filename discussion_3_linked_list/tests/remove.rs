use linked_list::LinkedList;
use malloc::MALLOC;

#[test]
fn test_remove_head() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    l.remove_head();
    assert_eq!(l.len(), 2);
    unsafe {
        assert_eq!((*l.get(0)).value, 2);
        assert_eq!((*l.get(1)).value, 1);
    }
}

#[test]
fn test_remove_head_single() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.remove_head();
    assert_eq!(l.len(), 0);
}

#[test]
#[should_panic]
fn test_remove_head_empty() {
    MALLOC.clear();
    let mut l: LinkedList<i32> = LinkedList::new();
    l.remove_head();
}

#[test]
fn test_remove_first() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    l.remove(0);
    assert_eq!(l.len(), 2);
    unsafe {
        assert_eq!((*l.get(0)).value, 2);
        assert_eq!((*l.get(1)).value, 1);
    }
}

#[test]
fn test_remove_middle() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    l.remove(1);
    assert_eq!(l.len(), 2);
    unsafe {
        assert_eq!((*l.get(0)).value, 3);
        assert_eq!((*l.get(1)).value, 1);
    }
}

#[test]
fn test_remove_last() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    l.remove(2);
    assert_eq!(l.len(), 2);
    unsafe {
        assert_eq!((*l.get(0)).value, 3);
        assert_eq!((*l.get(1)).value, 2);
    }
}

#[test]
#[should_panic]
fn test_remove_out_of_bounds() {
    MALLOC.clear();
    let mut l = LinkedList::new();
    l.push_front(1);
    l.push_front(2);
    l.push_front(3);
    l.remove(3);
}
