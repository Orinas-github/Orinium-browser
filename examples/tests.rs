use orinium_browser::{
    platform::ui::App,
    platform::network::NetworkCore,
    engine::html::parser,
};

use std::env;
use winit::event_loop::EventLoop;

#[tokio::main]
async fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() >= 2  {
        match args[1].as_str() {
            "help" => {
                println!("This is a test application for Orinium Browser development.");
                println!("Usage: cargo run --example tests [NAME]\n");
                println!("Test names:");
                println!("create_window - Create a window and display it.");
                println!("parse_dom [URL] - Test DOM parsing functionality.");
            }
            "create_window" => {
                let _ = run();
            }
            "parse_dom" => {
                if args.len() == 3 {
                    let url = &args[2];
                    println!("Parsing DOM for URL: {}", url);
                    let net = NetworkCore::new().unwrap();
                    let resp = net.fetch(url).await.expect("Failed to fetch URL");
                    let html = String::from_utf8_lossy(&resp.body).to_string();
                    println!("Fetched HTML (first 50 chars):\n{}", &html[..50.min(html.len())]);
                    let mut parser = parser::Parser::new(&html);
                    let dom = parser.parse();
                    parser::print_dom_tree(&dom, &[]);
                } else {
                    eprintln!("Please provide a URL for DOM parsing test.");
                }
            }
            _ => {
                eprintln!("Unknown argument: {}", args[1]);
                eprintln!("Use --help or -h for usage information.");
            }
        }
    } else {
        eprintln!("No arguments provided. Use help for usage information.");
    }
}

fn run() -> anyhow::Result<()> {
    env_logger::init();

    let event_loop = EventLoop::with_user_event().build()?;
    let mut app = App::new();
    event_loop.run_app(&mut app)?;

    Ok(())
}
