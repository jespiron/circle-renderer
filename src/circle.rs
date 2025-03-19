use rand::Rng;

#[derive(Clone)]
pub struct Circle {
    pub pos: (f32, f32, f32),
    pub vel: (f32, f32, f32),
    pub radius: f32,
    pub color: (f32, f32, f32, f32), // (R, G, B, Alpha)
}

pub fn generate_circles(count: usize) -> Vec<Circle> {
    let mut rng = rand::rng();
    (0..count)
        .map(|_| Circle {
            pos: (
                rng.random_range(100.0..700.0),
                rng.random_range(100.0..500.0),
                rng.random_range(0.0..1000.0),
            ),
            vel: (
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
                rng.random_range(-1.0..1.0),
            ),
            radius: rng.random_range(50.0..100.0),
            color: (
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.0..1.0),
                rng.random_range(0.2..0.8),
            ),
        })
        .collect()
}

pub fn update_circle(circle: &mut Circle, dt: f32) {
    circle.pos.0 += circle.vel.0 * dt;
    circle.pos.1 += circle.vel.1 * dt;
    circle.pos.2 += circle.vel.2 * dt;
}

pub fn blend_color(bg: (f32, f32, f32), fg: (f32, f32, f32, f32)) -> (f32, f32, f32) {
    let (pr, pg, pb) = bg;
    let (cr, cg, cb, alpha) = fg;
    (
        alpha * cr + (1.0 - alpha) * pr,
        alpha * cg + (1.0 - alpha) * pg,
        alpha * cb + (1.0 - alpha) * pb,
    )
}

pub fn draw_circle(
    buffer: &mut Vec<(f32, f32, f32)>,
    width: usize,
    height: usize,
    circle: &Circle,
) {
    let screen_x = circle.pos.0 as i32;
    let screen_y = circle.pos.1 as i32;
    let radius = circle.radius as i32;

    let bbox_x_min = (screen_x - radius).max(0);
    let bbox_x_max = (screen_x + radius).min(width as i32);
    let bbox_y_min = (screen_y - radius).max(0);
    let bbox_y_max = (screen_y + radius).min(height as i32);

    for y in bbox_y_min..bbox_y_max {
        for x in bbox_x_min..bbox_x_max {
            let center = (x as f32 + 0.5, y as f32 + 0.5);
            let dx = center.0 - circle.pos.0;
            let dy = center.1 - circle.pos.1;
            if dx * dx + dy * dy <= circle.radius * circle.radius {
                let idx = (y * width as i32 + x) as usize;
                buffer[idx] = blend_color(buffer[idx], circle.color);
            }
        }
    }
}
