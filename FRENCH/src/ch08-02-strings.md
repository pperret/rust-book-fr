<!--
## Storing UTF-8 Encoded Text with Strings
-->

## Stocker du texte encodé en UTF-8 avec les Strings

<!--
We talked about strings in Chapter 4, but we’ll look at them in more depth now.
New Rustaceans commonly get stuck on strings for a combination of three
reasons: Rust’s propensity for exposing possible errors, strings being a more
complicated data structure than many programmers give them credit for, and
UTF-8. These factors combine in a way that can seem difficult when you’re
coming from other programming languages.
-->

Nous avons déjà parlé des chaînes de caractères dans le chapitre 4, mais nous
allons à présent les analyser plus en détail. Les nouveaux Rustacés bloquent
souvent avec les chaînes de caractères pour trois raisons : la tendance de Rust
à prévenir les erreurs, le fait que les chaînes de caractères sont des structures de
données plus compliquées que ne le pensent la plupart des développeurs, et
l'UTF-8. Ces raisons cumulées rendent les choses compliquées lorsque vous
venez d'un autre langage de programmation.

<!--
It’s useful to discuss strings in the context of collections because strings
are implemented as a collection of bytes, plus some methods to provide useful
functionality when those bytes are interpreted as text. In this section, we’ll
talk about the operations on `String` that every collection type has, such as
creating, updating, and reading. We’ll also discuss the ways in which `String`
is different from the other collections, namely how indexing into a `String` is
complicated by the differences between how people and computers interpret
`String` data.
-->

Il est pertinent de présenter les chaînes de caractères comme des collections
car les chaînes de caractères sont en réalité des ensembles d'octets, avec
quelques méthodes supplémentaires qui sont utiles lorsque ces octets sont
considérés comme du texte. Dans cette section, nous allons voir les points
communs entre le fonctionnement des `String` et celui des autres collections,
comme la création, la modification, et la lecture. Nous verrons les raisons pour
lesquelles les `String` sont différents des autres collections, en particulier
pourquoi l'indexation d'un `String` est compliquée à cause de la façon dont les
gens et les ordinateurs interprètent les données d'une `String`.

<!--
### What Is a String?
-->

### Qu'est-ce qu'une chaîne de caractères ?

<!--
We’ll first define what we mean by the term *string*. Rust has only one string
type in the core language, which is the string slice `str` that is usually seen
in its borrowed form `&str`. In Chapter 4, we talked about *string slices*,
which are references to some UTF-8 encoded string data stored elsewhere. String
literals, for example, are stored in the program’s binary and are therefore
string slices.
-->

Nous allons d'abord définir ce que nous entendons par le terme *chaîne de
caractères*. Rust a un seul type de chaînes de caractères dans le noyau du
langage, qui est la slice de chaîne de caractères `str` qui est habituellement
utilisée sous sa forme empruntée, `&str`. Dans le chapitre 4, nous avons abordé
les *slices de chaîne de caractères*, qui sont des références à une partie des
données d'une chaîne de caractères encodée en UTF-8 qui sont stockés autre part.
Les chaînes de caractères pures, par exemple, sont stockées dans le binaire du
programme et sont des slices de chaînes de caractères.

<!--
The `String` type, which is provided by Rust’s standard library rather than
coded into the core language, is a growable, mutable, owned, UTF-8 encoded
string type. When Rustaceans refer to “strings” in Rust, they usually mean the
`String` and the string slice `&str` types, not just one of those types.
Although this section is largely about `String`, both types are used heavily in
Rust’s standard library, and both `String` and string slices are UTF-8 encoded.
-->

Le type `String`, qui est fourni par la bibliothèque standard de Rust plutôt que
d'être intégré au noyau du langage, est un type de chaîne de caractères encodé
en UTF-8 qui peut s'agrandir, être mutable, et être possédé. Lorsque les
Rustacés parlent de “chaînes de caractères” en Rust, cela désigne le type
`String` mais aussi le type de slice de chaînes de caractères `&str`, et non pas
un seul de ces types. Bien que cette section traite essentiellement de `String`,
ces deux types sont utilisés massivement dans la bibliothèque standard de Rust,
et tous les deux sont encodés en UTF-8.

<!--
Rust’s standard library also includes a number of other string types, such as
`OsString`, `OsStr`, `CString`, and `CStr`. Library crates can provide even
more options for storing string data. See how those names all end in `String`
or `Str`? They refer to owned and borrowed variants, just like the `String` and
`str` types you’ve seen previously. These string types can store text in
different encodings or be represented in memory in a different way, for
example. We won’t discuss these other string types in this chapter; see their
API documentation for more about how to use them and when each is appropriate.
-->

La bibliothèque standard de Rust apporte aussi un certain nombre d'autres types
de chaînes de caractères, comme `OsString`, `OsStr`, `CString`, et `CStr`. Les
crates de bibliothèque peuvent fournir encore plus de solutions pour stocker des
chaînes de caractères. Avez vous remarqué que ces noms finissent tous par `String`
ou `Str` ? Cela fait référence aux variantes possédées et empruntées, comme les
types `String` et `str` que nous avons vu précédemment. Ces types de chaînes de
caractères peuvent stocker leur texte dans de différents encodages, ou le
stocker en mémoire de manière différente, par exemple. Nous n'allons pas traiter
ces autres types de chaînes de caractères dans ce chapitre ; référez-vous à
la documentation de leur API pour en savoir plus sur leur utilisation et leur utilité.

<!--
### Creating a New String
-->

### Créer une nouvelle String

<!--
Many of the same operations available with `Vec<T>` are available with `String`
as well, starting with the `new` function to create a string, shown in Listing
8-11.
-->

De nombreuses opérations disponibles avec `Vec<T>` sont aussi disponibles avec
`String`, en commençant par la fonction `new` pour créer une `String`, utilisée
dans l'encart 8-11.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-11/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-11: Creating a new, empty `String`</span>
-->

<span class="caption">Encart 8-11 : Création d'une nouvelle `String` vide</span>

<!--
This line creates a new empty string called `s`, which we can then load data
into. Often, we’ll have some initial data that we want to start the string
with. For that, we use the `to_string` method, which is available on any type
that implements the `Display` trait, as string literals do. Listing 8-12 shows
two examples.
-->

Cette ligne créée une nouvelle `String` vide qui s'appelle `s`, dans laquelle
nous pouvons ensuite y charger des données. Parfois, nous aurons quelques
données initiales que nous voudrions ajouter dans la `String`. Pour cela, nous
utilisons la méthode `to_string`, qui est disponible sur tous les types qui
implémentent le trait `Display`, comme le font les chaînes de caractères pures.
L'encart 8-12 nous montre deux exemples.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-12/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-12/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-12: Using the `to_string` method to create a
`String` from a string literal</span>
-->

<span class="caption">Encart 8-12 : Utilisation de la méthode `to_string` pour
créer un `String` à partir d'une chaîne de caractères pure</span>

<!--
This code creates a string containing `initial contents`.
-->

Ce code crée une `String` qui contient `contenu initial`.

<!--
We can also use the function `String::from` to create a `String` from a string
literal. The code in Listing 8-13 is equivalent to the code from Listing 8-12
that uses `to_string`.
-->

Nous pouvons aussi utiliser la fonction `String::from` pour créer une `String`
à partir d'une chaîne de caractères pure. Le code dans l'encart 8-13 est
équivalent au code dans l'encart 8-12 qui utilisait `to_string`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-13/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-13/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-13: Using the `String::from` function to create
a `String` from a string literal</span>
-->

<span class="caption">Encart 8-13 : Utilisation de la fonction `String::from`
afin de créer une `String` à partir d'une chaîne de caractères pure</span>

<!--
Because strings are used for so many things, we can use many different generic
APIs for strings, providing us with a lot of options. Some of them can seem
redundant, but they all have their place! In this case, `String::from` and
`to_string` do the same thing, so which you choose is a matter of style.
-->

Comme les chaînes de caractères sont utilisées pour de nombreuses choses, nous
pouvons utiliser beaucoup d'APIs génériques pour les chaînes de caractères.
Certaines d'entre elles peuvent paraître redondantes, mais elles ont toutes
leur place ! Dans notre cas, `String::from` et `to_string` font la même
chose, donc votre choix est une question de goût.

<!--
Remember that strings are UTF-8 encoded, so we can include any properly encoded
data in them, as shown in Listing 8-14.
-->

Souvenez-vous que les chaînes de caractères sont encodées en UTF-8, donc nous
pouvons y intégrer n'importe quelle donnée valide, comme nous
le voyons dans l'encart 8-14.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-14/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-14: Storing greetings in different languages in
strings</span>
-->

<span class="caption">Encart 8-14 : Stockage de texte dans différents langages
dans des chaînes de caractères</span>

<!--
All of these are valid `String` values.
-->

Toutes ces chaînes sont des valeurs `String` valides.

<!--
### Updating a String
-->

### Modifier une `String`

<!--
A `String` can grow in size and its contents can change, just like the contents
of a `Vec<T>`, if you push more data into it. In addition, you can conveniently
use the `+` operator or the `format!` macro to concatenate `String` values.
-->

Une `String` peut s'agrandir et son contenu peut changer, exactement comme le
contenu d'un `Vec<T>`, si vous utilisez `push` pour y rajouter des données à
l'intérieur. De plus, vous pouvez correctement utiliser l'opérateur `+` ou la
macro `format!` pour concatener des valeurs `String`.

<!--
#### Appending to a String with `push_str` and `push`
-->

#### Ajouter du texte à une chaîne avec `push_str` et `push`

<!--
We can grow a `String` by using the `push_str` method to append a string slice,
as shown in Listing 8-15.
-->

Nous pouvons agrandir un `String` en utilisant la méthode `push_str` pour
ajouter une slice de chaîne de caractères, comme dans l'encart 8-15.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-15/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-15/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-15: Appending a string slice to a `String`
using the `push_str` method</span>
-->

<span class="caption">Encart 8-15 : Ajout d'une slice de chaîne de caractères
dans un `String` en utilisant la méthode `push_str`</span>

<!--
After these two lines, `s` will contain `foobar`. The `push_str` method takes a
string slice because we don’t necessarily want to take ownership of the
parameter. For example, the code in Listing 8-16 shows that it would be
unfortunate if we weren’t able to use `s2` after appending its contents to `s1`.
-->

A l'issue de ces deux lignes, `s` va contenir `foobar`. La méthode `push_str`
prend une slice de chaîne de caractères car nous ne souhaitons pas forcément
prendre possession du paramètre. Par exemple, le code de l'encart 8-16 nous
montre une situation où il serait regrettable si nous ne pouvions plus utiliser
`s2` après avoir ajouté son contenu dans `s1`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-16/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-16/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-16: Using a string slice after appending its
contents to a `String`</span>
-->

<span class="caption">Encart 8-16 : Utilisation d'une slice de chaîne de
caractères après avoir ajouté son contenu dans une `String`</span>

<!--
If the `push_str` method took ownership of `s2`, we wouldn’t be able to print
its value on the last line. However, this code works as we’d expect!
-->

Si la méthode `push_str` prenait possession de `s2`, à la dernière ligne, nous
ne pourrions pas afficher sa valeur. Cependant, ce code fonctionne comme nous
l'espérions !

<!--
The `push` method takes a single character as a parameter and adds it to the
`String`. Listing 8-17 shows code that adds the letter *l* to a `String` using
the `push` method.
-->

La méthode `push` prend un seul caractère en paramètre et l'ajoute au `String`.
L'encart 8-17 nous montre du code qui ajoute la lettre *l* au `String` en
utilisant la méthode `push`.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-17/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-17/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-17: Adding one character to a `String` value
using `push`</span>
-->

<span class="caption">Encart 8-17 : Ajout d'un unique caractère à la valeur d'une
`String` en utilisant `push`</span>

<!--
As a result of this code, `s` will contain `lol`.
-->

Après exécution de ce code, `s` contiendra `lol`.

<!--
#### Concatenation with the `+` Operator or the `format!` Macro
-->

#### Concaténation avec l'opérateur `+` ou la macro `format!`

<!--
Often, you’ll want to combine two existing strings. One way is to use the `+`
operator, as shown in Listing 8-18.
-->

Parfois, vous aurez besoin de combiner deux chaînes de caractères existantes.
Une façon de faire cela est d'utiliser l'opérateur `+`, comme dans l'encart
8-18.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-18/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-18: Using the `+` operator to combine two
`String` values into a new `String` value</span>
-->

<span class="caption">Encart 8-18 : Utilisation de l'opérateur `+` pour
combiner deux valeurs de `String`</span>

<!--
The string `s3` will contain `Hello, world!` as a result of this code. The
reason `s1` is no longer valid after the addition and the reason we used a
reference to `s2` has to do with the signature of the method that gets called
when we use the `+` operator. The `+` operator uses the `add` method, whose
signature looks something like this:
-->

La chaîne de caractères `s3` va contenir `Hello, world!` à l'issue de
l'exécution de ce code. La raison pour laquelle `s1` n'est plus utilisable après
avoir été ajouté et la raison pour laquelle nous utilisons une référence vers
`s2` s'expliquent par la signature de la méthode qui est appelée lorsque nous
utilisons l'opérateur `+`. L'opérateur `+` utilise la méthode `add`, dont la
signature ressemble à ceci :

<!--
```rust,ignore
fn add(self, s: &str) -> String {
```
-->

```rust,ignore
fn add(self, s: &str) -> String {
```

<!--
This isn’t the exact signature that’s in the standard library: in the standard
library, `add` is defined using generics. Here, we’re looking at the signature
of `add` with concrete types substituted for the generic ones, which is what
happens when we call this method with `String` values. We’ll discuss generics
in Chapter 10. This signature gives us the clues we need to understand the
tricky bits of the `+` operator.
-->

Ce n'est pas exactement la même signature que celle de la bibliothèque
standard : dans la bibliothèque standard, `add` est défini avec des types
génériques. Ici, nous voyons la signature de `add` avec des types concrets à la
place des génériques, ce qui se passe lorsque nous utilisons cette méthode avec
des valeurs de type `String`. Nous verrons les génériques au chapitre 10. Cette
signature nous donne les éléments dont nous avons besoin pour comprendre les
subtilités de l'opérateur `+`.

<!--
First, `s2` has an `&`, meaning that we’re adding a *reference* of the second
string to the first string because of the `s` parameter in the `add` function:
we can only add a `&str` to a `String`; we can’t add two `String` values
together. But wait—the type of `&s2` is `&String`, not `&str`, as specified in
the second parameter to `add`. So why does Listing 8-18 compile?
-->

Premièrement, `s2` a un `&`, ce qui veut dire que nous ajoutons une *référence*
vers la seconde chaîne de caractères en raison du paramètre `s` dans la fonction
`add` : nous pouvons seulement ajouter une `&str` à une `String` ; nous ne
pouvons pas ajouter deux valeurs de type `String` ensemble. Mais attendez — le
type de `&s2` est `&String`, et non pas `&str`, comme c'est écrit dans le second
paramètre de `add`. Donc pourquoi est-ce que le code l'encart 8-18 se compile ?

<!--
The reason we’re able to use `&s2` in the call to `add` is that the compiler
can *coerce* the `&String` argument into a `&str`. When we call the `add`
method, Rust uses a *deref coercion*, which here turns `&s2` into `&s2[..]`.
We’ll discuss deref coercion in more depth in Chapter 15. Because `add` does
not take ownership of the `s` parameter, `s2` will still be a valid `String`
after this operation.
-->

La raison pour laquelle nous pouvons utiliser `&s2` dans l'appel à `add` est que
le compilateur peut *extrapoler* l'argument `&String` en un `&str`. Lorsque
nous appelons la méthode `add`, Rust va utiliser une *extrapolation de
déréférencement*, qui transforme ici `&s2` en `&s2[..]`. Nous verrons plus en
détails l'extrapolation de déréférencement au chapitre 15. Comme `add` ne prend
pas possession du paramètre `s`, `s2` sera toujours une `String` valide après
cette opération.

<!--
Second, we can see in the signature that `add` takes ownership of `self`,
because `self` does *not* have an `&`. This means `s1` in Listing 8-18 will be
moved into the `add` call and no longer be valid after that. So although `let
s3 = s1 + &s2;` looks like it will copy both strings and create a new one, this
statement actually takes ownership of `s1`, appends a copy of the contents of
`s2`, and then returns ownership of the result. In other words, it looks like
it’s making a lot of copies but isn’t; the implementation is more efficient
than copying.
-->

Ensuite, nous pouvons constater que la signature de `add` prend possession de
`self`, car `self` n'a *pas* de `&`. Cela signifie que `s1` dans l'encart 8-18
va être déplacé dans l'appel à `add` et ne sera plus en vigueur après cela. Donc
bien que `let s3 = s1 + &s2` semble copier les deux chaînes de caractères pour
en créer une nouvelle, cette instruction va en réalité prendre possession du
résultat. Autrement dit, cela semble faire beaucoup de copies mais en réalité
non ; son implémentation est plus efficace que la copie.

<!--
If we need to concatenate multiple strings, the behavior of the `+` operator
gets unwieldy:
-->

Si nous avons besoin de concaténer plusieurs chaînes de caractères, le
comportement de l'opérateur `+` devient difficile à utiliser :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-01-concat-multiple-strings/src/main.rs:here}}
```

<!--
At this point, `s` will be `tic-tac-toe`. With all of the `+` and `"`
characters, it’s difficult to see what’s going on. For more complicated string
combining, we can use the `format!` macro:
-->

Au final, `s` vaudra `tic-tac-toe`. Avec tous les caractères `+`et `"`, il est
difficile de visualiser ce qu'il se passe. Pour une combinaison de chaînes de
caractères plus complexes, nous pouvons utiliser la macro `format!` :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/no-listing-02-format/src/main.rs:here}}
```

<!--
This code also sets `s` to `tic-tac-toe`. The `format!` macro works in the same
way as `println!`, but instead of printing the output to the screen, it returns
a `String` with the contents. The version of the code using `format!` is much
easier to read and doesn’t take ownership of any of its parameters.
-->

Ce code assigne lui aussi à `s` la valeur `tic-tac-toe`. La macro `format!`
fonctionne de la même manière que `println!`, mais au lieu d'afficher son
résultat à l'écran, elle retourne un `String` avec son contenu. La version du
code qui utilise `format!` est plus facile à lire et ne prends pas possession de
ses paramètres.

<!--
### Indexing into Strings
-->

### L'indexation des `String`

<!--
In many other programming languages, accessing individual characters in a
string by referencing them by index is a valid and common operation. However,
if you try to access parts of a `String` using indexing syntax in Rust, you’ll
get an error. Consider the invalid code in Listing 8-19.
-->

Dans de nombreux autres langages de programmation, l'accès individuel aux
caractères d'une chaîne de caractères en utilisant leur indice est une
opération valide et courante. Cependant, si vous essayez d'accéder à des
éléments d'une `String` en utilisant la syntaxe des indices avec Rust, vous
allez avoir une erreur. Nous tentons cela dans le code invalide de l'encart
8-19.

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-19/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-19/src/main.rs:here}}
```

<!--
<span class="caption">Listing 8-19: Attempting to use indexing syntax with a
String</span>
-->

<span class="caption">Encart 8-19 : Essai d'utilisation de la syntaxe
d'indexation avec une `String`</span>

<!--
This code will result in the following error:
-->

Ce code va produire l'erreur suivante :

<!--
```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```
-->

```console
{{#include ../listings/ch08-common-collections/listing-08-19/output.txt}}
```

<!--
The error and the note tell the story: Rust strings don’t support indexing. But
why not? To answer that question, we need to discuss how Rust stores strings in
memory.
-->

L'erreur et la remarque nous expliquent le problème : les `String` de Rust
n'acceptent pas l'utilisation des indices. Mais pourquoi ? Pour répondre à
cette question, nous avons besoin de savoir comment Rust enregistre les chaînes
de caractères dans la mémoire.

<!--
#### Internal Representation
-->

#### Représentation du fonctionnement interne

<!--
A `String` is a wrapper over a `Vec<u8>`. Let’s look at some of our properly
encoded UTF-8 example strings from Listing 8-14. First, this one:
-->

Une `String` est une surcouche de `Vec<u8>`. Revenons sur certains exemples de
chaînes de caractères correctement encodées en UTF-8 que nous avions dans
l'encart 8-14. Premièrement, celle-ci :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:spanish}}
```

<!--
In this case, `len` will be 4, which means the vector storing the string “Hola”
is 4 bytes long. Each of these letters takes 1 byte when encoded in UTF-8. But
what about the following line? (Note that this string begins with the capital
Cyrillic letter Ze, not the Arabic number 3.)
-->

Dans ce cas-ci, `len` vaudra 4, ce qui veut dire que le vecteur qui stocke la
chaîne “Hola” a une taille de 4 octets. Chacune des lettres prends 1 octet
lorsqu'elles sont encodées en UTF-8. Mais que se passe-t-il à la ligne
suivante ? (Notez que cette chaîne de caractères commence avec la lettre
majuscule cyrillique Ze, et non pas le nombre arabe 3.)

<!--
```rust
{{#rustdoc_include ../listings-sources/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```
-->

```rust
{{#rustdoc_include ../listings/ch08-common-collections/listing-08-14/src/main.rs:russian}}
```

<!--
Asked how long the string is, you might say 12. However, Rust’s answer is 24:
that’s the number of bytes it takes to encode “Здравствуйте” in UTF-8, because
each Unicode scalar value in that string takes 2 bytes of storage. Therefore,
an index into the string’s bytes will not always correlate to a valid Unicode
scalar value. To demonstrate, consider this invalid Rust code:
-->

Si vous vous demandez la longueur de la chaîne de caractères, vous vous direz
probablement 12. Cependant, la réponse de Rust sera 24 : c'est le nombre
d'octets nécessaires pour encoder “Здравствуйте” en UTF-8, car chaque valeur
scalaire Unicode dans cette chaîne de caractères prend 2 octets en mémoire.
Par conséquent, un indice dans les octets de la chaîne de caractères ne
correspondra pas forcément à une valeur unicode valide. Pour démontrer cela,
utilisons ce code Rust invalide :

<!--
```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```
-->

```rust,ignore,does_not_compile
let hello = "Здравствуйте";
let answer = &hello[0];
```

<!--
What should the value of `answer` be? Should it be `З`, the first letter? When
encoded in UTF-8, the first byte of `З` is `208` and the second is `151`, so
`answer` should in fact be `208`, but `208` is not a valid character on its
own. Returning `208` is likely not what a user would want if they asked for the
first letter of this string; however, that’s the only data that Rust has at
byte index 0. Users generally don’t want the byte value returned, even if the
string contains only Latin letters: if `&"hello"[0]` were valid code that
returned the byte value, it would return `104`, not `h`. To avoid returning an
unexpected value and causing bugs that might not be discovered immediately,
Rust doesn’t compile this code at all and prevents misunderstandings early in
the development process.
-->

Quelle sera la valeur de `answer` ? Est-ce que ce sera `З`, la première
lettre ?  Lorsqu'il est encodé en UTF-8, le premier octet de `З` est `208` et
le second est `151`, donc en vérité `answer` vaudra `208`, mais `208` n'est pas
un caractère valide à lui seul. Renvoyer `208` n'est pas ce qu'un utilisateur
attend s'il demande la première lettre de cette chaîne de caractères ;
cependant, c'est la seule valeur que Rust a à l'indice 0 des octets. Les
utilisateurs ne souhaitent généralement pas obtenir la valeur en octets, même si
la chaîne de caractères contient uniquement des lettres latines : si
`&"hello"[0]` était un code valide qui retournerait la valeur de l'octet, il
retournerait `104` et non pas `h`. Pour éviter de retourner une valeur
inattendue et générer des bogues qui ne seraient pas découverts immédiatement,
Rust ne va pas compiler ce code et ainsi éviter des erreurs dès le début du
processus de développement.

<!--
#### Bytes and Scalar Values and Grapheme Clusters! Oh My!
-->

#### Les valeurs des octets et des scalaires et les groupes de graphèmes !

<!--
Another point about UTF-8 is that there are actually three relevant ways to
look at strings from Rust’s perspective: as bytes, scalar values, and grapheme
clusters (the closest thing to what we would call *letters*).
-->

Un autre problème avec l'UTF-8 est qu'il a en fait trois manières pertinentes
pour considérer les chaînes de caractères avec Rust : comme des octets, comme
des valeurs scalaires et comme des groupes de graphèmes (la chose qui se
rapproche le plus de ce que nous pourrions appeler des *lettres*).

<!--
If we look at the Hindi word “नमस्ते” written in the Devanagari script, it is
stored as a vector of `u8` values that looks like this:
-->

Si l'on considère le mot en Hindi “नमस्ते” écrit dans le style Devanagari, il est
stocké comme un vecteur de valeurs `u8` qui sont les suivants :

<!--
```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```
-->

```text
[224, 164, 168, 224, 164, 174, 224, 164, 184, 224, 165, 141, 224, 164, 164,
224, 165, 135]
```

<!--
That’s 18 bytes and is how computers ultimately store this data. If we look at
them as Unicode scalar values, which are what Rust’s `char` type is, those
bytes look like this:
-->

Cela fait 18 octets et c'est ainsi que les ordinateurs stockeront cette donnée.
Si nous les voyons comme des valeurs scalaires Unicode, ce qu'est le type `char`
de Rust, ces octets seront les suivants :

<!--
```text
['न', 'म', 'स', '्', 'त', 'े']
```
-->

```text
['न', 'म', 'स', '्', 'त', 'े']
```

<!--
There are six `char` values here, but the fourth and sixth are not letters:
they’re diacritics that don’t make sense on their own. Finally, if we look at
them as grapheme clusters, we’d get what a person would call the four letters
that make up the Hindi word:
-->

Nous avons six valeurs `char` ici, mais la quatrième et sixième valeur ne sont
pas des lettres : ce sont des signes diacritiques qui n'ont pas de sens employés seuls.
Enfin, si nous les voyons comme des groupes de graphèmes, on obtient ce qu'une
personne pourrait appeler les quatre lettres qui constituent le mot Hindi :

<!--
```text
["न", "म", "स्", "ते"]
```
-->

```text
["न", "म", "स्", "ते"]
```

<!--
Rust provides different ways of interpreting the raw string data that computers
store so that each program can choose the interpretation it needs, no matter
what human language the data is in.
-->

Rust fournit différentes manières d'interpréter les données brutes des chaînes
de caractères que les ordinateurs stockent afin que chaque programme puisse
choisir l'interprétation dont il a besoin, peu importe dans quel langage humain
sont les données.

<!--
A final reason Rust doesn’t allow us to index into a `String` to get a
character is that indexing operations are expected to always take constant time
(O(1)). But it isn’t possible to guarantee that performance with a `String`,
because Rust would have to walk through the contents from the beginning to the
index to determine how many valid characters there were.
-->

Une dernière raison pour laquelle Rust ne nous autorise pas à indexer une
`String` comme caractère est que les opérations d'indexation sont censés prendre
un temps constant (O(1)). Mais il n'est pas possible de garantir cette
performance avec une `String`, car Rust devrait parcourir le contenu depuis le
début jusqu'à l'indice pour déterminer combien il y a de caractères valides.

<!--
### Slicing Strings
-->

### Les slices de chaînes de caractères

<!--
Indexing into a string is often a bad idea because it’s not clear what the
return type of the string-indexing operation should be: a byte value, a
character, a grapheme cluster, or a string slice. Therefore, Rust asks you to
be more specific if you really need to use indices to create string slices. To
be more specific in your indexing and indicate that you want a string slice,
rather than indexing using `[]` with a single number, you can use `[]` with a
range to create a string slice containing particular bytes:
-->

L'utilisation des indices sur une chaîne de caractères est souvent une mauvaise
idée car le type de retour de l'opération n'est pas toujours évident : une
valeur en octets, en caractères, un groupe de graphèmes, ou une slice de chaîne
de caractères. C'est pourquoi Rust vous demande d'être plus précis si vous avez
vraiment besoin d'utiliser des indices pour créer un déoupage de chaîne de
caractères. Afin d'être plus précis sur l'utilisation des indices et que vous
souhaitez obtenir une slice de chaine de caractères, vous pouvez utiliser `[]`
avec une intervalle d'indices pour créer une slice de chaîne de caractères
contenant des octets bien précis, plutôt que d'utiliser `[]` avec un seul
nombre :

<!--
```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```
-->

```rust
let hello = "Здравствуйте";

let s = &hello[0..4];
```

<!--
Here, `s` will be a `&str` that contains the first 4 bytes of the string.
Earlier, we mentioned that each of these characters was 2 bytes, which means
`s` will be `Зд`.
-->

Ici, `s` sera un `&str` qui contiendra les 4 premiers octets de la chaine de
caractères. Précédemment, nous avions mentionné que chacun de ces caractères
étaient encodés sur 2 octets, ce qui veut dire que `s` vaudra `Зд`.

<!--
What would happen if we used `&hello[0..1]`? The answer: Rust would panic at
runtime in the same way as if an invalid index were accessed in a vector:
-->

Que se serait-il passé si nous avions utilisé `&hello[0..1]` ? Réponse : Rust
aurait paniqué au moment de l'exécution de la même façon que si nous
utiliserions un indice invalide pour accéder à un élément d'un vecteur :

<!--
```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```
-->

```console
{{#include ../listings/ch08-common-collections/output-only-01-not-char-boundary/output.txt}}
```

<!--
You should use ranges to create string slices with caution, because doing so
can crash your program.
-->

Vous devriez utiliser les intervalles pour créer des slices avec prudence, car
cela peut provoquer un plantage de votre programme.

<!--
### Methods for Iterating Over Strings
-->

### Les méthodes pour parcourir les chaînes de caractères

<!--
Fortunately, you can access elements in a string in other ways.
-->

Heureusement, il existe d'autres manières d'accéder aux éléments d'une chaine
de caractères.

<!--
If you need to perform operations on individual Unicode scalar values, the best
way to do so is to use the `chars` method. Calling `chars` on “नमस्ते” separates
out and returns six values of type `char`, and you can iterate over the result
to access each element:
-->

Si vous avez besoin de faire des opérations sur les valeurs scalaires Unicodes
une par une, la meilleure façon de procéder est d'utiliser la méthode `chars`.
Utiliser `chars` sur “नमस्ते” sépare et retourne six valeurs de type `char`, et
vous pouvez itérer sur le résultat pour accéder sur chaque élément :

<!--
```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```
-->

```rust
for c in "नमस्ते".chars() {
    println!("{}", c);
}
```

<!--
This code will print the following:
-->

Ce code va afficher ceci :

<!--
```text
न
म
स
्
त
े
```
-->

```text
न
म
स
्
त
े
```

<!--
The `bytes` method returns each raw byte, which might be appropriate for your
domain:
-->

La méthode `bytes` va retourner chaque octet brut, ce qui sera peut-être plus
utile selon ce que vous voulez faire :

<!--
```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```
-->

```rust
for b in "नमस्ते".bytes() {
    println!("{}", b);
}
```

<!--
This code will print the 18 bytes that make up this `String`:
-->

Ce code va imprimer les 18 octets qui constituent cette `String` :

<!--
```text
224
164
// --snip--
165
135
```
-->

```text
224
164
// -- éléments masqués ici --
165
135
```

<!--
But be sure to remember that valid Unicode scalar values may be made up of more
than 1 byte.
-->

Rappelez-vous bien que des valeurs scalaires Unicode peuvent être constituées de
plus d'un octet.

<!--
Getting grapheme clusters from strings is complex, so this functionality is not
provided by the standard library. Crates are available on
[crates.io](https://crates.io/) if this is the functionality you need.
-->

L'obtention des groupes de graphèmes à partir des chaines de caractères est
complexe, donc cette fonctionnalité n'est pas fournie par la bibliothèque
standard. Des crates sont disponibles sur [crates.io](https://crates.io/) si
c'est la fonctionnalité dont vous avez besoin.

<!--
### Strings Are Not So Simple
-->

### Les chaînes de caractères ne sont pas si simples

<!--
To summarize, strings are complicated. Different programming languages make
different choices about how to present this complexity to the programmer. Rust
has chosen to make the correct handling of `String` data the default behavior
for all Rust programs, which means programmers have to put more thought into
handling UTF-8 data upfront. This trade-off exposes more of the complexity of
strings than is apparent in other programming languages, but it prevents you
from having to handle errors involving non-ASCII characters later in your
development life cycle.
-->

Pour résumer, les chaînes de caractères sont complexes. De nombreux langages
de programmation ont fait différents choix sur la façon de présenter cette
complexité aux développeurs. Rust a choisi d'appliquer par défaut la gestion
rigoureuse des données de `String` pour tous les programmes Rust, ce qui veut
dire que les développeurs doivent réfléchir davantage à la gestion des données
UTF-8. Ce compromis révèle davantage la complexité des chaînes de caractères par
rapport à ce que les autres langages de programmation laissent paraître, mais vous
évite d'avoir à gérer plus tard dans votre cycle de développement des erreurs à
cause de caractères non ASCII.

<!--
Let’s switch to something a bit less complex: hash maps!
-->

Changeons maintenant pour quelque chose de moins complexe : les tables de
hachage !
