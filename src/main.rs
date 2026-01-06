
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
    vy: f64,
    trail: Vec<(f64,f64)>,


}

impl Ray{
    fn new(x: f64, y: f64, vx: f64, vy: f64)-> Self{
        Self {x, y, vx, vy, trail: Vec::new()}
    }
    fn update(&mut self){
        self.x += self.vx;
        self.y += self.vy;
        self.trail.push((self.x, self.y));
        if self.trail.len() > 50{
            self.trail.remove(0);
        }

    }
    fn apply_gravity(&mut self, bh_x:f64, bh_y:f64, mass:f64){
        let mut dx = bh_x - self.x;
        let mut dy = bh_y - self.y;
        let distance = (dx*dx + dy*dy).sqrt();
        if distance < RADIUS as f64{
            self.vx = 0.0;
            self.vy = 0.0;
            return;
        }
        dx = dx / distance;
        dy = dy / distance;
        let a = mass / (distance * distance);
        self.vx += a * dx;
        self.vy += a * dy;


    }
    fn draw_trail(&self, buffer: &mut Vec<u32>, color: u32){
        let len = self.trail.len();
        if len == 0{
            return;
        }

        for (i, (x,y)) in self.trail.iter().enumerate(){
            if *x < 0.0 || *x >= WIDTH as f64 || *y < 0.0 || *y >= HEIGHT as f64{
                continue;
            }

            let brightness = i as f64 / len as f64;
            let c = (brightness * 255.0) as u32;
            let color = (c<<16)|(c<<8)|c;

            buffer[*y as usize * WIDTH + *x as usize] = color;
        }
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

    let mut ray_vec:Vec<Ray> = vec![];

    for i in 0..10{
        ray_vec.push(Ray::new(100.0,200.0+ 40.0*i as f64,10.0,0.0));
    }


    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);


        for ray in ray_vec.iter_mut(){
            ray.apply_gravity(CIRCLE_X as f64, CIRCLE_Y as f64, 10000.0);
            ray.update();
            ray.draw_trail(&mut buffer, 0xFFFFFF);
            draw_circle(&mut buffer, ray.x as usize, ray.y as usize, 2, 0xFFFFFF);
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
