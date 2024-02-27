use std::ops::Deref;
use std::ops::DerefMut;

/* Appropriate lifetimes */
// Since at call time, we don't want any constraint between the lifetimes of those two strings,
// we just infer the smallest lifetime to our return reference.
fn choose_str<'a>(s1: &'a str, s2: &'a str, select_s1: bool) -> &'a str {
    if select_s1 {
        s1
    } else {
        s2
    }
}

// OOR (Owned or Reference) type
enum OOR<'a> {
    Borrowed(&'a str),
    Owned(String),
}

// Deref trait for OOR
impl<'a> Deref for OOR<'a> {
    type Target = str;
    fn deref(&self) -> &str {
        match *self {
            OOR::Borrowed(s) => s,
            OOR::Owned(ref s) => s,
        }
    }
}

// DrefMut trait for OOR
impl<'a> DerefMut for OOR<'a> {
    fn deref_mut(&mut self) -> &mut str {
        match *self {
            OOR::Borrowed(str) => {
                let mut s = String::from("");
                // swaps the content of s with the content of String::from(str). 
                // This effectively transfers ownership of the string from str to s.
                std::mem::swap(&mut s, &mut String::from(str));
                // We can also use
                // let mut s;
                // s = std::mem::take(&mut String::from(str));
                // in both cases we take the ownership of the string from str to s
                *self = OOR::Owned(s);
                self
            }
            OOR::Owned(ref mut s) => s,
        }
    }
}

fn main() {
    // Check Deref for both variants of OOR
    let s1 = OOR::Owned(String::from("  Hello, world.  "));
    assert_eq!(s1.trim(), "Hello, world.");
    let mut s2 = OOR::Borrowed("  Hello, world!  ");
    assert_eq!(s2.trim(), "Hello, world!");

    // Check choose
    let s = choose_str(&s1, &s2, true);
    assert_eq!(s.trim(), "Hello, world.");
    let s = choose_str(&s1, &s2, false);
    assert_eq!(s.trim(), "Hello, world!");

    // Check DerefMut, a borrowed string should become owned
    assert!(matches!(s1, OOR::Owned(_)));
    assert!(matches!(s2, OOR::Borrowed(_)));
    unsafe {
        for c in s2.as_bytes_mut() {
            if *c == b'!' {
                *c = b'?';
            }
        }
    }
    assert!(matches!(s2, OOR::Owned(_)));
    assert_eq!(s2.trim(), "Hello, world?");
    println!("All tests passed!")
}
