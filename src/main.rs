use std::collections::HashMap;
use rand::Rng;

fn main() {
    let mut array = Vec::new();
    let arr_ref = &mut array;
    for num in 1..100
    {
        let x = rand::thread_rng().gen_range(1, 10000);
        arr_ref.push(x);
    }
    array = merge_sort(array);
    for num in &array
    {
        println!("{}", num);
    }
}

fn merge(mut left: Vec<i32>, mut right: Vec<i32>) -> Vec<i32>
{
    let mut result: Vec<i32> = Vec::new();
    let mut left_index = 0;
    let mut right_index = 0;

    while left_index < left.len() && right_index < right.len()
    {
        if &left[left_index] < &right[right_index]
        {
            result.push(left[left_index]);
            left_index = left_index + 1;
        }
        else
        {
            result.push(right[right_index]);
            right_index = right_index + 1;
        }
    }

    if left_index < left.len()
    {
        let part = &mut left[left_index..].to_vec();
        result.append(part);
    }
    else if right_index < right.len()
    {
        let part = &mut right[right_index..].to_vec();
        result.append(part);
    }
    result
}

fn merge_sort(array: Vec<i32>) -> Vec<i32>
{
    if array.len() < 2
    {
        return array;
    }
    else
    {
        let left = &array[..array.len() / 2];
        let right = &array[array.len()/2..];
        return merge(merge_sort(left.to_vec()), merge_sort(right.to_vec()));
    }

}