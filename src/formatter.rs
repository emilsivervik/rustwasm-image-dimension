pub fn format_names_string(mut name_vec:Vec<String>) -> String {
    let name_vec_length = name_vec.len();
    if name_vec_length > 2 {
        let last_name = name_vec.pop().unwrap();
        let multiple_names = name_vec.join(", ");
        format!("{} och {}", multiple_names, last_name)
    } else if name_vec_length == 2 {
        name_vec.join(" och ")
    } else if name_vec_length == 1 {
        name_vec.pop().unwrap()
    } else {
        "ingen".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn correct_formating() {
        assert_eq!(String::from("ingen"), format_names_string(vec![]));

        let name = String::from("emil");
        let one_names = vec![name.to_owned()];
        assert_eq!(String::from("emil"), format_names_string(one_names));

        let two_names = vec![name.to_owned(), name.to_owned()];
        assert_eq!(String::from("emil och emil"), format_names_string(two_names));

        let three_names = vec![name.to_owned(),name.to_owned(),name.to_owned()];
        assert_eq!(String::from("emil, emil och emil"), format_names_string(three_names));
    }
}