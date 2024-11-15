use javascript::JSEngine;
use network::Fetch as net;
use renderer::HTMLRenderer as html;
use ui::GUI as uisystem;


mod javascript;
mod network;
mod renderer;
mod ui;

use bevy::prelude::*;

fn main() {
    App::build()
        .add_plugins(DefaultPlugins) // デフォルトのプラグインを追加
        .add_startup_system(setup.system()) // 初期設定システムを追加
        .run(); // アプリを実行
}

fn setup(mut commands: Commands, mut materials: ResMut<Assets<ColorMaterial>>) {
    // カメラを追加
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());
    
    // 四角形を追加
    commands.spawn_bundle(SpriteBundle {
        material: materials.add(Color::rgb(0.2, 0.7, 0.5).into()), // 四角形の色
        ..Default::default()
    });
}

/*
fn main() {
    // 各モジュールを初期化
    let js_engine = JSEngine::new();
    let gui = uisystem;
    let htmlcode = r#"<!DOCTYPE html>
<html lang="ja">
    <h1>hello</h1>
    <p><span>goodbye</span></p>
</html>"#;

    // JSを実行
    js_engine.execute("console.log('Hello, World!');");

    // HTMLを取得、レンダリング、描画
    // html::render(net::fetch("https://example.com").as_str());
    gui.display(html::render(&htmlcode));

    // グラフィックの初期化とウィンドウ作成をする

    App::new().run();

}
*/