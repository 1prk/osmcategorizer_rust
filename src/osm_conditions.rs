use std::collections::HashMap;

// struct ist die klassendefinition
// 'a ist die lifetime-definition (solange Conditions existiert gilt der verweis zur hashmap)
// pythons gc kümmert sich dadrum automatisch
// & ist die referenz auf eine hashmap woanders
pub(crate) struct Conditions<'a> {
    pub(crate) tags: &'a HashMap<String, String>
}

// hier beginnt die Implementierung der Methoden für die struct Conditions
impl<'a> Conditions<'a> {
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

    pub fn is_designated(&self) -> bool {
        self.tags.get("bicycle").map(|v| v == "designated").unwrap_or(false)
    }

    // hier ebenfalls richtungsfein. entweder left oder right
    pub fn is_bicycle_designated(&self, direction: &str) -> bool {
        (self.tags.get("bicycle").map(|v| v == "designated").unwrap_or(false)
        || self.tags.get(&("cycleway".to_string() + direction + "bicycle")).map(|v| v == "designated").unwrap_or(false))
        || self.tags.get("cycleway:bicycle").map(|v| v == "designated").unwrap_or(false)
    }
}


