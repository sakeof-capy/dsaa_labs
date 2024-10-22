use common::containers::traits::ReadableContainer;
use std::ops::Add;

enum TripleReplacement {
    NotReplaced,
    ReplacedNone,
    ReplacedMax,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Triple<T>
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    pub first: Option<T>,
    pub second: Option<T>,
    pub third: Option<T>,
}

impl<T> Triple<T>
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    fn push(&mut self, element: T, compare_fn: impl Fn(&T, &T) -> bool) {
        Self::push_utility(
            &mut [&mut self.first, &mut self.second, &mut self.third],
            element,
            compare_fn,
        );
    }

    fn push_utility(maxs: &mut [&mut Option<T>], element: T, compare_fn: impl Fn(&T, &T) -> bool) {
        if let [max, rest @ ..] = maxs {
            match Self::try_replace(*max, element.clone(), &compare_fn) {
                (Some(replaced), TripleReplacement::ReplacedMax) => {
                    Self::push_utility(rest, replaced, compare_fn);
                }
                (None, TripleReplacement::ReplacedNone) => {
                    // stop the recursion
                }
                (None, TripleReplacement::NotReplaced) => {
                    Self::push_utility(rest, element, compare_fn);
                }
                _ => panic!("Unexpected case"),
            }
        }
    }

    fn try_replace(
        max: &mut Option<T>,
        element: T,
        compare_fn: &impl Fn(&T, &T) -> bool,
    ) -> (Option<T>, TripleReplacement) {
        if max.is_none() || compare_fn(&element, max.as_ref().unwrap()) {
            let replaced = std::mem::replace(max, Some(element));
            let replacement_type = if replaced.is_some() {
                TripleReplacement::ReplacedMax
            } else {
                TripleReplacement::ReplacedNone
            };

            (replaced, replacement_type)
        } else {
            (None, TripleReplacement::NotReplaced)
        }
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MaxsTriple<T>
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    pub triple: Triple<T>,
}

impl<T> MaxsTriple<T>
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    fn push_max(&mut self, element: T) {
        self.triple.push(element, |a, b| a > b);
    }
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct MinsTriple<T>
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    pub triple: Triple<T>,
}

impl<T> MinsTriple<T>
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    fn push_min(&mut self, element: T) {
        self.triple.push(element, |a, b| a < b);
    }
}

pub fn sum<T>(mut container: impl ReadableContainer<T>) -> T
where
    T: std::fmt::Debug + Default + Clone + Add<T, Output = T>,
{
    let mut sum = T::default();

    while let Some(pivot) = container.pivot() {
        sum = sum + pivot.clone();
        container.pop();
    }

    sum
}

pub fn mean<T>(mut container: impl ReadableContainer<T>) -> Option<f64>
where
    T: std::fmt::Debug + Default + Clone + Add<T, Output = T> + Into<f64>,
{
    let mut sum = T::default();
    let mut size = 0usize;

    while let Some(pivot) = container.pivot() {
        sum = sum + pivot.clone();
        container.pop();
        size += 1;
    }

    let sum: f64 = sum.into();

    (size > 0).then_some(sum / size as f64)
}

pub fn evaluate_mins_and_maxs_triples<T>(
    mut container: impl ReadableContainer<T>,
) -> (MinsTriple<T>, MaxsTriple<T>)
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    let mut mins = MinsTriple::<T>::default();
    let mut maxs = MaxsTriple::<T>::default();

    while let Some(pivot) = container.pop() {
        mins.push_min(pivot.clone());
        maxs.push_max(pivot);
    }

    (mins, maxs)
}

pub fn central_element<T>(mut container: impl ReadableContainer<T>) -> Option<T>
where
    T: std::fmt::Debug + Default + Clone + Ord,
{
    let mut central_index = container.size() / 2;

    while central_index > 0 {
        let _ = container
            .pop()
            .expect("Container has an inconsistent size calculation algorithm.");
        central_index -= 1;
    }

    container.pop()
}
