fn main() {
    fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
        let mut result = Vec::new();
        let mut i=0;
        let mut j=0;
        while i<left.len() && j<right.len() {
            if left[i] <= right[j] {
                result.push(left[i]);
                i+=1;
            } else {
                result.push(right[j]);
                j+=1;
            }
        }
        while i<left.len() {
            result.push(left[i]);
            i+=1;
        }
        while j<right.len() {
            result.push(right[j]);
            j+=1;
        }
        return result;
    }
    fn mergesort(arr: &[i32]) -> Vec<i32> {
        if arr.len() <= 1 {
            return arr.to_vec();
        }
        let mid = arr.len() / 2;
        let left = mergesort(&arr[0..mid]); // Recursive call on the left slice
        let right = mergesort(&arr[mid..]); // Recursive call on the right slice

        return merge(&left, &right);
    }

    let arr = [5, 3, 2, 4, 1];
    let sorted=mergesort(&arr);
    println!("{:?}", sorted);
}