use std::time::Instant;
use linked_list::LinkedList;

fn main() {
    let sizes = [1_000_000, 5_000_000, 10_000_000];
    for n in sizes {
        println!("n = {}", n);
        println!("  linked list:");

        let mut list = LinkedList::new();
        for k in 0..n {
            list.push_front(k);
        }

        // Measure len
        let timer = Instant::now();
        let len = list.len();
        println!("    len {:?}", timer.elapsed());
        assert_eq!(len, n);

        // Measure push_back
        let timer = Instant::now();
        list.push_back(n);
        println!("    push_back {:?}", timer.elapsed());

        // Measure get middle element.
        let timer = Instant::now();
        let element = list.get(n / 2);
        println!("    get(n/2) {:?}", timer.elapsed());
        unsafe {
            assert_eq!((*element).value, n / 2 - 1);
        }

        // Measure push_front
        let timer = Instant::now();
        list.push_front(n);
        println!("    push_front {:?}", timer.elapsed());

        // Compare to the same operations using vector!
        println!("  vec:");
        let mut vec = Vec::with_capacity(n + 10);
        for i in 0..n {
            vec.push(n - i - 1);
        }

        // Measure len
        let timer = Instant::now();
        let len = vec.len();
        println!("    len {:?}", timer.elapsed());
        assert_eq!(len, n);

        // Measure push_back
        let timer = Instant::now();
        vec.push(n);
        println!("    push_back {:?}", timer.elapsed());

        // Measure get last element.
        let timer = Instant::now();
        let element = vec[n / 2];
        println!("    get(n/2) {:?}", timer.elapsed());
        assert_eq!(element, n / 2 - 1);

        // Measure push_front
        let timer = Instant::now();
        vec.insert(0, n);
        println!("    push_front {:?}", timer.elapsed());
    }
}
