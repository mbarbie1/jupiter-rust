mod space;

use space::Game;

fn main() {
    println!("This is Jupiter, the asteroids-like game.");

    let game = Game {
        width: 640,
        height: 480,
    };

    game.init_game();

    println!("Exit Jupiter");
}
