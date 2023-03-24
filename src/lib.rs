use std::fmt;
use std::collections::HashMap;

pub fn tally(match_results: &str) -> String {

    let team_names = vec![
        "Allegoric Alaskans",
        "Blithering Badgers",
        "Courageous Californians",
        "Devastating Donkeys",      
    ];

    let mut teams = HashMap::new();

    for team_name in team_names {
        teams.insert(team_name, Team::new(team_name));
    }

    let mut table = vec![];
    table.push("Team                           | MP |  W |  D |  L |  P");

    for result_line in match_results.lines() {
        let tokens: Vec<&str> = result_line.split(";").collect();
        let (team1, team2, result) = (tokens[0], tokens[1], tokens[2]);

        match result {
            "win" => {
                teams.get_mut(team1).map(|val| val.add_win());
                teams.get_mut(team2).map(|val| val.add_loss());
            },
            "loss" => {
                teams.get_mut(team1).map(|val| val.add_loss());
                teams.get_mut(team2).map(|val| val.add_win());
            },
            "draw" => {
                teams.get_mut(team1).map(|val| val.add_draw());
                teams.get_mut(team2).map(|val| val.add_draw());
            },
            &_ => (),
        }
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

    fn add_win(&mut self) {
        self.played += 1;
        self.won += 1;
        self.points += 3;
    }

    fn add_draw(&mut self) {
        self.played += 1;
        self.drawn += 1;
        self.points += 1;
    }

    fn add_loss(&mut self) {
        self.played += 1;
        self.lost += 1;
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
