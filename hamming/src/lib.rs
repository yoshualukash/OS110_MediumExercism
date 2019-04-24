/// Hamming
/// https://exercism.io/my/solutions/0f5f28bc1f78474cad44121fd970efb3 

/// Return the Hamming distance between the strings,
/// or None if the lengths are mismatched.
pub fn hamming_distance(s1: &str, s2: &str) -> Option<usize> {
    if s1.len() != s2.len() { 
        return None 
    }
    else if s1 == s2 {
        Some(0)
    }
    else{
        let mut s1_char = s1.chars();
        let mut s2_char = s2.chars();
        let mut x = 0;
        for i in 0..s1.len(){
            if s1_char.next() != s2_char.next(){
                x += 1;
            }
        }
        Some(x) 
    }
}
