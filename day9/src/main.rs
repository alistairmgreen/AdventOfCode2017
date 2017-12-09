fn main() {
    let puzzle_input = include_str!("puzzle_input.txt");
    let (without_garbage, characters_removed) = remove_garbage(puzzle_input);
    println!("Score for puzzle input is {}", score(&without_garbage));
    println!("{} garbage characters were removed.", characters_removed);
}

fn remove_garbage(input: &str) -> (String, usize) {
    let mut output = String::new();
    let mut characters_removed: usize = 0;
    let mut in_garbage = false;
    let mut ignore_next = false;

    for c in input.chars() {
        if ignore_next {
            ignore_next = false;
            continue;
        }

        match c {
            '!' => ignore_next = true,
            '<' if !in_garbage => in_garbage = true,
            '>' => in_garbage = false,
            _ => if in_garbage {
                characters_removed += 1
            } else {
                output.push(c)
            },
        }
    }

    (output, characters_removed)
}

fn score(input: &str) -> usize {
    let mut bracket_depth: usize = 0;
    let mut score: usize = 0;

    for c in input.chars() {
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
        assert_eq!(remove_garbage("{<a>,<a>,<a>,<a>}"), ("{,,,}".to_string(), 4usize));
    }

    #[test]
    fn remove_garbage_ignores_character_after_exclamation_mark() {
        let(garbage_removed, _) = remove_garbage("{{<!>},{<!>},{<!>},{<a>}}");
        assert_eq!(garbage_removed, "{{}}");
    }

    #[test]
    fn garbage_character_count0() {
        let (_, removed) = remove_garbage("<>");
        assert_eq!(removed, 0);
    }

    #[test]
    fn garbage_character_count17() {
        let (_, removed) = remove_garbage("<random characters>");
        assert_eq!(removed, 17);
    }

    #[test]
    fn garbage_character_count_includes_angle_brackets_except_first_one() {
        let (_, removed) = remove_garbage("<<<<>");
        assert_eq!(removed, 3);
    }

    #[test]
    fn garbage_character_count_excludes_ignored_characters() {
        let (_, removed) = remove_garbage("<{!>}>");
        assert_eq!(removed, 2);
    }

    #[test]
    fn garbage_character_count_excludes_ignored_characters_2() {
        let (_, removed) = remove_garbage("<!!>");
        assert_eq!(removed, 0);
    }

    #[test]
    fn garbage_character_count_excludes_ignored_characters_3() {
        let (_, removed) = remove_garbage("<{o\"i!a,<{i<a>");
        assert_eq!(removed, 10);
    }

    #[test]
    fn garbage_character_count_last_example() {
        let (_, removed) = remove_garbage("<!!!>>");
        assert_eq!(removed, 0);
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
}
