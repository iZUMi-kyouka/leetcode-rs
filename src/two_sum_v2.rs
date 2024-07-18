pub fn two_sum(numbers: Vec<i32>, target: i32) -> Vec<i32> {
    let mut start = 0usize;
    let mut end = numbers.len() - 1;
    let mut cur_num;
    loop {
        cur_num = unsafe { numbers.get_unchecked(start) + numbers.get_unchecked(end)};
        if cur_num == target {
            return vec![(start+1) as i32, (end+1) as i32];
        } else if cur_num > target {
            end -= 1;
        } else {
            start += 1;
        }
    }
}