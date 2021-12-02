use std::fs;

fn main() {
    let contents = fs::read_to_string("data/01_sonar_sweeps.txt").expect("Something went wrong.");
    let measurements = contents.trim_end().split("\n").map(|s| s.parse::<i32>().unwrap());
    let (increases, _) = measurements.fold((0, None), |(increases, prev), item| {
        if let Some(value) = prev {
            if value < item {
                return (increases + 1, Some(item))
            }
        }
        return (increases, Some(item))
    });

    println!("{}", increases);
}
