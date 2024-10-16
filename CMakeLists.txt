cmake_minimum_required(VERSION 3.10)
project(Orinium_Browser)

set(CMAKE_CXX_STANDARD 20)

# FLTKのインクルードパスを指定
find_package(FLTK REQUIRED)

add_executable(Orinium_Browser src/main.cpp src/ui/window.cpp) # ソースファイル名を指定
target_link_libraries(my_fltk_app PRIVATE fltk fltk_images)
