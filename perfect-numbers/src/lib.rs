/// Perfect-Numbers
/// https://exercism.io/my/solutions/7d8d9cfecdc2439ba07b91301215ca20 

#[derive(Debug, PartialEq, Eq)]
pub enum Classification {
    Abundant,
    Perfect,
    Deficient,
}
use std::collections::HashSet;
pub fn classify(num: u64) -> Option<Classification> {
    let mut list_num = HashSet::new();
    let mut i = 1;
    while i < num{
        if num % i == 0{
            list_num.insert(i);
            i += 1;
        }
        else{
            i += 1;
        }
    }
    let mut total = 0;
    for iter in list_num.iter(){
        total = total + iter;
    }
    if num == 0{
        return None
    }
    if total == num{
        Some(Classification::Perfect)
    }
    else if total < num{
        Some(Classification::Deficient)
    }
    else if total > num{
        Some(Classification::Abundant)
    }
    else{
        return None
    }
}
