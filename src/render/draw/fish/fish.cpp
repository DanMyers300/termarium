#include "fish.hpp"
#include "render.hpp"
#include <vector>
#include <string>
#include <cstdlib>

static std::vector<Fish> fishList;

void drawFish(std::vector<Cell>& curr, int rows, int cols, Fish fish) {
  for (int i = 0; i < fish.text.length(); i++) {
    curr[fish.x * cols + fish.y + i].ch = fish.text[i];
  }
}

void fish(std::vector<Cell>& curr, int rows, int cols) {
  static Fish firstFish {
    rows / 2,
    cols / 2,
  };

  int direction = rand() % 101;

  if (direction <= 25) {
    firstFish.x += 1;
  } else if (direction <= 50 && direction >= 25) {
    firstFish.x -= 1;
  } else if (direction <= 75 && direction >= 50) {
    firstFish.y += 1;
  } else {
    firstFish.y -= 1;
  };

  drawFish(curr, rows, cols, firstFish);
}
