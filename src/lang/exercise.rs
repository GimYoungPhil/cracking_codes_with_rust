use std::thread;
use std::time::Duration;

fn simulated_expensive_calculation(intensity: u32) -> u32 {
    println!("it takes a long time...");
    thread::sleep(Duration::from_secs(5));
    intensity
}

pub fn run_main() {
    let simulated_user_specified_value = 10;
    let simulated_random_number = 7;

    generate_workout_2(
        simulated_user_specified_value,
        simulated_random_number,
    );
}

fn generate_workout_2(intensity: u32, random_number: u32) {
    let expensive_closure = | intensity | {
        println!("it takes a long time...");
        thread::sleep(Duration::from_secs(5));
        intensity
    };

    if intensity < 25 {
        println!("Push-Up: {}", expensive_closure(intensity));
        println!("Sit-Up: {}", expensive_closure(intensity));
    } else {
        if random_number == 3 {
            println!("Breaktime!");
        } else {
            println!("Running: {}", expensive_closure(intensity));
        }
    }
}

fn generate_workout_1(intensity: u32, random_number: u32) {
    let expensive_reuslt = simulated_expensive_calculation(intensity);

    if intensity < 25 {
        println!("Push-Up: {}", expensive_reuslt);
        println!("Sit-Up: {}", expensive_reuslt);
    } else {
        if random_number == 3 {
            println!("Breaktime!");
        } else {
            println!("Running: {}", expensive_reuslt);
        }
    }
}

fn generate_workout(intensity: u32, random_number: u32) {
    if intensity < 25 {
        println!("Push-Up: {}", simulated_expensive_calculation(intensity));
        println!("Sit-Up: {}", simulated_expensive_calculation(intensity));
    } else {
        if random_number == 3 {
            println!("Breaktime!");
        } else {
            println!("Running: {}", simulated_expensive_calculation(intensity));
        }
    }
}

