use crate::engine;
use super::{GameObject, coordinate_transform::to_gl_space, gui_element};
use serde::{Deserialize, Serialize};


#[derive(Clone, Deserialize, Serialize)]
pub struct Gui {
    pub size: (f32, f32),
    pub position: (f32, f32),
}

impl Gui {
    pub fn new(size: (f32, f32), position: (f32, f32)) -> Gui{
        Gui {
            size,
            position,
        }
    }

    pub fn add_background(&mut self, engine: &mut engine::Engine, element_name: &str, position: (f32, f32), size: (f32, f32)) {
        let pos_x = to_gl_space((position.0 / self.size.0) * self.size.0 + self.position.0);
        let pos_y = to_gl_space((position.1 / self.size.1) * self.size.1 + self.position.1);
        let size_x = (1.0 / (1000.0 / self.size.0) * (size.0 / self.size.0)) * 2.0;
        let size_y = (1.0 / (1000.0 / self.size.1) * (size.1 / self.size.1)) * 2.0;
        engine.register_render_object(element_name.to_string(), glm::vec3(pos_x, pos_y, 0.0), 
        glm::vec3(0.0, 0.0, 0.0), 0.0, glm::vec3(size_x, size_y, 1.0));
    }

    pub fn add_element(&mut self, engine: &mut engine::Engine, element_name: &str, position: (f32, f32), size: (f32, f32)) {
        let position = self.aspect_coordinates(position, engine);   
        let size = self.aspect_coordinates(size, engine);
        let pos_x = to_gl_space((position.0 / self.size.0) * self.size.0 + self.position.0);
        let pos_y = to_gl_space((position.1 / self.size.1) * self.size.1 + self.position.1);
        let size_x = (1.0 / (1000.0 / self.size.0) * (size.0 / self.size.0)) * 2.0;
        let size_y = (1.0 / (1000.0 / self.size.1) * (size.1 / self.size.1)) * 2.0;
        engine.register_render_object(element_name.to_string(), glm::vec3(pos_x, pos_y, 0.0), 
        glm::vec3(0.0, 0.0, 0.0), 0.0, glm::vec3(size_x, size_y, 1.0));
    }

    fn aspect_coordinates(&self, coordinates: (f32, f32), engine: &engine::Engine) -> (f32, f32) {
        let aspect_multiplier_x = 1000.0 / (engine.game_window.size_x as f32);
        let aspect_multiplier_y = 1000.0 / (engine.game_window.size_y as f32);
        (aspect_multiplier_x * coordinates.0, aspect_multiplier_y * coordinates.1)
    }
}