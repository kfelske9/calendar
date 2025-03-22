/**
 * @ author kfelske@gmx.net
 * */
 
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

mod easter;
pub mod i18n;


/**
get an iterator to the lines in the file
@filename 	File with lines of words
@return 	an iterator to lines 
*/
pub fn read_lines <P : AsRef<Path>> (filename : P) -> io::Result<
														io::Lines<
															io::BufReader<
																File>>>
		
	where P: AsRef<Path>, {	
		let file = File::open(filename)?;
		Ok(io::BufReader::new(file).lines()) //missing ";" marking the return
}

/**
get an iterator to the lines in the file
@filename 	File with lines of numbers
@return 	an vector with int -no "0"
*/
pub fn get_numbers <P : AsRef<Path>> (filename : P) -> Vec<i32> {
	
	let mut births: Vec<i32> 	= vec![];
	
	if let Ok(lines) = read_lines(filename) {
		for line in lines {
			if let Ok(ip) = line {
				let letters		= ip.chars().collect();
				let mut date 	= parse(letters);
				date %= 10000;
				if date != 0 {
					births.push(date);
				}			
			}
		}
	}
	births
}
   
/**
parse char to int
@letters 	vector with char
@return 	an integer over "0", -1 when not a number  
*/
pub fn parse(letters: Vec<char>) -> i32 {
   
    let mut re	 = 0;
    let mut neg	 = 1;
	
    for n in letters.iter() {	
		match n {
			'1' => {re *= 10; 
					re += 1},
					
			'2' => {re *= 10; 
					re += 2},
					
			'3' => {re *= 10; 
					re += 3},
					
			'4' => {re *= 10; 
					re += 4},
					
			'5' => {re *= 10; 
					re += 5},
					
			'6' => {re *= 10; 
					re += 6},
					
			'7' => {re *= 10; 
					re += 7},
					
			'8' => {re *= 10; 
					re += 8},
					
			'9' => {re *= 10; 
					re += 9},
					
			'0' => {re *= 10},
			
			'-' => { if neg == 1 && re == 0 {
							neg = -1; 
						} 	else 			{
							return re * neg;
						}
					},
					
			'_' => {},
			
			' ' => {},
							
			 _  => return re * neg,
		}	
	}

    return re * neg;
}

/**
check if year is a leap year
@year 		year to check
@return 	bool   
*/
pub fn is_leap_year(year: i32) -> bool {
	
	if year % 4 == 0 {
		if year % 100 == 0 {
			if year % 400 == 0 {
				if year % 3200 == 0 {
					return false;
				}
				return true;
			}
			return false;
		}
		return true;
	}	
	return false;
}

/**
get the first day in the  year
@year 		year to check
@return 	day of the week,  monday = 0
*/
pub fn get_day_code (year: i32) -> i32 {
	let lyear = year -1;
	let x1 = lyear / 4;
	let x2 = lyear / 100;
	let x3 = lyear / 400;

	let re = (lyear + x1 - x2 + x3 ) % 7;
	
	
	return re;
} 

/**
print	the year
@year	year to print
*/
//macro_rules! RED		{ () => { print!("\x1b[31m")	};}
macro_rules! GRN		{ () => { print!("\x1b[32m")	};}
//macro_rules! ORA		{ () => { print!("\x1b[33;40m")	};}
macro_rules! BLU		{ () => { print!("\x1b[34m")	};}
macro_rules! MAG		{ () => { print!("\x1b[35m")	};}
//macro_rules! CYN		{ () => { print!("\x1b[36m")	};}
//macro_rules! WHT		{ () => { print!("\x1b[37;40m")	};}//no success
//macro_rules! XXX		{ () => { print!("\x1b[40;37m")	};}//no success

//macro_rules! MOVE		{ () => { print!("\x1b[5;20H")		};
macro_rules! RESET		{ () => { print!("\x1b[0m")		};}
//macro_rules! CLEARSCREEN	{ () => { print!("\x1b[2j")		};}//no success
macro_rules! SETGRN	{ ($value:expr) => { GRN!();								
								$value; 
								RESET!()};}
macro_rules! SETBLU	{ ($value:expr) => { BLU!();								
								$value; 
								RESET!()};}
macro_rules! SETMAG	{ ($value:expr) => { MAG!();								
								$value; 
								RESET!()};}


pub fn print_calendar (year: i32, easters : Vec<i32>, births : Vec<i32>) {
		
	let days_in_month		= get_months(year);
	let	mut day_code		= get_day_code (year);
	
	let mut e_month 		= 0;
	let mut e_day			= 0;
	let mut es 				= 0;
	
	let mut b_month			= 0;
	let mut b_day			= 0;
	let mut bi				= 0;
	let mut temp			;
	
	print!("\n\n\t\t\t");
	SETGRN!(print!("{:?}", year));

	for month in 1..13 {
		let name = i18n::get_month_names()[month - 1];
		print!("\n\n\n\t{}", name);
		
		print!("{}", i18n::get_short_days());
		SETGRN!(println!("{}", i18n::get_sunday()));
		
		//correct position
		for _day in 1..day_code +1 {
			print!("\t");
		}
        
        //print dates for 1 month
        //for day in 1..days_in_month + 1 {
        for day in 1..days_in_month[month -1] + 1 {
			//rightside
			if day <= 9 {
				print!(" ");
			}
			
			//read birthdays
			if births.len() > 0 && bi < births.len() {
				temp				= births[bi];
				b_month 			= temp / 100;
				b_day				= temp % 100;
			}
			
			//read sundays
			if easters.len() > 0 && es < easters.len() {
				temp				= easters[es];
				e_month 			= temp / 100;
				e_day				= temp % 100;
			}
			
			//colour sunday
			if (day + day_code - 1) % 7 == 6 {
				if b_month == month as i32 && b_day == day {
					SETMAG!(print!("{:?}", day));
					bi += 1;
				} else {
					SETGRN!(print!("{:?}", day));
				}
			//colour other days
			} else {
				if b_month == month as i32 && b_day == day {
					if e_month == month as i32 && e_day == day {
						SETMAG!(print!("{:?}", day));
						bi += 1;
						es += 1;
					} else {
						SETBLU!(print!("{:?}", day));
						bi += 1;
					}
				} else if e_month == month as i32 && e_day == day {
					SETGRN!(print!("{:?}", day));
					es += 1;
				} else {
					print!("{:?}", day);
				}
			}
			
			if (day + day_code) % 7 > 0 {
				print!("\t");
			} else  {
				print!("\n");
			}
		}
		day_code = (day_code + days_in_month[month -1]) % 7;
	}
}

/**
 * create an array with days in the month
 * @param 	year 
 * @return 	array
 **/
pub fn get_months (year : i32) -> Vec<i32> {
	let re	: Vec<i32>;
	
	if is_leap_year (year) {
		re	=	vec![31, 29, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
	} else {
		re 	=	vec![31, 28, 31, 30, 31, 30, 31, 31, 30, 31, 30, 31];
	}
	
	return re
}

/**
 * add days to a date
 * @param 	date YYYYMMDD
 * @param 	numb number of days
 * @return 	new date YYYYMMDD
 **/
pub fn add_days (date : i32, numb : i32) -> i32 {
	
	let year			= date / 10000;
	let month 			= ((date / 100) % 100) as usize;
	let day 			= date  % 100;
	let ndate 			;
	let mut i 			= month -1;
	let mut diff 		= numb + day;
	let mut temp_year	= year;
	
	let mut re	: Vec<i32> = vec![];
	
	let mut months		= get_months (year);
	
	while diff < 0 {

		if !(month > 2) {
			temp_year 	-= 1;
		}
		 
		if is_leap_year (temp_year) {
			diff 		+= 	366;
			months[1] 	= 	29;
		} else {	 
			diff 		+= 	365;
			months[1] 	= 	28;
		}

		if month > 2 {
			temp_year 	-= 1;
		}
	}
	
	loop  {
		if diff - months[i] < 1 {
			break;
		} else {
			diff 	-= months[i];
			i 		+= 1;
			if i > 11 {
				i 			= 0;
				temp_year 	+= 1;
				months 		= get_months (temp_year);
			}
		}
	}
	
	ndate = temp_year * 10000 
			+((i +1) * 100) as i32 
			+ diff;

	re.push(ndate);
	
	return ndate
}

/**
 * call the birthdays
 * @param 	filename
 * @return 	Vector with dates
 **/
pub fn deal_birth <P : AsRef<Path>> (filename : P) -> Vec<i32> {
	
	let mut re : Vec<i32> = get_numbers(filename);
	
	re.sort();
	re	=	eliminate_double(re.clone());
	
	re
}

/**
 * call the celebrations
 * @param 	filename
 * @return 	Vector with dates
 **/
pub fn deal_easter <P : AsRef<Path>> (year : i32, filename : P) -> Vec<i32> {

	let mut re : Vec<i32> = get_numbers(filename);
	
	let weaster = easter::get_easter(year);
	
	let mut date	= 0;
	let mut temp	;
	
	while date < re.len() {
		if re[date] < 0 {
			temp 		= re[date] * -1;
			temp 		= add_days(weaster, temp - 1000);
			re[date] 	= temp % 10000;
		}
		
		date += 1;
	}
	
	re.sort();
	re	=	eliminate_double(re.clone());
	
	re
}

/**
 * @param sorted vector i32
 * @return vector i32 without double values
 * */
pub fn eliminate_double (arr: Vec<i32>) -> Vec<i32> {
	
	let mut re: Vec<i32> 	= vec![];
	let mut i				= 1;
	
	while i < arr.len()  {
		if arr[i - 1] != arr[i] {
			re.push(arr[i - 1]);
		}
		i += 1;
	}
	
	re.push(arr[i -1]);
	
	re
}
