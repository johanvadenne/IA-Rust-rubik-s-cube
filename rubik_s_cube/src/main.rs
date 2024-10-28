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
            self.faces[bottom][2-layer][2-i] = self.faces[back][2-i][layer];
            self.faces[back][2-i][layer] = temp[i];
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
    }

    fn display(&self) {
        println!("{:?}", self.faces);
    }
}

fn main() {
    // let list_color = ['W','B','Y','G','R','O'];
    // let mut rubikscube: [[[char; 3]; 3]; 6] = [[['_';3];3];6];
    // let mut rubikscube_ligne_x: [[[char; 3]; 3]; 4] = [[['_';3];3];4];
    // let mut rubikscube_ligne_y: [[[char; 3]; 3]; 4] = [[['_';3];3];4];
    // let mut rubikscube_ligne_z: [[[char; 3]; 3]; 4] = [[['_';3];3];4];


    // //   1
    // // 1 1 1 1
    // //   1
    // for i in 0..list_color.len() {
    //     rubikscube[i] = [[list_color[i]; 3]; 3];
    // }

    // //   0
    // // 1 1 1 1
    // //   0
    // for i in 0..rubikscube.len() - 2 {
    //     for j in 0..rubikscube[i].len() {
    //         for k in 0..rubikscube[i][j].len() {
    //             rubikscube_ligne_z[i][j][k] = rubikscube[i][j][k];
    //         }
    //     }
    // }

    // //   1
    // // 0 1 0 1
    // //   1
    // let ind_couleur_y: [usize; 4] = [1,3,4,5];
    // for i in 0..rubikscube.len() - 2 {
    //     for j in 0..rubikscube[i].len() {
    //         for k in 0..rubikscube[i][j].len() {
    //             rubikscube_ligne_y[i][j][k] = rubikscube[ind_couleur_y[i]][k][j];
    //         }
    //     }
    // }

    // //   1
    // // 1 0 1 0
    // //   1
    // let ind_couleur_x: [usize; 4] = [0,2,4,5];
    // for i in 0..rubikscube.len() - 2 {
    //     for j in 0..rubikscube[i].len() {
    //         for k in 0..rubikscube[i][j].len() {
    //             if i == 0 || i == 2 {
    //                 rubikscube_ligne_x[i][j][k] = rubikscube[ind_couleur_x[i]][k][j];

    //             }
    //             else {
    //                 rubikscube_ligne_x[i][j][k] = rubikscube[ind_couleur_x[i]][j][k];
    //             }
    //         }
    //     }
    // }
    
    // println!("{rubikscube:?}");
    // println!("{rubikscube_ligne_z:?}");
    // println!("{rubikscube_ligne_y:?}");
    // println!("{rubikscube_ligne_x:?}");

    let mut cube = RubiksCube::new();
    
    // Afficher le cube initial
    println!("Cube initial:");
    cube.display();

    // Effectuer une rotation sur chaque axe
    println!("\nRotation X:");
    cube.rotate_y(1);
    // Effectuer une rotation sur chaque axe
    println!("\nRotation X:");
    cube.rotate_y(2);
    cube.display();
    // Effectuer une rotation sur chaque axe
    println!("\nRotation X:");
    cube.rotate_y(0);
    cube.display();
    
}