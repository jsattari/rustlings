// Calls of this function should be replaced with calls of `string_slice` or `string`.
fn placeholder() {}

fn string_slice(arg: &str) {
    println!("{arg}");
}

fn string(arg: String) {
    println!("{arg}");
}

// TODO: Here are a bunch of values - some are `String`, some are `&str`.
// Your task is to replace `placeholder(…)` with either `string_slice(…)`
// or `string(…)` depending on what you think each value is.
fn main() {
    // returning string
    string_slice("blue");

    // returns string
    string("red".to_string());

    // returns string
    string(String::from("hi"));

    // turns string into owned string
    string("rust is fun!".to_owned());

    // returns string
    string("nice weather".into());

    // returns string
    string(format!("Interpolation {}", "Station"));

    // WARNING: This is byte indexing, not character indexing.
    // Character indexing can be done using `s.chars().nth(INDEX)`.
    string_slice(&String::from("abc")[0..1]);

    // borrowed string because it needs to change the string
    string_slice("  hello there ".trim());

    // returns string
    string("Happy Monday!".replace("Mon", "Tues"));

    // returns string, doesn't take ownership
    string("mY sHiFt KeY iS sTiCkY".to_lowercase());
}
