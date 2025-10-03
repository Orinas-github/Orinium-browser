/// ブラウザのコア機能を提供するモジュール
/// このモジュールには、HTML/CSSパーサー、DOMツリー構築、
/// JavaScriptエンジンなどブラウザの中核となる機能が含まれます。
pub mod engine;

/// プラットフォーム依存の機能を提供するモジュール
/// このモジュールには、ネットワーク処理、レンダリング、UI表示、
/// ファイルI/Oなどプラットフォーム固有の実装が含まれます。
pub mod platform;

// 便利な再エクスポート
pub use engine::html;
pub use platform::network;
pub use engine::html::parser; // parser モジュールを直接再エクスポート
