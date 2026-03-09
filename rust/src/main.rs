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

use safe_input_feedback::get_password;
use std::io::{self, Write};

fn main() -> io::Result<()> {
    let password = get_password("Enter your password: ")?;
    println!("Actual input was: {}", password);

    print!("next input: ");
    io::stdout().flush()?;
    let mut next = String::new();
    io::stdin().read_line(&mut next)?;
    let next = next.trim_end();
    if !next.is_empty() {
        println!("You entered: {}", next);
    }

    Ok(())
}
