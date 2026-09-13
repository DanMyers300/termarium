#include "box.hpp"
#include "render.hpp"
#include <vector>

void box(std::vector<Cell>& curr, int rows, int cols) {
  for (int c = 0; c < cols; c++) {
    curr[0 * cols + c].ch = '_';
    curr[(rows - 1) * cols + c].ch = '_';
  }

  for (int r = 1; r < rows; r++) {
    curr[r * cols + 0].ch = '|';
    curr[r * cols + (cols - 1)].ch = '|';
  }
}
