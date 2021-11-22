> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/180).

<!--
## Customizing Builds with Release Profiles
-->

## Personnaliser les compilations avec les profils de publication

<!--
In Rust, *release profiles* are predefined and customizable profiles with
different configurations that allow a programmer to have more control over
various options for compiling code. Each profile is configured independently of
the others.
-->

Dans Rust, les *profils de publication* sont des profils prédéfinis et
personnalisables avec différentes configurations qui permettent au développeur
d'avoir plus de contrôle sur différentes options de compilation du code. Chaque
profil est configuré indépendamment des autres.

<!--
Cargo has two main profiles: the `dev` profile Cargo uses when you run `cargo
build` and the `release` profile Cargo uses when you run `cargo build
--release`. The `dev` profile is defined with good defaults for development,
and the `release` profile has good defaults for release builds.
-->

Cargo a deux profils principaux : le profil `dev` que cargo utilise lorsque vous
lancez `cargo build` et le profil `release` (NdT : publication) que cargo
utilise lorsque vous lancez `cargo build --release`. Le profil `dev` est défini
avec de bons réglages par défaut pour le développement, et le profil `release` a
de bons réglages par défaut de compilations pour publication.

<!--
These profile names might be familiar from the output of your builds:
-->

Ces noms de profils vous rappellent peut-être quelque chose sur la sortie
standard de vos compilations :

<!--
<!-- manual-regeneration
anywhere, run:
cargo build
cargo build --release
and ensure output below is accurate
-- >
-->

<!--
```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```
-->

```console
$ cargo build
    Finished dev [unoptimized + debuginfo] target(s) in 0.0s
$ cargo build --release
    Finished release [optimized] target(s) in 0.0s
```

<!--
The `dev` and `release` shown in this build output indicate that the compiler
is using different profiles.
-->

Les profils `dev` et `release` sont mentionnés dans cette sortie de compilation,
pour indiquer les différents profils qu'utilise le compilateur.

<!--
Cargo has default settings for each of the profiles that apply when there
aren’t any `[profile.*]` sections in the project’s *Cargo.toml* file. By adding
`[profile.*]` sections for any profile you want to customize, you can override
any subset of the default settings. For example, here are the default values
for the `opt-level` setting for the `dev` and `release` profiles:
-->

Cargo a des réglages par défaut pour chacun des profils qui s'appliquent
lorsqu'il n'y a pas de section `[profile.*]` dans le fichier *Cargo.toml* du
projet. En ajoutant les sections `[profile.*]` pour chaque profil que vous
souhaitez personnaliser, vous pouvez remplacer n'importe quel paramètre par
défaut. Par exemple, voici les valeurs par défaut pour le paramètre `opt-level`
des profils `dev` et `release` :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```
-->

```toml
[profile.dev]
opt-level = 0

[profile.release]
opt-level = 3
```

<!--
The `opt-level` setting controls the number of optimizations Rust will apply to
your code, with a range of 0 to 3. Applying more optimizations extends
compiling time, so if you’re in development and compiling your code often,
you’ll want faster compiling even if the resulting code runs slower. That is
the reason the default `opt-level` for `dev` is `0`. When you’re ready to
release your code, it’s best to spend more time compiling. You’ll only compile
in release mode once, but you’ll run the compiled program many times, so
release mode trades longer compile time for code that runs faster. That is why
the default `opt-level` for the `release` profile is `3`.
-->

Le paramètre `opt-level` contrôle le nombre d'optimisations que Rust va
appliquer à votre code, sur une échelle allant de 0 à 3. L'application d'un
niveau plus haut d'optimisation signifie un allongement de la durée de
compilation, donc si vous êtes en train de développer et que vous compilez
souvent votre code, vous préférerez certainement avoir une compilation rapide
même si le code qui en résulte s'exécute plus lentement. C'est la raison pour
laquelle la valeur par défaut de `opt-level` pour `dev` est à `0`. Lorsque vous
serez prêt à publier votre code, il sera préférable de passer un peu plus de
temps à le compiler. Vous ne compilerez en mode publication (NdT : release)
qu'une seule fois, mais vous exécuterez le programme compilé plusieurs fois,
donc le mode publication opte pour un temps de compilation plus long afin que le
code s'exécute plus rapidement. C'est pourquoi le paramètre `opt-level` par
défaut pour le profil `release` est à `3`.

<!--
You can override any default setting by adding a different value for it in
*Cargo.toml*. For example, if we want to use optimization level 1 in the
development profile, we can add these two lines to our project’s *Cargo.toml*
file:
-->

Vous pouvez remplacer n'importe quel paramètre par défaut en ajoutant une valeur
différente dans *Cargo.toml*. Par exemple, si nous voulons utiliser le niveau 1
d'optimisation dans le profil de développement, nous pouvons ajouter ces deux
lignes à notre fichier *Cargo.toml* :

<!--
<span class="filename">Filename: Cargo.toml</span>
-->

<span class="filename">Fichier : Cargo.toml</span>

<!--
```toml
[profile.dev]
opt-level = 1
```
-->

```toml
[profile.dev]
opt-level = 1
```

<!--
This code overrides the default setting of `0`. Now when we run `cargo build`,
Cargo will use the defaults for the `dev` profile plus our customization to
`opt-level`. Because we set `opt-level` to `1`, Cargo will apply more
optimizations than the default, but not as many as in a release build.
-->

Ce code remplace le paramètre par défaut à `0`. Maintenant, lorsque nous lançons
`cargo build`, cargo va utiliser les réglages par défaut du profil `dev` ainsi
que notre valeur personnalisée de `opt-level`. Comme nous avons réglé
`opt-level` à `1`, Cargo va appliquer plus d'optimisation que par défaut, mais
pas autant que dans une compilation de publication.

<!--
For the full list of configuration options and defaults for each profile, see
[Cargo’s documentation](https://doc.rust-lang.org/cargo/reference/profiles.html).
-->

Pour la liste complète des options de configuration et leurs valeurs par défaut
pour chaque profil, rendez-vous à la
[documentation de cargo](https://doc.rust-lang.org/cargo/reference/profiles.html).
