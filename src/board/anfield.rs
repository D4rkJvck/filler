use {
    super::{
        get_data,
        Matrix,
        Player,
        Size,
    },
    crate::log,
    std::io::{
        self,
        BufRead,
        Result,
    },
};

pub struct Anfield {
    grid:   Matrix,
    size:   Size,
    filler: Player,
    foe:    Player,
}

impl Anfield {
    pub fn get(input: &mut impl BufRead) -> Result<Self> {
        let mut line = String::new();
        input.read_line(&mut line)?;

        let is_p1 = line.contains("p1");
        let filler = if is_p1 { ('a', '@') } else { ('s', '$') };
        let foe = if is_p1 { ('s', '$') } else { ('a', '@') };

        let (grid, size) = get_data(input, "Anfield")?;

        log(format!(
            "{:?}\n{:?}\n{:?}\n{:?}",
            grid, size, filler, foe
        )
        .as_str())?;

        Ok(Self {
            grid,
            size,
            filler,
            foe,
        })
    }

    pub fn update(&mut self, input: &mut impl BufRead) -> io::Result<()> {
        for y in 0..=self.size.y() {
            let mut line = String::new();

            input.read_line(&mut line)?;

            if line.len() > 4 {
                line = (&line[4..]).to_string();
            };

            for (x, c) in line.chars().enumerate() {
                self.grid[y][x] = c
            }
        }

        Ok(())
    }
}
