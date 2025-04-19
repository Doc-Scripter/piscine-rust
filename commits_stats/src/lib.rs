/* commits_stats
Instructions:
In this exercise, you will be provided with a json file commits.json(download) with data corresponding to git commits in GitHub (extracted using the GitHub rest API). 
Your objective is to extract the relevant data and place it in a struct called CommitData.

Create two functions:

commits_per_author: which returns a hash map with the number of commits per author. The auditors will be identified by their GitHub login.
commits_per_week: which returns a hash map with the number of commits per week.
A week is represented by the year followed by the number of the week. For example, January 1, 2020 is in week 1 of 2020 and will be represented by a String with the form "2020-W1".

Expected functions
pub fn commits_per_week(data: &json::JsonValue) -> HashMap<String, u32> {
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
}
Usage
Here is a possible test for your function:

use commits_stats::{commits_per_week, commits_per_author};

fn main() {
	let contents = fs::read_to_string("commits.json").unwrap();
	let serialized = json::parse(&contents).unwrap();
	println!("{:?}", commits_per_week(&serialized));
	println!("{:?}", commits_per_author(&serialized));
}
And its output:

$ cargo run
{"2020-W44": 5, "2020-W36": 1, "2020-W31": 1, ... ,"2020-W45": 4, "2020-W46": 4}
{"homembaixinho": 2, "mwenzkowski": 3, ... ,"tamirzb": 1, "paul-ri": 2, "RPigott": 1}
$ */

use std::collections::HashMap;
use std::fs;
use json::JsonValue;


pub fn commits_per_week(data: &JsonValue) -> HashMap<String, u32> {
    let mut week_commits = HashMap::new();

    if let JsonValue::Array(commits) = data {
        for commit in commits.iter() {
            if let Some(date_str) = commit["commit"]["author"]["date"].as_str() {
                if let Some(year_week) = get_year_week(date_str) {
                    *week_commits.entry(year_week).or_insert(0) += 1;
                }
            }
        }
    }

    week_commits
}

pub fn commits_per_author(data: &json::JsonValue) -> HashMap<String, u32> {
    let mut commits_per_author: HashMap<String, u32> = HashMap::new();
    for commit in data.members() {
        let author = commit["author"]["login"].as_str().unwrap();
        let count = commits_per_author.entry(author.to_string()).or_insert(0);
        *count += 1;
    }
    commits_per_author
}


fn get_year_week(date_str: &str) -> Option<String> {
    if date_str.len() < 10 {
        return None;
    }

    let year: String = date_str[0..4].to_string();
    let month: String = date_str[5..7].to_string();
    let day: String = date_str[8..10].to_string();

    if let (Ok(year_num), Ok(month_num), Ok(day_num)) = (
        year.parse::<i32>(),
        month.parse::<u32>(),
        day.parse::<u32>(),
    ) {
       
    let day_of_year = day_of_year(year_num, month_num, day_num);   

    // Calculate ISO week number
    let mut week_number = ((day_of_year + 8) / 7) as u32;

    // Handle weeks that cross the year boundary
    if week_number == 0 {
        week_number = 52;
    } else if week_number == 53 {
        week_number = 1;
    }
        let year_week = format!("{:04}-W{:02}", year_num, week_number);
        println!("year:{}-month:{}-day:{} -> year:{}-week:{}", year, month, day, year, week_number);  

        Some(year_week)
    } else {
        None
    }
}

fn day_of_year(year: i32, month: u32, day: u32) -> u32 {
    let mut day_of_year = day;
    let month_lengths: [u32; 12] = [31, 28 + leap_year_adjust(year), 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
    for i in 1..(month as usize) {
        day_of_year += month_lengths[i - 1];
    }

    day_of_year
}

fn leap_year_adjust(year: i32) -> u32 {
    if year % 4 == 0 && (year % 100 != 0 || year % 400 == 0) {
        1
    } else {
        0
    }
}