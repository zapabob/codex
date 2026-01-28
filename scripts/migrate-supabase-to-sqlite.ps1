# Supabase to SQLite Migration Script
# This script helps migrate data from Supabase to SQLite

param(
    [string]$SupabaseUrl = "",
    [string]$SupabaseKey = "",
    [string]$DbPath = "codex-gui.db"
)

Write-Host "=== Supabase to SQLite Migration ===" -ForegroundColor Cyan

if (-not $SupabaseUrl -or -not $SupabaseKey) {
    Write-Host "Error: Supabase URL and Key are required" -ForegroundColor Red
    Write-Host "Usage: .\migrate-supabase-to-sqlite.ps1 -SupabaseUrl <url> -SupabaseKey <key>" -ForegroundColor Yellow
    exit 1
}

# Check if sqlx CLI is installed
$sqlxInstalled = Get-Command sqlx -ErrorAction SilentlyContinue
if (-not $sqlxInstalled) {
    Write-Host "Installing sqlx CLI..." -ForegroundColor Yellow
    cargo install sqlx-cli --features sqlite
}

# Create SQLite database
Write-Host "Creating SQLite database..." -ForegroundColor Cyan
sqlx database create --database-url "sqlite:$DbPath"

# Create tables
Write-Host "Creating tables..." -ForegroundColor Cyan
sqlx migrate run --database-url "sqlite:$DbPath" --source migrations

# Export data from Supabase (manual step)
Write-Host "`n=== Manual Steps Required ===" -ForegroundColor Yellow
Write-Host "1. Export Plans data from Supabase:" -ForegroundColor White
Write-Host "   - Go to Supabase Dashboard > Table Editor > plans" -ForegroundColor Gray
Write-Host "   - Export as CSV or JSON" -ForegroundColor Gray
Write-Host ""
Write-Host "2. Import Plans data to SQLite:" -ForegroundColor White
Write-Host "   sqlite3 $DbPath" -ForegroundColor Gray
Write-Host "   .mode csv" -ForegroundColor Gray
Write-Host "   .import plans.csv plan_metadata" -ForegroundColor Gray
Write-Host ""
Write-Host "3. Export Users data from Supabase (if needed):" -ForegroundColor White
Write-Host "   - Go to Supabase Dashboard > Authentication > Users" -ForegroundColor Gray
Write-Host "   - Export user data" -ForegroundColor Gray
Write-Host ""
Write-Host "4. Import Users data to SQLite:" -ForegroundColor White
Write-Host "   sqlite3 $DbPath" -ForegroundColor Gray
Write-Host "   .mode csv" -ForegroundColor Gray
Write-Host "   .import users.csv users" -ForegroundColor Gray
Write-Host ""
Write-Host "Note: Password hashes need to be re-hashed with bcrypt" -ForegroundColor Yellow
Write-Host "      Sessions will need to be recreated after migration" -ForegroundColor Yellow

Write-Host "`nMigration script completed!" -ForegroundColor Green
