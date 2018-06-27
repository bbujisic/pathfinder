extern crate serde;
extern crate serde_json;
extern crate base64;

use std::env;
use serde_json::{Value};
use std::collections::HashMap;

/**
 * Reads the given routes enviroments variable name, decodes and
 * Prints the HTTP/S URL for the given upstream name
 * 
 * First argument: is the enviromental variable name
 * Second argument: is the upstream name
 */
fn main() {
	let command_line_args: Vec<String> = env::args().collect();
	
	if command_line_args.len() < 3 {
		panic!("All command line arguments are required.");
	}
	
	let decoded: HashMap<String, Value> = json_decode(&load_json_data(&command_line_args[1]));     
        
    for (key, value) in decoded {		
		if value["type"] == "upstream" && value["upstream"] == command_line_args[2]{			
			println!("{}", key);
		}
	}    
}

fn load_json_data(routes_variable_name: &str) -> String {
	// Load the Enviroment Variable - This will be a base64 encoded string
	let routes_base64_string: String = match env::var(routes_variable_name) {
		Ok(val) => val,
		Err(err) => panic!("Problem looking up the enviroment variable \"{}\": {}", routes_variable_name, err),
	};
	
	// Decode the base64 string - This will be serialized JSON data
	let json_string : String= match base64::decode(&routes_base64_string) {
		Ok(val) => match String::from_utf8(val) {
			Ok(val) => val,
			Err(_err) => panic!("Found invalid UTF-8: {}", _err)
		},
		Err(_err) => panic!("Could not decode base64 data: {}", _err)
	};	
	
	return json_string;
}

fn json_decode(data: &str) -> HashMap<String, Value> {
	// Deserialize the JSON data into a JSON object
	let deserialized: HashMap<String, Value> = match serde_json::from_str(&data) {
		Ok(value) => value,
		Err(error) => {panic!("There was a problem parsing JSON {:?}", error)}
	};
	
	return deserialized;
}
