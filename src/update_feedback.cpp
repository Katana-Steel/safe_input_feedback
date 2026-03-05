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
#include "update_feedback.hpp"
#include <algorithm>
#include <random>
#include <ranges>

namespace {
char buffer[27] = "**************************";
char permu[5] = "*#@&";
// weighted random selection of characters from permu
// the first character is selected with a probability of 60%,
// the second with a probability of 25%,
// the third with a probability of 10%,
// and the fourth with a probability of 5%
char weighted_random_selection() {
  static std::random_device rd;
  static std::mt19937 gen(rd());
  std::uniform_real_distribution<double> dis(0.0, 1.0);
  double r = dis(gen);
  if (r < 0.6) {
    return permu[0];
  } else if (r < 0.85) {
    return permu[1];
  } else if (r < 0.95) {
    return permu[2];
  } else {
    return permu[3];
  }
}

std::string last_input{};
} // namespace

namespace katana_steel {
std::string_view update_feedback(std::string_view input) {
  // no change no need to update
  if (input.compare(last_input) == 0) {
    return buffer;
  }
  last_input = input;
  for (auto &c : buffer) {
    c = weighted_random_selection();
  }
  buffer[26] = '\0';
  if (input.empty()) {
    std::fill(std::begin(buffer), std::end(buffer) - 1, '*');
    return buffer;
  }

  return buffer;
}

std::string_view get_last_input() { return std::string_view(last_input); }

void clear_last_input() {
  std::ranges::copy(buffer, last_input.begin());
  last_input.clear();
}

} // namespace katana_steel