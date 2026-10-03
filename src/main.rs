use dw21::river::{crossing_advice, food_status};
use dw21::travel::{miles_per_day, Pace, Weather};

fn main() {
    println!("Miles today: {}", miles_per_day(Pace::Steady, Weather::Rain));
    println!("Crossing: {}", crossing_advice(5));
    println!("Food: {}", food_status(Some(20)));
}
