/* Copyright 2017 Christopher Bacher
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 * http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

//! The collection module contains matchers for asserting properties of collections and iterators.

use std::fmt::Debug;
use super::super::*;

use std::iter::FromIterator;

/// Matches if the asserted collection contains *all and only* the expected elements in any order.
pub struct ContainsInAnyOrder<T> {
    expected_elements: Vec<T>
}

/// Matches if the asserted collection contains *all and only* of the expected elements in any order.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![1,2,3,4,5,6], contains_in_any_order(vec![2,4,1,5,3,6]));
/// assert_that!(
///     // 6 is missing
///     assert_that!(&vec![1,2,3,4,5,6], contains_in_any_order(vec![2,4,1,5,3])),
///     panics
/// );
/// assert_that!(
///     // 7 is added
///     assert_that!(&vec![1,2,3,4,5,6], contains_in_any_order(vec![2,4,1,5,3,6,7])),
///     panics
/// );
/// # }
pub fn contains_in_any_order<'a,T:'a,I:'a>(expected_elements: I) -> Box<Matcher<'a,I> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T>,
      ContainsInAnyOrder<T>: Matcher<'a,I> {
    Box::new(ContainsInAnyOrder {
        expected_elements: expected_elements.into_iter().collect()
    })
}

impl<'a,T,I> Matcher<'a,I> for ContainsInAnyOrder<T>
where T: PartialEq + Debug + 'a,
      &'a I: IntoIterator<Item=&'a T> + Debug + 'a {
    fn check(&self, actual: &'a I) -> MatchResult {
        let repr = format!("{:?}", actual);
        let builder = MatchResultBuilder::for_("contains_in_any_order");
        let mut expected_elements = Vec::from_iter(self.expected_elements.iter());

        for ref element in actual.into_iter() {
            let maybe_pos = expected_elements.iter()
                                             .position(|candidate| element == candidate);;
            if let Some(idx) = maybe_pos {
                expected_elements.remove(idx);
            } else {
                return builder.failed_because(
                    &format!("{} contains an unexpected element: {:?}", repr, element)
                );
            }
        }

        if !expected_elements.is_empty() {
            builder.failed_because(
                &format!("{} did not contain the following elements: {:?}", repr, expected_elements)
            )
        } else { builder.matched() }
    }
}

/// Matches if the asserted collection contains *all and only* of the expected elements in the given order.
pub struct ContainsInOrder<T> {
    expected_elements: Vec<T>
}

/// Matches if the asserted collection contains *all and only* of the expected elements in the given order.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![1,2,3,4,5,6], contains_in_order(vec![1,2,3,4,5,6]));
/// assert_that!(
///     // 6 is missing
///     assert_that!(&vec![1,2,3,4,5,6], contains_in_order(vec![1,2,3,4,5])),
///     panics
/// );
/// assert_that!(
///     // 7 is added
///     assert_that!(&vec![1,2,3,4,5,6], contains_in_order(vec![1,2,3,4,5,6,7])),
///     panics
/// );
/// # }
pub fn contains_in_order<'a,T:'a,I:'a>(expected_elements: I) -> Box<Matcher<'a,I> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T>,
      ContainsInOrder<T>: Matcher<'a,I> {
    Box::new(ContainsInOrder {
        expected_elements: expected_elements.into_iter().collect()
    })
}

impl<'a, T, I:'a> Matcher<'a,I> for ContainsInOrder<T>
where T: PartialEq + Debug + 'a,
      &'a I: IntoIterator<Item=&'a T> + Debug + 'a {
    fn check(&self, actual: &'a I) -> MatchResult {
        let builder = MatchResultBuilder::for_("contains_in_order");
        let actual_list: Vec<_> = actual.into_iter().collect();

        if actual_list.len() > self.expected_elements.len() {
            return builder.failed_because(
                &format!("The expected list is shorter than the actual list by {} elements",
                         actual_list.len() - self.expected_elements.len())
            );
        }

        if actual_list.len() < self.expected_elements.len() {
            return builder.failed_because(
                &format!("The actual list is shorter than the expected list by {} elements",
                         self.expected_elements.len() - actual_list.len())
            );
        }

        let nonmatching: Vec<_> = actual_list.into_iter()
                                             .zip(self.expected_elements.iter())
                                             .filter(|&(act, exp)| act != exp)
                                             .collect();
        if !nonmatching.is_empty() {
            builder.failed_because(
                &format!("the following actual/expected pairs do not match: {:?}", nonmatching)
            )
        } else { builder.matched() }
    }
}

/// Matches if the asserted collection contains *all* (possibly more) of the expected elements.
pub struct ContainsSubset<T> {
    expected_elements: Vec<T>
}

/// Matches if the asserted collection contains *all* (possibly more) of the expected elements.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![1,2,3,4,5,6], contains_subset(vec![3,1,2,4]));
/// # }
pub fn contains_subset<'a,T:'a,I:'a>(expected_elements: I) -> Box<Matcher<'a,I> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T>,
      ContainsSubset<T>: Matcher<'a,I> {
    Box::new(ContainsSubset {
        expected_elements: expected_elements.into_iter().collect()
    })
}

impl<'a, T, I:'a> Matcher<'a,I> for ContainsSubset<T>
where T: PartialEq + Debug + 'a,
      &'a I: IntoIterator<Item=&'a T> + Debug + 'a {
    fn check(&self, actual: &'a I) -> MatchResult {
        let repr = format!("{:?}", actual);
        let builder = MatchResultBuilder::for_("contains_subset");
        let mut expected_elements = Vec::from_iter(self.expected_elements.iter());

        for element in actual.into_iter() {
            let maybe_pos = expected_elements.iter()
                                             .position(|candidate| element == *candidate);
            if let Some(idx) = maybe_pos {
                expected_elements.remove(idx);
            }
        }

        if !expected_elements.is_empty() {
            builder.failed_because(
                &format!("{} did not contain the following elements: {:?}", repr, expected_elements)
            )
        } else { builder.matched() }
    }
}

/// Matches if the asserted (single) value is contained in the expected elements.
pub struct ContainedIn<T> {
    expected_to_contain: Vec<T>
}

/// Matches if the asserted (single) value is contained in the expected elements.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&5, contained_in(vec![1,2,3,4,5,6,7,8]));
/// # }
pub fn contained_in<'a,T:'a,I>(expected_to_contain: I) -> Box<Matcher<'a,T> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T> {
    Box::new(ContainedIn {
        expected_to_contain: expected_to_contain.into_iter().collect()
    })
}

impl<'a,T> Matcher<'a,T> for ContainedIn<T>
where T: PartialEq + Debug + 'a  {
    fn check(&self, element: &T) -> MatchResult {
        let builder = MatchResultBuilder::for_("containd_in");
        if let None = self.expected_to_contain.iter().position(|e| e == element) {
            builder.failed_because(
                &format!("{:?} does not contain: {:?}", self.expected_to_contain, element)
            )
        } else { builder.matched() }
    }
}

/// Matches if the elements in the asserted collection are sorted weakly monotone according to the given `predicate` in the expected order.
///
/// The `predicate` is applied to all consecutive pairs of elements and returns the `Ordering` of the pair.
/// The ordering is allowed to be weakly monotone, i.e., equal elements are allowed to follow each other.
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// use std::cmp::Ordering;
/// # fn main() {
/// assert_that!(&vec![1,2,2,3,3,4,5,6], sorted_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
/// # }
pub fn sorted_by<'a,T,I,P>(predicate: P, expected_ordering: std::cmp::Ordering) -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a,
      P: Fn(&'a T,&'a T) -> std::cmp::Ordering + 'static {
    Box::new(move |elements: &'a I| {
        let builder = MatchResultBuilder::for_("sorted_by");
        let mut iter = elements.into_iter();
        let maybe_prev = iter.next();

        if maybe_prev.is_none() { return builder.matched() }
        let mut prev = maybe_prev.unwrap();

        for cur in iter {
            let ordering = predicate(&prev, &cur);
            if ordering != std::cmp::Ordering::Equal
                      && expected_ordering != ordering  {
                return builder.failed_because(
                    &format!("ordering is not monotone: predicate({:?}, {:?}) != {:?}",
                             prev, cur, expected_ordering)
                );
            }
            prev = cur;
        }
        builder.matched()
    })
}

/// Matches if the elements in the asserted collection are sorted strictly monotone according to the given `predicate` in the expected order`.
///
/// The `predicate` is applied to all consecutive pairs of elements and returns the `Ordering` of the pair.
/// The ordering is allowed to be weakly monotone, i.e., equal elements are allowed to follow each other.
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// use std::cmp::Ordering;
/// # fn main() {
/// assert_that!(&vec![1,2,3,4,5,6], sorted_strictly_by(|a: &i32, b: &i32| a.cmp(b), Ordering::Less));
/// # }
pub fn sorted_strictly_by<'a,T,I,P>(predicate: P, expected_ordering: std::cmp::Ordering) -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a,
      P: Fn(&'a T,&'a T) -> std::cmp::Ordering + 'static {
    Box::new(move |elements: &'a I| {
        let builder = MatchResultBuilder::for_("sorted_strictly_by");
        let mut iter = elements.into_iter();
        let maybe_prev = iter.next();

        if maybe_prev.is_none() { return builder.matched() }
        let mut prev = maybe_prev.unwrap();

        for cur in iter {
            let ordering = predicate(&prev, &cur);
            if expected_ordering != ordering  {
                return builder.failed_because(
                    &format!("ordering is not strictly monotone: predicate({:?}, {:?}) != {:?}", prev, cur, expected_ordering)
                );
            }
            prev = cur;
        }
        builder.matched()
    })
}

/// Matches if the elements in the asserted collection are sorted weakly monotone according to the given `predicate` in any order.
///
/// The `predicate` is applied to all consecutive pairs of elements and returns the `Ordering` of the pair.
/// The first `Ordering` different to `Ordering::Equal` defines the expected order of the collection.
/// The ordering is allowed to be weakly monotone, i.e., equal elements are allowed to follow each other.
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![5,4,3,3,2,1,1], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
/// assert_that!(&vec![1,1,2,3,3,4,5], sorted_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
/// # }
pub fn sorted_by_in_any_order<'a,T,I,P>(predicate: P) -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a,
      P: Fn(&'a T,&'a T) -> std::cmp::Ordering + 'static {
    Box::new(move |elements: &'a I| {
        let builder = MatchResultBuilder::for_("sorted_by_in_any_order");
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
                return builder.failed_because(
                    &format!("ordering is not monotone: predicate({:?}, {:?}) != {:?}",
                             prev, cur, expected_ordering.unwrap())
                );
            }
            prev = cur;
        }
        builder.matched()
    })
}

/// Matches if the elements in the asserted collection are sorted strictly monotone according to the given `predicate` in any order.
///
/// The `predicate` is applied to all consecutive pairs of elements and returns the `Ordering` of the pair.
/// The first `Ordering` different to `Ordering::Equal` defines the expected order of the collection.
/// The ordering is allowed to be weakly monotone, i.e., equal elements are allowed to follow each other.
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![5,4,3,2,1], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
/// assert_that!(&vec![1,2,3,4,5], sorted_strictly_by_in_any_order(|a: &i32, b: &i32| a.cmp(b)));
/// # }
pub fn sorted_strictly_by_in_any_order<'a,T,I,P>(predicate: P) -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a,
      P: Fn(&'a T,&'a T) -> std::cmp::Ordering + 'static {
    Box::new(move |elements: &'a I| {
        let builder = MatchResultBuilder::for_("sorted_strictly_by_in_any_order");
        let mut iter = elements.into_iter();
        let mut expected_ordering: Option<std::cmp::Ordering> = None;
        let maybe_prev = iter.next();
        if maybe_prev.is_none() {
            return builder.matched();
        }
        let mut prev = maybe_prev.unwrap();

        for cur in iter {
            let ordering = predicate(&prev, &cur);
            if ordering == std::cmp::Ordering::Equal {
                return builder.failed_because(
                    &format!("ordering is not strictly monotone: predicate({:?}, {:?}) = {:?}",
                             prev, cur, ordering)
                );
            }
            if expected_ordering == None {
                expected_ordering = Some(ordering);
            } else if expected_ordering.unwrap() != ordering  {
                return builder.failed_because(
                    &format!("ordering is not strictly monotone: predicate({:?}, {:?}) != {:?}",
                             prev, cur, expected_ordering.unwrap())
                );
            }
            prev = cur;
        }
        builder.matched()
    })
}

/// Matches if the asserted collection is sorted weakly ascending.
///
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![1,2,2,3,4,4,5], sorted_ascending());
/// # }
pub fn sorted_ascending<'a,T,I>() -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a {
    sorted_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Less)
}

/// Matches if the asserted collection is sorted strictly ascending.
///
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![1,2,3,4,5], sorted_strictly_ascending());
/// # }
pub fn sorted_strictly_ascending<'a,T,I>() -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a {
    sorted_strictly_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Less)
}

/// Matches if the asserted collection is sorted weakly descending.
///
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![5,4,4,3,3,2,1], sorted_descending());
/// # }
pub fn sorted_descending<'a,T,I>() -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a {
    sorted_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Greater)
}

/// Matches if the asserted collection is sorted strictly descending.
///
/// An empty collection is assumed to be always sorted.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![5,4,3,2,1], sorted_strictly_descending());
/// # }
pub fn sorted_strictly_descending<'a,T,I>() -> Box<Fn(&'a I) -> MatchResult>
where &'a I: IntoIterator<Item=&'a T> + 'a,
      T: Ord + Debug + 'a {
    sorted_strictly_by(|a: &T, b: &T| a.cmp(b), std::cmp::Ordering::Greater)
}

/// Matches if all elements in the asserted collection satisfy the given `predicate`.
///
/// An empty collection always satisfies this matcher as all (=no) element satisfies the predicate.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![1,2,3,4,5], all_elements_satisfy(|&a| 0 <= a && a < 100));
/// # }
pub fn all_elements_satisfy<'a,T,I,P>(predicate: P) -> Box<Fn(&'a I) -> MatchResult>
where T: Debug + 'a,
      &'a I: IntoIterator<Item=&'a T> + 'a,
      P: Fn(&'a T) -> bool + 'static {
    Box::new(move |elements: &'a I| {
        let builder = MatchResultBuilder::for_("all_elements_satisfy");
        let nonsatisfying_elements: Vec<_> = elements.into_iter().filter(|e| !predicate(e)).collect();
        if !nonsatisfying_elements.is_empty() {
            builder.failed_because(
                &format!("the following elements do not satisfy the predicate: {:?}", nonsatisfying_elements)
            )
        } else {
            builder.matched()
        }
    })
}

/// Matches if at least one element in the asserted collection satisfy the given `predicate`.
///
/// An empty collection never satisfies this matcher as no element satisfies the predicate.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// assert_that!(&vec![1,2,3,4,5], some_elements_satisfy(|&a| 2 <= a && a < 5));
/// # }
pub fn some_elements_satisfy<'a,T,I,P>(predicate: P) -> Box<Fn(&'a I) -> MatchResult>
where T: Debug + 'a,
      &'a I: IntoIterator<Item=&'a T> + 'a,
      P: Fn(&T) -> bool + 'static {
    Box::new(move |elements: &'a I| {
        let builder = MatchResultBuilder::for_("some_elements_satisfy");
        if !elements.into_iter().any(|ref e| predicate(e)) {
            builder.failed_because("no elements satisfy the predicate")
        } else {
            builder.matched()
        }
    })
}

/// Matches if the map-like collection contains the given key/value pair.
///
/// The `Matcher` tests for this by converting the map-like data structure
/// into a key/value pair iterator.
///
/// The alternative would be to use the Index trait though experiments showed
/// that this would not be composable with `all_of!` or `any_of!`.
pub struct HasEntry<K,V> {
    key: K,
    value: V
}

/// Matches if the map-like collection contains the given key/value pair.
///
/// The `Matcher` tests for this by converting the map-like data structure
/// into a key/value pair iterator.
///
/// The alternative would be to use the Index trait though experiments showed
/// that this would not be composable with `all_of!` or `any_of!`.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// let mut map = std::collections::HashMap::<i32,i32>::new();
/// map.insert(0, 2);
/// map.insert(1, 2);
/// map.insert(2, 5);
/// map.insert(3, 3);
/// map.insert(4, 3);
///
/// assert_that!(&map, has_entry(1, 2));
/// # }
pub fn has_entry<'a,K:'a,V:'a,M:'a>(key: K, value: V) -> Box<Matcher<'a,M> + 'a>
where &'a M: IntoIterator<Item=(&'a K,&'a V)> + 'a,
      HasEntry<K,V>: Matcher<'a,M> {
    Box::new(HasEntry {
        key: key,
        value: value
    })
}

impl<'a,K,V,M> Matcher<'a,M> for HasEntry<K,V>
where V: PartialEq + Debug + 'a,
      K: PartialEq + Debug + 'a,
      &'a M: IntoIterator<Item=(&'a K,&'a V)> + 'a {

    fn check(&self, map: &'a M) -> MatchResult {
        let builder = MatchResultBuilder::for_("has_entry");
        let mut same_keys = Vec::new();
        let mut same_values = Vec::new();
        for (key, value) in map.into_iter() {
            if key == &self.key && value == &self.value {
                return builder.matched()
            }
            if key == &self.key {
                same_keys.push(value);
            }
            if value == &self.value {
                same_values.push(key);
            }
        }

        builder.failed_because(&format!(
            "Entry ({:?}, {:?}) not found.\n\tEntries with same key: {:?}\n\tEntries with same value: {:?}",
            &self.key, &self.value,
            same_keys, same_values
        ))
    }
}

/// Matches if the map-like collection contains the given key.
///
/// The `Matcher` tests for this by converting the map-like data structure
/// into a key/value pair iterator.
///
/// The alternative would be to use the Index trait though experiments showed
/// that this would not be composable with `all_of!` or `any_of!`.
pub struct HasKey<K> {
    key: K
}

/// Matches if the map-like collection contains the given key.
///
/// The `Matcher` tests for this by converting the map-like data structure
/// into a key/value pair iterator.
///
/// The alternative would be to use the Index trait though experiments showed
/// that this would not be composable with `all_of!` or `any_of!`.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// let mut map = std::collections::HashMap::<i32,i32>::new();
/// map.insert(0, 2);
/// map.insert(1, 2);
/// map.insert(2, 5);
/// map.insert(3, 3);
/// map.insert(4, 3);
///
/// assert_that!(&map, has_key(2));
/// # }
pub fn has_key<'a,K:'a,V:'a,M:'a>(key: K) -> Box<Matcher<'a,M> + 'a>
where &'a M: IntoIterator<Item=(&'a K,&'a V)> + 'a,
      HasKey<K>: Matcher<'a,M> {
    Box::new(HasKey {
        key: key
    })
}

impl<'a,K,V,M> Matcher<'a,M> for HasKey<K>
where V: PartialEq + Debug + 'a,
      K: PartialEq + Debug + 'a,
      &'a M: IntoIterator<Item=(&'a K,&'a V)> + 'a {

    fn check(&self, map: &'a M) -> MatchResult {
        let builder = MatchResultBuilder::for_("has_key");
        for (key, _) in map.into_iter() {
            if key == &self.key {
                return builder.matched();
            }
        }

        builder.failed_because(&format!("No entrywith key {:?} found", &self.key))
    }
}


/// Matches if the map-like collection contains the given value.
///
/// The `Matcher` tests for this by converting the map-like data structure
/// into a key/value pair iterator.
pub struct HasValue<V> {
    value: V
}

/// Matches if the map-like collection contains the given value.
///
/// The `Matcher` tests for this by converting the map-like data structure
/// into a key/value pair iterator.
///
/// #Examples
/// ```rust
/// # #[macro_use] extern crate galvanic_assert;
/// use galvanic_assert::matchers::collection::*;
/// # fn main() {
/// let mut map = std::collections::HashMap::<i32,i32>::new();
/// map.insert(0, 2);
/// map.insert(1, 2);
/// map.insert(2, 5);
/// map.insert(3, 3);
/// map.insert(4, 3);
///
/// assert_that!(&map, has_value(3));
/// # }
pub fn has_value<'a,K:'a,V:'a,M:'a>(key: K) -> Box<Matcher<'a,M> + 'a>
where &'a M: IntoIterator<Item=(&'a K,&'a V)> + 'a,
      HasKey<K>: Matcher<'a,M> {
    Box::new(HasKey {
        key: key
    })
}

impl<'a,K,V,M> Matcher<'a,M> for HasValue<V>
where V: PartialEq + Debug + 'a,
      K: PartialEq + Debug + 'a,
      &'a M: IntoIterator<Item=(&'a K,&'a V)> + 'a {

    fn check(&self, map: &'a M) -> MatchResult {
        let builder = MatchResultBuilder::for_("has_value");
        for (_, value) in map.into_iter() {
            if value == &self.value {
                return builder.matched();
            }
        }

        builder.failed_because(&format!("No entry with value {:?} found", &self.value))
    }
}
