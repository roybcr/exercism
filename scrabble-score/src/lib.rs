pub fn score(word: &str) -> u64 {
    word.chars().fold(0, |mut acc, mut curr| {
        if !curr.is_ascii_alphabetic() {
            return acc;
        }
        curr.make_ascii_uppercase();
        match curr {
            'K' => acc += 5,
            'D' | 'G' => acc += 2,
            'J' | 'X' => acc += 8,
            'Q' | 'Z' => acc += 10,
            'B' | 'C' | 'M' | 'P' => acc += 3,
            'F' | 'H' | 'V' | 'W' | 'Y' => acc += 4,
            _ => acc += 1,
        }
        acc
    })
}
