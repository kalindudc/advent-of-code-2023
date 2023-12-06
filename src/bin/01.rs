advent_of_code::solution!(1);

pub fn part_one(input: &str) -> Option<u32> {
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut digit = String::new();

        for c in line.chars() {
            if c.is_digit(10) {
                digit.push(c);
                break;
            }
        }

        for c in line.chars().rev() {
            if c.is_digit(10) {
                digit.push(c);
                break;
            }
        }
        if digit.len() == 2 {
          sum += digit.parse::<u32>().unwrap();
        }
    }
    Some(sum)
}

pub fn part_two(input: &str) -> Option<u32> {
    let valid_nums: [&str; 9] = ["one", "two", "three", "four", "five", "six", "seven", "eight", "nine"];
    let mut sum: u32 = 0;

    for line in input.lines() {
        let mut digit = String::new();
        let mut found = false;
        for (i, c) in line.chars().enumerate() {
            if found {
                break;
            }

            if c.is_digit(10) {
                digit.push(c);
                break;
            }

            for (j, num) in valid_nums.iter().enumerate() {
               if i + num.len() < line.len() && &line[i..i+num.len()] == *num {
                    digit += &(j + 1).to_string();
                    found = true;
                    break;
               }
            }
        }

        found = false;
        for (i, c) in line.chars().rev().enumerate() {
            let rev_i = line.len() - i;
            if found {
                break;
            }

            if c.is_digit(10) {
                digit.push(c);
                break;
            }

            for (j, num) in valid_nums.iter().enumerate() {
                if rev_i >= num.len() && &line[rev_i-num.len()..rev_i] == *num {
                    digit += &(j + 1).to_string();
                    found = true;
                    break;
                }
            }
        }
        sum += digit.parse::<u32>().unwrap();
    }
    Some(sum)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(209));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(281));
    }
}
