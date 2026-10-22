#include "fish.hpp"
#include "render.hpp"
#include <vector>
#include <string>
#include <cstdlib>
#include <iostream>
#include <fstream>

static std::vector<Fish> fishList;
static int dirFishX;
static int dirFishY;

void drawFish(std::vector<Cell>& curr, int rows, int cols, Fish fish) {
  for (int i = 0; i < fish.text.length(); i++) {
    curr[fish.x * cols + fish.y + i].ch = fish.text[i];
  }
}

void choosePoint(int rows, int cols) {
  dirFishY = rand() % rows;
  dirFishX = rand() % cols;
}

void fish(std::vector<Cell>& curr, int rows, int cols) {
  static Fish firstFish {
    rows / 2,
    cols / 2,
  };

  if (!dirFishY || !dirFishX || (firstFish.x == dirFishX && firstFish.y == dirFishY)) {
    choosePoint(rows, cols);
  }

  std::ofstream logFile("log.txt", std::ios::app);

  logFile << dirFishX << "|" << dirFishY << "\n";

  logFile.close();

  if (firstFish.x < dirFishX) {
    firstFish.x += 1;
  } else if (firstFish.x > dirFishX) {
    firstFish.x -= 1;
  }

  if (firstFish.y < dirFishY) {
    firstFish.y += 1;
  } else if (firstFish.y > dirFishY) {
    firstFish.y -= 1;
  }

  drawFish(curr, rows, cols, firstFish);
}
