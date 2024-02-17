use std::fs::File;
use std::io::BufReader;
use strum::EnumCount;
use viuer::{Config, print, };
use cairo;
use image::{DynamicImage, ImageBuffer};
use once_cell::sync::Lazy;

use crate::territories::Territory;

use super::GameState;

const TERRITORY_MAP_COORDS: &[(f64, f64); Territory::COUNT] = &[
    (45.0, 80.0), // Alaska
    (120.0, 74.0), // Northwest Territory
    (270.0, 55.0), // Greenland
    (110.0, 125.0), // Alberta
    (168.0, 128.0), // Ontario
    (220.0, 130.0), // Quebec
    (115.0, 180.0), // Western United States
    (170.0, 200.0), // Eastern United States
    (125.0, 250.0), // Central America
    (175.0, 300.0), // Venezuela
    (190.0, 370.0), // Peru
    (240.0, 360.0), // Brazil
    (200.0, 440.0), // Argentina

    (335.0, 100.0), // Iceland
    (400.0, 100.0), // Scandinavia
    (470.0, 140.0), // Ukraine
    (310.0, 160.0), // Great Britain
    (390.0, 180.0), // Northern Europe
    (335.0, 240.0), // Western Europe
    (397.0, 235.0), // Southern Europe
    (370.0, 340.0), // North Africa
    (425.0, 305.0), // Egypt
    (470.0, 380.0), // East Africa
    (425.0, 405.0), // Congo
    (430.0, 465.0), // South Africa
    (505.0, 475.0), // Madagascar

    (550.0, 135.0), // Ural
    (590.0, 80.0), // Siberia
    (655.0, 60.0), // Yakutsk
    (720.0, 65.0), // Kamchatka
    (640.0, 130.0), // Irkutsk
    (650.0, 185.0), // Mongolia
    (625.0, 235.0), // China
    (540.0, 195.0), // Afghanistan
    (490.0, 270.0), // Middle East
    (580.0, 280.0), // India
    (650.0, 310.0), // Siam
    (660.0, 400.0), // Indonesia
    (720.0, 375.0), // New Guinea
    (695.0, 475.0), // Western Australia
    (755.0, 465.0), // Eastern Australia
    (740.0, 175.0), // Japan
];

pub struct DrawMapOptions {
    filename: String,
    should_print: bool,
}

impl Default for DrawMapOptions {
    fn default() -> Self {
        Self {
            filename: "output-map.png".to_string(),
            should_print: false,
        }
    }
}

impl DrawMapOptions {
    pub fn filename(mut self, filename: &str) -> Self {
        self.filename = filename.to_string();
        self
    }

    pub fn should_print(mut self) -> Self {
        self.should_print = true;
        self
    }
}

impl GameState {
    pub fn draw_map(&self, options: DrawMapOptions) -> Result<(), cairo::Error> {
        const WIDTH: u32 = 800;
        const HEIGHT: u32 = 533;

        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, WIDTH.try_into().unwrap(), HEIGHT.try_into().unwrap())
            .expect("Can't create surface");
        {
            let cr = cairo::Context::new(&surface).unwrap();

            static MAP_IMAGE_SURFACE_DATA: Lazy<Vec<u8>> = Lazy::new(|| {
                cairo::ImageSurface::create_from_png(&mut BufReader::new(File::open("map.png").unwrap())).expect("Failed to load map image").take_data().unwrap().to_vec()
            });

            let map_image_surface = cairo::ImageSurface::create_for_data(MAP_IMAGE_SURFACE_DATA.clone(), cairo::Format::Rgb24, WIDTH as i32, HEIGHT as i32, 3200).expect("Can't create map image surface");

            cr.set_source_surface(&map_image_surface, 0.0, 0.0)?;
            cr.paint()?;
            cr.select_font_face("Purisa", cairo::FontSlant::Normal, cairo::FontWeight::Normal);
            cr.set_font_size(20.0);

            for ((x, y), t) in TERRITORY_MAP_COORDS.iter().zip(&self.territories) {
                let player = t.player;
                cr.arc(*x, *y, 20.0, 0.0, 2.0 * std::f64::consts::PI);
                let (red, green, blue) = player.color();
                cr.set_source_rgb(red, green, blue);
                cr.fill()?;

                cr.set_source_rgb(1.0, 1.0, 1.0);
                let army_text = format!("{}", t.armies);

                let text_size = cr.text_extents(&army_text)?;
                cr.move_to(*x - text_size.width() / 2.0, *y + text_size.height() / 2.0);
                cr.show_text(&army_text)?;
            }
        }

        let data = surface.take_data().expect("Can't get surface data");

        let mut rgb_data = Vec::with_capacity(WIDTH as usize * HEIGHT as usize * 3);
        for chunk in data.chunks(4) {
            rgb_data.extend_from_slice(&[chunk[2], chunk[1], chunk[0]]);
        }

        let img = DynamicImage::ImageRgb8(ImageBuffer::from_vec(WIDTH, HEIGHT, rgb_data).unwrap());

        img.save(options.filename).unwrap();

        if options.should_print {
            print(&img, &Config {
                width: Some(80),
                ..Default::default()
            }).unwrap();
        }

        Ok(())
    }
}
