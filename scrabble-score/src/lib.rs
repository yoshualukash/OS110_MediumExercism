/// Scrabble Score : 
/// https://exercism.io/my/solutions/c12e50652b12483d88dc8e76baf4ac8a 
/// Compute the Scrabble score for a word.
pub fn score(word: &str) -> u64 {
    let word_upper = word.to_ascii_uppercase();
    let word_char = word_upper.chars();
    let mut points = 0;
    for i in word_char {
        let point = match i{
            'A' | 'E' | 'I' | 'O' | 'U' | 'L' | 'N' | 'R' | 'S' | 'T' => 1,
            'D' | 'G'  => 2,
            'B' | 'C' | 'M' | 'P' => 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => 4,
            'K' => 5,
            'J' | 'X' => 8,
            'Q' | 'Z' => 10,
            _ => 0,
        };
        points = points + point
    }
    points
}
