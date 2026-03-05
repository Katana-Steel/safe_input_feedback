#pragma once
#include <cstddef>
#include <cstdio>
#include <cstdlib>
#include <readline/readline.h>
#include <string_view>

namespace katana_steel {
void initialize_lib();
std::string_view get_password(std::string_view prompt);
extern void clear_last_input();
} // namespace katana_steel