use crate::linkedlist::LinkedList;

pub mod linkedlist;

fn main() {
    let mut l: LinkedList = LinkedList::new();

    println!("LinkedList: {{ {} }}", l);
    l.push(3);
    println!("LinkedList: {{ {} }}", l);
    l.push(4);
    println!("LinkedList: {{ {} }}", l);
    l.push(5);
    println!("LinkedList: {{ {} }}", l);
    l.push(6);
    println!("LinkedList: {{ {} }}", l);

}
