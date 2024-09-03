use numpy::{
    ndarray::{Array3, ArrayBase, Dim, ViewRepr},
    PyReadonlyArray2,
};
use pyo3::prelude::*;

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
    img: Bound<'_, PyAny>, // NumPy Array
    color_palette: PyReadonlyArray2<'py, usize>,
) -> PyResult<Array3<usize>> {
    // NumPy配列をndarrayに変換
    let color_palette = color_palette.as_array();

    // 画像のサイズを取得
    let shape: (usize, usize, usize) = img.getattr("shape")?.extract()?;
    let (h, w, _) = shape;

    // 出力用のndarrayを作成
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
    Ok(changed)
}

/// A Python module implemented in Rust.
#[pymodule]
fn pixelart(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(convert, m)?)?;
    Ok(())
}
