pub struct Fetch;

impl Fetch {
    pub fn fetch(url: &str) -> String {
        // URLからデータを取得する
        println!("Fetching data from: {}", url);
        "<html><body>Hello, World!</body></html>".to_string()
    }
}