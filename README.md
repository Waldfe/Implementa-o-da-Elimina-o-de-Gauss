# Implementação-da-Eliminação-de-Gauss
## Integrantes do grupo:
- Felipe Leonardo Kerwald Santana
- Kalani Sosa Pereira
- Lucas Superti da Silva

## Compilação e utilização
### Utilização genérica
Todas as implementações podem ser executadas com

`$ [executável] <tamanho da matriz> <número de execuções>`.

### Utilização do script de teste
A sequência de testes consiste em matrizes de tamanho 3, 5, 7 e 9 com 100 milhões de execuções de cada tamanho em cada build. Os resultados são adicionados ao arquivo `resultados.txt`. O arquivo de resultados não é truncado no início da execução da sequência de testes, então testes sucessivos serão armazenados em sequência no arquivo. Com o terminal na pasta raiz deste repositório é possível executar a sequência de testes executando o script `testes.py`

`$ python testes.py`.

### Compilação
Binários para Windows já são disponibilizados neste repositório. Para realizar a compilação siga as seguintes instruções para cada linguagem:
#### C
Para compilar utilizando o GCC entre no diretório da implementação em C com `$ cd C` e execute `$ gcc ./eliminacaodegauss.c -O0 -o ./unoptimized` para uma build não otimizada e `$ gcc ./eliminacaodegauss.c -O3 -o ./optimized` para uma build otimizada.
#### Go
Para compilar utilizando o sistema de build da implementação de referência do Go ente no diretório da implementação em Go com `$ cd Go` e execute `$ go build -gcflags "-N -l" -o unoptimized` para uma build não otimizada e `$ go build -o optimized` para uma build otimizada.
#### Rust
Para compilar utilizando o Cargo entre no diretório da implementação em Rust com `$ cd Rust` e execute `$ cargo build` para uma build não otimizada e `$ cargo build --release` para uma build otimizada. Os executáveis da versão não otimizada e otimizada estarão em `./target/debug/eliminacaodegauss` e `./target/release/eliminacaodegauss`, respectivamente.
