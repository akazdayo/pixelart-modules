use numpy::{
    convert::IntoPyArray,
    ndarray::{Array3, ArrayBase, Dim, ViewRepr},
    PyArray3, PyReadonlyArray2,
};
use pyo3::{
    prelude::{Bound, PyAnyMethods},
    pymodule,
    types::{PyAny, PyModule},
    PyResult, Python,
};

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

#[pymodule]
fn pixelart_modules(_py: Python<'_>, m: &Bound<'_, PyModule>) -> PyResult<()> {
    #[pyfn(m)]
    #[pyo3(name = "convert")]
    fn convert_py<'py>(
        py: Python<'py>,
        img: pyo3::prelude::Bound<'_, PyAny>, // NumPy Array
        color_palette: PyReadonlyArray2<'py, usize>,
    ) -> PyResult<Bound<'py, PyArray3<usize>>> {
        // NumPy配列をndarrayに変換
        let color_palette = color_palette.as_array();
        let shape: (usize, usize, usize) = img.getattr("shape")?.extract()?;
        let (h, w, _) = shape;
        let mut changed: Array3<usize> = Array3::zeros((h, w, 3));

        for y in 0..h {
            for x in 0..w {
                let color = color_change(
                    img.get_item((y, x, 0))?.extract::<usize>()? as f64,
                    img.get_item((y, x, 1))?.extract::<usize>()? as f64,
                    img.get_item((y, x, 2))?.extract::<usize>()? as f64,
                    &color_palette,
                )
                .unwrap();
                changed[[y, x, 0]] = color[0];
                changed[[y, x, 1]] = color[1];
                changed[[y, x, 2]] = color[2];
            }
        }

        // output
        let output = changed.to_owned();
        Ok(output.into_pyarray_bound(py))
    }
    Ok(())
}
