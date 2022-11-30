use std::path::{Path, PathBuf};

use bevy::prelude::*;
use image::ImageError;

use crate::prelude::{Image, *};

pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Editor>();
    }
}

#[derive(Resource, Default)]
pub struct Editor {
    pub input: Image,
    pub path: Option<PathBuf>,
    pub mods: Vec<Modification>,
    pub selected_index: Option<ModifierIndex>,
}

impl Editor {
    pub fn new_from_input_path(path: impl AsRef<Path>) -> Result<Self, ImageError> {
        Ok(Self {
            input: Image::open(path.as_ref())?,
            path: Some(path.as_ref().to_path_buf()),
            ..default()
        })
    }

    pub fn export(&self, path: impl AsRef<Path>) -> Result<(), ImageError> {
        self.get_output().save(path)
    }

    pub fn get_output(&self) -> Image {
        let mut output = self.input.clone();

        for modifier in &self.mods {
            modifier.apply(&mut output);
        }

        output
    }

    pub fn receive_mod(&mut self, modifier: impl Modifier + Send + Sync + 'static) {
        let mut new = Modification::new(modifier);
        new.add_selection(CanvasSelection);
        self.mods.push(new)
    }
}