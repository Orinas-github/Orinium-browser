// http_request.h

#ifndef HTTP_REQUEST_H // インクルードガード
#define HTTP_REQUEST_H

#include <string>

//関数の宣言
std::string FetchURL(const std::string& url, const std::string& useragent);

#endif // HTTP_REQUEST_H // インクルードガードの終了