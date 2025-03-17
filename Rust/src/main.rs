use rand::Rng;
use std::env;

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

    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        eprintln!("Uso: {} <tamanho> <número de testes>", args[0]);
        return;
    }

    let tamanho: usize = match args[1].parse() {
        Ok(val) if val > 0 => val,
        _ => {
            eprintln!("'tamanho' deve ser um número maior que 0.");
            return;
        }
    };

    let numero_de_testes: usize = match args[2].parse() {
        Ok(val) if val > 0 => val,
        _ => {
            eprintln!("'número de testes' deve ser um número maior que 0.");
            return;
        }
    };

    let mut rng = rand::rng();

    // Gerar 10 casos de teste aleatórios
    let test_cases: Vec<(Vec<Vec<f64>>, Vec<f64>)> = (0..10)
        .map(|_| {
            let a: Vec<Vec<f64>> = (0..tamanho)
                .map(|_| (0..tamanho).map(|_| rng.random_range(-10.0..10.0)).collect())
                .collect();
            let b: Vec<f64> = (0..tamanho).map(|_| rng.random_range(-10.0..10.0)).collect();
            (a, b)
        })
        .collect();

    // Executar um caso de teste aleatório numeroDeTestes vezes
    for _ in 0..numero_de_testes {
        let (a, b) = &test_cases[rng.random_range(0..test_cases.len())];
        let _resultado = gauss_solver(a.len(), a.clone(), b.clone());
    }
}
