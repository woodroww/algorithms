fn main() {
    // Ok so rust vec if you use push and pop is a LIFO stack
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
