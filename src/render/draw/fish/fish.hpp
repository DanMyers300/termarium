#ifndef FISH_HPP
#define FISH_HPP

#include "render.hpp"
#include <vector>
#include <string>

struct Fish {
  int x;
  int y;
  std::string text = "><^;>";
};

void fish(std::vector<Cell>& buf, int rows, int cols);

//void drawFish(std::vector<Cell>& curr, int rows, int cols, Fish fish);

#endif // FISH_HPP
