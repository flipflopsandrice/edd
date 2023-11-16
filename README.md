# EDD

> A Rust CLI Tool for Managing TODO.md Tasks

## Description
EDD is a command-line tool designed for managing tasks in a `TODO.md` file. It allows users to navigate tasks using keyboard controls, mark tasks as complete/incomplete, and insert new tasks easily.

## Features
- **Task Navigation**: Use the up/down arrow keys to navigate through tasks.
- **Complete/Uncomplete Tasks**: Mark tasks as complete or incomplete.
- **Insert Tasks**: Insert a new task before (`i`) or after (`b`) the selected task.
- **User-Friendly Interface**: Clear and intuitive CLI interface.

## Getting Started

### Prerequisites
- Rust and Cargo (latest stable version)
- Git (for cloning the repository)

### Installation
1. Clone the repository:
   ```bash
   git clone https://github.com/flipflopsnrice/edd.git
   ```
2. Navigate to the cloned directory:
   ```bash
   cd edd
   ```
3. Build the project:
   ```bash
   cargo build --release
   ```
4. Installation:
- **Option 1**: Add the binary to your PATH.
   Copy the built binary to the chosen location. Assuming `~/bin/` is in your PATH, use the following command:
   
   ```bash
   # Copy the binary to the ~/bin/ directory:
   cp target/release/edd ~/bin/
   
   # Ensure that the binary is executable:
   chmod +x ~/bin/edd
   ```
- **Option 2**: Run the binary directly.
  ```bash
  # Use cargo to run the binary:
  cargo run

  # or run the binary directly:
  ./target/release/edd
  ```
### Usage
Run the tool from the command line:
```bash
cargo run <optional path to TODO.md file>
```

### Keyboard Controls
- `i`: Insert a new task.
- `e`: Edit the description of the selected task, hit `Enter` to save or `Esc` to cancel.
- `s`: Save changes to the `TODO.md` file.
- `q`: Quit the program.
- `Space`: Complete/Uncomplete selected task.
- `Arrow Up/Down or j/k`: Navigate through tasks.
- `<ctrl> + Arrow Up/Down or j/k`: Move the selected task up/down.
- `TAB`: Make the task a subtask of the previous task.
- `<shift> + TAB`: Make the task a main task.

## Contributing
Contributions are welcome! Please read our [Contributing Guide](CONTRIBUTING.md) for details on our code of conduct and the process for submitting pull requests.

## License
This project is licensed under the [MIT License](LICENSE).

