param(
    [string]$ProjectRoot = $PSScriptRoot,
    [string]$GlobalAgentsRoot = (Join-Path $HOME ".agents\personas-global"),
    [string]$GlobalSkillsRoot = (Join-Path $HOME ".codex\skills"),
    [switch]$Apply,
    [switch]$Prune,
    [switch]$IncludeSystemSkills
)

Set-StrictMode -Version Latest
$ErrorActionPreference = "Stop"

function Invoke-Robocopy {
    param(
        [Parameter(Mandatory = $true)][string]$Source,
        [Parameter(Mandatory = $true)][string]$Destination,
        [Parameter(Mandatory = $true)][string[]]$Options
    )

    if (-not (Test-Path -LiteralPath $Source)) {
        throw "Source path not found: $Source"
    }

    New-Item -ItemType Directory -Path $Destination -Force | Out-Null

    $args = @($Source, $Destination) + $Options
    & robocopy @args | Out-Host
    $code = $LASTEXITCODE
    if ($code -gt 7) {
        throw "Robocopy failed with exit code $code for source '$Source' and destination '$Destination'."
    }
}

$resolvedProjectRoot = (Resolve-Path -LiteralPath $ProjectRoot).Path
$resolvedGlobalAgentsRoot = (Resolve-Path -LiteralPath $GlobalAgentsRoot).Path
$resolvedGlobalSkillsRoot = (Resolve-Path -LiteralPath $GlobalSkillsRoot).Path

$projectAgentsRoot = Join-Path $resolvedProjectRoot ".agents"
$projectSkillsRoot = Join-Path $projectAgentsRoot "skills"

$modeText = if ($Apply) { "APPLY" } else { "PREVIEW" }
Write-Host "Sync mode: $modeText"
Write-Host "Project root: $resolvedProjectRoot"
Write-Host "Global agents root: $resolvedGlobalAgentsRoot"
Write-Host "Global skills root: $resolvedGlobalSkillsRoot"
Write-Host ""

$common = @("/E", "/R:1", "/W:1", "/FFT", "/NFL", "/NDL", "/NP")
if (-not $Apply) { $common += "/L" }
if ($Prune) { $common += "/MIR" }

Write-Host "== Sync agents/rules =="
Invoke-Robocopy -Source $resolvedGlobalAgentsRoot -Destination $projectAgentsRoot -Options $common

Write-Host ""
Write-Host "== Sync skills =="
if ($IncludeSystemSkills) {
    Invoke-Robocopy -Source $resolvedGlobalSkillsRoot -Destination $projectSkillsRoot -Options $common
} else {
    Get-ChildItem -LiteralPath $resolvedGlobalSkillsRoot -Directory |
        Where-Object { $_.Name -ne ".system" } |
        ForEach-Object {
            $src = $_.FullName
            $dst = Join-Path $projectSkillsRoot $_.Name
            Write-Host "-- Skill: $($_.Name)"
            Invoke-Robocopy -Source $src -Destination $dst -Options $common
        }
}

Write-Host ""
if ($Apply) {
    Write-Host "Done. Global agents/rules/skills synced to project."
    Write-Host "Destination:"
    Write-Host "  $projectAgentsRoot"
    Write-Host "  $projectSkillsRoot"
} else {
    Write-Host "Preview complete. Re-run with -Apply to perform the sync."
}
