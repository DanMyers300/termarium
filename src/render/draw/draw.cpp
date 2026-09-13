#include "draw.hpp"
#include "box.hpp"
#include "render.hpp"
#include "clearScreen.hpp"
#include <vector>

void draw(std::vector<Cell>& curr, int rows, int cols) {
  clearScreen();
  box(curr, rows, cols);
}
