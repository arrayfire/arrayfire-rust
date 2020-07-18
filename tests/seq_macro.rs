use arrayfire::{af_print, dim4, index, randu, seq};

#[test]
fn array_view() {
    let _dim1d = dim4!(2);
    let _dim2d = dim4!(2, 3);
    let _dim3d = dim4!(2, 3, 4);

    let mut dim4d = dim4!(5, 3, 2, 1);
    dim4d[2] = 1;

    let a = randu::<f32>(dim4d);
    let seqs = &[seq!(1:3:1), seq!()];
    let sub = index(&a, seqs);
    af_print!("A", a);
    af_print!("Indexed A", sub);
}
