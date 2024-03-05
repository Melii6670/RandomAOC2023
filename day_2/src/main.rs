use std::fs;
use std::time::Instant;

const RED_MAX: u32 = 12;
const GREEN_MAX: u32 = 13;
const BLUE_MAX: u32 = 14;

fn main() {
    let data = fs::read_to_string("data.txt").unwrap();

    let mut timer = Instant::now();
    let part_1_result = part_1(&data);
    let part_1_time = timer.elapsed();
    println!("part 1: {part_1_result}, completed in {:.2?}", part_1_time);

    timer = Instant::now();
    let part_2_result = part_2(&data);
    let part_2_time = timer.elapsed();
    println!("part 2: {part_2_result}, completed in {:.2?}", part_2_time);
}

fn part_1(input_data: &String) -> u32 {
    input_data
        .split("\r\n")
        .filter_map(|game| parse_game_part_1(game))
        .sum()
}

fn part_2(input_data: &String) -> u32 {
    input_data
        .split("\r\n")
        .map(|game| parse_game_part_2(game))
        .sum()
}

fn parse_game_part_1(input: &str) -> Option<u32> {
    let game_and_id_separated = input.split(": ").collect::<Vec<&str>>();

    let game_id: u32 = game_and_id_separated
        .get(0)
        .unwrap()
        .split(" ")
        .collect::<Vec<&str>>()
        .get(1)
        .unwrap()
        .parse()
        .unwrap();

    let colours = game_and_id_separated
        .get(1)
        .unwrap()
        .split("; ")
        .map(|round| get_colours_array_from_round(round))
        .collect::<Vec<[u32; 3]>>();

    for round in colours {
        if round[0] > RED_MAX {
            return None;
        }
        if round[1] > GREEN_MAX {
            return None;
        }
        if round[2] > BLUE_MAX {
            return None;
        }
    }

    return Some(game_id);
}

fn parse_game_part_2(input: &str) -> u32 {
    let game_and_id_separated = input.split(": ").collect::<Vec<&str>>();

    let colours = game_and_id_separated
        .get(1)
        .unwrap()
        .split("; ")
        .map(|round| get_colours_array_from_round(round))
        .collect::<Vec<[u32; 3]>>();

    let mut min_colours: [u32; 3] = [0, 0, 0];

    for round in colours {
        if round[0] > min_colours[0] {
            min_colours[0] = round[0];
        }
        if round[1] > min_colours[1] {
            min_colours[1] = round[1];
        }
        if round[2] > min_colours[2] {
            min_colours[2] = round[2];
        }
    }

    return min_colours[0] * min_colours[1] * min_colours[2];
}

fn get_colours_array_from_round(round: &str) -> [u32; 3] {
    let mut colours = [0, 0, 0];
    for colour_listing_split in round.split(", ") {
        let colour_and_name = colour_listing_split.split(" ").collect::<Vec<&str>>();
        
        match *colour_and_name.get(1).unwrap() {
            "red" => colours[0] = colour_and_name.get(0).unwrap().parse().unwrap(),
            "green" => colours[1] = colour_and_name.get(0).unwrap().parse().unwrap(),
            "blue" => colours[2] = colour_and_name.get(0).unwrap().parse().unwrap(),
            _ => {}
        }
    }
    colours
}

#[test]
fn test_get_colours() {
    assert_eq!(
        get_colours_array_from_round("3 blue, 4 green, 2 red"),
        [2, 4, 3]
    );
}

#[test]
fn test_part_1_game_1() {
    assert_eq!(
        parse_game_part_1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        Some(1)
    );
}

#[test]
fn test_part_1_game_2() {
    assert_eq!(
        parse_game_part_1("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
        Some(2)
    );
}

#[test]
fn test_part_1_game_3() {
    assert_eq!(
        parse_game_part_1(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
        ),
        None
    );
}

#[test]
fn test_part_1_game_4() {
    assert_eq!(
        parse_game_part_1(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
        ),
        None
    );
}

#[test]
fn test_part_1_game_5() {
    assert_eq!(
        parse_game_part_1("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        Some(5)
    );
}

#[test]
fn test_part_1_game_6() {
    assert_eq!(
        parse_game_part_1("Game 6: 20 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        None
    );
}

#[test]
fn test_part_1() {
    assert_eq!(part_1(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\r\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\r\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\r\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\r\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()), 8);
}

#[test]
fn test_part_2_game_1() {
    assert_eq!(
        parse_game_part_2("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green"),
        48
    );
}

#[test]
fn test_part_2_game_2() {
    assert_eq!(
        parse_game_part_2("Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue"),
        12
    );
}

#[test]
fn test_part_2_game_3() {
    assert_eq!(
        parse_game_part_2(
            "Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red"
        ),
        1560
    );
}

#[test]
fn test_part_2_game_4() {
    assert_eq!(
        parse_game_part_2(
            "Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red"
        ),
        630
    );
}

#[test]
fn test_part_2_game_5() {
    assert_eq!(
        parse_game_part_2("Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green"),
        36
    );
}

#[test]
fn test_part_2() {
    assert_eq!(part_2(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\r\nGame 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\r\nGame 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\r\nGame 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\r\nGame 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()), 2286);
}
