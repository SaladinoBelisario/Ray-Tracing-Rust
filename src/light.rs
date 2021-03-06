use super::color::Color;
use super::tuple::Tuple;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct PointLight {
    pub position: Tuple,
    pub intensity: Color
}

impl PointLight {
    pub fn new(position: Tuple, intensity: Color) -> PointLight {
        PointLight { position, intensity }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::color::WHITE;

    #[test]
    fn point_light_has_position_and_intensity() {
        let intensity = WHITE;
        let position = Tuple::point(0., 0., 0.);
        let light = PointLight::new(position, intensity);

        assert_eq!(light.position, position);
        assert_eq!(light.intensity, intensity);
    }
}