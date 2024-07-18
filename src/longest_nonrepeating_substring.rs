use std::collections::HashMap;

pub const V_LEN: usize = (5*(10i32.pow(4))) as usize;

// Sliding window approach
pub fn length_of_longest_substring(s: String) -> i32 {
    let v = s.as_bytes();
    let mut longest = 0;
    let mut w_start = 0usize;
    let mut w_end = 1usize;
    let mut cursor = 0usize;
    let mut map = HashMap::new();
    let mut cur_char;

    while w_end <= v.len() {
        cur_char = v[cursor];
        if let Some(count) = map.get(&cur_char) {
            println!("\n\n====NEW ITERATION====\n{:?}", &map);
            println!("Char: {} | w_start: {}, w_end: {} | count of this char: {} ", cur_char, w_start, w_end, count);
            // cur_char is a duplicate in the current window
            if *count >= 1 {
                // Check if this is the longest substring
                println!("Duplicate found on char {}. Length of current window excluding duplicate: {}", cur_char, w_end - w_start - 1);
                longest = longest.max(w_end - w_start - 1);
                // Move the start pointer to the right until the duplicate
                // is found to eliminate duplicate
                while v[w_start] != cur_char {
                    *(map.get_mut(&v[w_start]).unwrap()) -= 1;
                    w_start += 1;
                }
                // v[w_start] is the duplicate. Move 1 index to the right to ensure there is only one character of this kind
                w_start += 1;
            } else {

                // cur_char is a unique letter in the window
                // Increment counter
                *(map.get_mut(&cur_char).unwrap()) += 1;
            }
        } else {
            println!("\n\n====NEW ITERATION====\n{:?}", &map);
            println!("Char: {} | w_start: {}, w_end: {} | count of this char: {}", cur_char, w_start, w_end, 0);
            // New character. Must not be a duplicate
            map.insert(cur_char, 1u16);
        }

        // Move cursor and end pointer
        cursor += 1;
        w_end += 1;
        println!("\n\n");
    }
    
    println!("Unaccounted last window - w_start: {}, w_end: {}, length: {}", w_start, w_end, w_end - 1 - w_start);
    longest.max(w_end - 1 - w_start) as i32
}
