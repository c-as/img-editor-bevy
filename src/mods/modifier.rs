use crate::prelude::{Image, *};

pub struct Modification {
    pub modifier: Box<dyn Modifier + Send + Sync>,
    pub index: ModifierIndex,
    pub selection: Vec<Selection>,
    pub cache: Option<Image>,
    pub id: usize,
}

impl Modification {
    pub fn new<M>(modifier: M) -> Self
    where
        M: Modifier + Default + Send + Sync + 'static,
    {
        Self {
            modifier: Box::new(modifier),
            index: M::get_index(),
            selection: Vec::new(),
            cache: None,
            id: rand::random(),
        }
    }

    pub fn add_selection<S>(&mut self, selection: S)
    where
        S: Selector + Default + Send + Sync + 'static,
    {
        self.selection.push(Selection {
            selector: Box::new(selection),
            index: S::get_index(),
        });
    }

    pub fn apply(&mut self, mut output: &mut Image) {
        if let Some(cached) = &self.cache {
            *output = cached.clone();
        } else {
            let mut modifier_state = dyn_clone::clone_box(&self.modifier);
            for selection in self.selection.iter() {
                for position in selection.selector.get_pixels(&output) {
                    if let Some(color) = modifier_state.get_pixel(position, &mut output) {
                        output.set_pixel(position, color).unwrap();
                    }
                }
            }

            self.cache = Some(output.clone());
        }
    }
}
