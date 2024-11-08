use eframe::{egui::*};

pub struct GUI;

#[derive(Default)]
pub struct Orinium {}

impl Orinium {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {Self::default()}
}

impl eframe::App for Orinium {
    fn save(&mut self, _storage: &mut dyn eframe::Storage) {}       
    fn update(&mut self, _ctx: &Context, _frame: &mut eframe::Frame) {}
}

impl GUI {
    pub fn display(&self, data: Vec<String>) {
        // GUIを表示するロジック
        println!("Displaying the GUI!");
    }
}