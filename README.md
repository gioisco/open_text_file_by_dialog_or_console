# Read text file by console or dialog
    
## Ibrid application:
Like CLI, use structopt for read args, when there is first argument.

Otherwise, use native-dialog for open a GUI dialog for pick a file.

## How it works:
- Read first argument like path file
- If occurs error while reading a file print an error message
- If program was execute from GUI, show alert error message
- When file exists, print lines in console
