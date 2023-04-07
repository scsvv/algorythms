fn length_of_longest_substring(s: String) -> i32 {
    let mut map = HashMap::new();
    let mut start = 0; 
    let mut max_len = 0;
    for (end, c) in s.chars().enumerate() {
        if let Some(idx) = map.get(&c) {
            start = start.max(*idx + 1);
        }
        map.insert(c, end);
        max_len = max_len.max(end - start + 1);
    }
    max_len as i32
}