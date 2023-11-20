use crossterm::style::{Print, SetBackgroundColor};
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
    for (index, task) in state.tasks.iter().enumerate() {
        if index == state.selected_index {
            execute!(stdout, SetForegroundColor(Color::Green)).ok();
        }

        execute!(stdout, Print(&format!("[{}] {}{}", if task.completed { "x" } else { " " }, LEVEL_INDENTATION.repeat(task.level), task.description)), MoveToNextLine(1)).ok();

        if index == state.selected_index {
            execute!(stdout, ResetColor).ok();
        }
    }
}

pub(crate) fn draw_controls(stdout: &mut Stdout, changed: bool) {
    let (width, height) = size().unwrap();

    execute!(
        stdout,
        MoveTo(0, height - 1)
    ).ok();

    execute!(stdout, SetBackgroundColor(Color::DarkGrey)).ok();
    execute!(stdout, Print("<ArrK>/j/k: Move, Space: Toggle, i: Ins, d: Del, e: Edit, s: Quit & Save,  q: Quit")).ok();

    let changed_text = if changed { "[CHANGED]" } else { "[NO CHANGES]" };
    match string_length_to_u16(changed_text) {
        Some(length) => {
            execute!(stdout, SetBackgroundColor(if changed { Color::Red } else { Color::Green })).ok();
            execute!(
                stdout,
                MoveTo(width - length, height - 1)
            ).ok();
            execute!(stdout, Print(changed_text)).ok();
            execute!(stdout, ResetColor).ok();
        }
        None => println!("String is too long for u16"),
    }
}

fn move_cursor_and_readline(state: &mut AppState) {
    let mut stdout = stdout();
    let min_pos = 4 + state.tasks[state.selected_index].level * LEVEL_INDENTATION.len();
    let mut pos = state.tasks[state.selected_index].description.len() + min_pos;

    terminal::enable_raw_mode().unwrap();
    execute!(stdout, MoveTo(min_pos as u16, state.selected_index as u16)).unwrap();

    let mut input = state.tasks[state.selected_index].description.clone();
    print!("{}", input);
    stdout.flush().unwrap();

    while let Event::Key(KeyEvent { code, .. }) = event::read().unwrap() {
        match code {
            KeyCode::Char(c) => {
                input.insert(pos - min_pos, c);
                pos += 1;
            }
            KeyCode::Right => {
                if pos - min_pos < input.len() {
                    pos += 1;
                }
            }
            KeyCode::Left => {
                if pos > min_pos {
                    pos -= 1;
                }
            }
            KeyCode::Backspace => {
                if pos > min_pos {
                    input.remove(pos - min_pos - 1);
                    pos -= 1;
                }
            }
            KeyCode::Delete => {
                if pos - min_pos < input.len() {
                    input.remove(pos - min_pos);
                }
            }
            KeyCode::Esc => break,
            KeyCode::Enter => {
                state.tasks[state.selected_index].description = input;
                break;
            }
            KeyCode::Home => {
                pos = min_pos;
            }
            KeyCode::End => {
                pos = input.len() + min_pos;
            }

            _ => {}
        }

        execute!(stdout, MoveTo(0, state.selected_index as u16)).ok();
        execute!(stdout, terminal::Clear(terminal::ClearType::CurrentLine)).ok();
        execute!(stdout, Print(&format!("[{}] {}{}", if state.tasks[state.selected_index].completed { "x" } else { " " }, LEVEL_INDENTATION.repeat(state.tasks[state.selected_index].level), input))).ok();
        stdout.flush().unwrap();
        execute!(stdout, MoveTo(pos as u16, state.selected_index as u16)).ok();
    }
}

pub(crate) async fn render_tasks(state: &mut AppState) -> bool {
    let mut stdout = stdout();
    terminal::enable_raw_mode().unwrap();

    let mut _should_save: bool = false;
    let mut changed: bool = false;
    let mut editing: bool = false;

    loop {
        redraw(state, &mut stdout, changed);

        if event::poll(Duration::from_millis(100)).unwrap() {
            if let Event::Key(key_event) = event::read().unwrap() {
                changed = true;
                if key_event.code == KeyCode::Char('s') {
                    _should_save = true;
                    break;
                }

                if key_event.code == KeyCode::Char('q') {
                    _should_save = false;
                    break;
                }

                if key_event.code == KeyCode::Char('j') || key_event.code == KeyCode::Down {
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
                    if state.selected_index > 0 {
                        state.selected_index -= 1;
                    }
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
                }

                if key_event.code == KeyCode::Char('e') {
                    editing = true;
                }

                if key_event.code == KeyCode::Tab {
                    state.tasks[state.selected_index].level += 1;
                }

                if key_event.code == KeyCode::BackTab {
                    if state.tasks[state.selected_index].level > 0 {
                        state.tasks[state.selected_index].level -= 1;
                    }
                }
            }
        }

        if editing {
            redraw(state, &mut stdout, changed);
            move_cursor_and_readline(state);
            editing = false;
        }

        tokio::time::sleep(Duration::from_millis(50)).await;
    }

    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();
    execute!(stdout, MoveTo(0, 0)).ok();

    terminal::disable_raw_mode().unwrap();

    _should_save
}

fn redraw(state: &mut AppState, mut stdout: &mut Stdout, changed: bool) {
    execute!(stdout, MoveTo(0, 0)).ok();
    execute!(stdout, terminal::Clear(terminal::ClearType::All)).unwrap();

    draw_tasks(&mut stdout, state);
    draw_controls(&mut stdout, changed);

    stdout.flush().unwrap();
}