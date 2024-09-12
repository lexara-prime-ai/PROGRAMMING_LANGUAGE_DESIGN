# Get the current system PATH
$currentPath = [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)

# Define the old and new paths
$oldPath = "C:\Program Files\LLVM\bin"
$newPath = "C:\LLVM\bin"

# Check if the old path exists in the PATH variable
if ($currentPath.Contains($oldPath)) {
    # Replace the old path with the new one
    $updatedPath = $currentPath -replace [regex]::Escape($oldPath), $newPath

    # Update the PATH in the system environment variables
    [System.Environment]::SetEnvironmentVariable("Path", $updatedPath, [System.EnvironmentVariableTarget]::Machine)

    # Output the updated PATH to verify
    [System.Environment]::GetEnvironmentVariable("Path", [System.EnvironmentVariableTarget]::Machine)
}
else {
    Write-Host "The specified old path does not exist in the system PATH."
}
