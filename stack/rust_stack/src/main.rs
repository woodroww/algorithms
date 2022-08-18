use std::collections::VecDeque;

fn queue() {

    let mut queue: VecDeque<i32> = VecDeque::new();
    queue.push_back(1);
    queue.push_back(2);
    queue.push_back(3);

    for i in &queue {
        print!("{} ", i);
    }
    println!();
    while let Some(i) = queue.pop_front() {
        print!("{} ", i);
    }
    println!();
}

fn stack() {
// Rust Vec is a LIFO stack if you use push and pop 
    let mut stack = Vec::new();
    stack.push(1);
    stack.push(2);
    stack.push(3);
    stack.push(4);
    stack.pop();
    stack.push(5);
    stack.pop();
    println!("{:?}", stack);
}


fn main() {
    println!("a stack");
    stack();
    println!("a queue");
    queue();
}
