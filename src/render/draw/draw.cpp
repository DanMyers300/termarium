#include "draw.hpp"
#include "box.hpp"
#include "clearScreen.hpp"
#include <string>

void draw(std::string& buf, int rows, int cols) {
  clearScreen();
  box(buf, rows, cols);
}
