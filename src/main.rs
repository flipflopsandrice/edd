use tokio::runtime::Runtime;
use types::AppState;
use crate::ui::render_tasks;

mod utils;
mod resolver;
mod parser;
mod ui;
mod constants;
mod types;

async fn run_app() {
    let target_file = resolver::determine_target_file();
    let tasks = parser::file_to_tasks(&target_file).await;

    let mut app_state = AppState {
        tasks: tasks,
        selected_index: 0,
        changed: false,
    };

    let should_save = render_tasks(&mut app_state).await;

    if should_save {
        let str_tasks = parser::tasks_to_str(&app_state.tasks);
        utils::write_file(&target_file, &str_tasks);
        println!("Saved tasks to file: {}\n", target_file);
    }
}

fn main() {
    Runtime::new().unwrap().block_on(run_app());
}
