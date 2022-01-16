use super::*;

#[test]
fn regex_first_guess() {
    let state = vec![None, None, None, None, None];
    let off_limit = HashSet::new();
    let must_include = HashMap::new();
    let regex_exp = build_regex_query(&state, &off_limit, &must_include);
    let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

    assert!(re.is_match("alone").unwrap(), "did not match 'alone'");
    assert!(re.is_match("ALONE").unwrap(), "did not match 'ALONE'");
    assert!(re.is_match("aLoNE").unwrap(), "did not match 'aLoNE'");
    assert!(
        !re.is_match("aaaa").unwrap(),
        "matched a string that was too short"
    );
    assert!(
        !re.is_match("aaaaaa").unwrap(),
        "matched a string that was too long"
    );
}

#[test]
fn regex_off_limits() {
    let state = vec![None, None, None, None, None];
    let off_limit = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);
    let must_include = HashMap::new();
    let regex_exp = build_regex_query(&state, &off_limit, &must_include);
    let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

    assert!(re.is_match("zzzzz").unwrap(), "did not match 'zzzzz'");
    assert!(
        !re.is_match("azzzz").unwrap(),
        "incorrectly matched 'azzzz'"
    );
    assert!(
        !re.is_match("zzzza").unwrap(),
        "incorrectly matched 'zzzza'"
    );
    assert!(
        !re.is_match("abcde").unwrap(),
        "incorrectly matched 'abcde'"
    );
    assert!(!re.is_match("z").unwrap(), "incorrectly matched 'z'");
    assert!(
        !re.is_match("zzzzzz").unwrap(),
        "incorrectly matched 'zzzzzz'"
    );
}

#[test]
fn regex_must_include() {
    let state = vec![None, None, None, None, None];
    let off_limit = HashSet::new();
    let must_include = HashMap::from([('r', vec![]), ('y', vec![])]);
    let regex_exp = build_regex_query(&state, &off_limit, &must_include);
    let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

    assert!(re.is_match("rusty").unwrap(), "did not match 'rusty'");
    assert!(re.is_match("weary").unwrap(), "did not match 'weary'");
    assert!(
        !re.is_match("sorts").unwrap(),
        "incorrectly matched 'sorts'"
    );
}

#[test]
fn regex_heavy_restriction() {
    let state = vec![None, None, None, None, None];
    let off_limit = HashSet::from(['a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i']);
    let must_include = HashMap::from([('r', vec![]), ('y', vec![])]);
    let regex_exp = build_regex_query(&state, &off_limit, &must_include);
    let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

    assert!(re.is_match("rusty").unwrap(), "did not match 'rusty'");
    assert!(
        !re.is_match("rusts").unwrap(),
        "incorrectly matched 'rusts'"
    );
    assert!(
        !re.is_match("rasty").unwrap(),
        "incorrectly matched 'rasty'"
    );
}

#[test]
fn regex_known_letters() {
    let state = vec![None, Some('a'), Some('n'), None, Some('y')];
    let off_limit = HashSet::from(['l', 'o', 'e', 'd', 'r', 'i', 'o', 'd', 'l']);
    let must_include = HashMap::from([('a', vec![0, 2]), ('n', vec![3, 4])]);
    let regex_exp = build_regex_query(&state, &off_limit, &must_include);
    let re = Regex::new(&regex_exp).expect("regex expression failed to compile");

    assert!(
        !re.is_match("alone").unwrap(),
        "incorrectly matched 'alone'"
    );
    assert!(
        !re.is_match("drain").unwrap(),
        "incorrectly matched 'drain'"
    );
    assert!(re.is_match("wanky").unwrap(), "did not match 'wanky'");
    assert!(re.is_match("tangy").unwrap(), "did not match 'tangy'");
}
