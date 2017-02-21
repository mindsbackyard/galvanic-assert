use std::fmt::Debug;
use super::super::*;

use std::iter::FromIterator;

pub struct ContainsInAnyOrder<T> {
    expected_elements: Vec<T>
}

pub fn contains_in_any_order<T,I>(expected_elements: I) -> ContainsInAnyOrder<T>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T> {
    ContainsInAnyOrder {
        expected_elements: expected_elements.into_iter().collect()
    }
}

impl<'a, T, I> Matcher<I> for ContainsInAnyOrder<T>
where T: PartialEq + Debug + 'a,
      I: IntoIterator<Item=T> + Debug {
    fn check(&self, actual: I) -> MatchResult {
        let repr = format!("{:?}", actual);
        let mut expected_elements = Vec::from_iter(self.expected_elements.iter());

        for element in actual.into_iter() {
            let maybe_pos = expected_elements.iter()
                                             .position(|candidate| element == **candidate);
            if let Some(idx) = maybe_pos {
                expected_elements.remove(idx);
            } else {
                return MatchResult::Failed {
                    name: "contains_in_any_order".to_owned(),
                    reason: format_fail_reason(&format!("{} contains an unexpected element: {:?}", repr, element))
                };
            }
        }

        if !expected_elements.is_empty() {
            let failed = MatchResult::Failed {
                name: "contains_in_any_order".to_owned(),
                reason: format_fail_reason(&format!("{} did not contain the following elements: {:?}", repr, expected_elements))
            };
            return failed;
        }

        MatchResult::Matched { name: "contains_in_any_order".to_owned() }
    }
}


pub struct ContainsInOrder<T> {
    expected_elements: Vec<T>
}

pub fn contains_in_order<T,I>(expected_elements: I) -> ContainsInOrder<T>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T> {
    ContainsInOrder {
        expected_elements: expected_elements.into_iter().collect()
    }
}

impl<'a, T, I> Matcher<I> for ContainsInOrder<T>
where T: PartialEq + Debug + 'a,
      I: IntoIterator<Item=T> + Debug {
    fn check(&self, actual: I) -> MatchResult {
        let actual_list: Vec<_> = actual.into_iter().collect();

        if actual_list.len() > self.expected_elements.len() {
            return MatchResult::Failed {
                name: "contains_in_order".to_owned(),
                reason: format_fail_reason(&format!("The expected list is shorter than the actual list by {} elements", actual_list.len() - self.expected_elements.len()))
            };
        }

        if actual_list.len() < self.expected_elements.len() {
            return MatchResult::Failed {
                name: "contains_in_order".to_owned(),
                reason: format_fail_reason(&format!("The actual list is shorter than the expected list by {} elements", self.expected_elements.len() - actual_list.len()))
            };
        }

        let nonmatching: Vec<_> = actual_list.into_iter()
                                             .zip(self.expected_elements.iter())
                                             .filter(|&(ref act, exp)| *act != *exp)
                                             .collect();
        if !nonmatching.is_empty() {
            return MatchResult::Failed {
                name: "contains_in_order".to_owned(),
                reason: format_fail_reason(&format!("the following actual/expected pairs do not match: {:?}", nonmatching))
            };
        }
        MatchResult::Matched { name: "contains_in_order".to_owned() }
    }
}


pub struct ContainedIn<T> {
    expected_to_contain: Vec<T>
}

pub fn contained_in<T,I>(expected_to_contain: I) -> ContainedIn<T>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T> {
    ContainedIn {
        expected_to_contain: expected_to_contain.into_iter().collect()
    }
}

impl<'a, T> Matcher<T> for ContainedIn<T>
where T: PartialEq + Debug  {
    fn check(&self, element: T) -> MatchResult {
        if let None = self.expected_to_contain.iter().position(|e| *e == element) {
            MatchResult::Failed {
                name: "contained_in".to_owned(),
                reason: format_fail_reason(&format!("{:?} does not contain: {:?}", self.expected_to_contain, element))
            }
        } else {
            MatchResult::Matched { name: "contained_in".to_owned() }
        }
    }
}

pub fn sorted_by<T,I,P>(predicate: P, expected_ordering: std::cmp::Ordering) -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug,
      P: Fn(&T,&T) -> std::cmp::Ordering {
    move |elements: I| {
        let mut iter = elements.into_iter();
        let maybe_prev = iter.next();
        if maybe_prev.is_none() {
            return MatchResult::Matched { name: "sorted_by".to_owned() };
        }
        let mut prev = maybe_prev.unwrap();

        for cur in iter {
            let ordering = predicate(&prev, &cur);
            if ordering != std::cmp::Ordering::Equal
                      && expected_ordering != ordering  {
                return MatchResult::Failed {
                    name: "sorted_by".to_owned(),
                    reason: format_fail_reason(&format!("ordering is not monotone: predicate({:?}, {:?}) != {:?}", prev, cur, expected_ordering))
                };
            }
            prev = cur;
        }
        MatchResult::Matched { name: "sorted_by".to_owned() }
    }
}

pub fn sorted_strictly_by<T,I,P>(predicate: P, expected_ordering: std::cmp::Ordering) -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug,
      P: Fn(&T,&T) -> std::cmp::Ordering {
    move |elements: I| {
        let mut iter = elements.into_iter();
        let maybe_prev = iter.next();
        if maybe_prev.is_none() {
            return MatchResult::Matched { name: "sorted_strictly_by".to_owned() };
        }
        let mut prev = maybe_prev.unwrap();

        for cur in iter {
            let ordering = predicate(&prev, &cur);
            if expected_ordering != ordering  {
                return MatchResult::Failed {
                    name: "sorted_strictly_by".to_owned(),
                    reason: format_fail_reason(&format!("ordering is not strictly monotone: predicate({:?}, {:?}) != {:?}", prev, cur, expected_ordering))
                };
            }
            prev = cur;
        }
        MatchResult::Matched { name: "sorted_strictly_by".to_owned() }
    }
}

pub fn sorted_by_in_any_order<T,I,P>(predicate: P) -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug,
      P: Fn(&T,&T) -> std::cmp::Ordering {
    move |elements: I| {
        let mut iter = elements.into_iter();
        let mut expected_ordering: Option<std::cmp::Ordering> = None;
        let maybe_prev = iter.next();
        if maybe_prev.is_none() {
            return MatchResult::Matched { name: "sorted_by_in_any_order".to_owned() };
        }
        let mut prev = maybe_prev.unwrap();

        for cur in iter {
            let ordering = predicate(&prev, &cur);
            if expected_ordering == None && ordering != std::cmp::Ordering::Equal {
                expected_ordering = Some(ordering);
            } else if ordering != std::cmp::Ordering::Equal
                      && expected_ordering.unwrap() != ordering  {
                return MatchResult::Failed {
                    name: "sorted_by_in_any_order".to_owned(),
                    reason: format_fail_reason(&format!("ordering is not monotone: predicate({:?}, {:?}) != {:?}", prev, cur, expected_ordering.unwrap()))
                };
            }
            prev = cur;
        }
        MatchResult::Matched { name: "sorted_by_in_any_order".to_owned() }
    }
}

pub fn sorted_strictly_by_in_any_order<T,I,P>(predicate: P) -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug,
      P: Fn(&T,&T) -> std::cmp::Ordering {
    move |elements: I| {
        let mut iter = elements.into_iter();
        let mut expected_ordering: Option<std::cmp::Ordering> = None;
        let maybe_prev = iter.next();
        if maybe_prev.is_none() {
            return MatchResult::Matched { name: "sorted_strictly_by_in_any_order".to_owned() };
        }
        let mut prev = maybe_prev.unwrap();

        for cur in iter {
            let ordering = predicate(&prev, &cur);
            if ordering == std::cmp::Ordering::Equal {
                return MatchResult::Failed {
                    name: "sorted_strictly_by_in_any_order".to_owned(),
                    reason: format_fail_reason(&format!("ordering is not strictly monotone: predicate({:?}, {:?}) = {:?}", prev, cur, ordering))
                };
            }
            if expected_ordering == None {
                expected_ordering = Some(ordering);
            } else if expected_ordering.unwrap() != ordering  {
                return MatchResult::Failed {
                    name: "sorted_strictly_by_in_any_order".to_owned(),
                    reason: format_fail_reason(&format!("ordering is not strictly monotone: predicate({:?}, {:?}) != {:?}", prev, cur, expected_ordering.unwrap()))
                };
            }
            prev = cur;
        }
        MatchResult::Matched { name: "sorted_strictly_by_in_any_order".to_owned() }
    }
}

pub fn sorted_ascending<T,I>() -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug {
    sorted_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Less)
}

pub fn sorted_strictly_ascending<T,I>() -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug {
    sorted_strictly_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Less)
}

pub fn sorted_descending<T,I>() -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug {
    sorted_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Greater)
}

pub fn sorted_strictly_descending<T,I>() -> impl Fn(I) -> MatchResult
where I: IntoIterator<Item=T>,
      T: Ord + Debug {
    sorted_strictly_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Greater)
}


pub fn all_elements_satisfy<T,I,P>(predicate: P) -> impl Fn(I) -> MatchResult
where T: Debug,
      I: IntoIterator<Item=T>,
      P: Fn(&T) -> bool {
    move |elements: I| {
        let nonsatisfying_elements: Vec<_> = elements.into_iter().filter(|e| !predicate(e)).collect();
        if !nonsatisfying_elements.is_empty() {
            MatchResult::Failed {
                name: "all_elements_satisfy".to_owned(),
                reason: format_fail_reason(&format!("the following elements do not satisfy the predicate: {:?}", nonsatisfying_elements))
            }
        } else {
            MatchResult::Matched { name: "all_elements_satisfy".to_owned() }
        }
    }
}


pub fn some_elements_satisfy<T,I,P>(predicate: P) -> impl Fn(I) -> MatchResult
where T: Debug,
      I: IntoIterator<Item=T>,
      P: Fn(&T) -> bool {
    move |elements: I| {
        if !elements.into_iter().any(|ref e| predicate(e)) {
            MatchResult::Failed {
                name: "some_elements_satisfy".to_owned(),
                reason: format_fail_reason("no elements satisfy the predicate")
            }
        } else {
            MatchResult::Matched { name: "some_elements_satisfy".to_owned() }
        }
    }
}
