fn ret_string() -> String {
    String::from("  A String object  ")
}

fn main() {
    let binding = ret_string();
    let s = binding.trim(); // returns a new string with the whitespace trimmed
    // The problem with let s = ret_string().trim(); is that the ret_sting() returns a String object, 
    // and the trim() method allocates a new string object, so the original string object is not modified, the value 
    // is fropped while borrowed.
    // temporary value is freed at the end of this statement
    //creates a temporary value which is freed while still in use so there is no variable that ownes the string that is still in use
    // let s = ret_string().trim();
    assert_eq!(s, "A String object");
    println!(
        "The original string is not modified, the value is borrowed: {} ",
        binding
    )
}

/* Appropriate lifetimes */
// Since at call time, we don't want any constraint between the lifetimes of those two strings,
// we just infer the smallest lifetime to our return reference.
fn choose_str<'a>(s1: &'a str, s2: &'a str, select_s1: bool) -> &'a str {
    if select_s1 { s1 } else { s2 }
}