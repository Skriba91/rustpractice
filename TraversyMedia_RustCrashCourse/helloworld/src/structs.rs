// Structs - Used to create custom data types

// Traditional struct
struct Color {
    red: u8,
    green: u8,
    blue: u8,
}

// Tuple struct
struct ColorTuple(u8, u8, u8);

// Struct with functions
struct Person {
    first_name: String,
    last_name: String
}

impl Person {
    // Contruct a person
    fn new(fist: &str, last: &str) -> Person {
        Person {
            first_name: fist.to_string(),
            last_name: last.to_string()
        }
    }

    // Get full name
    fn full_name(&self) -> String {
        format!("{} {}", self.first_name, self.last_name)
    }

    // Set last name
    fn set_last_name(&mut self, last: &str) {
        self.last_name = last.to_string();
    }

    // Name to tuple
    fn to_tuple(self) -> (String, String) {
        (self.first_name, self.last_name)
    }
}

pub fn run() {
    // Create a red color
    let mut c = Color {
        red: 255,
        green: 0,
        blue: 0,
    };

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    c.red = 200;

    println!("Color: {} {} {}", c.red, c.green, c.blue);

    let mut ct = ColorTuple(255, 0, 0);

    ct.1 = 200;

    println!("Color: {} {} {}", ct.0, ct.1, ct.2);

    let mut p = Person::new("John", "Doe");

    println!("Person: {} {}", p.first_name, p.last_name);

    p.first_name = String::from("Jane");

    println!("Person: {}", p.full_name());

    p.set_last_name("Smith");

    println!("Person: {}", p.full_name());

    println!("Person Tuple: {:?}", p.to_tuple());

}