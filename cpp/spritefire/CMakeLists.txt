cmake_minimum_required(VERSION 3.12)
set(SPRITEFIRE_LIB "spritefire")
project(${SPRITEFIRE_LIB} C CXX)

set(CMAKE_CXX_STANDARD 11)
set(CMAKE_CXX_STANDARD_REQUIRED True)

set(CMAKE_CXX_FLAGS_RELEASE "-O3")

find_package(OpenCV REQUIRED)

include_directories(${OpenCV_INCLUDE_DIRS})
include_directories(include src)

file(GLOB_RECURSE SOURCES "src/*.cpp")

add_library(${SPRITEFIRE_LIB} STATIC ${SOURCES})
