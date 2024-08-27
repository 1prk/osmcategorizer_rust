use std::collections::HashMap;
use super::osm_conditions::Conditions;

struct Assessor<'a> {
    tags: &'a HashMap<String, String>
}

impl<'a> Assessor<'a> {
    pub fn bicycle_way_right(&self, conditions: Conditions) -> Vec<bool> {
        vec![
            conditions.can_bike() && (conditions.is_path() || conditions.is_track() || !conditions.can_walk("right"))
        ]
    }
}