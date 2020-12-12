# Docker + Raspberry Pi Zero + Rust

> Hi! This is a personnal project/boilerplate for compiling Rust to my Raspberry pi zero W on a Windows 10 computer. Not bullet-proof.

## Pourquoi?

3 raisons essentielles:

- j'ai un Raspberry Pi Zero W configuré devant moi.
- mon ordi tourne sous Windows 10.
- cela fait un moment que j'aimerai me frotter a Rust.

## L'idée principale:

- créer une image Docker avec les outils de dev (Rust, toolchain armv6...) prête à l'emploi
- avoir un "hello world" (blinking Led) en Rust.
- lancer la compilation des sources via une ligne de commande Docker qui target le raspberry (armv6 pour le Pi Zero)
- avoir le binaire compilé pret a etre scp vers le raspberry!

## Pré-requis

- Docker installé et lancé sur votre machine

## How to

Après avoir cloner le projet sur votre machine, ouvrir une ligne de commande (personnellement, je prefere git-bash a powershell) à la racine du projet

#### builder l'image Docker

```
docker build --tag pi/armv6:latest ./
```

(c'est le moment d'aller boire un café)

> **Note**: l'image utilise une version 'nightly' de Rust pour permettre (oui, je sais, c'est mal) de passer outre certaines erreurs (comme des deprecation par exemple).

> **Note 2**: Je me suis très (très) largement inspiré de https://github.com/mdirkse/rust_armv6 pour le Dockerfile. Thanks!

#### Lancer le container et recuperer un shell interactif

```
docker run -it --rm -v Path/to/project:/mnt/nameOfProject pi/armv6:latest /bin/bash
```

#### compiler le code (exemple)

```
cd /mnt/nameOfProject
cargo build --release --target=arm-unknown-linux-gnueabihf
```

Le resultat de la compilation se trouve dans le dossier **/target/arm-unknown-linux-gnueabihf/release**

## Hello World

Le projet Boilerplate est un simple hello world a la raspberry-style, c.a.d faire clignoter une LED 5 fois.
![alt text](https://github.com/st3ffane/rasp-docker-rust/blob/main/schema.png?raw=true)
