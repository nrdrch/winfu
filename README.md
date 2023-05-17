## winfu - Commandline Function Manager For Windows! 
 

    
## How do the winfu 
| **Option**       | **Description**    | **Example**   |
| :---:        | :---          | :---     |
| sv          | Save function   | winfu sv hi "echo hello world"         |     
| rm          | Remove funciton | winfu rm hi        |
| ls          | List functions | winfu ls     |
| cp          | Import clipboard | winfu cp     |
| svp          | Save variable   | winfu svp docs "C:\Users\Username\Documents\"    |     
| rmp          | Remove funciton | winfu rmp docs |
| lsp          | List functions | winfu lsp     |
---------


<details>
<summary> 
Installation:</summary> 
  
### Potential Dependencies:
:large_blue_diamond: [Cargo & Rust:](https://doc.rust-lang.org/cargo/getting-started/installation.html)
:large_blue_diamond: [Git for Windows](https://gitforwindows.org/)
  
Recommended Method
```
cargo install winfu
```
:large_blue_diamond: Alternative Method
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

</details>
