mod game;
use crate::game::Game;
// use winit::{event::*, event_loop::{ControlFlow, EventLoop}, window::WindowBuilder};


fn main() {

    let game: Game = Game{};
    game.initialize();
    //game.run();
    //game.destroy();

}
