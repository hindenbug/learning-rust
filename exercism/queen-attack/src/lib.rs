pub struct ChessPosition {
    x: i8,
    y: i8,
}

pub struct Queen {
    position: ChessPosition
}

impl ChessPosition {
    pub fn new(x: i8, y: i8) -> Result<ChessPosition, &'static str> {
        if x < 0 || y < 0 || x > 7 || y > 7{
            Err("Invalid Position")
        } else {
            Ok(ChessPosition { x: x, y: y})
        }
    }
}

impl Queen {
    pub fn new(position: ChessPosition) -> Self {
        Queen{ position: position }
    }

    pub fn can_attack(&self, q: &Queen) -> bool {
        self.position.x == q.position.x || self.position.y == q.position.y ||
            (self.position.x - q.position.x).abs() == (self.position.y - q.position.x).abs()
    }
}
