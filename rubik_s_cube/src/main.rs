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
            Self::rotate_face(&mut self.faces[2], false);
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

    fn _reward(&self) -> i32 {
        // Reward based on number of completed rows of same color
        let mut score = 0;
        for face in &self.faces {
            for row in face.iter() {
                if row.iter().all(|&color| color == row[0]) {
                    score += 1;
                }
            }
        }
        score
    }

    fn _is_solved(&self) -> bool {
        self.faces.iter().all(|face| {
            face.iter().all(|row| row.iter().all(|&color| color == face[0][0]))
        })
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
    for _ in 0..1000 {
        cube.random_move();
    }
    
    cube.display_color();
    
}