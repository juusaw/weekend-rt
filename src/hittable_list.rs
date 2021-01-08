use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::Ray;

pub struct HittableList {
  pub objects: Vec<Box<dyn Hittable>>,
}

impl Hittable for HittableList {
  fn hit(self: &HittableList, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
    let mut h: Option<HitRecord> = None;
    let mut closest_t = t_max;

    for obj in self.objects.iter() {
      let hit = obj.hit(ray, t_min, t_max);
      match hit {
        Some(ht) => {
          if ht.t < closest_t {
            h = hit;
            closest_t = ht.t;
          }
        }
        None => {}
      }
    }
    h
  }
}
