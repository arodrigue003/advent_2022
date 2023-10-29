pub struct Grid {
    data: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Grid {
    pub fn from_str(data: &str) -> Grid {
        let data: Vec<Vec<u8>> = data
            .lines()
            .map(|line| line.chars().map(|char| char as u8 - 48).collect())
            .collect();

        let width = data[0].len();
        let height = data.len();

        Grid {
            data,
            width,
            height,
        }
    }

    #[inline(always)]
    pub fn height(&self) -> usize {
        self.height
    }

    #[inline(always)]
    pub fn width(&self) -> usize {
        self.width
    }

    #[inline(always)]
    fn get(&self, line: usize, column: usize) -> u8 {
        self.data[line][column]
    }

    pub fn pretty_print(&self) {
        for line in &self.data {
            for column in line {
                print!("{}", column)
            }
            println!();
        }
    }

    pub fn is_visible(&self, line: usize, column: usize) -> bool {
        let tree_height = self.get(line, column);

        // Check line before
        if (0..column)
            .map(|i| self.get(line, i))
            .all(|other_tree_height| tree_height > other_tree_height)
        {
            return true;
        }

        // check line after
        if ((column + 1)..self.width)
            .map(|i| self.get(line, i))
            .all(|other_tree_height| tree_height > other_tree_height)
        {
            return true;
        }

        // check column before
        if (0..line)
            .map(|i| self.get(i, column))
            .all(|other_tree_height| tree_height > other_tree_height)
        {
            return true;
        }

        ((line + 1)..self.height)
            .map(|i| self.get(i, column))
            .all(|other_tree_height| tree_height > other_tree_height)
    }

    pub fn scenic_score(&self, line: usize, column: usize) -> usize {
        let tree_height = self.get(line, column);

        // Check line after
        let mut view_distance: usize = 0;
        for col in (column + 1..self.width) {
            // if self.get(line, col)
        }

        view_distance
    }
}
