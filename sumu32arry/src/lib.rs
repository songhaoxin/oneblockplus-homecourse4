
pub fn sum_arry(arry: &[u32]) -> Option<u32> {

    let mut sum = 0u32;
    let mut overflow = false;
    for value in arry {
        match (sum.checked_add(*value)) {
            Some(v) => {
                sum = v;
            },
            None => {
                overflow = true;
                break;
            },
        }
    }

    if overflow == true {
        None
    } else {
        Some(sum)
    }
}
#[cfg(test)]
mod tests {
    use crate::*;
    #[test]
    fn it_works() {
        let arry1 = [100,200,300,300];
        let arry2 = [100,200,u32::MAX,1];
        assert_eq!(sum_arry(&arry1),Some(900));
        assert_eq!(sum_arry(&arry2),None);
    }
}
