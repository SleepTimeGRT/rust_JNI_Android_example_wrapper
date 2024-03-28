use std::str::FromStr;
use num_traits::Num;
use shamir_secret_sharing::num_bigint::BigInt;
use shamir_secret_sharing::ShamirSecretSharing as SSS;

fn parse_input(input: &str) -> Result<(usize, BigInt), String> {
    let mut parts = input.split('+');

    // 첫 번째 부분을 usize로 변환
    let index = match parts.next() {
        Some(s) => usize::from_str(s).map_err(|e| format!("{}은 유효한 인덱스가 아닙니다: {}", e, s))?,
        None => return Err("입력 문자열에 '+' 기호가 없습니다.".to_string()),
    };

    // 두 번째 부분을 BigInt로 변환
    let bigint = match parts.next() {
        Some(s) => <BigInt as Num>::from_str_radix(s, 16).map_err(|e| format!("{}은 유효한 숫자가 아닙니다: {}", e, s))?,
        None => return Err("입력 문자열에 숫자가 없습니다.".to_string()),
    };

    // 모든 검사를 통과하면 결과 반환
    Ok((index, bigint))
}

fn transform_vec(input_vec: Vec<String>) -> Vec<(usize, BigInt)> {
    input_vec
        .into_iter()
        .map(|value| parse_input(&value).unwrap())
        .collect()   // Collects them into a new Vec
}

pub fn split(threshold: usize, share_amount: usize, secret: BigInt) -> Vec<String> {
    let sss = SSS {
        threshold,
        share_amount,
        prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap(),
    };

    sss.split(secret.clone()).into_iter().map(|(x, y)| format!("{}+{}", x, y.to_str_radix(16))).collect()
}

pub fn recover(threshold: usize, share_amount: usize, shares: &[String]) -> BigInt {
    let sss = SSS {
        threshold,
        share_amount,
        prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap(),
    };

    let x: Vec<(usize, BigInt)> = transform_vec(shares.to_vec());
    sss.recover(x[0..threshold].to_owned().as_slice())
}

#[cfg(test)]
mod sss_tests {
    use super::*;

    mod sss_main_tests {
        use crate::lib_sss;
        use super::*;

        #[test]
        fn split_and_recover_test() {
            println!("----------------------------");
            println!("split_and_recover_test");
            println!("----------------------------");
            let sss = SSS {
                threshold: 2,
                share_amount: 3,
                prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap(),
            };

            let secret = BigInt::from(0x1234);
            println!("{}", secret.to_string());
            let shares = sss.split(secret.clone());
            println!("split {:?}", shares);
            let recover1 = sss.recover(&shares[0..sss.threshold]);
            let recover2 = sss.recover(&vec![shares[0].clone(), shares[2].clone()]);
            let recover3 = sss.recover(&vec![shares[1].clone(), shares[2].clone()]);
            println!("1:{}, 2:{}, 3:{}", recover1.to_string(), recover2.to_string(), recover3.to_string());
            assert_eq!(secret, recover1);
            assert_eq!(recover2, recover1);
            assert_eq!(recover3, recover1);
        }

        #[test]
        fn split_and_recover_3_test() {
            println!("----------------------------");
            println!("split_and_recover_3_test");
            println!("----------------------------");
            let sss = SSS {
                threshold: 2,
                share_amount: 3,
                prime: BigInt::parse_bytes(b"fffffffffffffffffffffffffffffffffffffffffffffffffffffffefffffc2f", 16).unwrap(),
            };

            let secret = BigInt::from(0x1234);
            println!("{}", secret.to_string());
            let shares = sss.split(secret.clone());
            println!("split {:?}", shares);

            let recover1 = sss.recover(&shares[0..sss.threshold]);
            let recover2 = sss.recover(&vec![shares[0].clone(), shares[2].clone()]);
            let recover3 = sss.recover(&vec![shares[1].clone(), shares[2].clone()]);
            println!("1:{}, 2:{}, 3:{}", recover1.to_string(), recover2.to_string(), recover3.to_string());
            assert_eq!(secret, recover1);
            assert_eq!(secret, recover2);
            assert_eq!(secret, recover3);
        }

        #[test]
        fn split_and_recover_2_test() {
            println!("----------------------------");
            println!("split_and_recover_2_test");
            println!("----------------------------");
            let secret = BigInt::from(0x1234);
            println!("{}", secret.to_string());
            let shares = split(2, 3, secret.clone());
            println!("split {:?}", shares);
            let recover1 = recover(2, 3, &shares[0..2]);
            let recover2 = recover(2, 3, &vec![shares[0].clone(), shares[2].clone()]);
            let recover3 = recover(2, 3, &vec![shares[1].clone(), shares[2].clone()]);
            println!("1:{}, 2:{}, 3:{}", recover1.to_string(), recover2.to_string(), recover3.to_string());
            assert_eq!(secret, recover1);
            assert_eq!(recover2, recover1);
            assert_eq!(recover3, recover1);
        }

        #[test]
        fn test2() {
            use num_traits::Num;

            let result = <BigInt as Num>::from_str_radix("27", 10);
            assert_eq!(result, Ok(BigInt::from(27)));

            let result = <BigInt as Num>::from_str_radix("foo", 10);
            assert!(result.is_err());
        }

        #[test]
        fn test3(){
            let rust_shares = vec![
                "1+9b276953e972d8421676e55ddea53ec37f687dc23443117adc65ac35be1f748d".to_owned(),
                "2+364ed2a7d2e5b0842cedcabbbd4a7d86fed0fb84688622f5b8cb586c7c3edab7".to_owned(),
                "3+d1763bfbbc5888c64364b0199befbc4a7e3979469cc93470953104a23a5e3d10".to_owned()
            ];

            let secret = BigInt::from(0x1234);
            let recover = recover(2, 3, rust_shares.as_slice());
            assert_eq!(secret, recover)
        }
    }
}
