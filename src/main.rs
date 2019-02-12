use std::io;
use std::process;
fn main() {
	println!("Welcome to my Pythagorean Theorem Calculator! Only positive numbers will be shown because it isn't possible to have negetive numbers as an input or output shown with the Pythagorean Theorem.");
	loop {
		let mut choice = String::new();
		println!("\nNow select the action you'd like to do by entering the number that's next to the action.");
		println!("1. a² + b² = c²");
		println!("2. c² - a² = b²");
		println!("3. c² - b² = a²");
		println!("4. Exit program");
		io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
			eprintln!("Application Error: {}", err);
			process::exit(1);
		});
		let choice = choice.trim() as &str;
		match choice {
			"1" | "1." => {
				println!("Enter the value of a.");
				let sum = convert();
				println!("Enter the value of b.");
				let choice = convert();
				println!("\nThe solution is:");
				println!("{}² + {}² = c²", sum.abs(), choice.abs());
				println!("{} + {} = {}", (sum * sum).abs(), (choice * choice).abs(), ((sum * sum) + (choice * choice)).abs());
				println!("The square root of {} is {}", ((sum * sum) + (choice * choice)).abs(), ((sum * sum) + (choice * choice)).abs().sqrt());
			}
			"2" | "2." => {
				println!("Enter the value of c.");
				let sum = convert();
				println!("Enter the value of a.");
				let choice = convert();
				println!("\nThe solution is:");
				println!("{}² - {}² = b²", sum.abs(), choice.abs());
				println!("{} - {} = {}", (sum * sum).abs(), (choice * choice).abs(), ((sum * sum) - (choice * choice)).abs());
				println!("The square root of {} is {}", ((sum * sum) - (choice * choice)).abs(), ((sum * sum) - (choice * choice)).abs().sqrt());
			}
			"3" | "3." => {
				println!("Enter the value of c.");
				let sum = convert();
				println!("Enter the value of b.");
				let choice = convert();
				println!("\nThe solution is:");
				println!("{}² - {}² = a²", sum.abs(), choice.abs());
				println!("{} - {} = {}", (sum * sum).abs(), (choice * choice).abs(), ((sum * sum) - (choice * choice)).abs());
				println!("The square root of {} is {}", ((sum * sum) - (choice * choice)).abs(), ((sum * sum) - (choice * choice)).abs().sqrt());
			}
			"4" | "4." => {
				println!("Goodbye!");
				process::exit(0);
			}
			&_ => {
				eprintln!("You entered an invalid value.");
				process::exit(1);
			}
		}
	}
}
fn convert()->f64 {
	let mut choice = String::new();
	io::stdin().read_line(&mut choice).unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	let choice:f64 = choice.trim().parse().unwrap_or_else(|err| {
		eprintln!("Application Error: {}", err);
		process::exit(1);
	});
	return choice;
}