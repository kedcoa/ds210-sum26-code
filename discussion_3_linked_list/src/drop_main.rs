use linked_list::LinkedList;
use tracker::Tracker;

fn main() {
    let mut tracker = Tracker::new();
    {
        let mut list = LinkedList::new();
        // add some elements.
        list.push_front(tracker.track(String::from("first")));
        list.push_front(tracker.track(String::from("second")));
        list.push_front(tracker.track(String::from("third")));
    } // list goes out of scope here, drop() is called.
    tracker.print_status();
}
