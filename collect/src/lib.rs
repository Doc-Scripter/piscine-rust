pub fn bubble_sort(arr: &mut [i32]) {
    let len = arr.len();
    
    // Return early if the array is empty or has only one element
    if len <= 1 {
        return;
    }
    
    // Outer loop: each pass through the array
    for i in 0..len {
        // Flag to optimize: if no swaps occur in a pass, the array is sorted
        let mut swapped = false;
        
        // Inner loop: compare adjacent elements and swap if needed
        // With each pass, the largest element "bubbles" to the end,
        // so we can reduce the number of comparisons
        for j in 0..len - i - 1 {
            if arr[j] > arr[j + 1] {
                // Swap elements using Rust's built-in swap method
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        
        // If no swaps occurred in this pass, the array is already sorted
        if !swapped {
            break;
        }
    }
}
