// Isogram
// https://exercism.io/my/solutions/cc08eb28caef4dbaafa73959bda0c42f 
use std::collections::HashSet;
pub fn check(candidate: &str) -> bool {
    let mut list_huruf = HashSet::new();
    if candidate.len() == 0{
        return true
    }
    else{
        let candidate_low = candidate.to_lowercase();
        let candidate_new = candidate_low.chars();
        for huruf in candidate_new{
            if huruf.is_alphabetic(){
                if list_huruf.contains(&huruf) {
                    return false
                }
                else{
                    list_huruf.insert(huruf);
                }
            }
        }
        return true
    }
}
