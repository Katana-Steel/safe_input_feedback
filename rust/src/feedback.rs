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

use rand::Rng;

/// Default width of the fixed feedback display.
pub const DEFAULT_FEEDBACK_LEN: usize = 26;

/// Picks a random feedback character with weighted probabilities:
/// '*' 60%, '#' 25%, '@' 10%, '&' 5%
fn random_feedback_char() -> char {
    let val = rand::rng().random_range(0..100);
    match val {
        0..60 => '*',
        60..85 => '#',
        85..95 => '@',
        _ => '&',
    }
}

/// Returns a fixed-width feedback string of random symbols.
/// When input is empty, returns all asterisks.
/// The output length never reveals the actual input length.
pub fn update_feedback(input: &str) -> String {
    update_feedback_with_width(input, DEFAULT_FEEDBACK_LEN)
}

/// Returns a feedback string of `width` random symbols.
/// When input is empty, returns all asterisks.
pub fn update_feedback_with_width(input: &str, width: usize) -> String {
    if input.is_empty() {
        return "*".repeat(width);
    }

    (0..width).map(|_| random_feedback_char()).collect()
}
