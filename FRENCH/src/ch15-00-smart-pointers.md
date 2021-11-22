> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/185).

<!--
# Smart Pointers
-->

# Les pointeurs intelligents

<!--
A *pointer* is a general concept for a variable that contains an address in
memory. This address refers to, or “points at,” some other data. The most
common kind of pointer in Rust is a reference, which you learned about in
Chapter 4. References are indicated by the `&` symbol and borrow the value they
point to. They don’t have any special capabilities other than referring to
data. Also, they don’t have any overhead and are the kind of pointer we use
most often.
-->

Un *pointeur* est un concept général pour une variable qui contient une adresse
vers la mémoire. Cette adresse pointe vers d'autres données. Le type de pointeur
le plus courant en Rust est la référence, que vous avez appris au chapitre 4.
Les références sont marquées par le symbole `&` et empruntent la valeur sur
laquelle ils pointent. Elles n'ont pas d'autres fonctionnalités que celle de
pointer sur une donnée. De plus, elles n'ont aucun coût sur les performances et
c'est le type de pointeur que nous utilisons le plus souvent.

<!--
*Smart pointers*, on the other hand, are data structures that not only act like
a pointer but also have additional metadata and capabilities. The concept of
smart pointers isn’t unique to Rust: smart pointers originated in C++ and exist
in other languages as well. In Rust, the different smart pointers defined in
the standard library provide functionality beyond that provided by references.
One example that we’ll explore in this chapter is the *reference counting*
smart pointer type. This pointer enables you to have multiple owners of data by
keeping track of the number of owners and, when no owners remain, cleaning up
the data.
-->

Les *pointeurs intelligents*, d'une autre côté, sont des structures de données
qui, non seulement se comportent comme un pointeur, mais ont aussi des
fonctionnalités et métadonnées supplémentaires. Le concept de pointeur
intelligent n'est pas propre à Rust : les pointeurs intelligents sont
originaires du C++ et existent aussi dans d'autres langages. En Rust, les
différents pointeurs intelligents définis dans bibliothèque standard
fournissent des fonctionnalités supplémentaires à celles des références.
Un exemple que nous allons explorer dans ce chapitre est le type de pointeur
intelligent *compteur de références*. Ce pointeur vous permet d'avoir
plusieurs propriétaires d'une donnée tout en gardant une trace de leur nombre et,
lorsqu'il n'y en a plus, nettoyer cette donnée.

<!--
In Rust, which uses the concept of ownership and borrowing, an additional
difference between references and smart pointers is that references are
pointers that only borrow data; in contrast, in many cases, smart pointers
*own* the data they point to.
-->

En Rust, qui utilise le concept de propriétaire et d'emprunt, une différence
supplémentaire entre les références et les pointeurs intelligents est que les
références sont des pointeurs qui empruntent seulement la donnée ; alors qu'au
contraire, dans de nombreux cas, les pointeurs intelligents sont
*propriétaires* des données sur lesquels ils pointent.

<!--
We’ve already encountered a few smart pointers in this book, such as `String`
and `Vec<T>` in Chapter 8, although we didn’t call them smart pointers at the
time. Both these types count as smart pointers because they own some memory and
allow you to manipulate it. They also have metadata (such as their capacity)
and extra capabilities or guarantees (such as with `String` ensuring its data
will always be valid UTF-8).
-->

Nous avons déjà rencontré quelques pointeurs intelligents au cours de ce
livre, comme `String` et `Vec<T>` au chapitre 8, même si nous ne les avons pas
désignés comme étant des pointeurs intelligents à ce moment-là. Ces deux types
sont considérés comme des pointeurs intelligents car ils sont propriétaires de
ces données et vous permettent de les manipuler. Ils ont aussi des métadonnées
(comme leur capacité) et certaines fonctionnalités ou garanties (comme `String`
qui s'assure que ses données soient toujours en UTF-8).

<!--
Smart pointers are usually implemented using structs. The characteristic that
distinguishes a smart pointer from an ordinary struct is that smart pointers
implement the `Deref` and `Drop` traits. The `Deref` trait allows an instance
of the smart pointer struct to behave like a reference so you can write code
that works with either references or smart pointers. The `Drop` trait allows
you to customize the code that is run when an instance of the smart pointer
goes out of scope. In this chapter, we’ll discuss both traits and demonstrate
why they’re important to smart pointers.
-->

Les pointeurs intelligents sont souvent implémentés en utilisant des
structures. Les caractéristiques qui distinguent un pointeur intelligent d'une
structure classique est que les pointeurs intelligents implémentent les traits
`Deref` et `Drop`. Le trait `Deref` permet à une instance d'un pointeur
intelligent de se comporter comme une référence afin que vous puissiez écrire
du code qui fonctionne aussi bien avec des références qu'avec des pointeurs
intelligents. Le trait `Drop` vous permet de personnaliser le code qui est
exécuté lorsqu'une instance d'un pointeur intelligent sort de la portée. Dans
ce chapitre, nous verrons ces deux traits et expliquerons pourquoi ils sont
importants pour les pointeurs intelligents.

<!--
Given that the smart pointer pattern is a general design pattern used
frequently in Rust, this chapter won’t cover every existing smart pointer. Many
libraries have their own smart pointers, and you can even write your own. We’ll
cover the most common smart pointers in the standard library:
-->

Vu que le motif des pointeurs intelligents est un motif de conception général
fréquemment utilisé en Rust, ce chapitre ne couvrira pas tous les pointeurs
intelligents existants. De nombreuses bibliothèques ont leurs propres pointeurs
intelligents, et vous pouvez même écrire le vôtre. Nous allons voir les
pointeurs intelligents les plus courants de la bibliothèque standard :

<!--
* `Box<T>` for allocating values on the heap
* `Rc<T>`, a reference counting type that enables multiple ownership
* `Ref<T>` and `RefMut<T>`, accessed through `RefCell<T>`, a type that enforces
  the borrowing rules at runtime instead of compile time
-->

* `Box<T>` pour l'allocation de valeurs sur le tas
* `Rc<T>`, un type comptant les références, qui permet d'avoir plusieurs
  propriétaires
* `Ref<T>` et `RefMut<T>`, par lesquels on y accède via `RefCell<T>`, un type
  qui permet d'appliquer les règles d'emprunt au moment de l'exécution au lieu
  du moment de la compilation

<!--
In addition, we’ll cover the *interior mutability* pattern where an immutable
type exposes an API for mutating an interior value. We’ll also discuss
*reference cycles*: how they can leak memory and how to prevent them.
-->

En outre, nous allons voir le motif de *mutabilité interne* dans lequel un
type immuable propose une API pour muter une valeur interne. Nous allons aussi
parler des *boucles de références* : comment elles peuvent provoquer des fuites
de mémoire et comment les éviter.

<!--
Let’s dive in!
-->

Allons-y !
