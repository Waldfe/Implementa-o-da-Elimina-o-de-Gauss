use std::io;

fn gauss_solver(n: usize, mut A: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let mut X = vec![0.0; n];

    // Etapa de escalonamento
    for k in 0..n - 1 {
        let mut max_index = k;
        let mut max_value = A[k][k].abs();

        // Encontrando o pivô
        for i in k + 1..n {
            if A[i][k].abs() > max_value {
                max_value = A[i][k].abs();
                max_index = i;
            }
        }

        // Troca de linhas se necessário
        if max_index != k {
            A.swap(k, max_index);
            b.swap(k, max_index);
        }

        // Se A[k][k] for zero, a matriz é singular
        if A[k][k] == 0.0 {
            println!("A matriz dos coeficientes é singular");
            return vec![];
        }

        // Transformação em forma triangular
        for i in k + 1..n {
            let factor = -A[i][k] / A[k][k];
            A[i][k] = 0.0;
            b[i] += factor * b[k];
            for j in k + 1..n {
                A[i][j] += factor * A[k][j];
            }
        }
    }

    // Resolução do sistema
    for i in (0..n).rev() {
        X[i] = b[i];
        for j in i + 1..n {
            X[i] -= A[i][j] * X[j];
        }
        X[i] /= A[i][i];
    }

    X
}

fn main() {
    let A = vec![
        vec![2.0, 1.0, -1.0],
        vec![1.0, 2.0, 1.0],
        vec![1.0, 1.0, 1.0],
    ];
    let b = vec![-3.0, 3.0, 2.0];

    let result = gauss_solver(3, A, b);

    println!("Solução:");
    for (i, x) in result.iter().enumerate() {
        println!("x{} = {:.6}", i + 1, x);
    }
}