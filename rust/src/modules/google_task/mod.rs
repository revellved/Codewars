pub fn find_operattors_for_numbers(nums: &[T]) -> T where T: Eq {
    if nums.is_empty() {
        return 0;
    }
    nums.iter().skip(1).fold(nums[0], |sum, num| {
        *[sum + num, sum -num, sum*num, sum/num].iter().max().unwrap()
    })
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn returns_expected() {
        assert_eq!(find_operattors_for_numbers(&[1,2,3,4]), 36);
        assert_eq!(find_operattors_for_numbers(&[1,2,-6,3,0.5,1,1,1,9421,-0.321352,4]), 36);
    }
}
