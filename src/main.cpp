#include "readline.hpp"
#include <cstdlib>
#include <iostream>
#include <readline/readline.h>

// Custom redisplay function

int main() {
  katana_steel::initialize_lib();
  std::string input =
      katana_steel::get_password("Enter your password: ").data();
  std::cout << "\nActual input was: " << input << std::endl;
  auto next = readline("next input: ");
  if (next) {
    std::cout << "You entered: " << next << std::endl;
    free(next);
  }

  return 0;
}