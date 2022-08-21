mod list;

use list::LinkedList;

fn main() {
    println!("\n----------------------------------");
    println!("CREATED EMPTY DOUBLY-LINKED LIST");
    let mut list: LinkedList<i32> = LinkedList::new();
    println!("{:#?}", list);
    println!("----------------------------------");

    list.push_front(12);
    println!("INSERTED HEAD: {}", 12);
    list.push_front(10);
    println!("INSERTED HEAD: {}", 10);
    println!("{:#?}", list);
    println!("----------------------------------");
    
    let len = list.len();
    println!("LEN: {}", len);
    println!("{:#?}", list);
    println!("----------------------------------");

    let n: i32 = list.pop_front();
    println!("HEAD POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");

    let n: i32 = list.pop_front();
    println!("HEAD POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");

    list.push_back(20);
    println!("INSERTED TAIL: {}", 20);
    list.push_back(25);
    println!("INSERTED TAIL: {}", 25);
    println!("{:#?}", list);
    println!("----------------------------------");
    
    let n: i32 = list.pop_back();
    println!("TAIL POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");
    
    let n: i32 = list.pop_back();
    println!("TAIL POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");
    
    list.push_back(20);
    println!("INSERTED TAIL: {}", 20);
    list.push_back(25);
    println!("INSERTED TAIL: {}", 25);
    println!("{:#?}", list);
    println!("----------------------------------");

    list.clear();
    println!("CLEARED LIST:");
    println!("{:#?}", list);
    println!("----------------------------------");
}
