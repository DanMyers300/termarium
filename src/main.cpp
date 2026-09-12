#include "render.hpp"
#include <thread>
#include <chrono>

int main() {
  while (true) {
    render();
    std::this_thread::sleep_for(std::chrono::microseconds(16667)); // ~60 fps
  }
}
