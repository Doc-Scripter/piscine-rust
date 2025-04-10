use chrono::{Datelike, NaiveDate, Weekday};

/// Returns the weekday of the middle day of the given year.
/// 
/// If the year has an even number of days (like leap years),
/// it returns None since there is no single middle day.
/// 
/// # Arguments
/// 
/// * `year` - The year to find the middle day for
/// 
/// # Returns
/// 
/// * `Option<Weekday>` - The weekday of the middle day if it exists, None otherwise
pub fn middle_day(year: i32) -> Option<Weekday> {
    // Get the first day of the year
    let first_day = NaiveDate::from_ymd_opt(year, 1, 1).unwrap();
    
    // Get the last day of the year
    let last_day = NaiveDate::from_ymd_opt(year, 12, 31).unwrap();
    
    // Calculate the total number of days in the year
    let days_in_year = last_day.ordinal() as i32;
    
    // Check if the year has an even number of days
    if days_in_year % 2 == 0 {
        // Years with even number of days don't have a middle day
        return None;
    }
    
    // Calculate the middle day (days_in_year / 2 + 1)
    // Integer division gives us the floor, so we add 1 to get the middle
    let middle_day_ordinal = days_in_year / 2 + 1;
    
    // Create a date for the middle day
    let middle_date = NaiveDate::from_yo_opt(year, middle_day_ordinal as u32).unwrap();
    
    // Return the weekday of the middle day
    Some(middle_date.weekday())
}

#[cfg(test)]
mod tests {
    use super::*;
    use chrono::Weekday;
    
    #[test]
    fn test_middle_day_non_leap_year() {
        // 2023 is not a leap year, so it has 365 days
        // Middle day is day 183 (July 2, 2023) which is a Sunday
        assert_eq!(middle_day(2023), Some(Weekday::Sun));
    }
    
    #[test]
    fn test_middle_day_leap_year() {
        // 2024 is a leap year, so it has 366 days (even)
        // Should return None
        assert_eq!(middle_day(2024), None);
    }
    
    #[test]
    fn test_middle_day_another_non_leap_year() {
        // 2021 is not a leap year, so it has 365 days
        // Middle day is day 183 (July 2, 2021) which is a Friday
        assert_eq!(middle_day(2021), Some(Weekday::Fri));
    }
}
