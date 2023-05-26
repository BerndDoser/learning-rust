enum Matrix
{
    Zero,
    Dense { values : Vec<f64> },
    Sparse { values : f64 }
}

fn norm(m:Matrix) -> f64
{
    match m
    {
        Matrix::Zero => 0.0,
        Matrix::Dense{ values } => values.iter().sum(),
        Matrix::Sparse{ values } => 2.0 * values,
    }
}

#[test]
fn test_sum_type()
{
    let mut m: Matrix = Matrix::Dense{ values: vec![1.0, 2.0] };
    
    // assert_eq!(m.values, 25); // no field `values` on type `Matrix`
    
    assert_eq!(norm(m), 3.0);

    m = Matrix::Zero;
    assert_eq!(norm(m), 0.0);

    m = Matrix::Sparse{ values: -3.0 };
    assert_eq!(norm(m), -6.0);
}
