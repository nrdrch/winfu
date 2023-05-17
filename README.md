### Winfu - Commandline Function Manager For Windows! 
#### **Winfu** aims to significantly reduce the time and effort you spend on managing PowerShell functions and variables. <br /> While originally inspired by [fish](https://fishshell.com/) on linux and its quick Terminal function creation, <br /> this Utility now has a more complex set of features to  **Save**,  **List**  amd  **Remove** functions or variables in a intuitive and foolproof way.<br />Implementation of importing complete functions or a variables directly from your clipboard, <br />always ensures a way to add complex code without the need for any additional effort.



------------------
#### Installation 
```
cargo install winfu
```

<details>
<summary> 
further Installation info:</summary> 
  
#### Potential Dependencies:
- [Cargo & Rust:](https://doc.rust-lang.org/cargo/getting-started/installation.html)
   
- [Git for Windows](https://gitforwindows.org/)
  

#### Alternative Method
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


------------------
| **Option**       | **Description**    | **Example**   |
| :---:        | :---          | :---     |
| sv          | Save function   | winfu sv hi "echo hello world"         |     
| rm          | Remove funciton | winfu rm hi        |
| ls          | List functions | winfu ls     |
| cp          | Import clipboard   | winfu cp     |
| svp          | Save variable   | winfu svp docs "C:\Users\Username\Documents\"    |     
| rmp          | Remove funciton | winfu rmp docs |
| lsp          | List functions | winfu lsp     |
---------

