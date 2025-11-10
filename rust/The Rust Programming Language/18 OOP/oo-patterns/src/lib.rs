pub struct AveragedCollection {
    list: Vec<i32>,
    average: f64,
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}


// Listing 18-1: An AveragedCollection struct that maintains a list of integers and the average of the items in the collection

