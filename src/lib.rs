use std::{collections::HashMap, fmt, fs::File, io::{self, Read}};

struct TrieNode {
    is_terminal: bool,
    was_terminal: bool,
    children: HashMap<char,TrieNode>,
    // optional because the root has no char.
    key_char: Option<char>
}

pub struct Trie {
    root: TrieNode
}

fn get_trie_node(pchar:Option<char>) -> TrieNode {
    TrieNode {   
        is_terminal:false,
        was_terminal: false,
        children: HashMap::new(),
        key_char: pchar
    }
}

impl Trie {

    pub fn new() -> Trie {
        Trie { root: get_trie_node(None)}
    }

    pub fn insert(&mut self, word:&str) -> () {
        // Standard implementation of insertion in a Trie, with a tiny tweak
        // where I added 'was_terminal' to indicated if a partial word was inserted before. 

        let mut rt = &mut self.root;
        for c in word.chars(){
            if !rt.children.contains_key(&c) {
                // cloning a character should be fine
                let d = c.clone();
                rt.children.insert(d, get_trie_node(Some(d)));
                if rt.is_terminal {
                    rt.is_terminal = false;
                    rt.was_terminal = true;
                }
            }
            // should always return a TrieNode
            rt = rt.children.get_mut(&c).unwrap();

        }
        // if say "ant" was inserted and next we insert "an", then at "n", we set was_terminal = true. 
        if rt.children.is_empty() {
            rt.is_terminal = true;
        } else {
            rt.was_terminal = true;
        }
    }

    pub fn search(&self, word:&str, only_inserted:bool) -> bool {
        // Standard implementation of search in a trie, with a little tweak 
        // where I added the only_inserted flag. If true, only return true
        // when there is an inserted value that matches word. If false,
        // the return value will be true if word forms a partial path in the trie.

        let mut rt = &self.root;
        for c in word.chars() {
            if !rt.children.contains_key(&c) {
                return false;
            }
            // should always return a TrieNode
            rt = rt.children.get(&c).unwrap();
        }

        if only_inserted {rt.is_terminal || rt.was_terminal} else {true} 
    }

    pub fn suggest(&self, partial_word:&str) -> String {
        // Given a partial word, suggest next words according to what was inserted into the trie.

        let mut rt = &self.root;
        for c in partial_word.chars() {
            if !rt.children.contains_key(&c) {
                return "".to_string();
            }
            // should always return a TrieNode
            rt = rt.children.get(&c).unwrap();
        }
        // now continue with rt, find all words in the trie
        let mut partial = partial_word.to_string();
        partial.pop(); // this is because we don't want the last character to duplicate
        // output
        self.add_next_letter(rt, &vec![partial]).join(", ")
    }

    pub fn load_from_txt(&mut self, fpath:&str) -> Result<(), io::Error> {
        let mut f = File::open(fpath)?;
        let mut s = String::new();
        f.read_to_string(&mut s)?;
        for line in s.lines() {
            for w in line.split(",") {
                let d = w.trim();
                if d != "" {
                    self.insert(d);
                }
            }
        }
        Ok(())
    }


    fn get_string(&self) -> String {
        let mut word_list:Vec<String> = Vec::new();
        for tn in self.root.children.values(){
            word_list.extend(self.add_next_letter(tn, &vec!["".to_string()]))
        }

        word_list.join(", ")

    }

    fn add_next_letter(&self, node:&TrieNode, words:&Vec<String>) -> Vec<String>{

        let add_letter:Vec<String> = words.iter().map(|x| {
            x.clone() + &node.key_char.unwrap().to_string()
        }).collect();

        if node.is_terminal {
            add_letter
        } else {
            let mut output:Vec<String> = if node.was_terminal {add_letter.clone()} else {Vec::new()};
            for c in node.children.values() {
                output.extend(self.add_next_letter(c, &add_letter));
            }

            output            
        }
    }

}

impl fmt::Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.get_string())
    }
}

