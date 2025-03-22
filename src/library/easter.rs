	/**
	 * Eastercalculation 
	 * @author Gauss 1816
	 * @param year
	 * @return day MMDD in gregorian calendar
	 */
pub fn get_easter (year : i32) -> i32 {
		if year > 100000 {
			return 1000;}
	
		let a = year % 19;
		let b = year % 4;
		let c = year % 7;
		let k = year / 100;
		
		let p = (8 * k + 13) /25;
		let q = k / 4;
		let m = (15 + k - p -q) % 30;
		let d = (19 * a + m) % 30;
		let n = (4 + k -q) % 7;
		let e = (2 * b + 4 * c + 6 * d + n) % 7;
		let o = 22 + d + e;
		
		let re;
		
		//julian date to gregorian
		if o > 92 {
			re = 600 - 92 + o;
		} else if o > 61 {
			re = 500 - 61 + o;
		} else if o > 31 {
			re = 400 - 31 + o;
		} else {
			re = 300 + o;
		}	
	
		return year * 10000 + re
}
