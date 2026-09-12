#include "render.hpp"
#include "draw.hpp"
#include <sys/ioctl.h>
#include <unistd.h>
#include <string>

void render() {
  struct winsize window_size;
  ioctl(STDOUT_FILENO, TIOCGWINSZ, &window_size);

  int rows = window_size.ws_row;
  int cols = window_size.ws_col;

  std::string buf;

  draw(buf, rows, cols);

  write(STDOUT_FILENO, buf.c_str(), buf.size());
}
