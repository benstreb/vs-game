pub fn move_clamp(
        movement: (i32, i32),
        coord: (i32, i32),
        board: (i32, i32))  -> (i32, i32) {
    let (x, y) = coord;
    let (delta_x, delta_y) = movement;
    let (board_x, board_y) = board;
    (
        clamp(x + delta_x, 0, board_x-1) - x,
        clamp(y + delta_y, 0, board_y-1) - y,
    )
}

pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    return if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}