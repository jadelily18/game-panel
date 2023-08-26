Start-Process powershell {
    docker build -t game-panel-frontend .\frontend
}

Start-Process powershell {
    docker build -t game-panel-backend .\backend
}
