package main

import (
	"fmt"
	"math"
	"math/rand"
	"os"
)

func gaussSolver(n int, A [][]float64, b []float64) []float64 {
	X := make([]float64, n)

	// Escalonamento
	for k := 0; k < n-1; k++ {
		maxIndex := k
		maxValue := math.Abs(A[k][k])

		// Encontrando o pivô
		for i := k + 1; i < n; i++ {
			if math.Abs(A[i][k]) > maxValue {
				maxValue = math.Abs(A[i][k])
				maxIndex = i
			}
		}

		// Troca de linhas se necessário
		if maxIndex != k {
			A[k], A[maxIndex] = A[maxIndex], A[k]
			b[k], b[maxIndex] = b[maxIndex], b[k]
		}

		// Se A[k][k] for zero, a matriz é singular
		if A[k][k] == 0 {
			fmt.Println("A matriz dos coeficientes é singular")
			return nil
		}

		// Transformação em forma triangular
		for i := k + 1; i < n; i++ {
			factor := -A[i][k] / A[k][k]
			A[i][k] = 0
			b[i] += factor * b[k]
			for j := k + 1; j < n; j++ {
				A[i][j] += factor * A[k][j]
			}
		}
	}

	// Resolução do sistema
	for i := n - 1; i >= 0; i-- {
		X[i] = b[i]
		for j := i + 1; j < n; j++ {
			X[i] -= A[i][j] * X[j]
		}
		X[i] /= A[i][i]
	}

	return X
}

func main() {
	if len(os.Args) < 3 {
		fmt.Printf("Uso: %s <tamanho> <número de testes>\n", os.Args[0])
		return
	}

	tamanho := 0
	numeroDeTestes := 0

	_, err1 := fmt.Sscanf(os.Args[1], "%d", &tamanho)
	_, err2 := fmt.Sscanf(os.Args[2], "%d", &numeroDeTestes)

	if err1 != nil || err2 != nil || tamanho <= 0 || numeroDeTestes <= 0 {
		fmt.Println("N e M devem ser maiores que 0.")
		return
	}

	// Generate 10 random test cases
	testCases := make([]struct {
		A [][]float64
		b []float64
	}, 10)

	for t := 0; t < 10; t++ {
		A := make([][]float64, tamanho)
		b := make([]float64, tamanho)
		for i := 0; i < tamanho; i++ {
			A[i] = make([]float64, tamanho)
			for j := 0; j < tamanho; j++ {
				A[i][j] = rand.Float64()*20 - 10 // Random values between -10 and 10
			}
			b[i] = rand.Float64()*20 - 10 // Random values between -10 and 10
		}
		testCases[t] = struct {
			A [][]float64
			b []float64
		}{A: A, b: b}
	}

	// Run one random test case numeroDeTestes times
	for i := 0; i < numeroDeTestes; i++ {
		tc := testCases[rand.Intn(len(testCases))]
		_ = gaussSolver(len(tc.A), tc.A, tc.b)
	}
}
