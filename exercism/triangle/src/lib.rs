pub struct Triangle{
    sides: [u16; 3],
}

impl Triangle {
    pub fn build(sides: [u16; 3]) -> Result<Triangle, String> {
        let mut n_sides = sides;
        n_sides.sort();
        let (a, b, c) = (n_sides[0], n_sides[1], n_sides[2]);

        if a < 1 || c > a + b {
            Err(String::from("Not a Triangle"))
        } else {
            Ok(Triangle{ sides: sides})
        }
    }

    pub fn is_equilateral(&self) -> bool {
        self.sides[0] == self.sides[1] && self.sides[1] == self.sides[2]
    }

    pub fn is_isosceles(&self) -> bool {
        match self.is_equilateral() {
            true => false,
            false => self.sides[0] == self.sides[1] || self.sides[1] == self.sides[2] || self.sides[2] == self.sides[0],
        }
    }

    pub fn is_scalene(&self) -> bool {
        return !self.is_equilateral() && !self.is_isosceles()
    }

}
