> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Push Request](https://github.com/Jimskapt/rust-book-fr/pull/94).

<!--
## Unrecoverable Errors with `panic!`
-->

## Les erreurs irrécupérables avec `panic!`

<!--
Sometimes, bad things happen in your code, and there’s nothing you can do about
it. In these cases, Rust has the `panic!` macro. When the `panic!` macro
executes, your program will print a failure message, unwind and clean up the
stack, and then quit. This most commonly occurs when a bug of some kind has
been detected and it’s not clear to the programmer how to handle the error.
-->

Parfois, les choses se passent mal dans votre code, et vous ne pouvez rien y
faire. Pour ces cas-là, Rust a la macro `panic!`. Quand la macro `panic!`
s'exécute, votre programme va afficher un message d'erreur, dérouler et nettoyer
la pile, et ensuite fermer le programme. Cela se produit fréquemment lorsqu'un
bogue a été détecté, et que le développeur n'a pas su comment gérer cette
erreur.

<!--
> ### Unwinding the Stack or Aborting in Response to a Panic
>
> By default, when a panic occurs, the program starts *unwinding*, which
> means Rust walks back up the stack and cleans up the data from each function
> it encounters. But this walking back and cleanup is a lot of work. The
> alternative is to immediately *abort*, which ends the program without
> cleaning up. Memory that the program was using will then need to be cleaned
> up by the operating system. If in your project you need to make the resulting
> binary as small as possible, you can switch from unwinding to aborting upon a
> panic by adding `panic = 'abort'` to the appropriate `[profile]` sections in
> your *Cargo.toml* file. For example, if you want to abort on panic in release
> mode, add this:
>
> ```toml
> [profile.release]
> panic = 'abort'
> ```
-->

> ### Dérouler la pile ou abandonner suite à un `panic!`
>
> Par défaut, quand un `panic!` se produit, le programme commence par
> *dérouler*, ce qui veut dire que Rust retourne en arrière dans la pile et
> nettoie les données de chaque fonction qu'il rencontre sur son passage. Mais
> cette marche arrière et le nettoyage demande beaucoup de travail. Une
> alternative est *d'abandonner* immédiatement, ce qui arrête le programme sans
> nettoyage. La mémoire qu'utilisait le programme va devoir ensuite être
> nettoyée par le système d'exploitation. Si dans votre projet vous avez besoin
> de construire un exécutable le plus petit possible, vous pouvez changer du
> dévidage à l'abandon lors d'un panic en ajoutant `panic = 'abort'` aux
> sections `[profile]` correspondantes dans votre fichier *Cargo.toml*. Par
> exemple, si vous souhaitez abandonner lors d'un panic en mode release, ajoutez
> ceci :
> 
> ```toml
> [profile.release]
> panic = 'abort'
> ```

<!--
Let’s try calling `panic!` in a simple program:
-->

Essayons d'appeler `panic!` dans un programme simple :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic,panics
fn main() {
    panic!("crash and burn");
}
```
-->

```rust,should_panic,panics
fn main() {
    panic!("crash and burn");
}
```

<!--
When you run the program, you’ll see something like this:
-->

Lorsque vous lancez le programme, vous allez voir quelque chose comme ceci :

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.25s
     Running `target/debug/panic`
thread 'main' panicked at 'crash and burn', src/main.rs:2:5
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

<!--
The call to `panic!` causes the error message contained in the last two lines.
The first line shows our panic message and the place in our source code where
the panic occurred: *src/main.rs:2:5* indicates that it’s the second line,
fifth character of our *src/main.rs* file.
-->

L'utilisation de `panic!` déclenche le message d'erreur présent dans les deux
dernières lignes. La première ligne affiche notre message associé au panic et
l'emplacement dans notre code source où se produit le panic : *src/main.rs:2:5*
indique que c'est la seconde ligne et cinquième caractère de notre fichier
*src/main.rs*.

<!--
In this case, the line indicated is part of our code, and if we go to that
line, we see the `panic!` macro call. In other cases, the `panic!` call might
be in code that our code calls, and the filename and line number reported by
the error message will be someone else’s code where the `panic!` macro is
called, not the line of our code that eventually led to the `panic!` call. We
can use the backtrace of the functions the `panic!` call came from to figure
out the part of our code that is causing the problem. We’ll discuss what a
backtrace is in more detail next.
-->

Dans cet exemple, la ligne indiquée fait partie de notre code, et si nous
allons voir cette ligne, nous verrons l'appel à la macro `panic!`. Dans d'autres
cas, l'appel de `panic!` pourrait se produire dans du code que notre
code utilise. Le nom du fichier et la ligne indiquée par le message d'erreur
sera alors du code de quelqu'un d'autre où la macro `panic!` est appelée, et non
pas la ligne de notre code qui nous a mené à cet appel de `panic!`. Nous pouvons
utiliser le re-traçage des fonctions qui appellent le `panic!` pour comprendre
la partie de notre code qui pose problème. Nous allons maintenant parler plus
en détail de ce qu'est le re-traçage.

<!--
### Using a `panic!` Backtrace
-->

### Utiliser le re-traçage de `panic!`

<!--
Let’s look at another example to see what it’s like when a `panic!` call comes
from a library because of a bug in our code instead of from our code calling
the macro directly. Listing 9-1 has some code that attempts to access an
element by index in a vector.
-->

Analysons un autre exemple pour voir ce qui se passe lors d'un appel de
`panic!` se produit dans un librairie à cause d'un bug dans notre code plutôt
que d'appeler la macro directent. L'encart 9-1 montre du code qui essaye
d'accéder à un élément d'un vecteur via son indice :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust,should_panic,panics
fn main() {
    let v = vec![1, 2, 3];

    v[99];
}
```

<!--
<span class="caption">Listing 9-1: Attempting to access an element beyond the
end of a vector, which will cause a call to `panic!`</span>
-->

<span class="caption">Encart 9-1 : Tentative d'accéder à un élément en dehors de
l'intervalle d'un vecteur, ce qui provoque un `panic!`</span>

<!--
Here, we’re attempting to access the 100th element of our vector (which is at
index 99 because indexing starts at zero), but it has only 3 elements. In this
situation, Rust will panic. Using `[]` is supposed to return an element, but if
you pass an invalid index, there’s no element that Rust could return here that
would be correct.
-->

Ici, nous essayons d'accéder au centième élément de notre vecteur (qui est à
l'indice 99 car l'indexation commence à zéro), mais le vecteur a seulement trois
éléments. Dans ce cas, Rust va faire un panic. Utiliser `[]` est censé retourner
un élément, mais si vous lui donnez un indice invalide, Rust ne pourra pas
retourner un élément acceptable dans ce cas.

<!--
Other languages, like C, will attempt to give you exactly what you asked for in
this situation, even though it isn’t what you want: you’ll get whatever is at
the location in memory that would correspond to that element in the vector,
even though the memory doesn’t belong to the vector. This is called a *buffer
overread* and can lead to security vulnerabilities if an attacker is able to
manipulate the index in such a way as to read data they shouldn’t be allowed to
that is stored after the array.
-->

D'autres langages, comme le C, vont tenter de vous donner exactement ce que vous
avez demandé, même si ce n'est pas ce que vous vouliez : vous allez récupérer
quelque chose à l'emplacement mémoire demandée qui devrait correspondre à
l'élément demandé dans le vecteur, même si cette partie de la mémoire
n'appartient pas au vecteur. C'est ce qu'on appelle une *sur-lecture de tampon*
et peut mener à une faille de sécurité si un attaquant à la possibilité de
piloter l'indice de telle manière qu'il puisse lire les données qui ne devraient
pas être lisibles en dehors du tableau.

<!--
To protect your program from this sort of vulnerability, if you try to read an
element at an index that doesn’t exist, Rust will stop execution and refuse to
continue. Let’s try it and see:
-->

Afin de protéger votre programme de ce genre de vulnérabilité, si vous essayez
de lire un élément à un indice qui n'existe pas, Rust va arrêter l'exécution et
refuser de continuer. Essayez et vous verrez :

```text
$ cargo run
   Compiling panic v0.1.0 (file:///projects/panic)
    Finished dev [unoptimized + debuginfo] target(s) in 0.27s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', libcore/slice/mod.rs:2448:10
note: Run with `RUST_BACKTRACE=1` for a backtrace.
```

<!--
This error points at a file we didn’t write, *libcore/slice/mod.rs*. That’s the
implementation of `slice` in the Rust source code. The code that gets run when
we use `[]` on our vector `v` is in *libcore/slice/mod.rs*, and that is where
the `panic!` is actually happening.
-->

Cette erreur se réfère à un fichier que nous n'avons pas codé,
*libcore/slice/mod.rs*. C'est l'implémentation de `slice` dans la bibliothèque
standard. Le code qui est lancé quand nous utilisons `[]` sur notre vecteur `v`
est dans *libcore/slice/mod.rs*, et c'est ici que le `panic!` se produit dans
notre cas.

<!--
The next note line tells us that we can set the `RUST_BACKTRACE` environment
variable to get a backtrace of exactly what happened to cause the error. A
*backtrace* is a list of all the functions that have been called to get to this
point. Backtraces in Rust work as they do in other languages: the key to
reading the backtrace is to start from the top and read until you see files you
wrote. That’s the spot where the problem originated. The lines above the lines
mentioning your files are code that your code called; the lines below are code
that called your code. These lines might include core Rust code, standard
library code, or crates that you’re using. Let’s try getting a backtrace by
setting the `RUST_BACKTRACE` environment variable to any value except 0.
Listing 9-2 shows output similar to what you’ll see.
-->

La ligne suivante nous informe que nous pouvons régler la variable
d'environnement `RUST_BACKTRACE` pour obtenir le re-traçage de ce qui s'est
exactement passé pour mener à cette erreur. Un *re-traçage* consiste à lister
toutes les fonctions qui ont été appelées pour arriver jusqu'à ce point. Avec
Rust, le re-traçage fonctionne comme il le fait dans d'autres langages : le
secret pour lire le re-traçage est de commencer d'en haut et lire jusqu'à ce
que vous voyez les fichiers que vous avez écris. C'est l'endroit où s'est
produit le problème. Les lignes avant celle qui mentionne vos fichier
représentent le code qu'à appelé votre code ; les lignes qui suivent
représentent le code qui a appelé votre code. Ces lignes peuvent être du code
du coeur de Rust, du code de la bibliothèque standard, ou des crates que vous
utilisez. Essayons d'obtenir un re-traçage en réglant la variable
d'environnement `RUST_BACKTRACE` à n'importe quelle valeur autre que 0. L'encart
9-2 nous montre un retour similaire à ce que vous devriez voir :

```text
$ RUST_BACKTRACE=1 cargo run
    Finished dev [unoptimized + debuginfo] target(s) in 0.00s
     Running `target/debug/panic`
thread 'main' panicked at 'index out of bounds: the len is 3 but the index is 99', libcore/slice/mod.rs:2448:10
stack backtrace:
   0: std::sys::unix::backtrace::tracing::imp::unwind_backtrace
             at libstd/sys/unix/backtrace/tracing/gcc_s.rs:49
   1: std::sys_common::backtrace::print
             at libstd/sys_common/backtrace.rs:71
             at libstd/sys_common/backtrace.rs:59
   2: std::panicking::default_hook::{{closure}}
             at libstd/panicking.rs:211
   3: std::panicking::default_hook
             at libstd/panicking.rs:227
   4: <std::panicking::begin_panic::PanicPayload<A> as core::panic::BoxMeUp>::get
             at libstd/panicking.rs:476
   5: std::panicking::continue_panic_fmt
             at libstd/panicking.rs:390
   6: std::panicking::try::do_call
             at libstd/panicking.rs:325
   7: core::ptr::drop_in_place
             at libcore/panicking.rs:77
   8: core::ptr::drop_in_place
             at libcore/panicking.rs:59
   9: <usize as core::slice::SliceIndex<[T]>>::index
             at libcore/slice/mod.rs:2448
  10: core::slice::<impl core::ops::index::Index<I> for [T]>::index
             at libcore/slice/mod.rs:2316
  11: <alloc::vec::Vec<T> as core::ops::index::Index<I>>::index
             at liballoc/vec.rs:1653
  12: panic::main
             at src/main.rs:4
  13: std::rt::lang_start::{{closure}}
             at libstd/rt.rs:74
  14: std::panicking::try::do_call
             at libstd/rt.rs:59
             at libstd/panicking.rs:310
  15: macho_symbol_search
             at libpanic_unwind/lib.rs:102
  16: std::alloc::default_alloc_error_hook
             at libstd/panicking.rs:289
             at libstd/panic.rs:392
             at libstd/rt.rs:58
  17: std::rt::lang_start
             at libstd/rt.rs:74
  18: panic::main
```

<!--
<span class="caption">Listing 9-2: The backtrace generated by a call to
`panic!` displayed when the environment variable `RUST_BACKTRACE` is set</span>
-->

<span class="caption">Encart 9-2: Le re-traçage généré par l'appel de `panic!`
est affiché quand la variable d'environnement `RUST_BACKTRACE` est définie
</span>

<!--
That’s a lot of output! The exact output you see might be different depending
on your operating system and Rust version. In order to get backtraces with this
information, debug symbols must be enabled. Debug symbols are enabled by
default when using `cargo build` or `cargo run` without the `--release` flag,
as we have here.
-->

Cela fait beaucoup de contenu ! L'exactitude de votre retour qui est affiché
peut être différent en fonction de votre système d'exploitation et de votre
version de Rust. Pour avoir le re-traçage avec ces informations, les instructions
de déboguage doivent être activés. Les instructions de déboguage sont activées
par défaut quand on utilise `cargo build` ou `cargo run` sans le drapeau
`--release`, comme c'est le cas ici.

<!--
In the output in Listing 9-2, line 12 of the backtrace points to the line in
our project that’s causing the problem: line 4 of *src/main.rs*. If we don’t
want our program to panic, the location pointed to by the first line mentioning
a file we wrote is where we should start investigating. In Listing 9-1, where
we deliberately wrote code that would panic in order to demonstrate how to use
backtraces, the way to fix the panic is to not request an element at index 99
from a vector that only contains 3 items. When your code panics in the future,
you’ll need to figure out what action the code is taking with what values to
cause the panic and what the code should do instead.
-->

Dans l'encart 9-2, la ligne 12 du re-traçage nous montre la ligne de notre projet
qui provoque le problème : la ligne 4 de *src/main.rs*. Si nous ne voulons pas
que notre programme panique, le premier endroit que nous devrions inspecter est
l'emplacement cité par la première ligne qui mentionne du code que nous avons
écrit. Dans l'encart 9-1, où nous avons délibérément écrit du code qui panique
dans le but de montrer comment utiliser le re-traçage, la solution pour ne pas
paniquer est de ne pas demander l'élément à l'indice 99 à un vecteur lorsqu'il
n'en contient que 3. A l'avenir quand votre code paniquera, vous aurez besoin de
prendre des dispositions dans votre code pour les valeurs qui font paniquer et
de coder quoi faire lorsque cela se produit.

<!--
We’ll come back to `panic!` and when we should and should not use `panic!` to
handle error conditions in the [“To `panic!` or Not to
`panic!`”][to-panic-or-not-to-panic]<!-- ignore -- > section later in this
chapter. Next, we’ll look at how to recover from an error using `Result`.
-->

Nous reviendrons sur le cas du `panic!` et sur les cas où nous devrions et ne
devrions pas utiliser `panic!` pour gérer les conditions d'erreur plus tard
à [la fin de ce chapitre][to-panic-or-not-to-panic]<!-- ignore -->. Pour le
moment, nous allons voir comment gérer une erreur en utilisant `Result`.

<!--
[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html#to-panic-or-not-to-panic
-->

[to-panic-or-not-to-panic]:
ch09-03-to-panic-or-not-to-panic.html
