use crate::utils::config::Config;

use regex::Regex;

pub fn is_problem_id(problem: &str) -> bool {
    /* Matches any alphanumeric character or period - so that it can match
    problem IDs like 'itu.seatallocation' or 'twosum' */
    Regex::new(r"^[a-zA-Z0-9\.]+$").unwrap().is_match(problem)
}

pub fn get_problem_url(problem: &str) -> String {
    let host_name = Config::new().unwrap().kattisrc.kattis.hostname;
    format!("https://{}/problems/{}", host_name, problem)
}
