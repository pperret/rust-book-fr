> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.

<!--
## Appendix D - Useful Development Tools
-->

## Annexe D - Des outils de développement utiles

<!--
In this appendix, we talk about some useful development tools that the Rust
project provides. We’ll look at automatic formatting, quick ways to apply
warning fixes, a linter, and integrating with IDEs.
-->

Dans cette annexe, nous allons découvrir quelques outils de développement utiles
que propose le projet Rust. Nous allons voir le formatage automatique, des
moyens rapides pour corriger des avertissements, un analyseur statique, et
l'intégration avec un IDE.

<!--
### Automatic Formatting with `rustfmt`
-->

### Le formatage automatique avec `rustfmt`

<!--
The `rustfmt` tool reformats your code according to the community code style.
Many collaborative projects use `rustfmt` to prevent arguments about which
style to use when writing Rust: everyone formats their code using the tool.
-->

L'outil `rustfmt` reformate votre code suivant le style de code de la
communauté. De nombreux projets collaboratifs utilisent `rustfmt` pour éviter
des désaccords sur le style à utiliser lorsqu'ils écrivent du code Rust : tout le
monde formate leur code en utilisant l'outil.

<!--
To install `rustfmt`, enter the following:
-->

Pour installer `rustfmt`, saisissez ceci :

```text
$ rustup component add rustfmt
```

<!--
This command gives you `rustfmt` and `cargo-fmt`, similar to how Rust gives you
both `rustc` and `cargo`. To format any Cargo project, enter the following:
-->

Cette commande vous offre `rustfmt` et `cargo-fmt`, de la même manière que Rust
vous installe `rustc` et `cargo`. Pour formater un projet Cargo, saisissez
ceci :

```text
$ cargo fmt
```

<!--
Running this command reformats all the Rust code in the current crate. This
should only change the code style, not the code semantics. For more information
on `rustfmt`, see [its documentation][rustfmt].
-->

L'exécution de cette commande reformate tout le code Rust dans la crate
courante. Cela va uniquement changer le style de code, pas sa sémantique. Pour
plus d'informations sur `rustfmt`, voyez [sa documentation][rustfmt].

[rustfmt]: https://github.com/rust-lang/rustfmt

<!--
### Fix Your Code with `rustfix`
-->

### Corriger votre code avec `rustfix`

<!--
The rustfix tool is included with Rust installations and can automatically fix
some compiler warnings. If you’ve written code in Rust, you’ve probably seen
compiler warnings. For example, consider this code:
-->

L'outil rustfix est inclus lors de l'installation de Rust et peut
automatiquement corriger certains avertissements de compilateur. Si vous avez
déjà écrit du code en Rust, vous avez probablement vu des avertissements du
compilateur. Par exemple, avec le code suivant :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
fn do_something() {}

fn main() {
    for i in 0..100 {
        do_something();
    }
}
```
-->

```rust
fn fait_quelquechose() {}

fn main() {
    for i in 0..100 {
        fait_quelquechose();
    }
}
```

<!--
Here, we’re calling the `do_something` function 100 times, but we never use the
variable `i` in the body of the `for` loop. Rust warns us about that:
-->

Ici, nous appelons la fonction `fait_quelquechose` 100 fois, mais nous
n'utilisons jamais la variable `i` dans le corps de la boucle `for`. Rust nous
avertit de cela :

<!--
```text
$ cargo build
   Compiling myprogram v0.1.0 (file:///projects/myprogram)
warning: unused variable: `i`
 -- > src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```
-->

```text
$ cargo build
   Compiling mon_programme v0.1.0 (file:///projects/mon_programme)
warning: unused variable: `i`
 -- > src/main.rs:4:9
  |
4 |     for i in 1..100 {
  |         ^ help: consider using `_i` instead
  |
  = note: #[warn(unused_variables)] on by default

    Finished dev [unoptimized + debuginfo] target(s) in 0.50s
```

<!--
The warning suggests that we use `_i` as a name instead: the underscore
indicates that we intend for this variable to be unused. We can automatically
apply that suggestion using the `rustfix` tool by running the command `cargo
fix`:
-->

L'avertissement indique que nous devrions utiliser `_i` comme nom à sa place :
le tiret bas indique que nous avons l'intention de ne pas utiliser cette
variable. Nous pouvons appliquer automatiquement cette suggestion en utilisant
l'outil `rustfix` en lançant la commande `cargo fix` :

<!--
```text
$ cargo fix
    Checking myprogram v0.1.0 (file:///projects/myprogram)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```
-->

```text
$ cargo fix
    Checking mon_programme v0.1.0 (file:///projects/mon_programme)
      Fixing src/main.rs (1 fix)
    Finished dev [unoptimized + debuginfo] target(s) in 0.59s
```

<!--
When we look at *src/main.rs* again, we’ll see that `cargo fix` has changed the
code:
-->

Lorsque nous regardons à nouveau *src/main.rs*, nous pouvons constater que
`cargo fix` a changé le code :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
fn do_something() {}

fn main() {
    for _i in 0..100 {
        do_something();
    }
}
```
-->

```rust
fn fait_quelquechose() {}

fn main() {
    for _i in 0..100 {
        fait_quelquechose();
    }
}
```

<!--
The `for` loop variable is now named `_i`, and the warning no longer appears.
-->

La variable de la boucle `for` s'appelle maintenant `_i`, et l'avertissement ne
s'affiche plus.

<!--
You can also use the `cargo fix` command to transition your code between
different Rust editions. Editions are covered in Appendix E.
-->

Vous pouvez aussi utiliser la commande `cargo fix` pour corriger votre code
entre différentes éditions de Rust. Les éditions sont abordées à l'annexe E.

<!--
### More Lints with Clippy
-->

### Une analyse statique plus complète avec Clippy

<!--
The Clippy tool is a collection of lints to analyze your code so you can catch
common mistakes and improve your Rust code.
-->

L'outil Clippy est une collection d'analyses statiques pour analyser votre code
afin que vous puissiez débusquer certaines erreurs courantes et ainsi améliorer
votre code.

<!--
To install Clippy, enter the following:
-->

Pour installer Clippy, saisissez ceci :

```text
$ rustup component add clippy
```

<!--
To run Clippy’s lints on any Cargo project, enter the following:
-->

Pour lancer l'analyse statique de Clippy sur un projet Cargo, saisissez ceci :

```text
$ cargo clippy
```

<!--
For example, say you write a program that uses an approximation of a
mathematical constant, such as pi, as this program does:
-->

Par exemple, imaginons que vous écrivez un programme qui utilise une
approximation d'une constante mathématique, comme pi, comme le fait ce
programme :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```
-->

```rust
fn main() {
    let x = 3.1415;
    let r = 8.0;
    println!("l'aire du cercle vaut {}", x * r * r);
}
```

<!--
Running `cargo clippy` on this project results in this error:
-->

L'exécution de `cargo clippy` sur ce projet va afficher cette erreur :

```text
error: approximate value of `f{32, 64}::consts::PI` found. Consider using it directly
 -- > src/main.rs:2:13
  |
2 |     let x = 3.1415;
  |             ^^^^^^
  |
  = note: #[deny(clippy::approx_constant)] on by default
  = help: for further information visit https://rust-lang-nursery.github.io/rust-clippy/master/index.html#approx_constant
```

<!--
This error lets you know that Rust has this constant defined more precisely and
that your program would be more correct if you used the constant instead. You
would then change your code to use the `PI` constant. The following code
doesn’t result in any errors or warnings from Clippy:
-->

Cette erreur vous fais savoir que Rust a cette constante qui est définie plus
précisément et que votre programme serait plus pertinent si vous utilisiez à la
place la constante. Vous changeriez alors votre code pour utiliser la constante
`PI`. Le code suivant ne donne pas d'erreur ou d'avertissement avec Clippy :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("the area of the circle is {}", x * r * r);
}
```
-->

```rust
fn main() {
    let x = std::f64::consts::PI;
    let r = 8.0;
    println!("l'aire du cercle vaut {}", x * r * r);
}
```

<!--
For more information on Clippy, see [its documentation][clippy].
-->

Pour en savoir plus Clippy, voyez [sa documentation][clippy].

[clippy]: https://github.com/rust-lang/rust-clippy

<!--
### IDE Integration Using the Rust Language Server
-->

### L'intégration aux IDE en utilisant le Rust Language Server

<!--
To help IDE integration, the Rust project distributes the *Rust Language
Server* (`rls`). This tool speaks the [Language Server
Protocol][lsp], which is a specification for IDEs and programming
languages to communicate with each other. Different clients can use the `rls`,
such as [the Rust plug-in for Visual Studio Code][vscode].
-->

Pour aider l'intégration aux IDE, le projet Rust distribue le
*Rust Language Server* (`rls`). Cet outil suit le
[Language Server Protocol][lsp], qui est une spécification entre les IDE et les
langages pour communiquer entre eux. Différents clients peuvent utiliser le
`rls`, comme [le plug-in Rust pour Visual Studio Code][vscode].

[lsp]: http://langserver.org/
[vscode]: https://marketplace.visualstudio.com/items?itemName=rust-lang.rust

<!--
To install the `rls`, enter the following:
-->

Pour installer le `rls`, saisissez ceci :

```text
$ rustup component add rls
```

<!--
Then install the language server support in your particular IDE; you’ll gain
abilities such as autocompletion, jump to definition, and inline errors.
-->

Installez ensuite le système du *language server* dans votre IDE ; vous devriez
obtenir des capacités supplémentaires comme l'auto-complétion, pouvoir se rendre
à la définition de l'élément, et la mise en valeur d'erreurs sur la ligne
concernée.

<!--
For more information on the `rls`, see [its documentation][rls].
-->

Pour plus d'information sur `rls`, rendez-vous [à sa documentation][rls].

[rls]: https://github.com/rust-lang/rls
