use std::collections::HashMap;

pub fn is_segregated(tags: &HashMap<String, String>) -> bool {
    tags.get("segregated").map(|v| v == "yes").unwrap_or(false)
}

pub fn is_footpath(tags: &HashMap<String, String>) -> bool {
    tags.get("highway").map(|v| v == "footway" || v == "pedestrian").unwrap_or(false)
}

pub fn is_not_accessible(tags: &HashMap<String, String>) -> bool {
    tags.get("access").map(|v| v == "no").unwrap_or(false)
}

// Beispielfunktion um zu schauen ob ein Key den Begriff "bicycle" enthält.
// gleich wie lambda x: any(k for k, v in x.items() if 'bicycle' in k and v == 'use_sidepath')
pub fn use_sidepath(tags: &HashMap<String, String>) -> bool {
    tags.iter().any(|(key, value)| key.contains("bicycle") && value == "use_sidepath")
}

pub fn is_indoor(tags: &HashMap<String, String>) -> bool {
    tags.get("indoor").map(|v| v == "yes").unwrap_or(false)
}

pub fn is_path(tags: &HashMap<String, String>) -> bool {
    tags.get("highway").map(|v| v == "path").unwrap_or(false)
}

pub fn is_track(tags: &HashMap<String, String>) -> bool {
    tags.get("highway").map(|v| v == "track").unwrap_or(false)
}

pub fn can_walk(tags: &HashMap<String, String>) -> bool {
    tags.get("foot").map(|v| v == "yes" || v == "designated").unwrap_or(false)
        || ["right", "left"].iter().any(|&direction|
            tags.iter().any(|(key, value)| key.contains(&(direction.to_string()+":foot")) && (value == "yes" || value == "designated")) ||
        tags.get(&("sidewalk:".to_string() + direction)).map(|v| v == "yes" || v == "separated" || v == "both" || v == "left").unwrap_or(false)
        )
        || tags.get("sidewalk").map(|v| v == "yes" || v == "separated" || v == "both" || v == "right" || v == "left").unwrap_or(false)
        || tags.get("sidewalk:both").map(|v| v == "yes" || v == "separated" || v == "both").unwrap_or(false)
    }

// pub fn can_bike(tags: &HashMap<String, String>) -> bool {
//
// }