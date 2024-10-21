cmake_minimum_required(VERSION 3.10)
project(Orinium_Browser)

set(CMAKE_CXX_STANDARD 20)

# wxWidgetsの設定
find_package(wxWidgets REQUIRED COMPONENTS core base)

# wxWidgetsの設定を使って、コンパイルに必要なフラグを取得
include(${wxWidgets_USE_FILE})

# 実行可能ファイルの作成
add_executable(Orinium_Browser src/main.cpp src/ui/window.cpp) # ソースファイル名を指定

# wxWidgetsのライブラリをリンク
target_link_libraries(Orinium_Browser ${wxWidgets_LIBRARIES})