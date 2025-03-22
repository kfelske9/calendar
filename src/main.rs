/**
 * @ author kfelske@gmx.net
 * for use as examplecode
 * stolen at: rust examples, mobileC, Gauss, gtk examples
 * */
 
mod library;

use std::env;
use std::io;
use chrono::Local;

//use core::ptr::null;

/*using the cc crate never worked:
the compiler proudly presented where it searched the rgcu file from which i never heard
		extern "C" {
			pub fn get_system_year () -> i32;	
		}
*/
		
fn main() {
	//cargo run 2016
	let args: Vec<String> 		= env::args().collect();
	let length 					= args.len();
	let births	: Vec<i32>		; 
	let easters : Vec<i32>		;
	let mut i			 		= 0;
	let mut argument			= String::new();
	let  letters	: Vec<char>	;
	let mut year	: i32		;
	
	while i < length {
		
		if i == 1 {
			argument = args[i].clone();
		}
		i +=1;
	}
		
	if length == 1 {
		println!("\n{}", library::i18n::get_dialog()[0]);
		io::stdin()
			.read_line(& mut argument)
			.expect("Lesefehler");

		argument.pop();	

	}
	
	letters		= argument.chars().collect();	
	year 		= library::parse(letters);


	//get system time
	if year == 0 {
		
		let temp	= format!("{}",Local::now().format("%Y"));
		year 		= temp.trim().parse().expect("REASON");
		
	}
	
	//read birthdays
	births = library::deal_birth ("./birth.txt");
	
	//read sundays
	easters = library::deal_easter(year, "./easter.txt");
		
	library::print_calendar (year, easters, births);
	
	if library::is_leap_year(year) {
		argument = String::from(library::i18n::get_dialog()[1]);
	} else {
		argument = String::from(library::i18n::get_dialog()[2]);
	}

	println!("\n\n{} {} {} {} {}", 
		library::i18n::get_dialog()[3],
		year,
		library::i18n::get_dialog()[4],
		argument,
		library::i18n::get_dialog()[5]);

}

