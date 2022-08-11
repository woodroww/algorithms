// these functions are O(log n)
// binary search is also O(log n)
// it keeps splitting array in half for each iteration
// like the divide by 2 in the log fn

fn log_recursive(x: u32) -> u32 {
    fn log_func(x: u32, max_level: &mut u32) {
        let n = (x as f32 / 2.0).floor() as u32;
        if n > 0 {
            *max_level += 1;
            log_func(n, max_level)
        }
    }
    let mut log = 0;
    log_func(x, &mut log);
    log
}

fn logn(x: u32) -> u32 {
    let mut n = x;
    let mut count = 0;
    while n > 1 {
        n = (n as f32 / 2.0).floor() as u32;
        count += 1;
    }
    count
}

fn main() {
    // on a calculator log2 n = ln(num) / ln(2)

    let n = 8;
    println!("log {} = {}", n, logn(n));
    println!("log {} = {}", n, log_recursive(n));
}
