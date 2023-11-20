use git2::Repository;
use std::env;
use std::path::{Path, PathBuf};

fn find_git_root() -> Option<PathBuf> {
    let current_dir = env::current_dir().unwrap();
    Repository::discover(current_dir)
        .ok()
        .and_then(|repo| repo.workdir().map(|path| path.to_path_buf()))
}

fn find_todo_files(starting_path: &Path) -> Vec<PathBuf> {
    let mut current_path = starting_path.to_path_buf();
    let mut todo_files = Vec::new();

    loop {
        let todo_path = current_path.join("TODO.md");
        if todo_path.exists() {
            todo_files.push(todo_path);
        }

        if !current_path.pop() { break; }
    }

    todo_files
}


pub fn determine_target_file () -> String {
    let args: Vec<String> = env::args().collect();
    let target_file = if args.len() > 1 && !args[1].is_empty() {
        args[1].clone()
    } else {
        let git_root = match find_git_root() {
            Some(path) => path,
            None => {
                println!("No git repository found!\n- specify a TODO file path\n- move to a git repository folder\n");
                std::process::exit(1);
            }
        };
        let todo_files = find_todo_files(&git_root);

        if todo_files.len() > 0 && todo_files[0].exists() {
            todo_files[0].to_str().expect("TODO file path is not valid UTF-8").to_string()
        } else {
            git_root.join("TODO.md").to_str().unwrap().to_string()
        }
    };

    target_file
}