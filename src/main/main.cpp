#include "render.hpp"
#include "clearScreen.hpp"
#include <thread>
#include <chrono>

int main() {

  clearScreen();

  while (true) {
    render();
    std::this_thread::sleep_for(std::chrono::microseconds(16667)); // ~60 fps
  }

  return 0;
}
