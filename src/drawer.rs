use sfml::{
    graphics::{
        Color, PrimitiveType, RectangleShape, RenderStates, RenderTarget, RenderWindow,
        Transformable, Vertex,
    },
    system::Vector2f,
};

pub struct Drawer {
    pixel_size: f32,
    plane_size: usize,
}

impl Drawer {
    pub fn new(pixel_size: f32, plane_size: usize) -> Self {
        Self {
            pixel_size,
            plane_size,
        }
    }

    pub fn cells_to_pixels(&self, cells: &[bool], window: &mut RenderWindow) {
        let mut pixels: Vec<RectangleShape> = Vec::new();

        for (i, p) in cells.iter().enumerate() {
            if *p == false {
                continue;
            }

            let mut pix = RectangleShape::new();
            pix.set_size(Vector2f::new(self.pixel_size, self.pixel_size));
            pix.set_position(Vector2f::new(
                (i % self.plane_size) as f32 * self.pixel_size,
                (i / self.plane_size) as f32 * self.pixel_size,
            ));
            pixels.push(pix);
        }

        for pix in pixels.iter() {
            window.draw(pix);
        }
    }

    pub fn vertex_pixels(&self, cells: &[bool], window: &mut RenderWindow) {
        let rs = RenderStates::default();

        let mut pixels: Vec<Vertex> = Vec::new();

        for (i, p) in cells.iter().enumerate() {
            if *p == false {
                continue;
            }

            let base_pos = (
                (i % self.plane_size) as f32 * self.pixel_size,
                (i / self.plane_size) as f32 * self.pixel_size,
            );

            pixels.push(Vertex::with_pos_color(base_pos.into(), Color::WHITE));
        }

        window.draw_primitives(&pixels, PrimitiveType::POINTS, &rs);
    }
}
