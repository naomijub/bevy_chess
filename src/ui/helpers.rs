pub fn square_to_coord(square: &(i8, i8)) -> String {
    match square {
        (x, 0) => format!("{}{}", x + 1, 'a'),
        (x, 1) => format!("{}{}", x + 1, 'b'),
        (x, 2) => format!("{}{}", x + 1, 'c'),
        (x, 3) => format!("{}{}", x + 1, 'd'),
        (x, 4) => format!("{}{}", x + 1, 'e'),
        (x, 5) => format!("{}{}", x + 1, 'f'),
        (x, 6) => format!("{}{}", x + 1, 'g'),
        (x, 7) => format!("{}{}", x + 1, 'h'),
        _ => unreachable!("Invalid square: {:?}", square),
    }
}
