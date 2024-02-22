cmake_minimum_required(VERSION 3.12)

project("spritefire" C CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_CXX_FLAGS_RELEASE "-O3")

add_subdirectory("spritefire")

set(TARGET main)
add_executable(${TARGET} main.cpp)
target_compile_features(${TARGET} PRIVATE cxx_std_11)
target_link_libraries(${TARGET} "spritefirelib")
