fn main() {
    let _a = 0;
    {
        let _b = 1;
        let _c = 2;
        {
            let _d = 3;
            let _e = 4;
            println!("de")
        }
        let _f = 5;
        {
            let _g = 6;
            println!("g")
        }
        let _h = 7;
        println!("bcfh")
    }
    let _i = 8;
    println!("ai")
}
