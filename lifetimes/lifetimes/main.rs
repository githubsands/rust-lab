fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// error: lifetime 'a required for x
// error: lifetime 'a required for y
// NOTE: why lifetimes required for both variables: the function can only return x or y thus we
// only need one lifetime
fn longest_three_fixed<'a, 'b>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}

// NOTE: type, String, exists on the heap so no ownership is needed
fn string_concatenate<'a>(x: String, y: &'a str) -> String {
    return x + y
}

fn strings_referenced_concatenate<'a>(x: &'a str, y: &'a str) -> String {
    let z = String::from(x);
    return z + y
}

// NOTE: example of two different life times that need to keep on living
fn two_string_life_times<'a, 'b>(x: &'a str, y: &'b str) -> (&'a str, &'b str) {
    return (x, y)
}

// NOTE: Two static strings where no lifetime elided occurs
fn two_string_no_life_times(x: &'static str, y: &'static str) -> (&'static str, &'static str) {
    return (x, y)
}

struct StringHolderOneLifeTime<'a> {
    one: &'a str,
    two: &'a str,
}

struct StringHolderTwoLifeTimes<'a, 'b> {
    one: &'a str,
    two: &'b str,
}

fn two_strings_two_lifetime_structure<'a, 'b>(x: &'a str, y: &'b str) -> StringHolderTwoLifeTimes<'a, 'b>{
    let sh = StringHolderTwoLifeTimes {
        one: x,
        two: y
    };
    return sh
}

fn two_strings_one_lifetime_structure<'a, 'b>(x: &'a str, y: &'a str) -> StringHolderOneLifeTime<'a> {
    let sh = StringHolderOneLifeTime {
        one: x,
        two: y,
    };
    return sh
}

// life_time_mutable_int changes a mutable int
//
// fn main() {
//   let mut x: i32= 12;
//   let x = life_time_mutable_int(&mut x);
// }

fn life_time_mutable_int<'a>(x: &'a mut i32) -> &'a mut i32 {
    *x = *x + 12;
    x
}

fn life_times_string_parameter<'a>(string: String) -> String {
    return string
}

fn main(){}


