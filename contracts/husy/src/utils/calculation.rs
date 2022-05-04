pub(crate) fn calculate_procentage(percentage: u32, amount_to_pay: u128) -> u128 {
    percentage as u128 * amount_to_pay / 10_000u128
}

#[cfg(test)]
mod test {
    use super::calculate_procentage;

    #[test]
    fn calculate_procentage_test() {
        let result = calculate_procentage(200, 10000);

        assert_eq!(result, 200);
    }
}
