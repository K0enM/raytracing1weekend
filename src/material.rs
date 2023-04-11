pub trait Material {
  fn scatter(ray_in: &Ray, rec: &HitRecord, attenuation: &Color3, scattered: &Ray) -> bool;
}