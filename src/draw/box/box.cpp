#include "box.hpp"
#include "render.hpp"
#include <vector>

void box(std::vector<Cell>& buf, int rows, int cols) {
  auto set = [&](int r, int c, char ch) {
    buf[r * cols + c].ch = ch;
  };

  for (int c = 0; c < cols; c++) {
    set(0, c, '_');
    set(rows - 1, c, '_');
  }

  for (int r = 0; r < rows; r++) {
    set(r, 0, '|');
    set(r, cols - 1, '|');
  }
}
