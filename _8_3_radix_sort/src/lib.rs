use counting_sort::counting_sort;

pub fn radix_sort(arr: Vec<usize>, scale: usize, max_bit: u32) -> Result<Vec<usize>, &'static str> {
    if let Some(max_value) = scale.checked_pow(max_bit) {
        for &e in arr.iter() {
            if e >= max_value {
                return Err("Element overflow!");
            }
        }
    } else {
        return Err("Max value overflow!");
    }

    let mut result = arr;

    for bit in 1..=max_bit {
        result = counting_sort(result, scale, |&e| { (e % scale.pow(bit)) / scale.pow(bit - 1) })?;
    }
    Ok(result)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_sort_ascending() -> Result<(), &'static str> {
        let mut v = vec![22, 43, 145, 1, 9];
        v = radix_sort(v, 10, 3)?;
        assert_eq!(v, vec![1, 9, 22, 43, 145]);
        Ok(())
    }
}
