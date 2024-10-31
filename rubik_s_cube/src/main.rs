// use std::path::Display;

use rand::Rng;

#[derive(Debug, Clone, Copy, PartialEq)]
enum Color {
    White,
    Blue,
    Yellow,
    Green,
    Red,
    Orange,
}

#[derive(Debug, Clone)]
struct RubiksCube {
    faces: [[[Color; 3]; 3]; 6],
}

//    🔴
// ⚪🔵🟡🟢
//    🟠
impl RubiksCube {
    fn new() -> Self {
        let faces = [
            [[Color::White; 3]; 3],
            [[Color::Blue; 3]; 3],
            [[Color::Yellow; 3]; 3],
            [[Color::Green; 3]; 3],
            [[Color::Red; 3]; 3],
            [[Color::Orange; 3]; 3],
        ];
        Self { faces }
    }

    //   1
    // 1 0 1 0
    //   1
    fn rotate_x(&mut self, layer: usize) {
        // Rotation logic on X-axis for specified layer
        let (front, back, top, bottom) = (0, 2, 4, 5);
        let temp = self.faces[top][layer];
        
        for i in 0..3 {
            self.faces[top][layer][i] = self.faces[front][2-i][layer];
            self.faces[front][2-i][layer] = self.faces[bottom][2-layer][2-i];
            self.faces[bottom][2-layer][2-i] = self.faces[back][i][2-layer];
            self.faces[back][i][2-layer] = temp[i];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[3], false);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[1], true);
        }
    }


    //   1
    // 0 1 0 1
    //   1
    fn rotate_y(&mut self, layer: usize) {
        // Rotation logic on X-axis for specified layer
        let (front, back, top, bottom) = (1,3,4,5);
        let temp = self.faces[top];

        for i in 0..3 {
            self.faces[top][i][layer] = self.faces[front][i][layer];
            self.faces[front][i][layer] = self.faces[bottom][i][layer];
            self.faces[bottom][i][layer] = self.faces[back][2-i][2-layer];
            self.faces[back][2-i][2-layer] = temp[i][layer];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[0], false);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[2], true);
        }
    }


    //   0
    // 1 1 1 1
    //   0
    fn rotate_z(&mut self, layer: usize) {
        // Rotation logic on X-axis for specified layer
        let (front, back, top, bottom) = (0,2,1,3);
        let temp = self.faces[top][layer];

        for i in 0..3 {
            self.faces[top][layer][i] = self.faces[front][layer][i];
            self.faces[front][layer][i] = self.faces[bottom][layer][i];
            self.faces[bottom][layer][i] = self.faces[back][layer][i];
            self.faces[back][layer][i] = temp[i];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[4], false);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[5], true);
        }
    }

    fn rotate_face(face: &mut [[Color; 3]; 3], sens_aiguille: bool) {
        let temp = *face;
        
        if sens_aiguille {
            for i in 0..3 {
                for j in 0..3 {
                    face[j][2 - i] = temp[i][j];
                }
            }
        } 
        else {
            for i in 0..3 {
                for j in 0..3 {
                    face[j][i] = temp[i][2-j];
                }
            }
        }
    }

    // fn display(&self) {
    //     println!("{:?}", self.faces);
    // }

    fn display_color(&self) {
        
        for lign in self.faces[4] {
            print!("⬛⬛⬛");
            for color in lign {
                match color {
                    Color::White => print!("⬜"),
                    Color::Blue => print!("🟦"),
                    Color::Yellow => print!("🟨"),
                    Color::Green => print!("🟩"),
                    Color::Red => print!("🟥"),
                    _ => print!("🟧"),
                }
            }
            print!("⬛⬛⬛⬛⬛⬛");
            println!();
        }

        for i in 0..3 {
            for j in 0..4 {
                for k in 0..3 {
                    match self.faces[j][i][k] {
                        Color::White => print!("⬜"),
                        Color::Blue => print!("🟦"),
                        Color::Yellow => print!("🟨"),
                        Color::Green => print!("🟩"),
                        Color::Red => print!("🟥"),
                        _ => print!("🟧"),
                    }
                }
            }
            println!();
        }
        
        for lign in self.faces[5] {
            print!("⬛⬛⬛");
            for color in lign {
                match color {
                    Color::White => print!("⬜"),
                    Color::Blue => print!("🟦"),
                    Color::Yellow => print!("🟨"),
                    Color::Green => print!("🟩"),
                    Color::Red => print!("🟥"),
                    _ => print!("🟧"),
                }
            }
            print!("⬛⬛⬛⬛⬛⬛");
            println!();
        }
    }

    fn _reward(&self) {
        

        // Attention ne pas donnée de récompense si la combinaison à déjà été trouvé au moin une dans le puzzle d'un rubik's cube

        // à répéter pour chaque nouvelle combinaison trouver

            // 2 couleur aligner + bonus si les couleur sont bien positioner

            // 3 couleur aligner + bonus si les couleur sont bien positioner

            // 3 + 1 couleur alignner + bonus si les couleur sont bien positioner

            // 3 + 2 couleur alignner + bonus si les couleur sont bien positioner

            // 3 + 3 couleur aligner + bonus si les couleur sont bien positioner

            // 3 + 3 + 1 couleur alignner + bonus si les couleur sont bien positioner

            // 3 + 3 + 2 couleur alignner + bonus si les couleur sont bien positioner

            // i face couleur aligner + bonus si les couleur sont bien positioner


    }


    // Fonction de récompense
    fn calculate_reward(&self) -> i32 {
        let mut score = 0;

        for face in &self.faces {
            // 1. Vérifier l'alignement des lignes sur chaque face
            for row in face.iter() {
                let row_score = self.calculate_alignment(row);
                score += row_score;
            }

            // 2. Vérifier l'alignement des colonnes
            for col in 0..3 {
                let column = [face[0][col], face[1][col], face[2][col]];
                let column_score = self.calculate_alignment(&column);
                score += column_score;
            }

            // 3. Bonus de face complète
            if face.iter().all(|row| row.iter().all(|&color| color == face[0][0])) {
                score += 10; // Bonus pour une face complète
            }
        }
        
        score
    }

    // Calculer les points d'alignement d'une ligne ou d'une colonne
    fn calculate_alignment(&self, line: &[Color; 3]) -> i32 {
        if line[0] == line[1] && line[1] == line[2] {
            3 // 3 cases alignées
        } else if line[0] == line[1] || line[1] == line[2] {
            1 // 2 cases alignées
        } else {
            0 // Pas d'alignement
        }
    }

    fn random_move(&mut self) {
        let mut rng = rand::thread_rng();
        let axis = rng.gen_range(0..3);
        let layer = rng.gen_range(0..3);
    
        match axis {
            0 => self.rotate_x(layer),
            1 => self.rotate_y(layer),
            _ => self.rotate_z(layer),
        }
    }
}



fn main() {

    let mut cube = RubiksCube::new();
    let mut reward = 0;
    for _ in 0..10000 {
        cube.random_move();
        reward += cube.calculate_reward();
    }
    println!("Score de récompense: {}", reward);
    
    cube.display_color();
    
}