#include <iostream>

int main() {
  while (true) {
    std::cout << "\e[?25l\033[2J";
  }
  return 0;
}
