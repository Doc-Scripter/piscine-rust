pub use chrono::{Datelike, NaiveDate, Weekday};

/// Returns the weekday of the middle day of the given year.
///
/// The middle day is only defined for years with an odd number of days.
/// For years with an even number of days, returns `None`.
///
/// # Arguments
///
/// * `year` - The year to find the middle day of
///
/// # Returns
///
/// * `Some(Weekday)` - The weekday of the middle day
/// * `None` - If the year has an even number of days
pub fn middle_day(year: i32) -> Option<Weekday> {
    // Determine if the year is a leap year
    let is_leap_year = NaiveDate::from_ymd_opt(year, 2, 29).is_some();
    
    // Calculate total days in the year
    let total_days = if is_leap_year { 366 } else { 365 };
    
    // Only years with odd number of days have a middle day
    if total_days % 2 == 0 {
        return None;
    }
    
    // Calculate the middle day (integer division)
    let middle_day_number = (total_days / 2) + 1;
    
    // Create a date for the middle day and get its weekday
    if let Some(date) = NaiveDate::from_yo_opt(year, middle_day_number as u32) {
        Some(date.weekday())
    } else {
        None // This shouldn't happen with valid inputs
    }
}
