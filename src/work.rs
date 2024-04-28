/*
File defining a struct and impl for it.
Explanation:
    name: Stands for task's/work's name
    desc: Stands for task's/work's description
    date_start: Stands for task's/work's start DateTime in seconds
    date_end: Stands for task's/work's end DateTime in seconds
*/

use std::collections::HashSet;
use std::time::Duration;

use chrono::TimeZone;

#[derive(Debug)]
pub enum WorkParams {
    Name(String),
    Desc(String),
    DateStart(i64),
    DateEnd(i64),
}
#[derive(serde::Serialize, serde::Deserialize, Hash, Eq, PartialEq, Debug, Clone)]
pub struct Work {
    pub name: String,
    pub desc: String,
    pub date_start: i64,
    pub date_end: i64,
}

impl Work {
    #[allow(dead_code)]
    pub fn new() -> Work {
        //!Creates default empty work
        Work {
            name: "".to_string(),
            desc: "".to_string(),
            date_start: 0,
            date_end: 0,
        }
    }
    pub fn from(name: String, desc: String, date_start: i64, date_end: i64) -> Work {
        //!Creates work with specified params
        Work {
            name,
            desc,
            date_start,
            date_end,
        }
    }

    #[allow(dead_code)]
    pub fn from_hashset(hashset: HashSet<Work>) -> Vec<Work> {
        //!O(n); HashSet< Work > to Vec< Work >
        let mut vec: Vec<Work> = Vec::new();
        for work in hashset {
            vec.push(work);
        }
        vec
    }

    #[allow(dead_code)]
    pub fn vec_to_hashset_work(vec: Vec<Work>) -> HashSet<Work> {
        vec.into_iter().collect()
    }

    #[allow(dead_code)]
    pub fn filter_duplicates(vec: Vec<Work>) -> Vec<Work> {
        //! Filters duplicates using HashSet [1,2,2,3] to [1,2,3]
        Self::from_hashset(Self::vec_to_hashset_work(vec))
    }

    pub fn remove_duplicates(old_vec: Vec<Work>, mut new_vec: Vec<Work>) -> Vec<Work> {
        //! Deletes duplicates using retain [1,2,2,3] to [1,3]
        new_vec.retain(|work| !old_vec.contains(work));
        new_vec
    }
    pub fn from_vec(vec: Vec<WorkParams>) -> Option<Work> {
        //!Parse from vector of works aka Vec< Work >
        //!Iteration through all items in vector
        let mut name = String::new();
        let mut desc = String::new();
        let mut date_start = 0;
        let mut date_end = 0;
        for i in vec {
            match i {
                WorkParams::Name(s) => {
                    name = s;
                }
                WorkParams::Desc(s) => {
                    desc = s;
                }
                WorkParams::DateStart(i) => {
                    date_start = i;
                }
                WorkParams::DateEnd(i) => {
                    date_end = i;
                }
            }
        }
        if name.is_empty() || desc.is_empty() || date_start == 0 || date_end == 0 {
            None
        } else {
            Some(Work {
                name,
                desc,
                date_start,
                date_end,
            })
        }
    }
    pub fn from_vec_string(string: String) -> Vec<Work> {
        //!Parse from String Vec
        serde_json::from_str(&string)
            .map_err(|e| println!("Failed to convert into work: {e}"))
            .expect("")
    }

    #[allow(dead_code)]
    pub fn to_json_string(&self) -> String {
        //!Converts work to json, return as String

        serde_json::to_string(self).expect("Failed to convert into work")
    }

    pub fn edit(&mut self, param: WorkParams) {
        //!Edits specified param
        match param {
            WorkParams::Name(new_val) => {
                self.name = new_val;
            }
            WorkParams::Desc(new_val) => {
                self.desc = new_val;
            }
            WorkParams::DateStart(new_val) => {
                self.date_start = new_val;
            }
            WorkParams::DateEnd(new_val) => {
                self.date_end = new_val;
            }
        }

        println!("\n{self}")
    }

    pub fn get_time_progress(&self) -> u8 {
        //!Returns the percent of time passed
        if chrono::Utc::now().timestamp() <= self.date_start {
            0
        } else if chrono::Utc::now().timestamp() >= self.date_end {
            100
        } else {
            return (((chrono::Utc::now().timestamp() - self.date_start) as f64
                / (self.date_end - self.date_start) as f64)
                * 100f64) as u8;
        }
    }

    pub fn get_time_progress_graph(&self) -> String {
        let mut str = String::from("[");
        str.push_str("I".repeat((self.get_time_progress() / 2) as usize).as_str());
        str.push_str(
            " ".repeat((50 - self.get_time_progress() / 2) as usize)
                .as_str(),
        );
        str.push(']');
        str
    }
}

impl std::fmt::Display for Work {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Name: {}\nDesc: {}\nStart: {}\nDuration: {}\nEnd: {}\nProgress by time passed: {}%\n{}\n", self.name, self.desc, chrono::Local.timestamp_opt(self.date_start, 0).unwrap(), humantime::format_duration(Duration::from_secs((self.date_end - self.date_start).unsigned_abs())), chrono::Local.timestamp_opt(self.date_end, 0).unwrap(), self.get_time_progress(), self.get_time_progress_graph())
    }
}

impl std::str::FromStr for Work {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        serde_json::from_str(s)
    }
}
