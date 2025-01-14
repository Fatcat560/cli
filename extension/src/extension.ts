import * as vscode from 'vscode';
import { spawn } from "child_process";

export function activate(context: vscode.ExtensionContext) {
	function registerCommand(cmd: string) {
		function convert(cmd: string) {
			const editor = vscode.window.activeTextEditor;// Get the active text editor
			if (editor) {
				const html = editor.document.getText(editor.selection);
				if (html.length > 0) {
					const params = ["translate"];
					if (cmd.includes("Component")) params.push("--component");
					params.push("--source");
					params.push(html);
					const child_proc = spawn("dioxus", params);
					let result = '';
					child_proc.stdout?.on('data', data => result += data);
					child_proc.on('close', () => {
						if (result.length > 0) editor.edit(editBuilder => editBuilder.replace(editor.selection, result));
					});
				} else {
					vscode.window.showWarningMessage("Please select HTML fragment before invoking this command!");
				}
			}
		}
		const handle = vscode.commands.registerCommand(cmd, () => convert(cmd));
		context.subscriptions.push(handle);
	}

	registerCommand('extension.htmlToDioxusRsx');
	registerCommand('extension.htmlToDioxusComponent');
	context.subscriptions.push(vscode.commands.registerCommand('extension.addDioxusComponent', (...p) => AddDioxusComponent(p)));

}

function AddDioxusComponent(p: any[]) {
	if (p && typeof p[0] == 'object' && p[0]['path']) {
		const file: vscode.Uri = p[0];
		vscode.workspace.fs.stat(file).then((fStat) => {
			if (fStat.type == vscode.FileType.Directory) {
				vscode.window.showInputBox({ title: 'Select a name' }).then((value) => {
					if (value) {
						const params: string[] = ["add", "component", file.fsPath, value];
						const child_proc = spawn('dioxus', params);
						let err_txt = '';
						let success_txt = '';
						child_proc.stderr.on('data', d => err_txt += d);
						child_proc.stdout.on('data', d => success_txt += d);
						child_proc.on('close', (code) => {
							if (code !== 0) {
								vscode.window.showErrorMessage(err_txt);
								return;
							} else {
								vscode.window.showInformationMessage(success_txt);
							}
						});
					}
				});
			}
		});
	}
}
