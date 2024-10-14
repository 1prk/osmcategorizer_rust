use std::collections::HashMap;
use super::osm_conditions::Conditions;

pub(crate) struct Assessor<'a> {
    pub(crate) conditions: Conditions<'a>,
}

impl<'a> Assessor<'a> {
    pub fn new(tags: &mut HashMap<String, String>) -> Assessor {
        let conditions = Conditions::new(tags);  // Conditions kann nur innerhalb von `assessor.rs` erstellt werden
        Assessor { conditions }
    }

    pub fn bicycle_way(&self, c: &Conditions, direction: &str) -> Vec<bool> {
        // condition 1
        let cond_1 = c.is_bikepath(direction) && !c.can_walk(direction);
        // condition 2
        let cond_2 = c.is_bikepath(direction) && c.is_segregated();
        // condition 3
        let cond_3 = c.can_bike() && (c.is_path() || c.is_track()) && !c.can_walk(direction);
        // condition 4
        let cond_4 = c.can_bike() && (c.is_track() || c.is_footpath() || c.is_path()) && c.is_segregated();
        // condition 5
        let cond_5 = c.can_bike() && c.is_obligated();
        // condition 6
        let cond_6 = c.is_designated_bicycle(direction, "bike") && c.is_designated_bicycle(direction, "foot") && c.is_segregated();

        vec![cond_1, cond_2, cond_3, cond_4, cond_5, cond_6]

    }

    pub fn mixed_way(&self, c: &Conditions, direction: &str) -> Vec<bool> {
        let cond_1 = c.is_bikepath(direction) && c.can_walk(direction) && !c.is_segregated();
        let cond_2 = c.is_footpath() && c.can_bike() && !c.is_segregated();
        let cond_3 = (c.is_path() || c.is_track()) && c.can_bike() && c.can_walk(direction) && !c.is_segregated();
        vec![cond_1, cond_2, cond_3]
    }

    pub fn set_infra<'b>(&mut self, infrastructure: &'b str) -> &'b str {
       self.conditions.tags.insert("bicycle_infrastructure".to_string(), infrastructure.to_string());
       infrastructure
    }

    pub fn assess(&mut self) {
        let cnd_bicycle_way_right = self.bicycle_way(&self.conditions, "right");
        let cnd_bicycle_way_left = self.bicycle_way(&self.conditions, "left");
        let cnd_mixed_right = self.mixed_way(&self.conditions,"right");
        let cnd_mixed_left = self.mixed_way(&self.conditions, "left");

        // |&x| ist eine closure die wie eine lambda-funktion bei python funktioniert.
        // x ist die variable - da es boolean ist
        // reicht ein einfaches true um die bedingung zu erfüllen
        if cnd_bicycle_way_right.iter().any(|&x| x) {
            if cnd_bicycle_way_left.iter().any(|&x| x) {
                self.set_infra("bicycle_way_both");
            }
            else if self.conditions.is_bikelane("left") {
                self.set_infra("bicycle_way_right_lane_left");
            }
            else if self.conditions.is_buslane("left")  {
                self.set_infra("bicycle_way_right_bus_left");
            }
            else if cnd_mixed_left.iter().any(|&x| x)  {
                self.set_infra("bicycle_way_right_mixed_left");
            }
            else if cnd_mixed_right.iter().any(|&x| x)  {
                self.set_infra("bicycle_way_right_mit_left");
            }
            else {
                self.set_infra("bicycle_way_right_no_left");
            }
        }
    }



}