#[cfg(test)]
mod tests {
    use crate::{polynomial, Polynomial};

    #[test]
    fn degree() {
        assert_eq!(
            polynomial! { 200 => 0.0, 100 => 1.0, 0 => 5.0 }.degree(),
            Some(100)
        );
        assert_eq!(
            polynomial! { 1 => 1.0, 2 => 5.0, 0 => 5.0, 3 => -2.0, 4 => -1.0, 5 => 1.0 }.degree(),
            Some(5)
        );
        assert_eq!(
            polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 }.degree(),
            Some(3)
        );
        assert_eq!(polynomial! { 1 => 10.0, 0 => 15.0 }.degree(), Some(1));
        assert_eq!(polynomial! { 0 => 15.0 }.degree(), Some(0));
        assert_eq!(Polynomial::new().degree(), None);
    }

    #[test]
    fn at() {
        let p = polynomial! { 1 => 1.0, 2 => 5.0, 0 => 5.0, 3 => -2.0, 4 => -1.0, 5 => 1.0 };
        assert_eq!(p.at(3.0), 161.0);
    }

    #[test]
    fn plot() {
        let p = polynomial! { 4 => 0.0, 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 };
        let q = polynomial! { 5 => 0.0, 2 => -5.0, 1 => -1.0, 0 => 30.0 };
        let r = polynomial! { 6 => 0.0, 1 => -100.0, 0 => 30.0 };
        assert_eq!(
            Polynomial::plot(&[&p, &q, &r], -13.0, 5.0, 50, "plot_test"),
            Ok(())
        );
        assert_eq!(
            Polynomial::plot(&[&p, &q, &r], -13.0, 5.0, 1, "should_not_exist"),
            Err("Requested less than 2 samples for plotting.")
        );
    }

    #[test]
    #[should_panic]
    fn plot_in_non_exisiting_dir() {
        let p = polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 };
        assert_eq!(
            Polynomial::plot(&[&p], -13.0, 5.0, 50, "foobar/plot_test"),
            Ok(())
        );
    }

    #[test]
    fn derivative() {
        assert_eq!(
            polynomial! { 1 => 0.0, 0 => 15.0 }.derivative(),
            Polynomial::new()
        );
        assert_eq!(
            polynomial! { 2 => 0.0, 1 => 10.0, 0 => 15.0 }.derivative(),
            polynomial! { 0 => 10.0 }
        );
        assert_eq!(
            polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 }.derivative(),
            polynomial! { 2 => -3.0, 1 => -20.0, 0 => 10.0 }
        );
    }

    #[test]
    fn integral() {
        assert_eq!(Polynomial::new().integral(-5.0), polynomial! { 0 => -5.0 });
        assert_eq!(
            polynomial! { 2 => 0.0, 0 => 10.0 }.integral(15.0),
            polynomial! { 2 => 0.0, 1 => 10.0, 0 => 15.0 },
        );
        assert_eq!(
            polynomial! { 2 => -3.0, 1 => -20.0, 0 => 10.0 }.integral(15.0),
            polynomial! { 3 => -1.0, 2 => -10.0, 1 => 10.0, 0 => 15.0 },
        );
    }

    #[test]
    fn reflect_about_y_axis() {
        assert_eq!(Polynomial::new().reflect_about_y_axis(), Polynomial::new());
        assert_eq!(
            polynomial! { 0 => 10.0 }.reflect_about_y_axis(),
            polynomial! { 0 => 10.0 },
        );
        assert_eq!(
            polynomial! { 3 => 2.0, 2 => -3.0, 1 => -17.0, 0 => 6.0 }.reflect_about_y_axis(),
            polynomial! { 3 => -2.0, 2 => -3.0, 1 => 17.0, 0 => 6.0 },
        );
    }

    #[test]
    fn real_roots() {
        assert_eq!(Polynomial::new().real_roots(0.001), vec![]);
        assert_eq!(
            polynomial! {7 => 0.0, 1 => 0.0, 0 => 0.0}.real_roots(0.001),
            vec![]
        );

        assert_eq!(polynomial! {0 => 1.0}.real_roots(0.001), vec![]);
        assert_eq!(polynomial! {0 => 7.167}.real_roots(0.001), vec![]);

        assert_eq!(polynomial! {1 => 1.0}.real_roots(0.001), vec![0.0]);
        assert_eq!(polynomial! {100 => 1.0}.real_roots(0.001), vec![0.0]);

        assert_eq!(polynomial! {2 => 1.0, 0 => 1.0}.real_roots(0.001), vec![]);

        println!(
            "{:?}",
            polynomial! {2 => 1.0, 1 => -4.0, 0 => 4.0}.real_roots(0.001)
        );

        println!(
            "{:?}",
            polynomial! {3 => 1.0, 2 => -6.0, 1 => 12.0, 0 => -8.0}.real_roots(0.001)
        );

        println!("{:?}", polynomial! {1 => 1.0, 0 => -1.0}.real_roots(0.001));
        println!("{:?}", polynomial! {1 => 1.0, 0 => 1.0}.real_roots(0.001));

        println!("{:?}", polynomial! {2 => 1.0, 1 => -1.0}.real_roots(0.001));
        println!("{:?}", polynomial! {2 => 1.0, 1 => 1.0}.real_roots(0.001));

        println!("{:?}", polynomial! {3 => 1.0, 1 => -1.0}.real_roots(0.001));
        println!("{:?}", polynomial! {5 => 1.0, 3 => -1.0}.real_roots(0.001));

        println!(
            "{:?}",
            polynomial! {2 => 1.0, 1 => -5.0, 0 => 6.0}.real_roots(0.001)
        );
        println!(
            "{:?}",
            polynomial! {4 => 1.0, 3 => -10.0, 2 => 35.0, 1 => -50.0, 0 => 24.0}.real_roots(0.001)
        );
        println!(
            "{:?}",
            polynomial! {4 => 1.0, 3 => -22.0, 2 => 152.0, 1 => -362.0, 0 => 231.0}
                .real_roots(0.001)
        );
        println!(
            "{:?}",
            polynomial! {2 => 1.0, 1 => -1100.0, 0 => 100000.0}.real_roots(0.1)
        );
    }

    #[test]
    fn ignore_zero_coeff() {
        assert_eq!(
            polynomial! { 4 => 0.0, 3 => 0.0, 2 => 0.0, 1 => 0.0 },
            Polynomial::new(),
        );
        assert_eq!(
            Polynomial::new(),
            polynomial! { 4 => 0.0, 3 => 0.0, 2 => 0.0, 1 => 0.0 },
        );
        assert_eq!(
            polynomial! { 4 => 1.0, 2 => -3.0},
            polynomial! { 4 => 1.0, 3 => 0.0, 2 => -3.0, 1 => 0.0 },
        );
        assert_eq!(
            polynomial! { 4 => 1.0, 3 => 0.0, 2 => -3.0, 1 => 0.0 },
            polynomial! { 4 => 1.0, 2 => -3.0},
        );
    }

    #[test]
    fn add() {
        let p = polynomial! { 5 => 1.0, 1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 7 => 0.0, 5 => -1.0, 3 => 73.0, 2 => -118.0, 0 => 40.0 };
        assert_eq!(
            p + q,
            polynomial! { 3 => 73.0, 2 => -61.0, 1 => 11.0, 0 => 91.0 }
        );
    }

    #[test]
    fn add_assign() {
        let mut p = polynomial! { 8 => 0.0, 6 => 1.1, 1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 6 => -1.1, 3 => 73.0, 2 => -118.0, 0 => 40.0 };
        p += q;
        assert_eq!(
            p,
            polynomial! { 3 => 73.0, 2 => -61.0, 1 => 11.0, 0 => 91.0 }
        );
    }

    #[test]
    fn sub() {
        let p = polynomial! { 5 => 300.0, 1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 5 => 300.0, 3 => 73.0, 2 => -118.0, 0 => 40.0 };
        assert_eq!(
            p - q,
            polynomial! { 3 => -73.0, 2 => 175.0, 1 => 11.0, 0 => 11.0 }
        );
    }

    #[test]
    fn sub_assign() {
        let mut p = polynomial! { 5 => 300.0,  1 => 11.0, 2 => 57.0, 0 => 51.0 };
        let q = polynomial! { 5 => 300.0,  3 => 73.0, 2 => -118.0, 0 => 40.0 };
        p -= q;
        assert_eq!(
            p,
            polynomial! { 3 => -73.0, 2 => 175.0, 1 => 11.0, 0 => 11.0 }
        );
    }

    #[test]
    fn mul() {
        let p = polynomial! { 6 => 0.0, 1 => 1.0, 2 => 5.0, 0 => 5.0 };
        let q = polynomial! { 5 => 0.0, 3 => 7.0, 2 => -8.0, 0 => 4.0 };
        assert_eq!(
            p * q,
            polynomial! { 5 => 35.0, 4 => -33.0, 3 => 27.0, 2 => -20.0, 1 => 4.0, 0 => 20.0 }
        );
    }

    #[test]
    fn div() {
        let p = Polynomial::new();
        let q = polynomial! { 1 => 1.0, 0 => -2.0 };
        assert_eq!(p / q, Polynomial::new());
        let p = polynomial! { 2 => 1.0, 1 => -5.0, 0 => 6.0 };
        let q = polynomial! { 6 => 0.0, 1 => 1.0, 0 => -2.0 };
        assert_eq!(p / q, polynomial! { 1 => 1.0, 0 => -3.0});
        let p = polynomial! { 6 => 0.0, 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = polynomial! { 1 => 1.0, 0 => 3.0 };
        assert_eq!(p / q, polynomial! { 2 => 2.0, 1 => -11.0, 0 => 32.0});
        let p = polynomial! { 4 => 6.0, 3 => 5.0, 1 => 4.0, 0 => -4.0 };
        let q = polynomial! { 6 => 0.0, 2 => 2.0, 1 => 1.0, 0 => -1.0 };
        assert_eq!(p / q, polynomial! { 2 => 3.0, 1 => 1.0, 0 => 1.0});
    }

    #[test]
    #[should_panic]
    fn div_with_zero_polynomial1() {
        let p = Polynomial::new();
        let q = Polynomial::new();
        let _ = p / q;
    }

    #[test]
    #[should_panic]
    fn div_with_zero_polynomial2() {
        let p = polynomial! { 6 => 0.0, 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = Polynomial::new();
        let _ = p / q;
    }

    #[test]
    fn rem() {
        let p = Polynomial::new();
        let q = polynomial! { 1 => 1.0, 0 => -2.0 };
        assert_eq!(p % q, Polynomial::new());
        let p = polynomial! { 2 => 1.0, 1 => -5.0, 0 => 6.0 };
        let q = polynomial! { 6 => 0.0, 1 => 1.0, 0 => -2.0 };
        assert_eq!(p % q, Polynomial::new());
        let p = polynomial! { 6 => 0.0, 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = polynomial! { 1 => 1.0, 0 => 3.0 };
        assert_eq!(p % q, polynomial! { 0 => -93.0});
        let p = polynomial! { 4 => 6.0, 3 => 5.0, 1 => 4.0, 0 => -4.0 };
        let q = polynomial! { 6 => 0.0, 2 => 2.0, 1 => 1.0, 0 => -1.0 };
        assert_eq!(p % q, polynomial! { 1 => 4.0, 0 => -3.0 });
    }

    #[test]
    #[should_panic]
    fn rem_with_zero_polynomial1() {
        let p = Polynomial::new();
        let q = Polynomial::new();
        let _ = p % q;
    }

    #[test]
    #[should_panic]
    fn rem_with_zero_polynomial2() {
        let p = polynomial! { 6 => 0.0, 3 => 2.0, 2 => -5.0, 1 => -1.0, 0 => 3.0 };
        let q = Polynomial::new();
        let _ = p % q;
    }
}
