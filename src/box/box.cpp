#include <string>

int box(std::string& buf, int rows, int cols) {
  buf += "\033[1;1H" + std::string(cols, '_');
  buf += "\033[" + std::to_string(rows - 1) + ";1H" + std::string(cols, '_');

  for (int i = 2; i < rows; i++) {
    buf += "\033[" + std::to_string(i) + ";1H" + std::string(1, '|');
    buf += "\033[" + std::to_string(i) + ";" + std::to_string(cols) + "H" + std::string(1, '|');
  }

  return 0;
}
