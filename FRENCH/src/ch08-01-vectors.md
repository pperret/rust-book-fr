> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/83).

<!--
## Storing Lists of Values with Vectors
-->

## Stocker des listes de valeurs avec des vecteurs

<!--
The first collection type we’ll look at is `Vec<T>`, also known as a *vector*.
Vectors allow you to store more than one value in a single data structure that
puts all the values next to each other in memory. Vectors can only store values
of the same type. They are useful when you have a list of items, such as the
lines of text in a file or the prices of items in a shopping cart.
-->

Le premier type de collection que nous allons voir est `Vec<T>`, aussi appelé
*vecteur*. Les vecteurs vous permettent de stocker plus qu'une seule valeur dans
une seule structure de données qui stocke les valeurs proches l'une de l'autre
dans la mémoire. Les vecteurs peuvent stocker uniquement des valeurs du même
type. Ils sont utiles lorsque vous avez une liste d'éléments, tels que les
lignes de texte provenant d'un fichier ou les prix des articles d'un panier
d'achat.

<!--
### Creating a New Vector
-->

### Créer un nouveau vecteur

<!--
To create a new, empty vector, we can call the `Vec::new` function, as shown in
Listing 8-1.
-->

Pour créer un nouveau vecteur vide, nous pouvons appeler la fonction
`Vec::new`, comme dans l'encart 8-1.

```rust
let v: Vec<i32> = Vec::new();
```

<!--
<span class="caption">Listing 8-1: Creating a new, empty vector to hold values
of type `i32`</span>
-->

<span class="caption">Encart 8-1 : Créer un nouveau vecteur vide pour y stocker
des valeurs de type `i32`</span>

<!--
Note that we added a type annotation here. Because we aren’t inserting any
values into this vector, Rust doesn’t know what kind of elements we intend to
store. This is an important point. Vectors are implemented using generics;
we’ll cover how to use generics with your own types in Chapter 10. For now,
know that the `Vec<T>` type provided by the standard library can hold any type,
and when a specific vector holds a specific type, the type is specified within
angle brackets. In Listing 8-1, we’ve told Rust that the `Vec<T>` in `v` will
hold elements of the `i32` type.
-->

Remarquez que nous avons ajouté ici une annotation de type. Comme nous
n'ajoutons de valeurs dans ce vecteur, Rust ne sait pas quel type d'éléments
nous souhaitons stocker. C'est une information importante. Les vecteurs sont
implémentés avec la généricité, nous allons voir comment utiliser la généricité
sur vos propres types au chapitre 10. Pour l'instant, sachez que le type
`Vec<T>` qui est fourni par la bibliothèque standard peut stocker n'importe quel
type, et lorsqu'un vecteur précis stocke un type précis, ce type est renseigné
entre des chevrons. Dans l'encart 8-1, nous précisons à Rust que le `Vec<T>`
dans `v` va stocker des éléments de type `i32`.

<!--
In more realistic code, Rust can often infer the type of value you want to
store once you insert values, so you rarely need to do this type annotation.
It’s more common to create a `Vec<T>` that has initial values, and Rust
provides the `vec!` macro for convenience. The macro will create a new vector
that holds the values you give it. Listing 8-2 creates a new `Vec<i32>` that
holds the values `1`, `2`, and `3`.
-->

Dans du code plus réaliste, Rust peut parfois deviner le type de la valeur que
vous souhaitez stocker lorsque vous ajouterez des valeurs, donc vous n'aurez pas
souvent besoin de faire cette annotation de type. Il est plus fréquent de créer
un `Vec<T>` qui a des valeurs initiales, et Rust fournit la macro `vec!` par
commodité. La macro va créer un nouveau vecteur qui stockera les valeurs que
vous lui donnerez. L'encart 8-2 crée un nouveau `Vec<i32>` qui stocke les
valeurs `1`, `2` et `3`.

<!--
```rust
let v = vec![1, 2, 3];
```
-->

```rust
let v = vec![1, 2, 3];
```

<!--
<span class="caption">Listing 8-2: Creating a new vector containing
values</span>
-->

<span class="caption">Encart 8-2 : Création d'un nouveau vecteur qui contient
des valeurs</span>

<!--
Because we’ve given initial `i32` values, Rust can infer that the type of `v`
is `Vec<i32>`, and the type annotation isn’t necessary. Next, we’ll look at how
to modify a vector.
-->

Comme nous avons donné des valeurs initiales `i32`, Rust peut en déduire que le
type de `v` est `Vec<i32>`, et l'annotation de type n'est plus nécessaire.
Maintenant, nous allons voir comment modifier un vecteur.

<!--
### Updating a Vector
-->

### Modifier un vecteur

<!--
To create a vector and then add elements to it, we can use the `push` method,
as shown in Listing 8-3.
-->

Pour créer un vecteur et ensuite lui ajouter des éléments, nous pouvons utiliser
la méthode `push`, comme dans l'encart 8-3.

```rust
let mut v = Vec::new();

v.push(5);
v.push(6);
v.push(7);
v.push(8);
```

<!--
<span class="caption">Listing 8-3: Using the `push` method to add values to a
vector</span>
-->

<span class="caption">Encart 8-3 : Utiliser la méthode `push` pour ajouter des
valeurs à un vecteur</span>

<!--
As with any variable, if we want to be able to change its value, we need to
make it mutable using the `mut` keyword, as discussed in Chapter 3. The numbers
we place inside are all of type `i32`, and Rust infers this from the data, so
we don’t need the `Vec<i32>` annotation.
-->

Comme avec chaque variable, si nous voulons pouvoir modifier sa valeur, nous
devons la rendre mutable en utilisant le mot-clé `mut`, comme nous l'avons vu
au chapitre 3. Les nombres que nous ajoutons dedans sont tous du type `i32`, et
Rust le devine à partir des données, donc nous n'avons pas besoin de
l'annotation `Vec<i32>`.

<!--
### Dropping a Vector Drops Its Elements
-->

### Libérer un vecteur libère aussi ses éléments

<!--
Like any other `struct`, a vector is freed when it goes out of scope, as
annotated in Listing 8-4.
-->

Comme tous les autres `struct`, un vecteur est libéré quand il sort de la
portée, comme précisé dans l'encart 8-4.

<!--
```rust
{
    let v = vec![1, 2, 3, 4];

    // do stuff with v

} // <- v goes out of scope and is freed here
```
-->

```rust
{
    let v = vec![1, 2, 3, 4];

    // on fait des choses avec v

} // <- v sort de la portée et est libéré ici
```

<!--
<span class="caption">Listing 8-4: Showing where the vector and its elements
are dropped</span>
-->

<span class="caption">Encart 8-4 : On met en évidence où le vecteur et ses
éléments sont libérés</span>

<!--
When the vector gets dropped, all of its contents are also dropped, meaning
those integers it holds will be cleaned up. This may seem like a
straightforward point but can get a bit more complicated when you start to
introduce references to the elements of the vector. Let’s tackle that next!
-->

Lorsque le vecteur est libéré, tout son contenu est aussi libéré, ce qui veut
dire que les entiers qu'il stocke vont être effacés de la mémoire. Cela semble
très simple mais cela peut devenir plus compliqué quand vous commencez à
utiliser des références dans les éléments du vecteur. Voyons ceci dès à
présent !

<!--
### Reading Elements of Vectors
-->

### Lire les éléments des vecteurs

<!--
Now that you know how to create, update, and destroy vectors, knowing how to
read their contents is a good next step. There are two ways to reference a
value stored in a vector. In the examples, we’ve annotated the types of the
values that are returned from these functions for extra clarity.
-->

Maintenant que vous savez comment créer, modifier, et détruire des vecteurs,
savoir lire leur contenu est la suite logique. Il existe deux façons de désigner
une valeur enregistrée dans un vecteur. Dans ces exemples, nous avons précisé
les types des valeurs qui sont retournées par ces fonctions pour plus de clarté.

<!--
Listing 8-5 shows both methods of accessing a value in a vector, either with
indexing syntax or the `get` method.
-->

L'encart 8-5 nous montre les deux façons pour accéder à une valeur d'un vecteur,
soit la syntaxe avec un indice, soit avec la méthode `get`.

<!--
```rust
let v = vec![1, 2, 3, 4, 5];

let third: &i32 = &v[2];
println!("The third element is {}", third);

match v.get(2) {
    Some(third) => println!("The third element is {}", third),
    None => println!("There is no third element."),
}
```
-->

```rust
let v = vec![1, 2, 3, 4, 5];

let troisieme: &i32 = &v[2];
println!("Le troisième élément est {}", troisieme);

match v.get(2) {
    Some(troisieme) => println!("Le troisième élément est {}", troisieme),
    None => println!("Il n'y a pas de troisième élément."),
}
```

<!--
<span class="caption">Listing 8-5: Using indexing syntax or the `get` method to
access an item in a vector</span>
-->

<span class="caption">Encart 8-5 : Utilisation de la syntaxe avec l'indice ou
la méthode `get` pour accéder à un élément d'un vecteur</span>

<!--
Note two details here. First, we use the index value of `2` to get the third
element: vectors are indexed by number, starting at zero. Second, the two ways
to get the third element are by using `&` and `[]`, which gives us a reference,
or by using the `get` method with the index passed as an argument, which gives
us an `Option<&T>`.
-->

Il y a deux détails à remarquer ici. Premièrement, nous avons utilisé l'indice
`2` pour obtenir le troisième élément : les vecteurs sont indexés par des
nombres, qui commencent à partir zéro. Deuxièmement, les deux façons d'obtenir
le troisième éléments consistent soit à utiliser `&` et `[]`, ce qui nous donne
une référence, soit en utilisant la méthode `get` avec l'indice en argument, ce
qui nous fournit un `Option<&T>`.

<!--
Rust has two ways to reference an element so you can choose how the program
behaves when you try to use an index value that the vector doesn’t have an
element for. As an example, let’s see what a program will do if it has a vector
that holds five elements and then tries to access an element at index 100, as
shown in Listing 8-6.
-->

Rust a deux manières de récupérer un élément afin que vous puissiez choisir
comment le programme doit se comporter lorsque vous essayerez d'utiliser un
indice dont le vecteur n'a pas d'élément correspondant. Par exemple, regardons
ce qu'un programme fait s'il a vecteur qui contient cinq éléments et qu'il
essaye d'accéder à l'élément à l'indice 100, comme dans l'encart 8-6.

<!--
```rust,should_panic,panics
let v = vec![1, 2, 3, 4, 5];

let does_not_exist = &v[100];
let does_not_exist = v.get(100);
```
-->

```rust,should_panic,panics
let v = vec![1, 2, 3, 4, 5];

let existe_pas = &v[100];
let existe_pas = v.get(100);
```

<!--
<span class="caption">Listing 8-6: Attempting to access the element at index
100 in a vector containing five elements</span>
-->

<span class="caption">Encart 8-6 : Tentative d'accès à l'élément à l'indice 100
dans un vecteur qui contient cinq éléments</span>

<!--
When we run this code, the first `[]` method will cause the program to panic
because it references a nonexistent element. This method is best used when you
want your program to crash if there’s an attempt to access an element past the
end of the vector.
-->

Lorsque nous exécutons ce code, la première méthode `[]` va faire paniquer le
programme car il demande un élément non existant. Cette méthode doit être
favorisée lorsque vous souhaitez que votre programme plante s'il y a une
tentative d'accéder à un élément après la fin du vecteur.

<!--
When the `get` method is passed an index that is outside the vector, it returns
`None` without panicking. You would use this method if accessing an element
beyond the range of the vector happens occasionally under normal circumstances.
Your code will then have logic to handle having either `Some(&element)` or
`None`, as discussed in Chapter 6. For example, the index could be coming from
a person entering a number. If they accidentally enter a number that’s too
large and the program gets a `None` value, you could tell the user how many
items are in the current vector and give them another chance to enter a valid
value. That would be more user-friendly than crashing the program due to a typo!
-->

Lorsque nous passons un indice en dehors de l'intervalle du vecteur à la méthode
`get`, elle retourne `None` sans paniquer. Vous devriez utiliser cette méthode
s'il vous arrive occasionnellement de vouloir accéder à un élément en dehors de
l'intervalle du vecteur en temps normal. Votre code va ensuite devoir gérer les
deux valeurs `Some(&element)` ou `None`, comme nous l'avons vu au chapitre 6.
Par exemple, l'indice peut provenir d'une saisie utilisateur. Si par accident il
saisit un nombre qui est trop grand et que le programme obtient une valeur
`None`, vous pouvez alors dire à l'utilisateur combien il y a d'éléments dans le
vecteur courant et lui donner une nouvelle chance de saisir une valeur valide.
Cela sera plus convivial que de faire planter le programme à cause d'une faute
de frappe !

<!--
When the program has a valid reference, the borrow checker enforces the
ownership and borrowing rules (covered in Chapter 4) to ensure this reference
and any other references to the contents of the vector remain valid. Recall the
rule that states you can’t have mutable and immutable references in the same
scope. That rule applies in Listing 8-7, where we hold an immutable reference to
the first element in a vector and try to add an element to the end, which won’t
work.
-->

Lorsque le programme obtient une référence valide, le vérificateur d'emprunt va
faire appliquer les règles de possession et d'emprunt (que nous avons vu au
chapitre 4) pour s'assurer que cette référence ainsi que toutes les autres
références au contenu de ce vecteur restent valides. Souvenez-vous de la règle
qui dit que vous ne pouvez pas avoir des références mutables et immuables dans
la même portée. Cette règle s'applique à l'encart 8-7, où nous obtenons une
référence immuable vers le premier élément d'un vecteur et nous essayons
d'ajouter un élément à la fin, ce qui ne fonctionne pas.

<!--
```rust,ignore,does_not_compile
let mut v = vec![1, 2, 3, 4, 5];

let first = &v[0];

v.push(6);

println!("The first element is: {}", first);
```
-->

```rust,ignore,does_not_compile
let mut v = vec![1, 2, 3, 4, 5];

let premier = &v[0];

v.push(6);

println!("Le premier élément est : {}", premier);
```

<!--
<span class="caption">Listing 8-7: Attempting to add an element to a vector
while holding a reference to an item</span>
-->

<span class="caption">Encart 8-7 : Tentative d'ajout d'un élément à un vecteur
alors que nous utilisons une référence à un élément</span>

<!--
Compiling this code will result in this error:
-->

Compiler ce code va nous mener à cette erreur :

<!--
```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 -- > src/main.rs:6:5
  |
4 |     let first = &v[0];
  |                  - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("The first element is: {}", first);
  |                                          ----- immutable borrow later used here
```
-->

```text
error[E0502]: cannot borrow `v` as mutable because it is also borrowed as immutable
 --> src/main.rs:6:5
  |
4 |     let premier = &v[0];
  |                    - immutable borrow occurs here
5 |
6 |     v.push(6);
  |     ^^^^^^^^^ mutable borrow occurs here
7 |
8 |     println!("Le premier élément est : {}", premier);
  |                                             ------- immutable borrow later used here
```

<!--
The code in Listing 8-7 might look like it should work: why should a reference
to the first element care about what changes at the end of the vector? This
error is due to the way vectors work: adding a new element onto the end of the
vector might require allocating new memory and copying the old elements to the
new space, if there isn’t enough room to put all the elements next to each
other where the vector currently is. In that case, the reference to the first
element would be pointing to deallocated memory. The borrowing rules prevent
programs from ending up in that situation.
-->

Le code dans l'encart 8-7 semble pourtant marcher : pourquoi une référence au
premier élément devrait se soucier de ce qui se passe à la fin du vecteur ?
Cette erreur s'explique par la façon dont les vecteurs fonctionnent : ajouter un
nouvel élément à la fin du vecteur peut nécessiter d'allouer un nouvel espace
mémoire et copier tous les anciens éléments dans ce nouvel espace, s'il n'y a
pas assez de place pour placer tous les éléments les un à côté des autres dans
la mémoire où se trouve actuellement le vecteur. Dans notre cas, la référence
au premier élément pourrait pointer sur de la mémoire supprimée. Les règles
d'emprunt évitent aux programmes de se retrouver dans cette situation.

<!--
> Note: For more on the implementation details of the `Vec<T>` type, see “The
> Rustonomicon” at https://doc.rust-lang.org/stable/nomicon/vec.html.
-->

> Remarque : pour plus de détails sur l'implémentation du type `Vec<T>`,
> consultez [“The Rustonomicon”](https://doc.rust-lang.org/stable/nomicon/vec.html)

<!--
### Iterating over the Values in a Vector
-->

### Itérer sur les valeurs d'un vecteur

<!--
If we want to access each element in a vector in turn, we can iterate through
all of the elements rather than use indices to access one at a time. Listing
8-8 shows how to use a `for` loop to get immutable references to each element
in a vector of `i32` values and print them.
-->

Si nous voulons accéder à chaque élément d'un vecteur chacun à leur tour, nous
pouvons itérer sur tous les éléments plutôt que d'utiliser individuellement les
indices. L'encart 8-8 nous montre comment utiliser une boucle `for` pour obtenir
des références imuables pour chacun des éléments dans un vecteur de `i32`, et
les afficher.

```rust
let v = vec![100, 32, 57];
for i in &v {
    println!("{}", i);
}
```

<!--
<span class="caption">Listing 8-8: Printing each element in a vector by
iterating over the elements using a `for` loop</span>
-->

<span class="caption">Encart 8-8 : Afficher chaque élément d'un vecteur en
itérant sur les éléments en utilisant une boucle `for`</span>

<!--
We can also iterate over mutable references to each element in a mutable vector
in order to make changes to all the elements. The `for` loop in Listing 8-9
will add `50` to each element.
-->

Nous pouvons aussi itérer avec des références mutables pour chacun des éléments
d'un vecteur mutable afin de modifier tous les éléments. La boucle `for` de
l'encart 8-9 va ajouter `50` à chacun des éléments.

```rust
let mut v = vec![100, 32, 57];
for i in &mut v {
    *i += 50;
}
```

<!--
<span class="caption">Listing 8-9: Iterating over mutable references to
elements in a vector</span>
-->

<span class="caption">Encart 8-9 : Itérer avec des références mutables vers des
éléments d'un vecteur</span>

<!--
To change the value that the mutable reference refers to, we have to use the
dereference operator (`*`) to get to the value in `i` before we can use the
`+=` operator. We’ll talk more about the dereference operator in the
[“Following the Pointer to the Value with the Dereference Operator”][deref]
section of Chapter 15.
-->

Afin de changer la valeur que la référence mutable pointe, nous devons utiliser
l'opérateur de déréférencement (`*`) pour obtenir la valeur dans `i` avant que
nous puissions utiliser l'opérateur `+=`. Nous verrons l'opérateur de
déréférencement dans une section du [chapitre 15][deref].

<!--
### Using an Enum to Store Multiple Types
-->

### Utiliser une énumération pour stocker différents types

<!--
At the beginning of this chapter, we said that vectors can only store values
that are the same type. This can be inconvenient; there are definitely use
cases for needing to store a list of items of different types. Fortunately, the
variants of an enum are defined under the same enum type, so when we need to
store elements of a different type in a vector, we can define and use an enum!
-->

Au début de ce chapitre, nous avons dit que les vecteurs ne peuvent que stocker
des valeurs du même type. Cela peut être un problème ; il y a forcément des cas
où on a besoin de stocker une liste d'éléments de types différents.
Heureusement, les variantes d'une énumération sont définies sous le même type
qui est l'énumération, donc lorsque nous avons besoin de stocker des éléments
d'un type différent dans un vecteur, nous pouvons définir et utiliser une
énumération !

<!--
For example, say we want to get values from a row in a spreadsheet in which
some of the columns in the row contain integers, some floating-point numbers,
and some strings. We can define an enum whose variants will hold the different
value types, and then all the enum variants will be considered the same type:
that of the enum. Then we can create a vector that holds that enum and so,
ultimately, holds different types. We’ve demonstrated this in Listing 8-10.
-->

Par exemple, imaginons que nous voulons obtenir les valeurs d'une ligne d'une
feuille de calculs dans laquelle quelques colonnes sont des entiers, d'autres
des nombres à virgule flottante, et quelques chaînes de caractères. Nous pouvons
définir une énumération dont les variantes vont avoir les différents types, et
ainsi toutes les variantes de l'énumération seront du même type : celle de
l'énumération. Ensuite, nous pouvons créer un vecteur qui stocke cette
énumération et ainsi, au final, qui stocke différents types. La démonstration de
cette technique est dans l'encart 8-10.

<!--
```rust
enum SpreadsheetCell {
    Int(i32),
    Float(f64),
    Text(String),
}

let row = vec![
    SpreadsheetCell::Int(3),
    SpreadsheetCell::Text(String::from("blue")),
    SpreadsheetCell::Float(10.12),
];
```
-->

```rust
enum Cellule {
    Int(i32),
    Float(f64),
    Text(String),
}

let ligne = vec![
    Cellule::Int(3),
    Cellule::Text(String::from("bleu")),
    Cellule::Float(10.12),
];
```

<!--
<span class="caption">Listing 8-10: Defining an `enum` to store values of
different types in one vector</span>
-->

<span class="caption">Encart 8-10 : Définition d'une `enum` pour stocker des
valeurs de différents types dans un seul vecteur</span>

<!--
Rust needs to know what types will be in the vector at compile time so it knows
exactly how much memory on the heap will be needed to store each element. A
secondary advantage is that we can be explicit about what types are allowed in
this vector. If Rust allowed a vector to hold any type, there would be a chance
that one or more of the types would cause errors with the operations performed
on the elements of the vector. Using an enum plus a `match` expression means
that Rust will ensure at compile time that every possible case is handled, as
discussed in Chapter 6.
-->

Rust a besoin de savoir quel type de données sera stocké dans le vecteur au
moment de la compilation afin de savoir exactement combien de mémoire il aura
besoin pour stocker chaque élément sur le tas. Le second avantage est que nous
sommes précis sur les types autorisés dans ce vecteur. Si Rust avait permis
qu'un vecteur stocke n'importe quel type, il aurait pu avoir un risque qu'un ou
plusieurs des types causerait une erreur avec les manipulations effectuées sur
les éléments du vecteur. L'utilisation d'une énumération ainsi qu'une expression
`match` peut permettre à Rust de garantir au moment de la compilation que tous
les cas possibles sont traités, comme nous l'avons appris au chapitre 6.

<!--
When you’re writing a program, if you don’t know the exhaustive set of types
the program will get at runtime to store in a vector, the enum technique won’t
work. Instead, you can use a trait object, which we’ll cover in Chapter 17.
-->

Lorsque vous écrivez un programme, si vous n'avez pas une liste précise des
types que votre vecteur va stocker, la technique de l'énumération ne va pas
fonctionner. A la place, vous pouvez utiliser un objet trait, que nous verrons
au chapitre 17.

<!--
Now that we’ve discussed some of the most common ways to use vectors, be sure
to review the API documentation for all the many useful methods defined on
`Vec<T>` by the standard library. For example, in addition to `push`, a `pop`
method removes and returns the last element. Let’s move on to the next
collection type: `String`!
-->

Maintenant que nous avons vu les manières les plus courantes d'utiliser les
vecteurs, prenez le temps de consulter la documentation de l'API pour découvrir
toutes les méthodes très utiles définies dans la bibliothèque standard pour
`Vec<T>`. Par exemple, en plus de `push`, nous avons une méthode `pop` qui
retire et retourne le dernier élément. Intéressons-nous maintenant au prochain
type de collection : les `String` !

<!--
[deref]: ch15-02-deref.html#following-the-pointer-to-the-value-with-the-dereference-operator
-->

[deref]: ch15-02-deref.html
