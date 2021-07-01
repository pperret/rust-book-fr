> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/109).

<!--
## Generic Data Types
-->

## Les types de données génériques

<!--
We can use generics to create definitions for items like function signatures or
structs, which we can then use with many different concrete data types. Let’s
first look at how to define functions, structs, enums, and methods using
generics. Then we’ll discuss how generics affect code performance.
-->

Nous pouvons utiliser la généricité pour créer des définitions pour des éléments
comme les signatures de fonctions ou les structures, que nous pouvons ensuite
utiliser sur de nombreux types de données concrets. Commençons par regarder
comment définir des fonctions, des structures, des énumérations, et des méthodes
en utilisant la généricité. Ensuite nous verrons comment la généricité impacte
la performance du code.

<!--
### In Function Definitions
-->

### Dans la définition d'une fonction

<!--
When defining a function that uses generics, we place the generics in the
signature of the function where we would usually specify the data types of the
parameters and return value. Doing so makes our code more flexible and provides
more functionality to callers of our function while preventing code duplication.
-->

Lorsque nous définissons une fonction en utilisant la généricité, nous utilisons
des types génériques dans la signature de la fonction là où nous précisons
habituellement
les types de données des paramètres et de la valeur de retour. Faire ainsi rend
notre code plus flexible et apporte plus de fonctionnalités au code appelant
notre fonction, tout en évitant la duplication de code.

<!--
Continuing with our `largest` function, Listing 10-4 shows two functions that
both find the largest value in a slice.
-->

Pour continuer avec notre fonction `le_plus_grand`, l'encart 10-4 nous montre
deux fonctions qui trouvent toutes les deux la valeur la plus grande dans une
slice.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-04/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-4: Two functions that differ only in their
names and the types in their signatures</span>
-->

<span class="caption">Encart 10-4 : deux fonctions qui se distinguent seulement
par leur nom et le type dans leur signature</span>

<!--
The `largest_i32` function is the one we extracted in Listing 10-3 that finds
the largest `i32` in a slice. The `largest_char` function finds the largest
`char` in a slice. The function bodies have the same code, so let’s eliminate
the duplication by introducing a generic type parameter in a single function.
-->

La fonction `le_plus_grand_i32` est celle que nous avons construite à l'encart 10-3
lorsqu'elle trouvait le plus grand `i32` dans une slice. La fonction
`le_plus_grand_caractere` recherche le plus grand `char` dans une slice. Les
corps des fonctions ont le même code, donc essayons d'éviter cette duplication
en utilisant un paramètre de type générique dans une seule et unique fonction.

<!--
To parameterize the types in the new function we’ll define, we need to name the
type parameter, just as we do for the value parameters to a function. You can
use any identifier as a type parameter name. But we’ll use `T` because, by
convention, parameter names in Rust are short, often just a letter, and Rust’s
type-naming convention is CamelCase. Short for “type,” `T` is the default
choice of most Rust programmers.
-->

Pour paramétrer les types dans la nouvelle fonction que nous allons définir,
nous avons besoin de donner un nom au paramètre de type, comme nous l'avons
fait pour les paramètres de valeur des fonctions. Vous pouvez utiliser
n'importe quel identificateur pour nommer le paramètre de type. Mais ici nous allons
utiliser `T` car, par convention, les noms de paramètres en Rust sont courts,
souvent même une seule lettre, et la convention de nommage des types en Rust est
d'utiliser le CamelCase. Et puisque la version courte de “type” est `T`, c'est
le choix par défaut de nombreux développeurs Rust.

<!--
When we use a parameter in the body of the function, we have to declare the
parameter name in the signature so the compiler knows what that name means.
Similarly, when we use a type parameter name in a function signature, we have
to declare the type parameter name before we use it. To define the generic
`largest` function, place type name declarations inside angle brackets, `<>`,
between the name of the function and the parameter list, like this:
-->

Lorsqu'on utilise un paramètre dans le corps de la fonction, nous devons
déclarer le nom du paramètre dans la signature afin que le compilateur puisse
savoir à quoi réfère ce nom. De la même manière, lorsqu'on utilise un nom de
paramètre de type dans la signature d'une fonction, nous devons déclarer le nom
du paramètre de type avant de pouvoir l'utiliser. Pour déclarer la fonction
générique `le_plus_grand`, il faut placer la déclaration du nom du type entre
des chevrons `<>`, le tout entre le nom de la fonction et la liste des
paramètres, comme ceci :

<!--
```rust,ignore
fn largest<T>(list: &[T]) -> T {
```
-->

```rust,ignore
fn le_plus_grand<T>(liste: &[T]) -> &T {
```

<!--
We read this definition as: the function `largest` is generic over some type
`T`. This function has one parameter named `list`, which is a slice of values
of type `T`. The `largest` function will return a value of the
same type `T`.
-->

Cette définition se lit comme ceci : la fonction `le_plus_grand` est générique
en fonction du type `T`. Cette fonction a un paramètre qui s'appelle `liste`,
qui est une slice de valeurs de type `T`. Cette fonction `le_plus_grand` va
retourner une référence vers la valeur du même type `T`.

<!--
Listing 10-5 shows the combined `largest` function definition using the generic
data type in its signature. The listing also shows how we can call the function
with either a slice of `i32` values or `char` values. Note that this code won’t
compile yet, but we’ll fix it later in this chapter.
-->

L'encart 10-5 nous montre la combinaison de la définition de la fonction
`le_plus_grand` avec le type de données générique présent dans sa signature.
L'encart montre aussi que nous pouvons appeler la fonction avec une slice soit
de valeurs `i32`, soit de valeurs `char`. Notez que ce code ne se compile pas
encore, mais nous allons y remédier plus tard dans ce chapitre.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/src/main.rs}}
```

<!--
<span class="caption">Listing 10-5: A definition of the `largest` function that
uses generic type parameters but doesn’t compile yet</span>
-->

<span class="caption">Encart 10-5 : une définition de la fonction
`le_plus_grand` qui utilise des paramètres de type génériques, mais qui ne
compile pas encore</span>

<!--
If we compile this code right now, we’ll get this error:
-->

Si nous essayons de compiler ce code dès maintenant, nous aurons l'erreur
suivante :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-05/output.txt}}
```

<!--
The note mentions `std::cmp::PartialOrd`, which is a *trait*. We’ll talk about
traits in the next section. For now, this error states that the body of
`largest` won’t work for all possible types that `T` could be. Because we want
to compare values of type `T` in the body, we can only use types whose values
can be ordered. To enable comparisons, the standard library has the
`std::cmp::PartialOrd` trait that you can implement on types (see Appendix C
for more on this trait). You’ll learn how to specify that a generic type has a
particular trait in the [“Traits as Parameters”][traits-as-parameters]<!--
ignore -- > section, but let’s first explore other ways of using generic type
parameters.
-->

La note cite `std::cmp::PartialOrd`, qui est un *trait*. Nous allons voir les
traits dans la prochaine section. Pour le moment, cette erreur nous informe que
le corps de `le_plus_grand` ne va pas fonctionner pour tous les types possibles
que `T` peut représenter. Comme nous voulons comparer des valeurs de type `T`
dans le corps, nous pouvons utiliser uniquement des types dont les valeurs
peuvent être triées dans l'ordre. Pour effectuer des comparaisons, la bibliothèque
standard propose le trait `std::cmp::PartialOrd` que vous pouvez implémenter sur
des types (voir l'annexe C pour en savoir plus sur ce trait). Vous allez
apprendre à indiquer qu'un type générique a un trait spécifique dans la section
[“Des traits en paramètres”][traits-as-parameters]<!-- ignore -->, mais d'abord
nous allons explorer d'autres manières d'utiliser les paramètres de types
génériques.

<!--
### In Struct Definitions
-->

### Dans la définition des structures

<!--
We can also define structs to use a generic type parameter in one or more
fields using the `<>` syntax. Listing 10-6 shows how to define a `Point<T>`
struct to hold `x` and `y` coordinate values of any type.
-->

Nous pouvons aussi définir des structures en utilisant des paramètres de type
génériques dans un ou plusieurs champs en utilisant la syntaxe `<>`. L'encart
10-6 nous montre comment définir une structure `Point<T>` pour stocker des
valeurs de coordonnées `x` et `y` de n'importe quel type.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-06/src/main.rs}}
```

<!--
<span class="caption">Listing 10-6: A `Point<T>` struct that holds `x` and `y`
values of type `T`</span>
-->

<span class="caption">Encart 10-6 : une structure `Point<T>` qui stocke les
valeurs `x` et `y` de type `T`</span>

<!--
The syntax for using generics in struct definitions is similar to that used in
function definitions. First, we declare the name of the type parameter inside
angle brackets just after the name of the struct. Then we can use the generic
type in the struct definition where we would otherwise specify concrete data
types.
-->

La syntaxe pour l'utilisation des génériques dans les définitions de structures
est similaire à celle utilisée dans les définitions de fonctions. D'abord, on
déclare le nom du paramètre de type entre des chevrons juste après le nom de la
structure. Ensuite, on peut utiliser le type générique dans la définition de la
structure là où on indiquerait en temps normal des types de données concrets.

<!--
Note that because we’ve used only one generic type to define `Point<T>`, this
definition says that the `Point<T>` struct is generic over some type `T`, and
the fields `x` and `y` are *both* that same type, whatever that type may be. If
we create an instance of a `Point<T>` that has values of different types, as in
Listing 10-7, our code won’t compile.
-->

Notez que comme nous n'avons utilisé qu'un seul type générique pour définir
`Point<T>`, cette définition dit que la structure `Point<T>` est générique en
fonction d'un type `T`, et les champs `x` et `y` sont *tous les deux* de ce même
type, quel qu'il soit. Si nous créons une instance de `Point<T>` qui a des
valeurs de types différents, comme dans l'encart 10-7, notre code ne va pas se
compiler.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/src/main.rs}}
```

<!--
<span class="caption">Listing 10-7: The fields `x` and `y` must be the same
type because both have the same generic data type `T`.</span>
-->

<span class="caption">Encart 10-7 : les champs `x` et `y` doivent être du même
type car ils ont tous les deux le même type de données générique `T`.</span>

<!--
In this example, when we assign the integer value 5 to `x`, we let the
compiler know that the generic type `T` will be an integer for this instance of
`Point<T>`. Then when we specify 4.0 for `y`, which we’ve defined to have the
same type as `x`, we’ll get a type mismatch error like this:
-->

Dans cet exemple, lorsque nous assignons l'entier 5 à `x`, nous laissons
entendre au compilateur que le type générique `T` sera un entier pour cette
instance de `Point<T>`. Ensuite, lorsque nous assignons 4.0 à `y`, que nous avons
défini comme ayant le même type que `x`, nous obtenons une erreur d'incompatibilité de
type comme celle-ci :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-07/output.txt}}
```

<!--
To define a `Point` struct where `x` and `y` are both generics but could have
different types, we can use multiple generic type parameters. For example, in
Listing 10-8, we can change the definition of `Point` to be generic over types
`T` and `U` where `x` is of type `T` and `y` is of type `U`.
-->

Pour définir une structure `Point` où `x` et `y` sont tous les deux génériques
mais peuvent avoir des types différents, nous pouvons utiliser plusieurs 
paramètres de types génériques différents. Par exemple, dans l'encart 10-8, 
nous pouvons changer la définition de `Point` pour être générique en 
fonction des types `T` et `U` où `x` est de type `T` et `y` est de type `U`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-08/src/main.rs}}
```

<!--
<span class="caption">Listing 10-8: A `Point<T, U>` generic over two types so
that `x` and `y` can be values of different types</span>
-->

<span class="caption">Encart 10-8: un `Point<T, U>` générique en fonction de
deux types `x` et `y` qui peuvent être des valeurs de types différents</span>

<!--
Now all the instances of `Point` shown are allowed! You can use as many generic
type parameters in a definition as you want, but using more than a few makes
your code hard to read. When you need lots of generic types in your code, it
could indicate that your code needs restructuring into smaller pieces.
-->

Maintenant, toutes les instances de `Point` montrées ici sont valides ! Vous
pouvez utiliser autant de paramètres de type génériques que vous souhaitez dans
la déclaration de la définition, mais en utiliser plus de quelques-uns rend
votre code difficile à lire. Lorsque vous avez besoin de nombreux types
génériques dans votre code, cela peut être un signe que votre code a besoin
d'être remanié en éléments plus petits.

<!--
### In Enum Definitions
-->

### Dans les définitions d'énumérations

<!--
As we did with structs, we can define enums to hold generic data types in their
variants. Let’s take another look at the `Option<T>` enum that the standard
library provides, which we used in Chapter 6:
-->

Comme nous l'avons fait avec les structures, nous pouvons définir des
énumérations qui utilisent des types de données génériques dans leurs variantes.
Commençons par regarder à nouveau l'énumération `Option<T>` que fournit la
bibliothèque standard, et que nous avons utilisée au chapitre 6 :

<!--
```rust
enum Option<T> {
    Some(T),
    None,
}
```
-->

```rust
enum Option<T> {
    Some(T),
    None,
}
```

<!--
This definition should now make more sense to you. As you can see, `Option<T>`
is an enum that is generic over type `T` and has two variants: `Some`, which
holds one value of type `T`, and a `None` variant that doesn’t hold any value.
By using the `Option<T>` enum, we can express the abstract concept of having an
optional value, and because `Option<T>` is generic, we can use this abstraction
no matter what the type of the optional value is.
-->

Cette définition devrait désormais avoir plus de sens pour vous. Comme vous pouvez le
constater, `Option<T>` est une énumération qui est générique en fonction du type
`T` et a deux variantes : `Some`, qui contient une valeur de type `T`, et une
variante `None` qui ne contient aucune valeur. En utilisant l'énumération
`Option<T>`, nous pouvons exprimer le concept abstrait d'avoir une valeur
optionnelle, et comme `Option<T>` est générique, nous pouvons utiliser cette
abstraction peu importe le type de la valeur optionnelle.

<!--
Enums can use multiple generic types as well. The definition of the `Result`
enum that we used in Chapter 9 is one example:
-->

Les énumérations peuvent aussi utiliser plusieurs types génériques. La
définition de l'énumération `Result` que nous avons utilisée au chapitre 9 en est
un exemple :

<!--
```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```
-->

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!--
The `Result` enum is generic over two types, `T` and `E`, and has two variants:
`Ok`, which holds a value of type `T`, and `Err`, which holds a value of type
`E`. This definition makes it convenient to use the `Result` enum anywhere we
have an operation that might succeed (return a value of some type `T`) or fail
(return an error of some type `E`). In fact, this is what we used to open a
file in Listing 9-3, where `T` was filled in with the type `std::fs::File` when
the file was opened successfully and `E` was filled in with the type
`std::io::Error` when there were problems opening the file.
-->

L'énumération `Result` est générique en fonction de deux types, `T` et `E`, et a
deux variantes : `Ok`, qui contient une valeur de type `T`, et `Err`, qui
contient une valeur de type `E`. Cette définition rend possible l'utilisation de
l'énumération `Result` partout où nous avons une opération qui peut réussir (et
retourner une valeur du type `T`) ou échouer (et retourner une erreur du type
`E`). En fait, c'est ce qui est utilisé pour ouvrir un fichier dans l'encart
9-3, où `T` contenait un type `std::fs::File` lorsque le fichier était ouvert
avec succès et `E` contenait un type `std::io::Error` lorsqu'il y avait des
problèmes pour ouvrir le fichier.

<!--
When you recognize situations in your code with multiple struct or enum
definitions that differ only in the types of the values they hold, you can
avoid duplication by using generic types instead.
-->

Lorsque vous reconnaîtrez des cas dans votre code où vous aurez plusieurs
définitions de structures ou d'énumérations qui se distinguent uniquement par le
type de valeurs qu'elles stockent, vous pourrez éviter les doublons en utilisant
des types génériques à la place.

<!--
### In Method Definitions
-->

### Dans la définition des méthodes

<!--
We can implement methods on structs and enums (as we did in Chapter 5) and use
generic types in their definitions, too. Listing 10-9 shows the `Point<T>`
struct we defined in Listing 10-6 with a method named `x` implemented on it.
-->

Nous pouvons implémenter des méthodes sur des structures et des énumérations
(comme nous l'avons fait dans le chapitre 5) et aussi utiliser des types
génériques dans leurs définitions. L'encart 10-9 montre la structure `Point<T>`
que nous avons définie dans l'encart 10-6 avec une méthode qui s'appelle `x`
implémentée sur cette dernière.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-09/src/main.rs}}
```

<!--
<span class="caption">Listing 10-9: Implementing a method named `x` on the
`Point<T>` struct that will return a reference to the `x` field of type
`T`</span>
-->

<span class="caption">Encart 10-9 : implémentation d'une méthode `x` sur la
structure `Point<T>` qui va retourner une référence au champ `x`, de type `T`
</span>

<!--
Here, we’ve defined a method named `x` on `Point<T>` that returns a reference
to the data in the field `x`.
-->

Ici, nous avons défini une méthode qui s'appelle `x` sur `Point<T>` qui retourne
une référence à la donnée présente dans le champ `x`.

<!--
Note that we have to declare `T` just after `impl` so we can use it to specify
that we’re implementing methods on the type `Point<T>`.  By declaring `T` as a
generic type after `impl`, Rust can identify that the type in the angle
brackets in `Point` is a generic type rather than a concrete type.
-->

Notez que nous devons déclarer `T` juste après `impl` afin de pouvoir l'utiliser
pour préciser que nous implémentons des méthodes sur le type `Point<T>`. En
déclarant `T` comme un type générique après `impl`, Rust peut comprendre que le
type entre les chevrons dans `Point` est un type générique plutôt qu'un type
concret.

<!--
We could, for example, implement methods only on `Point<f32>` instances rather
than on `Point<T>` instances with any generic type. In Listing 10-10 we use the
concrete type `f32`, meaning we don’t declare any types after `impl`.
-->

Nous pouvons par exemple implémenter des méthodes uniquement sur des instances
de `Point<f32>` plutôt que sur des instances de n'importe quel type `Point<T>`.
Dans l'encart 10-10, nous utilisons le type concret `f32`, ce qui veut dire que
nous n'avons pas besoin de déclarer un type après `impl`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-10: An `impl` block that only applies to a
struct with a particular concrete type for the generic type parameter `T`</span>
-->

<span class="caption">Encart 10-10 : un bloc `impl` qui ne s'applique que sur
une structure d'un type concret particulier pour le paramètre de type générique
`T`
</span>

<!--
This code means the type `Point<f32>` will have a method named
`distance_from_origin` and other instances of `Point<T>` where `T` is not of
type `f32` will not have this method defined. The method measures how far our
point is from the point at coordinates (0.0, 0.0) and uses mathematical
operations that are available only for floating point types.
-->

Ce code signifie que le type `Point<f32>` va avoir une méthode qui s'appelle
`distance_depuis_lorigine` et les autres instances de `Point<T>` où `T` n'est
pas du type `f32` ne pourront pas appeler cette méthode. Cette méthode 
calcule la distance entre notre point et la coordonnée (0.0, 0.0) et utilise des
opérations mathématiques qui ne sont disponibles que pour les types de
flottants.

<!--
Generic type parameters in a struct definition aren’t always the same as those
you use in that struct’s method signatures. For example, Listing 10-11 defines
the method `mixup` on the `Point<T, U>` struct from Listing 10-8. The method
takes another `Point` as a parameter, which might have different types from the
`self` `Point` we’re calling `mixup` on. The method creates a new `Point`
instance with the `x` value from the `self` `Point` (of type `T`) and the `y`
value from the passed-in `Point` (of type `W`).
-->

Les paramètres de type génériques dans la définition d'une structure ne sont
pas toujours les mêmes que ceux qui sont utilisés dans la signature des
méthodes de cette structure. Par exemple, l'encart 10-11 définit la méthode
`melange` sur la structure `Point<T, U>` de l'encart 10-8. La méthode prend un
autre `Point` en paramètre, qui peut avoir des types différents du `Point`
`self` sur lequel nous appelons `melange`. La méthode crée une nouvelle
instance de `Point` avec la valeur de `x` provenant du `Point` `self` (de type
`T`) et la valeur de `y` provenant du `Point` en paramètre (de type `W`).

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-11/src/main.rs}}
```

<!--
<span class="caption">Listing 10-11: A method that uses different generic types
from its struct’s definition</span>
-->

<span class="caption">Encart 10-11 : une méthode qui utilise différents types
génériques provenant de la définition de la structure</span>

<!--
In `main`, we’ve defined a `Point` that has an `i32` for `x` (with value `5`)
and an `f64` for `y` (with value `10.4`). The `p2` variable is a `Point` struct
that has a string slice for `x` (with value `"Hello"`) and a `char` for `y`
(with value `c`). Calling `mixup` on `p1` with the argument `p2` gives us `p3`,
which will have an `i32` for `x`, because `x` came from `p1`. The `p3` variable
will have a `char` for `y`, because `y` came from `p2`. The `println!` macro
call will print `p3.x = 5, p3.y = c`.
-->

Dans le `main`, nous avons défini un `Point` qui a un `i32` pour `x` (avec la
valeur `5`) et un `f64` pour `y` (avec la valeur 10.4). La variable `p2` est une
structure `Point` qui a une slice de chaine de caractères pour `x` (avec la
valeur `"Hello"`) et un caractère `char` pour `y` (avec la valeur `c`). L'appel
à `melange` sur `p1` avec l'argument `p2` nous donne `p3`, qui aura un `i32` pour
`x`, car `x` provient de `p1`. La variable `p3` aura un caractère `char` pour
`y`, car `y` provient de `p2`. L'appel à la macro `println!` va afficher
`p3.x = 5, p3.y = c`.

<!--
The purpose of this example is to demonstrate a situation in which some generic
parameters are declared with `impl` and some are declared with the method
definition. Here, the generic parameters `T` and `U` are declared after `impl`,
because they go with the struct definition. The generic parameters `V` and `W`
are declared after `fn mixup`, because they’re only relevant to the method.
-->

Le but de cet exemple est de montrer une situation dans laquelle des paramètres
génériques sont déclarés avec `impl` et d'autres sont déclarés dans la
définition de la méthode. Ici, les paramètres génériques `T` et `U` sont
déclarés après `impl`, car ils sont liés à la définition de la structure. Les
paramètres génériques `V` et `W` sont déclarés après `fn melange`, car ils ne
sont liés qu'à cette méthode.

<!--
### Performance of Code Using Generics
-->

### Performance du code utilisant les génériques

<!--
You might be wondering whether there is a runtime cost when you’re using
generic type parameters. The good news is that Rust implements generics in such
a way that your code doesn’t run any slower using generic types than it would
with concrete types.
-->

Vous vous demandez peut-être s'il y a un coût à l'exécution lorsque vous
utilisez des paramètres de type génériques. La bonne nouvelle est que Rust
implémente les génériques de manière à ce que votre code ne s'exécute pas plus
lentement que vous utilisiez les types génériques ou des types concrets.

<!--
Rust accomplishes this by performing monomorphization of the code that is using
generics at compile time. *Monomorphization* is the process of turning generic
code into specific code by filling in the concrete types that are used when
compiled.
-->

Rust accomplit cela en pratiquant la monomorphisation à la compilation du code
qui utilise les génériques. La *monomorphisation* est un processus qui transforme
du code générique en code spécifique en définissant au moment de la compilation les
types concrets utilisés dans le code.

<!--
In this process, the compiler does the opposite of the steps we used to create
the generic function in Listing 10-5: the compiler looks at all the places
where generic code is called and generates code for the concrete types the
generic code is called with.
-->

Dans ce processus, le compilateur fait l'inverse des étapes que nous avons suivies
pour créer la fonction générique de l'encart 10-5 : le compilateur cherche tous
les endroits où le code générique est utilisé et génère du code pour les types
concrets avec lesquels le code générique est appelé.

<!--
Let’s look at how this works with an example that uses the standard library’s
`Option<T>` enum:
-->

Regardons comment cela fonctionne avec un exemple qui utilise l'énumération
`Option<T>` de la bibliothèque standard :

<!--
```rust
let integer = Some(5);
let float = Some(5.0);
```
-->

```rust
let entier = Some(5);
let flottant = Some(5.0);
```

<!--
When Rust compiles this code, it performs monomorphization. During that
process, the compiler reads the values that have been used in `Option<T>`
instances and identifies two kinds of `Option<T>`: one is `i32` and the other
is `f64`. As such, it expands the generic definition of `Option<T>` into
`Option_i32` and `Option_f64`, thereby replacing the generic definition with
the specific ones.
-->

Lorsque Rust compile ce code, il applique la monomorphisation. Pendant ce
processus, le compilateur lit les valeurs qui ont été utilisées dans les
instances de `Option<T>` et en déduit les deux sortes de `Option<T>` : une est
`i32` et l'autre est `f64`. Ainsi, il décompose la définition générique de
`Option<T>` en `Option_i32` et en `Option_f64`, remplaçant ainsi la définition
générique par deux définitions concrètes.

<!--
The monomorphized version of the code looks like the following. The generic
`Option<T>` is replaced with the specific definitions created by the compiler:
-->

La version monomorphe du code ressemble à ce qui suit. Le `Option<T>` générique
est remplacé par deux définitions concrètes créées par le compilateur :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let integer = Option_i32::Some(5);
    let float = Option_f64::Some(5.0);
}
```
-->

```rust
enum Option_i32 {
    Some(i32),
    None,
}

enum Option_f64 {
    Some(f64),
    None,
}

fn main() {
    let entier = Option_i32::Some(5);
    let flottant = Option_f64::Some(5.0);
}
```

<!--
Because Rust compiles generic code into code that specifies the type in each
instance, we pay no runtime cost for using generics. When the code runs, it
performs just as it would if we had duplicated each definition by hand. The
process of monomorphization makes Rust’s generics extremely efficient at
runtime.
-->

Comme Rust compile le code générique dans du code qui précise le type dans
chaque instance, l'utilisation des génériques n'a pas de conséquence sur les
performances de l'exécution. Quand le code s'exécute, il fonctionne comme il
devrait le faire si nous avions dupliqué chaque définition à la main. Le
processus de monomorphisation rend les génériques de Rust très performants au
moment de l'exécution.

<!--
[traits-as-parameters]: ch10-02-traits.html#traits-as-parameters
-->

[traits-as-parameters]: ch10-02-traits.html#des-traits-en-paramètres
