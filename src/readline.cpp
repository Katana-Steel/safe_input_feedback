#include "readline.hpp"
#include "update_feedback.hpp"

#include <iostream>
#include <readline/readline.h>
#include <readline/rltypedefs.h>

namespace {
// A RAII holder to temporarily replace the readline redisplay function
// for the duration of the get_password function, ensuring that it is restored
struct holder {
  rl_voidfunc_t *redraw;
  explicit holder(rl_voidfunc_t replace) {
    redraw = rl_redisplay_function;
    rl_redisplay_function = replace;
  }
  ~holder() {
    rl_redisplay_function = redraw;
    std::cout << "\033[?25h" << std::flush;
  }
  holder(const holder &) = delete;
  holder &operator=(const holder &) = delete;
  holder(holder &&) = delete;
  holder &operator=(holder &&) = delete;
};
} // namespace

namespace katana_steel {

void redraw_input_prompt() {
  auto current_state = update_feedback(rl_line_buffer);
  std::cout << "\r\033[K" << rl_display_prompt;
  std::cout << current_state;
  int cursor_pos = strlen(rl_display_prompt) + rl_point;
  std::cout << "\r\033[" << cursor_pos << "C" << std::flush;
}

std::string_view get_password(std::string_view prompt) {
  holder h(redraw_input_prompt);
  clear_last_input();
  std::cout << "\033[?25l";
  char *input = readline(prompt.data());
  auto last_input = get_last_input();
  if (input == last_input) {
    free(input);
    return last_input;
  }
  return {};
}

void initialize_lib() {
  std::atexit([]() { clear_last_input(); });
  rl_initialize();
}

// Expose C-compatible functions for use in other languages or contexts
extern "C" {
void __attribute__((constructor)) init() { initialize_lib(); }
void __attribute__((destructor)) cleanup() { clear_last_input(); }
char *get_password(const char *prompt) {
  return const_cast<char *>(get_password(std::string_view(prompt)).data());
}
}
} // namespace katana_steel
