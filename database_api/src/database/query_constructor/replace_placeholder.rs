//Replace by reference a placeholder with a value in serain string
//Returns: Boolean representing the success of the function
pub fn replace_placeholder(string: &mut String, placeholder: &str, new_value: &str) -> bool {
    let postion = string.find(placeholder);

    match postion {
        Some(idx) => {
            string.insert_str(idx, new_value);
            return true;
        }
        None => return false,
    }
}
