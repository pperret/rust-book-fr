> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/205).

<!--
## All the Places Patterns Can Be Used
-->

## Tous les endroits où les motifs peuvent être utilisés

<!--
Patterns pop up in a number of places in Rust, and you’ve been using them a lot
without realizing it! This section discusses all the places where patterns are
valid.
-->

Les motifs apparaissent dans de nombreux endroits en Rust, et vous en avez
utilisé beaucoup sans vous en rendre compte ! Cette section va présenter les
différentes situations où l'utilisation des motifs est appropriée.

<!--
### `match` Arms
-->

### Les branches des `match`

<!--
As discussed in Chapter 6, we use patterns in the arms of `match` expressions.
Formally, `match` expressions are defined as the keyword `match`, a value to
match on, and one or more match arms that consist of a pattern and an
expression to run if the value matches that arm’s pattern, like this:
-->

Comme nous l'avons vu au chapitre 6, nous utilisons les motifs dans les
branches des expressions `match`. Techniquement, les expressions `match` sont
définies avec le mot-clé `match`, une valeur sur laquelle procéder, et une ou
plusieurs branches qui constituent un motif, ainsi qu'une expression à exécuter
si la valeur correspond au motif de cette branche, comme ceci :

<!--
```text
match VALUE {
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
    PATTERN => EXPRESSION,
}
```
-->

```text
match VALEUR {
    MOTIF => EXPRESSION,
    MOTIF => EXPRESSION,
    MOTIF => EXPRESSION,
}
```

<!--
One requirement for `match` expressions is that they need to be *exhaustive* in
the sense that all possibilities for the value in the `match` expression must
be accounted for. One way to ensure you’ve covered every possibility is to have
a catchall pattern for the last arm: for example, a variable name matching any
value can never fail and thus covers every remaining case.
-->

L'une des conditions à respecter pour les expressions `match` est qu'elle doit
être *exhaustive* dans le sens où toutes les possibilités pour la valeur dans
l'expression `match` doivent être prises en compte. Une façon de s'assurer que
vous avez couvert toutes les possibilités est d'avoir un motif passe-partout
pour la dernière branche : par exemple, une valeur quelconque ne pourra jamais
échouer car la dernière branche permet de couvrir tous les autres cas
possibles.

<!--
A particular pattern `_` will match anything, but it never binds to a variable,
so it’s often used in the last match arm. The `_` pattern can be useful when
you want to ignore any value not specified, for example. We’ll cover the `_`
pattern in more detail in the [“Ignoring Values in a
Pattern”][ignoring-values-in-a-pattern]<!-- ignore -- > section later in this
chapter.
-->

Le motif spécifique `_` va correspondre à tout, mais il ne fournira jamais de
variable, donc il est souvent utilisé dans la dernière branche. Le motif `_`
peut être utile lorsque vous souhaitez ignorer toutes les autres valeurs qui
n'ont pas été listées, par exemple. Nous allons voir plus en détail le motif
`_` dans une section
[plus tard dans ce chapitre][ignoring-values-in-a-pattern]<!-- ignore -->.

<!--
### Conditional `if let` Expressions
-->

### Les expressions conditionnelles `if let`

<!--
In Chapter 6 we discussed how to use `if let` expressions mainly as a shorter
way to write the equivalent of a `match` that only matches one case.
Optionally, `if let` can have a corresponding `else` containing code to run if
the pattern in the `if let` doesn’t match.
-->

Au chapitre 6, nous avons vu comment utiliser les expressions `if let`,
principalement pour pouvoir écrire l'équivalent d'un `match` qui ne correspond
qu'à un seul cas.
Accessoirement, `if let` peut avoir un `else` correspondant au code à exécuter
si le motif du `if let` ne correspond pas au premier critère.

<!--
Listing 18-1 shows that it’s also possible to mix and match `if let`, `else
if`, and `else if let` expressions. Doing so gives us more flexibility than a
`match` expression in which we can express only one value to compare with the
patterns. Also, the conditions in a series of `if let`, `else if`, `else if
let` arms aren’t required to relate to each other.
-->

L'encart 18-1 montre qu'il est aussi possible de conjuguer les expressions
`if let`, `else if`, et `else if let`. Faire ceci nous donne plus de
flexibilité qu'une expression `match` dans laquelle nous ne pouvons que
vérifier une seule valeur à comparer avec les motifs. De plus, dans une série
de branches `if let`, `else if`, et `else if let`, les conditions n'ont pas
besoin d'avoir les mêmes conditions et contextes.

<!--
The code in Listing 18-1 shows a series of checks for several conditions that
decide what the background color should be. For this example, we’ve created
variables with hardcoded values that a real program might receive from user
input.
-->

Le code de l'encart 18-1 montre une série de vérifications pour quelques
conditions qui décident quelle devrait être la couleur de fond. Pour cet
exemple, nous avons créé les variables avec des valeurs codées en dur qu'un
vrai programme devrait recevoir d'une saisie d'un utilisateur.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-01/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-01/src/main.rs}}
```

<!--
<span class="caption">Listing 18-1: Mixing `if let`, `else if`, `else if let`,
and `else`</span>
-->

<span class="caption">Encart 18-1 : mélange de `if let`, `else if`,
`else if let`, et `else`</span>

<!--
If the user specifies a favorite color, that color is the background color. If
today is Tuesday, the background color is green. If the user specifies
their age as a string and we can parse it as a number successfully, the color
is either purple or orange depending on the value of the number. If none of
these conditions apply, the background color is blue.
-->

Si l'utilisateur renseigne une couleur favorite, c'est cette couleur qui
devient la couleur de fond. Sinon, si nous sommes mardi, la couleur de fond
sera vert. Sinon, si l'utilisateur a renseigné son age dans une chaîne de
caractères et que nous pouvons l'interpréter comme un nombre avec succès, la
couleur de fond sera soit violet, soit orange en fonction de la valeur de ce
nombre. Si aucune de ces conditions s'applique, la couleur de fond sera bleu.

<!--
This conditional structure lets us support complex requirements. With the
hardcoded values we have here, this example will print `Using purple as the
background color`.
-->

Cette structure conditionnelle nous permet de répondre à des conditions
complexes. Avec les valeurs codées en dur que nous avons ici, cet exemple
devrait afficher `Utilisation du violet comme couleur de fond`.

<!--
You can see that `if let` can also introduce shadowed variables in the same way
that `match` arms can: the line `if let Ok(age) = age` introduces a new
shadowed `age` variable that contains the value inside the `Ok` variant. This
means we need to place the `if age > 30` condition within that block: we can’t
combine these two conditions into `if let Ok(age) = age && age > 30`. The
shadowed `age` we want to compare to 30 isn’t valid until the new scope starts
with the curly bracket.
-->

Vous pouvez constater que le `if let` nous permet d'utiliser les variables
masquées de la même manière que le fait le font les branches `match` : la ligne
`if let Ok(age) = age` crée une nouvelle variable masquée `age` qui contient la
valeur présente dans la variante `Ok`. Cela signifie que nous devons placer la
condition `if age > 30` à l'intérieur de ce bloc : nous ne pouvons pas combiner
ces deux conditions dans une seule `if let Ok(age) = age && age > 30`. La
variable masquée `age` que nous souhaitons comparer à 30 n'est pas encore en
vigueur tant que la nouvelle portée entre les accolades n'a pas commencée.

<!--
The downside of using `if let` expressions is that the compiler doesn’t check
exhaustiveness, whereas with `match` expressions it does. If we omitted the
last `else` block and therefore missed handling some cases, the compiler would
not alert us to the possible logic bug.
-->

Le désavantage de l'utilisation des expressions `if let` est que le compilateur
ne vérifie pas l'exhaustivité, ce que fait l'expression `match`. Si nous avions
enlevé le dernier bloc `else` et ainsi que nous oublions de gérer certains cas,
le compilateur ne pourrait pas nous prévenir d'un possible bogue de logique.

<!--
### `while let` Conditional Loops
-->

### les boucles conditionelles `while let`

<!--
Similar in construction to `if let`, the `while let` conditional loop allows a
`while` loop to run for as long as a pattern continues to match. The example in
Listing 18-2 shows a `while let` loop that uses a vector as a stack and prints
the values in the vector in the opposite order in which they were pushed.
-->

Comme les constructions `if let`, les boucles conditionnelles `while let`
permettent à une boucle `while` de s'exécuter aussi longtemps qu'un motif
continue à correspondre. L'exemple dans l'encart 18-2 montre une boucle
`while let` qui utilise un vecteur comme une pile et affiche les valeurs du
vecteur dans l'ordre opposé à celui dans lequel elles ont été insérées.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-02/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-02/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-2: Using a `while let` loop to print values
for as long as `stack.pop()` returns `Some`</span>
-->

<span class="caption">Encart 18-2 : utilisation d'une boucle `while let` pour
afficher les valeurs aussi longtemps que `pile.pop()` retourne une `Some`</span>

<!--
This example prints 3, 2, and then 1. The `pop` method takes the last element
out of the vector and returns `Some(value)`. If the vector is empty, `pop`
returns `None`. The `while` loop continues running the code in its block as
long as `pop` returns `Some`. When `pop` returns `None`, the loop stops. We can
use `while let` to pop every element off our stack.
-->

Cet exemple affiche 3, 2 puis ensuite 1. La méthode `pop` sort le dernier
élément du vecteur et retourne `Some(valeur)`. Si le vecteur est vide, `pop`
retourne alors `None`. La boucle `while` continue à exécuter le code de son bloc
aussi longtemps que `pop` retourne un `Some`. Lorsque `pop` retournera `None`,
la boucle s'arrêtera. Nous pouvons utiliser `while let` pour extraire tous les
éléments de la pile.

<!--
### `for` Loops
-->

### Les boucles `for`

<!--
In Chapter 3, we mentioned that the `for` loop is the most common loop
construction in Rust code, but we haven’t yet discussed the pattern that `for`
takes. In a `for` loop, the pattern is the value that directly follows the
keyword `for`, so in `for x in y` the `x` is the pattern.
-->

Au chapitre 3, nous avions mentionné que la boucle `for` était la construction
de boucle la plus utilisé dans du code Rust, mais nous n'avons pas encore abordé
le motif que prend `for`. Dans une boucle `for`, le motif est la valeur qui suit
directement le mot-clé `for`, de sorte que `x` est le motif dans `for x in y`.

<!--
Listing 18-3 demonstrates how to use a pattern in a `for` loop to destructure,
or break apart, a tuple as part of the `for` loop.
-->

L'encart 18-3 montre comment utiliser un motif dans une boucle `for` pour
déstructurer, ou décomposer, un tuple comme étant une partie de la boucle `for`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-03/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-3: Using a pattern in a `for` loop to
destructure a tuple</span>
-->

<span class="caption">Encart 18-3 : utilisation d'un motif dans une boucle `for`
pour déstructurer un tuple</span>

<!--
The code in Listing 18-3 will print the following:
-->

Le code de l'encart 18-3 va afficher ceci :

<!--
```console
{{#include ../listings-sources/ch18-patterns-and-matching/listing-18-03/output.txt}}
```
-->

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-03/output.txt}}
```

<!--
We use the `enumerate` method to adapt an iterator to produce a value and that
value’s index in the iterator, placed into a tuple. The first value produced is
the tuple `(0, 'a')`. When this value is matched to the pattern `(index,
value)`, `index` will be `0` and `value` will be `'a'`, printing the first line
of the output.
-->

Nous avons utilisé la méthode `enumerate` pour produire une valeur et son indice
à partir d'un itérateur, que nous avons placé dans un tuple. La premiere valeur
produite est le tuple `(0, 'a')`. Lorsque cette valeur est adaptée au motif
`(indice, valeur)`, `indice` vaudra `0` et `valeur` vaudra `'a'`, et affichera
la première ligne sur la sortie.

<!--
### `let` Statements
-->

### Les instructions `let`

<!--
Prior to this chapter, we had only explicitly discussed using patterns with
`match` and `if let`, but in fact, we’ve used patterns in other places as well,
including in `let` statements. For example, consider this straightforward
variable assignment with `let`:
-->

Avant d'arriver à ce chapitre, nous n'avions abordé explicitement l'utilisation
des motifs uniquement avec `match` et `if let`, mais en réalité, nous avons
utilisé les motifs dans d'autres endroits, y compris les instructions `let`. Par
exemple, considérons l'assignation à la variable suivante avec `let` :

<!--
```rust
let x = 5;
```
-->

```rust
let x = 5;
```

<!--
Throughout this book, we’ve used `let` like this hundreds of times, and
although you might not have realized it, you were using patterns! More
formally, a `let` statement looks like this:
-->

Tout au long de ce livre, nous avons utilisé `let` de cette manière des
centaines de fois, et malgré tout vous ne vous êtes probablement pas rendu
compte que vous utilisiez les motifs ! Plus formellement, une instruction `let`
ressemble à ceci :

<!--
```text
let PATTERN = EXPRESSION;
```
-->

```text
let MOTIF = EXPRESSION;
```

<!--
In statements like `let x = 5;` with a variable name in the `PATTERN` slot, the
variable name is just a particularly simple form of a pattern. Rust compares
the expression against the pattern and assigns any names it finds. So in the
`let x = 5;` example, `x` is a pattern that means “bind what matches here to
the variable `x`.” Because the name `x` is the whole pattern, this pattern
effectively means “bind everything to the variable `x`, whatever the value is.”
-->

Dans des instructions comme `let x = 5;` avec un nom de variable dans
l'emplacement `MOTIF`, le nom de la variable est juste une forme simple de
motif. Rust compare l'expression avec le motif et assigne tous les noms qu'il
rencontre. Dans l'exemple `let x = 5;`, `x` est un motif qui signifie
“relie ce qui suit à la variable `x`”.  Comme le nom `x` est un motif en entier,
ce motif demande exactement dans ce cas “relie tout ce qui suit à la variable
`x`, peu importe quelle valeur c'est”.

<!--
To see the pattern matching aspect of `let` more clearly, consider Listing
18-4, which uses a pattern with `let` to destructure a tuple.
-->

Pour comprendre plus clairement l'aspect filtrage par motif de `let`, admettons
l'encart 18-4, qui utilise un motif avec `let` pour destructurer un tuple.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-04/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-4: Using a pattern to destructure a tuple and
create three variables at once</span>
-->

<span class="caption">Encart 18-4 : utilisation d'un motif pour destructurer un
tuple et créer trois variables à la fois</span>

<!--
Here, we match a tuple against a pattern. Rust compares the value `(1, 2, 3)`
to the pattern `(x, y, z)` and sees that the value matches the pattern, so Rust
binds `1` to `x`, `2` to `y`, and `3` to `z`. You can think of this tuple
pattern as nesting three individual variable patterns inside it.
-->

Ici, nous avons fait correspondre un tuple à un motif. Rust compare la valeur
`(1, 2, 3)` au motif `(x, y, z)` et constate que la valeur correspond au motif,
donc Rust relie `1` à `x`, `2` à `y`, et `3` à `z`. Vous pouvez ainsi considérer
que ce motif de tuple contient trois variables individuelles en son sein.

<!--
If the number of elements in the pattern doesn’t match the number of elements
in the tuple, the overall type won’t match and we’ll get a compiler error. For
example, Listing 18-5 shows an attempt to destructure a tuple with three
elements into two variables, which won’t work.
-->

Si le nombre d'éléments dans le motif ne correspondent pas au nombre d'éléments
dans le tuple, le type au complet ne va pas correspondre et nous allons obtenir
une erreur de compilation. Par exemple, l'encart 18-5 montre une tentative de
déstructurer un tuple avec trois éléments dans deux variables, ce qui ne va pas
fonctionner.

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-05/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-5: Incorrectly constructing a pattern whose
variables don’t match the number of elements in the tuple</span>
-->

<span class="caption">Encart 18-5 : construction incorrecte d'un motif dont les
variables ne vont pas correspondre au nombre d'éléments présents dans le tuple
</span>

<!--
Attempting to compile this code results in this type error:
-->

Si vous essayez de compiler ce code, vous obtiendrez cette erreur de type :

<!--
```console
{{#include ../listings-sources/ch18-patterns-and-matching/listing-18-05/output.txt}}
```
-->

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-05/output.txt}}
```

<!--
If we wanted to ignore one or more of the values in the tuple, we could use `_`
or `..`, as you’ll see in the [“Ignoring Values in a
Pattern”][ignoring-values-in-a-pattern]<!-- ignore -- > section. If the problem
is that we have too many variables in the pattern, the solution is to make the
types match by removing variables so the number of variables equals the number
of elements in the tuple.
-->

Si nous souhaitons ignorer une ou plusieurs valeurs dans un tuple, nous pouvons
utiliser `_` ou `..`, comme vous allez le voir à la dernière section de ce
chapitre. Si le problème est que nous avons trop de variables dans le motif, la
solution faire correspondre les types en enlevant les variables afin que le
nombre de variables corresponde au nombre d'éléments présents dans le tuple.

<!--
### Function Parameters
-->

### Les paramètres de fonctions

<!--
Function parameters can also be patterns. The code in Listing 18-6, which
declares a function named `foo` that takes one parameter named `x` of type
`i32`, should by now look familiar.
-->

Les paramètres de fonctions peuvent aussi être des motifs. Le code de l'encart
18-6 déclare une fonction `foo` qui prend un paramètre `x` de type `i32`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-06/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-06/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-6: A function signature uses patterns in the
parameters</span>
-->

<span class="caption">Encart 18-6 : une signature de fonction qui utilise des
motifs dans ses paramètres</span>

<!--
The `x` part is a pattern! As we did with `let`, we could match a tuple in a
function’s arguments to the pattern. Listing 18-7 splits the values in a tuple
as we pass it to a function.
-->

La partie `x` est un motif ! Comme nous l'avons dit pour `let`, nous pouvons
faire correspondre le motif avec un tuple dans les arguments de la fonction.
L'encart 18-7 déstructure les valeurs d'un tuple que nous passons en argument
d'une fonction.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-07/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-07/src/main.rs}}
```

<!--
<span class="caption">Listing 18-7: A function with parameters that destructure
a tuple</span>
-->

<span class="caption">Encart 18-7 : une fonction avec des paramètres qui
déstructurent un tuple</span>

<!--
This code prints `Current location: (3, 5)`. The values `&(3, 5)` match the
pattern `&(x, y)`, so `x` is the value `3` and `y` is the value `5`.
-->

Ce code affiche `Coordonées actuelles : (3, 5)`. Les valeurs `&(3, 5)`
correspondent au motif `&(x, y)`, donc `x` a la valeur `3` et `y` a la valeur
`5`.

<!--
We can also use patterns in closure parameter lists in the same way as in
function parameter lists, because closures are similar to functions, as
discussed in Chapter 13.
-->

Nous pouvons aussi utiliser les motifs dans la liste des paramètres d'une
fermeture de la même manière que la liste des paramètres d'une fonction, car les
fermetures sont similaires aux fonctions, comme nous l'avons dit au chapitre 13.

<!--
At this point, you’ve seen several ways of using patterns, but patterns don’t
work the same in every place we can use them. In some places, the patterns must
be irrefutable; in other circumstances, they can be refutable. We’ll discuss
these two concepts next.
-->

A présent, vous avez vu plusieurs façons d'utiliser les motifs, mais les motifs
ne fonctionnent pas de la même manière dans toutes les situations où nous les
utilisons. Des fois, le motif sera irréfutable ; d'autres fois, il sera
réfutable. C'est ce que nous allons voir tout de suite.

<!--
[ignoring-values-in-a-pattern]:
ch18-03-pattern-syntax.html#ignoring-values-in-a-pattern
-->

[ignoring-values-in-a-pattern]: ch18-03-pattern-syntax.html
