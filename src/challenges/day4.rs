// #![allow(dead_code)]
#[derive(Debug, Clone)]
struct InputGame<'a> {
    _id: usize,
    _games: Vec<&'a str>, // specify the lifetime for the string references
    _my_numbers: Vec<i32>,
    _winning_numbers: Vec<i32>,
}

impl<'a> InputGame<'a> {
    /// Creates a new [`InputGame`].
    fn new(id: usize, games: Vec<&'a str>, my: Vec<i32>, winning: Vec<i32>) -> Self {
        Self {
            _id: id,
            _games: games,
            _my_numbers: my,
            _winning_numbers: winning,
        }
    }

    fn games(&self) -> &[&str] {
        self._games.as_ref()
    }

    fn my_numbers(&self) -> &[i32] {
        self._my_numbers.as_ref()
    }

    fn winning_numbers(&self) -> &[i32] {
        self._winning_numbers.as_ref()
    }
}

fn parse_input<'a>(input: &[&'a str]) -> Vec<InputGame<'a>> {
    let mut result = Vec::new();

    for (index, game) in input.iter().enumerate() {
        if let Some(subsets_str) = game.split(": ").nth(1) {
            let subsets: Vec<&str> = subsets_str.split("|").collect();

            let my_numbers: Vec<i32> = subsets
                .get(0)
                .expect("There must be my numbers")
                .split(" ")
                .into_iter()
                .filter(|e| !e.is_empty())
                .map(|f| f.parse::<i32>().expect("Failed to parse integer"))
                .collect();
            let winning_numbers: Vec<i32> = subsets
                .get(1)
                .expect("There msut be winning numbers")
                .split(" ")
                .into_iter()
                .filter(|e| !e.is_empty())
                .map(|f| f.parse::<i32>().expect("Failed to parse integer"))
                .collect();

            let input_game = InputGame::new(index + 1, subsets, my_numbers, winning_numbers);
            result.push(input_game);
        } else {
            // Handle cases where the input string format is incorrect
            println!("Invalid format for game at index {}", index);
            return Vec::new(); // or any other error representation
        }
    }

    result
}

fn check_game_posibility(input: &InputGame) -> usize {
    let set1: std::collections::HashSet<_> = input.my_numbers().into_iter().collect();
    let set2: std::collections::HashSet<_> = input.winning_numbers().into_iter().collect();

    let intersection: Vec<_> = set1.intersection(&set2).cloned().collect();

    return intersection.len();
}

fn exe1(input: Vec<&str>) -> usize {
    let inout = parse_input(&input);
    let mut result = 0;

    for f in inout.iter() {
        let e = check_game_posibility(f);
        if e == 0 {
            result += 0;
        } else {
            let mut sum = 1;
            for _ in 0..e - 1 {
                sum *= 2;
            }
            result += sum;
        }
    }

    result
}

fn exe2(input: Vec<&str>) -> usize {
    let input: Vec<InputGame> = parse_input(&input);
    let mut factors: [usize; 256] = [1usize; 256];
    let mut sum = 0;

    for (i, ele) in input.into_iter().enumerate() {
        let factor: usize = factors[i];
        let win_count = check_game_posibility(&ele);
        for i in i..i + win_count {
            factors[i + 1] += factor
        }
        sum += factor * win_count + 1
    }

    return sum;
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::read_lines;

    fn get_input_file() -> String {
        std::env::current_dir()
            .expect("Failed to get current directory")
            .join("resources")
            .join("test")
            .join("day4.txt")
            .to_string_lossy()
            .to_string()
    }

    #[test]
    fn should_resolve_dummy_input_1() {
        let input: Vec<&str> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let result = exe1(input);

        assert_eq!(result, 13);
    }

    #[test]
    fn should_resolve_solution_1() {
        let input: Vec<String> = read_lines(get_input_file());

        let result = exe1(input.iter().map(String::as_str).collect());

        assert_eq!(result, 22488);
    }

    #[test]
    fn should_resolve_dummy_input_2() {
        let input: Vec<&str> = vec![
            "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53",
            "Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19",
            "Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1",
            "Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83",
            "Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36",
            "Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11",
        ];

        let result = exe2(input);

        assert_eq!(result, 30);
    }

    #[test]
    fn should_resolve_solution_2() {
        let input: Vec<String> = read_lines(get_input_file());

        let result = exe2(input.iter().map(String::as_str).collect());

        //to low
        assert_eq!(result, 7013204);
    }
}
