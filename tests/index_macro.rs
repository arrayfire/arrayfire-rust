use arrayfire::{af_print, randu, seq, view, Array, Dim4};

#[test]
fn array_view() {
    let dims = Dim4::new(&[5, 5, 2, 1]);
    let a = randu::<f32>(dims);
    let b = a.clone();
    let c = a.clone();
    let d = a.clone();
    let e = a.clone();

    let v = view!(a);
    af_print!("v = a[None]", v);

    let m = view!(c[1:3:1, 1:3:2]);
    af_print!("m = c[:, :]", m);

    let x = seq!(1:3:1);
    let y = seq!(1:3:2);
    let u = view!(b[x, y]);
    af_print!("u = b[seq(), seq()]", u);

    let values: [u32; 3] = [1, 2, 3];
    let indices = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));
    let indices2 = Array::new(&values, Dim4::new(&[3, 1, 1, 1]));

    let w = view!(d[indices, indices2]);
    af_print!("w = d[Array, Array]", w);

    let z = view!(e[indices, y]);
    af_print!("z = e[Array, Seq]", z);
}
