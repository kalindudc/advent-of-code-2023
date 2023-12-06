advent_of_code::solution!(2);

static TOTAL_RED: u32 = 12;
static TOTAL_GREEN: u32 = 13;
static TOTAL_BLUE: u32 = 14;

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let game_split = line.split(": ").collect::<Vec<&str>>();
        let id: u32 = game_split[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let sets = game_split[1].split("; ").collect::<Vec<&str>>();

        // count colours in each set
        let mut invalid = false;
        for set in sets {
            let colour_split = set.split(", ").collect::<Vec<&str>>();
            for colour in colour_split {
                let colour_count = colour.split(" ").collect::<Vec<&str>>();
                let colour_id: &str = colour_count[1];
                let colour_total: u32 = colour_count[0].parse::<u32>().unwrap();

                if (colour_id == "red" && colour_total > TOTAL_RED) ||
                (colour_id == "green" && colour_total > TOTAL_GREEN) ||
                (colour_id == "blue" && colour_total > TOTAL_BLUE) {
                    invalid = true;
                    break;
                }
            }

            if invalid {
                break;
            }
        }

        if !invalid {
            sum += id;
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let mut sum = 0;
    for line in input.lines() {
        let game_split = line.split(": ").collect::<Vec<&str>>();
        let id: u32 = game_split[0].split(" ").collect::<Vec<&str>>()[1].parse::<u32>().unwrap();
        let sets = game_split[1].split("; ").collect::<Vec<&str>>();

        // count colours in each set
        let mut max_red = 0;
        let mut max_green = 0;
        let mut max_blue = 0;
        for set in sets {
            let colour_split = set.split(", ").collect::<Vec<&str>>();
            for colour in colour_split {
                let colour_count = colour.split(" ").collect::<Vec<&str>>();
                let colour_id: &str = colour_count[1];
                let colour_total: u32 = colour_count[0].parse::<u32>().unwrap();

                if colour_id == "red" && colour_total > max_red {
                    max_red = colour_total;
                } else if colour_id == "green" && colour_total > max_green {
                    max_green = colour_total;
                } else if colour_id == "blue" && colour_total > max_blue {
                    max_blue = colour_total;
                }
            }
        }

        sum += max_red * max_green * max_blue;
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
}
