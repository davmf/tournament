use std::cmp::Ordering;
use std::fmt;
use std::iter;

pub fn tally(match_results: &str) -> String {

    let mut teams = Teams::new();

    let table = "Team                           | MP |  W |  D |  L |  P".to_string();

    for result_line in match_results.lines() {
        let tokens: Vec<&str> = result_line.split(';').collect();
        let (team1_name, team2_name, result) = (tokens[0], tokens[1], tokens[2]);

        // process team 1
        let team = {
            if let Some(team1) = teams.get_team(team1_name) {
                team1
            } else {
                teams.new_team(Team::new(team1_name))
            }
        };

        match result {
            "win" => team.add_win(),
            "loss" => team.add_loss(),
            "draw" => team.add_draw(),
            &_ => (),
        }
        
        // process team 2
        let team = {
            if let Some(team2) = teams.get_team(team2_name) {
                team2
            } else {
                teams.new_team(Team::new(team2_name))
            }
        };

        match result {
            "win" => team.add_loss(),
            "loss" => team.add_win(),
            "draw" => team.add_draw(),
            &_ => (),
         }
    }

    teams.sort();

    if teams.is_empty() {
        table
    } else {
        format!("{}\n{}", table, teams)
    }
}

#[derive(Debug)]
struct Teams(Vec<Team>);

impl Teams {
    fn new() -> Teams {
        Teams(vec![])
    }

    fn new_team(&mut self, team: Team) -> &mut Team {
        self.0.push(team);
        self.0.last_mut().unwrap()
    }

    fn get_team(&mut self, team_name: &str) -> Option<&mut Team> {
        self.0.iter_mut().find(|team| team.has_name(team_name))
    }

    fn sort(&mut self) {
        self.0.sort();
    }

    fn is_empty(&self) -> bool {
        self.0.is_empty()
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
        for team in &self.0[..self.0.len()-1] {
            writeln!(f, "{}", team)?;
        }
        write!(f, "{}", &self.0[self.0.len()-1])?;
        Ok(())
    }
}

#[derive(Debug, Eq, PartialEq)]
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

impl PartialOrd for Team {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self.points.cmp(&other.points) {
            Ordering::Less => Some(Ordering::Greater),
            Ordering::Greater => Some(Ordering::Less),
            Ordering::Equal => self.name.partial_cmp(&other.name),
        }
    }
}

impl Ord for Team {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
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