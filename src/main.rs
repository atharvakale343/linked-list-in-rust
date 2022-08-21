mod list;

use list::LinkedList;

fn main() {
    println!("\n----------------------------------");
    println!("CREATED EMPTY DOUBLY-LINKED LIST");
    let mut list: LinkedList<i32> = LinkedList::new();
    println!("{:#?}", list);
    println!("----------------------------------");

    list.insert_first(12);
    println!("INSERTED HEAD: {}", 12);
    list.insert_first(10);
    println!("INSERTED HEAD: {}", 10);
    println!("{:#?}", list);
    println!("----------------------------------");

    let n: i32 = list.pop_first();
    println!("HEAD POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");

    let n: i32 = list.pop_first();
    println!("HEAD POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");

    list.insert_last(20);
    println!("INSERTED TAIL: {}", 20);
    list.insert_last(25);
    println!("INSERTED TAIL: {}", 25);
    println!("{:#?}", list);
    println!("----------------------------------");

    let n: i32 = list.pop_last();
    println!("TAIL POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");

    let n: i32 = list.pop_last();
    println!("TAIL POP: {}", n);
    println!("{:#?}", list);
    println!("----------------------------------");
}
