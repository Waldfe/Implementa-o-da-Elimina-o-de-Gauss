$resultsFile = "resultado.txt"

Add-Content -Path $resultsFile -Value "C não otimizado:"
Add-Content -Path $resultsFile -Value (Measure-Command {.\C\unoptimized.exe}).ToString()
Add-Content -Path $resultsFile -Value "C otimizado (-O3):"
Add-Content -Path $resultsFile -Value (Measure-Command {.\C\optimized.exe}).ToString()

Add-Content -Path $resultsFile -Value "Go não otimizado (-gcflags `"-N -l`"):"
Add-Content -Path $resultsFile -Value (Measure-Command {.\Go\unoptimized.exe}).ToString()
Add-Content -Path $resultsFile -Value "Go otimizado:"
Add-Content -Path $resultsFile -Value (Measure-Command {.\Go\optimized.exe}).ToString()

Add-Content -Path $resultsFile -Value "Rust debug:"
Add-Content -Path $resultsFile -Value (Measure-Command {.\Rust\target\debug\eliminacaodegauss.exe}).ToString()
Add-Content -Path $resultsFile -Value "Rust release (opt-level = 3):"
Add-Content -Path $resultsFile -Value (Measure-Command {.\Rust\target\release\eliminacaodegauss.exe}).ToString()
