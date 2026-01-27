pub fn rotate(matrix: &mut Vec<Vec<i32>>) -> &mut Vec<Vec<i32>> {
    matrix.reverse();

    for i in 1..matrix.len() {
        for j in 0..i {
            let t = matrix[i][j];
            matrix[i][j] = matrix[j][i];
            matrix[j][i] = t
        }
    }

    return matrix;
}
