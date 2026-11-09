mod map;
mod player;

use map::Map;

fn main() {
    let ma_carte = Map {
        long: 5,
        larg: 5,
        x_depart: 0,
        y_depart: 0,

        grille: vec![vec!['.'; 5]; 5]
    };
}


