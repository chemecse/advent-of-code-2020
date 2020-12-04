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
        for x in values.iter() {
            let complement: i32 = 2020 - x;
            if values.contains(&complement) {
                println!("{}", x * complement);
                break;
            }
        }

        println!("-- Part Two");
        let values_count: usize = values.len();
        let mut done = false;
        for i in 0..values_count {
            let x = values[i];
            for j in (i + 1)..values_count {
                let y = values[j];
                for k in (j + 1)..values_count {
                    let z = values[k];
                    if (x + y + z) == 2020 {
                        println!("{}", x * y * z);
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
    }
}

