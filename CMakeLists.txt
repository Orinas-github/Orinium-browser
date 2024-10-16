cmake_minimum_required(VERSION 3.10)
project(Orinium_Browser)

set(CMAKE_CXX_STANDARD 20)

add_executable(Orinium_Browser src/main.cpp src/ui/window.cpp) # ソースファイル名を指定
target_link_libraries(Orinium_Browser)
