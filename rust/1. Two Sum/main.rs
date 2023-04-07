fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
    let mut x;  
    let mut result = vec![0, 0];
    let mut prev: HashMap<i32, i32> = HashMap::new();

    for i in 0..nums.len() {
        x = nums[i]; 
        let y = target - x;
        let tmp = prev.get(&y);

        match tmp {
            Some(x) => {
                result[0] = i as i32;
                result[1] = *x;
                break; 
            }
            None => {}
        }
        prev.insert(x, i as i32); 
    }
    result
}