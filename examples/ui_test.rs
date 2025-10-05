use orinium_browser::platform::ui::App;
use std::env;
use winit::event_loop::EventLoop;

fn main() {
    let args: Vec<String> = env::args().collect::<Vec<String>>();
    if args.len() == 2  {
        match args[1].as_str() {
            "help" => {
                println!("This is a UI test application for Orinium Browser.");
                println!("Usage: cargo run --example ui_test [NAME]\n");
                println!("Test names:");
                println!("create_window - Create a window and display it.");
            }
            "create_window" => {
                let _ = run();
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
