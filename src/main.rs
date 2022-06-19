use std::process;
mod lib;

fn main() {
    
    let mut t = lib::Trie::new();
    // let vocab = vec!["hello", "test", "testing", "world", "worldly", "hellokitty", "ant", "an"];
    // for v in vocab {
    //     t.insert(v);
    // }
    if let Err(e) = t.load_from_txt("vocab.txt") {
        println!("Error when loading from text: {}", e);
        process::exit(1);
    }

    println!("hello?: {}", t.search("hello", true));
    println!("test?: {}", t.search("test", true));
    println!("world?: {}", t.search("world", true));
    println!("hellokitty?: {}", t.search("hellokitty", true));
    println!("ant?: {}", t.search("ant", true));
    println!("an?: {}", t.search("an", true));
    println!("abc?: {}", t.search("abc", true));
    println!("hell?: {}", t.search("hell", true));
    println!("The substring hell is 'in the' trie, although it is never inserted: {}", t.search("hell", false));
    println!("All inserted words of the tire are: {}", t);
    println!("If I type in 'he', the trie will suggest the words: {}", t.suggest("he"));
    println!("If I type in 'a', the trie will suggest the words: {}", t.suggest("a"));



}
