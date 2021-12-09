#![cfg(test)]

use crate::dead_code::Settings;

fn run(content: &str) -> String {
    let ast = rnix::parse(content);
    assert_eq!(0, ast.errors().len());

    let results = Settings {
        no_lambda_arg: false,
        no_underscore: false,
    }
    .find_dead_code(&ast.node());
    crate::edit::edit_dead_code(content, &ast.node(), results.into_iter())
}

macro_rules! no_edits {
    ($s: expr) => {
        let s = $s.to_string();
        assert_eq!(run(&s), s);
    };
}

#[test]
fn let_in_alive() {
    no_edits!("let alive = 23; in alive");
}

#[test]
fn let_in_alive_deep() {
    no_edits!("let alive = 23; in if true then 42 else { ... }: alive");
}

#[test]
fn let_in_alive_dead() {
    let results = run("let alive = 42; dead = 23; in alive");
    assert_eq!(results, "let alive = 42; in alive");
}

#[test]
fn let_in_dead_only() {
    let results = run("let dead = 42; in alive");
    assert_eq!(results, "alive");
}

#[test]
fn let_inherit_in_alive() {
    no_edits!("let inherit (x) alive; in alive");
}

#[test]
fn let_inherit_in_alive_dead() {
    let results = run("let inherit alive dead; in alive");
    assert_eq!(results, "let inherit alive; in alive");
}

#[test]
fn let_inherit_dead_let_alive_in_dead() {
    let results = run("let inherit dead; alive = true; in alive");
    assert_eq!(results, "let alive = true; in alive");
}

#[test]
fn let_inherit_in_dead_only() {
    let results = run("let inherit dead; in alive");
    assert_eq!(results, "alive");
}

#[test]
fn let_inherit_from_in_alive() {
    no_edits!("let inherit (x) alive; in alive");
}

#[test]
fn let_inherit_from_in_alive_dead() {
    let results = run("let inherit (x) alive dead; in alive");
    assert_eq!(results, "let inherit (x) alive; in alive");
}

#[test]
fn let_inherit_from_dead_let_alive_in_dead() {
    let results = run("let inherit (x) dead; alive = true; in alive");
    assert_eq!(results, "let alive = true; in alive");
}

#[test]
fn let_inherit_from_in_dead_only() {
    let results = run("let inherit (x) dead; in alive");
    assert_eq!(results, "alive");
}

#[test]
fn lambda_arg_alive() {
    no_edits!("alive: alive");
}

#[test]
fn lambda_arg_anon() {
    let results = run("_anon: false");
    assert_eq!(results, "_anon: false");
}

#[test]
fn lambda_at_pattern_dead() {
    let results = run("dead@{ dead2 ? dead, ... }: false");
    assert_eq!(results, "{ ... }: false");
}

#[test]
fn lambda_lead_at_dead() {
    let results = run("dead@{ ... }: false");
    assert_eq!(results, "{ ... }: false");
}

#[test]
fn lambda_trail_at_dead() {
    let results = run("{ ... }@dead: false");
    assert_eq!(results, "{ ... }: false");
}

#[test]
fn lambda_at_shadowed() {
    let results = run("dead@{ ... }: dead@{ ... }: dead");
    assert_eq!(results, "{ ... }: dead@{ ... }: dead");
}

#[test]
fn lambda_pattern_dead() {
    let results = run("alive@{ dead, ... }: alive");
    assert_eq!(results, "alive@{ ... }: alive");
}

#[test]
fn lambda_pattern_mixed() {
    let results = run("dead1@{ dead2, alive, ... }: alive");
    assert_eq!(results, "{ alive, ... }: alive");
}
