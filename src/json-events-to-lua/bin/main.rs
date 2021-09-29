use std::fs;
use std::fs::File;
use std::io::Write;

// The struct event that we will be using to store our events
struct Event {
    time : f32,
    event_type : String,
    arg1 : String,
    arg2 : String
}

fn strip_newline(s : &mut String) {
	if s.ends_with('\n') {
		s.pop();
	}
	// We also need to account for Windows Line Endings, which do CRLF.
	if s.ends_with('\r') {
		s.pop();
	}
}

// Uses pattern matching to find the Lua function name from the JSON name
fn get_lua_name(json_name : &str) -> &str {
    match json_name {
        "timeline_wait" => "t_wait",
        "timeline_clear" => "t_clear()",
        "time_stop" => "u_haltTime",
        "message_add" => "e_messageAdd",
        "message_important_add" => "e_messageAddImportant",
        "menu" => "e_kill()",
        
        "side_changing_start" => "l_enableRndSideChanges(true)",
        "side_changing_stop" => "l_enableRndSideChanges(false)",
        "increment_start" => "l_setIncEnabled(true)",
        "increment_stop" => "l_setIncEnabled(false)",

        "style_set" => "s_setStyle",

        "music_set" => "a_setMusic",
        "music_set_segment" => "a_setMusicSegment",
        "music_set_seconds" => "a_setMusicSeconds",
        "play_sound" => "a_playPackSound",

        // These strings are retrieved from the arguments inside events like level_int_set or style_int_set
        // Level Attributes
        "speed_multiplier" => "l_setSpeedMult",
        "speed_increment" => "l_setSpeedInc",
        "rotation_speed" => "l_setRotationSpeed",
        "rotation_increment" => "l_setRotationInc",
        "delay_multiplier" => "l_setDelayMult",
        "delay_increment" => "l_setDelayInc",
        "fast_spin" => "l_setFastSpin",
        "sides" => "l_setSides",
        "sides_min" => "l_setSidesMin",
        "sides_max" => "l_setSidesMax",
        "increment_time" => "l_setIncTime",
        "rotation_speed_max" => "l_setRotationSpeedMax",

        "wall_skew_left" => "l_setWallSkewLeft",
        "wall_skew_right" => "l_setWallSkewRight",
        "wall_angle_left" => "l_setWallAngleLeft",
        "wall_angle_right" => "l_setWallAngleRight",
        "pulse_min" => "l_setPulseMin",
        "pulse_max" => "l_setPulseMax",
        "pulse_speed" => "l_setPulseSpeed",
        "pulse_speed_r" => "l_setPulseSpeedR",
        "pulse_delay_max" => "l_setPulseDelayMax",
        "beatpulse_max" => "l_setBeatpulseMax",
        "beatpulse_delay_max" => "l_setBeatpulseMax",

        // Style attributes
        "hue_min" => "s_setHueMin",
        "hue_max" => "s_setHueMax",
        "hue_ping_pong" => "s_setHuePingPong",
        "hue_increment" => "s_setHueInc",
        "3D_depth" => "s_set3dDepth",
        "3D_skew" => "s_set3dSkew",
        "3D_spacing" => "s_set3dSpacing",
        "3D_darken_multiplier" => "s_set3dDarkenMult",
        "3D_alpha_multiplier" => "s_set3dAlphaMult",
        "3D_pulse_min" => "s_setPulseMin",
        "3D_pulse_max" => "s_setPulseMax",
        "3D_pulse_speed" => "s_setPulseSpeed",

        _ => "INVALID"
    }
}

// Given an event type, retrieve the name of the first argument associated with that type
fn get_first_arg_name(event_type : &str) -> &str {
    match event_type {
        "event_time_stop" | "timeline_wait" | "time_stop" => "duration",
        "message_add" | "message_important_add" => "message",
        "style_set" | "music_set" | "music_set_segment" |
        "music_set_seconds" | "play_sound" => "id",
        _ => "None"
    }
}

// Given an event type, retrieve the name of the second argument associated with that type
fn get_second_arg_name(event_type : &str) -> &str {
    match event_type {
        "message_add" | "message_important_add" => "duration",
        "music_set_segment" => "segment_index",
        "music_set_seconds" => "segment_seconds",
        _ => "None"
    }
}

// Takes the events JSON array and parses it into a legible Vector
fn parse_timeline(json : &str) -> Vec<Event> {
    // Get the number of JSON events and instantiate our vector
    let length : usize = gjson::get(json, "events.#").i32() as usize;
    let mut timeline = Vec::new();
    for i in 0..length {
        // Calculates the string needed to grab the ith JSON object
        let event_str = &("events.".to_string() + &i.to_string());
        // Grabs the ith JSON object in "events"
        let event_val = gjson::get(json, event_str);
        let event_obj = event_val.str(); // event_val will deallocate when we don't want it to. So we have to separate the statements
        // Grabs the time attribute in the event
        let event_time = gjson::get(event_obj, "time").f32();
        // Grab the type attribute in the event
        let event_type_val = gjson::get(event_obj, "type");
        let mut event_name = event_type_val.str();
        // Pattern match the arguments of the event type
        let mut first_arg_name = get_first_arg_name(event_name);
        let second_arg_name = get_second_arg_name(event_name);
        // Search for "value_name" in case this is a setting field event.
        let value_name_val = gjson::get(event_obj, "value_name");
        // If the type is setting a level/style field, change the event name to the field
        if value_name_val.exists() {
            event_name = value_name_val.str();
            first_arg_name = "value";
        }
        timeline.push(Event{time: event_time, event_type: event_name.to_string(), arg1: first_arg_name.to_string(), arg2: second_arg_name.to_string()});
    }

    return timeline;
    
}

// When you parse a string value in JSON, it doesn't include the quotes surrounding it. This function fixes that issue by adding the quotes if we know it's a string value
fn fix_strings(arg_name : &str, arg_value : &str) -> String {
    match arg_name {
        "id" | "message" => "\"".to_string() + arg_value + "\"",
        _ => arg_value.to_string()
    }
}

// Given the event type and arguments, along with the events location in the JSON, create the equivalent Lua function
fn create_lua_function(json : &str, index : usize, event_type : &str, arg1 : &str, arg2 : &str) -> String {
    // Calculates the string needed to grab the ith JSON object
    let event_str = &("events.".to_string() + &index.to_string());
    // Grabs the ith JSON object in "events"
    let event_val = gjson::get(json, event_str);
    let event_obj = event_val.str();
    // Try and fetch values from the arguments
    let arg1_val = gjson::get(event_obj, arg1);
    let arg2_val = gjson::get(event_obj, arg2);
    let mut arg1_value = arg1.to_string(); // This needs to be a Rust string as this string may need further modification
    let mut arg2_value = arg2;
    if arg1_val.exists() {
        arg1_value = arg1_val.str().to_string();
    }
    if arg2_val.exists() {
        arg2_value = arg2_val.str();
    }
    // Fix any string values by adding quotes to them
    arg1_value = fix_strings(arg1, &arg1_value);
    let arg_tuple : (&str, &str) = (&arg1_value, arg2_value);
    match arg_tuple {
        ("None", "None") => get_lua_name(event_type).to_string(), // Events with no arguments get the full lua function returned
        (x, "None") => get_lua_name(event_type).to_string() + "(" + x + ");",
        (x, y) => get_lua_name(event_type).to_string() + "(" + x + ", " + y + ");"
    }
}


fn main() {
    let mut file_name = String::new();
    let input = std::io::stdin();

    println!("Please provide the .json file name:");

    let _bytes = input.read_line(&mut file_name).unwrap();
    strip_newline(&mut file_name);

    // Give us the JSON contents from the JSON file
    let json_string = fs::read_to_string(file_name).unwrap();

    let events : Vec<Event> = parse_timeline(&json_string);
    let mut i = 0;
    let mut curr_time = 0f32;
    // Begin creating the string of our Lua code
    let mut final_string = "function convertedEvents()\n".to_string();
    for event in events.iter() {
        let time : f32 = event.time;
        // Check if there is a time change
        if time > curr_time {
            curr_time = time;
            // Append a special lua function to make the game "wait" for the next event
            final_string += "\t";
            final_string += "e_waitUntilS(";
            final_string += &time.to_string();
            final_string += ");\n";
        }
        // Append the next lua event into the string
        final_string += "\t";
        final_string += &create_lua_function(&json_string, i, &event.event_type, &event.arg1, &event.arg2);
        final_string += "\n";
        i += 1;
    }
    final_string += "end";
    
    // Now we need to write this string to a file
    let mut file = File::create("convertedEvents.lua").expect("File creation failed");
    file.write_all(final_string.as_bytes()).expect("File writing failed");
    println!("File written to \"convertedEvents.lua\"");
}