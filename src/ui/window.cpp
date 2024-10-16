#include <iostream>
#include <string>
#include <FL/Fl.H>
#include <FL/Fl_Window.H>
#include <FL/Fl_Button.H>
#include <FL/Fl_Box.H>
#include "window.h"

int setupWindow() {
    // ウィンドウを作成
    Fl_Window* window = new Fl_Window(300, 200, "FLTK Sample");

    // ウィンドウ表示
    window->end();
    window->show();

    return Fl::run(); // イベントループ開始
}