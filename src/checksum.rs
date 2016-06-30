/// Computes the Luhn checksum for the provided number.
///
/// # Examples
///
/// Provide the function with an identification number.
///
/// ```
/// let acct_num = 79927398713;
/// let checksum = checksum::luhn_checksum(acct_num);
/// assert_eq!(checksum, 0);
/// if checksum != 0 {
///     println!("Account number has been corrupted!");
/// }
/// ```
pub fn luhn_checksum(full_number: u64) -> u8 {
    let mut num_pre_div = full_number;
    let mut num_post_div;
    let mut sum = 0u32;
    let mut i = 0u8;
    // Scan digits from right to left
    loop {
        num_post_div = num_pre_div / 10;
        i += 1;
        let digit = num_pre_div - num_post_div * 10;
        if i % 2 == 0 {
            let mut second_digit = digit * 2;
            if second_digit > 9 {
                second_digit = second_digit - 9;
            }
            sum = sum + second_digit as u32;
        } else {
            sum = sum + digit as u32;
        }
        if num_post_div == 0 {
            break;
        }
        num_pre_div = num_post_div;
    }
    (sum % 10) as u8
}

/// Computes the Luhn check digit for the provided number.
///
/// # Examples
///
/// Provide the function with an identification number.
///
/// ```
/// let acct_num = 7992739871;
/// let checksum = checksum::luhn_calculate_check_digit(acct_num);
/// assert_eq!(checksum, 3);
/// ```
pub fn luhn_calculate_check_digit(num: u64) -> u8 {
    let mut num_pre_div = num;
    let mut num_post_div;
    let mut sum = 0u32;
    let mut i = 0u8;
    // Scan digits from right to left
    loop {
        num_post_div = num_pre_div / 10;
        i += 1;
        let digit = num_pre_div - num_post_div * 10;
        if i % 2 == 1 {
            let mut second_digit = digit * 2;
            if second_digit > 9 {
                second_digit = second_digit - 9;
            }
            sum = sum + second_digit as u32;
        } else {
            sum = sum + digit as u32;
        }
        if num_post_div == 0 {
            break;
        }
        num_pre_div = num_post_div;
    }
    (sum * 9 % 10) as u8
}

/// Verifies the check digit using the Luhn algorithm.
///
/// # Examples
///
/// Provide the function with an identification number.
///
/// ```
/// let acct_num = 79927398713;
/// let is_valid = checksum::luhn_is_valid(acct_num);
/// assert_eq!(is_valid, true);
/// if is_valid {
///     println!("Account number is valid!");
/// } else {
///     println!("Account number is NOT valid!");
/// }
/// ```
pub fn luhn_is_valid(full_number: u64) -> bool {
    luhn_checksum(full_number) == 0
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

/// Computes the Verhoeff checksum for the provided number.
///
/// # Examples
///
/// Generate a checksum for 236:
///
/// ```
/// let num = 2363;
/// let check_digit = checksum::verhoeff_checksum(num);
/// assert_eq!(check_digit, 0);
/// ```
pub fn verhoeff_checksum(num: u64) -> u8 {
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
    c
}

/// Computes the Verhoeff check digit for the provided number.
///
/// # Examples
///
/// Generate a check digit for 236:
///
/// ```
/// let num = 236;
/// let check_digit = checksum::verhoeff_calculate_check_digit(num);
/// assert_eq!(check_digit, 3);
/// ```
pub fn verhoeff_calculate_check_digit(num: u64) -> u8 {
    let c = verhoeff_checksum(num * 10);
    INV_DIHEDRAL_D5[c as usize]
}

/// Uses the Verhoeff checksum formula for error detection.
///
/// # Examples
///
/// Validate the check digit 2363.
///
/// ```
/// let num = 2363;
/// let is_valid = checksum::verhoeff_is_valid(num);
/// assert_eq!(is_valid, true);
/// ```
pub fn verhoeff_is_valid(num: u64) -> bool {
    verhoeff_checksum(num) == 0
}
