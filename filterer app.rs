// Define the FilterCondition struct with a field for the desired value
struct FilterCondition<T> {
    value: T,
}

// Implement the is_match method on FilterCondition
impl<T: PartialEq> FilterCondition<T> {
    fn is_match(&self, item: &T) -> bool {
        &self.value == item
    }
}

// Define the custom_filter function
fn custom_filter<T: PartialEq>(collection: &[T], condition: &FilterCondition<T>) -> Vec<T>
where
    T: Clone,
{
    let mut result = Vec::new();
    for item in collection {
        if condition.is_match(item) {
            result.push(item.clone());
        }
    }
    result
}

fn main() {
    // Create a collection (vector) with some elements
    let numbers = vec![1, 2, 3, 4, 5, 6];

    // Initialize a FilterCondition object
    let condition = FilterCondition { value: 3 };

    // Call the custom_filter function
    let filtered_numbers = custom_filter(&numbers, &condition);

    // Print the filtered result
    println!("Filtered numbers: {:?}", filtered_numbers);
}
