use rand::Rng;

fn gauss_solver(n: usize, mut a: Vec<Vec<f64>>, mut b: Vec<f64>) -> Vec<f64> {
    let mut x = vec![0.0; n];

    // Etapa de escalonamento
    for k in 0..n - 1 {
        let mut max_index = k;
        let mut max_value = a[k][k].abs();

        // Encontrando o pivô
        for i in k + 1..n {
            if a[i][k].abs() > max_value {
                max_value = a[i][k].abs();
                max_index = i;
            }
        }

        // Troca de linhas se necessário
        if max_index != k {
            a.swap(k, max_index);
            b.swap(k, max_index);
        }

        // Se a[k][k] for zero, a matriz é singular
        if a[k][k] == 0.0 {
            println!("A matriz dos coeficientes é singular");
            return vec![];
        }

        // Transformação em forma triangular
        for i in k + 1..n {
            let factor = -a[i][k] / a[k][k];
            a[i][k] = 0.0;
            b[i] += factor * b[k];
            for j in k + 1..n {
                a[i][j] += factor * a[k][j];
            }
        }
    }

    // Resolução do sistema
    for i in (0..n).rev() {
        x[i] = b[i];
        for j in i + 1..n {
            x[i] -= a[i][j] * x[j];
        }
        x[i] /= a[i][i];
    }

    x
}

fn main() {
    let test_cases = vec![
        (
            vec![
                vec![2.0, 1.0, -1.0],
                vec![1.0, 2.0, 1.0],
                vec![1.0, 1.0, 1.0],
            ],
            vec![-3.0, 3.0, 2.0],
        ),
        (
            vec![
                vec![1.0, 1.0, 1.0],
                vec![-2.0, 1.0, 1.0],
                vec![1.0, 3.0, 1.0],
            ],
            vec![2.0, 5.0, 4.0],
        ),
        (
            vec![
                vec![3.0, 2.0, -1.0],
                vec![2.0, -2.0, 4.0],
                vec![-1.0, 0.5, -1.0],
            ],
            vec![1.0, -2.0, 0.0],
        ),
        (
            vec![vec![2.0, 3.0], vec![4.0, 9.0]],
            vec![6.0, 15.0],
        ),
        (
            vec![
                vec![4.0, 1.0, 2.0, -3.0],
                vec![-3.0, 3.0, -1.0, 4.0],
                vec![-1.0, 2.0, 5.0, 1.0],
                vec![5.0, 4.0, 3.0, -1.0],
            ],
            vec![-16.0, 20.0, -4.0, -10.0],
        ),
        (
            vec![
                vec![4.0, 1.0, 2.0, -3.0, 5.0],
                vec![-3.0, 3.0, -1.0, 4.0, -2.0],
                vec![-1.0, 2.0, 5.0, 1.0, 3.0],
                vec![5.0, 4.0, 3.0, -1.0, 2.0],
                vec![1.0, -2.0, 3.0, -4.0, 5.0],
            ],
            vec![-16.0, 20.0, -4.0, -10.0, 3.0],
        ),
    ];

    let mut rng = rand::rng();

    for _ in 0..1_000_000_000 {
        let (a, b) = &test_cases[rng.random_range(0..test_cases.len())];
        let _resultado = gauss_solver(a.len(), a.clone(), b.clone());
    }
}
