use numpy::{
    ndarray::{ArrayBase, Dim, ViewRepr},
    PyReadonlyArray2, PyReadonlyArray3,
};
use pyo3::{ffi::PyObject, prelude::*};

/// Formats the sum of two numbers as string.
#[pyfunction]
fn sum_as_string(a: usize, b: usize) -> PyResult<String> {
    Ok((a + b).to_string())
}

fn color_change<'py>(
    r: f64,
    g: f64,
    b: f64,
    color_palette: &ArrayBase<ViewRepr<&usize>, Dim<[usize; 2]>>,
) -> PyResult<[usize; 3]> {
    // 変数定義
    let mut min_distance = f64::MAX; // 距離
    let mut color_name = [0, 0, 0]; // [r, g, b]

    // ループ処理
    for color in color_palette.rows() {
        let distance = ((r - color[0] as f64) * (r - color[0] as f64))
            + ((g - color[1] as f64) * (g - color[1] as f64))
            + ((b - color[2] as f64) * (b - color[2] as f64));
        if distance < min_distance {
            min_distance = distance;
            color_name = [color[0], color[1], color[2]];
        }
    }
    Ok(color_name)
}

#[pyfunction]
fn convert<'py>(
    img: PyReadonlyArray3<'py, usize>,
    color_palette: PyReadonlyArray2<'py, usize>,
) -> PyResult<()> {
    // NumPy配列をndarrayに変換
    let color_palette = color_palette.as_array();
    let img = img.as_array();
    let mut changed = img.to_owned();

    let (h, w, _) = img.dim();

    for y in 0..h {
        for x in 0..w {
            let color = color_change(
                img[[y, x, 0]] as f64,
                img[[y, x, 1]] as f64,
                img[[y, x, 2]] as f64,
                &color_palette,
            )
            .unwrap();
            changed[[y, x, 0]] = color[0];
            changed[[y, x, 1]] = color[1];
            changed[[y, x, 2]] = color[2];
        }
    }
    println!("{:?}", changed);

    Ok(())
}

#[pyfunction]
fn obj_test<'py>(obj: Bound<'_, PyAny>) -> PyResult<()> {
    let a = obj.call_method1("__getitem__", ((1, 0),))?;
    let a = a.extract::<usize>()?;
    println!("{:?}", a - 5);
    Ok(())
}

/// A Python module implemented in Rust.
#[pymodule]
fn pixelart(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(sum_as_string, m)?)?;
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    m.add_function(wrap_pyfunction!(obj_test, m)?)?;
    Ok(())
}
