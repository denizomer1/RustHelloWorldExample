// Define a function that takes two string slices and returns a new String
fn concatenate_strings(s1: &str, s2: &str) -> String {
    let mut result = String::new(); // Create a new String to hold the result
    result.push_str(s1);           // Append the first string slice
    result.push_str(s2);           // Append the second string slice
    result                          // Return the concatenated String
}

fn main() {
    // Initialize two String variables
    let string1 = String::from("Hello, ");
    let string2 = String::from("World!");

    // Call the function with string slices
    let concatenated_string = concatenate_strings(&string1, &string2);

    // Print the concatenated string
    println!("{}", concatenated_string);

    // Verify ownership of the original strings
    println!("Original string1: {}", string1);
    println!("Original string2: {}", string2);
}
