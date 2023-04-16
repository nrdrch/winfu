## winfu - Commandline Function Manager For Windows! 
#### Type less and get superior functionality. 
![Example](https://github.com/jds4nrdrch/pics/blob/main/example2.png)

### Potential Dependencies:
- [Cargo & Rust:](https://doc.rust-lang.org/cargo/getting-started/installation.html)
- [Git for Windows](https://gitforwindows.org/)
### Installation:
- Normal Method
```
cargo install winfu
```
- Alternative Method
```
git clone https://github.com/nrdrch/winfu.git
```
```
cd winfu
```
```
cargo build --release
```
- Preferably move the executable from target/release into a directory in your 'Path' enviorment variable for easy execution.

### Don't know how to add a directory to your 'Path' system variable?
1. Press **Win+R** on your Keyboard and enter this to open the Advanced System Propersties 
```
C:\Windows\System32\SystemPropertiesAdvanced.exe
```
2. Click **Enviorment Variables** at the bottom.
3. In the **System variables** box, search for the variable **'Path'**, click on it to mark it and hit **Edit**
4. To add a directory now, click **New** and enter the **full path** to your executables directory.



### Usage Examples:
- Save a function called 'hi' that will echo 'hello world'
```
winfu sv hi "echo hello world"
```
- Remove the function called 'hi'
```
winfu rm hi
```
- List all your created functions
```
winfu ls
```
- If your arguments contain a Path with whitespaces for example, format like this:
```
winfu sv MoveToPathContainingWhitespaces "cd 'C:\Path With\White Spaces\'"
```
