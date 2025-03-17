import time
import subprocess

comandos_otimizado = {
    "C": "./C/optimized",
    "Go": "./Go/optimized",
    "Rust":"./Rust/target/release/eliminacaodegauss"
}

comandos_nao_otimizado = {
    "C": "./C/unoptimized",
    "Go": "./Go/unoptimized",
    "Rust": "./Rust/target/debug/eliminacaodegauss"
}

tamanhos = [3, 5, 7, 9]
número_de_execuções = 100000000

arquivo_saida = "resultados.txt"


# Otimizado
with open(arquivo_saida, "a") as f:
    f.write("Otimizado:\n\n")

for linguagem, comando in comandos_otimizado.items():
    with open(arquivo_saida, "a") as f:
        f.write(f"{linguagem}:\n")
    
    for tamanho in tamanhos:
        with open(arquivo_saida, "a") as f:
            f.write(f"Tamanho {tamanho}: ")
        
        inicio = time.time()
        subprocess.run([comando, str(tamanho), str(número_de_execuções)], stdout=subprocess.PIPE)
        fim = time.time()
        
        with open(arquivo_saida, "a") as f:
            f.write(f"{fim-inicio}\n")
    
    with open(arquivo_saida, "a") as f:
        f.write("\n")

with open(arquivo_saida, "a") as f:
    f.write("\n")

# Não otimizado
with open(arquivo_saida, "a") as f:
    f.write("Não otimizado:\n\n")

for linguagem, comando in comandos_nao_otimizado.items():
    with open(arquivo_saida, "a") as f:
        f.write(f"{linguagem}:\n")
    
    for tamanho in tamanhos:
        with open(arquivo_saida, "a") as f:
            f.write(f"Tamanho {tamanho}: ")
        
        inicio = time.time()
        subprocess.run([comando, str(tamanho), str(número_de_execuções)], stdout=subprocess.PIPE)
        fim = time.time()
        
        with open(arquivo_saida, "a") as f:
            f.write(f"{fim-inicio}\n")
    
    with open(arquivo_saida, "a") as f:
        f.write("\n")