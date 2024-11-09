use rand::Rng;
use std::time::Instant;
// use std::time::Duration;
// use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Hash, Eq)]
enum Color {
    White,
    Blue,
    Yellow,
    Green,
    Red,
    Orange,
}


#[derive(Debug, Clone, PartialEq)]
struct RubiksCube {
    faces: [[[Color; 3]; 3]; 6],
}

#[derive(Debug, Clone)]
struct CoupRubiksCube {
    prec_tour: i32, 
    prec_axis: i32, 
    prec_lign: usize
}

#[derive(Debug, Clone)]
struct DataRubiksCube {
    rubikscube: RubiksCube,
    coups: Vec<CoupRubiksCube>
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

    fn display_color(&self) {

        print!("\x1B[2J\x1B[H");

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

    fn point(&mut self) -> i128 {

        let mut nbr_point: i128 = 0;
        let mut point_attribuet: i128 = 1;
        let mut point_ok = false;

        for i in 0..self.faces.len() {

            if point_ok {
                point_ok = false;
                point_attribuet *= 10;
            }

            for j in 0..self.faces[i].len() {
                for k in 0..self.faces[i][j].len() {
                    let mut multiplicateur: i128 = 1;

                    for l in 0..3-(k+1) {
                        if self.faces[i][j][k] == self.faces[i][j][k+l+1] {
                            nbr_point += point_attribuet * multiplicateur;
                            multiplicateur*=10;
                            point_ok = true;
                        }
                        else {
                            break;
                        }
                    }
                    
                    for l in 0..3-(j+1) {
                        if self.faces[i][j][k] == self.faces[i][j+l+1][k] {
                            nbr_point += point_attribuet * multiplicateur;
                            multiplicateur*=10;
                            point_ok = true;
                        }
                        else {
                            break;
                        }
                    }
                }
            }
        }

        nbr_point
    }

    fn prevision(&mut self,vec_rubikscuble: Vec<DataRubiksCube>, vec_rubikscuble_noire: &Vec<RubiksCube>, check: bool) -> (i128, i32, i32, usize, Vec<DataRubiksCube>, DataRubiksCube) {

        let mut best_tour = 0;
        let mut best_axis = 0;
        let mut best_lign = 0;
        let mut best_point = 0;
        let mut vec_rubikscuble_bis: Vec<DataRubiksCube> = Vec::new();
        let mut best_cube: DataRubiksCube = DataRubiksCube{rubikscube: vec_rubikscuble[0].rubikscube.clone(), coups: vec![]};

        for rubikcube_v in vec_rubikscuble {
            
            for axis in 0..3 {
                
                for lign in 0..3 {

                    for tour in 0..3 {

                        let coups = rubikcube_v.coups.clone();
                        let tour_bis = if tour == 0 {2} else {0};

                        if coups.len() >= 1 {
                            if coups[coups.len()-1].prec_tour == tour_bis && coups[coups.len()-1].prec_axis == axis && coups[coups.len()-1].prec_lign == lign {
                                continue;
                            }
                        }
                        
                        let mut rubikcube = rubikcube_v.rubikscube.clone();

                        for _ in 0..tour+1 {
                            
                            match axis {
                                0 => rubikcube.rotate_x(lign),
                                1 => rubikcube.rotate_y(lign),
                                _ => rubikcube.rotate_z(lign),
                            }

                        }

                        if check && vec_rubikscuble_noire.contains(&rubikcube) {
                            continue;
                        }

                        let mut coups = rubikcube_v.coups.clone();
                        coups.push(CoupRubiksCube{prec_axis:axis, prec_lign:lign, prec_tour:tour});
                        let point = rubikcube.point();
                        vec_rubikscuble_bis.push(DataRubiksCube{rubikscube:rubikcube.clone(), coups: coups.clone()});

                        if best_point < point {
                            best_point = point;
                            best_tour = tour+1;
                            best_axis = axis;
                            best_lign = lign;
                            best_cube = DataRubiksCube{rubikscube:rubikcube.clone(), coups: coups.clone()};
                        }
                    }
                }
            }
        }

        (best_point, best_tour, best_axis, best_lign, vec_rubikscuble_bis, best_cube)
    }

    fn validate(&mut self) -> bool {

        let mut valide = true;

        for face in self.faces {
            for line in face {
                let color_r = line[0];
                for color in line {
                    if color != color_r {
                        valide = false;
                        break;
                    }
                }
            }
        }

        
        valide
    }

    
}



fn main() {

    let start = Instant::now();

    let mut cube = RubiksCube::new();
    let mut liste_noireVec: Vec<RubiksCube> = Vec::new();
    // let mut reward = 0;
    for _ in 0..10000 {
        cube.random_move();
    }
    
    while !cube.validate() {

        let (mut best_point, mut _best_tour, mut _best_axis, mut _best_lign, mut vec_rubikscuble, mut best_cube) = cube.prevision(vec![DataRubiksCube{rubikscube:cube.clone(), coups: vec![]}], &liste_noireVec, false);


        for i in 0..3 {
            (best_point, _best_tour, _best_axis, _best_lign, vec_rubikscuble, best_cube) = cube.prevision(vec_rubikscuble, &liste_noireVec, i==3);
        }

        for _ in 0..best_cube.coups[0].prec_tour+1 {
            match best_cube.coups[0].prec_axis {
                0 => cube.rotate_x(best_cube.coups[0].prec_lign.clone()),
                1 => cube.rotate_y(best_cube.coups[0].prec_lign.clone()),
                _ => cube.rotate_z(best_cube.coups[0].prec_lign.clone()),
            }
        }
        
        cube.display_color();
        let nbr = cube.point();
        println!("{:?}", nbr);
        println!("{:?}", best_point);
        liste_noireVec.push(cube.clone());
        // thread::sleep(Duration::from_millis(1000));
    }
    
    cube.display_color();
    println!("Durée d'exécution: {:?}", start.elapsed());
    
}