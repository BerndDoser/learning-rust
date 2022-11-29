fn foo(v: &Vec<i32>) -> i32 {
    // v[0] = 2; not mutable
    let mut result = 0;
    for element in v { result += element; }
    result
}

#[test]
fn test() {
    let v = vec![1, 2, 3];
    let v2 = v;
    let a = foo(&v2);
    assert!(a == 6);

    //println!("v: {:?}", v2);
    assert!(v2[0] == 1);
}
