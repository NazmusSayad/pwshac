# pwshac

Reads current PowerShell aliases and prints remove commands.

## Usage

```bash
pwshac [alias_to_keep ...]
```

Examples:

```bash
pwshac
pwshac cd ls
```

Example output shape for `pwshac cd ls`:

```powershell
Remove-Alias -Name @('...all aliases except cd/ls...') -Force -ErrorAction SilentlyContinue
Remove-Alias -Name @('...aliases with non-empty Source except cd/ls...') -Force -ErrorAction SilentlyContinue
```

You can execute the output in PowerShell.
