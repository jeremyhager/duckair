fn main() {
    let person_name_slice = "Donlard Mallard";
    let person_name_string = person_name_slice.to_string();
    // other options
    let person_name_string1 = "Donlard Mallard".to_string();
    let person_name_string2 = String::from("Donald Mallard");
    // convert string to string slice
    let person_name_slice_from_string = &person_name_string;
    let person_name_slice_from_string_pointer = person_name_slice_from_string.as_str();

}
