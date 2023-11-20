pub struct Task {
    pub description: String,
    pub level: usize,
    pub completed: bool,
}

pub struct AppState {
    pub tasks: Vec<Task>,
    pub selected_index: usize,
    pub changed: bool,
    pub offsetY: usize,
}
