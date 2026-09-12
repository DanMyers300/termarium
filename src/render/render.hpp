#ifndef RENDER_HPP
#define RENDER_HPP

struct Cell {
  char ch = ' ';
  
  bool operator==(const Cell&) const = default;
};

void render();

#endif // RENDER_HPP
