pub struct PascalsTriangle{
    row_count: u32,
}

impl PascalsTriangle {
    pub fn new(row_count: u32) -> Self {
        PascalsTriangle { row_count: row_count }
    }

    pub fn rows(&self) -> Vec<Vec<u32>> {
        let mut rows: Vec<Vec<u32>> = Vec::new();

        if self.row_count == 0 {
            rows
        }else{
            rows.push(vec![1]);
            for i in 1..self.row_count {
                let mut t_row: Vec<u32> = vec![1];

                for j in 1..i {
                    t_row.push(PascalsTriangle::pascal_row(i, j) as u32);
                }
                t_row.push(1);
                rows.push(t_row);
            }
            rows
        }
    }

    pub fn pascal_row(row: u32, col: u32) -> u32 {
        if col == 0 || col >= row {
            1
        }else{
            PascalsTriangle::pascal_row(row - 1, col - 1) + PascalsTriangle::pascal_row(row - 1, col)
        }
    }
}
