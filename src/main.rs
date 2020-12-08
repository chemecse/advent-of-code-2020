fn load_day_data(day: i32) -> String {
    let filename = format!("data/{:02}.txt", day);
    std::fs::read_to_string(filename).expect("Unable to load data")
}

fn main() {
    // day 01
    {
        println!("Day 01");
        let data = load_day_data(1);
        let values: Vec<i32> = data
            .split("\n")
            .filter(|line| line.len() > 0)
            .map(|line| line.parse::<i32>().unwrap())
            .collect();

        println!("-- Part One");
        let mut result = 0;
        for x in values.iter() {
            let complement: i32 = 2020 - x;
            if values.contains(&complement) {
                result = x * complement;
                println!("{}", result);
                break;
            }
        }
        assert_eq!(result, 365619);

        println!("-- Part Two");
        let values_count: usize = values.len();
        let mut done = false;
        for i in 0..values_count {
            let x = values[i];
            for j in (i + 1)..values_count {
                let y = values[j];
                let xy_sum = x + y;
                for k in (j + 1)..values_count {
                    let z = values[k];
                    if (xy_sum + z) == 2020 {
                        result = x * y * z;
                        println!("{}", result);
                        done = true;
                        break;
                    }
                }
                if done {
                    break;
                }
            }
            if done {
                break;
            }
        }
        assert_eq!(result, 236873508);
    }

    // day 02
    {
        println!("Day 02");
        let data = load_day_data(2);

        println!("-- Part One");
        let result: u32 = data
            .lines()
            .filter(|line| line.len() > 0)
            .map(|line| {
                let fields: Vec<&str> = line.split(' ').collect();
                if fields.len() == 3 {
                    let range = fields[0];
                    let letter = fields[1].chars().nth(0).unwrap();
                    let password = fields[2];
                    let minmax: Vec<i32> = range.split('-').map(|v| v.parse::<i32>().unwrap()).collect();
                    let match_count = password.matches(letter).collect::<Vec<&str>>().len() as i32;
                    if match_count >= minmax[0] && match_count <= minmax[1] {
                        return 1
                    }
                }
                return 0;
            })
            .sum::<u32>();
        println!("{}", result);
        assert_eq!(result, 572);

        println!("-- Part Two");
        let result: u32 = data
            .lines()
            .filter(|line| line.len() > 0)
            .map(|line| {
                let fields: Vec<&str> = line.split(' ').collect();
                if fields.len() == 3 {
                    let positions = fields[0];
                    let letter = fields[1].chars().nth(0).unwrap();
                    let password: Vec<char> = fields[2].chars().collect();
                    let positions: Vec<usize> = positions.split('-').map(|v| v.parse::<usize>().unwrap()).collect();
                    let mut match_count: usize = 0;
                    for idx in positions {
                        if password[idx - 1] == letter {
                            match_count += 1;
                        }
                    }
                    if match_count == 1 {
                        return 1
                    }
                }
                return 0
            })
            .sum::<u32>();
        println!("{}", result);
        assert_eq!(result, 306);
    }
}

