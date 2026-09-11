#include "render.hpp"
#include "box.hpp"
#include <sys/ioctl.h>
#include <unistd.h>
#include <string>

int render() {

  struct winsize window_size;
  ioctl(STDOUT_FILENO, TIOCGWINSZ, &window_size);

  int rows = window_size.ws_row;
  int cols = window_size.ws_col;

  std::string buf;

  box(buf, rows, cols);

  write(STDOUT_FILENO, buf.c_str(), buf.size());

  return 0;
}
