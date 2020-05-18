extern crate image as im;
extern crate piston_window;
extern crate find_folder;
use piston_window::*;
use self::im::DynamicImage;
//use self::im::{ImageBuffer, GenericImage};

/*
enum Ships {
    Explorer(String),
    Lander(String),
}



impl Ships {
    fn get_sprite(&self) {
}

let ship = Ships::Explorer(String::from("explorer.png"));
ship.get_sprite();

struct Ship {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64,
    ax: f64,
    ay: f64,
    angle: f64,
    sprite: Image,
}

impl ship {

    fn get_default_ship(x: f64, y: f64) {
        let ship = Ship {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            ax: 0.0,
            ay: 0.0,
            angle: 0.0,
            sprite: get_sprite(, ),
        }
    }

    fn update_position(&mut self) {
        self.x = self.v;
    }
}
*/

pub fn get_sprite(file_name: String, folder_path: String) -> DynamicImage {
    print!("we are going to load an image!");
    let img = im::open("/Users/mbarbier/Documents/prog/rust/jupiter/images/mercury_240.png").unwrap();
    print!("DONE: load an image!");
    return img;
}
/*
pub fn get_sprite(file_name: String, folder_path: String) -> Image {

    let assets = find_folder::Search::ParentsThenKids(3, 3)
        .for_folder(folder_path.as_ref()).unwrap();
    let sprite = assets.join(file_name);
    let sprite: G2dTexture = Texture::from_path(
        &mut window.create_texture_context(),
        &rust_logo,
        Flip::None,
        &TextureSettings::new()
    ).unwrap();
    return sprite;
}
*/
pub struct Planet {
    x: f64,
    y: f64,
    sprite: DynamicImage,
}

impl Planet {
    pub fn new(x: f64, y: f64, sprite: DynamicImage) -> Planet {
        Planet {
            x,
            y,
            sprite,
        }
    }
}

pub struct Game {
    pub width: u32,
    pub height: u32,
    //pub ship: Ship,
    //pub window: PistonWindow,
    pub planet: Planet,
}

impl Game {
    pub fn init_game(self) {
        let mut window: PistonWindow =
            WindowSettings::new("Jupiter", [640, 480])
                .exit_on_esc(true).build().unwrap();

        //let mut canvas = im::ImageBuffer::new(self.width, self.height);

        while let Some(event) = window.next() {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([1.0; 4], graphics);
                rectangle([1.0, 0.0, 0.0, 1.0], // red
                          [0.0, 0.0, 100.0, 100.0],
                          context.transform,
                          graphics);
            });
        }
    }
}


