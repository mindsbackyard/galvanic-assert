use super::super::*;

#[macro_export]
macro_rules! all_of {
    ( $matcher: expr ) => {
        All::of($matcher)
    };
    ( $matcher: expr, $($matchers: expr),* ) => {
        All::of($matcher)$(.and($matchers))*
    };
}

pub struct All<'a, T:'a> {
    matcher: Box<Matcher<&'a T> + 'a>,
    next: Option<Box<All<'a,T>>>
}

impl<'a, T> All<'a, T> {
    pub fn of<M>(matcher: M) -> All<'a,T>
    where M: Matcher<&'a T> + 'a, T: 'a {
        All {
            matcher: Box::new(matcher),
            next: None
        }
    }

    pub fn and<M>(self, matcher: M) -> All<'a,T>
    where M: Matcher<&'a T> + 'a, T: 'a {
        All {
            matcher: Box::new(matcher),
            next: Some(Box::new(self))
        }
    }
}

impl<'a,T> Matcher<&'a T> for All<'a,T> {
    fn check(&self, actual: &'a T) -> MatchResult {
        match self.matcher.check(actual) {
            x@MatchResult::Matched {..} => {
                match self.next {
                    None => x,
                    Some(ref next) => next.check(actual)
                }
            },
            x@MatchResult::Failed {..} => x
        }
    }
}

#[macro_export]
macro_rules! any_of {
    ( $matcher: expr ) => {
        Any::of($matcher)
    };
    ( $matcher: expr, $($matchers: expr),* ) => {
        Any::of($matcher)$(.or($matchers))*
    };
}

pub struct Any<'a, T:'a> {
    matcher: Box<Matcher<&'a T> + 'a>,
    next: Option<Box<Any<'a,T>>>
}

impl<'a, T> Any<'a, T> {
    pub fn of<M>(matcher: M) -> Any<'a,T>
    where M: Matcher<&'a T> + 'a, T: 'a {
        Any {
            matcher: Box::new(matcher),
            next: None
        }
    }

    pub fn or<M>(self, matcher: M) -> Any<'a,T>
    where M: Matcher<&'a T> + 'a, T:'a {
        Any {
            matcher: Box::new(matcher),
            next: Some(Box::new(self))
        }
    }
}

impl<'a,T> Matcher<&'a T> for Any<'a,T> {
    fn check(&self, actual: &'a T) -> MatchResult {
        match self.matcher.check(actual) {
            MatchResult::Matched {..} => MatchResult::Matched { name: "any_of".to_owned() },
            x@MatchResult::Failed {..} => match self.next {
                None => x,
                Some(ref next) => next.check(actual)
            }
        }
    }
}
