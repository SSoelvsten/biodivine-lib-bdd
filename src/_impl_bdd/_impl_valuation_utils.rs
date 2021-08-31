use crate::{Bdd, BddPartialValuation, BddValuation};

/// Utilities for extracting interesting valuations and paths from a `Bdd`.
impl Bdd {
    /// Return the lexicographically first satisfying valuation of this `Bdd`.
    ///
    /// (In this context, lexicographically means `0 < 1`, with greatest variable id
    /// being the most significant).
    pub fn first_valuation(&self) -> Option<BddValuation> {
        if self.is_false() {
            return None;
        }

        let mut valuation = BddValuation::all_false(self.num_vars());
        let mut node = self.root_pointer();
        while !node.is_terminal() {
            if self.low_link_of(node).is_zero() {
                valuation.set(self.var_of(node));
                node = self.high_link_of(node);
            } else {
                node = self.low_link_of(node);
            }
        }

        Some(valuation)
    }

    /// Return the lexicographically last satisfying valuation of this `Bdd`.
    ///
    /// (In this context, lexicographically means `0 < 1`, with greatest variable id
    /// being the most significant).
    pub fn last_valuation(&self) -> Option<BddValuation> {
        if self.is_false() {
            return None;
        }

        let mut valuation = BddValuation::all_true(self.num_vars());
        let mut node = self.root_pointer();
        while !node.is_terminal() {
            if self.high_link_of(node).is_zero() {
                valuation.clear(self.var_of(node));
                node = self.low_link_of(node);
            } else {
                node = self.high_link_of(node);
            }
        }

        Some(valuation)
    }

    /// Return the lexicographically fist path in this `Bdd`, represented as
    /// a *conjunctive* clause.
    pub fn first_path(&self) -> Option<BddPartialValuation> {
        if self.is_false() {
            return None;
        }

        let mut valuation = BddPartialValuation::empty();
        let mut node = self.root_pointer();
        while !node.is_terminal() {
            if self.low_link_of(node).is_zero() {
                valuation.set_value(self.var_of(node), true);
                node = self.high_link_of(node);
            } else {
                valuation.set_value(self.var_of(node), false);
                node = self.low_link_of(node);
            }
        }

        Some(valuation)
    }

    /// Return the lexicographically last path in this `Bdd`, represented as
    /// a *conjunctive* clause.
    pub fn last_path(&self) -> Option<BddPartialValuation> {
        if self.is_false() {
            return None;
        }

        if self.is_false() {
            return None;
        }

        let mut valuation = BddPartialValuation::empty();
        let mut node = self.root_pointer();
        while !node.is_terminal() {
            if self.high_link_of(node).is_zero() {
                valuation.set_value(self.var_of(node), false);
                node = self.low_link_of(node);
            } else {
                valuation.set_value(self.var_of(node), true);
                node = self.high_link_of(node);
            }
        }

        Some(valuation)
    }

    /// Return a valuation in this `Bdd` that contains the greatest amount of positive literals.
    ///
    /// If such valuation is not unique, the method should return the one that is first
    /// lexicographically.
    pub fn most_positive_valuation(&self) -> Option<BddValuation> {
        if self.is_false() {
            return None;
        }

        let mut cache = Vec::with_capacity(self.size());
        cache.push((0, true));
        cache.push((0, true));

        for i in self.pointers().skip(2) {
            let i_var = self.var_of(i);
            let low_link = self.low_link_of(i);
            let high_link = self.high_link_of(i);

            // Parenthesis to avoid a chance of overflow.
            let low_link_diff =
                cache[low_link.to_index()].0 + ((self.var_of(low_link).0 - i_var.0) - 1);
            let high_link_diff =
                cache[high_link.to_index()].0 + ((self.var_of(high_link).0 - i_var.0) - 1);

            let result = if low_link.is_zero() && high_link.is_zero() {
                panic!("Non canonical BDD.")
            } else if low_link.is_zero() {
                (high_link_diff + 1, true)
            } else if high_link.is_zero() {
                (low_link_diff, false)
            } else if high_link_diff + 1 > low_link_diff {
                (high_link_diff + 1, true)
            } else {
                (low_link_diff, false)
            };

            cache.push(result);
        }

        let mut valuation = BddValuation::all_true(self.num_vars());
        let mut node = self.root_pointer();
        while !node.is_terminal() {
            let (_, child) = cache[node.to_index()];
            if child {
                node = self.high_link_of(node);
            } else {
                valuation.clear(self.var_of(node));
                node = self.low_link_of(node);
            }
        }

        Some(valuation)
    }

    /// Return a valuation in this `Bdd` that contains the greatest amount of negative literals.
    ///
    /// If such valuation is not unique, the method should return the one that is first
    /// lexicographically.
    pub fn most_negative_valuation(&self) -> Option<BddValuation> {
        if self.is_false() {
            return None;
        }

        let mut cache = Vec::with_capacity(self.size());
        cache.push((0, true));
        cache.push((0, true));

        for i in self.pointers().skip(2) {
            let i_var = self.var_of(i);
            let low_link = self.low_link_of(i);
            let high_link = self.high_link_of(i);

            // Parenthesis to avoid a chance of overflow.
            let low_link_diff =
                cache[low_link.to_index()].0 + ((self.var_of(low_link).0 - i_var.0) - 1);
            let high_link_diff =
                cache[high_link.to_index()].0 + ((self.var_of(high_link).0 - i_var.0) - 1);

            let result = if low_link.is_zero() && high_link.is_zero() {
                panic!("Non canonical BDD.")
            } else if low_link.is_zero() {
                (high_link_diff, true)
            } else if high_link.is_zero() {
                (low_link_diff + 1, false)
            } else if high_link_diff > low_link_diff + 1 {
                (high_link_diff, true)
            } else {
                (low_link_diff + 1, false)
            };

            cache.push(result);
        }

        let mut valuation = BddValuation::all_false(self.num_vars());
        let mut node = self.root_pointer();
        while !node.is_terminal() {
            let (_, child) = cache[node.to_index()];
            if child {
                valuation.set(self.var_of(node));
                node = self.high_link_of(node);
            } else {
                node = self.low_link_of(node);
            }
        }

        Some(valuation)
    }
}

#[cfg(test)]
mod tests {
    use crate::{BddPartialValuation, BddValuation, BddVariableSet};

    #[test]
    fn first_last_valuation() {
        let vars = BddVariableSet::new_anonymous(5);
        let bdd = vars.eval_expression_string("x_0 & (!x_2 | x_3) & !x_4");

        let first_valuation = BddValuation(vec![true, false, false, false, false]);
        let last_valuation = BddValuation(vec![true, true, true, true, false]);

        assert_eq!(Some(first_valuation), bdd.first_valuation());
        assert_eq!(Some(last_valuation), bdd.last_valuation());
    }

    #[test]
    fn first_last_path() {
        let vars = BddVariableSet::new_anonymous(5);
        let v = vars.variables();
        let bdd = vars.eval_expression_string("x_0 & (!x_2 | x_3) & !x_4");

        let first_path =
            BddPartialValuation::from_values(&[(v[0], true), (v[2], false), (v[4], false)]);

        let last_path = BddPartialValuation::from_values(&[
            (v[0], true),
            (v[2], true),
            (v[3], true),
            (v[4], false),
        ]);

        assert_eq!(Some(first_path), bdd.first_path());
        assert_eq!(Some(last_path), bdd.last_path());
    }

    #[test]
    fn most_positive_negative_valuation() {
        let vars = BddVariableSet::new_anonymous(5);
        let bdd = vars.eval_expression_string("x_0 & (!x_2 | x_3) & !x_4");

        let most_positive_valuation = BddValuation(vec![true, true, true, true, false]);
        let most_negative_valuation = BddValuation(vec![true, false, false, false, false]);

        assert_eq!(Some(most_positive_valuation), bdd.most_positive_valuation());
        assert_eq!(Some(most_negative_valuation), bdd.most_negative_valuation());
    }
}
