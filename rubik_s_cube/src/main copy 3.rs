fn main() {
    let list_color = ['W','B','Y','G','R','O'];
    let mut rubikscube: [[[char; 3]; 3]; 6] = [[['_';3];3];6];
    let mut rubikscube_ligne_x: [[[char; 3]; 3]; 4] = [[['_';3];3];4];
    let mut rubikscube_ligne_y: [[[char; 3]; 3]; 4] = [[['_';3];3];4];
    let mut rubikscube_ligne_z: [[[char; 3]; 3]; 4] = [[['_';3];3];4];


    //   1
    // 1 1 1 1
    //   1
    for i in 0..list_color.len() {
        rubikscube[i] = [[list_color[i]; 3]; 3];
    }

    //   0
    // 1 1 1 1
    //   0
    for i in 0..rubikscube.len() - 2 {
        for j in 0..rubikscube[i].len() {
            for k in 0..rubikscube[i][j].len() {
                rubikscube_ligne_z[i][j][k] = rubikscube[i][j][k];
            }
        }
    }

    //   1
    // 0 1 0 1
    //   1
    let ind_couleur_y: [usize; 4] = [1,3,4,5];
    for i in 0..rubikscube.len() - 2 {
        for j in 0..rubikscube[i].len() {
            for k in 0..rubikscube[i][j].len() {
                rubikscube_ligne_y[i][j][k] = rubikscube[ind_couleur_y[i]][k][j];
            }
        }
    }

    //   1
    // 1 0 1 0
    //   1
    let ind_couleur_x: [usize; 4] = [0,2,4,5];
    for i in 0..rubikscube.len() - 2 {
        for j in 0..rubikscube[i].len() {
            for k in 0..rubikscube[i][j].len() {
                if i == 0 || i == 2 {
                    rubikscube_ligne_x[i][j][k] = rubikscube[ind_couleur_x[i]][k][j];

                }
                else {
                    rubikscube_ligne_x[i][j][k] = rubikscube[ind_couleur_x[i]][j][k];
                }
            }
        }
    }
    
    println!("{rubikscube:?}");
    println!("{rubikscube_ligne_z:?}");
    println!("{rubikscube_ligne_y:?}");
    println!("{rubikscube_ligne_x:?}");
    
}