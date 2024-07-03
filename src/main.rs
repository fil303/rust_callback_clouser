fn main() {
    let number: i32 = 100;
    let _function = calculate(number);
    let number: i32 = _function();
    println!("main : {}", number);
    println!("from callback : {}", get_from_callback(|n| n) );
}

fn calculate(number: i32) -> fn() -> i32 {
    return | | {
        println!("{}", 10 * 2);
        10 * 2
    }
}

fn get_from_callback(callback: fn(n: i32) -> i32) -> i32 {
    return callback(50) * 2;
}

