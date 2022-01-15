use std::cmp;

type TargetArea = ((u32, i32), (u32, i32));
const PUZZLE_TARGET: TargetArea = ((169, -68), (206, -108));
const EXAMPLE_TARGET: TargetArea = ((20, -5), (30, -10));

#[derive(Copy, Clone, Debug)]
struct Probe {
    pos: (u32, i32),
    vel: (u32, i32),
}

fn main() {
    let target = &PUZZLE_TARGET;
    let max_y_traj = find_max_y_trajectory(target).unwrap();
    let num_valid_trajectories = get_num_valid_trajectories(target);
    println!("Best vel: {:?}", max_y_traj[0].vel);
    println!(
        "Max y: {}",
        max_y_traj
            .into_iter()
            .fold(0, |max_y, p| cmp::max(max_y, p.pos.1))
    );
    println!("Num valid trajectories: {}", num_valid_trajectories);
}

fn find_max_y_trajectory(target: &TargetArea) -> Option<Vec<Probe>> {
    let mut best_traj = None;

    for y_vel in 1..=(-target.1 .1) {
        let mut best_traj_at_y = None;
        for x_vel in 1..=target.1 .0 {
            let trajectory = get_target_trajectory(
                &Probe {
                    pos: (0, 0),
                    vel: (x_vel, y_vel),
                },
                target,
            );

            match trajectory {
                Ok(path) => {
                    best_traj_at_y = Some(path);
                    break;
                }
                _ => {}
            }
        }

        // If we found a valid trajectory at this y level, store it as the best and move on
        if let Some(path) = best_traj_at_y {
            best_traj = Some(path);
        }
    }
    return best_traj;
}

/// Get the number of valid initial trajectories that land in the target area
fn get_num_valid_trajectories(target: &TargetArea) -> u32 {
    let mut num_trajectories = 0;

    for y_vel in target.1 .1..=(-target.1 .1) {
        for x_vel in 1..=target.1 .0 {
            let trajectory = get_target_trajectory(
                &Probe {
                    pos: (0, 0),
                    vel: (x_vel, y_vel),
                },
                target,
            );

            if let Ok(_) = trajectory {
                num_trajectories += 1;
            }
        }
    }
    return num_trajectories;
}

/// Given an initial probe's parameters, find the trajectory into the target area
/// # Returns
/// Ok(trajectory) trajectory of the probe if it lands in the target
/// Err(trajectory) trajectory of the probe if it overshoots the target
fn get_target_trajectory(
    initial_probe: &Probe,
    target: &TargetArea,
) -> Result<Vec<Probe>, Vec<Probe>> {
    let mut current_probe = initial_probe.clone();
    let mut trajectory = vec![current_probe];

    while !has_overshot(target, &current_probe) {
        current_probe = step(&current_probe);
        trajectory.push(current_probe);

        if inside_target(target, &current_probe) {
            return Ok(trajectory);
        }
    }

    return Err(trajectory);
}

/// Whether the probe has overshot the target
fn has_overshot(target: &TargetArea, probe: &Probe) -> bool {
    probe.pos.0 > target.1 .0 || probe.pos.1 < target.1 .1
}

/// Whether the probe has landed within the target
fn inside_target(target: &TargetArea, probe: &Probe) -> bool {
    probe.pos.0 >= target.0 .0
        && probe.pos.0 <= target.1 .0
        && probe.pos.1 <= target.0 .1
        && probe.pos.1 >= target.1 .1
}

fn step(probe: &Probe) -> Probe {
    let new_pos = (probe.pos.0 + probe.vel.0, probe.pos.1 + probe.vel.1);
    let new_vel = (probe.vel.0.saturating_sub(1), probe.vel.1 - 1);

    Probe {
        pos: new_pos,
        vel: new_vel,
    }
}
