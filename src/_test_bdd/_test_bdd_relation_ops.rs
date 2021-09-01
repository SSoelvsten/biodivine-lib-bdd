use crate::_test_util::{mk_5_variable_set, mk_small_test_bdd};
use crate::{Bdd, BddVariable};
use rand::rngs::StdRng;
use rand::SeedableRng;

fn vars() -> (
    BddVariable,
    BddVariable,
    BddVariable,
    BddVariable,
    BddVariable,
) {
    return (
        BddVariable(0),
        BddVariable(1),
        BddVariable(2),
        BddVariable(3),
        BddVariable(4),
    );
}

#[test]
fn bdd_var_projection() {
    let variables = mk_5_variable_set();
    let bdd = variables.eval_expression_string("(v1 => (v2 <=> v3)) & (!v1 => !(v2 <=> v5))");
    let v1 = BddVariable(0);
    assert_eq!(
        bdd.var_project(v1),
        variables.eval_expression_string("(v2 <=> v3) | !(v2 <=> v5)")
    );
}

#[test]
fn bdd_var_pick() {
    let variables = mk_5_variable_set();
    let bdd = variables.eval_expression_string("(v1 => (v2 <=> v3)) & (!v1 => !(v2 <=> v5))");
    let v1 = BddVariable(0);
    assert_eq!(
        bdd.var_pick(v1),
        variables
            .eval_expression_string("(v1 => ((v2 <=> v3) & (v3 <=> v5))) & (!v1 => !(v2 <=> v5))")
    );
}

#[test]
fn bdd_var_pick_random() {
    let variables = mk_5_variable_set();
    let bdd = variables.eval_expression_string("(v1 => (v2 <=> v3)) & (!v1 => !(v2 <=> v5))");
    let v1 = BddVariable(0);
    let mut random = StdRng::seed_from_u64(1234567890);
    for _ in 0..10 {
        let picked = bdd.var_pick_random(v1, &mut random);
        assert_eq!(picked.and(&bdd), picked);
        let v1_true_paths = picked.var_select(v1, true).var_project(v1);
        let v1_false_paths = picked.var_select(v1, false).var_project(v1);
        assert!(v1_true_paths.and(&v1_false_paths).is_false());
    }
}

#[test]
fn bdd_var_select() {
    let variables = mk_5_variable_set();
    let bdd = variables.eval_expression_string("(v1 => (v2 <=> v3)) & (!v1 => !(v2 <=> v5))");
    let v1 = BddVariable(0);
    assert_eq!(
        variables.eval_expression_string("v1 & (v2 <=> v3)"),
        bdd.var_select(v1, true)
    );
    assert_eq!(
        variables.eval_expression_string("!v1 & !(v2 <=> v5)"),
        bdd.var_select(v1, false)
    );
}

#[test]
fn bdd_projection_trivial() {
    let variables = mk_5_variable_set();
    let bdd = mk_small_test_bdd(); // v_3 && !v_4
    let tt = variables.mk_true();
    let ff = variables.mk_false();

    let vars = (0..5).map(BddVariable).collect::<Vec<_>>();
    for k in 0..5 {
        assert_eq!(ff, ff.project(&vars[0..k]));
        assert_eq!(tt, tt.project(&vars[0..k]));
    }

    assert_eq!(bdd, bdd.project(&Vec::new()));
    assert_eq!(tt, bdd.project(&vars));
}

#[test]
fn bdd_projection_simple() {
    let variables = mk_5_variable_set();
    let (_, v2, v3, v4, v5) = vars();
    {
        let bdd = variables.eval_expression_string("(v1 <=> v2) & (v4 <=> v5)");
        let projected = variables.eval_expression_string("(v1 <=> v2)");
        assert_eq!(projected, bdd.project(&vec![v4, v5]));
        assert_eq!(bdd.project(&vec![v3, v4, v5]), bdd.project(&vec![v4, v5]));
    }
    {
        let bdd = variables.eval_expression_string("(v4 => (v1 & v2)) & (!v4 => (!v1 & v3))");
        let projected_3 = variables.eval_expression_string("(v1 & v2) | (!v1 & v3)");
        let projected_2 = variables.eval_expression_string("(v1 & v2) | !v1");
        assert_eq!(bdd, bdd.project(&vec![v5]));
        assert_eq!(projected_3, bdd.project(&vec![v4]));
        assert_eq!(projected_2, bdd.project(&vec![v3, v4]));
        assert_eq!(variables.mk_true(), bdd.project(&vec![v2, v3, v4]));
    }
}

#[test]
fn bdd_pick_trivial() {
    let variables = mk_5_variable_set();
    let bdd = mk_small_test_bdd(); // v_3 && !v_4
    let tt = variables.mk_true();
    let ff = variables.mk_false();
    let (v1, v2, v3, v4, v5) = vars();

    assert_eq!(ff, ff.pick(&vec![v2, v3, v4]));
    assert_eq!(ff, ff.pick(&vec![]));

    assert_eq!(tt, tt.pick(&vec![]));
    let expected = variables.eval_expression_string("!v1 & !v2 & !v3 & !v4 & !v5");
    assert_eq!(expected, tt.pick(&vec![v1, v2, v3, v4, v5]));
    let expected = variables.eval_expression_string("!v4 & !v5");
    assert_eq!(expected, tt.pick(&vec![v4, v5]));

    assert_eq!(bdd, bdd.pick(&vec![]));
    let expected = variables.eval_expression_string("!v1 & !v2 & v3 & !v4 & !v5");
    assert_eq!(expected, bdd.pick(&vec![v1, v2, v3, v4, v5]));
    let expected = variables.eval_expression_string("v3 & !v4 & !v5");
    assert_eq!(expected, bdd.pick(&vec![v4, v5]));
}

#[test]
fn bdd_pick_simple() {
    let variables = mk_5_variable_set();
    let bdd = variables.eval_expression_string("(v1 => (v4 <=> v5)) & (!v1 => !(v4 <=> v5))");
    let expected = variables.eval_expression_string("(v1 => (!v4 & !v5)) & (!v1 => (!v4 & v5))");
    let (v1, v2, v3, v4, v5) = vars();
    assert_eq!(expected, bdd.pick(&vec![v4, v5]));
    assert_eq!(bdd, bdd.pick(&vec![v5]));

    let bdd = variables.eval_expression_string("(v1 <=> v5) & (v2 => v4) & (v3 ^ v2)");
    let witness_bdd: Bdd = variables.eval_expression_string("!v1 & !v2 & v3 & !v4 & !v5");
    assert_eq!(witness_bdd, bdd.pick(&vec![v1, v2, v3, v4, v5]));
    let expected = variables
        .eval_expression_string("(v1 => (!v2 & v3 & !v4 & v5)) & (!v1 => (!v2 & v3 & !v4 & !v5))");
    assert_eq!(expected, bdd.pick(&vec![v2, v3, v4, v5]));
    let expected = variables.eval_expression_string("((v1 & v2) => (!v3 & v4 & v5)) & ((v1 & !v2) => (v3 & !v4 & v5)) & ((!v1 & v2) => (!v3 & v4 & !v5)) & ((!v1 & !v2) => (v3 & !v4 & !v5))");
    assert_eq!(expected, bdd.pick(&vec![v3, v4, v5]));
    assert_eq!(expected, bdd.pick(&vec![v4, v5])); // accidentally, this works out
    assert_eq!(bdd, bdd.pick(&vec![v5]));
    assert_eq!(bdd, bdd.pick(&vec![]));
}

#[test]
fn bdd_pick_random() {
    let variables = mk_5_variable_set();
    let bdd = variables.eval_expression_string("(v1 => (v4 <=> v5)) & (!v1 => !(v4 <=> v5))");
    let (_, v2, v3, _, _) = vars();

    let mut random = StdRng::seed_from_u64(1234567890);

    for _ in 0..100 {
        let picked = bdd.pick_random(&[v2, v3], &mut random);
        assert_eq!(picked.and(&bdd), picked);

        let picked_00 = picked
            .select(&[(v2, false), (v3, false)])
            .project(&[v2, v3]);
        let picked_01 = picked.select(&[(v2, false), (v3, true)]).project(&[v2, v3]);
        let picked_10 = picked.select(&[(v2, true), (v3, false)]).project(&[v2, v3]);
        let picked_11 = picked.select(&[(v2, true), (v3, true)]).project(&[v2, v3]);

        assert!(picked_00.and(&picked_01).is_false());
        assert!(picked_00.and(&picked_10).is_false());
        assert!(picked_00.and(&picked_11).is_false());
        assert!(picked_01.and(&picked_10).is_false());
        assert!(picked_01.and(&picked_11).is_false());
        assert!(picked_10.and(&picked_11).is_false());
    }
}

#[test]
fn bdd_select() {
    let variables = mk_5_variable_set();
    let bdd = variables.eval_expression_string("(v1 => (v4 <=> v5)) & (!v1 => !(v4 <=> v5))");
    let expected = variables.eval_expression_string("v1 & !v3 & !v4 & !v5");
    let (v1, _, v3, v4, _) = vars();
    assert_eq!(
        expected,
        bdd.select(&vec![(v1, true), (v4, false), (v3, false)])
    );
}
