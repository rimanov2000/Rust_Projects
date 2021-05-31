// Given a list of integers, use a vector and return the mean (the average value), 
// median (when sorted, the value in the middle position), and 
// mode (the value that occurs most often; 
// a hash map will be helpful here) of the list.

// By Rufat Imanov

use std::collections::HashMap;
// Insertion Sort
fn my_insertion_sort(my_vector: &mut [i32]) {
    let mut unsorted_idx = 1; // pointer to first element of unsorted array
    let mut change_idx; // idx pointer searches for right most smaller number of sorted arrray than left most unsorted array number
    let mut changed_val; // saves the LMNU - left most number of unsorted array
    let vec_size = my_vector.len(); 

    while unsorted_idx != vec_size {
        change_idx = 0;
        changed_val = my_vector[unsorted_idx];
        while change_idx != unsorted_idx && my_vector[(unsorted_idx - 1) - change_idx] > my_vector[unsorted_idx - change_idx] {
            my_vector[unsorted_idx - change_idx] = my_vector[(unsorted_idx - 1) - change_idx];
            my_vector[(unsorted_idx - 1) - change_idx] = changed_val;
            change_idx += 1;
        }
        unsorted_idx += 1;
    }
}

fn print_vector(my_vector: & [i32]) {
    for (idx,i) in my_vector[..].iter().enumerate(){
        println!("idx: {}, val: {}", idx, i);
    }
}

fn main() {
    let mut v = vec![50, 1, 22, 33, 8, 28, 38, 37, 12, 13, 12, 13, 13, 13, 31, 30, 20, 10];
    // first sort then calculate mean, meadian, and mode
    my_insertion_sort(v.as_mut_slice());
    print_vector(v.as_slice());
    let mut sum_vector : i32 = 0;
    for val in v.iter() {
        sum_vector += *val;
    }
    let mean_value : i32 = sum_vector / v.len() as i32;
    println!("The mean of the input array is {}",mean_value);
    println!("The meadian of the input array is {}",v[v.len() / 2]);
    // Mode
    let mut my_map = HashMap::new();
    for val in &v {
        let count = my_map.entry(val).or_insert(0);
        *count += 1;
    }
    let mut mode_val : Option<&i32> = None;
    for i in my_map.keys() {
        if mode_val == None {
            mode_val = Some(*i);
        }
        else if my_map[i] > my_map[mode_val.unwrap()] {
            mode_val = Some(*i);
        }
    }
    match mode_val {
        Some(val) => println!("The mode of the input array is {}",val),
        None => println!("Failed to find the mode"),
    }
    
}
