/// The `CheckDigitAlgorithm` trait is used to specify the functionality of a checksum function
/// that uses a check digit for error detection.
pub trait CheckDigitAlgorithm {
    /// Computes the checksum for the provided number.
    fn checksum(&self, num: u64) -> u8;

    /// Computes the check digit for the provided number.
    fn calculate_check_digit(&self, num: u64) -> u8;

    /// Tests if the provided number, which must be suffixed with a check digit, is erroneous.
    fn is_valid(&self, num: u64) -> bool;
}

/// Luhn check digit algorithm.
///
/// The Luhn algorithm will detect any single-digit error, as well as **almost** all transpositions
/// of adjacent digits. It will **not**, however, detect transposition of the two-digit sequence
/// *09* to *90* (or vice versa). It will detect 7 of the 10 possible twin errors (it will not
/// detect *22* <-> *55*, *33* <-> *66*, or *44* <-> *77*).
pub struct LuhnAlgorithm {}

impl LuhnAlgorithm {
    /// Performs the summation of digits in a number as specified by the Luhn algorithm.
    fn digit_sum(num: u64) -> u32 {
        let mut num_pre_div = num;
        let mut num_post_div;
        let mut sum = 0u32;
        let mut i = 0u8;
        // Scan digits from right to left
        loop {
            num_post_div = num_pre_div / 10;
            i += 1;
            let digit = num_pre_div - num_post_div * 10;
            if i % 2 == 0 {
                // Even indexed digits are doubled and adjusted if greater than 9
                let mut second_digit = digit * 2;
                if second_digit > 9 {
                    second_digit = second_digit - 9;
                }
                sum += second_digit as u32;
            } else {
                // Odd indexed digits are treated as-is
                sum += digit as u32;
            }
            if num_post_div == 0 {
                break;
            }
            num_pre_div = num_post_div;
        }
        sum
    }
}

impl CheckDigitAlgorithm for LuhnAlgorithm {
    /// Computes the Luhn checksum for the provided number.
    ///
    /// # Examples
    ///
    /// Provide the function with an identification number.
    ///
    /// ```
    /// use checksum::CheckDigitAlgorithm;
    /// let acct_num = 79927398713;
    /// let algo = checksum::LuhnAlgorithm {};
    /// let checksum = algo.checksum(acct_num);
    /// assert_eq!(checksum, 0);
    /// if checksum != 0 {
    ///     println!("Account number has been corrupted!");
    /// }
    /// ```
    fn checksum(&self, num: u64) -> u8 {
        (LuhnAlgorithm::digit_sum(num) % 10) as u8
    }

    /// Computes the Luhn check digit for the provided number.
    ///
    /// # Examples
    ///
    /// Provide the function with an identification number.
    ///
    /// ```
    /// use checksum::CheckDigitAlgorithm;
    /// let acct_num = 7992739871;
    /// let algo = checksum::LuhnAlgorithm {};
    /// let checksum = algo.calculate_check_digit(acct_num);
    /// assert_eq!(checksum, 3);
    /// ```
    fn calculate_check_digit(&self, num: u64) -> u8 {
        (LuhnAlgorithm::digit_sum(num * 10) * 9 % 10) as u8
    }

    /// Verifies the check digit using the Luhn algorithm.
    ///
    /// # Examples
    ///
    /// Provide the function with an identification number.
    ///
    /// ```
    /// use checksum::CheckDigitAlgorithm;
    /// let acct_num = 79927398713;
    /// let algo = checksum::LuhnAlgorithm {};
    /// let is_valid = algo.is_valid(acct_num);
    /// assert_eq!(is_valid, true);
    /// if is_valid {
    ///     println!("Account number is valid!");
    /// } else {
    ///     println!("Account number is NOT valid!");
    /// }
    /// ```
    fn is_valid(&self, num: u64) -> bool {
        self.checksum(num) == 0
    }
}

/// Verhoeff check digit algorithm.
pub struct VerhoeffAlgorithm {}

const VERHOEFF_D_TABLE: [[u8; 10]; 10] = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                                          [1, 2, 3, 4, 0, 6, 7, 8, 9, 5],
                                          [2, 3, 4, 0, 1, 7, 8, 9, 5, 6],
                                          [3, 4, 0, 1, 2, 8, 9, 5, 6, 7],
                                          [4, 0, 1, 2, 3, 9, 5, 6, 7, 8],
                                          [5, 9, 8, 7, 6, 0, 4, 3, 2, 1],
                                          [6, 5, 9, 8, 7, 1, 0, 4, 3, 2],
                                          [7, 6, 5, 9, 8, 2, 1, 0, 4, 3],
                                          [8, 7, 6, 5, 9, 3, 2, 1, 0, 4],
                                          [9, 8, 7, 6, 5, 4, 3, 2, 1, 0]];

const VERHOEFF_INV_D_TABLE: [u8; 10] = [0, 4, 3, 2, 1, 5, 6, 7, 8, 9];

const VERHOEFF_P_TABLE: [[u8; 10]; 8] = [[0, 1, 2, 3, 4, 5, 6, 7, 8, 9],
                                         [1, 5, 7, 6, 2, 8, 3, 0, 9, 4],
                                         [5, 8, 0, 3, 7, 9, 6, 1, 4, 2],
                                         [8, 9, 1, 6, 0, 4, 3, 5, 2, 7],
                                         [9, 4, 5, 3, 1, 2, 6, 8, 7, 0],
                                         [4, 2, 8, 6, 5, 7, 3, 9, 0, 1],
                                         [2, 7, 9, 3, 8, 0, 6, 4, 1, 5],
                                         [7, 0, 4, 6, 9, 1, 3, 2, 5, 8]];

impl CheckDigitAlgorithm for VerhoeffAlgorithm {
    /// Computes the Verhoeff checksum for the provided number.
    ///
    /// # Examples
    ///
    /// Generate a checksum for 236:
    ///
    /// ```
    /// use checksum::CheckDigitAlgorithm;
    /// let num = 2363;
    /// let algo = checksum::VerhoeffAlgorithm {};
    /// let check_digit = algo.checksum(num);
    /// assert_eq!(check_digit, 0);
    /// ```
    fn checksum(&self, num: u64) -> u8 {
        let mut num_pre_div = num;
        let mut num_post_div;
        let mut i = 0u8;
        let mut c = 0u8;
        loop {
            num_post_div = num_pre_div / 10;
            let digit = num_pre_div - num_post_div * 10;
            c = VERHOEFF_D_TABLE[c as usize][VERHOEFF_P_TABLE[(i % 8) as usize][digit as usize] as usize];
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
    /// use checksum::CheckDigitAlgorithm;
    /// let num = 236;
    /// let algo = checksum::VerhoeffAlgorithm {};
    /// let check_digit = algo.calculate_check_digit(num);
    /// assert_eq!(check_digit, 3);
    /// ```
    fn calculate_check_digit(&self, num: u64) -> u8 {
        let c = self.checksum(num * 10);
        VERHOEFF_INV_D_TABLE[c as usize]
    }

    /// Uses the Verhoeff checksum formula for error detection.
    ///
    /// # Examples
    ///
    /// Validate the check digit 2363.
    ///
    /// ```
    /// use checksum::CheckDigitAlgorithm;
    /// let num = 2363;
    /// let algo = checksum::VerhoeffAlgorithm {};
    /// let is_valid = algo.is_valid(num);
    /// assert_eq!(is_valid, true);
    /// ```
    fn is_valid(&self, num: u64) -> bool {
        self.checksum(num) == 0
    }
}
