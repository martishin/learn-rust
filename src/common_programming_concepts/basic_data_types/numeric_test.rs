pub fn main() {
    let fahrenheit_degree = 451.0;
    let celsius_degree = (fahrenheit_degree - 32.0) * 0.5556;
    println!("{}°F is {:.1}°C", fahrenheit_degree, celsius_degree);
}
