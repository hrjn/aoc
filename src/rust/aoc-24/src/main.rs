fn data_types() {
    // -- Use i32 and f64
    let _some_int: i32 = 42;
    let _some_float: f64 = 3.14128;
    // -- Parsing a string as a number
    let _some_str_turned_into_int = "42".parse::<i32>(); // See about Result below
}

fn display_things() {
    // Print(s)
    let my_name = "Alex";

    // -- String slice interpolation
    println!("[PRINT] - Hello {}", my_name);

    // -- Debug macro
    dbg!(my_name);

    // -- Implementing Debug trait for a struct
    #[derive(Debug)]
    struct Name {
        first_name: String,
        last_name: String,
    }
    let mememe = Name {
        first_name: "Alex".to_string(),
        last_name: "Bruh".to_string(),
    };
    println!(
        "[WITHOUT_DEBUG_TRAIT] - Name: {} {}",
        mememe.first_name, mememe.last_name
    );
    println!("[WITH_DEBUG_TRAIT] - My full name is: {:?}", mememe);
}

fn error_handling() {
    // Result
    let value_str = "42";
    // -- Option 1 (recommended): pattern matching
    let _value_as_int = match value_str.parse::<i32>() {
        Ok(val) => val,
        Err(e) => {
            println!("ERROR: {:?}", e);
            return;
        }
    };
    // -- Option 2: expect()
    let _value_expected_int = value_str
        .parse::<i32>()
        .expect("ERROR! Something went wrong here...");
}

fn on_ownership() {
    // First, mutability
    let _line = vec![1, 2, 3];
    let mut another_line = vec![4, 5, 6];
    println!("[BEFORE] - {:?}", another_line);
    // line.push(7); ERROR because line is immutable
    another_line.push(7);
    println!("[AFTER] - {:?}", another_line);
    fn show_value(value: &String) {
        println!("{}", value);
    }
    let mut sw_quote = "Hello there! ".to_string();
    show_value(&sw_quote);
    fn change_value(value: &mut String) {
        value.push_str(" General Kenobi :)");
    }
    change_value(&mut sw_quote);
    println!("{:?}", sw_quote);
}

fn main() {
    data_types();
    display_things();
    error_handling();
    on_ownership();
}
