fn longest_palindrome(s: String) -> String {
    let s = s.as_bytes();
    let n = s.len();

    let mut max_len = 0;
    let mut start = 0;
    let mut end = 0;

    for i in 0..n {
        for j in i..n {
            let mut l = i;
            let mut r = j;
            while l < r && s[l] == s[r] {
                l += 1;
                r -= 1;
            }
            if l >= r && j - i + 1 > max_len {
                max_len = j - i + 1;
                start = i;
                end = j;
            }
        }
    }
    String::from_utf8_lossy(&s[start..=end]).to_string()
}