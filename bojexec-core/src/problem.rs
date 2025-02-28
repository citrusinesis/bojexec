use serde::{Deserialize, Serialize};
use std::time::Duration;

#[derive(Debug, Serialize)]
pub struct Description {
    pub problem: String,
    pub input: String,
    pub output: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TestCase {
    pub input: String,
    pub output: String,
}

#[derive(Debug)]
pub struct Metadata {
    pub timeout: Option<Duration>,
    pub memory: Option<String>,
}

#[derive(Debug, Serialize)]
pub struct Problem {
    pub id: u32,
    pub title: String,
    pub description: Description,
    pub testcases: Vec<TestCase>,
}
