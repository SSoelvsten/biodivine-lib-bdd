use crate::{Bdd, BddNode, BddPartialValuation, BddPointer, BddVariable};
use fxhash::FxBuildHasher;
use std::collections::HashMap;

impl Bdd {
    /// **(internal)** A specialized algorithm for constructing BDDs from DNFs. It builds the BDD
    /// directly by recursively "splitting" the clauses. The advantage is that we avoid a lot of
    /// memory copying. The disadvantage is that when the number of variables is high and the
    /// number of clauses low, this could be slightly slower due to all the recursion. However,
    /// it definitely needs to be tested at some point.
    pub(crate) fn mk_dnf(num_vars: u16, dnf: &[BddPartialValuation]) -> Bdd {
        if dnf.is_empty() {
            return Bdd::mk_false(num_vars);
        }

        // TODO:
        //  Can we turn the algorithm into a normal loop to prevent stack overflow in
        //  extreme cases?
        fn build_recursive(
            num_vars: u16,
            mut variable: u16,
            dnf: &[&BddPartialValuation],
            result: &mut Bdd,
            node_cache: &mut HashMap<BddNode, BddPointer, FxBuildHasher>,
        ) -> BddPointer {
            // The loop will automatically skip variables that are not relevant for the validity
            // of the provided DNF. This should significantly decrease the risk of stack overflow,
            // since we only run recursion when it is reasonably likely that we actually need to
            // condition on the specific variable. Otherwise the variable is just skipped, since
            // we would get `low == high` anyway.
            loop {
                if variable == num_vars {
                    return BddPointer::from_bool(!dnf.is_empty());
                }
                if dnf.is_empty() {
                    return BddPointer::zero();
                }

                let var = BddVariable(variable);
                let should_branch = dnf.iter().any(|val| val.get_value(var).is_some());
                if !should_branch {
                    variable += 1;
                    continue;
                }

                let mut var_true = Vec::new();
                let mut var_false = Vec::new();
                for clause in dnf {
                    match clause.get_value(var) {
                        Some(true) => var_true.push(*clause),
                        Some(false) => var_false.push(*clause),
                        _ => {
                            var_true.push(*clause);
                            var_false.push(*clause);
                        }
                    }
                }

                let high = build_recursive(num_vars, variable + 1, &var_true, result, node_cache);
                let low = build_recursive(num_vars, variable + 1, &var_false, result, node_cache);

                if high == low {
                    return high;
                }

                let node = BddNode::mk_node(var, low, high);
                return if let Some(id) = node_cache.get(&node) {
                    *id
                } else {
                    result.push_node(node);
                    node_cache.insert(node, result.root_pointer());
                    result.root_pointer()
                };
            }
        }

        let mut result = Bdd::mk_true(num_vars);
        let mut node_cache = HashMap::with_capacity_and_hasher(dnf.len(), FxBuildHasher::default());
        node_cache.insert(BddNode::mk_zero(num_vars), BddPointer::zero());
        node_cache.insert(BddNode::mk_one(num_vars), BddPointer::one());

        let dnf = Vec::from_iter(dnf.iter());
        build_recursive(num_vars, 0, &dnf, &mut result, &mut node_cache);

        result
    }

    /// Construct a DNF representation of this BDD. This is equivalent to collecting all results
    /// of the `Bdd::sat_clauses` iterator. However, it uses a different algorithm that should be
    /// slightly faster for enumerating all results at the same time (the `sat_clauses` iterator
    /// is optimized for visiting the results one-by-one).
    pub fn to_dnf(&self) -> Vec<BddPartialValuation> {
        fn build_recursive(
            bdd: &Bdd,
            path: &mut BddPartialValuation,
            node: BddPointer,
            results: &mut Vec<BddPartialValuation>,
        ) {
            if node.is_terminal() {
                if node.is_one() {
                    results.push(path.clone());
                }
                return;
            }

            let var = bdd.var_of(node);
            let low = bdd.low_link_of(node);
            let high = bdd.high_link_of(node);

            // The zero checks are just to avoid the extra recursion step if it is clear that
            // the result is safe to discard. The algorithm is still correct without them.

            if !low.is_zero() {
                path.set_value(var, false);
                build_recursive(bdd, path, low, results);
                path.unset_value(var);
            }

            if !high.is_zero() {
                path.set_value(var, true);
                build_recursive(bdd, path, high, results);
                path.unset_value(var);
            }
        }

        let mut result = Vec::new();
        build_recursive(
            self,
            &mut BddPartialValuation::empty(),
            self.root_pointer(),
            &mut result,
        );
        result
    }
}
