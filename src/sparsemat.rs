use sprs::CsMat;

#[test]
fn test() {
    let a = CsMat::new_csc((3, 3),
                         vec![0, 2, 4, 5],
                         vec![0, 1, 0, 2, 2],
                         vec![1., 2., 3., 4., 5.]);
    println!("{:?}", a);
}
