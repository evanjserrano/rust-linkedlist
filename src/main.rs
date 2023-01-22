use crate::linkedlist::LinkedList;
use std::io::{self, Write};

pub mod linkedlist;

fn main() {
    let mut l: LinkedList = LinkedList::new();

    println!("Basic Linked List!");
    let mut input: i32;
    loop {
        print!("0. exit        \n\
                1. push_back   \n\
                2. pop_back    \n\
                3. push_front  \n\
                4. pop_front   \n\
                :    ==> ");
        io::stdout().flush().unwrap();
        input = read_int().unwrap_or(0);

        match &input {
           0 => break,
           1 => {
            print!("Enter value to push: ");
            io::stdout().flush().unwrap();
            match read_int() {
                Some(val) => l.push_back(val),
                None => continue,
            }
           },
           2 => drop(l.pop_back()),
           3 => {
            print!("Enter value to push: ");
            io::stdout().flush().unwrap();
            match read_int() {
                Some(val) => l.push_front(val),
                None => continue,
            }
           },
           4 => drop(l.pop_front()),
           _ => {}
        }
        println!("\n  {{ {} }}\n", l);
    }
}

fn read_int() -> Option<i32> {
    let mut str = String::new();
    io::stdin().read_line(&mut str).expect("stdin err");
    str.trim().parse().ok()
}