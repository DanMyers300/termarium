#include "draw.hpp"
#include "box.hpp"
#include "clearScreen.hpp"
#include <string>

void draw(std::string& curr, int rows, int cols) {
  clearScreen();
  box(curr, rows, cols);
}
