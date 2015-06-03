pub fn clamp<T: PartialOrd>(value: T, min: T, max: T) -> T {
    return if value < min {
        min
    } else if value > max {
        max
    } else {
        value
    }
}