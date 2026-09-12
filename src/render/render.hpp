#ifndef RENDER_HPP
#define RENDER_HPP

struct Cell {
  char ch = ' ';

  bool operator==(const Cell& other) {
    ch == other.ch;
  };
};

void render();

#endif // RENDER_HPP
