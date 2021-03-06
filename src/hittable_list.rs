use crate::hittable::HitRecord;
use crate::hittable::Hittable;
use crate::Ray;

pub type HittableObject = Box<dyn Hittable + Sync>;

pub struct HittableList {
  pub objects: Vec<HittableObject>,
}

impl HittableList {
  pub fn new() -> HittableList {
    HittableList {
      objects: Vec::new(),
    }
  }

  pub fn add(&mut self, h: HittableObject) {
    self.objects.push(h)
  }
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
            closest_t = ht.t;
            h = Some(ht);
          }
        }
        None => {}
      }
    }
    h
  }
}
