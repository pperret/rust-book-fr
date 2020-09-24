<!--
## References and Borrowing
-->

## Les références et l'emprunt

<!--
The issue with the tuple code in Listing 4-5 is that we have to return the
`String` to the calling function so we can still use the `String` after the
call to `calculate_length`, because the `String` was moved into
`calculate_length`.
-->

La difficulté avec le code du tuple à la fin de la section précédente est que
nous avons besoin de retourner la `String` au code appelant pour qu'il puisse
continuer à utiliser la `String` après l'appel à `calculer_taille`, car la
`String` a été déplacée dans `calculer_taille`.

<!--
Here is how you would define and use a `calculate_length` function that has a
reference to an object as a parameter instead of taking ownership of the
value:
-->

Voici comment définir et utiliser une fonction `calculer_taille` qui prend une
*référence* à un objet en paramètre plutôt que de prendre possession de la
valeur :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```
-->

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:all}}
```

<!--
First, notice that all the tuple code in the variable declaration and the
function return value is gone. Second, note that we pass `&s1` into
`calculate_length` and, in its definition, we take `&String` rather than
`String`.
-->

Premièrement, on peut observer que tout le code des *tuples* dans la déclaration
des variables et dans la valeur de retour de la fonction a été enlevé.
Deuxièmement, remarquez que nous passons `&s1` à `calculer_taille`, et que dans
sa définition, nous utilisons `&String` plutôt que `String`.

<!--
These ampersands are *references*, and they allow you to refer to some value
without taking ownership of it. Figure 4-5 shows a diagram.
-->

Ces esperluettes sont des *références*, et elles permettent de vous référer à
une valeur sans en prendre possession. L'illustration 4-5 nous montre cela dans
un schéma.

<!-- markdownlint-disable -->
<!--
<img alt="&String s pointing at String s1" src="img/trpl04-05.svg" class="center" />
-->
<!-- markdownlint-restore -->

<img alt="&String s qui pointe vers la String s1" src="img/trpl04-05.svg"
class="center" />

<!--
<span class="caption">Figure 4-5: A diagram of `&String s` pointing at `String
s1`</span>
-->

<span class="caption">Illustration 4-5 : Un schéma de la `&String s` qui pointe
vers la `String s1`</span>

<!--
> Note: The opposite of referencing by using `&` is *dereferencing*, which is
> accomplished with the dereference operator, `*`. We’ll see some uses of the
> dereference operator in Chapter 8 and discuss details of dereferencing in
> Chapter 15.
-->

> Remarque : l'opposé de la création de références avec `&` est le
> *déréférencement*, qui s'effectue avec l'opérateur de déréférencement, `*`.
> Nous allons voir quelques utilisations de l'opérateur de déréférencement dans
> le chapitre 8 et nous aborderons les détails du déréférencement dans le
> chapitre 15.

<!--
Let’s take a closer look at the function call here:
-->

Regardons de plus près l'appel à la fonction :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-07-reference/src/main.rs:here}}
```

<!--
The `&s1` syntax lets us create a reference that *refers* to the value of `s1`
but does not own it. Because it does not own it, the value it points to will
not be dropped when the reference goes out of scope.
-->

La syntaxe `&s1` nous permet de créer une référence qui se *réfère* à la valeur
de `s1` mais n'en prend pas possession. Et comme elle ne la possède pas, la
valeur vers laquelle elle pointe ne sera pas libérée quand cette référence
sortira de la portée.

<!--
Likewise, the signature of the function uses `&` to indicate that the type of
the parameter `s` is a reference. Let’s add some explanatory annotations:
-->

De la même manière, la signature de la fonction utilise `&` pour indiquer que
le type du paramètre `s` est une référence. Ajoutons quelques commentaires
explicatifs :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-08-reference-with-annotations/src/main.rs:here}}
```

<!--
The scope in which the variable `s` is valid is the same as any function
parameter’s scope, but we don’t drop what the reference points to when it goes
out of scope because we don’t have ownership. When functions have references as
parameters instead of the actual values, we won’t need to return the values in
order to give back ownership, because we never had ownership.
-->

La portée dans laquelle la variable `s` est en vigueur est la même que toute
portée d'un paramètre de fonction, mais nous ne libérons pas ce sur quoi cette
référence pointe quand elle sort de la portée, car nous n'en prenons pas
possession. Lorsque les fonctions ont des références en paramètres au lieu des
valeurs réelles, nous n'avons pas besoin de retourner les valeurs pour les
rendre, car nous n'en avons jamais pris possession.

<!--
We call having references as function parameters *borrowing*. As in real life,
if a person owns something, you can borrow it from them. When you’re done, you
have to give it back.
-->

Quand nous avons des références dans les paramètres d'une fonction, nous
appelons cela *l'emprunt*. Comme dans la vie réelle, quand un objet appartient
à quelqu'un, vous pouvez le lui emprunter. Et quand vous avez fini, vous devez
le lui rendre.

<!--
So what happens if we try to modify something we’re borrowing? Try the code in
Listing 4-6. Spoiler alert: it doesn’t work!
-->

Donc qu'est-ce qui se passe si nous essayons de modifier quelque chose que nous
empruntons ? Essayez le code dans l'encart 4-6. Attention, spoiler : cela ne
fonctionne pas !

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/listing-04-06/src/main.rs}}
```

<!--
<span class="caption">Listing 4-6: Attempting to modify a borrowed value</span>
-->

<span class="caption">Entrée 4-6 : Tentative de modification d'une valeur
empruntée.</span>

<!--
Here’s the error:
-->

Voici l'erreur :

<!--
```console
{{#include ../listings-sources/ch04-understanding-ownership/listing-04-06/output.txt}}
```
-->

```console
{{#include ../listings/ch04-understanding-ownership/listing-04-06/output.txt}}
```

<!--
Just as variables are immutable by default, so are references. We’re not
allowed to modify something we have a reference to.
-->

Comme les variables sont immuables par défaut, les références le sont aussi.
Nous ne sommes pas autorisés à modifier une chose quand nous avons une référence
vers elle.

<!--
### Mutable References
-->

### Les références mutables

<!--
We can fix the error in the code from Listing 4-6 with just a small tweak:
-->

Nous pouvons résoudre l'erreur du code de l'encart 4-6 avec une petite
modification :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-09-fixes-listing-04-06/src/main.rs}}
```

<!--
First, we had to change `s` to be `mut`. Then we had to create a mutable
reference with `&mut s` and accept a mutable reference with `some_string: &mut
String`.
-->

D'abord, nous avons dû préciser que `s` est `mut`. Ensuite, nous avons dû
créer une référence mutable avec `&mut s` et accepter de prendre une référence
mutable avec `texte: &mut String`.

<!--
But mutable references have one big restriction: you can have only one mutable
reference to a particular piece of data in a particular scope. This code will
fail:
-->

Mais les références mutables ont une grosse contrainte : vous ne pouvez avoir
qu'une seule référence mutable pour chaque donnée dans chaque portée. Le code
suivant va échouer :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/src/main.rs:here}}
```

<!--
Here’s the error:
-->

Voici l'erreur :

<!--
```console
{{#include ../listings-sources/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```
-->

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-10-multiple-mut-not-allowed/output.txt}}
```

<!--
This restriction allows for mutation but in a very controlled fashion. It’s
something that new Rustaceans struggle with, because most languages let you
mutate whenever you’d like.
-->

Cette contrainte autorise les mutations, mais de manière très contrôlée. C'est
quelque chose que les nouveaux Rustacés ont du mal à surmonter, car la plupart
des langages vous permettent de modifier les données quand vous le voulez.

<!--
The benefit of having this restriction is that Rust can prevent data races at
compile time. A *data race* is similar to a race condition and happens when
these three behaviors occur:
-->

L'avantage d'avoir cette contrainte est que Rust peut empêcher les accès
concurrents au moment de la compilation. Un *accès concurrent* est une situation
de concurrence qui se produit lorsque ces trois facteurs se combinent :

<!--
* Two or more pointers access the same data at the same time.
* At least one of the pointers is being used to write to the data.
* There’s no mechanism being used to synchronize access to the data.
-->

* Deux pointeurs ou plus accèdent à la même donnée au même moment.
* Au moins un des pointeurs est utilisé pour écrire dans cette donnée.
* On n'utilise aucun mécanisme pour synchroniser l'accès aux données.

<!--
Data races cause undefined behavior and can be difficult to diagnose and fix
when you’re trying to track them down at runtime; Rust prevents this problem
from happening because it won’t even compile code with data races!
-->

L'accès concurrent provoque des comportements indéfinis et rend difficile le
diagnostic et la résolution de problèmes lorsque vous essayez de les reproduire
au moment de l'exécution ; Rust évite ce problème parce qu'il ne va pas compiler
du code avec des accès concurrents !

<!--
As always, we can use curly brackets to create a new scope, allowing for
multiple mutable references, just not *simultaneous* ones:
-->

Comme d'habitude, nous pouvons utiliser des accolades pour créer une nouvelle
portée, pour nous permettre d'avoir plusieurs références mutables, mais pas
*en même temps* :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-11-muts-in-separate-scopes/src/main.rs:here}}
```

<!--
A similar rule exists for combining mutable and immutable references. This code
results in an error:
-->

Une règle similaire existe pour combiner les références immuables et mutables.
Ce code va mener à une erreur :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/src/main.rs:here}}
```

<!--
Here’s the error:
-->

Voici l'erreur :

<!--
```console
{{#include ../listings-sources/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```
-->

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-12-immutable-and-mutable-not-allowed/output.txt}}
```

<!--
Whew! We *also* cannot have a mutable reference while we have an immutable one.
Users of an immutable reference don’t expect the values to suddenly change out
from under them! However, multiple immutable references are okay because no one
who is just reading the data has the ability to affect anyone else’s reading of
the data.
-->

Ouah ! Nous ne pouvons pas *non plus* avoir une référence mutable pendant que
nous en avons une autre immuable. Les utilisateurs d'une référence immuable ne
s'attendent pas à ce que sa valeur change soudainement ! Cependant,
l'utilisation de plusieurs références immuables ne pose pas de problème, car
simplement lire une donnée ne va pas affecter la lecture de la donnée par les
autres.

<!--
Note that a reference’s scope starts from where it is introduced and continues
through the last time that reference is used. For instance, this code will
compile because the last usage of the immutable references occurs before the
mutable reference is introduced:
-->

Notez bien que la portée d'une référence commence dès qu'elle est introduite et
se poursuit jusqu'au dernier endroit où cette référence est utilisée. Par
exemple, le code suivant va se compiler car la dernière utilisation de la
référence immuable est située avant l'introduction de la référence mutable :

<!--
```rust,edition2018
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```
-->

```rust,edition2018
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-13-reference-scope-ends/src/main.rs:here}}
```

<!--
The scopes of the immutable references `r1` and `r2` end after the `println!`
where they are last used, which is before the mutable reference `r3` is
created. These scopes don’t overlap, so this code is allowed.
-->

Les portées des références immuables `r1` et `r2` se terminent après le
`println!` où elles sont utilisées pour la dernière fois, c'est-à-dire avant que
la référence mutable `r3` soit créée. Ces portées ne se chevauchent pas, donc ce
code est autorisé.

<!--
Even though borrowing errors may be frustrating at times, remember that it’s
the Rust compiler pointing out a potential bug early (at compile time rather
than at runtime) and showing you exactly where the problem is. Then you don’t
have to track down why your data isn’t what you thought it was.
-->

Même si ces erreurs d'emprunt peuvent parfois être frustrantes, n'oubliez pas
que le compilateur de Rust nous signale un bogue potentiel en avance (au moment
de la compilation plutôt que l'exécution) et vous montre où se situe exactement
le problème. Ainsi, vous n'avez pas à chercher pourquoi vos données ne
correspondent pas à ce que vous pensiez qu'elles devraient être.

<!--
### Dangling References
-->

### Les références pendouillantes

<!--
In languages with pointers, it’s easy to erroneously create a *dangling
pointer*, a pointer that references a location in memory that may have been
given to someone else, by freeing some memory while preserving a pointer to
that memory. In Rust, by contrast, the compiler guarantees that references will
never be dangling references: if you have a reference to some data, the
compiler will ensure that the data will not go out of scope before the
reference to the data does.
-->

Avec les langages qui utilisent les pointeurs, il est facile de créer par erreur
un *pointeur pendouillant* (*dangling pointer*), qui est un pointeur qui pointe
vers un emplacement mémoire qui a été donné à quelqu'un d'autre, en libérant de
la mémoire tout en conservant un pointeur vers cette mémoire. En revanche, avec
Rust, le compilateur garantit que les références ne seront jamais des références
pendouillantes : si nous avons une référence vers une donnée, le compilateur va
s'assurer que cette donnée ne va pas sortir de la portée avant que la référence
vers cette donnée en soit elle-même sortie.

<!--
Let’s try to create a dangling reference, which Rust will prevent with a
compile-time error:
-->

Essayons de créer une référence pendouillante, ce que Rust va empêcher avec une
erreur au moment de la compilation :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/src/main.rs}}
```

<!--
Here’s the error:
-->

Voici l'erreur :

<!--
```console
{{#include ../listings-sources/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```
-->

```console
{{#include ../listings/ch04-understanding-ownership/no-listing-14-dangling-reference/output.txt}}
```

<!--
This error message refers to a feature we haven’t covered yet: lifetimes. We’ll
discuss lifetimes in detail in Chapter 10. But, if you disregard the parts
about lifetimes, the message does contain the key to why this code is a problem:
-->

Ce message d'erreur fait référence à une fonctionnalité que nous n'avons pas
encore vue : les *durées de vie*. Nous aborderons les durées de vie dans le
chapitre 10. Mais, si vous mettez de côté les parties qui parlent de durées de
vie, le message explique pourquoi le code pose problème :

<!--
```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```
-->

```text
this function's return type contains a borrowed value, but there is no value
for it to be borrowed from.
```

Ce qui peut se traduire par :

```text
Le type de retour de cette fonction contient une valeur empruntée, mais il n'y a
plus aucune valeur qui peut être empruntée.
```

<!--
Let’s take a closer look at exactly what’s happening at each stage of our
`dangle` code:
-->

Regardons de plus près ce qui se passe exactement à chaque étape de notre code
de `pendouille` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-15-dangling-reference-annotated/src/main.rs:here}}
```

<!--
Because `s` is created inside `dangle`, when the code of `dangle` is finished,
`s` will be deallocated. But we tried to return a reference to it. That means
this reference would be pointing to an invalid `String`. That’s no good! Rust
won’t let us do this.
-->

Comme `s` est créé dans `pendouille`, lorsque le code de `pendouille` est
terminé, la variable `s` sera désallouée. Mais nous avons essayé de retourner
une référence vers elle. Cela veut dire que cette référence va pointer vers une
`String` invalide. Ce n'est pas bon ! Rust ne nous laissera pas faire cela.

<!--
The solution here is to return the `String` directly:
-->

Ici la solution est de renvoyer la `String` directement :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch04-understanding-ownership/no-listing-16-no-dangle/src/main.rs:here}}
```

<!--
This works without any problems. Ownership is moved out, and nothing is
deallocated.
-->

Cela fonctionne sans problème. La possession est transférée à la valeur de
retour de la fonction, et rien n'est désalloué.

<!--
### The Rules of References
-->

### Les règles de référencement

<!--
Let’s recap what we’ve discussed about references:
-->

Récapitulons ce que nous avons vu à propos des références :

<!--
* At any given time, you can have *either* one mutable reference *or* any
  number of immutable references.
* References must always be valid.
-->

* À un instant donné, vous pouvez avoir *soit* une référence mutable, *soit* un
  nombre quelconque de références immuables.
* Les références doivent toujours être en vigueur.

<!--
Next, we’ll look at a different kind of reference: slices.
-->

Ensuite, nous aborderons un autre type de référence : les *slices*.
