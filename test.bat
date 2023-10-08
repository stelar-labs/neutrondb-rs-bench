@echo off
setlocal

set "folderToDelete=test"

rem Check if the folder exists
if exist "%folderToDelete%" (
    rem Delete the folder and its contents
    rmdir /s /q "%folderToDelete%"
    if errorlevel 1 (
        echo Failed to delete "%folderToDelete%" folder.
        exit /b 1
    ) else (
        echo "%folderToDelete%" folder deleted successfully.
    )
) else (
    echo "%folderToDelete%" folder not found.
)

rem Run the cargo run command
cargo run

rem Pause to keep the command prompt window open
pause

endlocal
