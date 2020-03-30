use ::arrayfire::*;
use float_cmp::approx_eq;

#[test]
fn check_reorder_api() {
    let dims = Dim4::new(&[4, 5, 2, 3]);
    let A = randu::<f32>(dims);

    let transposedA = reorder_v2(&A, 1, 0, None);
    let swap_0_2 = reorder_v2(&A, 2, 1, Some(vec![0]));
    let swap_1_2 = reorder_v2(&A, 0, 2, Some(vec![1]));
    let swap_0_3 = reorder_v2(&A, 3, 1, Some(vec![2, 0]));
}
