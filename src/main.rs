extern crate image;

use rand::Rng;
use std::fmt::{self, Display, Formatter};
use strum::EnumCount;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, EnumCount};
use std::fs::File;
use std::io::BufReader;
use viuer::{Config, print, };
use cairo;
use image::{DynamicImage, ImageBuffer};

const TERRITORIES: &[&'static str] = &[
    "Alaska",
    "Northwest Territory",
    "Greenland",
    "Alberta",
    "Ontario",
    "Quebec",
    "Western United States",
    "Eastern United States",
    "Central America",
    "Venezuela",
    "Peru",
    "Brazil",
    "Argentina",
    "Iceland",
    "Scandinavia",
    "Ukraine",
    "Great Britain",
    "Northern Europe",
    "Western Europe",
    "Southern Europe",
    "North Africa",
    "Egypt",
    "East Africa",
    "Congo",
    "South Africa",
    "Madagascar",
    "Ural",
    "Siberia",
    "Yakutsk",
    "Kamchatka",
    "Irkutsk",
    "Mongolia",
    "China",
    "Afghanistan",
    "Middle East",
    "India",
    "Siam",
    "Indonesia",
    "New Guinea",
    "Western Australia",
    "Eastern Australia",
    "Japan",
];

const ALASKA: u8 = 00;
const NORTHWEST_TERRITORY: u8 = 01;
const GREENLAND: u8 = 02;
const ALBERTA: u8 = 03;
const ONTARIO: u8 = 04;
const QUEBEC: u8 = 05;
const WESTERN_UNITED_STATES: u8 = 06;
const EASTERN_UNITED_STATES: u8 = 07;
const CENTRAL_AMERICA: u8 = 08;
const VENEZUELA: u8 = 09;
const PERU: u8 = 10;
const BRAZIL: u8 = 11;
const ARGENTINA: u8 = 12;
const ICELAND: u8 = 13;
const SCANDINAVIA: u8 = 14;
const UKRAINE: u8 = 15;
const GREAT_BRITAIN: u8 = 16;
const NORTHERN_EUROPE: u8 = 17;
const WESTERN_EUROPE: u8 = 18;
const SOUTHERN_EUROPE: u8 = 19;
const NORTH_AFRICA: u8 = 20;
const EGYPT: u8 = 21;
const EAST_AFRICA: u8 = 22;
const CONGO: u8 = 23;
const SOUTH_AFRICA: u8 = 24;
const MADAGASCAR: u8 = 25;
const URAL: u8 = 26;
const SIBERIA: u8 = 27;
const YAKUTSK: u8 = 28;
const KAMCHATKA: u8 = 29;
const IRKUTSK: u8 = 30;
const MONGOLIA: u8 = 31;
const CHINA: u8 = 32;
const AFGHANISTAN: u8 = 33;
const MIDDLE_EAST: u8 = 34;
const INDIA: u8 = 35;
const SIAM: u8 = 36;
const INDONESIA: u8 = 37;
const NEW_GUINEA: u8 = 38;
const WESTERN_AUSTRALIA: u8 = 39;
const EASTERN_AUSTRALIA: u8 = 40;
const JAPAN: u8 = 41;

const NEIGHBORS: &[(u8, u8)] = &[
    (ALASKA, NORTHWEST_TERRITORY),
    (ALASKA, ALBERTA),
    (ALASKA, KAMCHATKA),

    (NORTHWEST_TERRITORY, GREENLAND),
    (NORTHWEST_TERRITORY, ALBERTA),
    (NORTHWEST_TERRITORY, ONTARIO),

    (GREENLAND, ONTARIO),
    (GREENLAND, QUEBEC),
    (GREENLAND, ICELAND),

    (ALBERTA, ONTARIO),
    (ALBERTA, WESTERN_UNITED_STATES),

    (ONTARIO, QUEBEC),
    (ONTARIO, WESTERN_UNITED_STATES),
    (ONTARIO, EASTERN_UNITED_STATES),

    (QUEBEC, EASTERN_UNITED_STATES),

    (WESTERN_UNITED_STATES, EASTERN_UNITED_STATES),
    (WESTERN_UNITED_STATES, CENTRAL_AMERICA),

    (EASTERN_UNITED_STATES, CENTRAL_AMERICA),

    (CENTRAL_AMERICA, VENEZUELA),

    (VENEZUELA, PERU),
    (VENEZUELA, BRAZIL),

    (PERU, BRAZIL),
    (PERU, ARGENTINA),

    (BRAZIL, ARGENTINA),
    (BRAZIL, NORTH_AFRICA),

    //(ARGENTINA, NORTH_AFRICA),

    (ICELAND, GREAT_BRITAIN),
    (ICELAND, SCANDINAVIA),

    (SCANDINAVIA, NORTHERN_EUROPE),
    (SCANDINAVIA, UKRAINE),
    (SCANDINAVIA, GREAT_BRITAIN),

    (UKRAINE, NORTHERN_EUROPE),
    (UKRAINE, SOUTHERN_EUROPE),
    (UKRAINE, MIDDLE_EAST),
    (UKRAINE, AFGHANISTAN),
    (UKRAINE, URAL),

    (GREAT_BRITAIN, NORTHERN_EUROPE),
    (GREAT_BRITAIN, WESTERN_EUROPE),

    (NORTHERN_EUROPE, SOUTHERN_EUROPE),
    (NORTHERN_EUROPE, WESTERN_EUROPE),

    (WESTERN_EUROPE, SOUTHERN_EUROPE),
    (WESTERN_EUROPE, NORTH_AFRICA),

    (SOUTHERN_EUROPE, MIDDLE_EAST),
    (SOUTHERN_EUROPE, EGYPT),
    (SOUTHERN_EUROPE, NORTH_AFRICA),

    (NORTH_AFRICA, EGYPT),
    (NORTH_AFRICA, EAST_AFRICA),
    (NORTH_AFRICA, CONGO),

    (EGYPT, MIDDLE_EAST),
    (EGYPT, EAST_AFRICA),

    (EAST_AFRICA, CONGO),
    (EAST_AFRICA, SOUTH_AFRICA),
    (EAST_AFRICA, MADAGASCAR),
    //(EAST_AFRICA, MIDDLE_EAST), // "the missing link in the 40th Anniversary Collector's Edition"

    (CONGO, SOUTH_AFRICA),

    (SOUTH_AFRICA, MADAGASCAR),

    (URAL, SIBERIA),
    (URAL, CHINA),
    (URAL, AFGHANISTAN),

    (SIBERIA, YAKUTSK),
    (SIBERIA, IRKUTSK),
    (SIBERIA, MONGOLIA),
    (SIBERIA, CHINA),

    (YAKUTSK, KAMCHATKA),
    (YAKUTSK, IRKUTSK),

    (KAMCHATKA, IRKUTSK),
    (KAMCHATKA, MONGOLIA),
    (KAMCHATKA, JAPAN),

    (IRKUTSK, MONGOLIA),

    (MONGOLIA, CHINA),
    (MONGOLIA, JAPAN),

    (CHINA, AFGHANISTAN),
    (CHINA, INDIA),
    (CHINA, SIAM),

    (AFGHANISTAN, MIDDLE_EAST),
    (AFGHANISTAN, INDIA),

    (MIDDLE_EAST, INDIA),

    (INDIA, SIAM),

    (SIAM, INDONESIA),

    (INDONESIA, NEW_GUINEA),
    (INDONESIA, WESTERN_AUSTRALIA),

    (NEW_GUINEA, WESTERN_AUSTRALIA),
    (NEW_GUINEA, EASTERN_AUSTRALIA),

    (WESTERN_AUSTRALIA, EASTERN_AUSTRALIA),
];

#[derive(Debug, Clone, Copy, PartialEq, Eq, EnumIter, EnumCount)]
enum Player {
    A,
    B,
}

impl Player {
    fn next(self) -> Self {
        match self {
            Player::A => Player::B,
            Player::B => Player::A,
        }
    }

    fn color(self) -> (f64, f64, f64) {
        match self {
            Player::A => (0.0, 0.0, 1.0),
            Player::B => (1.0, 0.0, 0.0),
        }
    }
}

#[derive(Debug, Clone)]
struct TerritoryStateDuringInitialPlacement {
    player: Option<Player>,
    troops: u8,
}

#[derive(Debug)]
struct TerritoryState {
    player: Player,
    troops: u8,
}

#[derive(Debug)]
struct GameStateDuringInitialPlacement {
    current_player: Player,
    territories: [TerritoryStateDuringInitialPlacement; TERRITORIES.len()]
}

#[derive(Debug)]
struct GameState {
    current_player: Player,
    territories: [TerritoryState; TERRITORIES.len()]
}

const TERRITORY_MAP_COORDS: &[(f64, f64)] = &[
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

impl GameState {
    fn draw_map_with_filename(&self, filename: &str) -> Result<(), cairo::Error> {
        let width: u32 = 800;
        let height: u32 = 533;

        let map_surface = cairo::ImageSurface::create_from_png(&mut BufReader::new(File::open("map.png").unwrap()))
        .expect("Failed to load map image");

        let surface = cairo::ImageSurface::create(cairo::Format::ARgb32, width.try_into().unwrap(), height.try_into().unwrap())
            .expect("Can't create surface");
        {
            let cr = cairo::Context::new(&surface).unwrap();

            cr.set_source_surface(&map_surface, 0.0, 0.0)?;
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
                let troop_text = format!("{}", t.troops);

                let text_size = cr.text_extents(&troop_text)?;
                cr.move_to(*x - text_size.width() / 2.0, *y + text_size.height() / 2.0);
                cr.show_text(&troop_text)?;
            }
        }

        let data = surface.take_data().expect("Can't get surface data");

        let mut rgb_data = Vec::with_capacity(width as usize * height as usize * 3);
        for chunk in data.chunks(4) {
            rgb_data.extend_from_slice(&[chunk[2], chunk[1], chunk[0]]);
        }

        let img = DynamicImage::ImageRgb8(ImageBuffer::from_vec(width, height, rgb_data).unwrap());

        img.save(filename).unwrap();

        print(&img, &Config {
            width: Some(80),
            ..Default::default()
        }).unwrap();

        Ok(())
    }

    fn draw_map(&self) -> Result<(), cairo::Error> {
        self.draw_map_with_filename("output-map.png")
    }
}

impl Display for GameState {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        writeln!(f, "Current player: {:?}", self.current_player)?;
        for (territory, state) in TERRITORIES.iter().zip(self.territories.iter()) {
            writeln!(f, "{:24} - {:?} {}", territory, state.player, state.troops)?;
        }
        Ok(())
    }
}

impl GameStateDuringInitialPlacement {
    const STARTING_PLAYER: Player = Player::A;

    fn new() -> Self {
        Self {
            current_player: Self::STARTING_PLAYER,
            territories: TERRITORIES.iter().map(|_| TerritoryStateDuringInitialPlacement { player: None, troops: 0 }).collect::<Vec<_>>().try_into().unwrap()
        }
    }

    fn start(&self) -> GameState {
        GameState {
            current_player: Self::STARTING_PLAYER,
            territories: self.territories.iter().map(|t| TerritoryState { player: t.player.unwrap(), troops: t.troops}).collect::<Vec<_>>().try_into().unwrap()
        }
    }

    fn place_random(&self) -> GameStateDuringInitialPlacement {
        let mut territories = self.territories.clone();
        let mut active_player = self.current_player;
        let mut rng = rand::thread_rng();

        let mut troops = [0; Player::COUNT];
        let mut territories_per_player = [0; Player::COUNT];

        // Place all players
        loop {
            let number_of_unclaimed_territories = territories.iter().filter(|t| t.player.is_none()).count();
            if number_of_unclaimed_territories < 1 {
                break;
            }

            let mut random_territory = rng.gen_range(0..number_of_unclaimed_territories);

            for t in territories.iter_mut() {
                if t.player.is_none() {
                    if random_territory == 0 {
                        t.player = Some(active_player);
                        t.troops = 1;

                        troops[active_player as usize] += 1;
                        territories_per_player[active_player as usize] += 1;
                        break;
                    }

                    random_territory -= 1;
                }
            }
            active_player = active_player.next();
        }

        // Place remaining troops
        const TROOP_COUNT: usize = 40;

        for player in Player::iter() {
            let mut remaining_troops = TROOP_COUNT - troops[player as usize];
            let players_territories = territories_per_player[player as usize];

            while remaining_troops > 0 {
                let mut random_territory = rng.gen_range(0..players_territories);

                for t in territories.iter_mut() {
                    if t.player == Some(player) {
                        if random_territory == 0 {
                            t.troops += 1;
                            remaining_troops -= 1;
                            break;
                        }

                        random_territory -= 1;
                    }
                }

                assert!(random_territory == 0);
            }
        }

        GameStateDuringInitialPlacement {
            current_player: active_player,
            territories
        }
    }
}


fn main() {
    let state = GameStateDuringInitialPlacement::new().place_random().start();
    println!("{}", state);
    state.draw_map().expect("Failed to draw map");
    check_neighbors();
}

fn check_neighbors() {
    // Check if there are any territories without a neighbor
    for (idx, name) in TERRITORIES.iter().enumerate() {
        let mut found = false;
        let idx = idx as u8;
        for (s, e) in NEIGHBORS.iter() {
            if *s == idx || *e == idx {
                found = true;
                break;
            }
        }
        assert!(found, "Territory {} had no neighbor", name);
    }

    // Check if there are any connections which are in both directions
    for (idx, (s, e)) in NEIGHBORS[..NEIGHBORS.len() - 1].iter().enumerate() {
        for (idx2, (s2, e2)) in NEIGHBORS[(idx + 1)..].iter().enumerate() {
            let is_equal = (*s2 == *s && *e2 == *e) || (*s2 == *e && *e2 == *s);
            assert!(!is_equal, "Connection between s={} and e={} (s2={}, e2={}) exists at least twice (idx {} and {})", s, e, s2, e2, idx, idx + idx2 + 1);
        }
    }
}
