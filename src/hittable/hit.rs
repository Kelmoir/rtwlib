#[derive(Debug, Clone)]
pub struct HitRecord {
    pub p: Vec3,
    pub normal: Vec3,
    pub t: f64,
    pub front_face: bool,
    pub material: Option<Rc<dyn Material>>,
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            p: Vec3::new(0.0, 0.0, 0.0),
            normal: Vec3::new(0.0, 0.0, 0.0),
            t: 0.0,
            front_face: false,
            material: None,
        }
    }
} 