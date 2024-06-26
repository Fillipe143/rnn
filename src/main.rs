use mat::*;

fn main() {
    let m_2x2: Mat<f32> = mat![0.0; 2, 2];
    let m_1x2: Mat<f32> = mat![row 1.0, 2.0];
    let m_2x1: Mat<f32> = mat![col 1.0, 2.0];
    let m_2x3: Mat<f32> = mat![
        1.0, 2.0, 3.0;
        4.0, 5.0, 6.0;
    ];


    println!("{:#?}", m_2x2);
    println!("{:#?}", m_1x2);
    println!("{:#?}", m_2x1);
    println!("{:#?}", m_2x3);
}
