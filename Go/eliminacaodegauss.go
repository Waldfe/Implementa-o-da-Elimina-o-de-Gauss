package main

import (
	"fmt"
	"math"
	"math/rand"
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
	testCases := []struct {
		A [][]float64
		b []float64
	}{
		{
			A: [][]float64{
				{2, 1, -1},
				{1, 2, 1},
				{1, 1, 1},
			},
			b: []float64{-3, 3, 2},
		},
		{
			A: [][]float64{
				{1, 1, 1},
				{-2, 1, 1},
				{1, 3, 1},
			},
			b: []float64{2, 5, 4},
		},
		{
			A: [][]float64{
				{3, 2, -1},
				{2, -2, 4},
				{-1, 0.5, -1},
			},
			b: []float64{1, -2, 0},
		},
		{
			A: [][]float64{
				{2, 3},
				{4, 9},
			},
			b: []float64{6, 15},
		},
		{
			A: [][]float64{
				{4, 1, 2, -3},
				{-3, 3, -1, 4},
				{-1, 2, 5, 1},
				{5, 4, 3, -1},
			},
			b: []float64{-16, 20, -4, -10},
		},
		{
			A: [][]float64{
				{4, 1, 2, -3, 5},
				{-3, 3, -1, 4, -2},
				{-1, 2, 5, 1, 3},
				{5, 4, 3, -1, 2},
				{1, -2, 3, -4, 5},
			},
			b: []float64{-16, 20, -4, -10, 3},
		},
	}

	for i := 0; i < 1_000_000_000; i++ {
		tc := testCases[rand.Intn(len(testCases))]
		_ = gaussSolver(len(tc.A), tc.A, tc.b)
	}
}