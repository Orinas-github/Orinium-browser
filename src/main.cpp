#include <iostream>
#include <string>
#include <filesystem>
#include <cstdlib>
#include "network/http_request.h"
#include "ui/window.h"

int main(int argc, char* argv[]) {
    setupWindow();
    // 初期化
    std::string url = "about:blank";
    // 起動引数を確認
    if (argc > 1) {
        std::cerr << "Usage: " << argv[1] << std::endl;
        if (argv[1] == "-u"){
            url = argv[2];
        }
    }
    // ネットワークモジュールを使ってURLの中身を取得
    FetchURL(url, "auto");
    // 取得したHTMLをレンダリングエンジンに渡してパース
    // JavaScriptエンジンで必要なスクリプトを実行
    // 最後にUIフレームワークで描画

    return 0;
}