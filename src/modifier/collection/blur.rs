use egui::Ui;

use crate::{
    editor::Editor,
    modifier::{cation::Output, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Blur {
    sigma: f32,
}

impl Modifier for Blur {
    fn apply(&mut self, input: &mut Output) {
        if let Some(image) = &mut input.image {
            image.blur(self.sigma);
        }
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("sigma:");
            ui.add(
                egui::DragValue::new(&mut self.sigma)
                    .speed(0.01)
                    .clamp_range(0.01..=f32::MAX),
            );
        });
    }
}
