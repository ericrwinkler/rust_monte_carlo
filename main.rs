use rand::Rng;
use std::env;
use std::thread;
use std::time::Instant;

fn main() {
    let timer = Instant::now();
    let mut number_of_points: i64 = 0;
    let mut number_of_threads: i64 = 0;
    let mut total_points_in_circle: i64 = 0;
    let mut threads = vec![];

    validate_input(&mut number_of_points, &mut number_of_threads);

    for _ in 0..number_of_threads {
        let thread_handle = thread::spawn(move || {
            monte_carlo_points_in_circle(number_of_points / number_of_threads)
        });

        threads.push(thread_handle);
    }

    for thread_handle in threads {
        total_points_in_circle += thread_handle.join().unwrap();
    }

    let pi: f64 = 4.0 * (total_points_in_circle as f64) / (number_of_points as f64);
    println!("Estimated value of Pi: {}", pi);
    println!("Elapsed time: {:?} ms", timer.elapsed().as_millis());
}

fn monte_carlo_points_in_circle(number_of_points: i64) -> i64 {
    let mut points_in_circle: i64 = 0;
    let mut rng = rand::thread_rng();

    for _ in 0..number_of_points {
        let random_x: f32 = rng.gen();
        let random_y: f32 = rng.gen();
        let d = (random_x.powi(2)) + (random_y.powi(2));

        if d <= 1.0 {
            // Point is inside the unit circle
            points_in_circle += 1;
        }
    }
    points_in_circle
}

fn validate_input(number_of_points: &mut i64, number_of_threads: &mut i64) {
    let args: Vec<String> = env::args().collect();
    if (args.len() < 2) || (args.len() > 3) {
        eprintln!("Usage: {} <number_of_points> <number_of_threads>", args[0]);
        std::process::exit(1);
    }

    *number_of_points = args[1]
        .parse::<i64>()
        .expect("Please provide a valid int64 as the first argument");
    *number_of_threads = args[2]
        .parse::<i64>()
        .expect("Please provide a valid int64 as the second argument");

    if *number_of_points < 1 || *number_of_threads < 1 {
        eprintln!(
            "Error: number_of_points and number_of_threads must be between 1 and {}",
            i64::MAX
        );
        std::process::exit(1);
    }

    if *number_of_threads > *number_of_points {
        eprintln!(
            "Error: number_of_threads must be less than number of points and should evenly divide {}",
            i64::MAX
        );
        std::process::exit(1);
    }

    if (*number_of_points % *number_of_threads) != 0 {
        println!("Warning: number_of_points not divisible evenly by number_of_threads");
    }
}
