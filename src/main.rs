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
    println!("the list len is {:?}", res);
    let vec = vec![1, 23, 15, 5];
    let res = reverse(vec);
    println!("the reversed list is {:?}", res);
    let vec = vec![1, 23, 15, 5];
    let vec2 = vec![1, 2, 3, 2, 1];
    let res = palindrome(vec);
    let res2 = palindrome(vec2);
    println!("{:?}", res);
    println!("{:?}", res2);
}

fn last(a_list: Vec<i32>) -> Option<i32> {
    /*P01 (*) Find the last element of a list.
    Example:

    scala> last(List(1, 1, 2, 3, 5, 8))
    res0: Int = 8*/
//    a_list.last() returns a ref: &...
    a_list.last().copied()
}

fn last_but_one(a_list: Vec<i32>) -> Option<i32> {
    /*P02 (*) Find the last but one element of a list.
    Example:

    scala> penultimate(List(1, 1, 2, 3, 5, 8))
    res0: Int = 5*/
    let mut tmp: Vec<i32> = a_list.clone();
    tmp.pop();
    tmp.last().copied()
}

fn k_th(a_list: Vec<i32>, k: usize) -> Option<i32> {
    /*P03 (*) Find the Kth element of a list.
    By convention, the first element in the list is element 0.
    Example:

    scala> nth(2, List(1, 1, 2, 3, 5, 8))
    res0: Int = 2*/
    a_list.get(k).copied()
}

fn num_of_element(a_list: Vec<i32>) -> usize {
    /*
    P04 (*) Find the number of elements of a list.
    Example:

    scala> length(List(1, 1, 2, 3, 5, 8))
    res0: Int = 6*/
    a_list.iter().count()
}

fn reverse(a_list: Vec<i32>) -> Vec<i32> {
    /*P05 (*) Reverse a list.
    Example:

    scala> reverse(List(1, 1, 2, 3, 5, 8))
    res0: List[Int] = List(8, 5, 3, 2, 1, 1)*/

//    a_list.iter().rev().collect();
    let mut tmp: Vec<i32> = a_list.clone();
    tmp.reverse();
    tmp
}

fn palindrome(mut a_list: Vec<i32>) -> bool {
    /*
    P06 (*) Find out whether a list is a palindrome.
        Example:

        scala> isPalindrome(List(1, 2, 3, 2, 1))
        res0: Boolean = true
    */

    let mut tmp: Vec<i32> = a_list.clone();
    tmp.reverse();
    a_list == tmp
}

fn flatten_list() {
    /*
    P07 (**) Flatten a nested list structure.
    Example:

    scala> flatten(List(List(1, 1), 2, List(3, List(5, 8))))
    res0: List[Any] = List(1, 1, 2, 3, 5, 8)
    */
//    this should not exists
}

fn eliminate_consecutive_duplicate(a_list: Vec<i32>) -> Vec<i32> {
    /*
    P08 (**) Eliminate consecutive duplicates of list elements.
    If a list contains repeated elements they should be replaced with a single copy of the element.
    The order of the elements should not be changed.
    Example:

    scala> compress(List('a, 'a, 'a, 'a, 'b, 'c, 'c, 'a, 'a, 'd, 'e, 'e, 'e, 'e))
    res0: List[Symbol] = List('a, 'b, 'c, 'a, 'd, 'e)
    */

}