use regex::Regex;
use crate::utils;
use crate::constants::LEVEL_INDENTATION;
use crate::types::Task;

fn parse_line_to_task(line: &str) -> Option<Task> {
    let regex = Regex::new(r"^(  *|)(?:- )\[([ |x|X])\](?: )(.*)$").unwrap();

    if let Some(caps) = regex.captures(line) {
        let level = caps.get(1).map_or(0, |m| m.as_str().len() / LEVEL_INDENTATION.len());
        let completed = caps.get(2).map_or(false, |m| m.as_str().trim().eq_ignore_ascii_case("x"));
        let description = caps.get(3).map_or("", |m| m.as_str());

        Some(Task {
            description: description.to_string(),
            level: level,
            completed: completed,
        })
    } else {
        None
    }
}


pub(crate) async fn file_to_tasks(target_file: &str) -> Vec<Task> {
    match utils::read_file(&target_file) {
        Ok(lines) => {
            let mut tasks: Vec<Task> = Vec::new();

            for line in lines {
                match parse_line_to_task(&line) {
                    Some(task) => tasks.push(task),
                    None => continue,
                }
            }
            tasks
        },
        Err(_) => {
            Vec::new()
        }
    }
}

pub(crate) fn tasks_to_str(tasks: &Vec<Task>) -> String {
    let mut output = String::new();

    for task in tasks.iter() {
        output.push_str(&format!("{}- [{}] {}\n", LEVEL_INDENTATION.repeat(task.level), if task.completed { "x" } else { " " }, task.description));
    }

    output
}