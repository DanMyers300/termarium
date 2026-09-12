#include "box.hpp"
#include <string>

void box(std::string& curr, int rows, int cols) {
  curr += "\033[1;1H" + std::string(cols, '_');
  curr += "\033[" + std::to_string(rows - 1) + ";1H" + std::string(cols, '_');

  for (int i = 2; i < rows; i++) {
    curr += "\033[" + std::to_string(i) + ";1H" + std::string(1, '|');
    curr += "\033[" + std::to_string(i) + ";" + std::to_string(cols) + "H" + std::string(1, '|');
  }
}
