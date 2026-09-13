#include "render.hpp"
#include <thread>
#include <chrono>
#include <iostream>
#include <csignal>

static void cleanup(int) {
  std::cout << "\e[?25h" << std::flush;
  std::exit(0);
}

int main() {
  std::signal(SIGINT, cleanup);
  std::signal(SIGTERM, cleanup);
  std::cout << "\e[?25l" << std::flush;

  while (true) {
    render();
    std::this_thread::sleep_for(std::chrono::microseconds(16667));
  }
}
