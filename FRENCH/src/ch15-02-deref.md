> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/189).

<!--
## Treating Smart Pointers Like Regular References with the `Deref` Trait
-->

## Considérer les pointeurs intelligents comme des références grâce au trait `Deref`

<!--
Implementing the `Deref` trait allows you to customize the behavior of the
*dereference operator*, `*` (as opposed to the multiplication or glob
operator). By implementing `Deref` in such a way that a smart pointer can be
treated like a regular reference, you can write code that operates on
references and use that code with smart pointers too.
-->

L'implémentation du trait `Deref` vous permet de personnaliser le comportement
de *l'opérateur de déréférencement* `*` (qui n'est pas l'opérateur de
multiplication ou le joker global). En implémentant `Deref` de manière à ce
qu'un pointeur intelligent puisse être considéré comme une référence classique,
vous pouvez écrire du code qui fonctionne avec des références mais aussi avec
des pointeurs intelligents.

<!--
Let’s first look at how the dereference operator works with regular references.
Then we’ll try to define a custom type that behaves like `Box<T>`, and see why
the dereference operator doesn’t work like a reference on our newly defined
type. We’ll explore how implementing the `Deref` trait makes it possible for
smart pointers to work in ways similar to references. Then we’ll look at
Rust’s *deref coercion* feature and how it lets us work with either references
or smart pointers.
-->

Regardons d'abord comment l'opérateur de déréférencement fonctionne avec des
références classiques. Ensuite nous essayerons de définir un type personnalisé
qui se comporte comme `Box<T>`, et voir pourquoi l'opérateur de déréférencement
ne fonctionne pas comme une référence sur notre type fraîchement défini. Nous
allons découvrir comment implémenter le trait `Deref` de manière à ce qu'il soit
possible que les pointeurs intelligents fonctionnent comme les références.
Ensuite nous verrons la fonctionnalité d'*extrapolation de déréférencement* de
Rust et comment elle nous permet de travailler à la fois avec des
références et des pointeurs intelligents.

<!--
> Note: there’s one big difference between the `MyBox<T>` type we’re about to
> build and the real `Box<T>`: our version will not store its data on the heap.
> We are focusing this example on `Deref`, so where the data is actually stored
> is less important than the pointer-like behavior.
-->

> Remarque : il y a une grosse différence entre le type `MaBoite<T>` que nous
> allons construire et la vraie `Box<T>` : notre version ne va pas stocker ses
> données sur le tas. Nous allons concentrer cet exemple sur `Deref`, donc
> l'endroit où est concrètement stocké la donnée est moins important que le
> comportement similaire aux pointeurs.

<!--
### Following the Pointer to the Value with the Dereference Operator
-->

### Suivre le pointeur vers la valeur grâce à l'opérateur de déréférencement

<!--
A regular reference is a type of pointer, and one way to think of a pointer is
as an arrow to a value stored somewhere else. In Listing 15-6, we create a
reference to an `i32` value and then use the dereference operator to follow the
reference to the data:
-->

Une référence classique est un type de pointeur, et une manière de modéliser un
pointeur est d'imaginer une flèche pointant vers une valeur stockée autre part.
Dans l'encart 15-6, nous créons une référence vers une valeur `i32` et utilisons
ensuite l'opérateur de déréférencement pour suivre la référence vers la donnée :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-06/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-06/src/main.rs}}
```

<!--
<span class="caption">Listing 15-6: Using the dereference operator to follow a
reference to an `i32` value</span>
-->

<span class="caption">Encart 15-6 : utiliser l'opérateur de déréférencement pour
suivre une référence vers une valeur `i32`</span>

<!--
The variable `x` holds an `i32` value, `5`. We set `y` equal to a reference to
`x`. We can assert that `x` is equal to `5`. However, if we want to make an
assertion about the value in `y`, we have to use `*y` to follow the reference
to the value it’s pointing to (hence *dereference*). Once we dereference `y`,
we have access to the integer value `y` is pointing to that we can compare with
`5`.
-->

La variable `x` stocke une valeur `i32` : `5`. Nous avons assigné à `y` une
référence vers `x`. Nous pouvons faire une `assert` pour vérifier que `x` est
égal à `5`. Cependant, si nous souhaitons faire une `assert` sur la valeur dans
`y`, nous devons utiliser `*y` pour suivre la référence vers la valeur sur
laquelle elle pointe (d'où le *déréférencement*). Une fois que nous avons
déréférencé `y`, nous avons accès à la valeur de l'entier sur laquelle `y`
pointe afin que nous puissions la comparer avec `5`.

<!--
If we tried to write `assert_eq!(5, y);` instead, we would get this compilation
error:
-->

Si nous avions essayé d'écrire `assert_eq!(5, y);` à la place, nous aurions
obtenu cette erreur de compilation :

<!--
```console
{{#include ../listings-sources/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/output-only-01-comparing-to-reference/output.txt}}
```

<!--
Comparing a number and a reference to a number isn’t allowed because they’re
different types. We must use the dereference operator to follow the reference
to the value it’s pointing to.
-->

Comparer un nombre et une référence vers un nombre n'est pas autorisé car ils
sont de types différents. Nous devons utiliser l'opérateur de déréférencement
pour suivre la référence vers la valeur sur laquelle elle pointe.

<!--
### Using `Box<T>` Like a Reference
-->

### Utiliser `Box<T>` comme étant une référence

<!--
We can rewrite the code in Listing 15-6 to use a `Box<T>` instead of a
reference; the dereference operator will work as shown in Listing 15-7:
-->

Nous pouvons réécrire le code l'encart 15-6 pour utiliser une `Box<T>` au lieu
d'une référence ; l'opérateur de déréférencement devrait fonctionner comme
montré dans l'encart 15-7 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-07/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-07/src/main.rs}}
```

<!--
<span class="caption">Listing 15-7: Using the dereference operator on a
`Box<i32>`</span>
-->

<span class="caption">Encart 15-7 : utilisation de l'opérateur de
déréférencement sur un `Box<i32>`</span>

<!--
The only difference between Listing 15-7 and Listing 15-6 is that here we set
`y` to be an instance of a box pointing to a copied value of `x` rather than a
reference pointing to the value of `x`. In the last assertion, we can use the
dereference operator to follow the box’s pointer in the same way that we did
when `y` was a reference. Next, we’ll explore what is special about `Box<T>`
that enables us to use the dereference operator by defining our own box type.
-->

La seule différence entre l'encart 15-7 et l'encart 15-6 est qu'ici nous avons
fait en sorte que `y` soit une instance de boite qui pointe sur une copie de la
valeur de `x` plutôt qu'avoir une référence vers la valeur de `x`. Dans la
dernière assertion, nous pouvons utiliser l'opérateur de déréférencement pour
suivre le pointeur de la boite de la même manière que nous l'avons fait lorsque
`y` était une référence. Maintenant, nous allons regarder ce qu'il y a de si
spécial dans `Box<T>` qui nous permet d'utiliser l'opérateur de déréférencement
en définissant notre propre type de boite.

<!--
### Defining Our Own Smart Pointer
-->

### Définir notre propre pointeur intelligent

<!--
Let’s build a smart pointer similar to the `Box<T>` type provided by the
standard library to experience how smart pointers behave differently from
references by default. Then we’ll look at how to add the ability to use the
dereference operator.
-->

Construisons un pointeur intelligent similaire au type `Box<T>` fourni par la
bibliothèque standard pour apprendre comment les pointeurs intelligents se
comportent différemment des références classiques. Ensuite nous regarderons
comment lui ajouter la possibilité d'utiliser l'opérateur de déréférencement.

<!--
The `Box<T>` type is ultimately defined as a tuple struct with one element, so
Listing 15-8 defines a `MyBox<T>` type in the same way. We’ll also define a
`new` function to match the `new` function defined on `Box<T>`.
-->

Le type `Box<T>` est essentiellement défini comme étant une structure de tuple
d'un seul élément, donc l'encart 15-8 définit un type `MaBoite<T>` de la même
manière. Nous allons aussi définir une fonction `new` pour correspondre à la
fonction `new` définie sur `Box<T>`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-08/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-8: Defining a `MyBox<T>` type</span>
-->

<span class="caption">Encart 15-8 : définition du type `MaBoite<T>`</span>

<!--
We define a struct named `MyBox` and declare a generic parameter `T`, because
we want our type to hold values of any type. The `MyBox` type is a tuple struct
with one element of type `T`. The `MyBox::new` function takes one parameter of
type `T` and returns a `MyBox` instance that holds the value passed in.
-->

Nous définissons une structure `MaBoite` et on déclare un paramètre générique
`T`, car nous souhaitons que notre type stocke des valeurs de n'importe quel
type. Le type `MaBoite` est une structure de tuple avec un seul élément de type
`T`. La fonction `MaBoite::new` prend un paramètre de type `T` et retourne une
instance `MaBoite` qui stocke la valeur qui lui est passée.

<!--
Let’s try adding the `main` function in Listing 15-7 to Listing 15-8 and
changing it to use the `MyBox<T>` type we’ve defined instead of `Box<T>`. The
code in Listing 15-9 won’t compile because Rust doesn’t know how to dereference
`MyBox`.
-->

Essayons d'ajouter la fonction `main` de l'encart 15-7 dans l'encart 15-8 et la
modifier pour utiliser le type `MaBoite<T>` que nous avons défini à la place de
`Box<T>`. Le code de l'encart 15-9 ne se compile pas car Rust ne sait pas
comment déréférencer `MaBoite`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-09/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-9: Attempting to use `MyBox<T>` in the same
way we used references and `Box<T>`</span>
-->

<span class="caption">Encart 15-9 : tentative d'utiliser `MaBoite<T>` de la même
manière que nous avions utilisé les références et `Box<T>`</span>

<!--
Here’s the resulting compilation error:
-->

Voici l'erreur de compilation qui en résulte :

<!--
```console
{{#include ../listings-sources/ch15-smart-pointers/listing-15-09/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-09/output.txt}}
```

<!--
Our `MyBox<T>` type can’t be dereferenced because we haven’t implemented that
ability on our type. To enable dereferencing with the `*` operator, we
implement the `Deref` trait.
-->

Notre type `MaBoite<T>` ne peut pas être déréférencée car nous n'avons pas
implémenté cette fonctionnalité sur notre type. Pour permettre le
déréférencement avec l'opérateur `*`, nous devons implémenter le trait `Deref`.

<!--
### Treating a Type Like a Reference by Implementing the `Deref` Trait
-->

### Considérer un type comme une référence en implémentant le trait `Deref`

<!--
As discussed in Chapter 10, to implement a trait, we need to provide
implementations for the trait’s required methods. The `Deref` trait, provided
by the standard library, requires us to implement one method named `deref` that
borrows `self` and returns a reference to the inner data. Listing 15-10
contains an implementation of `Deref` to add to the definition of `MyBox`:
-->

Comme nous l'avons vu dans le chapitre 10, pour implémenter un trait, nous
devons fournir les implémentations des méthodes nécessaires pour ce trait. Le
trait `Deref`, fourni par la bibliothèque standard, nécessite que nous
implémentions une méthode `deref` qui prend possession de `self` et retourne une
référence vers la donnée interne. L'encart 15-10 contient une implémentation de
`Deref` à ajouter à la définition de `MaBoite` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-10: Implementing `Deref` on `MyBox<T>`</span>
-->

<span class="caption">Encart 15-10 : implémentation de `Deref` sur `MaBoite<T>`
</span>

<!--
The `type Target = T;` syntax defines an associated type for the `Deref` trait
to use. Associated types are a slightly different way of declaring a generic
parameter, but you don’t need to worry about them for now; we’ll cover them in
more detail in Chapter 19.
-->

La syntaxe `type Target = T;` définit un type associé pour le trait `Deref` à
utiliser. Les types associés sont une manière légèrement différente de déclarer
un paramètre générique, mais vous n'avez pas à vous préoccuper d'eux pour le
moment ; nous les verrons plus en détail au chapitre 19.

<!--
We fill in the body of the `deref` method with `&self.0` so `deref` returns a
reference to the value we want to access with the `*` operator. The `main`
function in Listing 15-9 that calls `*` on the `MyBox<T>` value now compiles,
and the assertions pass!
-->

Nous renseignons le corps de la méthode `deref` avec `&self.0` afin que `deref`
retourne une référence vers la valeur que nous souhaitons accéder avec
l'opérateur `*`. La fonction `main` de l'encart 15-9 qui appelle `*` sur la
valeur `MaBoite<T>` se compile désormais, et le `assert` réussit aussi !

<!--
Without the `Deref` trait, the compiler can only dereference `&` references.
The `deref` method gives the compiler the ability to take a value of any type
that implements `Deref` and call the `deref` method to get a `&` reference that
it knows how to dereference.
-->

Sans le trait `Deref`, le compilateur peut seulement déréférencer des références
`&`. La méthode `deref` donne la possibilité au compilateur d'obtenir la valeur
de n'importe quel type qui implémente `Deref` en appelant la méthode `deref`
pour obtenir une référence `&` qu'il sait comment déréférencer.

<!--
When we entered `*y` in Listing 15-9, behind the scenes Rust actually ran this
code:
-->

Lorsque nous avons précisé `*y` dans l'encart 15-9, Rust fait tourner ce code en
coulisses :

<!--
```rust,ignore
*(y.deref())
```
-->

```rust,ignore
*(y.deref())
```

<!--
Rust substitutes the `*` operator with a call to the `deref` method and then a
plain dereference so we don’t have to think about whether or not we need to
call the `deref` method. This Rust feature lets us write code that functions
identically whether we have a regular reference or a type that implements
`Deref`.
-->

Rust remplace l'opérateur `*` par un appel à la méthode `deref` suivi par un
simple déréférencement afin que nous n'ayons pas à nous demander si nous devons
ou non appeler la méthode `deref`. Cette fonctionnalité de Rust nous permet
d'écrire du code qui fonctionne de manière identique que nous ayons une
référence classique ou un type qui implémente `Deref`.

<!--
The reason the `deref` method returns a reference to a value, and that the plain
dereference outside the parentheses in `*(y.deref())` is still necessary, is the
ownership system. If the `deref` method returned the value directly instead of
a reference to the value, the value would be moved out of `self`. We don’t want
to take ownership of the inner value inside `MyBox<T>` in this case or in most
cases where we use the dereference operator.
-->

La raison pour laquelle la méthode `deref` retourne une référence à une valeur,
et que le déréférencement du tout dans les parenthèses externes de
`*(y.deref())` reste nécessaire, est le système de possession. Si la méthode
`deref` retournait la valeur directement au lieu d'une référence à cette valeur,
la valeur serait *déplacée* à l'extérieur de `self`. Nous ne souhaitons pas
prendre possession de la valeur à l'intérieur de `MaBoite<T>` dans ce cas ainsi
que la plupart des cas où nous utilisons l'opérateur de déréférencement.

<!--
Note that the `*` operator is replaced with a call to the `deref` method and
then a call to the `*` operator just once, each time we use a `*` in our code.
Because the substitution of the `*` operator does not recurse infinitely, we
end up with data of type `i32`, which matches the `5` in `assert_eq!` in
Listing 15-9.
-->

Notez que l'opérateur `*` est remplacé par un appel à la méthode `deref` suivi
par un appel à l'opérateur `*` une seule fois, à chaque fois que nous utilisons
un `*` dans notre code. Comme la substitution de l'opérateur `*` ne s'effectue
pas de manière récursive et infinie, nous récupérerons une donnée de type `i32`,
qui correspond au `5` du `assert_eq!` de l'encart 15-9.

<!--
### Implicit Deref Coercions with Functions and Methods
-->

### Extrapolation de déréférencement implicite avec les fonctions et les méthodes

<!--
*Deref coercion* is a convenience that Rust performs on arguments to functions
and methods. Deref coercion works only on types that implement the `Deref`
trait. Deref coercion converts such a type into a reference to another type.
For example, deref coercion can convert `&String` to `&str` because `String`
implements the `Deref` trait such that it returns `&str`. Deref coercion happens
automatically when we pass a reference to a particular type’s value as an
argument to a function or method that doesn’t match the parameter type in the
function or method definition. A sequence of calls to the `deref` method
converts the type we provided into the type the parameter needs.
-->

L'*extrapolation de déréférencement* est une commodité que Rust applique sur les
arguments des fonctions et des méthodes. L'extrapolation de déréférencement
fonctionne uniquement avec un type qui implémente le trait `Deref`.
L'extrapolation de déréférencement convertit ce type en une référence vers un
autre type. Par exemple, l'extrapolation de déréférencement peut convertir
`&String` en `&str` car `String` implémente le trait `Deref` de sorte qu'il
puisse retourner `&str`. L'extrapolation de déréférencement s'applique
automatiquement lorsque nous passons une référence vers une valeur d'un type
particulier en argument d'une fonction ou d'une méthode qui ne correspond pas à
ce type de paramètre dans la définition de la fonction ou de la méthode. Une
série d'appels à la méthode `deref` convertit le type que nous donnons dans le
type que le paramètre nécessite.

<!--
Deref coercion was added to Rust so that programmers writing function and
method calls don’t need to add as many explicit references and dereferences
with `&` and `*`. The deref coercion feature also lets us write more code that
can work for either references or smart pointers.
-->

L'extrapolation de déréférencement a été ajouté à Rust afin de permettre aux
développeurs d'écrire des appels de fonctions et de méthodes qui n'ont pas
besoin d'indiquer explicitement les références et les déréférencements avec `&`
et `*`. La fonctionnalité d'extrapolation de déréférencement nous permet aussi
d'écrire plus de code qui peut fonctionner à la fois pour les références ou pour
les pointeurs intelligents.

<!--
To see deref coercion in action, let’s use the `MyBox<T>` type we defined in
Listing 15-8 as well as the implementation of `Deref` that we added in Listing
15-10. Listing 15-11 shows the definition of a function that has a string slice
parameter:
-->

Pour voir l'extrapolation de déréférencement en action, utilisons le type
`MaBoite<T>` que nous avons défini dans l'encart 15-8 ainsi que l'implémentation
de `Deref` que nous avons ajouté dans l'encart 15-10. L'encart 15-11 montre la
définition d'une fonction qui a un paramètre qui est une slice de chaîne de
caractères :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-11: A `hello` function that has the parameter
`name` of type `&str`</span>
-->

<span class="caption">Encart 15-11 : une fonction `saluer` qui prend en
paramètre `nom` du type `&str`</span>

<!--
We can call the `hello` function with a string slice as an argument, such as
`hello("Rust");` for example. Deref coercion makes it possible to call `hello`
with a reference to a value of type `MyBox<String>`, as shown in Listing 15-12:
-->

Nous pouvons appeler la fonction `saluer` avec une slice de chaîne de caractères
en argument, comme par exemple `saluer("Rust");`. L'extrapolation de
déréférencement rend possible l'appel de `saluer` avec une référence à une
valeur du type `MaBoite<String>`, comme dans l'encart 15-12 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-12/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-12: Calling `hello` with a reference to a
`MyBox<String>` value, which works because of deref coercion</span>
-->

<span class="caption">Encart 15-12 : appel à `saluer` avec une référence à une
valeur du type `MaBoite<String>`, qui fonctionne grâce à l'extrapolation de
déréférencement</span>

<!--
Here we’re calling the `hello` function with the argument `&m`, which is a
reference to a `MyBox<String>` value. Because we implemented the `Deref` trait
on `MyBox<T>` in Listing 15-10, Rust can turn `&MyBox<String>` into `&String`
by calling `deref`. The standard library provides an implementation of `Deref`
on `String` that returns a string slice, and this is in the API documentation
for `Deref`. Rust calls `deref` again to turn the `&String` into `&str`, which
matches the `hello` function’s definition.
-->

Ici nous appelons la fonction `saluer` avec l'argument `&m`, qui est une
référence vers une valeur de type `MaBoite<String>`. Comme nous avons implémenté
le trait `Deref` sur `MaBoite<T>` dans l'encart 15-10, Rust peut transformer le
`&MaBoite<String>` en `&String` en appelant `deref`. La bibliothèque standard
fournit une implémentation de `Deref` sur `String` qui retourne une slice de
chaîne de caractères, comme expliqué dans la documentation de l'API de `Deref`.
Rust appelle à nouveau `deref` pour transformer le `&String` en `&str`, qui
correspond à la définition de la fonction `saluer`.

<!--
If Rust didn’t implement deref coercion, we would have to write the code in
Listing 15-13 instead of the code in Listing 15-12 to call `hello` with a value
of type `&MyBox<String>`.
-->

Si Rust n'avait pas implémenté l'extrapolation de déréférencement, nous aurions
dû écrire le code de l'encart 15-13 au lieu du code de l'encart 15-12 pour
appeler `saluer` avec une valeur du type `&MaBoite<String>`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-13/src/main.rs:here}}
```

<!--
<span class="caption">Listing 15-13: The code we would have to write if Rust
didn’t have deref coercion</span>
-->

<span class="caption">Encart 15-13 : le code que nous aurions dû écrire si Rust
n'avait pas d'extrapolation de déréférencement</span>

<!--
The `(*m)` dereferences the `MyBox<String>` into a `String`. Then the `&` and
`[..]` take a string slice of the `String` that is equal to the whole string to
match the signature of `hello`. The code without deref coercions is harder to
read, write, and understand with all of these symbols involved. Deref coercion
allows Rust to handle these conversions for us automatically.
-->

Le `(*m)` déréférence la `MaBoite<String>` en une `String`. Ensuite le `&` et le
`[..]` créent une slice de chaîne de caractères à partir de la `String` qui est
égale à l'intégralité du contenu de la `String`, ceci afin de correspondre à la
signature de `saluer`. Le code sans l'extrapolation de déréférencement est bien
plus difficile à lire, écrire et comprendre avec la présence de tous ces
symboles. L'extrapolation de déréférencement permet à Rust d'automatiser ces
convertions pour nous.

<!--
When the `Deref` trait is defined for the types involved, Rust will analyze the
types and use `Deref::deref` as many times as necessary to get a reference to
match the parameter’s type. The number of times that `Deref::deref` needs to be
inserted is resolved at compile time, so there is no runtime penalty for taking
advantage of deref coercion!
-->

Lorsque le trait `Deref` est défini pour les types concernés, Rust va analyser
les types et utiliser `Deref::deref` autant de fois que nécessaire pour obtenir
une référence qui correspond au type du paramètre. Le nombre de fois qu'il est
nécessaire d'insérer `Deref::deref` est résolu au moment de la compilation,
ainsi il n'y a pas de surcoût au moment de l'exécution pour bénéficier de
l'extrapolation de déréférencement !

<!--
### How Deref Coercion Interacts with Mutability
-->

### L'interaction de l'extrapolation de déréférencement avec la mutabilité

<!--
Similar to how you use the `Deref` trait to override the `*` operator on
immutable references, you can use the `DerefMut` trait to override the `*`
operator on mutable references.
-->

De la même manière que vous pouvez utiliser le trait `Deref` pour remplacer le
comportement de l'opérateur `*` sur les références immuables, vous pouvez
utiliser le trait `DerefMut` pour remplacer le comportement de l'opérateur `*`
sur les références mutables.

<!--
Rust does deref coercion when it finds types and trait implementations in three
cases:
-->

Rust procède à l'extrapolation de déréférencement lorsqu'il trouve des types et
des implémentations de traits dans trois cas :

<!--
* From `&T` to `&U` when `T: Deref<Target=U>`
* From `&mut T` to `&mut U` when `T: DerefMut<Target=U>`
* From `&mut T` to `&U` when `T: Deref<Target=U>`
-->

* Passer de `&T` à `&U` lorsque `T: Deref<Target=U>`
* Passer de `&mut T` à `&mut U` lorsque `T: DerefMut<Target=U>`
* Passer de `&mut T` à `&U` lorsque `T: Deref<Target=U>`

<!--
The first two cases are the same except for mutability. The first case states
that if you have a `&T`, and `T` implements `Deref` to some type `U`, you can
get a `&U` transparently. The second case states that the same deref coercion
happens for mutable references.
-->

Les deux premiers cas sont exactement les mêmes, sauf pour la mutabilité. Le
premier cas signifie que si vous avez un `&T` et que `T` implémente `Deref` pour
le type `U`, vous pouvez obtenir un `&U` de manière transparente. Le second cas
signifie que la même extrapolation de déréférencement se déroule pour les
références mutables.

<!--
The third case is trickier: Rust will also coerce a mutable reference to an
immutable one. But the reverse is *not* possible: immutable references will
never coerce to mutable references. Because of the borrowing rules, if you have
a mutable reference, that mutable reference must be the only reference to that
data (otherwise, the program wouldn’t compile). Converting one mutable
reference to one immutable reference will never break the borrowing rules.
Converting an immutable reference to a mutable reference would require that the
initial immutable reference is the only immutable reference to that data, but
the borrowing rules don’t guarantee that. Therefore, Rust can’t make the
assumption that converting an immutable reference to a mutable reference is
possible.
-->

Le troisième cas est plus ardu : Rust va aussi procéder à une extrapolation de
déréférencement d'une référence mutable vers une référence immuable. Mais 
l'inverse n'est *pas* possible: une extrapolation de déréférencement d'une 
valeur immuable ne donnera jamais une référence mutable. A cause des règles
d'emprunt, si vous avez une référence mutable, cette référence mutable doit être 
la seule référence vers cette donnée (autrement, le programme ne peut pas être 
compilé). Convertir une référence mutable vers une référence immuable ne va
jamais casser les règles d'emprunt. Convertir une référence immuable vers une 
référence mutable nécessite que la référence immuable initiale soit la seule 
référence immuable vers cette donnée, mais les règles d'emprunt n'empêchent pas 
cela. Ainsi, Rust ne peut pas déduire que la conversion d'une référence immuable 
vers une référence mutable soit possible.
