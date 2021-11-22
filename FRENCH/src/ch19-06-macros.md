> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/212).

<!--
## Macros
-->

## Les macros

<!--
We’ve used macros like `println!` throughout this book, but we haven’t fully
explored what a macro is and how it works. The term *macro* refers to a family
of features in Rust: *declarative* macros with `macro_rules!` and three kinds
of *procedural* macros:
-->

Nous avons déjà utilisé des macros tout au long de ce livre, comme `println!`,
mais nous n'avons pas exploré en profondeur ce qu'est une macro et comment
elles fonctionnent. Le terme *macro* revoit à une famille de fonctionnalités de
Rust : les macros *déclaratives* avec `macro_rules!` et trois types de macros
*procédurales* :

<!--
* Custom `#[derive]` macros that specify code added with the `derive` attribute
  used on structs and enums
* Attribute-like macros that define custom attributes usable on any item
* Function-like macros that look like function calls but operate on the tokens
  specified as their argument
-->

* Des macros `#[derive]` personnalisées qui renseigne du code ajouté grâce à
  l'attribut `derive` utilisé sur les structures et les énumérations
* Les macros qui ressemblent à des attributs qui définissent des attributs
  personnalisés qui sont utilisables sur n'importe quel élément
* Les macros qui ressemblent à des fonctions mais qui opèrent sur les éléments
  renseignés en argument

<!--
We’ll talk about each of these in turn, but first, let’s look at why we even
need macros when we already have functions.
-->

Nous allons voir chacune d'entre elles à leur tour, mais avant, posons-nous la
question de pourquoi nous avons besoin de macros alors que nous avons déjà les
fonctions.

<!--
### The Difference Between Macros and Functions
-->

### La différence entre les macros et les fonctions

<!--
Fundamentally, macros are a way of writing code that writes other code, which
is known as *metaprogramming*. In Appendix C, we discuss the `derive`
attribute, which generates an implementation of various traits for you. We’ve
also used the `println!` and `vec!` macros throughout the book. All of these
macros *expand* to produce more code than the code you’ve written manually.
-->

Essentiellement, les macros sont une façon d'écrire du code qui écrit un autre
code, ce qui s'appelle la *métaprogrammation*. Dans l'annexe C, nous verrons
l'attribut `derive`, qui génère une implémentation de différents traits pour
vous. Nous avons aussi utilisé les macros `println!` et `vec!` dans ce livre.
Toutes ces macros *se déploient* pour produire plus de code que celui que vous
avez écrit manuellement.

<!--
Metaprogramming is useful for reducing the amount of code you have to write and
maintain, which is also one of the roles of functions. However, macros have
some additional powers that functions don’t.
-->

La métaprogrammation est utile pour réduire la quantité de code que vous avez à
écrire et à maintenir, ce qui est aussi un des rôles des fonctions. Cependant,
les macros ont quelques pouvoirs en plus que les fonctions n'ont pas.

<!--
A function signature must declare the number and type of parameters the
function has. Macros, on the other hand, can take a variable number of
parameters: we can call `println!("hello")` with one argument or
`println!("hello {}", name)` with two arguments. Also, macros are expanded
before the compiler interprets the meaning of the code, so a macro can, for
example, implement a trait on a given type. A function can’t, because it gets
called at runtime and a trait needs to be implemented at compile time.
-->

La signature d'une fonction doit déclarer le nombre et le type de paramètres
qu'à cette fonction. Les macros, d'un autre côté, peuvent prendre un nombre
variable de paramètres : nous pouvons faire appel à `println!("salut")` avec un
seul argument, ou `println!("salut {}", nom)` avec deux arguments. Aussi, les
macros se déploient avant que le compilateur interprète la signification du
code, donc une macro peut, par exemple, implémenter un trait sur un type donné.
Une fonction ne peut pas le faire, car elle est exécutée à l'exécution et un
trait doit être implémenté à la compilation.

<!--
The downside to implementing a macro instead of a function is that macro
definitions are more complex than function definitions because you’re writing
Rust code that writes Rust code. Due to this indirection, macro definitions are
generally more difficult to read, understand, and maintain than function
definitions.
-->

Le désavantage d'implémenter une macro par rapport à une fonction est que les
définitions de macros sont plus complexes que les définitions de fonction car
vous écrivez du code Rust qui écrit lui-même du code Rust. A cause de cette
approche, les définitions de macro sont généralement plus difficiles à lire,
comprendre et maintenir que les définitions de fonctions.

<!--
Another important difference between macros and functions is that you must
define macros or bring them into scope *before* you call them in a file, as
opposed to functions you can define anywhere and call anywhere.
-->

Une autre différence importante entre les macros et les fonctions est que vous
devez définir les macros ou les importer dans la portée *avant* de les utiliser
dans le fichier, contrairement aux fonctions que vous pouvez définir n'importe
où et y faire appel n'importe où.

<!--
### Declarative Macros with `macro_rules!` for General Metaprogramming
-->

### Les macros déclaratives avec `macro_rules!` pour la métaprogrammation générale

<!--
The most widely used form of macros in Rust is *declarative macros*. These are
also sometimes referred to as “macros by example,” “`macro_rules!` macros,” or
just plain “macros.” At their core, declarative macros allow you to write
something similar to a Rust `match` expression. As discussed in Chapter 6,
`match` expressions are control structures that take an expression, compare the
resulting value of the expression to patterns, and then run the code associated
with the matching pattern. Macros also compare a value to patterns that are
associated with particular code: in this situation, the value is the literal
Rust source code passed to the macro; the patterns are compared with the
structure of that source code; and the code associated with each pattern, when
matched, replaces the code passed to the macro. This all happens during
compilation.
-->

La forme la plus utilisée de macro en Rust est la *macro déclarative*. Elles
sont parfois appelées “macros définies par un exemple”,
“macros `macro_rules!`”, ou simplement “macros”. Au fond, les macros
déclaratives vous permettent d'écrire quelque chose de similaire à une
expression `match` de Rust. Comme nous l'avons vu au chapitre 6, les
expressions `match` sont des structures de contrôle qui prennent en argument
une expression, compare la valeur qui en résulte avec les motifs, et ensuite
exécute le code associé au motif qui correspond. Les macros comparent elles
aussi une valeur avec des motifs qui sont associés à code particulier : dans
cette situation, la valeur est littéralement le code source Rust envoyé à la
macro ; les motifs sont comparés avec la structure de ce code source ; et le
code associé à chaque motif vient remplacer le code passé à la macro, lorsqu'il
correspond. Tout ceci se passe lors de la compilation.

<!--
To define a macro, you use the `macro_rules!` construct. Let’s explore how to
use `macro_rules!` by looking at how the `vec!` macro is defined. Chapter 8
covered how we can use the `vec!` macro to create a new vector with particular
values. For example, the following macro creates a new vector containing three
integers:
-->

Pour définir une macro, il faut utiliser la construction `macro_rules!`.
Explorons l'utilisation de `macro_rules!` en observant comment la macro `vec!`
est définie. Le chapitre 8 nous a permis de comprendre comment utiliser la
macro `vec!` pour créer un nouveau vecteur avec des valeurs précises. Par
exemple, la macro suivante crée un nouveau vecteur qui contient trois entiers :

<!--
```rust
let v: Vec<u32> = vec![1, 2, 3];
```
-->

```rust
let v: Vec<u32> = vec![1, 2, 3];
```

<!--
We could also use the `vec!` macro to make a vector of two integers or a vector
of five string slices. We wouldn’t be able to use a function to do the same
because we wouldn’t know the number or type of values up front.
-->

Nous aurions pu aussi utiliser la macro `vec!` pour créer un vecteur de deux
entiers ou un vecteur de cinq slices de chaînes de caractères. Nous n'aurions
pas pu utiliser une fonction pour faire la même chose car nous n'aurions pas pu
connaître le nombre ou le type de valeurs au départ.

<!--
Listing 19-28 shows a slightly simplified definition of the `vec!` macro.
-->

L'encart 19-28 montre une définition légèrement simplifiée de la macro `vec!`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-28/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-28/src/lib.rs}}
```

<!--
<span class="caption">Listing 19-28: A simplified version of the `vec!` macro
definition</span>
-->

<span class="caption">Encart 19-28 : une version simplifiée de la définition de
la macro `vec!`</span>

<!--
> Note: The actual definition of the `vec!` macro in the standard library
> includes code to preallocate the correct amount of memory up front. That code
> is an optimization that we don’t include here to make the example simpler.
-->

> Remarque : la définition actuelle de la macro `vec!` de la bibliothèque
> standard embarque du code pour pré-allouer la bonne quantité de mémoire en
> amont. Ce code est une optimisation que nous n'allons pas intégrer ici pour
> simplifier l'exemple.

<!--
The `#[macro_export]` annotation indicates that this macro should be made
available whenever the crate in which the macro is defined is brought into
scope. Without this annotation, the macro can’t be brought into scope.
-->

L'annotation `#[macro_export]` indique que cette macro doit être disponible à
chaque fois que la crate dans laquelle la macro est définie est importée dans
la portée. Sans cette annotation, la macro ne pourrait pas être importée dans
la portée.

<!--
We then start the macro definition with `macro_rules!` and the name of the
macro we’re defining *without* the exclamation mark. The name, in this case
`vec`, is followed by curly brackets denoting the body of the macro definition.
-->

Ensuite, nous commençons la définition de la macro avec `macro_rules!` suivi du
nom de la macro que nous définissons *sans* le point d'exclamation. Le nom, qui
dans ce cas est `vec`, est suivi par des accolades indiquant le corps de la
définition de la macro.

<!--
The structure in the `vec!` body is similar to the structure of a `match`
expression. Here we have one arm with the pattern `( $( $x:expr ),* )`,
followed by `=>` and the block of code associated with this pattern. If the
pattern matches, the associated block of code will be emitted. Given that this
is the only pattern in this macro, there is only one valid way to match; any
other pattern will result in an error. More complex macros will have more than
one arm.
-->

La structure dans le corps de `vec!` ressemble à la structure d'une expression
`match`. Ici nous avons une branche avec le motif `( $( $x:expr ), * )`, suivi
par `=>` et le code du bloc associé à ce motif. Si le motif correspond, le bloc
de code associé sera déployé. Etant donné que c'est le seul motif dans cette
macro, il n'y a qu'une seule bonne façon d'y correspondre ; tout autre motif va
déboucher sur une erreur. Des macros plus complexes auront plus qu'une seule
branche.

<!--
Valid pattern syntax in macro definitions is different than the pattern syntax
covered in Chapter 18 because macro patterns are matched against Rust code
structure rather than values. Let’s walk through what the pattern pieces in
Listing 19-28 mean; for the full macro pattern syntax, see [the reference].
-->

La syntaxe correcte pour un motif dans les définitions de macros est différente
de la syntaxe de motif que nous avons vu au chapitre 18 car les motifs de
macros sont comparés à des structures de code Rust plutôt qu'à des valeurs.
Examinons la signification des éléments du motif de l'encart 19-28 ; pour voir
l'intégralité de la syntaxe du motif de la macro, rendez-vous
[à la documentation][the reference].

<!--
[the reference]: ../reference/macros-by-example.html
-->

[the reference]: https://doc.rust-lang.org/reference/macros-by-example.html

<!--
First, a set of parentheses encompasses the whole pattern. A dollar sign (`$`)
is next, followed by a set of parentheses that captures values that match the
pattern within the parentheses for use in the replacement code. Within `$()` is
`$x:expr`, which matches any Rust expression and gives the expression the name
`$x`.
-->

Premièrement, un jeu de parenthèses englobent l'intégralité du motif. Ensuite
vient le symbole dollar (`$`), suivi par un jeu de parenthèses qui capturent les
valeurs qui correspondent au motif entre les parenthèses pour les utiliser dans
le code de remplacement. A l'intérieur du `$()` nous avons `$x:expr`, qui
correspond à n'importe quelle expression Rust et donne le nom `$x` à
l'expression.

<!--
The comma following `$()` indicates that a literal comma separator character
could optionally appear after the code that matches the code in `$()`. The `*`
specifies that the pattern matches zero or more of whatever precedes the `*`.
-->

La virgule qui suit le `$()` signifie que cette virgule littérale comme
caractère littéral de séparation peut optionnellement apparaître après le code
qui correspond au code du `$()`. Le `*` informe que ce motif correspond à zéro
ou plus éléments répétés correspondant à ce qui précède ce `*`.

<!--
When we call this macro with `vec![1, 2, 3];`, the `$x` pattern matches three
times with the three expressions `1`, `2`, and `3`.
-->

Lorsque nous faisons appel à cette macro avec `vec![1, 2, 3];`, le motif `$x`
correspond à trois reprises avec les trois expressions `1`, `2`, et `3`.

<!--
Now let’s look at the pattern in the body of the code associated with this arm:
`temp_vec.push()` within `$()*` is generated for each part that matches `$()`
in the pattern zero or more times depending on how many times the pattern
matches. The `$x` is replaced with each expression matched. When we call this
macro with `vec![1, 2, 3];`, the code generated that replaces this macro call
will be the following:
-->

Maintenant, penchons-nous sur le motif dans le corps du code associé à cette
branche : `temp_vec.push()` dans le `$()*` est généré pour chacune des parties
qui correspondent au `$()` dans le motif pour zéro ou plus de fois, en fonction
de combien de fois le motif correspond. Le `$x` est remplacé par chaque
expression qui correspond. Lorsque nous faisons appel à cette macro avec
`vec![1, 2, 3];`, le code généré qui remplace cet appel de macro ressemblera à
ceci :

<!--
```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```
-->

```rust,ignore
{
    let mut temp_vec = Vec::new();
    temp_vec.push(1);
    temp_vec.push(2);
    temp_vec.push(3);
    temp_vec
}
```

<!--
We’ve defined a macro that can take any number of arguments of any type and can
generate code to create a vector containing the specified elements.
-->

Nous avons défini une macro qui peut prendre n'importe quel nombre d'arguments
de n'importe quel type et qui peut générer du code pour créer un vecteur qui
contient les éléments renseignés.

<!--
There are some strange edge cases with `macro_rules!`. In the future, Rust will
have a second kind of declarative macro that will work in a similar fashion but
fix some of these edge cases. After that update, `macro_rules!` will be
effectively deprecated. With this in mind, as well as the fact that most Rust
programmers will *use* macros more than *write* macros, we won’t discuss
`macro_rules!` any further. To learn more about how to write macros, consult
the online documentation or other resources, such as [“The Little Book of Rust
Macros”][tlborm] started by Daniel Keep and continued by Lukas Wirth.
-->

Il subsiste quelques cas marginaux étranges avec `macro_rules!`. Bientôt, Rust
rajoutera un second type de macro déclarative qui fonctionnera de la même
manière mais qui corrigera ces cas marginaux. Après cette mise à jour,
`macro_rules!` sera dépréciée. En sachant cela, ainsi que le fait que la
plupart des développeurs Rust vont plus *utiliser* les macros *qu'écrire* des
macros, nous ne verrons plus à nouveau `macro_rules!` à l'avenir. Pour en
savoir plus sur l'écriture des macros, consultez la documentation en ligne, ou
d'autres ressources comme [“The Little Book of Rust Macros”][tlborm], débuté
par Daniel Keep et continué par Lukas Wirth.

<!--
[tlborm]: https://veykril.github.io/tlborm/
-->

[tlborm]: https://veykril.github.io/tlborm/

<!--
### Procedural Macros for Generating Code from Attributes
-->

### Les macros procédurales pour générer du code à partir des attributs

<!--
The second form of macros is *procedural macros*, which act more like functions
(and are a type of procedure). Procedural macros accept some code as an input,
operate on that code, and produce some code as an output rather than matching
against patterns and replacing the code with other code as declarative macros
do.
-->

La seconde forme de macro est la *macro procédurale*, qui se comporte plus
comme des fonctions (et sont un type de procédure). Les macros procédurales
prennent du code en entrée, travaille sur ce code, et produit du code en sortie
plutôt que de faire des correspondances sur des motifs et remplacer du code
avec un autre code, comme le font les macros déclaratives.

<!--
The three kinds of procedural macros (custom derive, attribute-like, and
function-like) all work in a similar fashion.
-->

Les trois types de macros procédurales (les dérivées personnalisées, celles qui
ressemblent aux attributs, et celles qui ressemblent à des fonctions)
fonctionnent toutes de la même manière.

<!--
When creating procedural macros, the definitions must reside in their own crate
with a special crate type. This is for complex technical reasons that we hope
to eliminate in the future. Using procedural macros looks like the code in
Listing 19-29, where `some_attribute` is a placeholder for using a specific
macro.
-->

Lorsque vous créez une macro procédurale, les définitions doivent être rangées
dans leur propre crate avec un type spécial de crate. Ceci pour des raisons
techniques complexes que nous espérons supprimer dans l'avenir. L'utilisation
des macros procédurales ressemble au code de l'encart 19-29, dans lequel
`un_attribut_quelconque` est un emplacement pour l'utilisation d'une macro
spécifique.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
use proc_macro;

#[some_attribute]
pub fn some_name(input: TokenStream) -> TokenStream {
}
```
-->

```rust,ignore
use proc_macro;

#[un_attribut_quelconque]
pub fn un_nom_quelconque(entree: TokenStream) -> TokenStream {
}
```

<!--
<span class="caption">Listing 19-29: An example of using a procedural
macro</span>
-->

<span class="caption">Encart 19-29 : un exemple d'utilisation d'une macro
procédurale</span>

<!--
The function that defines a procedural macro takes a `TokenStream` as an input
and produces a `TokenStream` as an output. The `TokenStream` type is defined by
the `proc_macro` crate that is included with Rust and represents a sequence of
tokens. This is the core of the macro: the source code that the macro is
operating on makes up the input `TokenStream`, and the code the macro produces
is the output `TokenStream`. The function also has an attribute attached to it
that specifies which kind of procedural macro we’re creating. We can have
multiple kinds of procedural macros in the same crate.
-->

La fonction qui définit une macro procédurale prend un `TokenStream` en entrée
et produit un `TokenStream` en sortie. Le type `TokenStream` est défini par la
crate `proc_macro` qui est fournie par Rust et représente une séquence de
jetons. C'est le cœur de la macro : le code source sur lequel la macro opère
compose l'entrée `TokenStream`, et le code que la macro produit est une sortie
`TokenStream`. La fonction a aussi un attribut qui lui est rattaché qui
renseigne quel genre de macro procédurale nous créons. Nous pouvons avoir
différents types de macros procédurales dans la même crate.

<!--
Let’s look at the different kinds of procedural macros. We’ll start with a
custom derive macro and then explain the small dissimilarities that make the
other forms different.
-->

Voyons maintenant les différents types de macros procédurales. Nous allons
commencer par une macro dérivée personnalisée et nous expliquerons ensuite les
petites différences avec les autres types.

<!--
### How to Write a Custom `derive` Macro
-->

### Comment écrire une macro dérivée personnalisée

<!--
Let’s create a crate named `hello_macro` that defines a trait named
`HelloMacro` with one associated function named `hello_macro`. Rather than
making our crate users implement the `HelloMacro` trait for each of their
types, we’ll provide a procedural macro so users can annotate their type with
`#[derive(HelloMacro)]` to get a default implementation of the `hello_macro`
function. The default implementation will print `Hello, Macro! My name is
TypeName!` where `TypeName` is the name of the type on which this trait has
been defined. In other words, we’ll write a crate that enables another
programmer to write code like Listing 19-30 using our crate.
-->

Créons une crate `hello_macro` qui définit un trait qui s'appelle `HelloMacro`
avec une fonction associée `hello_macro`. Plutôt que de contraindre les
utilisateurs de notre crate à implémenter le trait `HelloMacro` sur chacun de
leurs types, nous allons fournir une macro procédurale qui permettra aux
utilisateurs de pouvoir annoter leur type avec `#[derive(HelloMacro)]` afin
d'obtenir une implémentation par défaut de la fonction `hello_macro`.
L'implémentation par défaut affichera `Hello, Macro ! Mon nom est TypeName !`,
dans lequel `TypeName` est le nom du type sur lequel ce trait a été défini.
Autrement dit, nous allons écrire une crate qui permet à un autre développeur
d'écrire du code comme l'encart 19-30 en utilisant notre crate.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-30/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-30/src/main.rs}}
```

<!--
<span class="caption">Listing 19-30: The code a user of our crate will be able
to write when using our procedural macro</span>
-->

<span class="caption">Encart 19-30 : le code qu'un utilisateur de notre crate
pourra écrire lorsqu'il utilisera notre macro procédurale</span>

<!--
This code will print `Hello, Macro! My name is Pancakes!` when we’re done. The
first step is to make a new library crate, like this:
-->

Ce code va afficher `Hello, Macro ! Mon nom est Pancakes !` lorsque vous en
aurez fini. La première étape est de créer une nouvelle crate de bibliothèque,
comme ceci :

<!--
```console
$ cargo new hello_macro --lib
```
-->

```console
$ cargo new hello_macro --lib
```

<!--
Next, we’ll define the `HelloMacro` trait and its associated function:
-->

Ensuite, nous allons définir le trait `HelloMacro` et sa fonction associée :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/hello_macro/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/hello_macro/src/lib.rs}}
```

<!--
We have a trait and its function. At this point, our crate user could implement
the trait to achieve the desired functionality, like so:
-->

Nous avons maintenant un trait et sa fonction. A partir de là, notre
utilisateur de la crate peut implémenter le trait pour accomplir la
fonctionnalité souhaitée, comme ceci :

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/pancakes/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/no-listing-20-impl-hellomacro-for-pancakes/pancakes/src/main.rs}}
```

<!--
However, they would need to write the implementation block for each type they
wanted to use with `hello_macro`; we want to spare them from having to do this
work.
-->

Cependant, l'utilisateur doit écrire le bloc d'implémentation pour chacun des
types qu'il souhaite utiliser avec `hello_macro` ; nous souhaitons leur
épargner de faire ce travail.

<!--
Additionally, we can’t yet provide the `hello_macro` function with default
implementation that will print the name of the type the trait is implemented
on: Rust doesn’t have reflection capabilities, so it can’t look up the type’s
name at runtime. We need a macro to generate code at compile time.
-->

De plus, nous ne pouvons pas encore fournir la fonction `hello_macro` avec
l'implémentation par défaut qui va afficher le nom du type du trait sur lequel
nous l'implémentons : Rust n'est pas réflexif, donc il ne peut pas connaître le
nom du type à l'exécution. Nous avons besoin d'une macro pour générer le code à
la compilation.

<!--
The next step is to define the procedural macro. At the time of this writing,
procedural macros need to be in their own crate. Eventually, this restriction
might be lifted. The convention for structuring crates and macro crates is as
follows: for a crate named `foo`, a custom derive procedural macro crate is
called `foo_derive`. Let’s start a new crate called `hello_macro_derive` inside
our `hello_macro` project:
-->

La prochaine étape est de définir la macro procédurale. A l'heure de l'écriture
de ces lignes, les macros procédurales au besoin d'être placées dans leur
propre crate. Cette restriction pourra éventuellement être levée. La convention
pour structurer les crates et les crates de macros est la suivante : pour
une crate `foo`, une crate de macro procédurale personnalisée de dérivée doit
s'appeler `foo_derive`. Créons une nouvelle crate `hello_macro_derive` au sein
de notre projet `hello_macro` :

<!--
```console
$ cargo new hello_macro_derive --lib
```
-->

```console
$ cargo new hello_macro_derive --lib
```

<!--
Our two crates are tightly related, so we create the procedural macro crate
within the directory of our `hello_macro` crate. If we change the trait
definition in `hello_macro`, we’ll have to change the implementation of the
procedural macro in `hello_macro_derive` as well. The two crates will need to
be published separately, and programmers using these crates will need to add
both as dependencies and bring them both into scope. We could instead have the
`hello_macro` crate use `hello_macro_derive` as a dependency and re-export the
procedural macro code. However, the way we’ve structured the project makes it
possible for programmers to use `hello_macro` even if they don’t want the
`derive` functionality.
-->

Nos deux crates sont étroitement liées, donc nous créons la crate de macro
procédurale à l'intérieur du dossier de notre crate `hello_macro`. Si nous
changeons la définition du trait dans `hello_macro`, nous aurons aussi à
changer l'implémentation de la macro procédurale dans `hello_macro_derive`. Les
deux crates vont devoir être publiées séparément, et les développeurs qui vont
utiliser ces crates vont avoir besoin d'ajouter les deux dépendances et les
importer dans la portée. Nous pouvons plutôt faire en sorte que la crate
`hello_macro` utilise `hello_macro_derive` comme dépendance et ré-exporter le
code de la macro procédurale. Cependant, la façon dont nous avons structuré le
projet donne la possibilité aux développeurs d'utiliser `hello_macro` même s'ils
ne veulent pas la fonctionnalité `derive`.

<!--
We need to declare the `hello_macro_derive` crate as a procedural macro crate.
We’ll also need functionality from the `syn` and `quote` crates, as you’ll see
in a moment, so we need to add them as dependencies. Add the following to the
*Cargo.toml* file for `hello_macro_derive`:
-->

Nous devons déclarer la crate `hello_macro_derive` comme étant une crate de
macro procédurale. Nous allons aussi avoir besoin des fonctionnalités des
crates `syn` et `quote`, comme vous allez de constater bientôt, donc nous
allons les ajouter comme dépendances. Ajoutez ceci dans le fichier *Cargo.toml*
de `hello_macro_derive` :

<!--
<span class="filename">Filename: hello_macro_derive/Cargo.toml</span>
-->

<span class="filename">Fichier : hello_macro_derive/Cargo.toml</span>

<!--
```toml
{{#include ../listings-sources/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```
-->

```toml
{{#include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/Cargo.toml:6:12}}
```

<!--
To start defining the procedural macro, place the code in Listing 19-31 into
your *src/lib.rs* file for the `hello_macro_derive` crate. Note that this code
won’t compile until we add a definition for the `impl_hello_macro` function.
-->

Pour commencer à définir la macro procédurale, placez le code de l'encart 19-31
dans votre fichier *src/lib.rs* de la crate `hello_macro_derive`. Notez que ce
code ne se compilera pas tant que nous n'ajouterons pas une définition pour la
fonction `impl_hello_macro`.

<!--
<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>
-->

<span class="filename">Fichier : hello_macro_derive/src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/src/lib.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-31/hello_macro/hello_macro_derive/src/lib.rs}}
```

<!--
<span class="caption">Listing 19-31: Code that most procedural macro crates
will require in order to process Rust code</span>
-->

<span class="caption">Encart 19-31 : du code que la plupart des macros
procédurales auront besoin pour travailler avec du code Rust</span>

<!--
Notice that we’ve split the code into the `hello_macro_derive` function, which
is responsible for parsing the `TokenStream`, and the `impl_hello_macro`
function, which is responsible for transforming the syntax tree: this makes
writing a procedural macro more convenient. The code in the outer function
(`hello_macro_derive` in this case) will be the same for almost every
procedural macro crate you see or create. The code you specify in the body of
the inner function (`impl_hello_macro` in this case) will be different
depending on your procedural macro’s purpose.
-->

Remarquez que nous avons divisé le code dans la fonction `hello_macro_derive`,
qui est responsable de parcourir le `TokenStream`, et la fonction
`impl_hello_macro`, qui est responsable de transformer l'arborescence
syntaxique : cela facilite l'écriture de la macro procédurale. Le code dans la
fonction englobante (qui est `hello_macro_derive` dans notre cas) sera le même
pour presque toutes les crates de macro procédurales que vous allez voir ou
créer. Le code que vous renseignez dans le corps de la fonction (qui est
`impl_hello_macro` dans notre cas) diffèrera en fonction de ce que fait votre
macro procédurale.

<!--
We’ve introduced three new crates: `proc_macro`, [`syn`], and [`quote`]. The
`proc_macro` crate comes with Rust, so we didn’t need to add that to the
dependencies in *Cargo.toml*. The `proc_macro` crate is the compiler’s API that
allows us to read and manipulate Rust code from our code.
-->

Nous avons ajouté trois nouvelles crates : `proc_macro`, [`syn`], et [`quote`].
La crate `proc_macro` est fournie par Rust, donc nous n'avons pas besoin de
l'ajouter aux dépendances dans *Cargo.toml*. La crate `proc_macro` fournit une
API du compilateur qui nous permet de lire et manipuler le code Rust à partir de
notre code.

<!--
[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote
-->

[`syn`]: https://crates.io/crates/syn
[`quote`]: https://crates.io/crates/quote

<!--
The `syn` crate parses Rust code from a string into a data structure that we
can perform operations on. The `quote` crate turns `syn` data structures back
into Rust code. These crates make it much simpler to parse any sort of Rust
code we might want to handle: writing a full parser for Rust code is no simple
task.
-->

La crate `syn` transforme le code Rust d'une chaîne de caractères en une
structure de données sur laquelle nous pouvons procéder à des opérations. La
crate `quote` re-transforme les structures de données de `syn` en code Rust.
Ces crates facilite le parcours de toute sorte de code Rust que nous aurions
besoin de gérer : l'écriture d'un interpréteur complet de code Rust n'a jamais
été aussi facile.

<!--
The `hello_macro_derive` function will be called when a user of our library
specifies `#[derive(HelloMacro)]` on a type. This is possible because we’ve
annotated the `hello_macro_derive` function here with `proc_macro_derive` and
specified the name, `HelloMacro`, which matches our trait name; this is the
convention most procedural macros follow.
-->

La fonction `hello_macro_derive` va être appelée lorsqu'un utilisateur de notre
bibliothèque utilisera `#[derive(HelloMacro)]` sur un type. Cela sera possible
car nous avons annoté notre fonction `hello_macro_derive` avec
`proc_macro_derive` et nous avons renseigné le nom, `HelloMacro`, qui
correspond au nom de notre trait ; c'est la convention que la plupart des
macros procédurales doivent suivre.

<!--
The `hello_macro_derive` function first converts the `input` from a
`TokenStream` to a data structure that we can then interpret and perform
operations on. This is where `syn` comes into play. The `parse` function in
`syn` takes a `TokenStream` and returns a `DeriveInput` struct representing the
parsed Rust code. Listing 19-32 shows the relevant parts of the `DeriveInput`
struct we get from parsing the `struct Pancakes;` string:
-->

La fonction `hello_macro_derive` commence par convertir le `input` qui est un
`TokenStream` en une structure de données que nous pouvons ensuite interpréter
et y faire des opérations dessus. C'est là que `syn` rentre en jeu. La fonction
`parse` de `syn` prend un `TokenStream` et retourne une structure `DeriveInput`
qui représente le code Rust. L'encart 19-32 montre les parties intéressantes de
la structure `DeriveInput` que nous obtenons en convertissant la chaîne de
caractères `struct Pancakes;` :

<!--
```rust,ignore
DeriveInput {
    // --snip--

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```
-->

```rust,ignore
DeriveInput {
    // -- partie masquée ici --

    ident: Ident {
        ident: "Pancakes",
        span: #0 bytes(95..103)
    },
    data: Struct(
        DataStruct {
            struct_token: Struct,
            fields: Unit,
            semi_token: Some(
                Semi
            )
        }
    )
}
```

<!--
<span class="caption">Listing 19-32: The `DeriveInput` instance we get when
parsing the code that has the macro’s attribute in Listing 19-30</span>
-->

<span class="caption">Encart 19-32 : l'instance de `DeriveInput` que nous
obtenons lorsque nous convertissons le code qui l'attribut de la macro dans
l'encart 19-30</span>

<!--
The fields of this struct show that the Rust code we’ve parsed is a unit struct
with the `ident` (identifier, meaning the name) of `Pancakes`. There are more
fields on this struct for describing all sorts of Rust code; check the [`syn`
documentation for `DeriveInput`][syn-docs] for more information.
-->

Les champs de cette structure montrent que ce code Rust que nous avons converti
est une structure unitaire avec l'`ident` (raccourci de `identifier`, qui
désigne le nom) `Pancakes`. Il y a d'autres champs sur cette structure
décrivant toutes sortes de codes Rust ; regardez la
[documentation de `syn` pour `DeriveInput`][syn-docs] pour en savoir plus.

<!--
[syn-docs]: https://docs.rs/syn/1.0/syn/struct.DeriveInput.html
-->

[syn-docs]: https://docs.rs/syn/1.0/syn/struct.DeriveInput.html

<!--
Soon we’ll define the `impl_hello_macro` function, which is where we’ll build
the new Rust code we want to include. But before we do, note that the output
for our derive macro is also a `TokenStream`. The returned `TokenStream` is
added to the code that our crate users write, so when they compile their crate,
they’ll get the extra functionality that we provide in the modified
`TokenStream`.
-->

Bientôt, nous définirons la fonction `impl_hello_macro`, qui nous permettra de
construire le nouveau code Rust que nous souhaitons injecter. Mais avant de faire
cela, remarquez que la sortie de notre macro derive est aussi un `TokenStream`.
Le `TokenStream` retourné est ajouté au code que les utilisateurs de notre
crate ont écrit, donc lorsqu'ils compilent leur crate, ils récupéreront la
fonctionnalité additionnelle que nous injectons dans le `TokenStream` modifié.

<!--
You might have noticed that we’re calling `unwrap` to cause the
`hello_macro_derive` function to panic if the call to the `syn::parse` function
fails here. It’s necessary for our procedural macro to panic on errors because
`proc_macro_derive` functions must return `TokenStream` rather than `Result` to
conform to the procedural macro API. We’ve simplified this example by using
`unwrap`; in production code, you should provide more specific error messages
about what went wrong by using `panic!` or `expect`.
-->

Vous avez peut-être remarqué que nous faisons appel à `unwrap` pour faire
paniquer la fonction `hello_macro_derive` si l'appel à la fonction `syn::parse`
que nous faisons échoue. Il est nécessaire de faire paniquer notre macro
procédurale si elle rencontre des erreurs car les fonctions `proc_macro_derive`
doivent retourner un `TokenStream` plutôt qu'un `Result` pour se conformer à
l'API de la macro procédurale. Nous avons simplifié cet exemple en utilisant
`unwrap` ; dans du code en production, vous devriez renseigner des messages
d'erreur plus précis sur ce qui s'est mal passé en utilisant `panic!` ou
`expect`.

<!--
Now that we have the code to turn the annotated Rust code from a `TokenStream`
into a `DeriveInput` instance, let’s generate the code that implements the
`HelloMacro` trait on the annotated type, as shown in Listing 19-33.
-->

Maintenant que nous avons le code pour transformer le code Rust annoté d'un
`TokenStream` en une instance de `DeriveInput`, créons le code qui implémente le
trait `HelloMacro` sur le type annoté, comme l'encart 19-33.

<!--
<span class="filename">Filename: hello_macro_derive/src/lib.rs</span>
-->

<span class="filename">Fichier : hello_macro_derive/src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch19-advanced-features/listing-19-33/hello_macro/hello_macro_derive/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch19-advanced-features/listing-19-33/hello_macro/hello_macro_derive/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 19-33: Implementing the `HelloMacro` trait using
the parsed Rust code</span>
-->

<span class="caption">Encart 19-33 : implémentation du trait `HelloMacro` en
utilisant le code Rust interprété</span>

<!--
We get an `Ident` struct instance containing the name (identifier) of the
annotated type using `ast.ident`. The struct in Listing 19-32 shows that when
we run the `impl_hello_macro` function on the code in Listing 19-30, the
`ident` we get will have the `ident` field with a value of `"Pancakes"`. Thus,
the `name` variable in Listing 19-33 will contain an `Ident` struct instance
that, when printed, will be the string `"Pancakes"`, the name of the struct in
Listing 19-30.
-->

Nous obtenons une instance de structure `Ident` qui contient le nom
(`identifier`) du type annoté en utilisant `ast.ident`. La structure de l'encart
19-32 montre ceci lorsque nous exécutons la fonction `impl_hello_macro` sur le
code de l'encart 19-30, le `ident` que nous obtenons aura le champ `ident` avec
la valeur `"Pancakes"`. Ainsi, la variable `nom` de l'encart 19-33 contiendra
une instance de la structure `Ident` qui, une fois affichée, sera la chaîne de
caractères `"Pancakes"`, le nom de la structure de l'encart 19-30.

<!--
The `quote!` macro lets us define the Rust code that we want to return. The
compiler expects something different to the direct result of the `quote!`
macro’s execution, so we need to convert it to a `TokenStream`. We do this by
calling the `into` method, which consumes this intermediate representation and
returns a value of the required `TokenStream` type.
-->

La macro `quote!` nous permet de définir le code Rust que nous souhaitons
retourner. Le compilateur attend quelque chose de différent en résultat que
ce qui correspond à l'exécution de `quote!`, donc nous devons le convertir en
`TokenStream`. Nous faisons ceci en faisant appel à la méthode `into`, qui se
base sur cette représentation intermédiaire et retourne une valeur attendue,
comme ici du type `TokenStream`.

<!--
The `quote!` macro also provides some very cool templating mechanics: we can
enter `#name`, and `quote!` will replace it with the value in the variable
`name`. You can even do some repetition similar to the way regular macros work.
Check out [the `quote` crate’s docs][quote-docs] for a thorough introduction.
-->

La macro `quote!` fournit aussi quelques mécaniques de gabarit intéressantes :
nous pouvons entrer `#nom`, et `quote!` va le remplacer avec la valeur dans la
variable `nom`. Vous pouvez même procéder à quelques répétions de façon
similaire au fonctionnement des macros classiques. Regardez dans
[la documentation de `quote`][quote-docs] pour une présentation plus détaillée.

<!--
[quote-docs]: https://docs.rs/quote
-->

[quote-docs]: https://docs.rs/quote

<!--
We want our procedural macro to generate an implementation of our `HelloMacro`
trait for the type the user annotated, which we can get by using `#name`. The
trait implementation has one function, `hello_macro`, whose body contains the
functionality we want to provide: printing `Hello, Macro! My name is` and then
the name of the annotated type.
-->

Nous souhaitons que notre macro procédurale génère une implémentation de notre
trait `HelloMacro` pour le type que l'utilisateur a annoté, que nous pouvons
obtenir en utilisant `#nom`. L'implémentation du trait utilise une fonction,
`hello_macro`, dont le corps contient la fonctionnalité que nous souhaitons
fournir : l'affichage de `Hello, Macro ! Mon nom est` suivi par le nom du type
annoté.

<!--
The `stringify!` macro used here is built into Rust. It takes a Rust
expression, such as `1 + 2`, and at compile time turns the expression into a
string literal, such as `"1 + 2"`. This is different than `format!` or
`println!`, macros which evaluate the expression and then turn the result into
a `String`. There is a possibility that the `#name` input might be an
expression to print literally, so we use `stringify!`. Using `stringify!` also
saves an allocation by converting `#name` to a string literal at compile time.
-->

La macro `stringify!` utilisée ici est écrite en Rust. Elle prend en argument
une expression Rust, comme `1 + 2`, et à la compilation transforme l'expression
en une chaîne de caractères littérale, comme `"1 + 2"`. Cela est différent de
`format!` ou de `println!`, des macros qui évaluent l'expression et retourne
ensuite le résultat dans une `String`. Il est possible que l'entrée `#nom` soit
une expression à écrire littéralement, donc nous utilisons `stringify!`.
L'utilisation de `stringify!` évite aussi une allocation en convertissant `#nom`
en une chaine de caractères littérale à la compilation.

<!--
At this point, `cargo build` should complete successfully in both `hello_macro`
and `hello_macro_derive`. Let’s hook up these crates to the code in Listing
19-30 to see the procedural macro in action! Create a new binary project in
your *projects* directory using `cargo new pancakes`. We need to add
`hello_macro` and `hello_macro_derive` as dependencies in the `pancakes`
crate’s *Cargo.toml*. If you’re publishing your versions of `hello_macro` and
`hello_macro_derive` to [crates.io](https://crates.io/), they would be regular
dependencies; if not, you can specify them as `path` dependencies as follows:
-->

Maintenant, `cargo build` devrait fonctionner correctement pour `hello_macro` et
`hello_macro_derive`. Relions maintenant ces crates au code de l'encart 19-30
pour voir les macros procédurales à l'oeuvre ! Créez un nouveau projet binaire
dans votre dossier *projects* en utilisant `cargo new pancakes`. Nous avons
besoin d'ajouter `hello_macro` et `hello_macro_derive` comme dépendances dans le
*Cargo.toml* de la crate `pancakes`. Si vous publiez vos versions de
`hello_macro` et de `hello_macro_derive` sur [crates.io](https://crates.io),
elles sont des dépendances classiques ; sinon, vous pouvez les renseigner comme
dépendance locale avec `path` comme ceci :

<!--
```toml
{{#include ../listings-sources/ch19-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```
-->

```toml
{{#include ../listings/ch19-advanced-features/no-listing-21-pancakes/pancakes/Cargo.toml:7:9}}
```

<!--
Put the code in Listing 19-30 into *src/main.rs*, and run `cargo run`: it
should print `Hello, Macro! My name is Pancakes!` The implementation of the
`HelloMacro` trait from the procedural macro was included without the
`pancakes` crate needing to implement it; the `#[derive(HelloMacro)]` added the
trait implementation.
-->

Renseignez le code l'encart 19-30 dans *src/main.rs*, puis lancez `cargo run` :
cela devrait afficher `Hello, Macro ! Mon nom est Pancakes !`. L'implémentation
du trait `HelloMacro` à l'aide de la macro procédurale est injectée sans que la
crate `pancakes` n'ai besoin de le faire manuellement ; le
`#[derive(HelloMacro)]` a rajouté automatiquement l'implémentation du trait.

<!--
Next, let’s explore how the other kinds of procedural macros differ from custom
derive macros.
-->

Maintenant, découvrons comment les autres types de macros procédurales se
distinguent des macros derive personnalisées.

<!--
### Attribute-like macros
-->

### Les macros qui ressemblent à des attributs

<!--
Attribute-like macros are similar to custom derive macros, but instead of
generating code for the `derive` attribute, they allow you to create new
attributes. They’re also more flexible: `derive` only works for structs and
enums; attributes can be applied to other items as well, such as functions.
Here’s an example of using an attribute-like macro: say you have an attribute
named `route` that annotates functions when using a web application framework:
-->

Les macros qui ressemblent à des attributs ressemblent aux macros derive
personnalisées, mais au lieu de générer du code pour l'attribut `derive`, elles
vous permettent de créer des nouveaux attributs. Elles sont aussi plus
flexibles : `derive` fonctionne uniquement pour les structures et les
énumérations ; les attributs peuvent être aussi appliqués aux autres éléments,
comme les fonctions. Voici un exemple d'utilisation d'une macro qui ressemble à
un attribut : imaginons que vous avez un attribut `chemin` qui est une
annotation pour des fonctions lorsque vous utilisez un environnement de
développement d'application web :

<!--
```rust,ignore
#[route(GET, "/")]
fn index() {
```
-->

```rust,ignore
#[chemin(GET, "/")]
fn index() {
```

<!--
This `#[route]` attribute would be defined by the framework as a procedural
macro. The signature of the macro definition function would look like this:
-->

Cet attribut `#[chemin]` sera défini par l'environnement de développement comme
étant une macro procédurale. La signature de la fonction de définition de la
macro ressemblera à ceci :

<!--
```rust,ignore
#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
```
-->

```rust,ignore
#[proc_macro_attribute]
pub fn chemin(attribut: TokenStream, element: TokenStream) -> TokenStream {
```

<!--
Here, we have two parameters of type `TokenStream`. The first is for the
contents of the attribute: the `GET, "/"` part. The second is the body of the
item the attribute is attached to: in this case, `fn index() {}` and the rest
of the function’s body.
-->

Maintenant, nous avons deux paramètres de type `TokenStream`. Le premier
correspond au contenu de l'attribut : la partie `GET, "/"`. Le second est le
corps de l'élément sur lequel cet attribut sera appliqué : dans notre cas,
`fn index() {}` et le reste du corps de la fonction.

<!--
Other than that, attribute-like macros work the same way as custom derive
macros: you create a crate with the `proc-macro` crate type and implement a
function that generates the code you want!
-->

Mis à part cela, les macros qui ressemblent à des attributs fonctionnent de la
même manière que les macros derive personnalisées : vous générez une crate avec
le type de la crate `proc-macro` et implémentez une fonction qui génèrera le
code que vous souhaitez !

<!--
### Function-like macros
-->

### Les macros qui ressemblent à des fonctions

<!--
Function-like macros define macros that look like function calls. Similarly to
`macro_rules!` macros, they’re more flexible than functions; for example, they
can take an unknown number of arguments. However, `macro_rules!` macros can be
defined only using the match-like syntax we discussed in the section
[“Declarative Macros with `macro_rules!` for General Metaprogramming”][decl]
earlier. Function-like macros take a `TokenStream` parameter and their
definition manipulates that `TokenStream` using Rust code as the other two
types of procedural macros do. An example of a function-like macro is an `sql!`
macro that might be called like so:
-->

Les macros qui ressemblent à des fonctions définissent des macros qui
ressemblent à des appels de fonction. De la même manière que les macros
`macro_rules!`, elles sont plus flexibles que les fonctions ; par exemple, elles
peuvent prendre une quantité non finie d'arguments. Cependant, les macros
`macro_rules!` peuvent être définies uniquement en utilisant la syntaxe qui
ressemble à `match` que nous avons vu dans [une section précédente][decl]. Les
macros qui ressemblent à des fonctions prennent en paramètre un `TokenStream` et
leurs définitions manipulent ce `TokenStream` en utilisant du code Rust comme le
font les deux autres types de macros procédurales. Voici un exemple d'une macro
qui ressemble à une fonction qui est une macro `sql!` qui devrait être utilisée
comme ceci :

<!--
[decl]: #declarative-macros-with-macro_rules-for-general-metaprogramming
-->

[decl]: #les-macros-déclaratives-avec-macro_rules-pour-la-métaprogrammation-générale

<!--
```rust,ignore
let sql = sql!(SELECT * FROM posts WHERE id=1);
```
-->

```rust,ignore
let sql = sql!(SELECT * FROM publications WHERE id=1);
```

<!--
This macro would parse the SQL statement inside it and check that it’s
syntactically correct, which is much more complex processing than a
`macro_rules!` macro can do. The `sql!` macro would be defined like this:
-->

Cette macro devrait interpréter l'instruction SQL qu'on lui envoie et vérifier
si elle est syntaxiquement correcte, ce qui est un procédé bien plus complexe
que ce qu'une macro `macro_rules!` peut faire. La macro `sql!` sera définie
comme ceci :

<!--
```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```
-->

```rust,ignore
#[proc_macro]
pub fn sql(input: TokenStream) -> TokenStream {
```

<!--
This definition is similar to the custom derive macro’s signature: we receive
the tokens that are inside the parentheses and return the code we wanted to
generate.
-->

Cette définition ressemble à la signature de la macro derive personnalisée :
nous récupérons les éléments entre parenthèses et retournons le code que nous
souhaitons générer.

<!--
## Summary
-->

## Résumé

<!--
Whew! Now you have some Rust features in your toolbox that you won’t use often,
but you’ll know they’re available in very particular circumstances. We’ve
introduced several complex topics so that when you encounter them in error
message suggestions or in other peoples’ code, you’ll be able to recognize
these concepts and syntax. Use this chapter as a reference to guide you to
solutions.
-->

Ouah ! Maintenant vous avez quelques fonctionnalités de Rust supplémentaires
dans votre boite à outils que vous utiliserez probablement rarement, mais vous
savez maintenant qu'elles vous aideront dans des situations très particulières.
Nous avons abordé plusieurs sujets complexes afin que vous puissiez les
reconnaître ainsi que leur syntaxe lorsque vous les rencontrerez dans des
messages de suggestions dans des erreurs ou dans du code de quelqu'un d'autre.
Utilisez ce chapitre comme référence pour vous guider vers ces solutions.

<!--
Next, we’ll put everything we’ve discussed throughout the book into practice
and do one more project!
-->

Au chapitre suivant, nous allons mettre en pratique tout ce que nous avons
appris dans ce livre en l'appliquant à un nouveau projet !
