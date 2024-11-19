# MyKeepass
## _Personnal open source Password Manager

[![N|Solid](https://logodix.com/logo/2164879.png)](https://nodesource.com/products/nsolid)

[![Build Status](https://travis-ci.org/joemccann/dillinger.svg?branch=master)](https://travis-ci.org/joemccann/dillinger)

My Keepass is an CLI tool password manager to simplify password connection with simple accounts

- Add Password in Encrypted Files
- Set Master Password
- ✨ List✨our passwords and filetering on it

## Technos
- [Rust] - Rustc & cargo

## Installation

MyKeepass requires [Rust](https://rust.lang.org/) to run

Clone the project

```sh
git clone https://github.com/Liam-Esteffe/MyKeepass.git
```

If you want to contribute on the project, you can create a new branch 

```sh
git checkout -b <branch_name>
```

Just for using, you can run the ./install.sh file who configure your environment
```sh
cd MyKeepass && ./install.sh
```

To use your Keepass CLI you must add your secret key in environment file
```sh
// .env
MASTER_PASSWORD=MonPasswordSuperSecret -> Change that by your personal password
```



## Commands List

For v1 version this is a list of all commands you can make

| Commands Name | Exemple |
| ------ | ------ |
| Add Password to Password Manager | ```./my-keepass add <site_name> <username> <password>```|
| List Password of Password Manager For Site | ```./my-keepass list <site_name>```|
| List All Password of Password Manager | ```./my-keepass list```|


## Development

Want to contribute? Great!

Dillinger uses Gulp + Webpack for fast developing.
Make a change in your file and instantaneously see your updates!


