> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/183).

<!--
## Installing Binaries from Crates.io with `cargo install`
-->

## Installer des binaires à partir de crates.io avec `cargo install`

<!--
The `cargo install` command allows you to install and use binary crates
locally. This isn’t intended to replace system packages; it’s meant to be a
convenient way for Rust developers to install tools that others have shared on
[crates.io](https://crates.io/)<!-- ignore -- >. Note that you can only install
packages that have binary targets. A *binary target* is the runnable program
that is created if the crate has a *src/main.rs* file or another file specified
as a binary, as opposed to a library target that isn’t runnable on its own but
is suitable for including within other programs. Usually, crates have
information in the *README* file about whether a crate is a library, has a
binary target, or both.
-->

La commande `cargo install` vous permet d'installer et utiliser des crates de
binaires localement. Cela n'est pas conçu pour remplacer les systèmes de
paquets ; c'est plutôt un moyen pratique pour les développeurs Rust d'installer
des outils que les autres ont partagé sur
[crates.io](https://crates.io/)<!-- ignore -->. Notez que vous ne pouvez
installer que des paquets qui ont des destinations binaires. Une *destination
binaire* est le programme exécutable qui est créé si la crate a un fichier
*src/main.rs* ou un autre fichier renseigné comme un binaire, contrairement
à une destination de bibliothèque qui n'est pas exécutable en tant que tel mais
qu'il est possible d'intégrer à un autre programme. Habituellement, les crates
ont l'information dans le fichier *README* pour savoir si elle est une
bibliothèque, a une destination binaire, ou les deux.

<!--
All binaries installed with `cargo install` are stored in the installation
root’s *bin* folder. If you installed Rust using *rustup.rs* and don’t have any
custom configurations, this directory will be *$HOME/.cargo/bin*. Ensure that
directory is in your `$PATH` to be able to run programs you’ve installed with
`cargo install`.
-->

Tous les binaires installés avec `cargo install` sont stockés dans le dossier
*bin* de la racine. Si vous installez Rust avec *rustup.rs* et que vous n'avez
pas personnalisé la configuration, ce dossier sera *$HOME/.cargo/bin*.
Assurez-vous que ce dossier est dans votre `$PATH` pour pouvoir exécuter des
programmes que vous avez installé avec `cargo install`.

<!--
For example, in Chapter 12 we mentioned that there’s a Rust implementation of
the `grep` tool called `ripgrep` for searching files. If we want to install
`ripgrep`, we can run the following:
-->

Par exemple, dans le chapitre 12 nous avions mentionné le fait qu'il existait
une implémentation de l'outil `grep` en Rust qui s'appelait `ripgrep` et qui
permettait de rechercher dans des fichiers. Si nous voulons installer
`ripgrep`, nous pouvons faire comme ceci :

<!--
<!-- manual-regeneration
cargo install something you don't have, copy relevant output below
-- >
-->

<!--
```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v11.0.2
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v11.0.2
--snip--
   Compiling ripgrep v11.0.2
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v11.0.2` (executable `rg`)
```
-->

```console
$ cargo install ripgrep
    Updating crates.io index
  Downloaded ripgrep v11.0.2
  Downloaded 1 crate (243.3 KB) in 0.88s
  Installing ripgrep v11.0.2
-- partie masquée ici --
   Compiling ripgrep v11.0.2
    Finished release [optimized + debuginfo] target(s) in 3m 10s
  Installing ~/.cargo/bin/rg
   Installed package `ripgrep v11.0.2` (executable `rg`)
```

<!--
The second-to-last line of the output shows the location and the name of the
installed binary, which in the case of `ripgrep` is `rg`. As long as the
installation directory is in your `$PATH`, as mentioned previously, you can
then run `rg --help` and start using a faster, rustier tool for searching files!
-->

L'avant-dernière ligne de la sortie nous montre l'emplacement et le nom du
binaire installé, qui est `rg` dans le cas de `ripgrep`. Tel que mentionné
précédemment, du moment que le dossier d'installation est dans votre `$PATH`,
vous pouvez ensuite lancer `rg --help` et commencer à utiliser un outil en Rust
qui est rapide pour rechercher dans les fichiers !
