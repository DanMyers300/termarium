#include "clearScreen.hpp"
#include <iostream>

int clearScreen() {

  std::cout << "\e[?25l\033[2J\033[H" << std::flush;

  return 0;
}
