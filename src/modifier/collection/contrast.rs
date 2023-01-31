use egui::Ui;

use crate::{
    editor::Editor,
    image::Image,
    modifier::{modification::CacheOutput, traits::Modifier},
};

#[derive(Clone, Default, PartialEq)]
pub struct Contrast {
    value: f32,
}

impl Modifier for Contrast {
    fn apply(&mut self, mut input: CacheOutput) -> Option<Image> {
        if let Some(image) = &mut input.image {
            image.contrast(self.value);
        }
        input.image
    }

    fn view(&mut self, ui: &mut Ui, _: &mut Editor) {
        ui.horizontal(|ui| {
            ui.label("amount:");
            ui.add(egui::DragValue::new(&mut self.value).clamp_range(-100.0..=f32::MAX));
        });
    }
}
