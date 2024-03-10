use std::f32::consts::PI;

use macroquad::prelude::*;


struct Pendulum {
    color: Color,

    a1: f32, // angle of the 1st segment
    a2: f32, // angle of the 2nd segment
    m1: f32, // mass
    m2: f32,
    v1: f32, // speed
    v2: f32,
    l1: f32, // length
    l2: f32,

    x0: f32, // center position
    y0: f32,
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
}

impl Pendulum {
    pub fn new(a1: f32, a2: f32, width: f32, height: f32, color: Color) -> Self {
        Self {
            color,
            a1,
            a2,
            m1: 10.,
            m2: 3.,
            v1: 0.,
            v2: 0.,
            l1: 250.,
            l2: 250.,

            x0: width / 2.,
            y0: height / 2.,
            x1: 0.,
            y1: 0.,
            x2: 0.,
            y2: 0.,
        }
    }

    pub fn update(&mut self) {
        self.v1 += self.dda1();
        self.v2 += self.dda2();
        self.a1 = (self.a1 + self.v1) % (2. * PI);
        self.a2 = (self.a2 + self.v2) % (2. * PI);
        self.update_positions()
    }

    fn update_positions(&mut self) {
        self.x1 = self.l1 * self.a1.sin() + self.x0;
        self.y1 = self.l1 * self.a1.cos() + self.y0;
        self.x2 = self.l2 * self.a2.sin() + self.x1;
        self.y2 = self.l2 * self.a2.cos() + self.y1;
    }

    fn dda1(&self) -> f32 {
        let n1 = (2. * self.m1 + self.m2) * self.a1.sin();
        let n2 = self.m2 * (self.a1 - 2. * self.a2).sin();
        let n3 = 2.
            * (self.a1 - self.a2).sin()
            * self.m2
            * (self.v2.powf(2.) * self.l2 + self.v1.powf(2.) * self.l1 * (self.a1 - self.a2).cos());
        let den =
            self.l1 * (2. * self.m1 + self.m2 - self.m2 * (2. * self.a1 - 2. * self.a2).cos());

        (-n1 - n2 - n3) / den
    }

    fn dda2(&self) -> f32 {
        let n1 = 2. * (self.a1 - self.a2).sin();
        let n2 = self.v1.powf(2.) * self.l1 * (self.m1 + self.m2);
        let n3 = (self.m1 + self.m2) * self.a1.cos();
        let n4 = self.v2.powf(2.) * self.l2 * self.m2 * (self.a1 - self.a2).cos();
        let den =
            self.l2 * (2. * self.m1 + self.m2 - self.m2 * (2. * self.a1 - 2. * self.a2).cos());

        n1 * (n2 + n3 + n4) / den
    }
}


fn window_conf() -> Conf {
    Conf {
        window_title: "Double Pendulum".to_owned(),
        high_dpi: true,
        window_width: 1000,
        window_height: 1000,
        sample_count: 4,
        ..Default::default()
    }
}

#[macroquad::main(window_conf())]
async fn main() {
    let n = 2500;
    let mut pendulums: Vec<Pendulum> = Vec::new();

    let start_c = Color::new(1., 0., 0., 1.);
    let end_c = Color::new(1., 0.6, 0., 1.);

    for i in 0..n {
        let ratio = i as f32 / n as f32;

        let r = start_c.r + (end_c.r - start_c.r) * ratio;
        let g = start_c.g + (end_c.g - start_c.g) * ratio;
        let b = start_c.b + (end_c.b - start_c.b) * ratio;
        let c = Color::new(r, g, b, 1.);

        let d = 0.01 + 0.02 * ratio;

        pendulums.push(Pendulum::new(
            PI + d,
            PI,
            screen_width(),
            screen_height(),
            c,
        ));
    }

    loop {
        clear_background(BLACK);

        for p in pendulums.iter_mut() {
            draw_line(p.x0, p.y0, p.x1, p.y1, 2., p.color);
            draw_line(p.x1, p.y1, p.x2, p.y2, 2., p.color);
            p.update();
        }

        next_frame().await;
    }
}
