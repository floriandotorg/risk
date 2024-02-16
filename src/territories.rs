pub const TERRITORIES: &[&'static str] = &[
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

pub const ALASKA: u8 = 00;
pub const NORTHWEST_TERRITORY: u8 = 01;
pub const GREENLAND: u8 = 02;
pub const ALBERTA: u8 = 03;
pub const ONTARIO: u8 = 04;
pub const QUEBEC: u8 = 05;
pub const WESTERN_UNITED_STATES: u8 = 06;
pub const EASTERN_UNITED_STATES: u8 = 07;
pub const CENTRAL_AMERICA: u8 = 08;
pub const VENEZUELA: u8 = 09;
pub const PERU: u8 = 10;
pub const BRAZIL: u8 = 11;
pub const ARGENTINA: u8 = 12;
pub const ICELAND: u8 = 13;
pub const SCANDINAVIA: u8 = 14;
pub const UKRAINE: u8 = 15;
pub const GREAT_BRITAIN: u8 = 16;
pub const NORTHERN_EUROPE: u8 = 17;
pub const WESTERN_EUROPE: u8 = 18;
pub const SOUTHERN_EUROPE: u8 = 19;
pub const NORTH_AFRICA: u8 = 20;
pub const EGYPT: u8 = 21;
pub const EAST_AFRICA: u8 = 22;
pub const CONGO: u8 = 23;
pub const SOUTH_AFRICA: u8 = 24;
pub const MADAGASCAR: u8 = 25;
pub const URAL: u8 = 26;
pub const SIBERIA: u8 = 27;
pub const YAKUTSK: u8 = 28;
pub const KAMCHATKA: u8 = 29;
pub const IRKUTSK: u8 = 30;
pub const MONGOLIA: u8 = 31;
pub const CHINA: u8 = 32;
pub const AFGHANISTAN: u8 = 33;
pub const MIDDLE_EAST: u8 = 34;
pub const INDIA: u8 = 35;
pub const SIAM: u8 = 36;
pub const INDONESIA: u8 = 37;
pub const NEW_GUINEA: u8 = 38;
pub const WESTERN_AUSTRALIA: u8 = 39;
pub const EASTERN_AUSTRALIA: u8 = 40;
pub const JAPAN: u8 = 41;

pub const NEIGHBORS: &[(u8, u8)] = &[
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
