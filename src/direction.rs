#[derive(Copy, Clone)]
/// This is a mask to be applied to a speed
pub enum Direction {
    Forward = 0x80,
    Reverse = 0x00,
}

#[derive(Copy, Clone)]
// TODO(richo) I really hate this name
enum Orientation {
    Cw,
    Ccw,
}
