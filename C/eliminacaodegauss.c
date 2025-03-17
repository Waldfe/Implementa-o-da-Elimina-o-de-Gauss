// Modificado de:
// GitHub: HenriqueIni
// www.blogcyberini.com

#include <stdio.h>
#include <stdlib.h>
#include <math.h>
#include <time.h>
#include <string.h>


/*
 * Algoritmo para resolução de sistemas lineares via eliminação de Gauss
 * Complexidade no tempo: O(n^3)
 * A é a matriz dos coeficientes do sistema
 * b é a matriz dos coeficientes dos termos independentes
 * n é a ordem do sistema
 * X é o vetor onde a solução será armazenada
 * Forma do sistema (matricial): Ax = b  
 */
#ifndef __INTELLISENSE__
 void gaussSolver(int n, double A[n][n], double b[n], double X[n]) {
#else
    void gaussSolver(int n, double A[1][1], double b[1], double X[1]) {
#endif
    int i, j, k, l, m;
    //ETAPA DE ESCALONAMENTO
    for (k = 0; k < n - 1; k++) {
        double max = fabs(A[k][k]);
        int maxIndex = k;
        //procura o maior k-ésimo coeficiente em módulo
        for (i = k + 1; i < n; i++) {
            if (max < fabs(A[i][k])) {
                max = fabs(A[i][k]);
                maxIndex = i;
            }
        }
        if (maxIndex != k) {
            /*
             troca a equação k pela equação com o
             maior k-ésimo coeficiente em módulo
             */
            for (j = 0; j < n; j++) {
                double temp = A[k][j];
                A[k][j] = A[maxIndex][j];
                A[maxIndex][j] = temp;
            }
            double temp = b[k];
            b[k] = b[maxIndex];
            b[maxIndex] = temp;
        }
        //Se A[k][k] é zero, então a matriz dos coeficiente é singular
        //det A = 0
        if (A[k][k] == 0) {
            printf("A matriz dos coeficientes é singular\n");
            return;
        } else {
            //realiza o escalonamento
            for (m = k + 1; m < n; m++) {
                double F = -A[m][k] / A[k][k];
                A[m][k] = 0; //evita uma iteração
                b[m] = b[m] + F * b[k];
                for (l = k + 1; l < n; l++) {
                    A[m][l] = A[m][l] + F * A[k][l];
                }
            }
        }
    }
    //ETAPA DE RESOLUÇÃO DO SISTEMA
    for (i = n - 1; i >= 0; i--) {
        X[i] = b[i];
        for (j = i + 1; j < n; j++) {
            X[i] = X[i] - X[j] * A[i][j];
        }
        X[i] = X[i] / A[i][i];
    }
}

//Código de testes
int main(int argc, char *argv[]) {
    if (argc != 3) {
        printf("Uso: %s <tamanho> <número de testes>\n", argv[0]);
        return 1;
    }

    long long tamanho = strtol(argv[1], NULL, 10);
    long long numeroDeTestes = strtol(argv[2], NULL, 10);

    if (tamanho <= 0 || numeroDeTestes <= 0) {
        printf("N e M devem ser maiores que 0.\n");
        return 1;
    }

    srand(time(NULL));

    #ifndef __INTELLISENSE__
    double testCases[10][tamanho][tamanho];
    double testB[10][tamanho];
    #else
    double testCases[10][1][1];
    double testB[10][1];
    #endif

    // Gerar 10 casos de teste aleatórios
    for (int t = 0; t < 10; t++) {
        for (int i = 0; i < tamanho; i++) {
            for (int j = 0; j < tamanho; j++) {
                testCases[t][i][j] = (rand() % 2001 - 1000) / 100.0; // valores entre -10.00 e 10.00
            }
            testB[t][i] = (rand() % 2001 - 1000) / 100.0; // valores entre -10.00 e 10.00
        }
    }

    #ifndef __INTELLISENSE__
    double A[tamanho][tamanho], b[tamanho], x[tamanho];
    #else
    double A[1][1], b[1], x[1];
    #endif

    // Executar numeroDeTestes vezes escolhendo um caso de teste aleatório
    for (int iter = 0; iter < numeroDeTestes; iter++) {
        int testCase = rand() % 10;
        memcpy(A, testCases[testCase], sizeof(A));
        memcpy(b, testB[testCase], sizeof(b));

        gaussSolver(tamanho, A, b, x);
    }

    return 0;
}
