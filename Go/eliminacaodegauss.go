package main

import (
	"fmt"
	"math"
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
			b[i] += factor *