use nutype::nutype;

#[cfg(test)]
mod sanitizers {
    use super::*;

    #[test]
    fn test_clamp() {
        #[nutype(sanitize(clamp(18, 99)))]
        struct Age(u8);

        assert_eq!(Age::new(17).into_inner(), 18);
        assert_eq!(Age::new(100).into_inner(), 99);
    }

    #[cfg(test)]
    mod with {
        use super::*;

        #[test]
        fn test_with_closure_with_explicit_type() {
            /// Some documentation for Cent goes here.
            #[nutype(sanitize(with = |n: i32| n.clamp(0, 100)))]
            pub struct Cent(i32);

            assert_eq!(Cent::new(-10).into_inner(), 0);
        }

        #[test]
        fn test_closure_with_no_type() {
            #[nutype(sanitize(with = |n| n.clamp(0, 100)))]
            pub struct Cent(i32);

            assert_eq!(Cent::new(-10).into_inner(), 0);
        }

        fn sanitize_cent(value: i32) -> i32 {
            value.clamp(0, 100)
        }

        #[test]
        fn test_with_function() {
            #[nutype(sanitize(with = sanitize_cent))]
            pub struct Cent(i32);

            assert_eq!(Cent::new(222).into_inner(), 100);
        }
    }

    #[test]
    fn test_from_trait() {
        #[nutype(sanitize(with = |a| a.clamp(18, 99)))]
        #[derive(*)]
        struct Age(u8);

        assert_eq!(Age::from(17).into_inner(), 18);
    }
}

#[cfg(test)]
mod validators {
    use super::*;

    #[test]
    fn test_min() {
        #[nutype(validate(min = 18))]
        #[derive(*)]
        struct Age(u8);

        assert_eq!(Age::new(17).unwrap_err(), AgeError::TooSmall);
        assert_eq!(Age::new(18).unwrap().into_inner(), 18);
    }

    #[test]
    fn test_max() {
        #[nutype(validate(max = 99))]
        #[derive(*)]
        struct Age(u8);

        assert_eq!(Age::new(100).unwrap_err(), AgeError::TooBig);
        assert_eq!(Age::new(99).unwrap().into_inner(), 99);
    }

    #[test]
    fn test_min_and_max() {
        #[nutype(validate(min = 18, max = 99))]
        #[derive(*)]
        struct Age(u8);

        assert_eq!(Age::new(17).unwrap_err(), AgeError::TooSmall);
        assert_eq!(Age::new(100).unwrap_err(), AgeError::TooBig);
        assert_eq!(Age::new(25).unwrap().into_inner(), 25);
    }

    #[cfg(test)]
    mod with {
        use super::*;

        #[test]
        fn test_with_closure_with_explicit_type() {
            #[nutype(validate(with = |c: &i32| (0..=100).contains(c) ))]
            #[derive(*)]
            pub struct Cent(i32);

            assert_eq!(Cent::new(-10), Err(CentError::Invalid));
            assert_eq!(Cent::new(101), Err(CentError::Invalid));
            assert_eq!(Cent::new(100).unwrap().into_inner(), 100);
        }

        #[test]
        fn test_closure_with_no_type() {
            #[nutype(validate(with = |c| (0..=100).contains(c) ))]
            #[derive(*)]
            pub struct Cent(i32);

            assert_eq!(Cent::new(-10), Err(CentError::Invalid));
            assert_eq!(Cent::new(101), Err(CentError::Invalid));
            assert_eq!(Cent::new(100).unwrap().into_inner(), 100);
        }

        fn is_cent_valid(val: &i32) -> bool {
            (0..=100).contains(val)
        }

        #[test]
        fn test_with_function() {
            #[nutype(validate(with = is_cent_valid))]
            #[derive(*)]
            pub struct Cent(i32);

            assert_eq!(Cent::new(-1), Err(CentError::Invalid));
            assert_eq!(Cent::new(101), Err(CentError::Invalid));
            assert_eq!(Cent::new(100).unwrap().into_inner(), 100);
        }
    }

    #[test]
    fn test_try_from_trait() {
        #[nutype(validate(min = 18))]
        #[derive(*)]
        struct Age(u8);

        assert_eq!(Age::try_from(17).unwrap_err(), AgeError::TooSmall);
        assert_eq!(Age::try_from(18).unwrap().into_inner(), 18);
    }
}

#[cfg(test)]
mod types {
    use super::*;

    #[test]
    fn test_u8_validate() {
        #[nutype(
            sanitize(clamp(0, 200))
            validate(min = 18, max = 99)
        )]
        #[derive(*)]
        struct Age(u8);

        assert_eq!(Age::new(17), Err(AgeError::TooSmall));
        assert_eq!(Age::new(100), Err(AgeError::TooBig));
        assert!(Age::new(20).is_ok());
    }

    #[test]
    fn test_u8_sanitize() {
        #[nutype(sanitize(clamp(10, 100)))]
        #[derive(*)]
        struct Percentage(u8);

        assert_eq!(Percentage::new(101), Percentage::new(100));
        assert_eq!(Percentage::new(9), Percentage::new(10));
    }

    #[test]
    fn test_u16() {
        #[nutype(validate(min = 18, max = 65000))]
        #[derive(*)]
        struct Age(u16);

        assert_eq!(Age::new(17), Err(AgeError::TooSmall));
        assert_eq!(Age::new(65001), Err(AgeError::TooBig));
        assert!(Age::new(20).is_ok());
    }

    #[test]
    fn test_u32() {
        #[nutype(validate(min = 1000, max = 100_000))]
        #[derive(*)]
        struct Amount(u32);

        assert_eq!(Amount::new(17), Err(AmountError::TooSmall));
        assert_eq!(Amount::new(100_001), Err(AmountError::TooBig));
        assert!(Amount::new(100_000).is_ok());
    }

    #[test]
    fn test_u64() {
        #[nutype(validate(min = 1000, max = 18446744073709551000))]
        #[derive(*)]
        struct Amount(u64);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(Amount::new(18446744073709551001), Err(AmountError::TooBig));
        assert!(Amount::new(1000).is_ok());
    }

    #[test]
    fn test_u128() {
        #[nutype(validate(min = 1000, max = 170141183460469231731687303715884105828))]
        #[derive(*)]
        struct Amount(u128);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(
            Amount::new(170141183460469231731687303715884105829),
            Err(AmountError::TooBig)
        );
        assert!(Amount::new(1000).is_ok());
        assert!(Amount::new(170141183460469231731687303715884105828).is_ok());
    }

    #[test]
    fn test_i8_sanitize() {
        #[nutype(sanitize(clamp(0, 100)))]
        #[derive(*)]
        struct Percentage(i8);

        assert_eq!(Percentage::new(101), Percentage::new(100));
        assert_eq!(Percentage::new(-1), Percentage::new(0));
    }

    #[test]
    fn test_i8_validate() {
        // TODO: use negative numbers
        #[nutype(validate(min = 18, max = 99))]
        #[derive(*)]
        struct Age(i8);

        assert_eq!(Age::new(17), Err(AgeError::TooSmall));
        assert_eq!(Age::new(100), Err(AgeError::TooBig));
        assert!(Age::new(20).is_ok());
    }

    #[test]
    fn test_i16_validate() {
        #[nutype(validate(min = 1000, max = 32_000))]
        #[derive(*)]
        struct Amount(i16);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(Amount::new(32_001), Err(AmountError::TooBig));
        assert!(Amount::new(1000).is_ok());
        assert!(Amount::new(32_000).is_ok());
    }

    #[test]
    fn test_i32_validate() {
        #[nutype(validate(min = 1000, max = 320_000))]
        #[derive(*)]
        struct Amount(i32);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(Amount::new(320_001), Err(AmountError::TooBig));
        assert!(Amount::new(1000).is_ok());
        assert!(Amount::new(320_000).is_ok());

        let amount = Amount::new(2055).unwrap();
        assert_eq!(amount.into_inner(), 2055);
    }

    #[test]
    fn test_i32_negative() {
        #[nutype(
            sanitize(clamp(-200, -5))
            validate(min = -100, max = -50)
        )]
        #[derive(*)]
        pub struct Balance(i32);

        assert_eq!(Balance::new(-300), Err(BalanceError::TooSmall));
        assert_eq!(Balance::new(-4), Err(BalanceError::TooBig));

        let balance = Balance::new(-55).unwrap();
        assert_eq!(balance.into_inner(), -55);
    }

    #[test]
    fn test_i64_validate() {
        #[nutype(validate(min = 1000, max = 8446744073709551000))]
        #[derive(Debug, PartialEq)]
        struct Amount(i64);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(Amount::new(8446744073709551001), Err(AmountError::TooBig));
        assert!(Amount::new(1000).is_ok());
        assert!(Amount::new(8446744073709551000).is_ok());
    }

    #[test]
    fn test_i128_validate() {
        #[nutype(validate(min = 1000, max = 70141183460469231731687303715884105000))]
        #[derive(Debug, PartialEq)]
        struct Amount(i128);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(
            Amount::new(70141183460469231731687303715884105001),
            Err(AmountError::TooBig)
        );
        assert!(Amount::new(1000).is_ok());
        assert!(Amount::new(70141183460469231731687303715884105000).is_ok());
    }

    #[test]
    fn test_usize_validate() {
        #[nutype(validate(min = 1000, max = 2000))]
        #[derive(Debug, PartialEq)]
        struct Amount(usize);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(Amount::new(2001), Err(AmountError::TooBig));
        assert!(Amount::new(1000).is_ok());
        assert!(Amount::new(2000).is_ok());
    }

    #[test]
    fn test_isize_validate() {
        #[nutype(validate(min = 1000, max = 2000))]
        #[derive(Debug, PartialEq)]
        struct Amount(isize);

        assert_eq!(Amount::new(999), Err(AmountError::TooSmall));
        assert_eq!(Amount::new(2001), Err(AmountError::TooBig));
        assert!(Amount::new(1000).is_ok());
        assert!(Amount::new(2000).is_ok());
    }
}

#[cfg(test)]
mod visibility {
    mod encapsulated {
        use nutype::nutype;

        #[nutype(sanitize(with = |n: i32| n.clamp(0, 100)))]
        pub struct Percentage(i32);
    }

    #[test]
    fn test_public_visibility() {
        let percentage = encapsulated::Percentage::new(133);
        assert_eq!(percentage.into_inner(), 100);
    }
}

#[cfg(test)]
mod traits {
    use super::*;
    // use nutype_test_suite::test_helpers::traits::*;

    #[test]
    fn test_trait_into() {
        #[nutype]
        #[derive(Into)]
        pub struct Age(u8);

        let age = Age::new(32);
        let age: u8 = age.into();
        assert_eq!(age, 32);
    }

    #[test]
    fn test_trait_from() {
        #[nutype]
        #[derive(From)]
        pub struct Amount(u32);

        let amount = Amount::from(350);
        assert_eq!(amount.into_inner(), 350);
    }

    #[test]
    fn test_trait_as_ref() {
        #[nutype]
        #[derive(AsRef)]
        pub struct Age(u8);

        let age = Age::new(32);
        let age_ref: &u8 = age.as_ref();
        assert_eq!(age_ref, &32);
    }

    #[test]
    fn test_trait_borrow() {
        use std::borrow::Borrow;

        #[nutype]
        #[derive(Borrow)]
        pub struct Age(u8);

        let age = Age::new(32);
        let age_borrowed: &u8 = age.borrow();
        assert_eq!(age_borrowed, &32);
    }

    #[test]
    fn test_trait_try_from() {
        #[nutype(validate(max = 1000))]
        #[derive(Debug, TryFrom)]
        pub struct Amount(i64);

        let amount = Amount::try_from(1000).unwrap();
        assert_eq!(amount.into_inner(), 1000);

        let error = Amount::try_from(1001).unwrap_err();
        assert_eq!(error, AmountError::TooBig);
    }
}
