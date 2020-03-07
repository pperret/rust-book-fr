> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Push Request](https://github.com/Jimskapt/rust-book-fr/pull/184).

<!--
## Extending Cargo with Custom Commands
-->

## Etendre les fonctionnalités de cargo avec des commandes personnalisées

<!--
Cargo is designed so you can extend it with new subcommands without having to
modify Cargo. If a binary in your `$PATH` is named `cargo-something`, you can
run it as if it was a Cargo subcommand by running `cargo something`. Custom
commands like this are also listed when you run `cargo --list`. Being able to
use `cargo install` to install extensions and then run them just like the
built-in Cargo tools is a super convenient benefit of Cargo’s design!
-->

Cargo est conçu pour que vous puissiez étendre ses fonctionnalités avec des
nouvelles sous-commandes sans avoir à modifier cargo. Si un binaire dans votre
`$PATH` est nommé selon `cargo-quelquechose`, vous pouvez le lancer comme s'il
était une sous-commande de cargo en lançant `cargo quelquechose`. Les commandes
personnalisées comme celle-ci  sont aussi listées lorsque vous lancez
`cargo --list`. Pouvoir utiliser `cargo install` pour installer des extensions
et ensuite les lancer comme étant un outil intégré à cargo est un avantage
super pratique de la conception de cargo !

<!--
## Summary
-->

## Résumé

<!--
Sharing code with Cargo and [crates.io](https://crates.io/)<!-- ignore -- > is
part of what makes the Rust ecosystem useful for many different tasks. Rust’s
standard library is small and stable, but crates are easy to share, use, and
improve on a timeline different from that of the language. Don’t be shy about
sharing code that’s useful to you on [crates.io](https://crates.io/)<!-- ignore
-- >; it’s likely that it will be useful to someone else as well!
-->

Le partage de code avec cargo et [crates.io](https://crates.io/)<!-- ignore -->
est la partie qui rend l'écosystème de Rust très utile pour de nombreuses
tâches. La bibliothèque standard de Rust est compacte et stable, et les crates
sont faciles à partager, utiliser, et à améliorer dans un cycle de vie différent
du langage. N'hésitez pas à partager du code qui vous est utile sur
[crates.io](https://crates.io/)<!-- ignore --> ; il est fort probable qu'il
sera aussi utile à quelqu'un d'autre !
