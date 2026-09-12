#include "render.hpp"
#include "box.hpp"
#include <sys/ioctl.h>
#include <unistd.h>
#include <string>
#include <vector>

static std::vector<Cell> prev;

void render() {
  struct winsize window_size;
  ioctl(STDOUT_FILENO, TIOCGWINSZ, &window_size);

  int rows = window_size.ws_row;
  int cols = window_size.ws_col;

  std::vector<Cell> curr(rows * cols);

  box(curr, rows, cols);

  if (prev.size() != curr.size()) {
    prev.assign(curr.size(), Cell{});
  }

  std::string buf;

  for (int r = 0; r < rows; r++) {
    for (int c = 0; c < cols; c++) {
      int i = r * cols + c;
      if (curr[i] == prev[i]) continue;
      buf += "\033[" + std::to_string(r+1) + ";" + std::to_string(c+1) + "H";
      buf += curr[i].ch;
    }
  }

  write(STDOUT_FILENO, buf.c_str(), buf.size());

  prev = curr;
}
