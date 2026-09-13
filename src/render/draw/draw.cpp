#include "draw.hpp"
#include "box.hpp"
#include "render.hpp"
#include <vector>

void draw(std::vector<Cell>& curr, int rows, int cols) {
  box(curr, rows, cols);
}
