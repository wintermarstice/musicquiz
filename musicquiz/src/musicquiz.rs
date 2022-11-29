use eframe::{egui::{Layout, Context, FontDefinitions, FontData, Ui, RichText, CentralPanel, ScrollArea, Separator, TopBottomPanel, Label, Hyperlink}, CreationContext, emath::Align, epaint::{FontId, Color32}, App, Frame};

pub struct MusicQuiz {
    tracks: Vec<TrackCardData>
}

impl MusicQuiz {
    pub fn new(cc: &CreationContext<'_>) -> Self {
        let iter = (0..10).map(|a| TrackCardData {
            album: format!("Album #{}", a),
            artist: format!("Artist {}", a),
            title: format!("Song Nr{}", a),
        });

        Self::configure_fonts(&cc.egui_ctx);

        Self { tracks: Vec::from_iter(iter) }
    }

    fn configure_fonts(ctx: &Context) {
        let mut font_definitions = FontDefinitions::default();
        let font_name = "Questrial".to_string();
        let font_data = FontData::from_static(include_bytes!("../../Questrial-Regular.ttf"));

        font_definitions.font_data.insert(font_name.clone(), font_data);
        font_definitions.families.insert(eframe::epaint::FontFamily::Proportional, vec![font_name.clone()]);
        ctx.set_fonts(font_definitions);
    }

    fn render_track_cards(&mut self, ui: &mut Ui) {
        for (id, track) in self.tracks.iter().enumerate() {
            render_track_card(id, track, ui);
        }
    }
}

fn render_track_card(number: usize, track: &TrackCardData, ui: &mut Ui) {
    ui.with_layout(Layout::left_to_right(Align::TOP), |ui| {

        ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
            let number_text = RichText::new(number.to_string())
                .font(FontId::proportional(32.0))
                .color(Color32::WHITE);

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
        CentralPanel::default().show(ctx, |ui| {
            render_header(ui);
            ScrollArea::vertical().auto_shrink([false, true]).show(ui, |ui| {
                self.render_track_cards(ui);
            });
            render_footer(ctx);
        });
    }
}

fn render_footer(ctx: &Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            let made_with_egui_link = Hyperlink::from_label_and_url(
                RichText::new("made with egui").monospace(), 
                "https://www.github.com/emilk/egui/");

            let about_label = Label::new(RichText::new("a reincarnation of the iPod game of the same name").monospace());

            
            
            ui.add_space(10.0);

            ui.add(about_label);
            ui.add(made_with_egui_link);

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