use linked_list::LinkedList;
use tracker::Tracker;

fn main() {
    let mut tracker = Tracker::new();
    let mut list = LinkedList::new();
    // add some elements.
    list.push_front(tracker.track(String::from("first")));
    list.push_front(tracker.track(String::from("second")));
    list.push_front(tracker.track(String::from("third")));

    // remove the elements.
    list.remove(1);
    list.remove(1);
    list.remove_head();

    assert_eq!(list.len(), 0);
    println!("All removes successful");
    tracker.print_status();
}
