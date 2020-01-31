
fn main() {
    println!("Hello, world!");
    let vec = vec![1, 23, 5, 5];
    let res = last(vec);
    println!("the last element is {:?}", res);
    let vec = vec![1, 23, 15, 5];
    let res = last_but_one(vec);
    println!("the last but one element is {:?}", res);
    let vec = vec![1, 23, 15, 5];
    let res = k_th(vec, 2); // handle panic
    println!("the k th element is {:?}", res);
    let vec = vec![1, 23, 15, 5];
    let res = num_of_element(vec);
    println!("the lst len is {:?}", res);
}

fn last(a_list: Vec<i32>) -> Option<i32> {
    /*P01 (*) Find the last element of a list.
    Example:

    scala> last(List(1, 1, 2, 3, 5, 8))
    res0: Int = 8*/
    match a_list.len() {
        0 => None,
        n => Some(a_list[n - 1])
    }
}

fn last_but_one(a_list: Vec<i32>) -> Option<i32> {
    /*P02 (*) Find the last but one element of a list.
    Example:

    scala> penultimate(List(1, 1, 2, 3, 5, 8))
    res0: Int = 5*/
    match a_list.len() {
        0..=1 => None,
        n => Some(a_list[n - 2])
    }
}

fn k_th(a_list: Vec<i32>, k: usize) -> i32 {
    /*P03 (*) Find the Kth element of a list.
    By convention, the first element in the list is element 0.
    Example:

    scala> nth(2, List(1, 1, 2, 3, 5, 8))
    res0: Int = 2*/
    a_list[k]
}

fn num_of_element(a_list: Vec<i32>) -> usize {
    /*
    P04 (*) Find the number of elements of a list.
    Example:

    scala> length(List(1, 1, 2, 3, 5, 8))
    res0: Int = 6*/
    a_list.len()
}

fn reverse(a_list: Vec<i32>) -> Vec<i32>{
    /*P05 (*) Reverse a list.
    Example:

    scala> reverse(List(1, 1, 2, 3, 5, 8))
    res0: List[Int] = List(8, 5, 3, 2, 1, 1)*/
    for i in a_list.iter() {
        print!("{:?}", i);
    }
    a_list.iter().rev().collect()

}