//! How far the wagon travels in a day.

#[derive(Clone, Copy)]
pub enum Pace {
    Slow,
    Steady,
    Grueling,
}

#[derive(Clone, Copy)]
pub enum Weather {
    Clear,
    Rain,
    Snow,
}

/// Miles covered in one day.
///
/// The base distance depends on the pace: Slow 8, Steady 12, Grueling 18.
/// Rain costs 2 miles. Snow cuts the distance in half.
///
/// ```
/// use dw21::travel::{miles_per_day, Pace, Weather};
///
/// assert_eq!(miles_per_day(Pace::Steady, Weather::Clear), 12);
/// assert_eq!(miles_per_day(Pace::Steady, Weather::Rain), 10);
/// assert_eq!(miles_per_day(Pace::Steady, Weather::Snow), 6);
/// ```
///
/// ```
/// use dw21::travel::{miles_per_day, Pace, Weather};
///
/// assert_eq!(miles_per_day(Pace::Slow, Weather::Clear), 8);
/// assert_eq!(miles_per_day(Pace::Grueling, Weather::Clear), 18);
/// ```
pub fn miles_per_day(pace: Pace, weather: Weather) -> u32 {
    let base = match pace {
        Pace::Slow => 8,
        Pace::Steady => 12,
        Pace::Grueling => 16,
    };
    match weather {
        _ => base,
        Weather::Snow => base / 2,
        Weather::Rain => base - 2,
    }
}
