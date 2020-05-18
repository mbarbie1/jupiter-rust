use tetra::graphics::{self, Color, Texture, Text};
use tetra::input::{self, Key};
use tetra::math::Vec2;
use tetra::{Context, ContextBuilder, State};

const BLACK: Color = Color::rgb(0.0, 0.0, 0.0);
const WINDOW_WIDTH: f32 = 800.0;
const WINDOW_HEIGHT: f32 = 600.0;

enum Ships {
    Explorer(String),
    Lander(String),
}

struct Ship {
    x: f32,
    y: f32,
    vx: f32,
    vy: f32,
    ax: f32,
    ay: f32,
    v: f32,
    a: f32,
    a_speed: f32,
    angle: f32,
    angle_speed: f32,
    sprite: Texture,
    collision_radius: f32,
    radius: f32,
}

impl Ship {

    fn new(x: f32, y: f32, sprite: Texture) -> Ship {
        let ship = Ship {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            v: 0.0,
            a: 0.0,
            a_speed: 0.01,
            angle: 0.0,
            angle_speed: 0.1,
            sprite,
            collision_radius: 32.0,
            radius: 32.0,
        };
        return ship;
//    fn update_position(&mut self) {
//        self.x = self.v;
    }

    fn backward(&mut self) {
        self.a = - self.a_speed;
    }

    fn forward(&mut self) {
        self.a = self.a_speed;
    }

    fn turn_left(&mut self) {
        self.angle = self.angle - self.angle_speed;
    }

    fn turn_right(&mut self) {
        self.angle = self.angle + self.angle_speed;
    }

    fn update_motion(&mut self) {
        self.update_collision();
        self.update_speed();
        self.update_position();
    }

    fn update_collision(&mut self) {
        if self.x < self.collision_radius {
            self.vx = self.vx.abs();
        }
        if self.x > (WINDOW_WIDTH-self.collision_radius) {
            self.vx = -self.vx.abs();
        }
        if self.y < self.collision_radius {
            self.vy = self.vy.abs();
        }
        if self.y > (WINDOW_HEIGHT-self.collision_radius) {
            self.vy = -self.vy.abs();
        }
    }

    fn update_speed(&mut self) {
        self.vx = self.a * self.angle.cos() + self.vx ;
        self.vy = self.a * self.angle.sin() + self.vy ;
    }

    fn update_position(&mut self) {
        self.x = self.vx + self.x;
        self.y = self.vy + self.y;
    }

}

pub enum Planets {
    Mercury(String),
}

pub struct Planet {
    x: f32,
    y: f32,
    sprite: Texture,
}

impl Planet {
    pub fn new(x: f32, y: f32, sprite: Texture ) -> Planet {

        Planet {
            x,
            y,
            sprite,
        }
    }
}

struct GameState {
    planet: Planet,
    ship: Ship,
}

impl GameState {
    fn new(ctx: &mut Context) -> tetra::Result<GameState> {

        let ship_image = Texture::new(ctx, "./images/detector.png")?;
        let ship = Ship::new(
            200.0,
            200.0,
            ship_image,
        );
        let planet_image= Texture::new(ctx, "./images/mercury_240.png")?;
        let planet = Planet::new(
            500.0,
            300.0,
            planet_image,
        );
        Ok(GameState { planet, ship })
    }
}

impl State for GameState {
    fn draw(&mut self, ctx: &mut Context) -> tetra::Result {
        // Feel free to change the color to something of your choice!
        graphics::clear(ctx, BLACK);

        graphics::draw(ctx, &self.planet.sprite, Vec2::new(self.planet.x, self.planet.y));
        graphics::draw(ctx, &self.ship.sprite, Vec2::new(self.ship.x-self.ship.radius, self.ship.y-self.ship.radius));

        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> tetra::Result {

        if input::is_key_down(ctx, Key::W) {
            self.ship.backward();
        }

        if input::is_key_down(ctx, Key::S) {
            self.ship.forward();
        }

        if input::is_key_down(ctx, Key::A) {
            self.ship.turn_left();
        }

        if input::is_key_down(ctx, Key::D) {
            self.ship.turn_right();
        }

        self.ship.update_motion();

        Ok(())
    }
}


fn main() -> tetra::Result {
    ContextBuilder::new("This is Jupiter, the asteroids-like game.", WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32)
            .quit_on_escape(true)
            .build()?
            .run(GameState::new)
}


//mod space;
//
//use space::Game;
//use space::Planet;


/*
fn main() {
    println!("This is Jupiter, the asteroids-like game.");

    let image_planet = space::get_sprite(
        "moon.png".parse().unwrap(),
        "../images".parse().unwrap()
    );

    let planet = Planet::new(
        320.0,
        240.0,
        image_planet
    );

    let game = Game {
        width: 640,
        height: 480,
        planet: planet,
    };

    game.init_game();

    println!("Exit Jupiter");
}
*/