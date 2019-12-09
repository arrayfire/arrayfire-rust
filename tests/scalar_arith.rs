use ::arrayfire::*;

#[allow(non_snake_case)]
#[test]
fn check_scalar_arith() {
    let dims = Dim4::new(&[5, 5, 1, 1]);
    let A = randu::<f32>(dims);
    let s: f32 = 2.0;
    let scalar_as_lhs = s * &A;
    let scalar_as_rhs = &A * s;
    let C = constant(s, dims);
    let no_scalars = A * C;
    let scalar_res_comp = eq(&scalar_as_lhs, &scalar_as_rhs, false);
    let res_comp = eq(&scalar_as_lhs, &no_scalars, false);
    let scalar_res = all_true_all(&scalar_res_comp);
    let res = all_true_all(&res_comp);

    assert_eq!(scalar_res.0, res.0);
}
