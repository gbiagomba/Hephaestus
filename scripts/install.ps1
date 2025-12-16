#Requires -Version 5.1
<#
.SYNOPSIS
    Hephaestus Installation Script for Windows

.DESCRIPTION
    This script will:
    1. Detect the OS architecture (x64/ARM64)
    2. Install dependencies (Rust, git) using winget or chocolatey
    3. Build and install Hephaestus from source

.NOTES
    Requires PowerShell 5.1 or higher
    May require Administrator privileges for some operations
#>

[CmdletBinding()]
param()

# Set error action preference
$ErrorActionPreference = "Stop"

# Colors for output
function Write-Info {
    param([string]$Message)
    Write-Host "[INFO] $Message" -ForegroundColor Green
}

function Write-Warn {
    param([string]$Message)
    Write-Host "[WARN] $Message" -ForegroundColor Yellow
}

function Write-Error-Custom {
    param([string]$Message)
    Write-Host "[ERROR] $Message" -ForegroundColor Red
    exit 1
}

# Detect architecture
function Get-Architecture {
    $arch = $env:PROCESSOR_ARCHITECTURE
    switch ($arch) {
        "AMD64" { return "x64" }
        "ARM64" { return "arm64" }
        default {
            Write-Error-Custom "Unsupported architecture: $arch"
        }
    }
}

# Check if a command exists
function Test-CommandExists {
    param([string]$Command)
    $null = Get-Command $Command -ErrorAction SilentlyContinue
    return $?
}

# Check if running as Administrator
function Test-IsAdmin {
    $currentUser = [Security.Principal.WindowsIdentity]::GetCurrent()
    $principal = New-Object Security.Principal.WindowsPrincipal($currentUser)
    return $principal.IsInRole([Security.Principal.WindowsBuiltInRole]::Administrator)
}

# Install Rust using rustup
function Install-Rust {
    if (Test-CommandExists rustc) {
        $rustVersion = (rustc --version).Split()[1]
        Write-Info "Rust is already installed (version $rustVersion)"
        return
    }

    Write-Info "Installing Rust via rustup..."

    # Download rustup-init.exe
    $rustupUrl = "https://win.rustup.rs/x86_64"
    if ((Get-Architecture) -eq "arm64") {
        $rustupUrl = "https://win.rustup.rs/aarch64"
    }

    $rustupPath = "$env:TEMP\rustup-init.exe"

    try {
        Write-Info "Downloading rustup..."
        Invoke-WebRequest -Uri $rustupUrl -OutFile $rustupPath -UseBasicParsing

        Write-Info "Running rustup installer..."
        Start-Process -FilePath $rustupPath -ArgumentList "-y" -Wait -NoNewWindow

        # Update PATH for current session
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"

        Write-Info "Rust installed successfully"
    }
    catch {
        Write-Error-Custom "Failed to install Rust: $_"
    }
    finally {
        if (Test-Path $rustupPath) {
            Remove-Item $rustupPath -Force
        }
    }
}

# Install git using package managers
function Install-Git {
    if (Test-CommandExists git) {
        $gitVersion = (git --version).Split()[2]
        Write-Info "Git is already installed (version $gitVersion)"
        return
    }

    Write-Info "Installing git..."

    # Try winget first (Windows 10 1809+ / Windows 11)
    if (Test-CommandExists winget) {
        Write-Info "Installing git using winget..."
        try {
            winget install --id Git.Git -e --source winget --silent --accept-package-agreements --accept-source-agreements
            # Refresh environment variables
            $env:PATH = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
            Write-Info "Git installed successfully via winget"
            return
        }
        catch {
            Write-Warn "winget installation failed, trying chocolatey..."
        }
    }

    # Try chocolatey
    if (Test-CommandExists choco) {
        Write-Info "Installing git using chocolatey..."
        try {
            choco install git -y
            # Refresh environment variables
            $env:PATH = [System.Environment]::GetEnvironmentVariable("Path", "Machine") + ";" + [System.Environment]::GetEnvironmentVariable("Path", "User")
            Write-Info "Git installed successfully via chocolatey"
            return
        }
        catch {
            Write-Warn "chocolatey installation failed..."
        }
    }
    else {
        Write-Info "Chocolatey not found. Installing Chocolatey..."
        if (-not (Test-IsAdmin)) {
            Write-Error-Custom "Administrator privileges required to install Chocolatey. Please run PowerShell as Administrator."
        }

        try {
            Set-ExecutionPolicy Bypass -Scope Process -Force
            [System.Net.ServicePointManager]::SecurityProtocol = [System.Net.ServicePointManager]::SecurityProtocol -bor 3072
            Invoke-Expression ((New-Object System.Net.WebClient).DownloadString('https://community.chocolatey.org/install.ps1'))

            # Refresh environment
            $env:ChocolateyInstall = Convert-Path "$((Get-Command choco).Path)\..\.."
            Import-Module "$env:ChocolateyInstall\helpers\chocolateyProfile.psm1"
            refreshenv

            # Install git
            choco install git -y
            Write-Info "Git installed successfully via chocolatey"
            return
        }
        catch {
            Write-Error-Custom "Failed to install Chocolatey and git: $_"
        }
    }

    # Manual download as last resort
    Write-Warn "Package managers failed. Please install git manually from: https://git-scm.com/download/win"
    Write-Error-Custom "Git installation required to continue."
}

# Install Visual Studio Build Tools (required for some Rust crates)
function Install-BuildTools {
    Write-Info "Checking for Visual Studio Build Tools..."

    # Check if Visual Studio or Build Tools are already installed
    $vsWhere = "${env:ProgramFiles(x86)}\Microsoft Visual Studio\Installer\vswhere.exe"
    if (Test-Path $vsWhere) {
        $vsInstall = & $vsWhere -latest -property installationPath
        if ($vsInstall) {
            Write-Info "Visual Studio Build Tools already installed"
            return
        }
    }

    Write-Warn "Visual Studio Build Tools not found."
    Write-Info "These are required for building Rust projects on Windows."
    Write-Info "You can install them using:"
    Write-Info "  winget install Microsoft.VisualStudio.2022.BuildTools"
    Write-Info "Or download from: https://visualstudio.microsoft.com/downloads/"

    $response = Read-Host "Would you like to install Visual Studio Build Tools now? (y/N)"
    if ($response -eq 'y' -or $response -eq 'Y') {
        if (Test-CommandExists winget) {
            Write-Info "Installing Visual Studio Build Tools..."
            winget install Microsoft.VisualStudio.2022.BuildTools --silent --override "--wait --quiet --add Microsoft.VisualStudio.Workload.VCTools --includeRecommended"
        }
        else {
            Write-Warn "winget not available. Please install Visual Studio Build Tools manually."
        }
    }
}

# Build and install Hephaestus
function Build-And-Install {
    Write-Info "Building Hephaestus from source..."

    # Ensure cargo is in PATH
    if (-not (Test-CommandExists cargo)) {
        $env:PATH = "$env:USERPROFILE\.cargo\bin;$env:PATH"
    }

    if (-not (Test-CommandExists cargo)) {
        Write-Error-Custom "Cargo not found. Rust installation may have failed."
    }

    try {
        # Build in release mode
        Write-Info "Running: cargo build --release"
        cargo build --release

        Write-Info "Installing Hephaestus..."
        cargo install --path . --force

        # Verify installation
        if (Test-CommandExists hephaestus) {
            $version = (hephaestus --version).Split()[1]
            Write-Info "Hephaestus $version installed successfully!"
            Write-Info "Run 'hephaestus --help' to get started"
        }
        else {
            Write-Warn "Hephaestus binary installed but not in PATH."
            Write-Warn "Add $env:USERPROFILE\.cargo\bin to your PATH environment variable."
        }
    }
    catch {
        Write-Error-Custom "Build failed: $_"
    }
}

# Main installation flow
function Main {
    Write-Host "========================================"
    Write-Host "  Hephaestus Installation Script"
    Write-Host "========================================"
    Write-Host ""

    $arch = Get-Architecture
    Write-Info "Detected architecture: Windows-$arch"

    Install-Git
    Install-Rust
    Install-BuildTools
    Build-And-Install

    Write-Host ""
    Write-Host "========================================"
    Write-Host "  Installation Complete!"
    Write-Host "========================================"
    Write-Host ""
    Write-Info "Hephaestus has been installed to: $env:USERPROFILE\.cargo\bin"
    Write-Info "This location should be automatically added to your PATH."
    Write-Info "You may need to restart your terminal for the changes to take effect."
    Write-Host ""
}

# Run main installation
Main
