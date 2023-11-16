use tokio::runtime::Runtime;
use crate::ui::render_tasks;

mod utils;
mod resolver;
mod parser;
mod ui;

struct Task {
    description: String,
    level: usize,
    completed: bool,
}
struct AppState {
    tasks: Vec<Task>, // Assuming Task is a struct representing a task
    selected_index: usize,
}

const LEVEL_INDENTATION: &str = "  ";

async fn run_app() {
    let target_file = resolver::determine_target_file();
    let tasks = parser::str_to_tasks(&target_file).await;

    let mut app_state = AppState {
        tasks: tasks,
        selected_index: 0,
    };

    let should_save = render_tasks(&mut app_state).await;

    if (should_save) {
        let str_tasks = parser::tasks_to_str(&app_state.tasks);
        utils::write_file(&target_file, &str_tasks);
        println!("Saved tasks to file: {}\n", target_file);
    }
}

fn main() {
    Runtime::new().unwrap().block_on(run_app());
}
