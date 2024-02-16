
fn concatenate_strings(str1: &str, str2: &str) -> String {
    let mut result: String = String::new();
    result.push_str(&str1);
    result.push_str(&format!(" {}", &str2)[..]);
    result
}


fn main() {
    let string1: String = String::from("string1");
    let string2: String = String::from("string2");
    let concatenated_string = concatenate_strings(&string1, &string2);
    println!("Concatenated String: {}", concatenated_string);
}


