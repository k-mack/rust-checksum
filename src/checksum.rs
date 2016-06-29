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
