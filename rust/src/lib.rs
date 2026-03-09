/*
 * safe_input_feedback
 * Copyright (C) 2026 Rene Kjellerup
 *
 * This program is free software: you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation, either version 3 of the License, or
 * (at your option) any later version.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 *
 * You should have received a copy of the GNU General Public License
 * along with this program.  If not, see <https://www.gnu.org/licenses/>.
 */

pub mod feedback;

use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{self, ClearType},
};
use std::io::{self, Write};

/// Prompts the user for password input with fixed-length visual feedback
/// that does not reveal the actual input length.
///
/// The terminal is put into raw mode during input. The cursor is hidden
/// and a 26-character randomized feedback string is displayed regardless
/// of how many characters have been typed.
pub fn get_password(prompt: &str) -> io::Result<String> {
    let mut stdout = io::stdout();
    let mut input = String::new();

    print!("{}", prompt);
    stdout.flush()?;

    let (prompt_col, prompt_row) = cursor::position()?;

    terminal::enable_raw_mode()?;
    execute!(stdout, cursor::Hide)?;

    // Show initial feedback
    let fb = feedback::update_feedback("");
    execute!(
        stdout,
        cursor::MoveTo(prompt_col, prompt_row),
        terminal::Clear(ClearType::UntilNewLine),
    )?;
    print!("{}", fb);
    stdout.flush()?;

    loop {
        if let Event::Key(KeyEvent {
            code, modifiers, ..
        }) = event::read()?
        {
            match code {
                KeyCode::Enter => break,
                KeyCode::Char('c') if modifiers.contains(KeyModifiers::CONTROL) => {
                    execute!(stdout, cursor::Show)?;
                    terminal::disable_raw_mode()?;
                    println!();
                    std::process::exit(0);
                }
                KeyCode::Backspace => {
                    input.pop();
                }
                KeyCode::Char(c) => {
                    input.push(c);
                }
                _ => {}
            }

            let fb = feedback::update_feedback(&input);
            execute!(
                stdout,
                cursor::MoveTo(prompt_col, prompt_row),
                terminal::Clear(ClearType::UntilNewLine),
            )?;
            print!("{}", fb);
            stdout.flush()?;
        }
    }

    execute!(stdout, cursor::Show)?;
    terminal::disable_raw_mode()?;
    println!();

    Ok(input)
}
