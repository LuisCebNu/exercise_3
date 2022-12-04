fn main() {
    println!("{}", pig_latin("Apples"));
    println!("{}",pig_latin("First"));

}

fn pig_latin (word: &str) -> String {
    let result: String;
    let vowels = vec!['a','A','e','E','i','I','o','O','u','U'];
    if vowels.iter().any(|&v| word.starts_with(v))  
    {
        result = format!("{}-hay", word.to_string());
    } else {
        result = format!("{}-{}ay",word[1..].to_string(), word[..1].to_string());
    }
    result
}