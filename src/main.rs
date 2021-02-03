use std::{cell::{RefCell, RefMut}, rc::Rc};

mod engine;
mod game;
mod resource_manager;

#[derive(Clone)]
pub struct Config {
    resource_manager: resource_manager::ResourceManager,
}

fn main() {
    /* In final executable
    let cmd_args: Vec<String> = env::args().collect();
    let asset_path: &str = &cmd_args[1];
    let config_path: &str = &cmd_args[2];
    let data_path: &str = &cmd_args[3]; */

    /* Testing */
    let asset_path: &str = "assets/";
    let config_path: &str = "config/";
    let data_path: &str = "data/";

    let mut program_config = Config {
        resource_manager: resource_manager::ResourceManager::new(
            asset_path,
            config_path,
            data_path,
        ),
    };
    program_config.resource_manager.set_world("world/");
    
    let game = game::Game::new(program_config.clone());
    let game = Rc::new(RefCell::new(game));

    let mut game_engine = engine::Engine::new(&program_config);
    //game_engine.open_title_screen();

    game.borrow_mut().load_world();

    game_engine.event_handler.register_event_object(game.clone());

    game.borrow_mut().open_screen("main_menu", &mut game_engine);

    while !game.borrow().close {
        unsafe {
            gl::ClearColor(0.6, 0.3, 0.2, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        game_engine.render_tick();
    }

    game.borrow().save_world();
}