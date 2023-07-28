use input_conv;
//   ^Specify dependency specifics in cargo.toml file
fn main() {
    println!("Enter a string to count the number of words (Counted by groups of characters separated by whitespace)");
    let input_str =  input_conv::read_string(); //For future reference, be sure to add the dependency with `cargo add` and to use the crate specifier
    //                  ^ Specify where this function came from
    let mut count = 0;
    let mut in_group = false;
    for c in input_str.chars(){
        if c.is_whitespace() {
            in_group = false;
        } else if !in_group {
            count += 1;
            in_group = true;
        }
    }
    println!("Number of character groups: {}", count);
}
