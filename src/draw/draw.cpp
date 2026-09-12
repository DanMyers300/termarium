#include "box.hpp"
#include "draw.hpp"
#include "render.hpp"
#include <vector>

void draw(std::vector<Cell>& buf, int rows, int cols) {
  box(buf, rows, cols);
}
