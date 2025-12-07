fn celsius_to_fahrenheit(temperature: f64) -> f64 {
    (temperature * 9.0 / 5.0) + 32.0
}

fn fahrenheit_to_celsius(temperature: f64) -> f64 {
    (temperature - 32.0) * 5.0 / 9.0
}

fn main() {
    let celsius = 42.0;
    let fahrenheit_result = celsius_to_fahrenheit(celsius);
    println!("{celsius}°C = {:.2}°F", fahrenheit_result);

    let fahrenheit = 97.4;
    let celsius_result = fahrenheit_to_celsius(fahrenheit);
    println!("{fahrenheit}°F = {:.2}°C", celsius_result);
}
