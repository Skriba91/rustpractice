// Tuples group together values of different types.
// Max 12 elements

pub fn run() {
    let person: (&str, &str, i8) = ("John", "engineer", 31);
    println!("{} is a {} year old {}", person.0, person.2, person.1);
}