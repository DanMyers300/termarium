cmake_minimum_required(VERSION 3.5)
project(Termarium)

file(GLOB_RECURSE sources
  src/main/*.cpp
  src/render/*.cpp
  src/clearScreen/*.cpp
  src/draw/*.cpp
  src/draw/box/*.cpp
  src/draw/fish/*.cpp
)

set(CPACK_PACKAGE_NAME "Termarium")
set(CPACK_PACKAGE_VERSION "0.1.0")

set(CPACK_MONOLITHIC_INSTALL 1)

add_executable(termarium ${sources})

target_compile_features(termarium PRIVATE cxx_std_20)

target_include_directories(termarium PRIVATE 
  src/render
  src/clearScreen
  src/draw
  src/draw/box
  src/draw/fish
)

install(TARGETS termarium RUNTIME DESTINATION bin)

include(CPack)
