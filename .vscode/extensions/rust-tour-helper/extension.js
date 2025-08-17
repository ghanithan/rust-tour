const vscode = require('vscode');
const fs = require('fs');
const path = require('path');

/**
 * Rust Tour Helper Extension
 * Provides setup completion notifications and quick access commands
 */

let statusBarItem;

function activate(context) {
    console.log('Rust Tour Helper extension is now active');

    // Create status bar item
    statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Left, 100);
    statusBarItem.command = 'rust-tour.startTour';
    statusBarItem.text = '$(rocket) Rust Tour';
    statusBarItem.tooltip = 'Start Rust Tour Learning Platform';
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);

    // Register commands
    const startTourCommand = vscode.commands.registerCommand('rust-tour.startTour', () => {
        runWelcomeScript();
    });

    const openWelcomeCommand = vscode.commands.registerCommand('rust-tour.openWelcome', () => {
        runWelcomeScript();
    });

    context.subscriptions.push(startTourCommand, openWelcomeCommand);

    // Check for setup completion on activation (handles restarts)
    checkSetupCompletion(context);

    // Watch for setup completion file
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (workspaceFolder) {
        const statusFilePath = path.join(workspaceFolder.uri.fsPath, '.setup-complete');
        
        // Create file system watcher
        const watcher = vscode.workspace.createFileSystemWatcher(
            new vscode.RelativePattern(workspaceFolder, '.setup-complete')
        );

        watcher.onDidCreate(() => {
            console.log('Setup completion file created - showing notification');
            showSetupCompleteNotification(context);
        });

        context.subscriptions.push(watcher);
    }
}

function checkSetupCompletion(context) {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) return;

    const statusFilePath = path.join(workspaceFolder.uri.fsPath, '.setup-complete');
    const binaryPath = path.join(workspaceFolder.uri.fsPath, 'bin', 'rust-tour');

    if (fs.existsSync(statusFilePath) || fs.existsSync(binaryPath)) {
        // Check if we've already shown the notification this session
        const hasShownWelcome = context.workspaceState.get('rust-tour.welcomeShown', false);
        
        if (!hasShownWelcome) {
            console.log('Setup appears complete, showing notification');
            showSetupCompleteNotification(context);
        } else {
            console.log('Setup complete but notification already shown this session');
        }
    }
}

function showSetupCompleteNotification(context) {
    // Mark that we've shown the welcome notification
    context.workspaceState.update('rust-tour.welcomeShown', true);

    vscode.window.showInformationMessage(
        '🎉 Rust Tour setup complete! Ready to start learning Rust?',
        'Start Learning',
        'Later'
    ).then(selection => {
        if (selection === 'Start Learning') {
            runWelcomeScript();
        }
    });

    // Update status bar to show ready state
    if (statusBarItem) {
        statusBarItem.text = '$(check) Rust Tour Ready';
        statusBarItem.tooltip = 'Rust Tour is ready - click to start!';
    }
}

function runWelcomeScript() {
    const workspaceFolder = vscode.workspace.workspaceFolders?.[0];
    if (!workspaceFolder) {
        vscode.window.showErrorMessage('No workspace folder found');
        return;
    }

    const welcomeScriptPath = path.join(workspaceFolder.uri.fsPath, 'scripts', 'welcome.sh');
    
    if (!fs.existsSync(welcomeScriptPath)) {
        vscode.window.showErrorMessage('Welcome script not found at scripts/welcome.sh');
        return;
    }

    // Create and show terminal
    const terminal = vscode.window.createTerminal({
        name: 'Rust Tour',
        cwd: workspaceFolder.uri.fsPath
    });
    
    terminal.show();
    terminal.sendText('./scripts/welcome.sh');
}

function deactivate() {
    if (statusBarItem) {
        statusBarItem.dispose();
    }
}

module.exports = {
    activate,
    deactivate
};