mod game_window;

pub struct Graphic {
    game_window: game_window::GameWindow,
}

impl Graphic {
    pub fn new(glfw_instance: &mut glfw::Glfw) -> Result<Self, glfw::Error> {
        Ok(Graphic {
            game_window: game_window::GameWindow::new(1920, 1080, "Fourth Age", glfw_instance)?,
        })
    }

    pub fn do_render_tick(&mut self) {
        self.game_window.update_window();
    }
}

impl Drop for Graphic {
    fn drop(&mut self) {
        drop(&self.game_window);
    }
}