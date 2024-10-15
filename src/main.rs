use nannou::prelude::*;

const HEART_SIZE: f32 = 250.0;
const PARTICLE_COUNT: usize = 20000;
const OUTLINE_THICKNESS: f32 = 30.0;
const INTERIOR_PARTICLE_RATIO: f32 = 0.1;

struct Model {
    particles: Vec<Particle>,
    phase: f32,
}

struct Particle {
    position: Vec2,
    initial_offset: Vec2,
    velocity: Vec2,
    size: f32,
    color: Rgb,
    phase_offset: f32,
}

fn main() {
    nannou::app(model).update(update).run();
}

fn model(app: &App) -> Model {
    app.new_window().size(800, 600).view(view).build().unwrap();
    
    let particles = (0..PARTICLE_COUNT).map(|_| create_particle()).collect();
    
    Model {
        particles,
        phase: 0.0,
    }
}

fn update(_app: &App, model: &mut Model, _update: Update) {
    model.phase += 0.05;
    let heartbeat = (model.phase.sin() * 0.5 + 0.5).powf(0.3);
    let global_wave = (model.phase * 0.5).sin() * 5.0;

    for particle in &mut model.particles {
        let individual_phase = model.phase + particle.phase_offset;
        let heartbeat_offset = vec2(
            (individual_phase * 2.0).cos(),
            (individual_phase * 2.0).sin()
        ) * 10.0 * heartbeat;

        let wave_offset = vec2(
            (individual_phase * 0.5).cos(),
            (individual_phase * 0.5).sin()
        ) * global_wave;

        particle.position = get_heart_point(particle.initial_offset.x)
            + particle.initial_offset * (1.0 + heartbeat * 0.2)
            + heartbeat_offset
            + wave_offset;

        particle.velocity += vec2(random_f32() - 0.5, random_f32() - 0.5) * 1.0;
        particle.velocity *= 0.9;
        particle.position += particle.velocity;

        let to_heart = get_heart_point(particle.initial_offset.x) - particle.position;
        if to_heart.length() > OUTLINE_THICKNESS * 1.5 {
            particle.velocity += to_heart * 0.05;
        }

        let speed = particle.velocity.length();
        let distance_from_center = particle.position.length() / HEART_SIZE;
        let hue = 0.0;
        let sat = map_range(distance_from_center, 0.0, 1.0, 0.9, 1.0);
        let val = map_range(speed, 0.0, 5.0, 0.7, 1.0);
        particle.color = hsv(hue, sat, val).into();
    }
}

fn view(app: &App, model: &Model, frame: Frame) {
    let draw = app.draw();
    draw.background().color(BLACK);

    for particle in &model.particles {
        draw.ellipse()
            .xy(particle.position)
            .radius(particle.size * (1.0 + 0.2 * (app.time * 2.0 + particle.phase_offset).sin()))
            .color(particle.color);
    }

    draw.to_frame(app, &frame).unwrap();
}

fn create_particle() -> Particle {
    let is_interior = random_f32() < INTERIOR_PARTICLE_RATIO;
    let angle = random_f32() * TAU;
    let radius = if is_interior {
        random_f32().powf(0.5) * HEART_SIZE * 0.9
    } else {
        OUTLINE_THICKNESS * (random_f32() - 0.5).powf(0.5)
    };
    let offset = vec2(radius * angle.cos(), radius * angle.sin());
    let position = get_heart_point(angle) + offset;

    Particle {
        position,
        initial_offset: offset,
        velocity: vec2(random_f32() - 0.5, random_f32() - 0.5) * 2.0,
        size: random_range(0.5, 3.0),
        color: rgb(1.0, 0.0, 0.0),
        phase_offset: random_f32() * TAU,
    }
}

fn get_heart_point(t: f32) -> Vec2 {
    let x = 16.0 * t.sin().powi(3);
    let y = 13.0 * t.cos() - 5.0 * (2.0 * t).cos() - 2.0 * (3.0 * t).cos() - (4.0 * t).cos();
    vec2(x, y) * (HEART_SIZE / 16.0)
}
