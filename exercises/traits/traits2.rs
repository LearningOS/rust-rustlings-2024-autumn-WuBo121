// traits2.rs
//
// Your task is to implement the trait `AppendBar` for a vector of strings. To
// implement this trait, consider for a moment what it means to 'append "Bar"'
// to a vector of strings.
//
// No boiler plate code this time, you can do this!
//
// Execute `rustlings hint traits2` or use the `hint` watch subcommand for a hint.



trait AppendBar {
    fn append_bar(&mut self);
}

// TODO: Implement trait `AppendBar` for a vector of strings.
impl AppendBar for Vec<String>{
    fn append_bar(mut self) {
        self.push("Bar".to_string());
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_vec_pop_eq_bar() {
        let mut vec = vec![String::from("Foo")];
        vec.append_bar();
        assert_eq!(vec.pop().unwrap(), String::from("Bar"));
        assert_eq!(vec.pop().unwrap(), String::from("Foo"));
    }
}
