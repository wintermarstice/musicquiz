use eframe::{run_native, epaint::Vec2, NativeOptions};

mod musicquiz;

use musicquiz::MusicQuiz;

fn main() {
    // Initialize tracing
    tracing_subscriber::fmt::init();
    

    let app_name = "Music Quiz";
    let mut native_options = NativeOptions::default();
    native_options.initial_window_size = Some(Vec2::new(540.0, 960.0));

    run_native(app_name, native_options, Box::new(|cc| Box::new(MusicQuiz::new(cc))));

}
