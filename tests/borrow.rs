fn take(v: Vec<i32>) -> i32
{
    let mut result = 0;
    for element in v { result += element; }
    result
}

fn borrow(v: &Vec<i32>) -> i32
{
    // v[0] = 2; not mutable
    let mut result = 0;
    for element in v { result += element; }
    result
}

#[test]
fn test()
{
    let v = vec![1, 2, 3];
    //println!("v: {:?}", v2);

    let v2 = v;
    //assert!(v.is_empty());
    assert_eq!(v2, [1, 2, 3]);

    let v3 = v2.clone();
    assert_eq!(v3, [1, 2, 3]);

    let sum = borrow(&v2);
    assert!(sum == 6);

    let sum2 = take(v2);
    assert!(sum2 == 6);
    //assert!(v2.is_empty());
}
