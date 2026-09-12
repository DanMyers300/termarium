#include "render.hpp"
#include "draw.hpp"
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

  std::string buf;
  for (int r = 0; r < rows; r++) {
    for (int c = 0; c < cols; c++) {
      int i = r * c;
      if (prev[i] == curr[i]) continue;
      buf += "\033[" + std::to_string(curr[i]) + ";" + std::to_string(curr[c]) + "H";
      curr += buf
    }
  }

  draw(curr, rows, cols);

  write(STDOUT_FILENO, buf.c_str(), buf.size());

  prev = curr;
}
