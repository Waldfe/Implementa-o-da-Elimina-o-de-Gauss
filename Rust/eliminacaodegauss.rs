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
    // Caso de teste 1
    {
        let a = vec![
            vec![2.0, 1.0, -1.0],
            vec![1.0, 2.0, 1.0],
            vec![1.0, 1.0, 1.0],
        ];
        let b = vec![-3.0, 3.0, 2.0];

        let resultado = gauss_solver(3, a, b);

        println!("Caso de Teste 1:");
        for (i, x) in resultado.iter().enumerate() {
            println!("x{} = {:.6}", i + 1, x);
        }
    }

    // Caso de teste 2
    {
        let a = vec![
            vec![1.0, 1.0, 1.0],
            vec![-2.0, 1.0, 1.0],
            vec![1.0, 3.0, 1.0],
        ];
        let b = vec![2.0, 5.0, 4.0];

        let resultado = gauss_solver(3, a, b);

        println!("Caso de Teste 2:");
        for (i, x) in resultado.iter().enumerate() {
            println!("x{} = {:.6}", i + 1, x);
        }
    }

    // Caso de teste 3
    {
        let a = vec![
            vec![3.0, 2.0, -1.0],
            vec![2.0, -2.0, 4.0],
            vec![-1.0, 0.5, -1.0],
        ];
        let b = vec![1.0, -2.0, 0.0];

        let resultado = gauss_solver(3, a, b);

        println!("Caso de Teste 3:");
        for (i, x) in resultado.iter().enumerate() {
            println!("x{} = {:.6}", i + 1, x);
        }
    }

    // Caso de teste 4 (sistema 2x2)
    {
        let a = vec![vec![2.0, 3.0], vec![4.0, 9.0]];
        let b = vec![6.0, 15.0];

        let resultado = gauss_solver(2, a, b);

        println!("Caso de Teste 4:");
        for (i, x) in resultado.iter().enumerate() {
            println!("x{} = {:.6}", i + 1, x);
        }
    }

    // Caso de teste 5 (sistema 4x4)
    {
        let a = vec![
            vec![4.0, 1.0, 2.0, -3.0],
            vec![-3.0, 3.0, -1.0, 4.0],
            vec![-1.0, 2.0, 5.0, 1.0],
            vec![5.0, 4.0, 3.0, -1.0],
        ];
        let b = vec![-16.0, 20.0, -4.0, -10.0];

        let resultado = gauss_solver(4, a, b);

        println!("Caso de Teste 5:");
        for (i, x) in resultado.iter().enumerate() {
            println!("x{} = {:.6}", i + 1, x);
        }
    }

    // Caso de teste 6 (sistema 5x5)
    {
        let a = vec![
            vec![4.0, 1.0, 2.0, -3.0, 5.0],
            vec![-3.0, 3.0, -1.0, 4.0, -2.0],
            vec![-1.0, 2.0, 5.0, 1.0, 3.0],
            vec![5.0, 4.0, 3.0, -1.0, 2.0],
            vec![1.0, -2.0, 3.0, -4.0, 5.0],
        ];
        let b = vec![-16.0, 20.0, -4.0, -10.0, 3.0];

        let resultado = gauss_solver(5, a, b);

        println!("Caso de Teste 6:");
        for (i, x) in resultado.iter().enumerate() {
            println!("x{} = {:.6}", i + 1, x);
        }
    }
}