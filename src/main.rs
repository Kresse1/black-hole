
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const CIRCLE_X: usize = WIDTH / 2;
const CIRCLE_Y: usize = HEIGHT /2;
const RADIUS: usize = 50;
const COLOR:u32 = 0xFF0000;

#[derive(Debug)]
struct Ray {
    x: f64,
    y: f64,
    vx: f64,
    vy: f64
}

impl Ray{
    fn new(x: f64, y: f64, vx: f64, vy: f64)-> Self{
        Self {x, y, vx, vy}
    }
    fn update(&mut self){
        self.x += self.vx;
        self.y += self.vy;

    }
    fn apply_gravity(&mut self, bh_x:f64, bh_y:f64, mass:f64){
        let mut dx = bh_x - self.x;
        let mut dy = bh_y - self.y;
        let distance = (dx*dx + dy*dy).sqrt();
        dx = dx / distance;
        dy = dy / distance;
        let a = mass / (distance * distance);
        self.vx += a * dx;
        self.vy += a * dy;
    }
}

fn main() {
    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT];

    let mut window = Window::new(
        "Test - ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    )
    .unwrap_or_else(|e| {
        panic!("{}", e);
    });

    // Limit to max ~60 fps update rate
    window.set_target_fps(60);

    let ray0 = Ray::new(200.0,300.0,3.0,0.0);
    let ray1 = Ray::new(200.0,400.0,3.0,0.0);
    let ray2 = Ray::new(200.0,100.0,3.0,0.0);
    let ray3 = Ray::new(200.0,200.0,3.0,0.0);
    let ray4 = Ray::new(200.0,500.0,3.0,0.0);

    let mut ray_arr = [ray0, ray1, ray2, ray3, ray4];


    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);


        for ray in ray_arr.iter_mut(){
            ray.apply_gravity(CIRCLE_X as f64, CIRCLE_Y as f64, 10000.0);
            ray.update();
            draw_circle(&mut buffer, ray.x as usize, ray.y as usize, 3, 0xFFFFFF);
        }
        draw_circle(&mut buffer, CIRCLE_X, CIRCLE_Y, RADIUS, COLOR);


        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_circle(buffer: &mut Vec<u32>, cx: usize, cy: usize, radius: usize, color: u32) {
    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let dx = x.abs_diff(cx);
            let dy = y.abs_diff(cy);
            if dx.pow(2) + dy.pow(2) < radius.pow(2) {
                buffer[y * WIDTH + x] = color;
            }
        }
    }
}
