/***********************************************************************************************
 *                Program Name: rotn
 *
 *                      Author: Browning Keith Smith <browningsmith@email.arizona.edu>
 *                        Date: November 24, 2019
 *                Last Updated: November 25, 2019
 *
 *                  Assignment: Project: Learn a New (to You!) Programming Language
 *                      Part 2: Common Program, ROTN (ROT13)
 *                    Due Date: November 25, 2019
 *                       Class: CSc 372
 *                  Instructor: L. McCann
 *                         TAs: Tito Ferra, Josh Xiong
 *
 *                Dependencies: See Cargo.toml
 *
 *                 Description: This program reads a text file, and any English letters in the File
 *                              are "rotated" around the alphabet by n letters. So if n is specified
 *                              as 13, an A would become an N, a B would become an O, and so on. the
 *                              program takes two command line arguments: n, which is an integer that we
 *                              want to rotate through the alphabet, and <filename> which is the
 *                              name of the file to read.
 *
 *                       Input: u32 n, String "<filename>"
 *                      Output: Program writes a file of the name "<filename>.rotn" in
 *                              the same directory as the input file, where <filename> is replaced with the
 *                              filename specified on input, and n is replaced with the integer specified
 *                              on input.
 *
 * Example Execution (Windows): .\rotn.exe 13 "text.txt"
 *                              This would create an output file of the name
 *                              "text.txt.rot13"
 *
 *      Link to Language Study: https://docs.google.com/document/d/1iInK2BWCybQMu_Oqyt0Vk5t5cwM1oJUSXZOpX7Y-5Jw/edit?usp=sharing_eip&ts=5ddc4adc
 **********************************************************************************************/

 use std::env;
 use std::process;
 use std::fs;
 use std::io::Write;

fn main() {
    
	//Read command line arguments into a vector args
	let args: Vec<String> = env::args().collect();

	//Check number of command line arguments. Must be greater than or equal to 3, since program name will be included
	if args.len() < 3 {
	
		println!("Error: Command line arguments must include an integer to rotate text by, and a filename to read.");
		println!("Example: \".\\rotn.exe\" 7 \"filename.txt\"");
		process::exit(0);
	}

	//Place first command line argument into n, an i32 type. n is the number of letters to rotate the text by.
	let n: i32 = args[1].trim().parse()
		.expect("Error: Second argument must be an integer"); //Print an error and exit if no valid integer was entered.

	//Grab the second command line argument, this is the file we will use.
	let filename = &args[2];

	//Open the input file and read the contents
	let text = fs::read_to_string(filename)
		.expect("Error reading the input file"); //Print an error and exit if the file is not found, or not readable.

	//Create empty String out_text to hold the output text
	let mut out_text = String::from("");

	//For each character in c, push the new character returned by do_the_rot onto the out_string
	for c in text.chars() {
	
		out_text.push(do_the_rot(c, n));
	}

	//Open an output file of the proper name
	let mut out_filename = String::from(""); //Create an empty string
	out_filename.push_str(filename); //Add input file name to the string
	out_filename.push_str(".rot"); //Add ".rot"
	out_filename.push_str(&n.to_string()); //Add a string representation of n

	//Create the file with the output filename
	let mut out_file = fs::File::create(out_filename)
		.expect("Unable to create output file");

	//Write the entirety of out_text to the output file
	out_file.write_all(out_text.as_bytes())
		.expect("Unable to write to the output file");
}

/*********************************************************************
 * Function Name: do_the_rot
 *
 * Input: char c, i32 n
 * Output: char
 * 
 * Description: This function performs the rot algorithm, taking the char
                c, and returning the letter that is plus or minus n letters
				from its location in the alphabet. If it is passed a letter
				that is not part of the English alphabet, or a special character,
				or whitespace, etc. it will simply return the same character it was
				passed.
 **********************************************************************/
fn do_the_rot(c: char, n: i32) -> char {

	let char_tuple = convert_to_int(c); //Call convert_to_int on c, and pass the tuple to char_tuple (needed to do this for proper type inferencing)
	let mut char_n = char_tuple.0; //Pass first value, the integer of the character, to char_n
	let upper = char_tuple.1; //Pass the second value, whether the character was upper or lower case, to upper

	if char_n == 0 { //If char_n is 0, then c is not a real letter we can use.
	
		return c; //If it is not a real letter, return whatever it is
	}

	//Otherwise, perform the rotation
	else {
		char_n = char_n + n; //Add the integer n to char_n
		
		//Continuously subtract 26 as long as char_n is greater than the range of real letters
		while char_n > 26 {
		
			char_n = char_n - 26;
		}

		//Or, continuously add 26 as long as char_n is lower than the range of real letters
		while char_n < 1 {
		
			char_n = char_n + 26;
		}

		convert_to_char(char_n, upper) //Return the new character by calling convert_to_char on char_n, and upper.
	}
}

/*********************************************************************
 * Function Name: convert_to_int
 *
 * Input: char c
 * Output: Tuple: (i32, char)
 * 
 * Description: This function converts a letter of the English
 *              alphabet to its integer representation, A being 1,
 *              B being 2, ... Z being 26. It returns a tuple, the
 *              first part being the integer value, and the second
 *              part being a boolean value, True if the letter was
 *              uppercase, and false if it was lowercase. Function
 *              returns a tuple of (0, false) if the character is
 *              a special character, whitespace, digit, or any
 *              letter outside the English alphabet.
 **********************************************************************/
fn convert_to_int(c: char) -> (i32, bool) {

	return match c {
	
		'a' => (1, false), //If c is 'a', return a 1, and false, since it is lower case
		'b' => (2, false),
		'c' => (3, false),
		'd' => (4, false),
		'e' => (5, false),
		'f' => (6, false),
		'g' => (7, false),
		'h' => (8, false),
		'i' => (9, false),
		'j' => (10, false),
		'k' => (11, false),
		'l' => (12, false),
		'm' => (13, false),
		'n' => (14, false),
		'o' => (15, false),
		'p' => (16, false),
		'q' => (17, false),
		'r' => (18, false),
		's' => (19, false),
		't' => (20, false),
		'u' => (21, false),
		'v' => (22, false),
		'w' => (23, false),
		'x' => (24, false),
		'y' => (25, false),
		'z' => (26, false),
		'A' => (1, true), //If c is 'A', return a 1, and true, since it is upper case
		'B' => (2, true),
		'C' => (3, true),
		'D' => (4, true),
		'E' => (5, true),
		'F' => (6, true),
		'G' => (7, true),
		'H' => (8, true),
		'I' => (9, true),
		'J' => (10, true),
		'K' => (11, true),
		'L' => (12, true),
		'M' => (13, true),
		'N' => (14, true),
		'O' => (15, true),
		'P' => (16, true),
		'Q' => (17, true),
		'R' => (18, true),
		'S' => (19, true),
		'T' => (20, true),
		'U' => (21, true),
		'V' => (22, true),
		'W' => (23, true),
		'X' => (24, true),
		'Y' => (25, true),
		'Z' => (26, true),
		_ => (0, false), //If c is any other character, return 0, and return false (the second value is arbitrary, it will not be used)
	}
}

/*********************************************************************
 * Function Name: convert_to_char
 *
 * Input: i31 char_n, bool upper
 * Output: char
 * 
 * Description: This function takes an Integer and a Boolean value, and
 *              returns a letter of the English alphabet. It recognizes
 *              any integer from 1 to 26, and converts it to the corresponding
 *              letter of the English alphabet. If the Boolean value passed to
 *              the function is True, an uppercase letter is returned. Otherwise,
 *              a lowercase letter is returned. If an Integer outside the range
 *              1-26 is passed, then the function returns a ' ' character.
 **********************************************************************/
fn convert_to_char(char_n: i32, upper: bool) -> char {
	
	if upper { //If upper is true, we need to return an uppercase letter
		
		return match char_n {
			
			1 => 'A', //Return 'A' if char_n is 1, and so on
			2 => 'B',
			3 => 'C',
			4 => 'D',
			5 => 'E',
			6 => 'F',
			7 => 'G',
			8 => 'H',
			9 => 'I',
			10 => 'J',
			11 => 'K',
			12 => 'L',
			13 => 'M',
			14 => 'N',
			15 => 'O',
			16 => 'P',
			17 => 'Q',
			18 => 'R',
			19 => 'S',
			20 => 'T',
			21 => 'U',
			22 => 'V',
			23 => 'W',
			24 => 'X',
			25 => 'Y',
			26 => 'Z',
			_ => ' ', //If char_n is outside the range 1-26, return ' '
		}
	}
	else { //Otherwise, we need to return a lower case character
		
		return match char_n {
			
			1 => 'a', //return 'a' if char_n is 1, and so on
			2 => 'b',
			3 => 'c',
			4 => 'd',
			5 => 'e',
			6 => 'f',
			7 => 'g',
			8 => 'h',
			9 => 'i',
			10 => 'j',
			11 => 'k',
			12 => 'l',
			13 => 'm',
			14 => 'n',
			15 => 'o',
			16 => 'p',
			17 => 'q',
			18 => 'r',
			19 => 's',
			20 => 't',
			21 => 'u',
			22 => 'v',
			23 => 'w',
			24 => 'x',
			25 => 'y',
			26 => 'z',
			_ => ' ', //If char_n is outside the range 1-26, return ' '
		}
	}
}
