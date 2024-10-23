use minifb::{Key, Window, WindowOptions};


#[derive(Debug, Copy, Clone)]
struct Color {
    num_color: i8,
    name_color: &'static str,
    code_color: u32,
} 

#[derive(Debug, Copy, Clone)]
struct Cube {
    color: Color,
    position: [i8; 3],
}

#[derive(Debug, Copy, Clone)]
struct Face {
    face: [[Cube; 3]; 3],
    position: [i8; 3],
}

#[derive(Debug)]
struct RubiksCube {
    faces: [Face; 6], // Chaque face du cube avec des entiers représentant les couleurs
}

const LIST_COLOR: [Color; 6] = [
    Color{num_color: 0, name_color: "Blanc", code_color: 0xFFFFFF},
    Color{num_color: 1, name_color: "Bleu", code_color: 0x0000FF},
    Color{num_color: 2, name_color: "Rouge", code_color: 0xFF0000},
    Color{num_color: 3, name_color: "Vert", code_color: 0x00FF00},
    Color{num_color: 4, name_color: "Orange", code_color: 0xFFA500},
    Color{num_color: 5, name_color: "Jaune", code_color: 0xFFFF00},
];

const WIDTH: usize = 800;  // Largeur de la fenêtre
const HEIGHT: usize = 800; // Hauteur de la fenêtre
const CELL_SIZE: usize = 40; // Taille d'une cellule du cube

// impl RubiksCube {
    //     fn rotate_face(&mut self, face: usize) {
//         // Logique pour faire tourner une face du cube
//     }
// }

fn main() {
    let mut rubikscube: RubiksCube = init_rubik_s_cube();

    affiche_rubik_s_cube(&rubikscube);
}

// initailiser le rubik's cube avec des nombre
fn init_rubik_s_cube() -> RubiksCube {


    let mut list_cube: [[Cube; 3]; 3] = [[Cube { color: LIST_COLOR[0].clone(), position: [0,0,0] }; 3]; 3];
    let mut faces: [Face; 6] = [Face {face: list_cube.clone(), position: [0,0,0]}; 6];

    let list_position: [[i8; 3]; 6] = [[0,2,1],[2,0,2],[0,4,3],[4,0,4],[1,1,0],[3,3,0]];


    for i in 0..faces.len() {
        for j in 0..list_cube[i].len() {
            for k in 0..list_cube[j].len() {
                list_cube[j][k].color = LIST_COLOR[i].clone();
                list_cube[j][k].position = [i.try_into().unwrap(),j.try_into().unwrap(),k.try_into().unwrap()];
                faces[i].position = list_position[i].clone();
                faces[i].face = list_cube.clone();
            }
        }
    }

    let rubikscube: RubiksCube = RubiksCube{faces};

    println!("{rubikscube:?}");

    rubikscube
}

fn affiche_rubik_s_cube(rubikscube: &RubiksCube) {
    let mut window = Window::new(
        "Rubik's Cube - Press ESC to exit",
        WIDTH,
        HEIGHT,
        WindowOptions::default(),
    ).unwrap_or_else(|e| {
        panic!("{}", e);
    });

    let mut buffer: Vec<u32> = vec![0; WIDTH * HEIGHT]; // Buffer pour les pixels

    // Boucle principale
    while window.is_open() && !window.is_key_down(Key::Escape) {
        draw_cube(&rubikscube, &mut buffer); // Dessiner le cube

        // Afficher le buffer à l'écran
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
    }
}

fn draw_cube(rubikscube: &RubiksCube, buffer: &mut Vec<u32>) {
    // Face avant : on dessine un simple carré 3x3 au centre de l'écran
    for i in 0..rubikscube.faces.len() {
        for j in 0..rubikscube.faces[i].len() {
            for k in 0..rubikscube.faces[i][j].len() {
                for x in 0..CELL_SIZE {
                    for y in 0..CELL_SIZE {
                        let screen_x = (((i+j+i+i) * CELL_SIZE) + x + (WIDTH / 100)) as usize;
                        let screen_y = (((k) * CELL_SIZE) + y + (HEIGHT / 4)) as usize;
                        buffer[screen_x + screen_y * WIDTH] = rubikscube.faces[i][j][k].color.code_color;
                    }
                }
            }
        }
    }
}


// mélanger le rubik's cube


fn rotate_rubik_s_cube(mut rubikscube: &RubiksCube, axe: char, nbr_ligne: i8, sens_des_aiguilles: bool) {
    
    let list_cube: [[Cube; 3]; 3] = [[Cube { color: LIST_COLOR[0].clone(), position: [0,0,0] }; 3]; 3];
    let mut faces_concerner: [Face; 4] = [Face {face: list_cube.clone(), position: [0,0,0]}; 4];
    let ind_axe: usize = if axe == 'x' {0} else if axe == 'y' {1} else if axe == 'z' {2} else {usize::MAX};

    for face in rubikscube.faces {
        if face.position[ind_axe] > 0 {
            faces_concerner[face.position[ind_axe] as usize] = face;
        }
    }

    if !sens_des_aiguilles { faces_concerner.reverse(); }

    for face in faces_concerner {
        
    }
}
