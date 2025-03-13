//GitHub: HenriqueIni
//www.blogcyberini.com

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
void gaussSolver(int n, double A[n][n], double b[n], double X[n]) {
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
int main() {
    srand(time(NULL));
    int n;
    double A[5][5], b[5], x[5];
    for (int iter = 0; iter < 1000000000; iter++) {
        int testCase = rand() % 6;
        switch (testCase) {
            case 0:
                n = 3;
                double A0[3][3] = {{2, 1, -1}, {1, 2, 1}, {1, 1, 1}};
                double b0[3] = {-3, 3, 2};
                memcpy(A, A0, sizeof(A0));
                memcpy(b, b0, sizeof(b0));
                break;
            case 1:
                n = 3;
                double A1[3][3] = {{1, 1, 1}, {-2, 1, 1}, {1, 3, 1}};
                double b1[3] = {2, 5, 4};
                memcpy(A, A1, sizeof(A1));
                memcpy(b, b1, sizeof(b1));
                break;
            case 2:
                n = 3;
                double A2[3][3] = {{3, 2, -1}, {2, -2, 4}, {-1, 0.5, -1}};
                double b2[3] = {1, -2, 0};
                memcpy(A, A2, sizeof(A2));
                memcpy(b, b2, sizeof(b2));
                break;
            case 3:
                n = 2;
                double A3[2][2] = {{2, 3}, {4, 9}};
                double b3[2] = {6, 15};
                memcpy(A, A3, sizeof(A3));
                memcpy(b, b3, sizeof(b3));
                break;
            case 4:
                n = 4;
                double A4[4][4] = {{4, 1, 2, -3}, {-3, 3, -1, 4}, {-1, 2, 5, 1}, {5, 4, 3, -1}};
                double b4[4] = {-16, 20, -4, -10};
                memcpy(A, A4, sizeof(A4));
                memcpy(b, b4, sizeof(b4));
                break;
            case 5:
                n = 5;
                double A5[5][5] = {{4, 1, 2, -3, 5}, {-3, 3, -1, 4, -2}, {-1, 2, 5, 1, 3}, {5, 4, 3, -1, 2}, {1, -2, 3, -4, 5}};
                double b5[5] = {-16, 20, -4, -10, 3};
                memcpy(A, A5, sizeof(A5));
                memcpy(b, b5, sizeof(b5));
                break;
        }
        gaussSolver(n, A, b, x);
    }
    return 0;
}