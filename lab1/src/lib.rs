#![feature(test)]

pub mod algorithms;
pub mod containers;

#[cfg(test)]
const BIG_SIZE: usize = 1_000_000;

#[cfg(test)]
mod tests {
    use super::*;
    use crate::algorithms::*;
    use common::containers::traits::{FillableContainer, ReadableContainer};
    use containers::*;

    type Number = i32;

    fn test_sum<Container>(numbers: &[Number])
    where
        Container: Default + FillableContainer<Number> + ReadableContainer<Number>,
    {
        let mut container = Container::default();

        for number in numbers {
            container.push(*number);
        }

        assert_eq!(sum(container), numbers.iter().sum())
    }

    fn test_mean<Container>(numbers: &[Number])
    where
        Container: Default + FillableContainer<Number> + ReadableContainer<Number>,
    {
        let mut container = Container::default();

        for number in numbers {
            container.push(*number);
        }

        let mean = mean(container);

        if numbers.len() == 0 && mean.is_none() {
            return;
        } else if numbers.len() > 0 && mean.is_none() {
            panic!("Mean must not be none for non-empty number input!");
        }

        let sum = numbers.iter().sum::<Number>() as f64;

        assert_eq!(mean.unwrap(), sum / numbers.len() as f64);
    }

    fn test_mins_and_maxs_evaluation<Container>(
        numbers: &[Number],
        expected_result: (MinsTriple<Number>, MaxsTriple<Number>),
    ) where
        Container: Default + FillableContainer<Number> + ReadableContainer<Number>,
    {
        let mut container = Container::default();

        for number in numbers {
            container.push(*number);
        }

        let (expected_mins, expected_maxs) = expected_result;
        let (mins, maxs) = evaluate_mins_and_maxs_triples(container);
        assert_eq!(mins, expected_mins);
        assert_eq!(maxs, expected_maxs);
    }

    fn test_central_element<Container>(numbers: &[Number])
    where
        Container: Default + FillableContainer<Number> + ReadableContainer<Number>,
    {
        let mut container = Container::default();

        for number in numbers {
            container.push(*number);
        }

        let central_element = central_element(container);

        if numbers.len() == 0 && central_element.is_none() {
            return;
        } else if numbers.len() > 0 && central_element.is_none() {
            panic!("Central element must not be none for non-empty number input!");
        }

        assert_eq!(
            central_element.unwrap(),
            *numbers.iter().nth(numbers.len() / 2).unwrap()
        );
    }

    #[test]
    fn list_based_queue() {
        test_sum::<ListBasedQueue<Number>>(&[]);
        test_sum::<ListBasedQueue<Number>>(&[2; BIG_SIZE]);
        test_mean::<ListBasedQueue<Number>>(&[]);
        test_mean::<ListBasedQueue<Number>>(&[2; BIG_SIZE]);

        let big_array: Vec<Number> = [
            &[2; BIG_SIZE / 2][..],
            &[1][..], // 1 at the center
            &[2; BIG_SIZE / 2][..],
        ]
            .concat();

        test_central_element::<ListBasedQueue<Number>>(&big_array);

        let big_array: Vec<Number> = [
            &[9, -3][..],
            &[2; BIG_SIZE / 2][..],
            &[8, -2][..],
            &[2; BIG_SIZE / 2][..],
            &[7, -1][..],
        ]
            .concat();

        test_mins_and_maxs_evaluation::<ListBasedQueue<Number>>(
            &big_array,
            (
                MinsTriple {
                    triple: Triple {
                        first: Some(-3),
                        second: Some(-2),
                        third: Some(-1),
                    },
                },
                MaxsTriple {
                    triple: Triple {
                        first: Some(9),
                        second: Some(8),
                        third: Some(7),
                    },
                },
            ),
        );
    }

    #[test]
    fn list_based_stack() {
        test_sum::<ListBasedStack<Number>>(&[]);
        test_sum::<ListBasedStack<Number>>(&[2; BIG_SIZE]);
        test_mean::<ListBasedStack<Number>>(&[]);
        test_mean::<ListBasedStack<Number>>(&[2; BIG_SIZE]);

        let big_array: Vec<Number> = [
            &[2; BIG_SIZE / 2][..],
            &[1][..], // 1 at the center
            &[2; BIG_SIZE / 2][..],
        ]
            .concat();

        test_central_element::<ListBasedStack<Number>>(&big_array);

        let big_array: Vec<Number> = [
            &[9, -3][..],
            &[2; BIG_SIZE / 2][..],
            &[8, -2][..],
            &[2; BIG_SIZE / 2][..],
            &[7, -1][..],
        ]
            .concat();

        test_mins_and_maxs_evaluation::<ListBasedStack<Number>>(
            &big_array,
            (
                MinsTriple {
                    triple: Triple {
                        first: Some(-3),
                        second: Some(-2),
                        third: Some(-1),
                    },
                },
                MaxsTriple {
                    triple: Triple {
                        first: Some(9),
                        second: Some(8),
                        third: Some(7),
                    },
                },
            ),
        );
    }

    #[test]
    fn array_based_queue() {
        test_sum::<ArrayBasedQueue<Number>>(&[]);
        test_sum::<ArrayBasedQueue<Number>>(&[2; BIG_SIZE]);
        test_mean::<ArrayBasedQueue<Number>>(&[]);
        test_mean::<ArrayBasedQueue<Number>>(&[2; BIG_SIZE]);

        let big_array: Vec<Number> = [
            &[2; BIG_SIZE / 2][..],
            &[1][..], // 1 at the center
            &[2; BIG_SIZE / 2][..],
        ]
            .concat();

        test_central_element::<ArrayBasedQueue<Number>>(&big_array);

        let big_array: Vec<Number> = [
            &[9, -3][..],
            &[2; BIG_SIZE / 2][..],
            &[8, -2][..],
            &[2; BIG_SIZE / 2][..],
            &[7, -1][..],
        ]
            .concat();

        test_mins_and_maxs_evaluation::<ArrayBasedQueue<Number>>(
            &big_array,
            (
                MinsTriple {
                    triple: Triple {
                        first: Some(-3),
                        second: Some(-2),
                        third: Some(-1),
                    },
                },
                MaxsTriple {
                    triple: Triple {
                        first: Some(9),
                        second: Some(8),
                        third: Some(7),
                    },
                },
            ),
        );
    }

    #[test]
    fn array_based_stack() {
        test_sum::<ArrayBasedStack<Number>>(&[]);
        test_sum::<ArrayBasedStack<Number>>(&[2; BIG_SIZE]);
        test_mean::<ArrayBasedStack<Number>>(&[]);
        test_mean::<ArrayBasedStack<Number>>(&[2; BIG_SIZE]);

        let big_array: Vec<Number> = [
            &[2; BIG_SIZE / 2][..],
            &[1][..], // 1 at the center
            &[2; BIG_SIZE / 2][..],
        ]
            .concat();

        test_central_element::<ArrayBasedStack<Number>>(&big_array);

        let big_array: Vec<Number> = [
            &[9, -3][..],
            &[2; BIG_SIZE / 2][..],
            &[8, -2][..],
            &[2; BIG_SIZE / 2][..],
            &[7, -1][..],
        ]
            .concat();

        test_mins_and_maxs_evaluation::<ArrayBasedStack<Number>>(
            &big_array,
            (
                MinsTriple {
                    triple: Triple {
                        first: Some(-3),
                        second: Some(-2),
                        third: Some(-1),
                    },
                },
                MaxsTriple {
                    triple: Triple {
                        first: Some(9),
                        second: Some(8),
                        third: Some(7),
                    },
                },
            ),
        );
    }
}

