cmake_minimum_required(VERSION 3.5)
project(Termarium)

file(GLOB_RECURSE sources      src/main/*.cpp src/main/*.h)
file(GLOB_RECURSE sources_test src/test/*.cpp)

set(CPACK_PACKAGE_NAME "Termarium")
set(CPACK_PACKAGE_VERSION "0.1.0")

set(CPACK_MONOLITHIC_INSTALL 1)

add_executable(termarium ${sources})

install(TARGETS termarium RUNTIME DESTINATION bin)

include(CPack)
