#[derive(Debug, Clone, Copy)]
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

    fn rotate_x(&mut self, layer: usize) {
        // Rotation logic on X-axis for specified layer
        let (front, back, top, bottom) = (0, 2, 4, 5);
        let mut temp = self.faces[top][layer];

        for i in 0..3 {
            self.faces[top][layer][i] = self.faces[front][layer][i];
            self.faces[front][layer][i] = self.faces[bottom][layer][i];
            self.faces[bottom][layer][i] = self.faces[back][layer][i];
            self.faces[back][layer][i] = temp[i];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[4]);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[5]);
        }
    }

    fn rotate_y(&mut self, layer: usize) {
        // Rotation logic on Y-axis for specified layer
        let (front, back, left, right) = (0, 2, 1, 3);
        let mut temp = [self.faces[left][layer][0], self.faces[left][layer][1], self.faces[left][layer][2]];

        for i in 0..3 {
            self.faces[left][layer][i] = self.faces[front][layer][i];
            self.faces[front][layer][i] = self.faces[right][layer][i];
            self.faces[right][layer][i] = self.faces[back][layer][i];
            self.faces[back][layer][i] = temp[i];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[0]);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[2]);
        }
    }

    fn rotate_z(&mut self, layer: usize) {
        // Rotation logic on Z-axis for specified layer
        let (top, bottom, left, right) = (4, 5, 1, 3);
        let mut temp = self.faces[top][layer];

        for i in 0..3 {
            self.faces[top][layer][i] = self.faces[left][layer][i];
            self.faces[left][layer][i] = self.faces[bottom][layer][i];
            self.faces[bottom][layer][i] = self.faces[right][layer][i];
            self.faces[right][layer][i] = temp[i];
        }

        if layer == 0 {
            Self::rotate_face(&mut self.faces[1]);
        } else if layer == 2 {
            Self::rotate_face(&mut self.faces[3]);
        }
    }

    fn rotate_face(face: &mut [[Color; 3]; 3]) {
        let mut temp = *face;
        for i in 0..3 {
            for j in 0..3 {
                face[j][2 - i] = temp[i][j];
            }
        }
    }

    fn display(&self) {
        println!("{:?}", self.faces);
    }
}

fn main() {
    let mut cube = RubiksCube::new();

    // Afficher le cube initial
    println!("Cube initial:");
    cube.display();

    // Effectuer une rotation sur chaque axe
    println!("\nRotation X:");
    cube.rotate_x(1);
    cube.display();

    println!("\nRotation Y:");
    cube.rotate_y(1);
    cube.display();

    println!("\nRotation Z:");
    cube.rotate_z(1);
    cube.display();
}
