cargo build -q --release
cargo build -q

$files_release = Get-ChildItem -Path "./target/release/" -Include "*.exe" -Depth 0
$files_debug = Get-ChildItem -Path "./target/debug/" -Include "*.exe" -Depth 0

Write-Host "-> Release <-"
for ($i = 0; $i -lt $files_release.Count; $i += 1) {
    # Do two dry-runs
    for ($j = 0; $j -lt 2; $j += 1) {
        & $files_release[$i].FullName > $null
    }
    # Output the third run
    & $files_release[$i].FullName
}
Write-Host

Write-Host "-> Debug <-"
for ($i = 0; $i -lt $files_debug.Count; $i += 1) {
    # Do two dry-runs
    for ($j = 0; $j -lt 2; $j += 1) {
        & $files_debug[$i].FullName > $null
    }
    # Output the third run
    & $files_debug[$i].FullName
}
