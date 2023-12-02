// #![allow(dead_code)]
use std::collections::HashMap;

#[derive(Debug)]
struct Game {
    _id: usize,
    _valid: bool,
}

impl Game {
    fn new_invalid(id: usize) -> Self {
        Self {
            _id: id,
            _valid: false,
        }
    }

    fn new_valid(id: usize) -> Self {
        Self {
            _id: id,
            _valid: true,
        }
    }

    fn id(&self) -> usize {
        self._id
    }

    fn is_valid(&self) -> bool {
        self._valid
    }
}

#[derive(Debug)]
struct GameResult {
    _red: usize,
    _blue: usize,
    _green: usize,
}

impl GameResult {
    fn new(tmp: &HashMap<&str, usize>) -> Self {
        let red = *tmp.get("red").unwrap_or(&0);
        let blue = *tmp.get("blue").unwrap_or(&0);
        let green = *tmp.get("green").unwrap_or(&0);
        Self {
            _red: red,
            _blue: blue,
            _green: green,
        }
    }

    fn calculate_power(&self) -> usize {
        return self._red * self._blue * self._green;
    }
}

#[derive(Debug)]
struct InputGame<'a> {
    _id: usize,
    _games: Vec<&'a str>, // specify the lifetime for the string references
}

impl<'a> InputGame<'a> {
    /// Creates a new [`InputGame`].
    fn new(id: usize, games: Vec<&'a str>) -> Self {
        Self {
            _id: id,
            _games: games,
        }
    }

    fn id(&self) -> usize {
        self._id
    }

    fn games(&self) -> &[&str] {
        self._games.as_ref()
    }
}

fn parse_input<'a>(input: &[&'a str]) -> Vec<InputGame<'a>> {
    let mut result = Vec::new();

    for (index, game) in input.iter().enumerate() {
        if let Some(subsets_str) = game.split(": ").nth(1) {
            let subsets: Vec<&str> = subsets_str.split("; ").collect();
            //loop iters from 0 but we got games from 1
            let input_game = InputGame::new(index + 1, subsets);
            result.push(input_game);
        } else {
            // Handle cases where the input string format is incorrect
            println!("Invalid format for game at index {}", index);
            return Vec::new(); // or any other error representation
        }
    }

    result
}

fn exe1(input: Vec<&str>) -> usize {
    fn check_game_posibility<'a>(input: &InputGame) -> Game {
        let configuration: HashMap<&str, i32> =
            HashMap::from_iter([("red", 12), ("green", 13), ("blue", 14)]);

        for subset in input.games() {
            let mut tmp: HashMap<&str, i32> = HashMap::new();

            for part in subset.split(",") {
                if let [amount_str, colour] =
                    part.split_whitespace().collect::<Vec<&str>>().as_slice()
                {
                    if let Ok(amount) = amount_str.parse::<i32>() {
                        tmp.insert(colour, amount);
                    }
                }
            }

            for (ele, &max) in &configuration {
                if let Some(&exist) = tmp.get(ele) {
                    if max < exist {
                        return Game::new_invalid(input.id());
                    }
                }
            }
        }

        Game::new_valid(input.id())
    }

    parse_input(&input)
        .iter()
        .map(|f: &InputGame<'_>| check_game_posibility(f))
        .filter(|game: &Game| game.is_valid())
        .map(|game: Game| game.id())
        .fold(0, |acc: usize, id: usize| acc + id)
}

fn exe2(input: Vec<&str>) -> usize {
    fn check_game_posibility<'a>(input: &InputGame) -> GameResult {
        let mut tmp = HashMap::new();
        for subset in input.games() {
            let parts: Vec<&str> = subset.split(",").collect();

            for part in parts {
                let amount: usize = part.split_whitespace().nth(0).unwrap().parse().unwrap();
                let colour = part.split_whitespace().nth(1).unwrap();

                let current: &mut usize = tmp.entry(colour).or_insert(0);

                //dereference mutable to immutable value
                if *current < amount {
                    *tmp.entry(colour).or_insert(0) = amount;
                }
            }
        }

        GameResult::new(&tmp)
    }

    parse_input(&input)
        .iter()
        .map(|game: &InputGame<'_>| check_game_posibility(game))
        .map(|result: GameResult| result.calculate_power())
        .fold(0, |acc: usize, current: usize| acc + current)
}

#[cfg(test)]
mod day2tests {
    use super::{exe1, exe2};
    use crate::utils::read_lines;

    fn get_input_file() -> String {
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join("resources")
            .join("test")
            .join("day2.txt")
            .to_string_lossy()
            .to_string()
    }

    #[test]
    fn should_resolve_dummy_input_1() {
        let input: Vec<&str> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let result = exe1(input);

        assert_eq!(result, 8);
    }

    #[test]
    fn should_resolve_solution_1() {
        let input: Vec<String> = read_lines(get_input_file());

        let result = exe1(input.iter().map(String::as_str).collect());

        assert_eq!(result, 2528);
    }

    #[test]
    fn should_resolve_dummy_input_2() {
        let input: Vec<&str> = vec![
            "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green",
            "Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue",
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red",
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red",
            "Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green",
        ];

        let result = exe2(input);

        assert_eq!(result, 2286);
    }

    #[test]
    fn should_resolve_solution_2() {
        let input: Vec<String> = read_lines(get_input_file());

        let result = exe2(input.iter().map(String::as_str).collect());

        assert_eq!(result, 67363);
    }
}
