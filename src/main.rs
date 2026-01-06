
use minifb::{Key, Window, WindowOptions};

const WIDTH: usize = 800;
const HEIGHT: usize = 600;

const OFF_WIDTH: usize = WIDTH / 2;
const OFF_HEIGHT: usize = HEIGHT /2;

const RADIUS: usize = 50;
const COLOR:u32 = 0xFF0000;


const C:f64 = 299792458.0;
const G:f64 = 6.67430e-11;
const BLACK_HOLE_MASS: f64 = 3.4e28;  // kg
const SCHWARZSCHILD_RADIUS: f64 = 2.0 * G * BLACK_HOLE_MASS / (C * C);

struct Ray {
    alive: bool,
    x: f64,
    y: f64,
    trail: Vec<(f64, f64)>,
    r: f64,
    phi: f64,
    dr: f64,
    dphi: f64,
    energy: f64,
}

impl Ray{
    fn new(x: f64, y: f64, vx: f64, vy: f64) -> Self {
        let r = (x * x + y * y).sqrt();
        let phi = y.atan2(x);
        let dr = vx * phi.cos() + vy * phi.sin();
        let dphi = (-vx * phi.sin() + vy * phi.cos()) / r;
        
        let metric_factor = 1.0 - SCHWARZSCHILD_RADIUS / r;
        let time_dilation = ((dr * dr) / (metric_factor * metric_factor)
                          + (r * r * dphi * dphi) / metric_factor).sqrt();
        let energy = metric_factor * time_dilation;
        
        Self {
            x, y,
            trail: Vec::new(),
            r, phi, dr, dphi,
            energy,
            alive: true,
        }
    }
    fn draw_trail(&self, buffer: &mut Vec<u32>) {
        let len = self.trail.len();
        if len == 0 {
            return;
        }
        
        for (i, (x, y)) in self.trail.iter().enumerate() {
            let screen_x = *x + WIDTH as f64 / 2.0;
            let screen_y = *y + HEIGHT as f64 / 2.0;
            
            if screen_x < 0.0 || screen_x >= WIDTH as f64 || screen_y < 0.0 || screen_y >= HEIGHT as f64 {
                continue;
            }
            
            let brightness = i as f64 / len as f64;
            let c = (brightness * 255.0) as u32;
            let color = (c << 16) | (c << 8) | c;
            
            buffer[screen_y as usize * WIDTH + screen_x as usize] = color;
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

    for i in 0..100{
        ray_vec.push(Ray::new(-600.0,-300.0+ 8.0*i as f64,3.0,0.0));
    }


    while window.is_open() && !window.is_key_down(Key::Escape) {
        buffer.fill(0);


        for ray in ray_vec.iter_mut(){
            if !ray.alive{
                continue;
            }
            rk4_step(ray, 1.0);
            ray.draw_trail(&mut buffer);
            let screen_x = (ray.x + OFF_WIDTH as f64) as usize;
            let screen_y = (ray.y + OFF_HEIGHT as f64) as usize;
            draw_circle(&mut buffer, screen_x, screen_y, 2, 0xFFFFFF);
        }
        draw_circle(&mut buffer, OFF_WIDTH, OFF_HEIGHT, RADIUS, COLOR);


        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_circle(buffer: &mut Vec<u32>, cx: usize, cy: usize, radius: usize, color: u32) {
    let x_start = cx.saturating_sub(radius);
    let x_end = (cx + radius + 1).min(WIDTH);
    let y_start = cy.saturating_sub(radius);
    let y_end = (cy + radius + 1).min(HEIGHT);
    
    for y in y_start..y_end {
        for x in x_start..x_end {
            let dx = x.abs_diff(cx);
            let dy = y.abs_diff(cy);
            if dx * dx + dy * dy < radius * radius {
                buffer[y * WIDTH + x] = color;
            }
        }
    }
}

fn geodesic_rhs(r: f64, dr: f64, dphi: f64, energy: f64, rs: f64) -> [f64; 4] {

    let metric_factor = 1.0 - rs / r;
    let time_dilation = energy / metric_factor;
    
    let radial_acceleration = 
        -(rs / (2.0 * r * r)) * metric_factor * time_dilation * time_dilation
        + (rs / (2.0 * r * r * metric_factor)) * dr * dr
        + (r - rs) * dphi * dphi;
    
    let angular_acceleration = -2.0 * dr * dphi / r;
    
    [dr, dphi, radial_acceleration, angular_acceleration]
}


fn rk4_step(ray: &mut Ray, dt: f64) {

    if ray.r <= SCHWARZSCHILD_RADIUS * 1.1 {
        ray.alive = false;
        return;
    }

    let state = [ray.r, ray.phi, ray.dr, ray.dphi];
    

    let k1 = geodesic_rhs(state[0], state[2], state[3], ray.energy, SCHWARZSCHILD_RADIUS);
    

    let k2 = geodesic_rhs(
        state[0] + k1[0] * dt / 2.0,
        state[2] + k1[2] * dt / 2.0,
        state[3] + k1[3] * dt / 2.0,
        ray.energy,
        SCHWARZSCHILD_RADIUS,
    );
    

    let k3 = geodesic_rhs(
        state[0] + k2[0] * dt / 2.0,
        state[2] + k2[2] * dt / 2.0,
        state[3] + k2[3] * dt / 2.0,
        ray.energy,
        SCHWARZSCHILD_RADIUS,
    );
    

    let k4 = geodesic_rhs(
        state[0] + k3[0] * dt,
        state[2] + k3[2] * dt,
        state[3] + k3[3] * dt,
        ray.energy,
        SCHWARZSCHILD_RADIUS,
    );
    

    ray.r    += dt / 6.0 * (k1[0] + 2.0 * k2[0] + 2.0 * k3[0] + k4[0]);
    ray.phi  += dt / 6.0 * (k1[1] + 2.0 * k2[1] + 2.0 * k3[1] + k4[1]);
    ray.dr   += dt / 6.0 * (k1[2] + 2.0 * k2[2] + 2.0 * k3[2] + k4[2]);
    ray.dphi += dt / 6.0 * (k1[3] + 2.0 * k2[3] + 2.0 * k3[3] + k4[3]);
    

    ray.x = ray.r * ray.phi.cos();
    ray.y = ray.r * ray.phi.sin();
    

    ray.trail.push((ray.x, ray.y));
    if ray.trail.len() > 200 {
        ray.trail.remove(0);
    }
}

