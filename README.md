# Read text file by console or dialog
    
## Ibrid application:
- It works like CLI, when there is first argument.
- Otherwise, open a GUI dialog for pick a file.

## Cross platform
Use structopt for read args.
Use native-dialog for simple GUI.

## How it works:
- Read first argument like path file
- If occurs error while reading a file print an error message
- If program was execute from GUI, show alert error message
- When file exists, print lines in console
