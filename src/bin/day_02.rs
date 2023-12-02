use advent_of_code_2023::utils::*;

fn main() {
    let mut games: Vec<Game> = vec![];

    for line in lines_to_vector(read_in_data("02.1.1")) {
        games.push(Game::new(&line));
    }

    let mut marbles_existing: Vec<Marbles> = vec![];

    for line in lines_to_vector(read_in_data("02.1.2")) {
        marbles_existing.push(Marbles::new(&line));
    }

    let mut games_sum = 0;
    for game in &games {
        let game_result = game.game_could_exist(&marbles_existing);
        println!("{:?}, {:?}", game_result, game);
        if game_result.0 {
            games_sum = games_sum + game_result.1;
        }
    }

    let mut games_power = 0;
    for game in &games {
        games_power = games_power + game.game_power();
    }

    println!("games_sum: {}", games_sum);
    println!("games_power: {}", games_power);
}

#[derive(Debug)]
struct Game {
    no: i64,
    rounds: Vec<Round>,
}

impl Game {
    fn new(input: &str) -> Self {
        let splits = input.rsplit_once(":").unwrap();
        let game_number = splits.0.replace("Game ", "");

        let mut game = Game {
            no: game_number.parse::<i64>().unwrap(),
            rounds: vec![],
        };

        for round in splits.1.split(";") {
            game.rounds.push(Round::new(round));
        }

        game
    }

    fn game_could_exist(&self, marbles: &Vec<Marbles>) -> (bool, i64) {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for round in &self.rounds {
            for mables in &round.marbles {
                match mables {
                    Marbles::Blue(count) => {
                        if count > &blue_max {
                            blue_max = count.clone();
                        };
                    }
                    Marbles::Red(count) => {
                        if count > &red_max {
                            red_max = count.clone();
                        };
                    }
                    Marbles::Green(count) => {
                        if count > &green_max {
                            green_max = count.clone();
                        };
                    }
                    Marbles::Unknown => panic!("wow these don't exist"),
                }
            }
        }

        for marble_count in marbles {
            match marble_count {
                Marbles::Blue(count) => {
                    if count < &blue_max {
                        return (false, self.no);
                    }
                }
                Marbles::Red(count) => {
                    if count < &red_max {
                        return (false, self.no);
                    }
                }
                Marbles::Green(count) => {
                    if count < &green_max {
                        return (false, self.no);
                    }
                }
                Marbles::Unknown => panic!("wow these don't exist"),
            }
        }

        (true, self.no)
    }

    fn game_power(&self) -> i64 {
        let mut red_max = 0;
        let mut green_max = 0;
        let mut blue_max = 0;

        for round in &self.rounds {
            for mables in &round.marbles {
                match mables {
                    Marbles::Blue(count) => {
                        if count > &blue_max {
                            blue_max = count.clone();
                        };
                    }
                    Marbles::Red(count) => {
                        if count > &red_max {
                            red_max = count.clone();
                        };
                    }
                    Marbles::Green(count) => {
                        if count > &green_max {
                            green_max = count.clone();
                        };
                    }
                    Marbles::Unknown => panic!("wow these don't exist"),
                }
            }
        }
        println!("{:?}{:?}",self, (green_max, red_max, blue_max));
        (red_max as i64) * (green_max as i64) * (blue_max as i64)
    }
    
}

#[derive(Debug)]
struct Round {
    marbles: Vec<Marbles>,
}

impl Round {
    fn new(input: &str) -> Self {
        let mut round = Round { marbles: vec![] };

        for marbles in input.split(",") {
            round.marbles.push(Marbles::new(marbles));
        }

        round
    }
}

#[derive(Debug)]
enum Marbles {
    Blue(i32),
    Red(i32),
    Green(i32),
    Unknown,
}

impl Marbles {
    fn new(input: &str) -> Self {
        let splits = input.rsplit_once(" ").unwrap();
        if splits.1 == "blue" {
            return Marbles::Blue(splits.0.trim().parse::<i32>().unwrap());
        };
        if splits.1 == "red" {
            return Marbles::Red(splits.0.trim().parse::<i32>().unwrap());
        };
        if splits.1 == "green" {
            return Marbles::Green(splits.0.trim().parse::<i32>().unwrap());
        };
        Marbles::Unknown
    }
}
