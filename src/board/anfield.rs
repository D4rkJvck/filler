use {
    super::{
        get_data,
        Matrix,
        Player,
        Size,
    },
    std::io::{
        self,
        BufRead,
    },
};

pub struct Anfield {
    grid:   Matrix,
    size:   Size,
    filler: Player,
    foe:    Player,
}

impl Anfield {
    pub fn get(
        input: &mut impl BufRead,
        players: (Player, Player),
    ) -> io::Result<Self> {
        let (grid, size) = get_data(input, "Anfield")?;
        let (filler, foe) = players;

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
