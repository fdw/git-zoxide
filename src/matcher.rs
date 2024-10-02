use strsim::jaro_winkler;

pub fn find_best_branch<'b>(branches: &'b Vec<String>, params: &Vec<&str>) -> Option<&'b str> {
    let mut best_score: f64 = 0.0;
    let mut best_scored_branch: Option<&str> = None;

    for branch in branches {
        let tokens: Vec<&str> = branch.split(['/', '-', '_']).collect();

        let branch_score = does_match(&tokens, params, &branch);

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

pub fn does_match(tokens: &Vec<&str>, params: &Vec<&str>, branch_name: &str) -> f64 {
    if params.is_empty() {
        return 1.0;
    }

    if params.len() > tokens.len() {
        return 0.0;
    }

    let mut best_score = 0.0;
    for token_index in 0..=(tokens.len() - params.len()) {
        let first_match = match_token(
            tokens[token_index],
            params.first().expect("This needs to be here"),
        );
        let later_matches = does_match(
            &tokens[token_index..].to_vec(),
            &params[1..].to_vec(),
            branch_name,
        );
        let unified_score = first_match * later_matches;

        if unified_score > best_score {
            best_score = unified_score
        }
    }

    best_score
}

fn match_token(a: &str, b: &str) -> f64 {
    if a.chars().all(|it| it.is_digit(10)) {
        return if b.starts_with(a) { 1.0 } else { 0.0 };
    }

    jaro_winkler(&a.to_lowercase(), &b.to_lowercase())
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
