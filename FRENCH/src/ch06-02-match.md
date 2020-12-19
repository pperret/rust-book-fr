<!--
## The `match` Control Flow Operator
-->

## La structure de contrôle `match`

<!--
Rust has an extremely powerful control flow operator called `match` that allows
you to compare a value against a series of patterns and then execute code based
on which pattern matches. Patterns can be made up of literal values, variable
names, wildcards, and many other things; Chapter 18 covers all the different
kinds of patterns and what they do. The power of `match` comes from the
expressiveness of the patterns and the fact that the compiler confirms that all
possible cases are handled.
-->

Rust a un opérateur de contrôle très puissant appelé `match` qui vous permet de
comparer une valeur avec une série de motifs et d'exécuter du code en fonction
du motif qui correspond. Les motifs peuvent être constitués de valeurs
littérales, de noms de variables, de jokers, parmi tant d'autres ; le
chapitre 18 va couvrir tous les différents types de motifs et ce qu'ils font. Ce
qui fait la puissance de `match` est l'expressivité des motifs et le fait que le
compilateur vérifie que tous les cas possibles sont bien gérés.

<!--
Think of a `match` expression as being like a coin-sorting machine: coins slide
down a track with variously sized holes along it, and each coin falls through
the first hole it encounters that it fits into. In the same way, values go
through each pattern in a `match`, and at the first pattern the value “fits,”
the value falls into the associated code block to be used during execution.
-->

Considérez l'expression `match` comme une machine à trier les pièces de
monnaie : les pièces descendent le long d'une piste avec des trous de tailles
différentes, et chaque pièce tombe dans le premier trou à sa taille qu'elle
rencontre. De manière similaire, les valeurs parcourent tous les motifs dans un
`match`, et au premier motif auquel la valeur “correspond”, la valeur va
descendre dans le bloc de code correspondant afin d'être utilisée pendant son
exécution.

<!--
Because we just mentioned coins, let’s use them as an example using `match`! We
can write a function that can take an unknown United States coin and, in a
similar way as the counting machine, determine which coin it is and return its
value in cents, as shown here in Listing 6-3.
-->

Comme nous venons de mentionner des pièces, utilisons-les avec un exemple qui
utilise `match` ! Nous pouvons écrire une fonction qui prend en paramètre une
pièce inconnue des États-Unis d'Amérique et qui peut, de la même manière qu'une
machine à trier, déterminer quelle pièce c'est et retourner sa valeur en
centimes, comme ci-dessous dans l'encart 6-3.

<!-- markdownlint-disable -->
<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```
-->
<!-- markdownlint-restore -->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-03/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-3: An enum and a `match` expression that has
the variants of the enum as its patterns</span>
-->

<span class="caption">Encart 6-3 : Une énumération et une expression `match` qui
trie les variantes de l'énumération dans ses motifs</span>

<!--
Let’s break down the `match` in the `value_in_cents` function. First, we list
the `match` keyword followed by an expression, which in this case is the value
`coin`. This seems very similar to an expression used with `if`, but there’s a
big difference: with `if`, the expression needs to return a Boolean value, but
here, it can be any type. The type of `coin` in this example is the `Coin` enum
that we defined on line 1.
-->

Décomposons le `match` dans la fonction `valeur_en_centimes`. En premier lieu,
nous utilisons le mot-clé `match` suivi par une expression, qui dans notre cas
est la valeur de `piece`. Cela ressemble beaucoup à une expression utilisée avec
`if`, mais il y a une grosse différence : avec `if`, l'expression doit retourner
une valeur booléenne, mais ici, elle peut être de n'importe quel type. Dans cet
exemple, `piece` est de type `PieceUs`, qui est l'énumération que nous avons
définie à la ligne 1.

<!--
Next are the `match` arms. An arm has two parts: a pattern and some code. The
first arm here has a pattern that is the value `Coin::Penny` and then the `=>`
operator that separates the pattern and the code to run. The code in this case
is just the value `1`. Each arm is separated from the next with a comma.
-->

Ensuite, nous avons les branches du `match`. Une branche a deux parties : un
motif et du code. La première branche a ici pour motif la valeur
`PieceUs::Penny` et ensuite l'opérateur `=>` qui sépare le motif et le code à
exécuter. Le code dans ce cas est uniquement la valeur `1`. Chaque branche est
séparée de la suivante par une virgule.

<!--
When the `match` expression executes, it compares the resulting value against
the pattern of each arm, in order. If a pattern matches the value, the code
associated with that pattern is executed. If that pattern doesn’t match the
value, execution continues to the next arm, much as in a coin-sorting machine.
We can have as many arms as we need: in Listing 6-3, our `match` has four arms.
-->

Lorsqu'une expression `match` est exécutée, elle compare la valeur de `piece`
avec le motif de chaque branche, dans l'ordre. Si un motif correspond à la
valeur, le code correspondant à ce motif est alors exécuté. Si ce motif ne
correspond pas à la valeur, l'exécution passe à la prochaine branche, un peu
comme dans une machine de tri de pièces. Nous pouvons avoir autant de branches
que nécessaire : dans l'encart 6-3, notre `match` a quatre branches.

<!--
The code associated with each arm is an expression, and the resulting value of
the expression in the matching arm is the value that gets returned for the
entire `match` expression.
-->

Le code correspondant à chaque branche est une expression, et la valeur qui
résulte de l'expression dans la branche correspondante est la valeur qui sera
retournée par l'expression `match`.

<!--
Curly brackets typically aren’t used if the match arm code is short, as it is
in Listing 6-3 where each arm just returns a value. If you want to run multiple
lines of code in a match arm, you can use curly brackets. For example, the
following code would print “Lucky penny!” every time the method was called with
a `Coin::Penny` but would still return the last value of the block, `1`:
-->

Les accolades ne sont généralement pas utilisées si le code de la branche
correspondante est court, comme c'est le cas dans l'encart 6-3 où chaque branche
retourne simplement une valeur. Si vous voulez exécuter plusieurs lignes de
code dans une branche d'un `match`, vous devez utiliser les accolades. Par
exemple, le code suivant va afficher “Un centime porte-bonheur !” à chaque fois
que la méthode est appelée avec une valeur `PieceUs::Penny` mais va continuer à
retourner la dernière valeur du bloc, `1` :

<!-- markdownlint-disable -->
<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```
-->
<!-- markdownlint-restore -->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-08-match-arm-multiple-lines/src/main.rs:here}}
```

<!--
### Patterns that Bind to Values
-->

### Des motifs reliés à des valeurs

<!--
Another useful feature of match arms is that they can bind to the parts of the
values that match the pattern. This is how we can extract values out of enum
variants.
-->

Une autre fonctionnalité intéressante des branches de `match` est qu'elles
peuvent se lier aux valeurs qui correspondent au motif. C'est ainsi que nous
pouvons extraire des valeurs d'une variante d'énumération.

<!--
As an example, let’s change one of our enum variants to hold data inside it.
From 1999 through 2008, the United States minted quarters with different
designs for each of the 50 states on one side. No other coins got state
designs, so only quarters have this extra value. We can add this information to
our `enum` by changing the `Quarter` variant to include a `UsState` value stored
inside it, which we’ve done here in Listing 6-4.
-->

En guise d'exemple, changeons une de nos variantes d'énumération pour stocker
une donnée à l'intérieur. Entre 1999 et 2008, les États-Unis d'Amérique ont
frappé un côté des *quarters* (pièces de 25 centimes) avec des dessins
différents pour chacun des 50 États. Les autres pièces n'ont pas eu de dessins
d'États, donc seul le *quarter* a cette valeur en plus. Nous pouvons ajouter
cette information à notre `enum` en changeant la variante `Quarter` pour y
ajouter une valeur `EtatUs` qui y sera stockée à l'intérieur, comme nous
l'avons fait dans l'encart 6-4.

<!-- markdownlint-disable -->
<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```
-->
<!-- markdownlint-restore -->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-4: A `Coin` enum in which the `Quarter` variant
also holds a `UsState` value</span>
-->

<span class="caption">Encart 6-4 : Une énumération `PieceUs` dans laquelle la
variante `Quarter` stocke en plus une valeur de type `EtatUs`</span>

<!--
Let’s imagine that a friend of ours is trying to collect all 50 state quarters.
While we sort our loose change by coin type, we’ll also call out the name of
the state associated with each quarter so if it’s one our friend doesn’t have,
they can add it to their collection.
-->

Imaginons qu'un de vos amis essaye de collectionner tous les *quarters* des 50
États. Pendant que nous trions notre monnaie en vrac par type de pièce, nous
mentionnerons aussi le nom de l'État correspondant à chaque *quarter* de sorte
que si notre ami ne l'a pas, il puisse l'ajouter à sa collection.

<!--
In the match expression for this code, we add a variable called `state` to the
pattern that matches values of the variant `Coin::Quarter`. When a
`Coin::Quarter` matches, the `state` variable will bind to the value of that
quarter’s state. Then we can use `state` in the code for that arm, like so:
-->

Dans l'expression `match` de ce code, nous avons ajouté une variable `etat` au
motif qui correspond à la variante `PieceUs::Quarter`. Quand on aura une
correspondance `PieceUs::Quarter`, la variable `etat` sera liée à la valeur de
l'État de cette pièce. Ensuite, nous pourrons utiliser `etat` dans le code de
cette branche, comme ceci :

<!-- markdownlint-disable -->
<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```
-->
<!-- markdownlint-restore -->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-09-variable-in-pattern/src/main.rs:here}}
```

<!--
If we were to call `value_in_cents(Coin::Quarter(UsState::Alaska))`, `coin`
would be `Coin::Quarter(UsState::Alaska)`. When we compare that value with each
of the match arms, none of them match until we reach `Coin::Quarter(state)`. At
that point, the binding for `state` will be the value `UsState::Alaska`. We can
then use that binding in the `println!` expression, thus getting the inner
state value out of the `Coin` enum variant for `Quarter`.
-->

Si nous appelons `valeur_en_centimes(PieceUs::Quarter(EtatUs::Alaska))`, `piece`
vaudra `PieceUs::Quarter(EtatUs::Alaska)`. Quand nous comparons cette valeur
avec toutes les branches du `match`, aucune d'entre elles ne correspondra
jusqu'à ce qu'on arrive à `PieceUs::Quarter(etat)`. À partir de ce moment, la
variable `etat` aura la valeur `EtatUs::Alaska`. Nous pouvons alors utiliser
cette variable dans l'expression `println!`, ce qui nous permet d'afficher la
valeur de l'État à l'intérieur de la variante `Quarter` de l'énumération
`PieceUs`.

<!--
### Matching with `Option<T>`
-->

### Utiliser `match` avec `Option<T>`

<!--
In the previous section, we wanted to get the inner `T` value out of the `Some`
case when using `Option<T>`; we can also handle `Option<T>` using `match` as we
did with the `Coin` enum! Instead of comparing coins, we’ll compare the
variants of `Option<T>`, but the way that the `match` expression works remains
the same.
-->

Dans la section précédente, nous voulions obtenir la valeur interne `T` dans le
cas de `Some` lorsqu'on utilisait `Option<T>` ; nous pouvons aussi gérer les
`Option<T>` en utilisant `match` comme nous l'avons fait avec l'énumération
`PieceUs` ! Au lieu de comparer des pièces, nous allons comparer les variantes
de `Option<T>`, mais la façon d'utiliser l'expression `match` reste la même.

<!--
Let’s say we want to write a function that takes an `Option<i32>` and, if
there’s a value inside, adds 1 to that value. If there isn’t a value inside,
the function should return the `None` value and not attempt to perform any
operations.
-->

Disons que nous voulons écrire une fonction qui prend une `Option<i32>` et qui,
s'il y a une valeur à l'intérieur, ajoute 1 à cette valeur. S'il n'y a pas de
valeur à l'intérieur, la fonction retournera la valeur `None` et ne va rien
faire de plus.

<!--
This function is very easy to write, thanks to `match`, and will look like
Listing 6-5.
-->

Cette fonction est très facile à écrire, grâce à `match`, et ressemblera à
l'encart 6-5.

<!-- markdownlint-disable -->
<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```
-->
<!-- markdownlint-restore -->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:here}}
```

<!--
<span class="caption">Listing 6-5: A function that uses a `match` expression on
an `Option<i32>`</span>
-->

<span class="caption">Encart 6-5 : Une fonction qui utilise une expression
`match` sur une `Option<i32>`</span>

<!--
Let’s examine the first execution of `plus_one` in more detail. When we call
`plus_one(five)`, the variable `x` in the body of `plus_one` will have the
value `Some(5)`. We then compare that against each match arm.
-->

Examinons la première exécution de `plus_un` en détail. Lorsque nous appelons
`plus_un(cinq)`, la variable `x` dans le corps de `plus_un` aura la valeur
`Some(5)`. Ensuite, nous comparons cela à chaque branche du `match`.

<!-- markdownlint-disable -->
<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```
-->
<!-- markdownlint-restore -->

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
The `Some(5)` value doesn’t match the pattern `None`, so we continue to the
next arm.
-->

La valeur `Some(5)` ne correspond pas au motif `None`, donc nous continuons à la
branche suivante.

<!-- markdownlint-disable -->
<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```
-->
<!-- markdownlint-restore -->

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:second_arm}}
```

<!--
Does `Some(5)` match `Some(i)`? Why yes it does! We have the same variant. The
`i` binds to the value contained in `Some`, so `i` takes the value `5`. The
code in the match arm is then executed, so we add 1 to the value of `i` and
create a new `Some` value with our total `6` inside.
-->

Est-ce que `Some(5)` correspond au motif `Some(i)` ? Bien sûr ! Nous avons la
même variante. Le `i` va prendre la valeur contenue dans le `Some`, donc `i`
prend la valeur `5`. Le code dans la branche du `match` est exécuté, donc nous
ajoutons 1 à la valeur de `i` et nous créons une nouvelle valeur `Some` avec
notre résultat `6` à l'intérieur.

<!--
Now let’s consider the second call of `plus_one` in Listing 6-5, where `x` is
`None`. We enter the `match` and compare to the first arm.
-->

Maintenant, regardons le second appel à `plus_un` dans l'encart 6-5, où `x` vaut
`None`. Nous entrons dans le `match` et nous le comparons à la première branche.

<!-- markdownlint-disable -->
<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```
-->
<!-- markdownlint-restore -->

```rust,ignore
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/listing-06-05/src/main.rs:first_arm}}
```

<!--
It matches! There’s no value to add to, so the program stops and returns the
`None` value on the right side of `=>`. Because the first arm matched, no other
arms are compared.
-->

Cela correspond ! Il n'y a pas de valeur à additionner, donc le programme
s'arrête et retourne la valeur `None` qui est dans le côté droit du `=>`. Comme
la première branche correspond, les autres branches ne sont pas comparées.

<!--
Combining `match` and enums is useful in many situations. You’ll see this
pattern a lot in Rust code: `match` against an enum, bind a variable to the
data inside, and then execute code based on it. It’s a bit tricky at first, but
once you get used to it, you’ll wish you had it in all languages. It’s
consistently a user favorite.
-->

La combinaison de `match` et des énumérations est utile dans de nombreuses
situations. Vous allez revoir de nombreuses fois ce schéma dans du code Rust :
utiliser `match` sur une énumération, récupérer la valeur qu'elle renferme, et
exécuter du code en fonction de sa valeur. C'est un peu délicat au début, mais
une fois que vous vous y êtes habitué, vous regretterez de ne pas l'avoir dans
les autres langages. Cela devient toujours l'outil préféré de ses utilisateurs.

<!--
### Matches Are Exhaustive
-->

### Les `match` sont toujours exhaustifs

<!--
There’s one other aspect of `match` we need to discuss. Consider this version
of our `plus_one` function that has a bug and won’t compile:
-->

Il y a un autre point de `match` que nous devons aborder. Examinez cette version
de notre fonction `plus_un` qui a un bogue et ne va pas se compiler :

<!-- markdownlint-disable -->
<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```
-->
<!-- markdownlint-restore -->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/src/main.rs:here}}
```

<!--
We didn’t handle the `None` case, so this code will cause a bug. Luckily, it’s
a bug Rust knows how to catch. If we try to compile this code, we’ll get this
error:
-->

Nous n'avons pas géré le cas du `None`, donc ce code va générer un bogue.
Heureusement, c'est un bogue que Rust sait gérer. Si nous essayons de compiler
ce code, nous allons obtenir cette erreur :

<!-- markdownlint-disable -->
<!--
```console
{{#include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```
-->
<!-- markdownlint-restore -->

```console
{{#include ../listings/ch06-enums-and-pattern-matching/no-listing-10-non-exhaustive-match/output.txt}}
```

<!--
Rust knows that we didn’t cover every possible case and even knows which
pattern we forgot! Matches in Rust are *exhaustive*: we must exhaust every last
possibility in order for the code to be valid. Especially in the case of
`Option<T>`, when Rust prevents us from forgetting to explicitly handle the
`None` case, it protects us from assuming that we have a value when we might
have null, thus making the billion-dollar mistake discussed earlier impossible.
-->

Rust sait que nous n'avons pas couvert toutes les possibilités et sait même quel
motif nous avons oublié ! Les `match` de Rust sont *exhaustifs* : nous devons
traiter toutes les possibilités afin que le code soit valide. C'est notamment le
cas avec `Option<T>` : quand Rust nous empêche d'oublier de gérer explicitement
le cas de `None`, il nous protège d'une situation où nous supposons que nous
avons une valeur alors que nous pourrions avoir null, ce qui rend impossible
l'erreur à un milliard de dollars que nous avons vue précédemment.

<!--
### The `_` Placeholder
-->

### Le motif générique `_`

<!--
Rust also has a pattern we can use when we don’t want to list all possible
values. For example, a `u8` can have valid values of 0 through 255. If we only
care about the values 1, 3, 5, and 7, we don’t want to have to list out 0, 2,
4, 6, 8, 9 all the way up to 255. Fortunately, we don’t have to: we can use the
special pattern `_` instead:
-->

Rust a aussi un motif que nous pouvons utiliser quand nous ne voulons pas lister
toutes les valeurs possibles. Par exemple, les valeurs valides d'un `u8` vont de
0 à 255. Si nous n'avons besoin que des valeurs 1, 3, 5 et 7, nous ne voulons
pas lister 0, 2, 4, 6, 8, 9 et ainsi de suite jusqu'à 255. Heureusement, nous
n'avons pas à le faire : nous pouvons utiliser le motif spécial `_` à la place :

<!-- markdownlint-disable -->
<!--
```rust
{{#rustdoc_include ../listings-sources/ch06-enums-and-pattern-matching/no-listing-11-underscore-placeholder/src/main.rs:here}}
```
-->
<!-- markdownlint-restore -->

```rust
{{#rustdoc_include ../listings/ch06-enums-and-pattern-matching/no-listing-11-underscore-placeholder/src/main.rs:here}}
```

<!--
The `_` pattern will match any value. By putting it after our other arms, the
`_` will match all the possible cases that aren’t specified before it. The `()`
is just the unit value, so nothing will happen in the `_` case. As a result, we
can say that we want to do nothing for all the possible values that we don’t
list before the `_` placeholder.
-->

Le motif `_` va correspondre à n'importe quelle valeur. En l'ajoutant après
toutes nos autres branches, le `_` va correspondre à tous les cas possibles qui
ne sont pas listés avant. Le `()` est tout simplement la valeur unité, donc rien
n'arrivera dans le cas du `_`. Ainsi, nous pouvons dire que nous ne voulons rien
faire pour les valeurs possibles que nous ne traitons pas avant le motif
générique `_`.

<!--
However, the `match` expression can be a bit wordy in a situation in which we
care about only *one* of the cases. For this situation, Rust provides `if let`.
-->

Cependant, l'expression `match` peut être un peu lourde dans une situation où
nous nous préoccupons uniquement d'*un seul* cas. Pour cette situation, Rust
nous fournit la structure de contrôle `if let`.

<!--
More about patterns and matching can be found in [chapter 18][ch18-00-patterns].
-->

Vous pouvez en apprendre plus sur les motifs et le filtrage par motif au
[chapitre 18][ch18-00-patterns].

<!--
[ch18-00-patterns]:
ch18-00-patterns.html
-->

[ch18-00-patterns]:
ch18-00-patterns.html
