trait Summary {
    fn summarize(&self) -> String;
}

// A basic user struct which will implement the above trait
struct User {
    name: String,
    age: i16,
    profession: String,
}


// Implementing the summary trait
impl Summary for User {
    fn summarize(&self) -> String {
        return format!("Hey my name is {}. I am {} years old and I am a {}", self.name, self.age, self.profession);
    }
}



pub fn run_trait() {

    let user = User {
        name: String::from("Ronak"),
        age: 21,
        profession: String::from("Software Developer")
    };

    println!("{}", user.summarize());

}