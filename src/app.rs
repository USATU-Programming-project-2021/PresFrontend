use crate::analyzer::generic_analyzer::YmlAnalyzer;
use crate::analyzer::presentation::PresYml;
use crate::analyzer::slide::SlideYml;
use eframe::{egui, epi};

#[derive(serde::Deserialize, serde::Serialize)]
pub struct TemplateApp {
    // Example stuff:
    label: String,
    value: f32,
    pres_data: Box<PresYml>,
    current_slide_index: i64,
}

impl TemplateApp {
    pub fn new(pres_yml: Box<PresYml>) -> Self {
        Self {
            label: "Hello world".to_string(),
            value: 1.0,
            pres_data: pres_yml,
            current_slide_index: 0,
        }
    }
}

enum SlideAction {
    Up,
    Down,
}

enum SlideActionResult {
    Changed(SlideAction),
    Unchanged,
}

impl TemplateApp {
    fn change_slide(&mut self, slide_action: SlideAction) -> SlideActionResult {
        //if self.pres_data.pres.slides.len() as i64 == self.current_slide_index + 1{
        //    return SlideActionResult::Unchanged;
        //}

        //}
        match slide_action {
            SlideAction::Up => {
                if self.current_slide_index+1 >= self.pres_data.pres.slides.len() as i64{
                    return SlideActionResult::Unchanged;
                }
                self.current_slide_index += 1;
                SlideActionResult::Changed(SlideAction::Up)
            }
            SlideAction::Down => {
                if self.current_slide_index-1 < 0{
                    return SlideActionResult::Unchanged;
                }
                self.current_slide_index -= 1;
                SlideActionResult::Changed(SlideAction::Down)
            }
        }
    }

    fn view_title(&self, title: &String, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.heading(title);
        });
    }


    fn view_subtitle(&self, subtitle: &String, ui: &mut egui::Ui) {
        ui.vertical_centered(|ui| {
            ui.label(subtitle);
        });
    }

    fn view_body(&self, body: &String, ui: &mut egui::Ui) {
        ui.with_layout(egui::Layout::centered_and_justified(egui::Direction::TopDown), |ui|{
            ui.small(body);
        });

        //ui.with_layout(egui::Layout::top_down(egui::Align::Max), |ui|{
        //});
    }


    fn view_slide(&mut self, ui: &mut egui::Ui) {
        let slide = &self
            .pres_data
            .pres
            .slides
            .get(self.current_slide_index as usize)
            .unwrap()
            .slide;
        let title = &slide.title;
        let subtitle = &slide.subtitle;
        let body = &slide.plain_text;
        match title {
            Some(title) => self.view_title(title, ui),
            None => (),
        };

        match subtitle {
            Some(subtitle) => self.view_subtitle(subtitle, ui),
            None => (),
        };
        
        ui.separator();
        match body {
            Some(body) => self.view_body(body, ui),
            None => (),
        };
    }
}

impl epi::App for TemplateApp {
    fn name(&self) -> &str {
        "egui template"
    }

    #[cfg(feature = "persistence")]
    fn load(&mut self, storage: &dyn epi::Storage) {
        *self = epi::get_value(storage, epi::APP_KEY).unwrap_or_default()
    }

    #[cfg(feature = "persistence")]
    fn save(&mut self, storage: &mut dyn epi::Storage) {
        epi::set_value(storage, epi::APP_KEY, self);
    }

    fn update(&mut self, ctx: &egui::CtxRef, frame: &mut epi::Frame<'_>) {
        let mut fonts = egui::FontDefinitions::default();
        fonts.family_and_size.insert(
            egui::TextStyle::Heading,
            (egui::FontFamily::Proportional, 64.0),
        );
        fonts.family_and_size.insert(
            egui::TextStyle::Body,
            (egui::FontFamily::Proportional, 32.0),
        );

        fonts.family_and_size.insert(
            egui::TextStyle::Small,
            (egui::FontFamily::Proportional, 16.0),
        );
        ctx.set_fonts(fonts);

        if ctx.input().key_released(egui::Key::ArrowRight) {
            match self.change_slide(SlideAction::Up) {
                SlideActionResult::Changed(_) => {
                    println!("Changed up {}", self.current_slide_index)
                }
                SlideActionResult::Unchanged => println!("Unchanged"),
            };
        }
        if ctx.input().key_released(egui::Key::ArrowLeft) {
            match self.change_slide(SlideAction::Down) {
                SlideActionResult::Changed(_) => {
                    println!("Changed down {}", self.current_slide_index)
                }
                SlideActionResult::Unchanged => println!("Unchanged"),
            };
        }

        egui::CentralPanel::default().show(ctx, |ui| self.view_slide(ui));
    }
}
