use std::collections::HashMap;
fn main() {
    let address_string = String::from("Afoot and light-hearted I take
    to the open road,
    Healthy, free, the world before me,
    The long brown path before me leading wherever I choose.
    Henceforth I ask not good-fortune, I myself am good-fortune,
    Henceforth I whimper no more, postpone no more, need nothing,
    Done with indoor complaints, libraries, querulous criticisms,
    Strong and content I travel the open road.
    The earth, that is sufficient,
    I do not want the constellations any nearer,
    I know they are very well where they are,
    I know they suffice for those who belong to them.
    Still here I carry my old delicious burdens,
    I carry them, men and women, I carry them with me wherever I go,
    I swear it is impossible for me to get rid of them,
    I am fill’d with them, and I will fill them in return.");
    
    let to_compress = String::from("Thissss stringgggg toooo beee compressedddd");
    let compressed = String::from("This3 string4 to3 be2 compres1ed3");
    let temp = word_count(&address_string);
    //top_k_frequent(address_string, 10);
    let mut count_vec: Vec<_> = temp.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for i in 0..5 {
        //println!("count_vec: {:?}", count_vec[i]);
    }
    let mut temp3 = address_string.split(|c: char| !c.is_alphanumeric())
    .filter(|w| !w.is_empty())
    .map(|w| w.to_lowercase())
    .fold(HashMap::new(), count_words);
    println!("temp3: {:?}", temp3);

    
    let compress_test = compress(&to_compress);
    println!("{}", compress_test);
    let test_parens = String::from("()");
    println!("{}", balanced_parens(test_parens));
}

fn create_part(character: char, count: i32) -> String {
    let count = if count == 1 { 
        "".to_string() 
    } else {
        count.to_string() 
    };
    println!("Char: {}, Count: {}", character, count);
    format!("{}{}", character, count)
}

fn compress(text: &str) -> String {
    // If input is null string, just return that
    if text == "" {
        return text.to_string()
    }

    // Converts input string into vector of characters
    let chars: Vec<char> = text.chars().collect();
    // Create and set some temp variables
    let mut result = String::new();
    let mut prev_char = chars[0];
    let mut count = 0;
    // Loop through vector of chars, if the current char is equal to previous char, 
    // increment count, else: push the result of create_part onto the string result
    for c in 0..(chars.len() - 1) {
        if chars[c] == prev_char {
            count += 1;
        } else {
            result.push_str(create_part(prev_char, count).as_str());
            prev_char = chars[c];
            count = 1;
        }
    }

    // Needs to be called again because chars is not empty
    result.push_str(create_part(prev_char, count).as_str());
    // If text and result are the same length, return the text variable as a string
    if text.len() == result.len() {
        String::from(text)
    } else {
        result
    }
}

fn top_k_frequent(input_string: String, k: usize){
    let chunks: Vec<&str> = input_string.split_whitespace().collect();
    let mut count: HashMap<&str, usize> = HashMap::new();

    for word in chunks {
        *count.entry(word).or_insert(0) += 1;
    }

    let mut count_vec: Vec<_> = count.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for i in 0..k {
        println!("count_vec: {:?}", count_vec[i]);
    }

}

fn word_count(sentence: &str) -> HashMap<String, u32> {
    sentence.split(|c: char| !c.is_alphanumeric())
        .filter(|w| !w.is_empty())
        .map(|w| w.to_lowercase())
        .fold(HashMap::new(), count_words)
}

fn count_words(mut hashMap: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
    {
        let c = hashMap.entry(word).or_insert(0);
        *c += 1;
    }

    hashMap
}

// does no error checking, but takes a string of parens and checks if it's balanced
fn balanced_parens(input: String) -> bool {
    let mut output = false;
    let mut left_count = 0;
    let mut right_count = 0;
    for i in 0..input.len() {
        if input.chars().nth(i).unwrap() == '('{
            left_count += 1;
        } else {
            right_count += 1;
        }
    }
    if left_count == right_count {
        return true;
    } else {
        return false;
    }
    
}

//This is someone else's code: jesskay on exercism and github
//TO DO: rewrite my own way
pub struct Brackets { is_balanced: bool }

impl Brackets {
    pub fn from(s: &str) -> Self {
        let mut open_brackets_stack = Vec::new();

        for c in s.chars() {
            match c {
                // opening a bracket/brace is always fine
                '{' | '(' | '[' => open_brackets_stack.push(c),
                // closing is only fine when it matches the most recent opened
                '}' | ')' | ']' => {
                    if let Some(prev) = open_brackets_stack.pop() {
                        match (prev, c) {
                            // closing correct pair, all fine (already popped)
                            ('{', '}') | ('(', ')') | ('[', ']') => continue,
                            // closing a wrong pair, bail
                            _ => return Self { is_balanced: false }
                        }
                    } else { // closing without any open pair, bail
                        return Self { is_balanced: false };
                    }
                },
                // ignore anything that's not a bracket, for this exercise
                _ => continue
            }
        }

        // only balanced if we didn't bail early *and* nothing left on the stack
        Self { is_balanced: open_brackets_stack.len() == 0 }
    }

    pub fn are_balanced(&self) -> bool {
        self.is_balanced
    }
}