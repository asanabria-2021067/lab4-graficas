// Cargador de OBJ: guarda los vértices y, aparte, los índices de cada cara.

use std::fs;

#[derive(Clone, Copy, Debug)]
pub struct Vec3 {
    pub x: f32,
    pub y: f32,
    /// Se carga pero todavía no se usa (no hay proyección 3D en este lab).
    #[allow(dead_code)]
    pub z: f32,
}

pub struct Obj {
    /// Lista de vértices (líneas "v x y z" del archivo).
    pub vertices: Vec<Vec3>,
    /// Cada cara son 3 índices a la lista de vértices (base 0).
    pub faces: Vec<[usize; 3]>,
}

impl Obj {
    pub fn load(path: &str) -> Result<Obj, String> {
        let text =
            fs::read_to_string(path).map_err(|e| format!("No se pudo leer {path}: {e}"))?;

        let mut vertices = Vec::new();
        let mut faces = Vec::new();

        for (num, line) in text.lines().enumerate() {
            let mut parts = line.split_whitespace();
            match parts.next() {
                Some("v") => {
                    let c: Vec<f32> = parts.take(3).filter_map(|s| s.parse().ok()).collect();
                    if c.len() != 3 {
                        return Err(format!("Vértice inválido en la línea {}", num + 1));
                    }
                    vertices.push(Vec3 { x: c[0], y: c[1], z: c[2] });
                }
                Some("f") => {
                    // "f 1/1/1 2/2/2 3/3/3" -> solo nos importa el primer número (el vértice)
                    let idx: Vec<usize> = parts
                        .filter_map(|s| s.split('/').next())
                        .filter_map(|s| s.parse::<usize>().ok())
                        .map(|i| i - 1)
                        .collect();
                    // Si la cara tiene más de 3 vértices se parte en triángulos (abanico)
                    for k in 1..idx.len().saturating_sub(1) {
                        faces.push([idx[0], idx[k], idx[k + 1]]);
                    }
                }
                _ => {}
            }
        }

        // Revisar que ningún índice se salga de la lista de vértices
        for f in &faces {
            if f.iter().any(|&i| i >= vertices.len()) {
                return Err("Hay una cara con un índice fuera de rango".to_string());
            }
        }

        Ok(Obj { vertices, faces })
    }
}
