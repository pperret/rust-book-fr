> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/207).

<!--
## Pattern Syntax
-->

## La syntaxe des motifs

<!--
Throughout the book, you’ve seen examples of many kinds of patterns. In this
section, we gather all the syntax valid in patterns and discuss why you might
want to use each one.
-->

A travers le livre, vous avez rencontré de nombreux types de motifs. Dans cette
section, nous allons rassembler toutes les syntaxes valides des motifs et
examiner les raisons pour lesquelles vous devriez utiliser chacune d'entre
elles.

<!--
### Matching Literals
-->

### Correspondre aux littéraux

<!--
As you saw in Chapter 6, you can match patterns against literals directly. The
following code gives some examples:
-->

Comme vous l'avez vu chapitre 6, vous pouvez faire directement correspondre des
motifs sur des littéraux. Le code suivant vous donne quelques exemples :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-01-literals/src/main.rs:here}}
```

<!--
This code prints `one` because the value in `x` is 1. This syntax is useful
when you want your code to take an action if it gets a particular concrete
value.
-->

Ce code affiche `un` car la valeur dans `x` est `1`. Cette syntaxe est très
utile lorsque vous souhaitez que votre code fasse quelque chose s'il obtient une
valeur précise.

<!--
### Matching Named Variables
-->

### Correspondre à des variables nommées

<!--
Named variables are irrefutable patterns that match any value, and we’ve used
them many times in the book. However, there is a complication when you use
named variables in `match` expressions. Because `match` starts a new scope,
variables declared as part of a pattern inside the `match` expression will
shadow those with the same name outside the `match` construct, as is the case
with all variables. In Listing 18-11, we declare a variable named `x` with the
value `Some(5)` and a variable `y` with the value `10`. We then create a
`match` expression on the value `x`. Look at the patterns in the match arms and
`println!` at the end, and try to figure out what the code will print before
running this code or reading further.
-->

Les variables nommées sont des motifs irréfutables qui correspondent à
n'importe quelle valeur, et nous les avons utilisés de nombreuses foix dans le
livre. Cependant, il subsiste un problème lorsque vous utilisez les variables
nommées dans les expressions `match`. Comme `match` débute une nouvelle portée,
les variables utilisées comme étant une partie du motif de la construction
`match` vont masquer celles avec le même nom provenant de l'extérieur de la
construction `match`, comme c'est le cas avec toutes les variables. Dans
l'encart 18-11, nous déclarons une variable `x` avec la valeur `Some(5)` et une
variable `y` avec la valeur `10`. Nous créons alors une expression `match` sur
la valeur `x`. Observez les motifs sur les branches du `match` et du `println!`
à la fin, et essayez de deviner ce qui sera écrit avant d'exécuter ce code ou
de lire la suite.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-11/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-11: A `match` expression with an arm that
introduces a shadowed variable `y`</span>
-->

<span class="caption">Encart 18-11 : une expression `match` avec une branche
qui crée une variable masquée `y`</span>

<!--
Let’s walk through what happens when the `match` expression runs. The pattern
in the first match arm doesn’t match the defined value of `x`, so the code
continues.
-->

Voyons ce qui se passe lorsque l'expression `match` est utilisée. Le motif
présent dans la première branche du `match` ne correspond pas à la valeur
actuelle de `x`, donc le code continue.

<!--
The pattern in the second match arm introduces a new variable named `y` that
will match any value inside a `Some` value. Because we’re in a new scope inside
the `match` expression, this is a new `y` variable, not the `y` we declared at
the beginning with the value 10. This new `y` binding will match any value
inside a `Some`, which is what we have in `x`. Therefore, this new `y` binds to
the inner value of the `Some` in `x`. That value is `5`, so the expression for
that arm executes and prints `Matched, y = 5`.
-->

Le motif dans la seconde branche du `match` ajoute une nouvelle variable `y`
qui va correspondre à n'importe quelle valeur logée dans une valeur `Some`.
Comme nous sommes dans une nouvelle portée à l'intérieur de l'expression
`match`, c'est une nouvelle variable `y`, et non pas le `y` que nous avons
déclaré au début avec la valeur 10. Cette nouvelle correspondance `y` va
correspondre à n'importe quelle valeur à l'intérieur d'un `Some`, ce qui est
la situation présente actuellement dans `x`. Ainsi, ce nouveau `y` correspondra
à la valeur interne du `Some` présent dans `x`. Cette valeur est `5`, donc
l'expression de cette branche s'exécute et affiche `Correspondance, y = 5`.

<!--
If `x` had been a `None` value instead of `Some(5)`, the patterns in the first
two arms wouldn’t have matched, so the value would have matched to the
underscore. We didn’t introduce the `x` variable in the pattern of the
underscore arm, so the `x` in the expression is still the outer `x` that hasn’t
been shadowed. In this hypothetical case, the `match` would print `Default
case, x = None`.
-->

Si `x` aurait été une valeur `None` plutôt que `Some(5)`, les motifs présents
dans les deux premières branches n'auraient pas correspondu, donc la valeur qui
aurait correspondu serait celui avec le tiret du bas. Nous n'avons pas
introduit de nouvelle variable `x` qui est présente dans la branche du motif,
donc le `x` dans l'expression désigne toujours la variable `x` en dehors, qui
n'a pas été masquée. Dans ce cas, le `match` devrait afficher
`Cas par défaut, x = None`.

<!--
When the `match` expression is done, its scope ends, and so does the scope of
the inner `y`. The last `println!` produces `at the end: x = Some(5), y = 10`.
-->

Lorsque l'expression `match` est terminée, sa portée se termine, et avec elle
la portée de la variable interne `y`. Le dernier `println!` affiche
`A la fin : x = Some(5), y = 10`.

<!--
To create a `match` expression that compares the values of the outer `x` and
`y`, rather than introducing a shadowed variable, we would need to use a match
guard conditional instead. We’ll talk about match guards later in the [“Extra
Conditionals with Match Guards”](#extra-conditionals-with-match-guards)<!--
ignore -- > section.
-->

Pour créer une expression `match` qui compare les valeurs d'une variable
externe `x` et `y`, plutôt que d'utiliser une variable masquée, nous aurons
besoin d'utiliser à la place un contrôle de correspondance. Nous verrons les
contrôles de correspondance plus loin dans une des sections.

<!--
### Multiple Patterns
-->

### Plusieurs motifs

<!--
In `match` expressions, you can match multiple patterns using the `|` syntax,
which means *or*. For example, the following code matches the value of `x`
against the match arms, the first of which has an *or* option, meaning if the
value of `x` matches either of the values in that arm, that arm’s code will
run:
-->

Dans les expressions `match`, vous pouvez faire correspondre plusieurs motifs
en utilisant la syntaxe `|`, qui signifie *ou*. Par exemple, le code suivant
applique un `match` sur la valeur de `x`, la première des branches a une option
*ou*, ce qui signifie que si la valeur de `x` correspond à un de ces motifs de
cette branche, le code de cette branche sera exécuté :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-02-multiple-patterns/src/main.rs:here}}
```

<!--
This code prints `one or two`.
-->

Ce code va afficher `un ou deux`.

<!--
### Matching Ranges of Values with `..=`
-->

### Faire correspondre un intervalle de valeurs avec `..=`

<!--
The `..=` syntax allows us to match to an inclusive range of values. In the
following code, when a pattern matches any of the values within the range, that
arm will execute:
-->

La syntaxe `..=` nous permet de faire correspondre un intervalle inclusif de
valeurs. Dans le code suivant, lorsqu'un motif correspond à une des valeurs
présentes dans l'intervalle, cette branche va s'exécuter :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-03-ranges/src/main.rs:here}}
```

<!--
If `x` is 1, 2, 3, 4, or 5, the first arm will match. This syntax is more
convenient than using the `|` operator to express the same idea; instead of
`1..=5`, we would have to specify `1 | 2 | 3 | 4 | 5` if we used `|`.
Specifying a range is much shorter, especially if we want to match, say, any
number between 1 and 1,000!
-->

Si `x` vaut 1, 2, 3, 4 ou 5, la première branche va correspondre. Cette syntaxe
est plus pratique à utiliser que d'avoir à utiliser l'opérateur `|` pour
exprimer la même idée ; à la place de `1..=5` nous aurions dû préciser
`1 | 2 | 3 | 4 | 5` si nous avions dû utiliser `|`. Renseigner un intervalle
est bien plus court, en particulier si nous souhaitons correspondre aux valeurs
entre 1 et 1000 par exemple !

<!--
Ranges are only allowed with numeric values or `char` values, because the
compiler checks that the range isn’t empty at compile time. The only types for
which Rust can tell if a range is empty or not are `char` and numeric values.
-->

Les intervalles peuvent être des nombres ou des `char` (caractères), car le
compilateur vérifie que l'intervalle n'est pas vide au moment de la
compilation. Les seuls types pour lesquels Rust peut dire si un intervalle est
vide ou non sont pour les nombres et les `char`.

<!--
Here is an example using ranges of `char` values:
-->

Voici un exemple d'utilisation d'intervalles de `char` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-04-ranges-of-char/src/main.rs:here}}
```

<!--
Rust can tell that `c` is within the first pattern’s range and prints `early
ASCII letter`.
-->

Rust peut nous dire que `c` est dans le premier intervalle du premier motif et
afficher `lettre ASCII du début`.

<!--
### Destructuring to Break Apart Values
-->

### Destructurer pour séparer les valeurs

<!--
We can also use patterns to destructure structs, enums, tuples, and references
to use different parts of these values. Let’s walk through each value.
-->

Nous pouvons aussi utiliser les motifs pour destructurer les structures, les
énumérations, les tuples et les références pour utiliser différentes parties de
ces valeurs. Passons en revue chacun des cas.

<!--
#### Destructuring Structs
-->

#### Destructurer les structures

<!--
Listing 18-12 shows a `Point` struct with two fields, `x` and `y`, that we can
break apart using a pattern with a `let` statement.
-->

L'encart 18-12 montre une structure `Point` avec deux champs, `x` et `y`, que
nous pouvons séparer en utilisant un motif avec une instruction `let`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-12/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-12/src/main.rs}}
```

```rust
struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let p = Point { x: 0, y: 7 };

    let Point { x: a, y: b } = p;
    assert_eq!(0, a);
    assert_eq!(7, b);
}
```

<!--
<span class="caption">Listing 18-12: Destructuring a struct’s fields into
separate variables</span>
-->

<span class="caption">Encart 18-12 : déstructuration des champs d'une structure
dans des variables séparées</span>

<!--
This code creates the variables `a` and `b` that match the values of the `x`
and `y` fields of the `p` struct. This example shows that the names of the
variables in the pattern don’t have to match the field names of the struct. But
it’s common to want the variable names to match the field names to make it
easier to remember which variables came from which fields.
-->

Ce code créé les variables `a` et `b` qui correspondent aux valeurs des champs
`x` et `y` de la structure `p`. Cet exemple montre que les noms des variables
du motif n'ont pas à correspondre aux noms des champs de la structure. Mais il
est courant de vouloir que les noms des variables correspondent aux noms des
champs pour se rappeler plus facilement quelle variable provient de quel champ.

<!--
Because having variable names match the fields is common and because writing
`let Point { x: x, y: y } = p;` contains a lot of duplication, there is a
shorthand for patterns that match struct fields: you only need to list the name
of the struct field, and the variables created from the pattern will have the
same names. Listing 18-13 shows code that behaves in the same way as the code
in Listing 18-12, but the variables created in the `let` pattern are `x` and
`y` instead of `a` and `b`.
-->

Comme avoir des noms de variables qui correspondent aux champs est courant et
comme écrire `let Point { x: x, y: y } = p;` est assez répétitif, il existe un
raccourci pour les motifs qui correspondent aux champs des structures : vous
avez simplement besoin de lister le nom des champs de la structure, et les
variables créées à partir du motif auront les mêmes noms. L'encart 18-12 montre
du code qui se comporte de la même manière que le code de l'encart 18-12, mais
les variables créées dans le motif du `let` sont `x` et `y` au lieu de `a` et
`b`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-13/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-13/src/main.rs}}
```

<!--
<span class="caption">Listing 18-13: Destructuring struct fields using struct
field shorthand</span>
-->

<span class="caption">Encart 18-13 : déstructuration des champs d'une structure
en utilisant le raccourci pour les champs des structures</span>

<!--
This code creates the variables `x` and `y` that match the `x` and `y` fields
of the `p` variable. The outcome is that the variables `x` and `y` contain the
values from the `p` struct.
-->

Ce code créé les variables `x` et `y` qui correspondent aux champs `x` et `y`
de la variable `p`. Il en résulte que les variables `x` et `y` contiennent les
valeurs correspondantes à la structure `p`.

<!--
We can also destructure with literal values as part of the struct pattern
rather than creating variables for all the fields. Doing so allows us to test
some of the fields for particular values while creating variables to
destructure the other fields.
-->

Nous pouvons aussi destructurer avec des valeurs littérales comme faisant partie
du motif de la structure plutôt que d'avoir à créer les variables pour tous les
champs. Ceci nous permet de tester certains champs pour des valeurs
particulières tout en créant les variables pour destructurer les autres champs.

<!--
Listing 18-14 shows a `match` expression that separates `Point` values into
three cases: points that lie directly on the `x` axis (which is true when `y =
0`), on the `y` axis (`x = 0`), or neither.
-->

L'encart 18-14 montre une expression `match` qui sépare les valeurs `Point`
en trois catégories : les points qui sont directement sur l'axe `x` (ce qui est
vrai lorsque `y = 0`), ceux directement sur l'axe `y` (`x = 0`), ou sur aucun
de ces deux axes.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-14/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-14/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-14: Destructuring and matching literal values
in one pattern</span>
-->

<span class="caption">Encart 18-14 : déstructurer et faire correspondre des
valeurs littérales grâce à un seul motif</span>

<!--
The first arm will match any point that lies on the `x` axis by specifying that
the `y` field matches if its value matches the literal `0`. The pattern still
creates an `x` variable that we can use in the code for this arm.
-->

La première branche va correspondre avec n'importe quel point qui se trouve sur
l'axe `x` en précisant que le champ `y` correspond au littéral `0`. Le motif va
toujours créer une variable `x` que nous pouvons utiliser dans le code de cette
branche.

<!--
Similarly, the second arm matches any point on the `y` axis by specifying that
the `x` field matches if its value is `0` and creates a variable `y` for the
value of the `y` field. The third arm doesn’t specify any literals, so it
matches any other `Point` and creates variables for both the `x` and `y` fields.
-->

De la même manière, la seconde branche correspondra avec tous les points sur
l'axe `y` en précisant que le champ `x` correspondra uniquement si sa valeur
est `0` et créera une variable `y` pour la valeur du champ `y`. La troisième
branche n'a pas besoin d'un littéral en particulier, donc elle correspondra à
n'importe quel autre `Point` et créera les variables pour les champs `x` et
`y`.

<!--
In this example, the value `p` matches the second arm by virtue of `x`
containing a 0, so this code will print `On the y axis at 7`.
-->

Dans cet exemple, la valeur `p` correspond avec la seconde branche car son `x`
vaut `0`, donc ce code va afficher `Sur l'axe y à la position 7`.

<!--
#### Destructuring Enums
-->

#### Destructurer une énumération

<!--
We’ve destructured enums earlier in this book, for example, when we
destructured `Option<i32>` in Listing 6-5 in Chapter 6. One detail we haven’t
mentioned explicitly is that the pattern to destructure an enum should
correspond to the way the data stored within the enum is defined. As an
example, in Listing 18-15 we use the `Message` enum from Listing 6-2 and write
a `match` with patterns that will destructure each inner value.
-->

Nous avons destructuré les énumérations précédemment dans ce livre, par exemple
lorsque nous avions destructuré `Option<i32>` dans l'encart 6-5 du chapitre 6.
Un détail que nous n'avons pas précisé explicitement était que le motif pour
destructurer une énumération devait correspondre à la façon dont est défini les
données dans l'énumération. Par exemple, dans l'encart 18-15 nous utilisons
l'énumération `Message` de l'encart 6-2 et nous rajoutons un `match` avec des
motifs qui devraient destructurer chaque valeur interne.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-15/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-15/src/main.rs}}
```

<!--
<span class="caption">Listing 18-15: Destructuring enum variants that hold
different kinds of values</span>
-->

<span class="caption">Encart 18-15 : déstructuration des variantes d'une
énumération qui stocke différents types de valeurs</span>

<!--
This code will print `Change the color to red 0, green 160, and blue 255`. Try
changing the value of `msg` to see the code from the other arms run.
-->

Ce code va afficher
`Changement des taux de rouge à 0, de vert à 160, et de bleu à 255`. Essayez de
changer la valeur de `message` pour voir le code qu'exécute les autres
branches.

<!--
For enum variants without any data, like `Message::Quit`, we can’t destructure
the value any further. We can only match on the literal `Message::Quit` value,
and no variables are in that pattern.
-->

Pour les variantes d'énumération sans aucune donnée, comme `Message::Quitter`,
nous ne pouvons pas destructurer de valeurs. Nous pouvons uniquement
correspondre à la valeur littérale `Message::Quitter`, et il n'y a pas de
variable dans ce motif.

<!--
For struct-like enum variants, such as `Message::Move`, we can use a pattern
similar to the pattern we specify to match structs. After the variant name, we
place curly brackets and then list the fields with variables so we break apart
the pieces to use in the code for this arm. Here we use the shorthand form as
we did in Listing 18-13.
-->

Pour les variantes d'énumération qui ressemblent aux structures, comme
`Message::Deplacer`, nous pouvons utiliser un motif similaire aux motifs que
nous utilisons pour correspondre aux structures. Après le nom de la variante,
nous utilisons des accolades et ensuite nous listons les champs avec des
variables afin de diviser les éléments à utiliser dans le code de cette
branche. Ici nous utilisons la forme raccourcie comme nous l'avons fait à
l'encart 18-13.

<!--
For tuple-like enum variants, like `Message::Write` that holds a tuple with one
element and `Message::ChangeColor` that holds a tuple with three elements, the
pattern is similar to the pattern we specify to match tuples. The number of
variables in the pattern must match the number of elements in the variant we’re
matching.
-->

Pour les variantes d'énumérations qui ressemblent à des tuples, comme
`Message::Ecrire` qui stocke un tuple avec un élément, et
`Message::ChangerCouleur` qui stocke un tuple avec trois éléments, le motif
ressemble au motif que nous renseignons pour correspondre aux tuples. Le nombre
de variables dans le motif doit correspondre aux nombre d'éléments dans la
variante qui correspond.

<!--
#### Destructuring Nested Structs and Enums
-->

#### Destructurer des structures imbriquées et des énumérations

<!--
Until now, all our examples have been matching structs or enums that were one
level deep. Matching can work on nested items too!
-->

Jusqu'à présent, tous nos exemples avaient des correspondances avec structures
ou des énumérations qui avaient un seul niveau de profondeur. Les
correspondances fonctionnent aussi sur les éléments imbriqués !

<!--
For example, we can refactor the code in Listing 18-15 to support RGB and HSV
colors in the `ChangeColor` message, as shown in Listing 18-16.
-->

Par exemple, nous pouvons remanier le code de l'encart 18-15 pour pouvoir
utiliser des couleurs RVB et TSV dans le message `ChangerCouleur`, comme dans
l'encart 18-16.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-16/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-16/src/main.rs}}
```

<!--
<span class="caption">Listing 18-16: Matching on nested enums</span>
-->

<span class="caption">Encart 18-16 : correspondance avec des énumérations
imbriquées</span>

<!--
The pattern of the first arm in the `match` expression matches a
`Message::ChangeColor` enum variant that contains a `Color::Rgb` variant; then
the pattern binds to the three inner `i32` values. The pattern of the second
arm also matches a `Message::ChangeColor` enum variant, but the inner enum
matches the `Color::Hsv` variant instead. We can specify these complex
conditions in one `match` expression, even though two enums are involved.
-->

Le motif de la première branche dans l'expression `match` correspond à la
variante d'énumération `Message::ChangerCouleur` qui contient une variante
`Couleur::Rvb` ; ensuite le motif fait correspondre des variables aux trois
valeurs `i32` à l'intérieur. Le motif de la seconde branche correspond aussi
à une variante de l'énumération de `Message::ChangerCouleur`, mais la valeur
interne correspond plutôt à la variante `Couleur::Tsv`. Nous pouvons renseigner
ces conditions complexes dans une seule expression `match`, même si nous la
faisons sur deux énumérations.

<!--
#### Destructuring Structs and Tuples
-->

#### Destructurer des structures et des tuples

<!--
We can mix, match, and nest destructuring patterns in even more complex ways.
The following example shows a complicated destructure where we nest structs and
tuples inside a tuple and destructure all the primitive values out:
-->

Nous pouvons mélanger les correspondances et les motifs pour déstructurer des
éléments imbriqués de manière bien plus complexe. L'exemple suivant montre une
déstructuration complexe dans laquelle nous imbriquons des structures et des
tuples à l'intérieur d'un tuple et nous y destructurons toutes les valeurs
primitives :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/no-listing-05-destructuring-structs-and-tuples/src/main.rs:here}}
```

<!--
This code lets us break complex types into their component parts so we can use
the values we’re interested in separately.
-->

Ce code nous permet de décomposer les parties qui composent des types complexes
pour pouvoir utiliser séparément les valeurs qui nous intéressent.

<!--
Destructuring with patterns is a convenient way to use pieces of values, such
as the value from each field in a struct, separately from each other.
-->

La déstructuration avec les motifs est un moyen efficace d'utiliser des parties
de valeurs, comme par exemple la valeur de chaque champ d'une structure,
indépendamment les unes des autres.

<!--
### Ignoring Values in a Pattern
-->

### Ignorer des valeurs dans un motif

<!--
You’ve seen that it’s sometimes useful to ignore values in a pattern, such as
in the last arm of a `match`, to get a catchall that doesn’t actually do
anything but does account for all remaining possible values. There are a few
ways to ignore entire values or parts of values in a pattern: using the `_`
pattern (which you’ve seen), using the `_` pattern within another pattern,
using a name that starts with an underscore, or using `..` to ignore remaining
parts of a value. Let’s explore how and why to use each of these patterns.
-->

Vous avez pu constater qu'il est parfois utile d'ignorer des valeurs dans un
motif, comme celle dans la dernière branche d'un `match`, pour obtenir un joker
qui ne fait rien mis à part qu'il représente toutes les autres valeurs
possibles. Il existe plusieurs façons d'ignorer des valeurs entières ou des
parties de valeurs dans un motif : l'utilisation du motif `_` (que vous avez
déjà vu), l'utilisation du motif `_` à l'intérieur d'un autre motif, utiliser
un nom qui commence avec un tiret bas, ou utiliser `..` pour ignorer les
parties restantes d'une valeur. Voyons comment et pourquoi utiliser ces motifs.

<!--
#### Ignoring an Entire Value with `_`
-->

#### Ignorer une valeur entière avec `_`

<!--
We’ve used the underscore (`_`) as a wildcard pattern that will match any value
but not bind to the value. Although the underscore `_` pattern is especially
useful as the last arm in a `match` expression, we can use it in any pattern,
including function parameters, as shown in Listing 18-17.
-->

Nous avons utilisé le tiret bas (`_`) comme un motif de joker qui devait
correspondre avec n'importe quelle valeur mais ne pouvait pas être assigné à
une valeur. Bien que le motif du tiret bas `_` est particulièrement utile dans
la dernière branche d'une expression `match`, nous pouvons l'utiliser dans
n'importe quel motif, y compris les paramètres de fonctions, comme montré dans
l'encart 18-17.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-17/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-17/src/main.rs}}
```

<!--
<span class="caption">Listing 18-17: Using `_` in a function signature</span>
-->

<span class="caption">Encart 18-17 : utilisation d'un `_` dans la signature
d'une fonction</span>

<!--
This code will completely ignore the value passed as the first argument, `3`,
and will print `This code only uses the y parameter: 4`.
-->

Ce code va complètement ignorer la valeur envoyée en premier argument, `3`, et
va afficher `Ce code utilise uniquement le paramètre y : 4`.

<!--
In most cases when you no longer need a particular function parameter, you
would change the signature so it doesn’t include the unused parameter. Ignoring
a function parameter can be especially useful in some cases, for example, when
implementing a trait when you need a certain type signature but the function
body in your implementation doesn’t need one of the parameters. The compiler
will then not warn about unused function parameters, as it would if you used a
name instead.
-->

Dans la plupart des cas lorsque vous n'avez pas besoin d'un paramètre d'une
fonction, vous pouvez changer la signature pour qu'elle n'inclue pas le
paramètre non utilisé. Ignorer un paramètre de fonction peut être
particulièrement utile dans certains cas, comme par exemple, lors de
l'implémentation d'un trait lorsque vous avez besoin d'un certain type de
signature mais que le corps de la fonction dans votre implémentation n'a pas
besoin d'un des paramètres. Le compilateur ne vous avertira plus que ces
paramètres de fonction ne sont pas utilisés, ce qui serait le cas si vous
utilisiez un nom à la place.

<!--
#### Ignoring Parts of a Value with a Nested `_`
-->

#### Ignorer des parties d'une valeur en utilisant un `_` imbriqué

<!--
We can also use `_` inside another pattern to ignore just part of a value, for
example, when we want to test for only part of a value but have no use for the
other parts in the corresponding code we want to run. Listing 18-18 shows code
responsible for managing a setting’s value. The business requirements are that
the user should not be allowed to overwrite an existing customization of a
setting but can unset the setting and give it a value if it is currently unset.
-->

Nous pouvons aussi utiliser `_` au sein d'un autre motif pour ignorer
uniquement une partie d'une valeur, par exemple, si nous souhaitons tester
qu'une seule partie d'une valeur mais que nous n'utilisons pas les autres
parties dans le code que nous souhaitons exécuter. L'encart 18-18 montre du
code qui s'occupe de gérer une valeur d'un réglage. Les règles métier sont que
l'utilisateur ne doit pas pouvoir réécrire un réglage personnalisé mais peut
annuler le réglage et lui donner une valeur s'il est bien inexistant.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-18/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-18: Using an underscore within patterns that
match `Some` variants when we don’t need to use the value inside the
`Some`</span>
-->

<span class="caption">Encart 18-18 : utilisation d'un tiret bas dans des motifs
qui correspondent avec des variantes `Some` lorsque nous n'avons pas besoin
d'utiliser la valeur à l'intérieur du `Some`</span>

<!--
This code will print `Can't overwrite an existing customized value` and then
`setting is Some(5)`. In the first match arm, we don’t need to match on or use
the values inside either `Some` variant, but we do need to test for the case
when `setting_value` and `new_setting_value` are the `Some` variant. In that
case, we print why we’re not changing `setting_value`, and it doesn’t get
changed.
-->

Ce code va afficher `Vous ne pouvez pas écraser une valeur déjà existante` et
ensuite `Le réglage vaut Some(5)`. Dans la première branche, nous n'avons pas
besoin de récupérer ou d'utiliser les valeurs à l'intérieur de chacune des
variantes `Some`, mais nous avons besoin de tester les cas lorsque
`valeur_du_reglage` et `nouvelle_valeur_du_reglage` vaudront la variante
`Some`. Dans ce cas, nous écrivons que nous n'allons pas changer
`valeur_du_reglage`, et il ne changera pas.

<!--
In all other cases (if either `setting_value` or `new_setting_value` are
`None`) expressed by the `_` pattern in the second arm, we want to allow
`new_setting_value` to become `setting_value`.
-->

Dans tous les autres cas (lorsque soit `valeur_du_reglage`, soit
`nouvelle_valeur_du_reglage` vaut `None`) qui correspondront avec le motif
`_` de la seconde branche, nous voulons permettre à la valeur de
`nouvelle_valeur_du_reglage` de remplacer celle de `valeur_du_reglage`.

<!--
We can also use underscores in multiple places within one pattern to ignore
particular values. Listing 18-19 shows an example of ignoring the second and
fourth values in a tuple of five items.
-->

Nous pouvons aussi utiliser les tirets bas à plusieurs endroits dans un même
motif pour ignorer des valeurs précises. L'encart 18-19 montre un exemple qui
ignore la seconde et quatrième valeur dans un tuple de cinq éléments.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-19/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-19/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-19: Ignoring multiple parts of a tuple</span>
-->

<span class="caption">Encart 18-19 : on ignore plusieurs éléments d'un tuple
</span>

<!--
This code will print `Some numbers: 2, 8, 32`, and the values 4 and 16 will be
ignored.
-->

Ce code va afficher `Voici quelques nombres : 2, 8, 32`, et les valeurs 4 et 16
seront ignorées.

<!--
#### Ignoring an Unused Variable by Starting Its Name with `_`
-->

#### Ignorer une variable non utilisée en démarrant son nom avec un `_`

<!--
If you create a variable but don’t use it anywhere, Rust will usually issue a
warning because that could be a bug. But sometimes it’s useful to create a
variable you won’t use yet, such as when you’re prototyping or just starting a
project. In this situation, you can tell Rust not to warn you about the unused
variable by starting the name of the variable with an underscore. In Listing
18-20, we create two unused variables, but when we compile this code, we should
only get a warning about one of them.
-->

Si vous créez une variable mais que vous ne l'utilisez nulle part, Rust va
lancer un avertissement car cela est peut-être un bogue. Mais parfois il est
utile de créer une variable que vous n'utilisez pas encore, ce qui peut arriver
lorsque vous créez un prototype ou créez un projet. Dans ce genre de situation,
vous pouvez demander à Rust de ne pas vous avertir de la variable non utilisée
en débutant le nom de la variable avec un tiret bas. Dans l'encart 18-20, nous
créons deux variables non utilisées, mais lorsque nous compilerons ce code,
nous n'aurons qu'un seul avertissement sur une seule d'entre elles.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-20/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-20/src/main.rs}}
```

<!--
<span class="caption">Listing 18-20: Starting a variable name with an
underscore to avoid getting unused variable warnings</span>
-->

<span class="caption">Encart 18-20 : démarrer le nom d'une variable avec un
tiret bas pour éviter d'avoir des avertissements à propos de variables non
utilisées</span>

<!--
Here we get a warning about not using the variable `y`, but we don’t get a
warning about not using the variable preceded by the underscore.
-->

Ici nous avons un avertissement qui nous prévient que nous n'utilisons pas la
variable `y`, mais nous n'avons pas d'avertissement à propos de la variable qui
a le nom qui commence par un tiret bas.

<!--
Note that there is a subtle difference between using only `_` and using a name
that starts with an underscore. The syntax `_x` still binds the value to the
variable, whereas `_` doesn’t bind at all. To show a case where this
distinction matters, Listing 18-21 will provide us with an error.
-->

Remarquez comme la différence est subtile entre l'utilisation d'uniquement `_`
et un nom qui commence par un tiret bas. La syntaxe `_x` continue à associer la
valeur à une variable, alors que `_` ne le fait pas du tout. Pour montrer un
cas où cette différence est importante, l'encart 18-21 va nous donner une
erreur.

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-21/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-21/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-21: An unused variable starting with an
underscore still binds the value, which might take ownership of the value</span>
-->

<span class="caption">Encart 18-21 : une variable non utilisée qui commence par
un tiret bas continue à assigner la valeur, qui pourrait prendre possession de
la valeur</span>

<!--
We’ll receive an error because the `s` value will still be moved into `_s`,
which prevents us from using `s` again. However, using the underscore by itself
doesn’t ever bind to the value. Listing 18-22 will compile without any errors
because `s` doesn’t get moved into `_`.
-->

Nous allons obtenir une erreur car la valeur `s` est toujours déplacée dans
`_s`, ce qui nous empêche d'utiliser `s` ensuite. Cependant, l'utilisation du
tiret bas tout seul n'assigne jamais la valeur à quelque chose. L'encart 18-22
va se compiler sans aucune erreur car `s` n'est pas déplacé dans `_`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-22/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-22/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-22: Using an underscore does not bind the
value</span>
-->

<span class="caption">Encart 18-22 : l'utilisation d'un tiret bas n'assigne pas
la valeur</span>

<!--
This code works just fine because we never bind `s` to anything; it isn’t moved.
-->

Ce code fonctionne correctement car nous n'assignons jamais `s` à quelque
chose ; elle n'est jamais déplacée.

<!--
#### Ignoring Remaining Parts of a Value with `..`
-->

#### Ignorer les éléments restants d'une valeur avec `..`

<!--
With values that have many parts, we can use the `..` syntax to use only a few
parts and ignore the rest, avoiding the need to list underscores for each
ignored value. The `..` pattern ignores any parts of a value that we haven’t
explicitly matched in the rest of the pattern. In Listing 18-23, we have a
`Point` struct that holds a coordinate in three-dimensional space. In the
`match` expression, we want to operate only on the `x` coordinate and ignore
the values in the `y` and `z` fields.
-->

Avec les valeurs qui ont de nombreux éléments, nous pouvons utiliser la syntaxe
`..` pour utiliser uniquement quelques éléments et ignorer les autres, ce qui
évite d'avoir à faire une liste de tirets bas pour chacune des valeurs
ignorées. Le motif `..` ignore tous les éléments d'une valeur qui ne
correspondent pas explicitement au reste du motif. Dans l'encart 18-23, nous
avons une structure `Point` qui stocke des coordonnées dans un espace
tridimensionnel. Dans l'expression `match`, nous souhaitons utiliser uniquement
la coordonnée `x` et ignorer les valeurs des champs `y` et `z`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-23/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-23/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-23: Ignoring all fields of a `Point` except
for `x` by using `..`</span>
-->

<span class="caption">Encart 18-23 : on ignore tous les champs d'un `Point`, à
l'exception de `x`, en utilisant `..`</span>

<!--
We list the `x` value and then just include the `..` pattern. This is quicker
than having to list `y: _` and `z: _`, particularly when we’re working with
structs that have lots of fields in situations where only one or two fields are
relevant.
-->

Nous ajoutons la valeur `x` et juste ensuite nous insérons le motif `..`. C'est
plus rapide que d'avoir à rajouter `y: _` et `z: _`, en particulier lorsque
nous travaillons avec des structures qui ont beaucoup de champs alors qu'un
seul champ ou deux nous intéressent.

<!--
The syntax `..` will expand to as many values as it needs to be. Listing 18-24
shows how to use `..` with a tuple.
-->

La syntaxe `..` va s'étendre à toutes les valeurs qu'elle devra couvrir.
L'encart 18-24 montre comment utiliser `..` avec un tuple.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-24/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-24/src/main.rs}}
```

<!--
<span class="caption">Listing 18-24: Matching only the first and last values in
a tuple and ignoring all other values</span>
-->

<span class="caption">Encart 18-24 : on correspond uniquement avec la première
et la dernière valeur d'un tuple et nous ignorons toutes les autres valeurs
</span>

<!--
In this code, the first and last value are matched with `first` and `last`. The
`..` will match and ignore everything in the middle.
-->

Dans ce code, la première et la dernière valeur correspondent à `premier` et
`dernier`. Le `..` va correspondre et ignorer tout ce qui se trouve entre les
deux.

<!--
However, using `..` must be unambiguous. If it is unclear which values are
intended for matching and which should be ignored, Rust will give us an error.
Listing 18-25 shows an example of using `..` ambiguously, so it will not
compile.
-->

Cependant, l'utilisation de `..` peut être ambigu. S'il n'est pas clair de
savoir quelles sont les valeurs qui doivent correspondre et celles qui doivent
être ignorées, Rust va nous donner une erreur. L'encart 18-25 nous montre un
exemple d'utilisation ambigu de `..`, donc il ne se compilera pas.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-25/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-25/src/main.rs}}
```

<!--
<span class="caption">Listing 18-25: An attempt to use `..` in an ambiguous
way</span>
-->

<span class="caption">Encart 18-25 : une tentative d'utilisation de `..` de
manière ambigu</span>

<!--
When we compile this example, we get this error:
-->

Lorsque nous compilons cet  exemple, nous obtenons l'erreur suivante :

<!--
```console
{{#include ../listings-sources/ch18-patterns-and-matching/listing-18-25/output.txt}}
```
-->

```console
{{#include ../listings/ch18-patterns-and-matching/listing-18-25/output.txt}}
```

<!--
It’s impossible for Rust to determine how many values in the tuple to ignore
before matching a value with `second` and then how many further values to
ignore thereafter. This code could mean that we want to ignore `2`, bind
`second` to `4`, and then ignore `8`, `16`, and `32`; or that we want to ignore
`2` and `4`, bind `second` to `8`, and then ignore `16` and `32`; and so forth.
The variable name `second` doesn’t mean anything special to Rust, so we get a
compiler error because using `..` in two places like this is ambiguous.
-->

Il n'est pas possible pour Rust de déterminer combien de valeurs sont à ignorer
dans le tuple avant de faire correspondre une valeur avec `second` et ensuite
combien d'autres à ignorer après cela. Ce code pourrait signifier que nous
voulons ignorer `2`, faire correspondre `second` avec `4`, en ensuite ignorer
`8`, `16`, et `32` ; ou que nous souhaitons ignorer `2` et `4`, et faire
correspondre `second` à `8`, et ignorer ensuite `16` et `32` ; et ainsi de
suite. Le nom de la variable `second` ne signifie pas grand-chose pour Rust,
donc nous obtenons une erreur de compilation à cause de l'utilisation de `..`
à deux endroits qui rendent la situation ambigu.

<!--
### Extra Conditionals with Match Guards
-->

### Plus de conditions avec les contrôles de correspondance

<!--
A *match guard* is an additional `if` condition specified after the pattern in
a `match` arm that must also match, along with the pattern matching, for that
arm to be chosen. Match guards are useful for expressing more complex ideas
than a pattern alone allows.
-->

Un *contrôle de correspondance* est une condition `if` supplémentaire renseignée
après le motif d'une branche d'un `match` qui doit elle aussi correspondre, de
même que le filtrage par motif, pour que cette branche soit choisie. Les
contrôles de correspondance sont utiles pour exprimer des idées plus complexes
que celles permises par les motifs tout seuls.

<!--
The condition can use variables created in the pattern. Listing 18-26 shows a
`match` where the first arm has the pattern `Some(x)` and also has a match
guard of `if x < 5`.
-->

La condition peut utiliser des variables créées dans le motif. L'encart 18-26
montre un `match` dans lequel la première branche a le motif `Some(x)` et
procède aussi au contrôle de correspondance `if x < 5`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-26/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-26/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-26: Adding a match guard to a pattern</span>
-->

<span class="caption">Encart 18-26 : ajout d'un contrôle de correspondance à un
motif</span>

<!--
This example will print `less than five: 4`. When `num` is compared to the
pattern in the first arm, it matches, because `Some(4)` matches `Some(x)`. Then
the match guard checks whether the value in `x` is less than `5`, and because
it is, the first arm is selected.
-->

Cet exemple va afficher `moins que cinq : 4`. Lorsque `nombre` est comparé au
motif de la première branche, il va correspondre, car `Some(4)` correspond à
`Some(x)`. Ensuite, le contrôle de correspondance vérifie si la valeur dans `x`
est inférieure à `5`, et comme c'est le cas, la première branche est choisie.

<!--
If `num` had been `Some(10)` instead, the match guard in the first arm would
have been false because 10 is not less than 5. Rust would then go to the second
arm, which would match because the second arm doesn’t have a match guard and
therefore matches any `Some` variant.
-->

Si `nombre` aurait été plutôt `Some(10)`, le contrôle de correspondance de la
première branche aurait été faux car 10 n'est pas inférieur à 5. Rust serait
donc allé à la seconde branche, qui devrait correspondre car la seconde branche
n'a pas de contrôle de correspondance et correspond parfois à la variante
`Some`.

<!--
There is no way to express the `if x < 5` condition within a pattern, so the
match guard gives us the ability to express this logic.
-->

Il n'y a pas d'autre moyen d'exprimer la condition `if x < 5` dans un motif,
donc le contrôle de correspondance nous donne la possibilité d'exprimer cette
logique.

<!--
In Listing 18-11, we mentioned that we could use match guards to solve our
pattern-shadowing problem. Recall that a new variable was created inside the
pattern in the `match` expression instead of using the variable outside the
`match`. That new variable meant we couldn’t test against the value of the
outer variable. Listing 18-27 shows how we can use a match guard to fix this
problem.
-->

Dans l'encart 18-11, nous avions mentionné le fait que nous pouvions utiliser
des contrôles de correspondance pour résoudre notre problème de masquage dans
le motif. Souvenez-vous qu'une nouvelle variable avait été créée à l'intérieur
du motif dans l'expression `match` au lieu d'utiliser la variable située à
l'extérieur du `match`. Cette nouvelle variable implique que nous ne pouvons
pas comparer avec la variable qui se situe à l'extérieur. L'encart 18-27 nous
montre comment nous pouvons utiliser un contrôle de correspondance pour
répondre à ce besoin.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-27/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-27/src/main.rs}}
```

<!--
<span class="caption">Listing 18-27: Using a match guard to test for equality
with an outer variable</span>
-->

<span class="caption">Encart 18-27 : utilisation d'un contrôle de
correspondance pour vérifier l'égalité avec une variable externe au bloc</span>

<!--
This code will now print `Default case, x = Some(5)`. The pattern in the second
match arm doesn’t introduce a new variable `y` that would shadow the outer `y`,
meaning we can use the outer `y` in the match guard. Instead of specifying the
pattern as `Some(y)`, which would have shadowed the outer `y`, we specify
`Some(n)`. This creates a new variable `n` that doesn’t shadow anything because
there is no `n` variable outside the `match`.
-->

Ce code va maintenant afficher `Cas par défaut, x = Some(5)`. Le motif de la
seconde branche du `match` ne crée pas de nouvelle variable `y` qui masquerait
le `y` externe, ce qui signifie que nous pouvons utiliser le `y` externe dans
le contrôle de correspondance. Au lieu de renseigner le motif comme étant
`Some(y)`, ce qui aurait masqué le `y` externe, nous renseignons `Some(n)`.
Cela va créer une nouvelle variable `n` qui ne masque rien car il n'y a pas de
variable `n` à l'extérieur du `match`.

<!--
The match guard `if n == y` is not a pattern and therefore doesn’t introduce
new variables. This `y` *is* the outer `y` rather than a new shadowed `y`, and
we can look for a value that has the same value as the outer `y` by comparing
`n` to `y`.
-->

Le contrôle de correspondance `if n == y` n'est pas un motif et donc il
n'introduit pas de nouvelle variable. Ce `y` *est* la variable externe `y` au
lieu d'être une nouvelle variable masquée `y`, et nous pouvons comparer une
valeur qui a la même valeur que le `y` externe en comparant `n` à `y`.

<!--
You can also use the *or* operator `|` in a match guard to specify multiple
patterns; the match guard condition will apply to all the patterns. Listing
18-28 shows the precedence of combining a match guard with a pattern that uses
`|`. The important part of this example is that the `if y` match guard applies
to `4`, `5`, *and* `6`, even though it might look like `if y` only applies to
`6`.
-->

Vous pouvez aussi utiliser l'opérateur *ou* `|` dans un contrôle de
correspondance pour y renseigner plusieurs motifs ; la condition du contrôle de
correspondance s'effectuera alors sur tous les motifs. L'encart 18-28 montre la
priorité de combinaison d'un contrôle de correspondance sur un motif qui
utilise `|`. La partie importante de cet exemple est que le contrôle de
correspondance `if y` s'applique sur `4`, `5`, *et* `6`, même si `if y` semble
s'appliquer uniquement à `6`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-28/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-28/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-28: Combining multiple patterns with a match
guard</span>
-->

<span class="caption">Encart 18-28 : combinaison de plusieurs motifs avec un
contrôle de correspondance</span>

<!--
The match condition states that the arm only matches if the value of `x` is
equal to `4`, `5`, or `6` *and* if `y` is `true`. When this code runs, the
pattern of the first arm matches because `x` is `4`, but the match guard `if y`
is false, so the first arm is not chosen. The code moves on to the second arm,
which does match, and this program prints `no`. The reason is that the `if`
condition applies to the whole pattern `4 | 5 | 6`, not only to the last value
`6`. In other words, the precedence of a match guard in relation to a pattern
behaves like this:
-->

La condition de correspondance signifie que la branche correspond uniquement si
la valeur de `x` vaut `4`, `5`, ou `6` *et* que `y` vaut `true`. Lorsque ce
code s'exécute, le motif de la première branche correspond car `x` vaut 4, mais
le contrôle de correspondance `if y` est faux, donc ce programme affiche `no`.
La raison est que la condition `if` s'applique à tout le motif `4 | 5 | 6`, et
pas seulement à la dernière valeur `6`. Autrement dit, la priorité d'un
contrôle de correspondance avec un motif se comporte comme ceci :

<!--
```text
(4 | 5 | 6) if y => ...
```
-->

```text
(4 | 5 | 6) if y => ...
```

<!--
rather than this:
-->

plutôt que comme ceci :

<!--
```text
4 | 5 | (6 if y) => ...
```
-->

```text
4 | 5 | (6 if y) => ...
```

<!--
After running the code, the precedence behavior is evident: if the match guard
were applied only to the final value in the list of values specified using the
`|` operator, the arm would have matched and the program would have printed
`yes`.
-->

Après avoir exécuté le code, le fonctionnement des priorités devient évident :
si le contrôle de correspondance était seulement appliqué à la dernière valeur
renseignée avec l'opérateur `|`, la branche correspondrait et le programme
aurait affiché `yes`.

<!--
### `@` Bindings
-->

### Capturer des valeurs avec `@`

<!--
The *at* operator (`@`) lets us create a variable that holds a value at the
same time we’re testing that value to see whether it matches a pattern. Listing
18-29 shows an example where we want to test that a `Message::Hello` `id` field
is within the range `3..=7`. But we also want to bind the value to the variable
`id_variable` so we can use it in the code associated with the arm. We could
name this variable `id`, the same as the field, but for this example we’ll use
a different name.
-->

L'opérateur `@` nous permet de créer une variable qui stocke une valeur en même
temps que nous testons cette valeur pour vérifier si elle correspond à un
motif. L'encart 18-29 montre un exemple dans lequel nous souhaitons tester
qu'un champ `id` d'un `Message::Hello` est dans un intervalle `3..=7`. Mais
nous voulons aussi associer la valeur à la variable `id_variable` pour que nous
puissions l'utiliser dans le code associé à la branche. Nous aurions pu nommer
cette variable avec le même nom que le champ `id`, mais pour cet exemple nous
allons utiliser un nom différent.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch18-patterns-and-matching/listing-18-29/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch18-patterns-and-matching/listing-18-29/src/main.rs:here}}
```

<!--
<span class="caption">Listing 18-29: Using `@` to bind to a value in a pattern
while also testing it</span>
-->

<span class="caption">Encart 18-29 : utilisation de `@` pour lier une valeur
d'un motif à une variable pendant qu'on la teste</span>

<!--
This example will print `Found an id in range: 5`. By specifying `id_variable
@` before the range `3..=7`, we’re capturing whatever value matched the range
while also testing that the value matched the range pattern.
-->

Cet exemple va afficher `Nous avons trouvé un id dans l'intervalle : 5`. En
renseignant `id_variable @` avant l'intervalle `3..=7`, nous capturons la
valeur qui correspond à l'intervalle pendant que nous vérifions que la valeur
correspond au motif de l'intervalle.

<!--
In the second arm, where we only have a range specified in the pattern, the code
associated with the arm doesn’t have a variable that contains the actual value
of the `id` field. The `id` field’s value could have been 10, 11, or 12, but
the code that goes with that pattern doesn’t know which it is. The pattern code
isn’t able to use the value from the `id` field, because we haven’t saved the
`id` value in a variable.
-->

Dans la seconde branche, où nous avons uniquement un intervalle renseigné dans
le motif, le code associé à la branche n'a pas besoin d'une variable qui
contient la valeur actuelle du champ `id`. La valeur du champ `id` aurait pu
être 10, 11, ou 12, mais le code associé à ce motif ne saura pas quelle est sa
valeur. Le code du motif n'est pas capable d'utiliser la valeur du champ `id`,
car nous n'avons pas enregistré `id` dans une variable.

<!--
In the last arm, where we’ve specified a variable without a range, we do have
the value available to use in the arm’s code in a variable named `id`. The
reason is that we’ve used the struct field shorthand syntax. But we haven’t
applied any test to the value in the `id` field in this arm, as we did with the
first two arms: any value would match this pattern.
-->

Dans la dernière branche, nous avons renseigné une variable sans intervalle,
nous avons la valeur qui peut être utilisée dans le code de la branche, dans la
variable `id`. La raison à cela est que nous avons utilisé la syntaxe
raccourcie pour les champs des structures. Mais nous n'avons pas appliqué de
tests à la valeur sur le champ `id` de cette branche, comme nous l'avions fait
avec les deux premières branches : n'importe quelle valeur correspondra à ce
motif.

<!--
Using `@` lets us test a value and save it in a variable within one pattern.
-->

L'utilisation de `@` nous permet de tester une valeur et de l'enregistrer dans
une variable au sein d'un seul et même motif.

<!--
## Summary
-->

## Résumé

<!--
Rust’s patterns are very useful in that they help distinguish between different
kinds of data. When used in `match` expressions, Rust ensures your patterns
cover every possible value, or your program won’t compile. Patterns in `let`
statements and function parameters make those constructs more useful, enabling
the destructuring of values into smaller parts at the same time as assigning to
variables. We can create simple or complex patterns to suit our needs.
-->

Les motifs de Rust sont très utiles lorsque nous devons distinguer différents
types de données. Lorsque nous les avions utilisés dans les expressions
`match`, Rust s'est assuré que vos motifs couvent l'intégralité de toutes
valeurs possibles, ou alors votre programme ne se compilait pas. Les motifs
dans les instructions `let` et les paramètres de fonction rendre ces
constructions encore plus utiles, permettant de déstructurer les valeurs en
parties plus petites tout en les assignant à des variables. Nous pouvons créer
des motifs très simples ou alors plus complexes pour répondre à nos besoins.

<!--
Next, for the penultimate chapter of the book, we’ll look at some advanced
aspects of a variety of Rust’s features.
-->

Dans le chapitre suivant, qui sera l'avant-dernier du livre, nous allons
découvrir quelques aspects avancés de l'éventail de fonctionnalités de Rust.
