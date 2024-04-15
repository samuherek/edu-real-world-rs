/// Group similar titles together that would account for typos.
/// The solution is to first transform the frequency of the characters
/// of a word and then group all the words that contain the same character
/// frequency and put them into one vector. That is the result of the search
/// accounting for typos.

/// Complexity is
/// Time: O(n x k) -> n is num of list, k is len of the string
/// Space: O(n x k) -> n is num of list, k is len of the string

const WORDS: &[&str] = &["duel", "dule", "deul", "speed", "spede", "cars"];

pub fn exec() {
    let mut map = std::collections::HashMap::new();
    let mut count = vec![0; 26];

    for word in WORDS {
        for c in word.chars() {
            let index = (c as u32 - 'a' as u32) as usize;
            count[index] += 1;
        }
        let key = count.iter().map(|v| format!("#{}", v)).collect::<String>();

        map.entry(key).or_insert(Vec::new()).push(word);
        count = vec![0; 26];
    }

    let query = "spede";
    if let Some((_, values)) = map.iter().find(|(_, val)| val.contains(&&query)) {
        println!("{:?}", values);
    }
}

