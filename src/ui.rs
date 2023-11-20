use crossterm::style::{Attribute, Print, SetBackgroundColor, style, Stylize};
use std::io::{stdout, Stdout, Write};
use std::time::Duration;
use crossterm::{event, execute, terminal};
use crossterm::cursor::{MoveTo, MoveToNextLine};
use crossterm::event::{Event, KeyCode, KeyEvent};
use crossterm::style::{Color, ResetColor, SetForegroundColor};
use crossterm::terminal::size;
use crate::constants::LEVEL_INDENTATION;
use crate::types::{AppState, Task};


fn string_length_to_u16(s: &str) -> Option<u16> {
    let length = s.len();
    if length <= u16::MAX as usize {
        Some(length as u16)
    } else {
        None
    }
}

fn draw_tasks(stdout: &mut Stdout, state: &AppState) {
    let mut idx = 0;
    for (index, task) in state.tasks.iter().
        enumerate().filter(
        |(index, task)| {
            index >= &state.offsetY && index < &(state.offsetY + size().unwrap().1 as usize - 1)
        }
    ) {
        let color = TaskColor {
            foreground: if index == state.selected_index { Some(Color::Green) } else { None },
            background: if index == state.selected_index { Some(Color::DarkGrey) } else { None },
        };
        draw_task(stdout, &color, idx, task);
        execute!(stdout, MoveToNextLine(1)).ok();
        idx+=1;
    }
}

struct TaskColor {
    foreground: Option<Color>,
    background: Option<Color>,
}

fn draw_task(stdout: &mut Stdout, color: &TaskColor, index: usize, task: &Task) {
    execute!(
        stdout,
        MoveTo(0, index as u16),
        terminal::Clear(terminal::ClearType::CurrentLine)
    ).ok();
    execute!(stdout,
            Print(&format!("[{}] {}", if task.completed { "x" } else { " " }, LEVEL_INDENTATION.repeat(task.level))),
            ).ok();

    if let Some(color) = color.foreground {
        execute!(stdout,SetForegroundColor(color)).ok();
    }
    if let Some(color) = color.background {
        execute!(stdout,SetBackgroundColor(color)).ok();
    }

    let mut description = style(task.description.to_string());
    if task.completed {
        description = style(task.description.to_string()).attribute(Attribute::Dim);
    }

    execute!(stdout,
            Print(&format!("{}", description)),
            ResetColor,
        ).ok();

    execute!(stdout, ResetColor).ok();
}


fn draw_help(stdout: &mut Stdout) {
    let (width, height) = size().unwrap();
    execute!(stdout, SetBackgroundColor(Color::Grey)).ok();
    execute!(stdout, SetForegroundColor(Color::White)).ok();
    execute!(
        stdout,
        MoveTo(0, height - 1)
    ).ok();
    execute!(stdout, Print(" ".repeat(width as usize))).ok();
    execute!(
        stdout,
        MoveTo(0, height - 1)
    ).ok();
    execute!(stdout, Print("<ArrK>/j/k: Move, Space: Toggle, i: Ins, d: Del, e: Edit, s: Quit & Save,  q: Quit")).ok();
}

fn draw_info(stdout: &mut Stdout, state: &mut AppState) {
    let (width, height) = size().unwrap();
    let tasks_text: &str = &format!(" {} uncompleted  ", state.tasks.iter().filter(
        |task| !task.completed
    ).count());
    let tasks_length = string_length_to_u16(tasks_text).unwrap();

    execute!(stdout, SetBackgroundColor(Color::White)).ok();
    execute!(stdout, SetForegroundColor(Color::Black)).ok();
    execute!(stdout, MoveTo(width - tasks_length, height - 1)).ok();
    execute!(stdout, Print(tasks_text)).ok();
    execute!(stdout, ResetColor).ok();

    let changed_text = if state.changed { " [CHANGED] " } else { " [NO CHANGES] " };
    let changed_length = string_length_to_u16(changed_text).unwrap();

    if state.changed {
        execute!(stdout, SetBackgroundColor(Color::Red)).ok();
    } else {
        execute!(stdout, SetBackgroundColor(Color::Green)).ok();
    }
    execute!(stdout, SetForegroundColor(Color::White)).ok();
    execute!(stdout, MoveTo(width - tasks_length - changed_length, height - 1)).ok();
    execute!(stdout, Print(changed_text)).ok();
    execute!(stdout, ResetColor).ok();

    execute!(stdout, MoveTo(width, height - 1)).ok();
}


fn draw_notice_no_tasks(stdout: &mut Stdout) {
    execute!(stdout, SetForegroundColor(Color::Grey)).ok();
    execute!(stdout, Print(&format!("{}", "No tasks."))).ok();
    execute!(stdout, ResetColor).ok();
}

fn draw_edit_task(state: &mut AppState) {
    let was_changed = state.changed.clone();
    let mut stdout = stdout();
    let min_pos = 4 + state.tasks[state.selected_index].level * LEVEL_INDENTATION.len();
    let mut pos = state.tasks[state.selected_index].description.len() + min_pos;

    terminal::enable_raw_mode().unwrap();
    execute!(stdout, MoveTo(min_pos as u16, state.selected_index as u16)).unwrap();

    let edit_colors = TaskColor {
        foreground: Some(Color::Black),
        background: Some(Color::Green),
    };

    let mut edited_task = Task {
        description: state.tasks[state.selected_index].description.clone(),
        level: state.tasks[state.selected_index].level,
        completed: state.tasks[state.selected_index].completed,
    };

    draw_task(&mut stdout, &edit_colors, state.selected_index, &edited_task);
    stdout.flush().unwrap();

    while let Event::Key(KeyEvent { code, modifiers, .. }) = event::read().unwrap() {
        state.changed = true;
        match code {
            KeyCode::Char(c) => {
                edited_task.description.insert(pos - min_pos, c);
                pos += 1;
            }
            KeyCode::Right => {
                while pos - min_pos < edited_task.description.len() {
                    pos += 1;
                    if modifiers == event::KeyModifiers::CONTROL {
                        match edited_task.description.chars().nth(pos - min_pos) {
                            Some(' ') => break,
                            None => break,
                            _ => {}
                        }
                    } else {
                        break;
                    }
                }
            }
            KeyCode::Left => {
                while pos > min_pos {
                    pos -= 1;
                    if modifiers == event::KeyModifiers::CONTROL {
                        if edited_task.description.chars().nth(pos - min_pos).unwrap() == ' ' {
                            break;
                        }
                    } else {
                        break;
                    }
                }
            }
            KeyCode::Backspace => {
                if pos > min_pos {
                    edited_task.description.remove(pos - min_pos - 1);
                    pos -= 1;
                }
            }
            KeyCode::Delete => {
                if pos - min_pos < edited_task.description.len() {
                    edited_task.description.remove(pos - min_pos);
                }
            }
            KeyCode::Esc => {
                if !was_changed && state.changed {
                    state.changed = false;
                }
                break
            },
            KeyCode::Enter => {
                if state.tasks[state.selected_index].description == edited_task.description && !was_changed {
                    state.changed = false;
                }
                state.tasks[state.selected_index].description = edited_task.description;
                break;
            }
            KeyCode::Home => {
                pos = min_pos;
            }
            KeyCode::End => {
                pos = edited_task.description.len() + min_pos;
            }

            _ => {}
        }

        draw_tasks(&mut stdout, state);
        draw_task(&mut stdout, &edit_colors, state.selected_index, &edited_task);

        stdout.flush().unwrap();
        execute!(stdout, MoveTo(pos as u16, state.selected_index as u16)).ok();
    }
}

pub(crate) async fn render_tasks(state: &mut AppState) -> bool {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();

    let mut _should_save: bool = false;
    let mut editing: bool = false;

    loop {
        redraw(state, &mut stdout);

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                if key_event.code == KeyCode::Char('s') {
                    _should_save = true;
                    break;
                }

                if key_event.code == KeyCode::Char('q') {
                    _should_save = false;
                    break;
                }

                if key_event.code == KeyCode::Char('j') || key_event.code == KeyCode::Down {
                    if state.selected_index + 1 >= state.offsetY + size().unwrap().1 as usize - 1 {
                        state.offsetY += 1;
                    }
                    if key_event.modifiers == event::KeyModifiers::CONTROL {
                        if state.selected_index < state.tasks.len() - 1 {
                            let task = state.tasks.remove(state.selected_index);
                            state.tasks.insert(state.selected_index + 1, task);
                            state.selected_index += 1;
                        }
                    } else {
                        if state.selected_index < state.tasks.len() - 1 {
                            state.selected_index += 1;
                        }
                    }
                }

                if key_event.code == KeyCode::Char('k') || key_event.code == KeyCode::Up {
                    if state.selected_index > 0 && state.selected_index <= state.offsetY {
                        state.offsetY -= 1;
                    }
                    if key_event.modifiers == event::KeyModifiers::CONTROL {
                        if state.selected_index > 0 {
                            let task = state.tasks.remove(state.selected_index);
                            state.tasks.insert(state.selected_index - 1, task);
                            state.selected_index -= 1;
                        }
                    } else {
                        if state.selected_index > 0 {
                            state.selected_index -= 1;
                        }
                    }
                }

                if key_event.code == KeyCode::Char(' ') {
                    state.tasks[state.selected_index].completed = !state.tasks[state.selected_index].completed;
                }

                if key_event.code == KeyCode::Char('d') {
                    state.tasks.remove(state.selected_index);
                    if state.tasks.len() - 1 < state.selected_index {
                        state.selected_index -= 1;
                    }
                    state.changed = true;
                }

                if key_event.code == KeyCode::Char('i') {
                    let level = if state.tasks.len() > 0 { state.tasks[state.selected_index].level } else { 0 };
                    let new_task = Task {
                        description: String::new(),
                        level,
                        completed: false,
                    };
                    if state.tasks.len() > 0 {
                        state.tasks.insert(state.selected_index + 1, new_task);
                        state.selected_index += 1;
                    } else {
                        state.tasks.insert(0, new_task);
                    }

                    editing = true;
                    state.changed = true;
                }

                if key_event.code == KeyCode::Char('e') {
                    editing = true;
                }

                if key_event.code == KeyCode::Tab {
                    state.tasks[state.selected_index].level += 1;
                    state.changed = true;
                }

                if key_event.code == KeyCode::BackTab {
                    if state.tasks[state.selected_index].level > 0 {
                        state.tasks[state.selected_index].level -= 1;
                    }
                    state.changed = true;
                }

                if key_event.code == KeyCode::Home {
                    state.selected_index = 0;

                    // Update the offset if out of screen
                    if state.selected_index < state.offsetY {
                        state.offsetY = state.selected_index;
                    }
                }

                if key_event.code == KeyCode::End {
                    state.selected_index = state.tasks.len() - 1;

                    if state.selected_index >= state.offsetY + size().unwrap().1 as usize - 1 {
                        state.offsetY = state.selected_index - size().unwrap().1 as usize + 2;
                    }
                }
            }
        }

        if editing {
            redraw(state, &mut stdout);
            draw_edit_task(state);
            editing = false;
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, MoveTo(0, 0)).ok();

    terminal::disable_raw_mode().unwrap();

    _should_save
}

fn redraw(state: &mut AppState, mut stdout: &mut Stdout) {
    execute!(stdout, MoveTo(0, 0)).ok();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    if state.tasks.len() > 0 {
        draw_tasks(&mut stdout, state);
    } else {
        draw_notice_no_tasks(&mut stdout);
    }

    draw_help(stdout);
    draw_info(stdout, state);

    stdout.flush().unwrap();
}

