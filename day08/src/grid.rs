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

        (0..column) // line before
            .map(|i| self.get(line, i))
            .all(|other_tree_height| tree_height > other_tree_height)
            || ((column + 1)..self.width) // line after
                .map(|i| self.get(line, i))
                .all(|other_tree_height| tree_height > other_tree_height)
            || (0..line) // column before
                .map(|i| self.get(i, column))
                .all(|other_tree_height| tree_height > other_tree_height)
            || ((line + 1)..self.height) // column after
                .map(|i| self.get(i, column))
                .all(|other_tree_height| tree_height > other_tree_height)
    }

    pub fn scenic_score(&self, line: usize, column: usize) -> usize {
        let tree_height = self.get(line, column);

        (match (0..column) // check line before
            .rev()
            .map(|i| self.get(line, i))
            .take_while(|height| *height < tree_height)
            .count()
        {
            count if count == column => count,
            count => count + 1,
        }) * (match (column + 1..self.width) // check line after
            .map(|i| self.get(line, i))
            .take_while(|height| *height < tree_height)
            .count()
        {
            count if count == self.width - column - 1 => count,
            count => count + 1,
        }) * (match (0..line) // check column before
            .rev()
            .map(|i| self.get(i, column))
            .take_while(|height| *height < tree_height)
            .count()
        {
            count if count == line => count,
            count => count + 1,
        }) * (match (line + 1..self.height) // check column after
            .map(|i| self.get(i, column))
            .take_while(|height| *height < tree_height)
            .count()
        {
            count if count == self.height - line - 1 => count,
            count => count + 1,
        })
    }
}
