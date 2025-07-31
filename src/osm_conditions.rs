use std::collections::HashMap;

// struct ist die klassendefinition
// 'a ist die lifetime-definition (solange Conditions existiert gilt der verweis zur hashmap)
// pythons gc kümmert sich dadrum automatisch
// & ist die referenz auf eine hashmap woanders
// #[derive(Clone)]

pub struct Conditions<'a> {
    pub tags: &'a HashMap<String, String>,
}

// hier beginnt die Implementierung der Methoden für die struct Conditions
impl<'a> Conditions<'a> {
    pub fn new(tags: &'a HashMap<String, String>) -> Conditions<'a> {
        Conditions { tags }
    }

    // pub fn set_infra<'b>(&mut self, infrastructure: &'b str) -> &'b str {
    //     self.tags.insert(
    //         "bicycle_infrastructure".to_string(),
    //         infrastructure.to_string(),
    //     );
    //     infrastructure
    // }

    pub fn is_segregated(&self) -> bool {
        self.tags
            .get("segregated")
            .map(|v| v == "yes")
            .unwrap_or(false)
    }

    pub fn is_footpath(&self) -> bool {
        self.tags
            .get("highway")
            .map(|v| v == "footway" || v == "pedestrian")
            .unwrap_or(false)
    }

    pub fn is_not_accessible(&self) -> bool {
        self.tags.get("access").map(|v| v == "no").unwrap_or(false)
    }

    // Beispielfunktion um zu schauen ob ein Key den Begriff "bicycle" enthält.
    // gleich wie lambda x: any(k for k, v in x.items() if 'bicycle' in k and v == 'use_sidepath')
    pub fn use_sidepath(&self) -> bool {
        self.tags
            .iter()
            .any(|(key, value)| key.contains("bicycle") && value == "use_sidepath")
    }

    pub fn is_indoor(&self) -> bool {
        self.tags.get("indoor").map(|v| v == "yes").unwrap_or(false)
    }

    pub fn is_path(&self) -> bool {
        self.tags
            .get("highway")
            .map(|v| v == "path")
            .unwrap_or(false)
    }

    pub fn is_track(&self) -> bool {
        self.tags
            .get("highway")
            .map(|v| v == "track")
            .unwrap_or(false)
    }

    // richtungsfeine condition, definiert durch direction. muss left oder right sein
    pub fn can_walk(&self, direction: &str) -> bool {
        self.tags
            .get("foot")
            .map(|v| v == "yes" || v == "designated")
            .unwrap_or(false)
            || self.tags.iter().any(|(key, value)| {
                key.contains(&(direction.to_string() + ":foot"))
                    && (value == "yes" || value == "designated")
            })
            || self
                .tags
                .get(&("sidewalk:".to_string() + direction))
                .map(|v| v == "yes" || v == "separated" || v == "both" || v == "left")
                .unwrap_or(false)
            || self
                .tags
                .get("sidewalk")
                .map(|v| {
                    v == "yes" || v == "separated" || v == "both" || v == "right" || v == "left"
                })
                .unwrap_or(false)
            || self
                .tags
                .get("sidewalk:both")
                .map(|v| v == "yes" || v == "separated" || v == "both")
                .unwrap_or(false)
    }

    pub fn can_bike(&self) -> bool {
        self.tags
            .get("bicycle")
            .map(|v| v == "yes" || v == "designated")
            .unwrap_or(false)
            && self
                .tags
                .get("highway")
                .map(|v| !v.contains("motorway"))
                .unwrap_or(false)
    }

    pub fn cannot_bike(&self) -> bool {
        self.tags
            .get("bicycle")
            .map(|v| v == "no" || v == "dismount" || v == "use_sidepath")
            .unwrap_or(false)
            || self
                .tags
                .get("highway")
                .map(|v| {
                    v == "corridor"
                        || ["motorway", "motorway_link", "trunk", "trunk_link"]
                            .contains(&v.as_str())
                        || v.contains("trunk")
                })
                .unwrap_or(false)
            || self
                .tags
                .get("access")
                .map(|v| v == "customers")
                .unwrap_or(false)
    }

    // hier ebenfalls richtungsfein. entweder left oder right
    // dazu mode: muss bicycle oder foot sein
    pub fn is_designated_bicycle(&self, direction: &str, mode: &str) -> bool {
        let sidewalk = format!("sidewalk:{}", mode);
        let cycleway = format!("cycleway:{}", mode);
        // Unused:
        // let cycleway_bicycle = format!("cycleway:{}:bicycle", direction);
        let sidewalk_directional = format!("sidewalk:{}:{}", direction, mode);
        let cycleway_directional = format!("cycleway:{}:{}", direction, mode);

        let is_designated = |key: &str| self.tags.get(key).map_or(false, |v| v == "designated");

        if mode == "foot" {
            is_designated(&sidewalk_directional)
                || is_designated("sidewalk:foot")
                || is_designated(&sidewalk)
        } else {
            is_designated(&cycleway_directional)
                || is_designated("cycleway:bicycle")
                || is_designated(&cycleway)
        }
    }

    pub fn is_designated(&self) -> bool {
        self.tags
            .get("bicycle")
            .map(|v| v == "designated")
            .unwrap_or(false)
    }

    pub fn is_service_tag(&self) -> bool {
        self.tags
            .get("highway")
            .map(|v| v == "service")
            .unwrap_or(false)
    }

    pub fn is_agricultural(&self) -> bool {
        self.tags
            .get("motor_vehicle")
            .map(|v| v == "agricultural" || v == "forestry")
            .unwrap_or(false)
    }

    pub fn is_accessible(&self) -> bool {
        match self.tags.get("access") {
            // das ist not self.is_not_accessible(x)
            Some(_) => !self.is_not_accessible(),
            // das ist pd.isnan(x.get('access') -> None ist gleich wie NaN in pandas
            None => false,
        }
    }

    pub fn is_smooth(&self) -> bool {
        match self.tags.get("tracktype") {
            Some(tracktype) => vec!["grade1", "grade2"].contains(&tracktype.as_str()),
            // das is pd.isnan(x.get('tracktype'))
            None => false,
        }
    }

    pub fn is_vehicle_allowed(&self) -> bool {
        match self.tags.get("motor_vehicle") {
            Some(motor_vehicle) if motor_vehicle != "no" => true,
            _ => false,
        }
    }

    pub fn is_service(&self) -> bool {
        let a = self.is_service_tag();
        let acc = self.is_accessible();
        let b = self.is_agricultural() && acc;
        let c = self.is_path() && acc;
        let d = self.is_track() && acc && self.is_smooth() && self.is_vehicle_allowed();
        let e = !self.is_designated();

        a || b || c || (d && e)
    }

    pub fn can_cardrive(&self) -> bool {
        match self.tags.get("highway") {
            Some(highway) => vec![
                "motorway",
                "trunk",
                "primary",
                "secondary",
                "tertiary",
                "unclassified",
                "road",
                "residential",
                "living_street",
                "primary_link",
                "secondary_link",
                "tertiary_link",
                "motorway_link",
                "trunk_link",
            ]
            .contains(&highway.as_str()),
            None => false,
        }
    }

    //lambda x: ((x["highway"] in ["cycleway", "track", "path"])
    //                                            and not self.cannot_bike(x))
    pub fn is_path_not_forbidden(&self) -> bool {
        match self.tags.get("highway") {
            Some(highway) => {
                let not_forbidden = ["cycleway", "track", "path"].contains(&highway.as_str());
                not_forbidden && !self.cannot_bike()
            }
            None => false,
        }
    }

    pub fn is_bikepath(&self, direction: &str) -> bool {
        let tag = |k: &str| self.tags.get(k).map(|v| v.as_str());

        let is_designated = |key| {
            self.tags
                .get(key)
                .map(|v| v == "designated")
                .unwrap_or(false)
        };

        let directional_keys = [format!("{}:bicycle", direction), "bicycle".to_string()];

        let is_cycleway_type =
            |key: &str| matches!(tag(key), Some("track" | "sidepath" | "crossing"));

        let cycleway_keys = [
            "cycleway",
            &format!("cycleway:{}", direction),
            "cycleway:both",
        ];

        if tag("highway") == Some("footway") && tag("segregated") != Some("yes") {
            return false;
        }

        if cycleway_keys.iter().any(|k| tag(k) == Some("no")) {
            return false;
        }

        let a = tag("highway") == Some("cycleway"); // hat die highway ein cycleway?
        let b = directional_keys.iter().any(|k| is_designated(k)); // ist der weg für ein beliebiges key designated?
        let c = !is_cycleway_type(&format!("cycleway:{}:lane", direction)); // ist die cycleway:{dir}:lane ein track, sidepath oder crossing?
        let d = !is_cycleway_type("cycleway:lane"); // ist die cycleway:lane ein track, sidepath oder crossing?
        let e = is_cycleway_type("cycleway"); // ist die cycleway ein track, sidepath oder crossing?
        let f = is_cycleway_type(&format!("cycleway:{}", direction)); // ist die cycleway:{dir} ein track, sidepath oder crossing?
        let g = is_cycleway_type("cycleway:both"); // ist die cycleway:both ein track, sidepath oder crossing?
        let h = matches!(tag(&format!("{}:traffic_sign", direction)), Some("237")); // hat die cycleway:{dir} ein VZ 237?
        let i = matches!(tag("traffic_sign"), Some("237")); // hat die cycleway ein VZ 237?

        a || (b && c && d) || e || f || g || h || i
    }

    pub fn is_pedestrian(&self, direction: &str) -> bool {
        let a = self.is_footpath();
        let b = !self.can_bike();
        let c = !self.is_indoor();
        let d = self.is_path();
        let e = self.can_walk(direction);

        (a && b && c) || (d && e && b && c)
    }

    pub fn is_cyclehighway(&self) -> bool {
        self.tags
            .get("cycle_highway")
            .map(|v| v == "yes")
            .unwrap_or(false)
    }

    pub fn is_bikeroad(&self) -> bool {
        let bicycle_road = self
            .tags
            .get("bicycle_road")
            .map(|v| v == "yes")
            .unwrap_or(false);
        let cyclestreet = self
            .tags
            .get("cyclestreet")
            .map(|v| v == "yes")
            .unwrap_or(false);

        bicycle_road || cyclestreet
    }

    //        self.is_bikelane_right = lambda x: (x.get("cycleway") in ["lane", "shared_lane"]
    //                                        or x.get("cycleway:right") in ["lane", "shared_lane"]
    //                                        or x.get("cycleway:both") in ["lane", "shared_lane"]
    //                                        or any(
    //                     key for key, value in x.items() if 'right:lane' in key and value in ['exclusive']))
    pub fn is_bikelane(&self, direction: &str) -> bool {
        let cycleway_direction = format!("cycleway:{}", direction);
        let lane_direction = format!("{}:lane", direction);

        let a = self
            .tags
            .get("cycleway")
            .map(|v| v == "lane" || v == "shared_lane")
            .unwrap_or(false);
        let b = self
            .tags
            .get(&cycleway_direction)
            .map(|v| v == "lane" || v == "shared_lane")
            .unwrap_or(false);
        let c = self
            .tags
            .get("cycleway:both")
            .map(|v| v == "lane" || v == "shared_lane")
            .unwrap_or(false);
        let d = self
            .tags
            .iter()
            .any(|(k, v)| k.contains(&lane_direction) && v == "exclusive");

        a || b || c || d
    }

    //         self.is_buslane_right = lambda x: (x.get("cycleway") == "share_busway"
    //                                       or x.get("cycleway:right") == "share_busway"
    //                                       or x.get("cycleway:both") == "share_busway")
    pub fn is_buslane(&self, direction: &str) -> bool {
        let cycleway_direction = format!("cycleway:{}", direction);

        let a = self
            .tags
            .get("cycleway")
            .map(|v| v == "share_busway")
            .unwrap_or(false);
        let b = self
            .tags
            .get(&cycleway_direction)
            .map(|v| v == "share_busway")
            .unwrap_or(false);
        let c = self
            .tags
            .get("cycleway:both")
            .map(|v| v == "share_busway")
            .unwrap_or(false);

        a || b || c
    }

    //lambda x: (
    //                 ('traffic_sign' in x.keys() and isinstance(x['traffic_sign'], str) and '241' in x['traffic_sign'])
    //                 or ('traffic_sign:forward' in x.keys() and isinstance(x['traffic_sign:forward'], str) and '241' in x[
    //             'traffic_sign:forward'])
    //         )
    pub fn is_obligated(&self) -> bool {
        self.tags
            .iter()
            .any(|(k, v)| k.contains("traffic_sign") && (v == "237" || v == "240" || v == "241"))
    }

    pub fn has_surface_values(&self, values: &[&'static str]) -> bool {
        for attribute in [
            "surface",
            "cycleway:surface",
            "cycleway:both:surface",
            "cycleway:right:surface",
            "cycleway:left:surface",
        ] {
            if let Some(value) = self.tags.get(attribute) {
                if values.contains(&value.as_str()) {
                    return true;
                }
            }
        }

        false
    }

    pub fn surface_cat1(&self) -> &[&'static str; 13] {
        &[
            "asphalt",
            "asphalt:paving_stones",
            "bricks",
            "concrete",
            "concrete:lanes",
            "concrete:plates",
            "granite:plates",
            "paved",
            "paving_stones",
            "paving_stones:50",
            "paving_stones:lanes",
            "plates",
            "tartan",
        ]
    }

    pub fn surface_cat2(&self) -> &[&'static str; 6] {
        &[
            "compacted",
            "unpaved",
            "fine_gravel",
            "gravel",
            "dirt",
            "dirt:sand",
        ]
    }

    pub fn surface_cat3(&self) -> &[&'static str; 9] {
        &[
            "asphalt:cobblestone",
            "cobblestone",
            "cobblestone:flattened",
            "grass_paver",
            "metal",
            "metal_grid",
            "sett",
            "tiles",
            "unhewn_cobblestone",
        ]
    }

    pub fn surface_cat4(&self) -> &[&'static str; 21] {
        &[
            "bare_rock",
            "bushes",
            "earth",
            "grass",
            "grass:ground",
            "grass_paver",
            "gravel:grass",
            "ground",
            "ground:grass",
            "ground:mud",
            "ground:wood",
            "mud",
            "pebblestone",
            "rock",
            "roots",
            "sand",
            "sandstone",
            "stepping_stones",
            "stone",
            "wood",
            "woodchips",
        ]
    }
}
