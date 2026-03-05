#pragma once
#include <string_view>

namespace katana_steel {

std::string_view update_feedback(std::string_view input);
std::string_view get_last_input();
void clear_last_input();

} // namespace katana_steel
