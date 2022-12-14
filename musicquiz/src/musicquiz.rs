use std::path::Path;

use eframe::{egui::{Layout, Context, FontDefinitions, FontData, Ui, RichText, CentralPanel, ScrollArea, Separator, TopBottomPanel, Label, Hyperlink, Button, Sense, Visuals, Window}, CreationContext, emath::Align, epaint::{FontId, Color32}, App, Frame};
use serde::{Serialize, Deserialize};

/// Dark orange color
const COLOR_DKORANGE: Color32 = Color32::from_rgb(252, 78, 3);

/// Orange color
const COLOR_ORANGE: Color32 = Color32::from_rgb(252, 152, 3);

pub struct MusicQuiz {
    tracks: Vec<TrackCardData>,
    config: MusicQuizConfig,
}

#[derive(Serialize, Deserialize)]
pub struct MusicQuizConfig {
    dark_mode: bool,
    music_sources: String,
}

impl Default for MusicQuizConfig {
    fn default() -> Self {
        let get_music_dir = || {
            directories::UserDirs::new()
                .map(|ref dirs| dirs.audio_dir().unwrap_or(dirs.home_dir()).to_owned())
                .map(|ref dir| dir.to_str().unwrap_or_default().to_owned())
                .unwrap_or_default()
        };

        Self { dark_mode: true, music_sources: get_music_dir().to_string() }
    }
}

impl MusicQuiz {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let iter = (0..30).map(|a| TrackCardData {
            album: format!("Album #{}", a),
            artist: format!("Artist {}", a),
            title: format!("Song Nr{}", a),
        });

        Self::configure_fonts(&cc.egui_ctx);

        let config: MusicQuizConfig = confy::load("MusicQuiz", None).unwrap_or_default();

        Self { tracks: Vec::from_iter(iter), config }
    }

    fn configure_fonts(ctx: &Context) {
        let mut font_definitions = FontDefinitions::default();
        let font_name = "Questrial".to_string();
        let matsym_font_name = "MaterialSymbols".to_string();
        let font_data = FontData::from_static(include_bytes!("../../Questrial-Regular.ttf"));
        let matsym_font_data = FontData::from_static(include_bytes!("../../MaterialIcons-Regular.ttf"));

        font_definitions.font_data.insert(font_name.clone(), font_data);
        font_definitions.font_data.insert(matsym_font_name.clone(), matsym_font_data);
        font_definitions.families.insert(eframe::epaint::FontFamily::Proportional, vec![font_name.clone()]);
        font_definitions.families.insert(eframe::epaint::FontFamily::Name(matsym_font_name.clone().into()), vec![matsym_font_name.clone(), font_name.clone()]);
        ctx.set_fonts(font_definitions);
    }

    fn render_track_cards(&mut self, ui: &mut Ui) {
        for (id, track) in self.tracks.iter().enumerate() {
            render_track_card(&self.config, id, track, ui);
        }
    }

    fn render_top_panel(&mut self, ctx: &Context, frame: &mut Frame) {
        let make_icon = |selector: &str| {
            RichText::new(selector).font(FontId::new(24.0, eframe::epaint::FontFamily::Name("MaterialSymbols".into())))
        };

        TopBottomPanel::top("top_panel").show(ctx, |ui| {
            ui.add_space(10.0);

            eframe::egui::menu::bar(ui, |ui| {
                // The logo
                ui.with_layout(Layout::left_to_right(Align::Min), |ui| {
                    let logo_widget = Label::new(make_icon("\u{e3a1}"));

                    ui.add(logo_widget);
                    ui.label("Music Quiz");
                });

                // Controls
                ui.with_layout(Layout::right_to_left(Align::Min), |ui| {
                    let close_button = Label::new(make_icon("\u{e5cd}"));
                    let config_button = Label::new(make_icon("\u{e8b8}"));

                    // Switch symbols for theme button depending on the theme mode.
                    let theme_button = Label::new(make_icon(match self.config.dark_mode {
                        true => "\u{e1c6}", // Sunrise symbol
                        false => "\u{e51c}", // Moon symbol
                    }));

                    if ui.add(close_button.sense(Sense::click())).clicked() {
                        // Quit button clicked. Quit the app.
                        frame.close();
                    }

                    ui.add(config_button);

                    if ui.add(theme_button.sense(Sense::click())).clicked() {
                        // Theme switching button clicked. Switch the theme
                        self.config.dark_mode = !self.config.dark_mode;
                        
                    }
                })
            });
        });
    }
}

fn render_track_card(config: &MusicQuizConfig, number: usize, track: &TrackCardData, ui: &mut Ui) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {

        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            // Change numeral color based on the theme
            let number_color = match config.dark_mode {
                true => Color32::WHITE,
                false => Color32::BLACK,
            };

            let number_text = RichText::new(number.to_string())
                .font(FontId::proportional(32.0))
                .color(number_color);

            ui.label(number_text);
        });

        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            let title_text = RichText::new(&track.title).font(FontId::proportional(24.0));
            let artist_text = RichText::new(&track.artist).font(FontId::proportional(16.0));
            let album_text = RichText::new(&track.album).font(FontId::proportional(16.0));
            
            ui.label(title_text);
            ui.label(artist_text);
            ui.label(album_text);

            ui.separator();
        });
    });
}

struct TrackCardData {
    title: String,
    artist: String,
    album: String
}

impl App for MusicQuiz {
    fn update(&mut self, ctx: &Context, frame: &mut Frame) {
        // Do updates here.
        match self.config.dark_mode {
            true => ctx.set_visuals(Visuals::dark()),
            false => ctx.set_visuals(Visuals::light()),
        };

        self.render_top_panel(ctx, frame);
        render_footer(&self.config, ctx);
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            
            ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                self.render_track_cards(ui);
            });
        });
    }

    fn on_exit(&mut self, _gl: Option<&eframe::glow::Context>) {
        // Save config file on exit
        confy::store("MusicQuiz", None, &self.config).expect("Failed to save configuration");
    }
}

fn render_footer(config: &MusicQuizConfig, ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        // Set hyperlink colors
        ui.style_mut().visuals.hyperlink_color = match &config.dark_mode {
            true => COLOR_DKORANGE,
            false => COLOR_ORANGE,
        };

        ui.vertical_centered(|ui| {
            let made_with_egui_link = Hyperlink::from_label_and_url(
                RichText::new("made with egui").monospace(), 
                "https://www.github.com/emilk/egui/");

            let about_label = Label::new(RichText::new("a reincarnation of the iPod game of the same name").monospace());

            let repo_link = Hyperlink::from_label_and_url(
                RichText::new("wintermarstice/musicquiz").monospace(),
                "https://www.github.com/wintermarstice/musicquiz");
            
            ui.add_space(10.0);

            ui.add(about_label);
            ui.add(made_with_egui_link);
            ui.add(repo_link);

            ui.add_space(10.0);
        });
    });
}

fn render_header(ui: &mut Ui) {
    ui.vertical_centered(|ui| {
        ui.heading("Music Quiz");
    });

    let separator = Separator::default().spacing(28.0);
    ui.add(separator);
}