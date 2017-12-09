fn main() {
    let puzzle_input = include_str!("puzzle_input.txt");
    println!("Score for puzzle input is {}", score(puzzle_input));
}

fn remove_garbage(input: &str) -> String {
    let mut output = String::new();

    let mut in_garbage = false;
    let mut ignore_next = false;

    for c in input.chars() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        match c {
            '!' => ignore_next = true,
            '<' => in_garbage = true,
            '>' => in_garbage = false,
            _  => if !in_garbage { output.push(c) }
        }
    }

    output
}

fn score(input: &str) -> usize {
    let mut bracket_depth: usize = 0;
    let mut score: usize = 0;

    for c in remove_garbage(input).chars() {
        if c == '{' {
            bracket_depth += 1;
            score += bracket_depth;
        }

        if c == '}' {
            bracket_depth -= 1;
        }
    }

    score
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn remove_garbage_removes_text_in_angle_brackets() {
        assert_eq!(remove_garbage("{<a>,<a>,<a>,<a>}"), "{,,,}");
    }

    #[test]
    fn remove_garbage_ignores_character_after_exclamation_mark() {
        assert_eq!(remove_garbage("{{<!>},{<!>},{<!>},{<a>}}"), "{{}}");
    }

    #[test]
    fn empty_curly_brackets_score_1() {
        assert_eq!(score("{}"), 1);
    }

    #[test]
    fn three_nested_curly_brackets_score_6() {
        assert_eq!(score("{{{}}}"), 6);
    }

    #[test]
    fn further_score_examples() {
        assert_eq!(score("{{},{}}"), 5);
        assert_eq!(score("{{{},{},{{}}}}"), 16);
    }

    #[test]
    fn scores_with_garbage() {
        assert_eq!(score("{<a>,<a>,<a>,<a>}"), 1);
        assert_eq!(score("{{<ab>},{<ab>},{<ab>},{<ab>}}"), 9);
    }

    #[test]
    fn score_with_garbage_and_ignores() {
        assert_eq!(score("{{<!!>},{<!!>},{<!!>},{<!!>}}"), 9);
        assert_eq!(score("{{<a!>},{<a!>},{<a!>},{<ab>}}"), 3);        
    }
}