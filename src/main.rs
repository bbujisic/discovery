extern crate json;
extern crate base64;
use std::env;

fn main() {		
    /*
     * Get Path and variable name from Command Line
     * 	0: Full path to this executable
     * 	1: Envirometal var name
     * 	2: JSON path to extract value from
    */
    let command_line_args: Vec<String> = env::args().collect();
	let relashionships_variable_name = &command_line_args[1]; // Example environmental name: "PLATFORM_RELATIONSHIPS";
    let json_path = &command_line_args[2]; // Example Path: "database.mysql.username";
	
	// Grab data from env variable
	let mut relashionships_base64_string = "".to_string();
	match env::var(relashionships_variable_name) {
		Ok(val) => relashionships_base64_string = val,
		Err(e) => println!("couldn't interpret {}: {}", relashionships_variable_name, e),
	}
	
	// Get and convert JSON from base64 string stored in env variable.
    let relashionships_json_bytes = base64::decode(&relashionships_base64_string).unwrap();
    let relashionships_json_string = String::from_utf8(relashionships_json_bytes).expect("Found invalid UTF-8");
    let relashionships_json = json::parse(&relashionships_json_string).unwrap();            
    
    // Split the JSON string path into a vector
    let mut json_path_vector: Vec<&str> = json_path.split(".").collect();
    
    // The first to elements in the path will always be <service_type>.<service_name>
    // The remaining items are to be looked up
    let mut json_service_search_path_vector = json_path_vector.split_off(2);
    let serice_type = json_path_vector[0];
    let service_name = json_path_vector[1];
    let mut result: json::JsonValue = json::JsonValue::Null;
    
    // Pick out the server we wish to grab the data from
	for service_detail in relashionships_json[serice_type].members() {
		if service_detail["service"] == service_name {
			// I don't know any better way to handle this :-(
			result = get_element(service_detail.clone(), &mut json_service_search_path_vector);	
			break;	
		}
	}
	
	if !result.is_null(){
		print!("{}", result);    
	}
}

/// Returns the requested data (json_service_search_path_vector) from the JSON object
/// data => A JSON datastructure
/// json_service_search_path_vector => a vector containing the requested var
fn get_element(data: json::JsonValue, json_service_search_path_vector: &mut Vec<&str>) -> json::JsonValue{	
	// Take the first element
	let mut index = json_service_search_path_vector.split_off(1);
	
	if index.is_empty() {
		// This is the data we are looking for
		return data[json_service_search_path_vector[0]].clone();
	}
	else{
		// Go to the next level
		return get_element(data[json_service_search_path_vector[0]].clone(), &mut index);
	}
}
