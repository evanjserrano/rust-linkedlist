use crate::linkedlist::LinkedList;
use std::io::{self, Write};

pub mod linkedlist;

fn main() {
    let mut l: LinkedList = LinkedList::new();

    println!("Basic Linked List!");
    let mut input: i32;
    loop {
        print!("\t1. push\n\t2. pop\n\t3. exit\ninput: ");
        io::stdout().flush().unwrap();
        input = read_int().unwrap_or(3);

        match &input {
           1 => {
            print!("Enter value to push: ");
            io::stdout().flush().unwrap();
            match read_int() {
                Some(val) => l.push_back(val),
                None => break,
            }
           },
           2 => drop(l.pop_back()),
           3 => break,
           _ => {}
        }
        println!("\nLinkedList: {{ {} }}\n", l);
    }
}

fn read_int() -> Option<i32> {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("stdin err");
    str.trim().parse().ok()
}