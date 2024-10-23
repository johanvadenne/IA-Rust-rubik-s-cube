use minifb::{Key, Window, WindowOptions};


struct RubiksCube {
    faces: [[[u8; 3]; 3]; 6], // Chaque face a 3x3 cellules
}

impl RubiksCube {
    fn new() -> Self {
        RubiksCube {
            faces: [
                [[0; 3]; 3], // Face 1 (blanc par exemple)
                [[1; 3]; 3], // Face 2 (bleu)
                [[2; 3]; 3], // Face 3 (rouge)
                [[3; 3]; 3], // Face 4 (vert)
                [[4; 3]; 3], // Face 5 (orange)
                [[5; 3]; 3], // Face 6 (jaune)
            ],
        }
    }

    fn get_color(&self, face: usize, x: usize, y: usize) -> u32 {
        match self.faces[face][x][y] {
            0 => 0xFFFFFF, // Blanc
            1 => 0x0000FF, // Bleu
            2 => 0xFF0000, // Rouge
            3 => 0x00FF00, // Vert
            4 => 0xFFA500, // Orange
            5 => 0xFFFF00, // Jaune
            _ => 0x000000, // Noir (par défaut)
        }
    }
}


const WIDTH: usize = 400;  // Largeur de la fenêtre
const HEIGHT: usize = 400; // Hauteur de la fenêtre
const CELL_SIZE: usize = 40; // Taille d'une cellule du cube

fn draw_cube(cube: &RubiksCube, buffer: &mut Vec<u32>) {
    // Face avant : on dessine un simple carré 3x3 au centre de l'écran
    for i in 0..3 {
        for j in 0..3 {
            let color = cube.get_color(0, i, j);
            for x in 0..CELL_SIZE {
                for y in 0..CELL_SIZE {
                    let screen_x = (i * CELL_SIZE + x + WIDTH / 4) as usize;
                    let screen_y = (j * CELL_SIZE + y + HEIGHT / 4) as usize;
                    buffer[screen_x + screen_y * WIDTH] = color;
                }
            }
        }
    }
}

fn main() {
    let mut window = Window::new(
        "Rubik's Cube - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // Buffer pour les pixels
    let cube = RubiksCube::new(); // Création du cube

    // Boucle principale
    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_cube(&cube, &mut buffer); // Dessiner le cube

        // Afficher le buffer à l'écran
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}
