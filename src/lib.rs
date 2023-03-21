use std::fmt;

pub fn tally(match_results: &str) -> String {
    let teams = vec![
        Team::new("Allegoric Alaskans"),
        Team::new("Blithering Badgers"),
        Team::new("Courageous Californians"),
        Team::new("Devastating Donkeys"),
    ];

    let mut table = vec![];
    table.push("Team                           | MP |  W |  D |  L |  P");

    for result_line in match_results.lines() {
        let tokens: Vec<&str> = result_line.split(";").collect();
        let (team1, team2, result) = (tokens[0], tokens[1], tokens[2]);

    }

    "".to_string()
}

struct Team {
    name: String,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
    points: u32,
}

impl Team {
    fn new(name: &str) -> Team {
        Team {
            name: name.to_string(),
            played: 0,
            won: 0,
            drawn: 0,
            lost: 0,
            points: 0,
        }
    }

    fn points(&self) -> u32 {
        self.won * 3 + self.drawn
    }
}


impl fmt::Display for Team {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{:30} | {:2} | {:2} | {:2} | {:2} | {:2}",
            self.name,
            self.played,
            self.won,
            self.drawn,
            self.lost,
            self.points,
        )
    }
}
