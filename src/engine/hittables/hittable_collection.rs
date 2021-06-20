use crate::engine::hittables::hittable::{Hittable, HitRecord};
use crate::engine::Ray;

pub struct HittableCollection {
    hittable_list: Vec<Box<dyn Hittable>>
}


impl HittableCollection {
    pub fn new() -> HittableCollection {
        HittableCollection {
            hittable_list: Vec::new()
        }
    }

    pub fn add(&mut self, hittable: Box<dyn Hittable>) {
        self.hittable_list.push(hittable);
    }

    pub fn clear(&mut self) {
        self.hittable_list.clear();
    }
}

impl Hittable for HittableCollection {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut smallest_distance = t_max;
        let mut result = None;

        for hittable in &self.hittable_list {
            let hit_result = hittable.hit(ray, t_min, smallest_distance);
            match hit_result {
                Some(hit_record) => {
                    smallest_distance = hit_record.t;
                    result = Some(hit_record);
                },
                _ => {}
            };
        }

        result
    }
}