use javascript::JSEngine;
use network::Fetch as net;
use renderer::HTMLRenderer as html;
use ui::GUI;

mod javascript;
mod network;
mod renderer;
mod ui;

fn main() {
    // 各モジュールを初期化
    let js_engine = JSEngine::new();
    let gui = GUI;
    let htmlcode = r#"<!DOCTYPE html>
<html lang="ja">
<head>
    <meta charset="UTF-8">
    <meta name="viewport" content="width=device-width, initial-scale=1.0">
    <title>ベンチマークテスト</title>
    <script>
        function startBenchmark() {
            const start = performance.now();
            // テスト対象のコードをここに記述
            for (let i = 0; i < 1000000; i++) {
                // サンプル処理
                Math.sqrt(i);
            }
            const end = performance.now();
            document.getElementById("result").innerText = `処理時間: ${end - start}ミリ秒`;
        }
    </script>
</head>
<body>
    <h1>HTMLベンチマークテスト</h1>
    <button onclick="startBenchmark()">テスト開始</button>
    <p id="result"></p>
</body>
</html>"#;

    // JSを実行
    js_engine.execute("console.log('Hello, World!');");

    // HTMLを取得、レンダリング
    // html::render(net::fetch("https://example.com").as_str());
    html::render(&htmlcode);

    // 描画
    gui.display();
}