#include "fish.hpp"
#include "render.hpp"
#include <vector>
#include <string>

void fish(std::vector<Cell>& curr, int rows, int cols) {
  std::string fish = "><^;>";

  int startingRow = rows / 2;
  int startingCol = cols / 2;

  for (int i = 0; i < fish.length(); i++) {
    curr[startingRow * startingCol + i].ch = fish[i];
  }
}
