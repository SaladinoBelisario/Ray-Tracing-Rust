use super::color::{Color, BLACK, WHITE};
use super::tuple::Tuple;
use super::light::PointLight;
use super::pattern::BoxPattern;
use super::shape::Shape;

#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<BoxPattern>
}

pub const DEFAULT_AMBIENT: f64 = 0.1;
pub const DEFAULT_DIFFUSE: f64 = 0.9;
pub const DEFAULT_SPECULAR: f64 = 0.9;
pub const DEFAULT_SHININESS: f64 = 200.0;
pub const DEFAULT_MATERIAL: Material = Material {
    color: WHITE,
    ambient: DEFAULT_AMBIENT,
    diffuse: DEFAULT_DIFFUSE,
    specular: DEFAULT_SPECULAR,
    shininess: DEFAULT_SHININESS,
    pattern: None };

impl Default for Material {
    fn default() -> Self {
        Material::new(WHITE, DEFAULT_AMBIENT, DEFAULT_DIFFUSE, DEFAULT_SPECULAR, DEFAULT_SHININESS, None)
    }
}

impl Material {
    pub fn new(color: Color, ambient: f64, diffuse: f64, specular: f64, shininess: f64, pattern: Option<BoxPattern>) -> Material {
        Material { color, ambient, diffuse, specular, shininess, pattern }
    }

    pub fn lighting(&self, object: &dyn Shape, light: &PointLight, point: Tuple, eyev: Tuple, normalv: Tuple, in_shadow: bool) -> Color {
        let color = match &self.pattern {
            Some(p) => p.pattern_at_shape(object, point),
            None => self.color
        };
        let effective_color = color * light.intensity;
        let lightv = (light.position - point).normalize();
        let ambient = effective_color * self.ambient;
        let light_dot_normal = lightv.dot(&normalv);
        let (diffuse, specular) =
            if light_dot_normal < 0.0 {
                (BLACK, BLACK)
            }
            else {
                let reflectv = (-lightv).reflect(normalv);
                let reflect_dot_eye = reflectv.dot(&eyev);
                (effective_color * self.diffuse * light_dot_normal,
                 if reflect_dot_eye <= 0.0 {
                     BLACK
                 }
                 else {
                     let factor = reflect_dot_eye.powf(self.shininess);
                     light.intensity * self.specular * factor
                 }
                )
            };
        ambient + if in_shadow { BLACK } else { diffuse + specular }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tuple::ORIGO;
    use crate::sphere::Sphere;
    use crate::pattern::StripePattern;

    #[test]
    fn default_material() {
        let m = Material::default();
        assert_eq!(m.ambient, 0.1);
        assert_eq!(m.diffuse, 0.9);
        assert_eq!(m.specular, 0.9);
        assert_eq!(m.shininess, 200.);
    }

    #[test]
    fn lighting_eye_between_light_and_surface() {
        let object = Sphere::new(None, None);
        let m = Material::default();
        let position = ORIGO;
        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), WHITE);
        let result = m.lighting(&object, &light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.9, 1.9, 1.9));
    }

    #[test]
    fn lighting_eye_between_light_and_surface_eye_offset_45_degrees() {
        let object = Sphere::new(None, None);
        let m = Material::default();
        let position = ORIGO;
        let pv = 2.0f64.sqrt() / 2.0;
        let eyev = Tuple::vector(0., pv, -pv);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), WHITE);
        let result = m.lighting(&object, &light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1., 1., 1.));
    }

    #[test]
    fn lighting_eye_opposite_surface_light_offset_45_degrees() {
        let object = Sphere::new(None, None);
        let m = Material::default();
        let position = ORIGO;
        let eyev = Tuple::vector(0., 0., -1.0 );
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), WHITE);
        let result = m.lighting(&object, &light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(0.7364, 0.7364, 0.7364));
    }

    #[test]
    fn lighting_eye_in_path_of_reflection_vector() {
        let object = Sphere::new(None, None);
        let m = Material::default();
        let position = ORIGO;
        let pv = -2.0f64.sqrt() / 2.0;
        let eyev = Tuple::vector(0., pv, pv);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 10., -10.), WHITE);
        let result = m.lighting(&object, &light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(1.6364, 1.6364, 1.6364));
    }

    #[test]
    fn lighting_light_behind_surface() {
        let object = Sphere::new(None, None);
        let m = Material::default();
        let position = ORIGO;
        let eyev = Tuple::vector(0., 0., -1.0 );
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., 10.), WHITE);
        let result = m.lighting(&object, &light, position, eyev, normalv, false);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_surface_in_shadow() {
        let object = Sphere::new(None, None);
        let m = Material::default();
        let position = ORIGO;
        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), WHITE);
        let in_shadow = true;
        let result = m.lighting(&object, &light, position, eyev, normalv, in_shadow);

        assert_eq!(result, Color::new(0.1, 0.1, 0.1));
    }

    #[test]
    fn lighting_with_pattern_applied() {
        let object = Sphere::new(None, None);
        let m = Material::new(WHITE, 1., 0., 0., DEFAULT_SHININESS, Some(StripePattern::new_boxed(WHITE, BLACK, None)));
        let eyev = Tuple::vector(0., 0., -1.);
        let normalv = Tuple::vector(0., 0., -1.);
        let light = PointLight::new(Tuple::point(0., 0., -10.), WHITE);
        let c1 = m.lighting(&object, &light, Tuple::point(0.9, 0., 0.), eyev, normalv, false);
        let c2 = m.lighting(&object, &light, Tuple::point(1.1, 0., 0.), eyev, normalv, false);

        assert_eq!(c1, WHITE);
        assert_eq!(c2, BLACK);
    }
}