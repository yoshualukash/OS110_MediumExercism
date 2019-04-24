// Pythagorean-triplet
// https://exercism.io/my/solutions/bba562e856614eda9c0b4c2e2c8f15dd 
use std::collections::HashSet;

pub fn find(sum: u32) -> HashSet<[u32; 3]> {
    let mut triplets = HashSet::new();
    for c in 1..sum{
        for b in 1..c{
            if b + c > sum{
                break
            }
            let a = sum - b - c;
            //syarat pythagoras kalau belum sesuai, terus looping
            if (a*a + b*b == c*c) && (a + b + c == sum) && (a < b){
                triplets.insert([a,b,c]);
            }
        }
    }
    triplets
    
}
