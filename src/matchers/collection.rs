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
use super::super::*;
use std::cmp::Ordering;
use std::fmt::Debug;
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
pub fn contains_in_any_order<'a,T:'a,I:'a,J:'a>(expected_elements: I) -> Box<Matcher<J> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T>,
      J: IntoIterator<Item=T>,
      ContainsInAnyOrder<T>: Matcher<J> {
    Box::new(ContainsInAnyOrder {
        expected_elements: expected_elements.into_iter().collect()
    })
}

impl<T,I> Matcher<I> for ContainsInAnyOrder<T>
where T: PartialEq + Debug,
      for<'all> &'all I: IntoIterator<Item=&'all T> + Debug {
    fn check(&self, actual: &I) -> MatchResult {
        let repr = format!("{:?}", actual);
        let builder = MatchResultBuilder::for_("contains_in_any_order");
        let mut expected_elements = Vec::from_iter(self.expected_elements.iter());

        for ref element in actual.into_iter() {
            let maybe_pos = expected_elements.iter()
                                             .position(|candidate| element == candidate);
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
pub fn contains_in_order<'a,T:'a,I:'a,J:'a>(expected_elements: I) -> Box<Matcher<J> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T>,
      J: IntoIterator<Item=T>,
      ContainsInOrder<T>: Matcher<J> {
    Box::new(ContainsInOrder {
        expected_elements: expected_elements.into_iter().collect()
    })
}

impl<T, I> Matcher<I> for ContainsInOrder<T>
where T: PartialEq + Debug,
      for<'all> &'all I: IntoIterator<Item=&'all T> + Debug {
    fn check(&self, actual: &I) -> MatchResult {
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
pub fn contains_subset<'a,T:'a,I:'a,J:'a>(expected_elements: I) -> Box<Matcher<J> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T>,
      J: IntoIterator<Item=T>,
      ContainsSubset<T>: Matcher<J> {
    Box::new(ContainsSubset {
        expected_elements: expected_elements.into_iter().collect()
    })
}

impl<T, I> Matcher<I> for ContainsSubset<T>
where T: PartialEq + Debug,
      for<'all >&'all I: IntoIterator<Item=&'all T> + Debug {
    fn check(&self, actual: &I) -> MatchResult {
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
pub fn contained_in<'a,T:'a,I>(expected_to_contain: I) -> Box<Matcher<T> + 'a>
where T: PartialEq + Debug,
      I: IntoIterator<Item=T> {
    Box::new(ContainedIn {
        expected_to_contain: expected_to_contain.into_iter().collect()
    })
}

impl<T> Matcher<T> for ContainedIn<T>
where T: PartialEq + Debug  {
    fn check(&self, element: &T) -> MatchResult {
        let builder = MatchResultBuilder::for_("containd_in");
        if let None = self.expected_to_contain.iter().position(|e| e == element) {
            builder.failed_because(
                &format!("{:?} does not contain: {:?}", self.expected_to_contain, element)
            )
        } else { builder.matched() }
    }
}


fn sorted_according_to<'a,T:'a, I, P:'a>(
    predicate: P,
    expected_ordering: Option<Ordering>,
    is_strict: bool,
) -> Box<Matcher<I> + 'a>
where
    T: Ord + Debug,
    for<'all> &'all I: IntoIterator<Item=&'all T>,
    for<'all> P: Fn(&'all T, &'all T) -> Ordering
{
    Box::new(move |elements: &I| {
        let builder = MatchResultBuilder::for_("sorted_according_to");

        let window_iter = elements.into_iter().zip({
            let mut second = elements.into_iter();
            second.next();
            second
        });

        let mut prev_ordering = expected_ordering;

        for (first, second) in window_iter {
            let ordering = predicate(first, second);
            if prev_ordering.map(|o| o != ordering && ordering != Ordering::Equal).unwrap_or(false) {
                return builder.failed_because(
                    &format!("Ordering of iterable is not monotone: {:?}) not {:?} {:?}", first, prev_ordering, second)
                );
            }
            if is_strict && ordering == Ordering::Equal {
                return builder.failed_because(
                    &format!("Ordering of iterable is not strictly monotone: {:?}) == {:?}", first, second)
                );
            }
            if prev_ordering.is_none() && ordering != Ordering::Equal {
                prev_ordering = Some(ordering)
            }
        }
        builder.matched()
    })
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
pub fn sorted_by<'a, T: 'a, I: 'a, P: 'a>(predicate: P, expected_ordering: Ordering) -> Box<Matcher<I> + 'a>
where
    T: Ord + Debug,
    for<'all> &'all I: IntoIterator<Item=&'all T>,
    for<'all> P: Fn(&'all T, &'all T) -> Ordering
{
    rename_matcher("sorted_by".to_owned(), sorted_according_to(predicate, Some(expected_ordering), false))
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
pub fn sorted_strictly_by<'a, T: 'a, I: 'a, P: 'a>(predicate: P, expected_ordering: Ordering) -> Box<Matcher<I> + 'a>
where
    T: Ord + Debug,
    for<'all> &'all I: IntoIterator<Item=&'all T>,
    for<'all> P: Fn(&'all T, &'all T) -> Ordering
{
    rename_matcher("sorted_strictly_by".to_owned(), sorted_according_to(predicate, Some(expected_ordering), true))
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
pub fn sorted_by_in_any_order<'a, T: 'a, I: 'a, P: 'a>(predicate: P) -> Box<Matcher<I> + 'a>
where
    T: Ord + Debug,
    for<'all> &'all I: IntoIterator<Item=&'all T>,
    for<'all> P: Fn(&'all T, &'all T) -> Ordering
{
    rename_matcher("sorted_by_in_any_order".to_owned(), sorted_according_to(predicate, None, false))
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
pub fn sorted_strictly_by_in_any_order<'a, T: 'a, I: 'a, P: 'a>(predicate: P) -> Box<Matcher<I> + 'a>
where
    T: Ord + Debug,
    for<'all> &'all I: IntoIterator<Item=&'all T>,
    for<'all> P: Fn(&'all T, &'all T) -> Ordering
{
    rename_matcher("sorted_strictly_by_in_any_order".to_owned(), sorted_according_to(predicate, None, true))
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
pub fn sorted_ascending<'a, T: 'a, I: 'a>() -> Box<Matcher<I> + 'a>
where T: Ord + Debug,
      for<'all> &'all I: IntoIterator<Item=&'all T> {
    sorted_by(|a: &T, b: &T| a.cmp(b), Ordering::Less)
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
pub fn sorted_strictly_ascending<'a, T: 'a, I: 'a>() -> Box<Matcher<I> + 'a>
where T: Ord + Debug,
      for<'all> &'all I: IntoIterator<Item=&'all T> {
    sorted_strictly_by(|a: &T, b: &T| a.cmp(b), Ordering::Less)
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
pub fn sorted_descending<'a, T: 'a, I: 'a>() -> Box<Matcher<I> + 'a>
where T: Ord + Debug,
      for<'all> &'all I: IntoIterator<Item=&'all T> {
    sorted_by(|a: &T, b: &T| a.cmp(b), Ordering::Greater)
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
pub fn sorted_strictly_descending<'a, T: 'a, I: 'a>() -> Box<Matcher<I> + 'a>
where T: Ord + Debug,
      for<'all> &'all I: IntoIterator<Item=&'all T> {
    sorted_strictly_by(|a: &T, b: &T| a.cmp(b), Ordering::Greater)
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
pub fn all_elements_satisfy<'a, T: 'a, I, P: 'a>(predicate: P) -> Box<Matcher<I> + 'a>
where T: Debug,
      for<'all> &'all I: IntoIterator<Item=&'all T>,
      for<'all> P: Fn(&'all T) -> bool {
    Box::new(move |elements: &I| {
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
pub fn some_elements_satisfy<'a, T: 'a, I, P: 'a>(predicate: P) -> Box<Matcher<I> + 'a>
where
    T: Debug,
    for<'all> &'all I: IntoIterator<Item=&'all T>,
    for<'all> P: Fn(&'all T) -> bool
{
    Box::new(move |elements: &I| {
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
pub fn has_entry<'a,K:'a,V:'a,M:'a>(key: K, value: V) -> Box<Matcher<M> + 'a>
where K: PartialEq + Debug,
      V: PartialEq + Debug,
      for<'all> &'all M: IntoIterator<Item=(&'all K, &'all V)> {
    Box::new(HasEntry { key, value })
}

impl<K,V,M> Matcher<M> for HasEntry<K,V>
where K: PartialEq + Debug,
      V: PartialEq + Debug,
      for<'all> &'all M: IntoIterator<Item=(&'all K, &'all V)> {

    fn check(&self, map: &M) -> MatchResult {
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
pub fn has_key<'a, K:'a, V, M>(key: K) -> Box<Matcher<M> + 'a>
where K: PartialEq + Debug,
      for<'all> &'all M: IntoIterator<Item=(&'all K, &'all V)> {
    Box::new(HasKey { key })
}

impl<K,V,M> Matcher<M> for HasKey<K>
where K: PartialEq + Debug,
      for<'all> &'all M: IntoIterator<Item=(&'all K, &'all V)> {

    fn check(&self, map: &M) -> MatchResult {
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
pub fn has_value<'a, K, V:'a, M>(value: V) -> Box<Matcher<M> + 'a>
where V: PartialEq + Debug,
      for<'all> &'all M: IntoIterator<Item=(&'all K, &'all V)> {
    Box::new(HasValue { value })
}

impl<K,V,M> Matcher<M> for HasValue<V>
where V: PartialEq + Debug,
      for<'all> &'all M: IntoIterator<Item=(&'all K, &'all V)> {

    fn check(&self, map: &M) -> MatchResult {
        let builder = MatchResultBuilder::for_("has_value");
        for (_, value) in map.into_iter() {
            if value == &self.value {
                return builder.matched();
            }
        }

        builder.failed_because(&format!("No entry with value {:?} found", &self.value))
    }
}
