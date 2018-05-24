#[macro_use]
extern crate itertools;

type QuadVec<'a> = (&'a [i32], &'a [i32], &'a [i32], &'a [i32]);

fn run((v1, v2, v3, v4): &QuadVec) -> u32 {
    izip!(v1.iter(), v2.iter(), v3.iter(), v4.iter())
        .map(|(a, b, c, d)| a + b + c + d)
        .filter(|sum| *sum == 0)
        .count() as u32
}

#[cfg(test)]
mod tests {
    use super::run;
    use super::QuadVec;

    struct TestCase<'a> {
        input: QuadVec<'a>,
        expected: u32,
    }

    static TESTS: &[TestCase] = &[
        TestCase {
            input: (&[], &[], &[], &[]),
            expected: 0,
        },
        TestCase {
            input: (&[1], &[0], &[0], &[0]),
            expected: 0,
        },
        TestCase {
            input: (&[1], &[-1], &[0], &[0]),
            expected: 1,
        },
        TestCase {
            input: (&[1, 1, 0, 1], &[0, 0, 2, 0], &[0, 0, -2, 0], &[-1, 0, 0, 0]),
            expected: 2,
        },
    ];

    #[test]
    fn it_works() {
        TESTS.iter().for_each(|test_case| run_test(test_case));
    }

    fn run_test(test_case: &TestCase) {
        let actual = run(&test_case.input);
        assert_eq!(
            actual, test_case.expected,
            "Test failed for input: {:?}",
            &test_case.input
        );
    }
}
