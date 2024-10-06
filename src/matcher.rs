use std::cmp::max;
use strsim::jaro_winkler;

pub fn find_best_branch<'b>(branches: &'b Vec<String>, needles: &Vec<&str>) -> Option<&'b str> {
    let mut best_score: f64 = 0.0;
    let mut best_scored_branch: Option<&str> = None;

    for branch in branches {
        let tokens: Vec<&str> = branch.split(['/', '-', '_']).collect();

        let branch_score = match_branch_name(&tokens, needles);

        if branch_score > best_score {
            best_score = branch_score;
            best_scored_branch = Some(branch);
        }
    }

    if best_score > 0.6 {
        best_scored_branch
    } else {
        None
    }
}

pub fn match_branch_name(haystack: &Vec<&str>, needles: &Vec<&str>) -> f64 {
    if needles.is_empty() {
        return 1.0;
    }

    if needles.len() > haystack.len() {
        return 0.0;
    }

    let mut best_score = 0.0;
    for token_index in 0..=(haystack.len() - needles.len()) {
        let first_match = match_token(
            needles.first().expect("This needs to be here"),
            haystack[token_index],
        );
        let later_matches = match_branch_name(
            &haystack[token_index..].to_vec(),
            &needles[1..].to_vec(),
        );
        let unified_score = first_match * later_matches;

        if unified_score > best_score {
            best_score = unified_score
        }
    }

    best_score
}

fn match_token(haystack: &str, needle: &str) -> f64 {
    if needle.chars().all(|it| it.is_digit(10)) {
        let prefix_length = haystack.chars().zip(needle.chars()).take_while(|(a, b)| a == b).count();
        if prefix_length < needle.len() {
            return 0.0;
        }

        return prefix_length as f64 / max(haystack.len(), needle.len()) as f64;
    }

    jaro_winkler(&needle.to_lowercase(), &haystack.to_lowercase())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_find_best_match() {
        let test_cases = vec![
            (
                vec!["master", "feature/new-feature"],
                vec!["feature/new-feature"],
                Some("feature/new-feature"),
            ),
            (
                vec!["master", "feature/new-feature"],
                vec!["feat"],
                Some("feature/new-feature"),
            ),
            (
                vec!["master", "feature/new-feature"],
                vec!["feat", "feat"],
                Some("feature/new-feature"),
            ),
            (
                vec!["master", "feature/new-feature/sub-feature"],
                vec!["sub"],
                Some("feature/new-feature/sub-feature"),
            ),
            (
                vec![
                    "master",
                    "feature/new-feature-one",
                    "feature/new-feature-two",
                ],
                vec!["two"],
                Some("feature/new-feature-two"),
            ),
            (vec!["master", "feature/new-feature"], vec!["unknown"], None),
            (
                vec![
                    "master",
                    "feature/GZ-12-test",
                    "feature/GZ-13-test"
                ],
                vec!["13"],
                Some("feature/GZ-13-test"),
            ),
        ];

        for (branches, params, expected) in test_cases {
            let branches = branches.iter().map(|it| it.to_string()).collect();
            let best_match = find_best_branch(&branches, &params);
            assert_eq!(best_match, expected);
        }
    }
}
