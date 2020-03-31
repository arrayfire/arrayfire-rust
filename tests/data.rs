use ::arrayfire::*;

#[allow(unused_variables)]
#[test]
fn check_reorder_api() {
    let dims = Dim4::new(&[4, 5, 2, 3]);
    let a = randu::<f32>(dims);

    let transposed = reorder_v2(&a, 1, 0, None);
    let swap_0_2 = reorder_v2(&a, 2, 1, Some(vec![0]));
    let swap_1_2 = reorder_v2(&a, 0, 2, Some(vec![1]));
    let swap_0_3 = reorder_v2(&a, 3, 1, Some(vec![2, 0]));
}
