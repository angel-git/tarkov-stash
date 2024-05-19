param (
    [string]$url,
    [string]$outputPath,
    [string]$accessToken
)

$headers = @{
    Authorization = "token $accessToken"
}

try {
    Invoke-WebRequest -Uri $url -OutFile $outputPath -Headers $headers -UseBasicParsing -ErrorAction Stop
    Write-Output "File downloaded successfully to $outputPath"
} catch {
    Write-Error "An error occurred while downloading the file: $_"
}
