use std::collections::HashMap;

// struct ist die klassendefinition
// 'a ist die lifetime-definition (solange Conditions existiert gilt der verweis zur hashmap)
// pythons gc kümmert sich dadrum automatisch
// & ist die referenz auf eine hashmap woanders
// #[derive(Clone)]

pub(crate) struct Conditions<'a> {
    pub(crate) tags: &'a mut HashMap<String, String>
}

// hier beginnt die Implementierung der Methoden für die struct Conditions
impl<'a> Conditions<'a> {
    pub(crate) fn new(tags: &'a mut HashMap<String, String>) -> Conditions<'a> {
        Conditions { tags }
    }
    pub fn set_infra<'b>(&mut self, infrastructure: &'b str) -> &'b str {
        self.tags.insert("bicycle_infrastructure".to_string(), infrastructure.to_string());
        infrastructure
    }
    pub fn is_segregated(&self) -> bool {
        self.tags.get("segregated").map(|v| v == "yes").unwrap_or(false)
    }

    pub fn is_footpath(&self) -> bool {
        self.tags.get("highway").map(|v| v == "footway" || v == "pedestrian").unwrap_or(false)
    }

    pub fn is_not_accessible(&self) -> bool {
        self.tags.get("access").map(|v| v == "no").unwrap_or(false)
    }

    // Beispielfunktion um zu schauen ob ein Key den Begriff "bicycle" enthält.
    // gleich wie lambda x: any(k for k, v in x.items() if 'bicycle' in k and v == 'use_sidepath')
    pub fn use_sidepath(&self) -> bool {
        self.tags.iter().any(|(key, value)| key.contains("bicycle") && value == "use_sidepath")
    }

    pub fn is_indoor(&self) -> bool {
        self.tags.get("indoor").map(|v| v == "yes").unwrap_or(false)
    }

    pub fn is_path(&self) -> bool {
        self.tags.get("highway").map(|v| v == "path").unwrap_or(false)
    }

    pub fn is_track(&self) -> bool {
        self.tags.get("highway").map(|v| v == "track").unwrap_or(false)
    }

    // richtungsfeine condition, definiert durch direction. muss left oder right sein
    pub fn can_walk(&self, direction: &str) -> bool {
        self.tags.get("foot").map(|v| v == "yes" || v == "designated").unwrap_or(false)
            || self.tags.iter().any(|(key, value)| key.contains(&(direction.to_string()+":foot")) && (value == "yes" || value == "designated"))
            || self.tags.get(&("sidewalk:".to_string() + direction)).map(|v| v == "yes" || v == "separated" || v == "both" || v == "left").unwrap_or(false)
            || self.tags.get("sidewalk").map(|v| v == "yes" || v == "separated" || v == "both" || v == "right" || v == "left").unwrap_or(false)
            || self.tags.get("sidewalk:both").map(|v| v == "yes" || v == "separated" || v == "both").unwrap_or(false)
    }

    pub fn can_bike(&self) -> bool {
        self.tags.get("bicycle").map(|v| v == "yes" || v == "designated").unwrap_or(false)
            && self.tags.get("highway").map(|v| v.contains("motorway")).unwrap_or(false)
    }

    pub fn cannot_bike(&self) -> bool {
        self.tags.get("bicycle").map(|v| v == "no" || v == "dismount" || v == "use_sidepath").unwrap_or(false)
            || self.tags.get("highway").map(|v| v == "corridor" || v.contains("motorway") || v.contains("trunk")).unwrap_or(false)
            || self.tags.get("access").map(|v| v == "customers").unwrap_or(false)

    }

    // hier ebenfalls richtungsfein. entweder left oder right
    // dazu mode: muss bicycle oder foot sein
    pub fn is_designated_bicycle(&self, direction: &str, mode: &str) -> bool {
        let sidewalk = format!("sidewalk:{}", mode);
        let cycleway = format!("cycleway:{}", mode);
        let sidewalk_directional = format!("sidewalk:{}:{}", direction, mode);
        let cycleway_directional = format!("cycleway:{}:{}", direction, mode);

        let is_designated = |key: &str| {
            self.tags.get(key).map_or(false, |v| v == "designated")
        };

        if mode == "foot" {
            is_designated(&sidewalk_directional) || is_designated("sidewalk:foot") || is_designated(&sidewalk)
        } else {
            is_designated(&cycleway_directional) || is_designated("cycleway:bicycle") || is_designated(&cycleway)
        }
    }

    pub fn is_designated(&self) -> bool {
        self.tags.get("bicycle").map(|v| v == "designated").unwrap_or(false)
    }

    pub fn is_service_tag(&self) -> bool {
        self.tags.get("highway").map(|v| v == "service").unwrap_or(false)
    }

    pub fn is_agricultural(&self) -> bool {
        self.tags.get("motor_vehicle").map(|v| v == "agricultural" || v == "forestry").unwrap_or(false)
    }

    pub fn is_accessible(&self) -> bool {
        match self.tags.get("access") {
            // das ist not self.is_not_accessible(x)
            Some(_) => !self.is_not_accessible(),
            // das ist pd.isnan(x.get('access') -> None ist gleich wie NaN in pandas
            None => false
        }
    }

    pub fn is_smooth(&self) -> bool {
        match self.tags.get("tracktype") {
            Some(tracktype) => vec!["grade1", "grade2"].contains(&tracktype.as_str()),
            // das is pd.isnan(x.get('tracktype'))
            None => false
        }

    }

    pub fn is_vehicle_allowed(&self) -> bool {
        match self.tags.get("motor_vehicle") {
            Some(motor_vehicle) if motor_vehicle != "no" => true,
            _ => false
        }
    }

    pub fn is_service(&self) -> bool {
        self.is_service_tag()
            || (self.is_agricultural() && self.is_accessible())
            || (self.is_path() && self.is_accessible())
            || (self.is_track() && self.is_accessible() && self.is_smooth() && self.is_vehicle_allowed())
            && !self.is_designated()
    }

    pub fn can_cardrive(&self) -> bool {
        match self.tags.get("highway") {
            Some(highway) => vec!["motorway", "trunk", "primary", "secondary", "tertiary",
                                  "unclassified", "road",
                                  "residential", "living_street",
                                  "primary_link", "secondary_link", "tertiary_link",
                                  "motorway_link", "trunk_link"]
                .contains(&highway.as_str()),
            None => false
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
            None => false
        }
    }

    //lambda x: (x.get("highway") in ["cycleway"]
    //                                        or (any(
    //                     key for key, value in x.items() if 'right:bicycle' in key and value in ['designated'])
    //                                            and not any(key for key, value in x.items() if key == 'cycleway:right:lane'))
    //                                        or x.get("cycleway") in ["track", "sidepath", "crossing"]
    //                                        or x.get("cycleway:right") in ["track", "sidepath", "crossing"]
    //                                        or x.get("cycleway:both") in ["track", "sidepath", "crossing"]
    //                                        or any(
    //                     key for key, value in x.items() if 'right:traffic_sign' in key and value in ['237']))

    pub fn is_bikepath(&self, direction: &str) -> bool {
        let bicycle_directional: String = format!("{}:bicycle", direction);
        let cycleway_directional: String = format!("cycleway:{}", direction);
        let lane_directional: String = format!("cycleway:{}:lane", direction);
        let trafficsign_directional: String = format!("{}:traffic_sign", direction);
        //let cycleway_conditions: = vec!["track", "sidepath", "crossing"]

        self.tags.get("highway").map(|v| v == "cycleway").unwrap_or(false)
            || ((self.tags.iter().any(|(k, v)| k.contains(&bicycle_directional) || k.contains("bicycle") && v == "designated")
            && !self.tags.iter().any(|(k,v)|  k.contains(&lane_directional) || k.contains("cycleway:lane")))
            || self.tags.get("cycleway").map(|v| v == "track" || v == "sidepath" || v == "crossing").unwrap_or(false)
            || self.tags.get(&cycleway_directional).map(|v| v == "track" || v == "sidepath" || v == "crossing").unwrap_or(false)
            || self.tags.get("cycleway:both").map(|v| v == "track" || v == "sidepath" || v == "crossing").unwrap_or(false)
            || self.tags.iter().any(|(k,v)| k.contains(&trafficsign_directional) || k.contains("traffic_sign") && v == "237"))
    }

    //        self.is_pedestrian_right = lambda x: ((self.is_footpath(x) and not self.can_bike(x) and not self.is_indoor(x))
    //                                           or (self.is_path(x) and self.can_walk_right(x) and not self.can_bike(x) and not self.is_indoor(x)))
    pub fn is_pedestrian(&self, direction: &str) -> bool {
        (self.is_footpath()
            && !self.can_bike()
            && !self.is_indoor())
            || (self.is_path()
            && self.can_walk(direction)
            && !self.can_bike()
            && !self.is_indoor())
    }

    pub fn is_cyclehighway(&self) -> bool {
        self.tags.get("cycle_highway").map(|v| v == "yes").unwrap_or(false)
    }

    pub fn is_bikeroad(&self) -> bool {
        self.tags.get("bicycle_road").map(|v| v == "yes").unwrap_or(false)
            || self.tags.get("cyclestreet").map(|v| v == "yes").unwrap_or(false)
    }

    //        self.is_bikelane_right = lambda x: (x.get("cycleway") in ["lane", "shared_lane"]
    //                                        or x.get("cycleway:right") in ["lane", "shared_lane"]
    //                                        or x.get("cycleway:both") in ["lane", "shared_lane"]
    //                                        or any(
    //                     key for key, value in x.items() if 'right:lane' in key and value in ['exclusive']))
    pub fn is_bikelane(&self, direction: &str) -> bool {
        let cycleway_direction = format!("cycleway:{}", direction);
        let lane_direction = format!("{}:lane", direction);

        self.tags.get("cycleway").map(|v| v == "lane" || v == "shared_lane").unwrap_or(false)
            || self.tags.get(&cycleway_direction).map(|v| v == "lane" || v == "shared_lane").unwrap_or(false)
            || self.tags.get("cycleway:both").map(|v| v == "lane" || v == "shared_lane").unwrap_or(false)
            || self.tags.iter().any(|(k,v)| k.contains(&lane_direction) && v == "exclusive")
    }

    //         self.is_buslane_right = lambda x: (x.get("cycleway") == "share_busway"
    //                                       or x.get("cycleway:right") == "share_busway"
    //                                       or x.get("cycleway:both") == "share_busway")
    pub fn is_buslane(&self, direction: &str) -> bool {
        let cycleway_direction = format!("cycleway:{}", direction);

        self.tags.get("cycleway").map(|v| v == "share_busway").unwrap_or(false)
            || self.tags.get(&cycleway_direction).map(|v| v == "share_busway").unwrap_or(false)
            || self.tags.get("cycleway_both").map(|v| v == "share_busway").unwrap_or(false)
    }

    //lambda x: (
    //                 ('traffic_sign' in x.keys() and isinstance(x['traffic_sign'], str) and '241' in x['traffic_sign'])
    //                 or ('traffic_sign:forward' in x.keys() and isinstance(x['traffic_sign:forward'], str) and '241' in x[
    //             'traffic_sign:forward'])
    //         )
    pub fn is_obligated(&self) -> bool {
        //benutzungspflicht durch VZ 237, 240, 241
        self.tags.iter().any(|(k,v)| k.contains("traffic_sign") && v == "241" || v == "240" || v == "237")
    }


}

