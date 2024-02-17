use num_enum::TryFromPrimitive;
use strum_macros::{EnumCount, EnumIter};

#[repr(u8)]
#[derive(PartialEq, Eq, Clone, Copy, Debug, EnumCount, TryFromPrimitive)]
#[cfg_attr(test, derive(EnumIter))]
pub enum Territory {
    Alaska = 00,
    NorthwestTerritory = 01,
    Greenland = 02,
    Alberta = 03,
    Ontario = 04,
    Quebec = 05,
    WesternUnitedStates = 06,
    EasternUnitedStates = 07,
    CentralAmerica = 08,
    Venezuela = 09,
    Peru = 10,
    Brazil = 11,
    Argentina = 12,
    Iceland = 13,
    Scandinavia = 14,
    Ukraine = 15,
    GreatBritain = 16,
    NorthernEurope = 17,
    WesternEurope = 18,
    SouthernEurope = 19,
    NorthAfrica = 20,
    Egypt = 21,
    EastAfrica = 22,
    Congo = 23,
    SouthAfrica = 24,
    Madagascar = 25,
    Ural = 26,
    Siberia = 27,
    Yakutsk = 28,
    Kamchatka = 29,
    Irkutsk = 30,
    Mongolia = 31,
    China = 32,
    Afghanistan = 33,
    MiddleEast = 34,
    India = 35,
    Siam = 36,
    Indonesia = 37,
    NewGuinea = 38,
    WesternAustralia = 39,
    EasternAustralia = 40,
    Japan = 41,
}

#[derive(EnumIter, PartialEq, Eq, Clone, Copy)]
pub enum Continent {
    NorthAmerica,
    SouthAmerica,
    Europe,
    Asia,
    Africa,
    Oceania
}

pub const NEIGHBORS: &[(Territory, Territory)] = &[
    (Territory::Alaska, Territory::NorthwestTerritory),
    (Territory::Alaska, Territory::Alberta),
    (Territory::Alaska, Territory::Kamchatka),

    (Territory::NorthwestTerritory, Territory::Greenland),
    (Territory::NorthwestTerritory, Territory::Alberta),
    (Territory::NorthwestTerritory, Territory::Ontario),

    (Territory::Greenland, Territory::Ontario),
    (Territory::Greenland, Territory::Quebec),
    (Territory::Greenland, Territory::Iceland),

    (Territory::Alberta, Territory::Ontario),
    (Territory::Alberta, Territory::WesternUnitedStates),

    (Territory::Ontario, Territory::Quebec),
    (Territory::Ontario, Territory::WesternUnitedStates),
    (Territory::Ontario, Territory::EasternUnitedStates),

    (Territory::Quebec, Territory::EasternUnitedStates),

    (Territory::WesternUnitedStates, Territory::EasternUnitedStates),
    (Territory::WesternUnitedStates, Territory::CentralAmerica),

    (Territory::EasternUnitedStates, Territory::CentralAmerica),

    (Territory::CentralAmerica, Territory::Venezuela),

    (Territory::Venezuela, Territory::Peru),
    (Territory::Venezuela, Territory::Brazil),

    (Territory::Peru, Territory::Brazil),
    (Territory::Peru, Territory::Argentina),

    (Territory::Brazil, Territory::Argentina),
    (Territory::Brazil, Territory::NorthAfrica),

    //(Territory::Argentina, Territory::NorthAfrica),

    (Territory::Iceland, Territory::GreatBritain),
    (Territory::Iceland, Territory::Scandinavia),

    (Territory::Scandinavia, Territory::NorthernEurope),
    (Territory::Scandinavia, Territory::Ukraine),
    (Territory::Scandinavia, Territory::GreatBritain),

    (Territory::Ukraine, Territory::NorthernEurope),
    (Territory::Ukraine, Territory::SouthernEurope),
    (Territory::Ukraine, Territory::MiddleEast),
    (Territory::Ukraine, Territory::Afghanistan),
    (Territory::Ukraine, Territory::Ural),

    (Territory::GreatBritain, Territory::NorthernEurope),
    (Territory::GreatBritain, Territory::WesternEurope),

    (Territory::NorthernEurope, Territory::SouthernEurope),
    (Territory::NorthernEurope, Territory::WesternEurope),

    (Territory::WesternEurope, Territory::SouthernEurope),
    (Territory::WesternEurope, Territory::NorthAfrica),

    (Territory::SouthernEurope, Territory::MiddleEast),
    (Territory::SouthernEurope, Territory::Egypt),
    (Territory::SouthernEurope, Territory::NorthAfrica),

    (Territory::NorthAfrica, Territory::Egypt),
    (Territory::NorthAfrica, Territory::EastAfrica),
    (Territory::NorthAfrica, Territory::Congo),

    (Territory::Egypt, Territory::MiddleEast),
    (Territory::Egypt, Territory::EastAfrica),

    (Territory::EastAfrica, Territory::Congo),
    (Territory::EastAfrica, Territory::SouthAfrica),
    (Territory::EastAfrica, Territory::Madagascar),
    (Territory::EastAfrica, Territory::MiddleEast), // "the missing link in the 40th Anniversary Collector's Edition"

    (Territory::Congo, Territory::SouthAfrica),

    (Territory::SouthAfrica, Territory::Madagascar),

    (Territory::Ural, Territory::Siberia),
    (Territory::Ural, Territory::China),
    (Territory::Ural, Territory::Afghanistan),

    (Territory::Siberia, Territory::Yakutsk),
    (Territory::Siberia, Territory::Irkutsk),
    (Territory::Siberia, Territory::Mongolia),
    (Territory::Siberia, Territory::China),

    (Territory::Yakutsk, Territory::Kamchatka),
    (Territory::Yakutsk, Territory::Irkutsk),

    (Territory::Kamchatka, Territory::Irkutsk),
    (Territory::Kamchatka, Territory::Mongolia),
    (Territory::Kamchatka, Territory::Japan),

    (Territory::Irkutsk, Territory::Mongolia),

    (Territory::Mongolia, Territory::China),
    (Territory::Mongolia, Territory::Japan),

    (Territory::China, Territory::Afghanistan),
    (Territory::China, Territory::India),
    (Territory::China, Territory::Siam),

    (Territory::Afghanistan, Territory::MiddleEast),
    (Territory::Afghanistan, Territory::India),

    (Territory::MiddleEast, Territory::India),

    (Territory::India, Territory::Siam),

    (Territory::Siam, Territory::Indonesia),

    (Territory::Indonesia, Territory::NewGuinea),
    (Territory::Indonesia, Territory::WesternAustralia),

    (Territory::NewGuinea, Territory::WesternAustralia),
    (Territory::NewGuinea, Territory::EasternAustralia),

    (Territory::WesternAustralia, Territory::EasternAustralia),
];

impl Territory {
    pub fn continent(self) -> Continent {
        match self {
            Territory::Alaska => Continent::NorthAmerica,
            Territory::NorthwestTerritory => Continent::NorthAmerica,
            Territory::Greenland => Continent::NorthAmerica,
            Territory::Alberta => Continent::NorthAmerica,
            Territory::Ontario => Continent::NorthAmerica,
            Territory::Quebec => Continent::NorthAmerica,
            Territory::WesternUnitedStates => Continent::NorthAmerica,
            Territory::EasternUnitedStates => Continent::NorthAmerica,
            Territory::CentralAmerica => Continent::NorthAmerica,
            Territory::Venezuela => Continent::SouthAmerica,
            Territory::Peru => Continent::SouthAmerica,
            Territory::Brazil => Continent::SouthAmerica,
            Territory::Argentina => Continent::SouthAmerica,
            Territory::Iceland => Continent::Europe,
            Territory::Scandinavia => Continent::Europe,
            Territory::Ukraine => Continent::Europe,
            Territory::GreatBritain => Continent::Europe,
            Territory::NorthernEurope => Continent::Europe,
            Territory::WesternEurope => Continent::Europe,
            Territory::SouthernEurope => Continent::Europe,
            Territory::NorthAfrica => Continent::Africa,
            Territory::Egypt => Continent::Africa,
            Territory::EastAfrica => Continent::Africa,
            Territory::Congo => Continent::Africa,
            Territory::SouthAfrica => Continent::Africa,
            Territory::Madagascar => Continent::Africa,
            Territory::Ural => Continent::Asia,
            Territory::Siberia => Continent::Asia,
            Territory::Yakutsk => Continent::Asia,
            Territory::Kamchatka => Continent::Asia,
            Territory::Irkutsk => Continent::Asia,
            Territory::Mongolia => Continent::Asia,
            Territory::China => Continent::Asia,
            Territory::Afghanistan => Continent::Asia,
            Territory::MiddleEast => Continent::Asia,
            Territory::India => Continent::Asia,
            Territory::Siam => Continent::Asia,
            Territory::Indonesia => Continent::Oceania,
            Territory::NewGuinea => Continent::Oceania,
            Territory::WesternAustralia => Continent::Oceania,
            Territory::EasternAustralia => Continent::Oceania,
            Territory::Japan => Continent::Asia,
        }
    }

    pub fn neighbors(self) -> Vec<Territory> {
        let mut result = vec![];
        for &(start, end) in NEIGHBORS {
            if start == self {
                result.push(end);
            } else if end == self {
                result.push(start);
            }
        }
        result
    }

    pub fn neighboring(self, neighbor: Territory) -> bool {
        NEIGHBORS.iter().any(|&(neighbor_start, neighbor_end)| (neighbor_start == self && neighbor_end == neighbor) || (neighbor_start == neighbor && neighbor_end == self))
    }
}

impl std::fmt::Display for Territory {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Territory::Alaska => "Alaska",
            Territory::NorthwestTerritory => "Northwest Territory",
            Territory::Greenland => "Greenland",
            Territory::Alberta => "Alberta",
            Territory::Ontario => "Ontario",
            Territory::Quebec => "Quebec",
            Territory::WesternUnitedStates => "Western United States",
            Territory::EasternUnitedStates => "Eastern United States",
            Territory::CentralAmerica => "Central America",
            Territory::Venezuela => "Venezuela",
            Territory::Peru => "Peru",
            Territory::Brazil => "Brazil",
            Territory::Argentina => "Argentina",
            Territory::Iceland => "Iceland",
            Territory::Scandinavia => "Scandinavia",
            Territory::Ukraine => "Ukraine",
            Territory::GreatBritain => "Great Britain",
            Territory::NorthernEurope => "Northern Europe",
            Territory::WesternEurope => "Western Europe",
            Territory::SouthernEurope => "Southern Europe",
            Territory::NorthAfrica => "North Africa",
            Territory::Egypt => "Egypt",
            Territory::EastAfrica => "East Africa",
            Territory::Congo => "Congo",
            Territory::SouthAfrica => "South Africa",
            Territory::Madagascar => "Madagascar",
            Territory::Ural => "Ural",
            Territory::Siberia => "Siberia",
            Territory::Yakutsk => "Yakutsk",
            Territory::Kamchatka => "Kamchatka",
            Territory::Irkutsk => "Irkutsk",
            Territory::Mongolia => "Mongolia",
            Territory::China => "China",
            Territory::Afghanistan => "Afghanistan",
            Territory::MiddleEast => "Middle East",
            Territory::India => "India",
            Territory::Siam => "Siam",
            Territory::Indonesia => "Indonesia",
            Territory::NewGuinea => "New Guinea",
            Territory::WesternAustralia => "Western Australia",
            Territory::EasternAustralia => "Eastern Australia",
            Territory::Japan => "Japan",
        })
    }
}

#[cfg(test)]
mod tests {
    use strum::IntoEnumIterator;

    use crate::territories::NEIGHBORS;

    use super::Territory;

    #[test]
    fn has_neighbors() {
        // Check if there are any territories without a neighbor
        for name in Territory::iter() {
            let mut found = false;
            for (s, e) in NEIGHBORS.iter() {
                if *s == name || *e == name {
                    found = true;
                    break;
                }
            }
            assert!(found, "Territory {} had no neighbor", name);
        }
    }

    #[test]
    fn check_neighbors() {
        // Check if there are any connections which are in both directions
        for (idx, (s, e)) in NEIGHBORS[..NEIGHBORS.len() - 1].iter().enumerate() {
            for (idx2, (s2, e2)) in NEIGHBORS[(idx + 1)..].iter().enumerate() {
                let is_equal = (*s2 == *s && *e2 == *e) || (*s2 == *e && *e2 == *s);
                assert!(!is_equal, "Connection between s={} and e={} (s2={}, e2={}) exists at least twice (idx {} and {})", s, e, s2, e2, idx, idx + idx2 + 1);
            }
        }
    }
}
