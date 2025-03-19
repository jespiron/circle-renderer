use crate::circle::{draw_circle, update_circle, Circle};

pub fn render_sequential(
    circles: &mut Vec<Circle>,
    width: usize,
    height: usize,
    dt: f32,
) -> Vec<u32> {
    let mut buffer = vec![(0.0, 0.0, 0.0); width * height];

    for circle in circles.iter_mut() {
        update_circle(circle, dt);
    }
    circles.sort_by(|a, b| a.pos.2.partial_cmp(&b.pos.2).unwrap());

    for circle in circles.iter() {
        draw_circle(&mut buffer, width, height, circle);
    }

    buffer
        .into_iter()
        .map(|(r, g, b)| {
            let r = (r.clamp(0.0, 1.0) * 255.0) as u32;
            let g = (g.clamp(0.0, 1.0) * 255.0) as u32;
            let b = (b.clamp(0.0, 1.0) * 255.0) as u32;
            (r << 16) | (g << 8) | b
        })
        .collect()
}
