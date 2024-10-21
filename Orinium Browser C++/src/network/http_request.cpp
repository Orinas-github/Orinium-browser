#include <iostream>
#include <string>
#include <curl/curl.h>


size_t WriteCallback(void* contents, size_t size, size_t nmemb, std::string* userp) {
    // コールバック関数
    size_t totalSize = size * nmemb;
    userp->append((char*)contents, totalSize);
    return totalSize;
}

std::string FetchURL(const std::string& url, const std::string& useragent) { 
    // useragent が自動の時の処理
    if (useragent == "@auto"){
        return FetchURL(url, "MyUserAgent/v0.1");
    } 

    if (false){
        // ブラウザ固有のURL用 ※未定義
    } else {
        // URLからデータを取得する
        CURL *curl;
        CURLcode res;
        std::string response; // レスポンスデータを格納するためのstring

        curl = curl_easy_init();
        if(curl) {
            curl_easy_setopt(curl, CURLOPT_URL, url);
            curl_easy_setopt(curl, CURLOPT_USERAGENT, useragent);
        
            // コールバック関数を設定
            curl_easy_setopt(curl, CURLOPT_WRITEFUNCTION, WriteCallback);
            curl_easy_setopt(curl, CURLOPT_WRITEDATA, &response);

            // データの取得を実行
            res = curl_easy_perform(curl);

            // 実行結果をチェック
            if(res != CURLE_OK) {
                std::cerr << "curl_easy_perform() failed: " << curl_easy_strerror(res) << std::endl;
            } else {
                std::cout << "Response received:" << std::endl;
                std::cout << response << std::endl; // レスポンスを表示
            }

            // 後片付け
            curl_easy_cleanup(curl);

            return response;
        }
    }
}
