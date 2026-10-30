mod framebuffer;
mod line;
mod obj;
mod triangle;

use framebuffer::Framebuffer;
use line::Vec2;
use obj::{Obj, Vec3};
use triangle::triangle;

const WIDTH: usize = 800;
const HEIGHT: usize = 600;
const MODEL_PATH: &str = "assets/nave.obj";
const LINE_COLOR: u32 = 0x5abef0; // celeste

/// Escala y traslación para pasar de coordenadas del modelo (x, y) a pantalla.
struct ScreenTransform {
    scale: f32,
    center_x: f32,
    center_y: f32,
}

impl ScreenTransform {
    /// Calcula la escala para que el modelo ocupe ~80% de la ventana y quede centrado.
    fn fit(vertices: &[Vec3], width: f32, height: f32) -> Self {
        let (mut min_x, mut max_x) = (f32::MAX, f32::MIN);
        let (mut min_y, mut max_y) = (f32::MAX, f32::MIN);
        for v in vertices {
            min_x = min_x.min(v.x);
            max_x = max_x.max(v.x);
            min_y = min_y.min(v.y);
            max_y = max_y.max(v.y);
        }
        let model_w = (max_x - min_x).max(1e-6);
        let model_h = (max_y - min_y).max(1e-6);
        let scale = (width * 0.8 / model_w).min(height * 0.8 / model_h);

        ScreenTransform {
            scale,
            center_x: (min_x + max_x) / 2.0,
            center_y: (min_y + max_y) / 2.0,
        }
    }

    /// Solo usamos x y y. En pantalla y crece hacia abajo, por eso se invierte.
    fn to_screen(&self, v: &Vec3, width: f32, height: f32) -> Vec2 {
        Vec2 {
            x: (v.x - self.center_x) * self.scale + width / 2.0,
            y: -(v.y - self.center_y) * self.scale + height / 2.0,
        }
    }
}

/// Recorre todas las caras del modelo y dibuja cada triángulo.
fn render(fb: &mut Framebuffer, model: &Obj, t: &ScreenTransform) {
    fb.clear();
    fb.set_current_color(LINE_COLOR);
    let (w, h) = (fb.width as f32, fb.height as f32);

    for face in &model.faces {
        // 1. Los tres índices de la cara
        let [i0, i1, i2] = *face;

        // 2. Buscar los tres vértices con esos índices
        let v0 = &model.vertices[i0];
        let v1 = &model.vertices[i1];
        let v2 = &model.vertices[i2];

        // 3. Pasarlos a pantalla (solo x, y) y dibujar el triángulo
        let a = t.to_screen(v0, w, h);
        let b = t.to_screen(v1, w, h);
        let c = t.to_screen(v2, w, h);
        triangle(fb, a, b, c);
    }
}

fn main() {
    let model = Obj::load(MODEL_PATH).unwrap_or_else(|e| {
        eprintln!("{e}");
        std::process::exit(1);
    });
    println!(
        "Modelo cargado: {} vertices, {} triangulos",
        model.vertices.len(),
        model.faces.len()
    );

    let mut fb = Framebuffer::new(WIDTH, HEIGHT);
    let transform = ScreenTransform::fit(&model.vertices, WIDTH as f32, HEIGHT as f32);
    render(&mut fb, &model, &transform);

    // Guardar captura sin abrir ventana
    if std::env::args().any(|a| a == "--screenshot") {
        std::fs::create_dir_all("docs").ok();
        match fb.save_png("docs/screenshot.png") {
            Ok(_) => println!("Guardado docs/screenshot.png"),
            Err(e) => eprintln!("Error guardando la captura: {e}"),
        }
        return;
    }

    run_window(&mut fb);
}

fn run_window(fb: &mut Framebuffer) {
    use raylib::prelude::*;

    let (mut rl, thread) = raylib::init()
        .size(WIDTH as i32, HEIGHT as i32)
        .title("Lab 4 - Carga de modelos")
        .build();
    rl.set_target_fps(60);

    let image = Image::gen_image_color(WIDTH as i32, HEIGHT as i32, Color::BLACK);
    let mut texture = rl
        .load_texture_from_image(&thread, &image)
        .expect("No se pudo crear la textura");
    texture
        .update_texture(&fb.to_rgba())
        .expect("No se pudo actualizar la textura");

    while !rl.window_should_close() {
        // F12: guardar captura desde la ventana
        if rl.is_key_pressed(KeyboardKey::KEY_F12) {
            std::fs::create_dir_all("docs").ok();
            if fb.save_png("docs/screenshot.png").is_ok() {
                println!("Guardado docs/screenshot.png");
            }
        }

        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        d.draw_texture(&texture, 0, 0, Color::WHITE);
    }
}
