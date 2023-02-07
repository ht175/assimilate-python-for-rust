/*A library for doing Marco Polo */
/* Accepts a string with a name.
If the name is "Marco", returns "Polo".
If the name is "any other value", it returns "Marco".
*/
pub fn marco_polo(input: &str) -> String {
    if input == "Marco" {
        "Polo".to_string()
    } else {
        "Marco".to_string()
    }
}
