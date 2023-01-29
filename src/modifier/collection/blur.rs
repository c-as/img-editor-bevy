use egui::Ui;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{modification::CacheOutput, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Blur {
    sigma: f32,
}

impl Modifier for Blur {
    fn apply(&mut self, mut input: CacheOutput) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.blur(self.sigma);
        }
        input.image
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
