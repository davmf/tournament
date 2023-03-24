use std::fmt;
use std::iter;

pub fn tally(match_results: &str) -> String {

    let mut teams = Teams::new();

    let table = "Team                           | MP |  W |  D |  L |  P".to_string();

    for result_line in match_results.lines() {
        let tokens: Vec<&str> = result_line.split(';').collect();
        let (team1, team2, result) = (tokens[0], tokens[1], tokens[2]);

         match result {
            "win" => {
                teams.get_team(team1).add_win();
                teams.get_team(team2).add_loss();
            },
            "loss" => {
                teams.get_team(team1).add_loss();
                teams.get_team(team2).add_win();
            },
            "draw" => {
                teams.get_team(team1).add_draw();
                teams.get_team(team2).add_draw();
            },
            &_ => (),
         }
    }

    teams.sort();

    format!("{}\n{}", table, teams)
}

#[derive(Debug)]
struct Teams(Vec<Team>);

impl Teams {
    fn new() -> Teams {
        Teams(vec![])
    }

    fn add_team(team_name: &str) {
        
    }

    fn get_team(&mut self, team_name: &str) -> Option<&mut Team> {
        self.0.iter_mut().find(|team| team.has_name(team_name))
    }

    fn sort(&mut self) {

    }
}

impl iter::IntoIterator for Teams {
    type Item = Team;
    type IntoIter = std::vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }    
}

impl fmt::Display for Teams {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for team in &self.0 {
            writeln!(f, "{}", team)?;
        }
        Ok(())
    }
}

#[derive(Debug, Eq, Ord, PartialEq, PartialOrd)]
struct Team {
    points: u32,
    name: String,
    played: u32,
    won: u32,
    drawn: u32,
    lost: u32,
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

    fn has_name(&self, name: &str) -> bool {
        name == self.name
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

#[cfg(test)]
mod tests {
    // Note this useful idiom: importing names from outer (for mod tests) scope.
    use super::*;

    #[test]
    fn test_results() {
        let results = "Allegoric Alaskans;Blithering Badgers;win\n\
            Devastating Donkeys;Courageous Californians;draw\n\
            Devastating Donkeys;Allegoric Alaskans;win\n\
            Courageous Californians;Blithering Badgers;loss\n\
            Blithering Badgers;Devastating Donkeys;loss\n\
            Allegoric Alaskans;Courageous Californians;win";
        println!("{}", tally(results));
    }

}