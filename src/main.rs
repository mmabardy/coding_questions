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
    let temp = word_count(&address_string);
    //top_k_frequent(address_string, 10);
    let mut count_vec: Vec<_> = temp.iter().collect();
    count_vec.sort_by(|a, b| b.1.cmp(a.1));
    for i in 0..5 {
        println!("count_vec: {:?}", count_vec[i]);
    }
    let mut temp3 = address_string.split(|c: char| !c.is_alphanumeric())
    .filter(|w| !w.is_empty())
    .map(|w| w.to_lowercase())
    .fold(HashMap::new(), countWords);
    println!("temp3: {:?}", temp3);
    
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
        .fold(HashMap::new(), countWords)
}

fn countWords(mut hashMap: HashMap<String, u32>, word: String) -> HashMap<String, u32> {
    {
        let c = hashMap.entry(word).or_insert(0);
        *c += 1;
    }

    hashMap
}
