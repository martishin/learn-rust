pub fn main() {
    println!("{}", bigger(3, 4));
}

fn bigger(a: i32, b: i32) -> i32 {
    if a > b {
        a
    } else {
        b
    }
}
