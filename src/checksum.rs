/// Implements the [Luhn algorithm][wiki], a simple checksum formula used to validate a variety of
/// identification numbers.
///
/// # Examples
///
/// Provide the function with an identification number.
///
/// ```
/// let acct_num = 79927398713;
/// let checksum = checksum::luhn(acct_num);
/// assert_eq!(checksum, true);
/// if checksum {
///     println!("Account number is valid!");
/// } else {
///     println!("Account number is NOT valid!");
/// }
/// ```
///
/// [wiki]: https://en.wikipedia.org/wiki/Luhn_algorithm
pub fn luhn(num: u64) -> bool {
    let mut num_pre_div = num;
    let mut num_post_div;
    let mut sum = 0u32;
    let mut i = 0u8;
    let mut assumed_check_digit = 0;
    // Scan digits from right to left
    loop {
        num_post_div = num_pre_div / 10;
        i += 1;
        let digit = num_pre_div - num_post_div * 10;
        // Sum all non-check digits
        if i != 1 {
            if i % 2 == 0 {
                let mut second_digit = digit * 2;
                if second_digit > 9 {
                    second_digit = second_digit - 9;
                }
                sum = sum + second_digit as u32;
            } else {
                sum = sum + digit as u32;
            }
        } else {
            assumed_check_digit = digit;
        }
        if num_post_div == 0 {
            break;
        }
        num_pre_div = num_post_div;
    }
    let check_digit = sum * 9 % 10;
    check_digit == assumed_check_digit as u32
}

const DIHEDRAL_D5: [[u8; 10]; 10] = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                                     [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
                                     [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
                                     [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
                                     [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
                                     [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
                                     [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
                                     [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
                                     [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
                                     [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]];

const INV_DIHEDRAL_D5: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

const VERHOEFF_P_TABLE: [[u8; 10]; 8] = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                                         [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
                                         [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
                                         [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
                                         [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
                                         [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
                                         [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
                                         [7, 0, 4, 6, 9, 1, 3, 2, 5, 8]];

/// Implements the [Verhoeff algorithm][wiki] for generating a number's check digit.
///
/// # Examples
///
/// Generate a check digit for 236:
///
/// ```
/// let num = 236;
/// let check_digit = checksum::verhoeff_generate_check_digit(num);
/// assert_eq!(check_digit, 3 as u8);
/// ```
///
/// [wiki]: https://en.wikipedia.org/wiki/Verhoeff_algorithm
pub fn verhoeff_generate_check_digit(num: u64) -> u8 {
    let mut num_pre_div = num * 10; // Append zero
    let mut num_post_div;
    let mut i = 0u8;
    let mut c = 0u8;
    loop {
        num_post_div = num_pre_div / 10;
        let digit = num_pre_div - num_post_div * 10;
        c = DIHEDRAL_D5[c as usize][VERHOEFF_P_TABLE[(i % 8) as usize][digit as usize] as usize];
        i += 1;
        if num_post_div == 0 {
            break;
        }
        num_pre_div = num_post_div;
    }
    INV_DIHEDRAL_D5[c as usize]
}

/// Implements the [Verhoeff algorithm][wiki], a checksum formula for error detection.
///
/// # Examples
///
/// Validate the check digit 2363.
///
/// ```
/// let num = 2363;
/// let is_valid = checksum::verhoeff_validate_check_digit(num);
/// assert_eq!(is_valid, true);
/// ```
///
/// [wiki]: https://en.wikipedia.org/wiki/Verhoeff_algorithm
pub fn verhoeff_validate_check_digit(num: u64) -> bool {
    let mut num_pre_div = num;
    let mut num_post_div;
    let mut i = 0u8;
    let mut c = 0u8;
    loop {
        num_post_div = num_pre_div / 10;
        let digit = num_pre_div - num_post_div * 10;
        c = DIHEDRAL_D5[c as usize][VERHOEFF_P_TABLE[(i % 8) as usize][digit as usize] as usize];
        i += 1;
        if num_post_div == 0 {
            break;
        }
        num_pre_div = num_post_div;
    }
    c == 0
}