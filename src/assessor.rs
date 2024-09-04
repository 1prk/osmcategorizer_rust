use std::collections::HashMap;
use super::osm_conditions::Conditions;

pub(crate) struct Assessor<'a> {
    pub(crate) conditions: Conditions<'a>,
    pub(crate) tags: &'a mut HashMap<String, String>
}

impl<'a> Assessor<'a> {
    //        conditions_b_way_right = [
    //           1  self.is_bikepath_right(x) and not self.can_walk_right(x),  # 0 and 1
    //           2  self.is_bikepath_right(x) and self.is_segregated(x),  # 0 and 2
    //           3  self.can_bike(x) and (self.is_path(x) or self.is_track(x)) and not self.can_walk_right(x),  # and not is_footpath, #3, 4, 1
    //           4  self.can_bike(x) and (self.is_track(x) or self.is_footpath(x) or self.is_path(x)) and self.is_segregated(x),  # b_way_right_5 #3, 6, 2
    //           5  self.can_bike(x) and self.is_obligated_segregated(x),  # 3,7
    //           6  self.is_bicycle_designated_right and self.is_pedestrian_designated_right(x) and self.is_segregated(x)
    pub fn bicycle_way(&self, c: Conditions, direction: &str) -> Vec<bool> {
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

    //        conditions_mixed_right = [
    //             self.is_bikepath_right(x) and self.can_walk_right(x) and not self.is_segregated(x),  # 0 and 1 and 2
    //             self.is_footpath(x) and self.can_bike(x) and not self.is_segregated(x),  # 3 and 4 and 2
    //             (self.is_path(x) or self.is_track(x)) and self.can_bike(x) and self.can_walk_right(x) and not self.is_segregated(x)  # 5 and 4 and 1 and 2
    //         ]
    //         conditions_mixed_left = [
    //             self.is_bikepath_left(x) and self.can_walk_left(x) and not self.is_segregated(x),  # 0 and 1 and 2
    //             self.is_footpath(x) and self.can_bike(x) and not self.is_segregated(x),  # 3 and 4 and 2
    //             (self.is_path(x) or self.is_track(x)) and self.can_bike(x) and self.can_walk_left(x) and not self.is_segregated(x)  # 5 and 4 and 1 and 2
    //         ]

    pub fn mixed_way(&self, c: Conditions, direction: &str) -> Vec<bool> {
        let cond_1 = c.is_bikepath(direction) && c.can_walk(direction) && !c.is_segregated();
        let cond_2 = c.is_footpath() && c.can_bike() && !c.is_segregated();
        let cond_3 = (c.is_path() || c.is_track()) && c.can_bike() && c.can_walk(direction) && !c.is_segregated();
        vec![cond_1, cond_2, cond_3]
    }

    pub fn set_infra<'b>(&mut self, infrastructure: &'b str) -> &'b str {
        self.tags.insert("bicycle_infrastructure".to_string(), infrastructure.to_string());
        infrastructure
    }

    pub fn assess(&mut self, c: Conditions) {
        let cnd_bicycle_way_right = self.bicycle_way(c, "right");
        let cnd_bicycle_way_left = self.bicycle_way(c, "left");
        let cnd_mixed_right = self.mixed_way(c,"right");
        let cnd_mixed_left = self.mixed_way(c, "left");

        // |&x| ist eine closure die wie eine lambda-funktion bei python funktioniert.
        // x ist die variable - da es boolean ist
        // reicht ein einfaches true um die bedingung zu erfüllen
        if cnd_bicycle_way_right.iter().any(|&x| x) {
            if cnd_bicycle_way_left.iter().any(|&x| x) {
                self.set_infra("bicycle_way_both");
            }
            else if c.is_bikelane("left") {
                self.set_infra("bicycle_way_right_lane_left");
            }
            else if c.is_buslane("left")  {
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