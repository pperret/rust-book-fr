> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/194).

<!--
## `RefCell<T>` and the Interior Mutability Pattern
-->

## `RefCell<T>` et le motif de mutabilité interne

<!--
*Interior mutability* is a design pattern in Rust that allows you to mutate
data even when there are immutable references to that data; normally, this
action is disallowed by the borrowing rules. To mutate data, the pattern uses
`unsafe` code inside a data structure to bend Rust’s usual rules that govern
mutation and borrowing. We haven’t yet covered unsafe code; we will in Chapter
19. We can use types that use the interior mutability pattern when we can
ensure that the borrowing rules will be followed at runtime, even though the
compiler can’t guarantee that. The `unsafe` code involved is then wrapped in a
safe API, and the outer type is still immutable.
-->

La *mutabilité interne* est un motif de conception en Rust qui vous permet de
muter une donnée même s'il existe des références immuables ; normalement, cette
action n'est pas autorisée par les règles d'emprunt. Pour muter des données, le
motif utilise du code `unsafe` dans une structure de données pour contourner les
règles courantes de Rust qui gouvernent la mutation et l'emprunt. Nous n'avons
pas encore parlé du code unsafe ; nous le ferons au chapitre 19. Nous pouvons
utiliser des types qui utilisent le motif de mutabilité interne lorsque nous
pouvons être sûr que les règles d'emprunt seront suivies au moment de
l'exécution, même si le compilateur ne peut pas en être sûr. Le code `unsafe`
concerné est ensuite incorporé dans une API stable, et le type externe reste
immuable.

<!--
Let’s explore this concept by looking at the `RefCell<T>` type that follows the
interior mutability pattern.
-->

Découvrons ce concept en examinant le type `RefCell<T>` qui applique le motif
de mutabilité interne.

<!--
### Enforcing Borrowing Rules at Runtime with `RefCell<T>`
-->

### Appliquer les règles d'emprunt au moment de l'exécution avec `RefCell<T>`

<!--
Unlike `Rc<T>`, the `RefCell<T>` type represents single ownership over the data
it holds. So, what makes `RefCell<T>` different from a type like `Box<T>`?
Recall the borrowing rules you learned in Chapter 4:
-->

Contrairement à `Rc<T>`, le type `RefCell<T>` représente un seul propriétaire
de la donnée qu'il contient. Donc, qu'est-ce qui rend `RefCell<T>` différent
d'un type comme `Box<T>` ? Souvenez-vous des règles d'emprunt que vous avez
appris au chapitre 4 :

<!--
* At any given time, you can have *either* (but not both of) one mutable
  reference or any number of immutable references.
* References must always be valid.
-->

* A un instant donné, vous pouvez avoir *soit* (mais pas les deux) une
  référence mutable, soit n'importe quelle quantité de références immuables
* Les références doivent toujours être en vigueur.

<!--
With references and `Box<T>`, the borrowing rules’ invariants are enforced at
compile time. With `RefCell<T>`, these invariants are enforced *at runtime*.
With references, if you break these rules, you’ll get a compiler error. With
`RefCell<T>`, if you break these rules, your program will panic and exit.
-->

Avec les références et `Box<T>`, les règles d'emprunt obligatoires sont
appliquées au moment de la compilation. Avec `RefCell<T>`, ces obligations
sont appliquées *au moment de l'exécution*. Avec les références, si vous ne
respectez pas ces règles, vous allez obtenir une erreur de compilation. Avec
`RefCell<T>`, si vous ne les respectez pas, votre programme va paniquer et se
fermer.

<!--
The advantages of checking the borrowing rules at compile time are that errors
will be caught sooner in the development process, and there is no impact on
runtime performance because all the analysis is completed beforehand. For those
reasons, checking the borrowing rules at compile time is the best choice in the
majority of cases, which is why this is Rust’s default.
-->

Les avantages de vérifier les règles d'emprunt au moment de la compilation est
que les erreurs vont se produire plus tôt dans le processus de développement,
et il n'y a pas d'impact sur la performance à l'exécution car toute l'analyse
a déjà été faite au préalable. Pour ces raisons, la vérification des règles
d'emprunt au moment de compilation est le meilleur choix à faire dans la
majorité des cas, ce qui explique pourquoi c'est le choix par défaut de Rust.

<!--
The advantage of checking the borrowing rules at runtime instead is that
certain memory-safe scenarios are then allowed, whereas they are disallowed by
the compile-time checks. Static analysis, like the Rust compiler, is inherently
conservative. Some properties of code are impossible to detect by analyzing the
code: the most famous example is the Halting Problem, which is beyond the scope
of this book but is an interesting topic to research.
-->

L'avantage de vérifier les règles d'emprunt plutôt à l'exécution est que cela
permet certains scénarios qui restent sûrs pour la mémoire, bien qu'interdites
à cause des vérifications à la compilation. L'analyse statique, comme le
compilateur Rust, est de nature prudente. Certaines propriétés du code sont
impossibles à détecter en analysant le code : l'exemple le plus connu est le
*problème de l'arrêt*, qui dépasse le cadre de ce livre mais qui reste un
sujet intéressant à étudier.

<!--
Because some analysis is impossible, if the Rust compiler can’t be sure the
code complies with the ownership rules, it might reject a correct program; in
this way, it’s conservative. If Rust accepted an incorrect program, users
wouldn’t be able to trust in the guarantees Rust makes. However, if Rust
rejects a correct program, the programmer will be inconvenienced, but nothing
catastrophic can occur. The `RefCell<T>` type is useful when you’re sure your
code follows the borrowing rules but the compiler is unable to understand and
guarantee that.
-->

Comme certaines analyses sont impossibles, si le compilateur Rust ne peut pas
s'assurer que le code respecte les règles d'emprunt, il risque de rejeter un
programme valide ; dans ce sens, il est prudent. Si Rust accepte un programme
incorrecte, les utilisateurs ne pourront pas avoir confiance dans les
garanties qu'apporte Rust. Cependant, si Rust rejette un programme valide, le
développeur sera importuné, mais rien de catastrophique ne va se passer. Le
type `RefCell<T>` est utile lorsque vous êtes sûr que votre code suit bien
les règles d'emprunt mais que le compilateur est incapable de comprendre et
de garantir cela.

<!--
Similar to `Rc<T>`, `RefCell<T>` is only for use in single-threaded scenarios
and will give you a compile-time error if you try using it in a multithreaded
context. We’ll talk about how to get the functionality of `RefCell<T>` in a
multithreaded program in Chapter 16.
-->

De la même manière que `Rc<T>`, `RefCell<T>` sert uniquement pour des
scénarios à une seule tâche et va vous donner une erreur à la compilation si
vous essayez de l'utiliser dans un contexte multitâches. Nous verrons
comment bénéficier les des fonctionnalités de `RefCell<T>` dans un programme
multi-processus au chapitre 16.

<!--
Here is a recap of the reasons to choose `Box<T>`, `Rc<T>`, or `RefCell<T>`:
-->

Voici un résumé des raisons pour choisir `Box<T>`, `Rc<T>`, ou `RefCell<T>` :

<!--
* `Rc<T>` enables multiple owners of the same data; `Box<T>` and `RefCell<T>`
  have single owners.
* `Box<T>` allows immutable or mutable borrows checked at compile time; `Rc<T>`
  allows only immutable borrows checked at compile time; `RefCell<T>` allows
  immutable or mutable borrows checked at runtime.
* Because `RefCell<T>` allows mutable borrows checked at runtime, you can
  mutate the value inside the `RefCell<T>` even when the `RefCell<T>` is
  immutable.
-->

* `Rc<T>` permet d'avoir plusieurs propriétaires pour une même donnée ;
  `Rc<T>` et `RefCell<T>` n'ont qu'un seul propriétaire.
* `Box<T>` permet des emprunts immuables ou mutables à la compilation ;
  `Rc<T>` permet uniquement des emprunts immuables, vérifiés à la
  compilation ; `RefCell<T>` permet des emprunts immuables ou mutables,
  vérifiés à l'exécution.
* Comme `RefCell<T>` permet des emprunts mutables, vérifiés à l'exécution,
  vous pouvez muter la valeur à l'intérieur du `RefCell<T>` même si le
  `RefCell<T>` est immuable.

<!--
Mutating the value inside an immutable value is the *interior mutability*
pattern. Let’s look at a situation in which interior mutability is useful and
examine how it’s possible.
-->

Modifer une valeur à l'intérieur d'une valeur immuable est ce qu'on appelle
le motif de *mutabilité interne*. Découvrons une situation pour laquelle la
mutabilité interne est utile est examinons comment c'est possible.

<!--
### Interior Mutability: A Mutable Borrow to an Immutable Value
-->

### Mutabilité interne : un emprunt mutable d'une valeur immuable

<!--
A consequence of the borrowing rules is that when you have an immutable value,
you can’t borrow it mutably. For example, this code won’t compile:
-->

Une des conséquences des règles d'emprunt est que lorsque vous avez une valeur
immuable, vous ne pouvez pas emprunter sa mutabilité. Par exemple, ce code ne
va pas se compiler :

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/src/main.rs}}
```

<!--
If you tried to compile this code, you’d get the following error:
-->

Si vous essayez de compiler ce code, vous allez obtenir l'erreur suivante :

<!--
```console
{{#include ../listings-sources/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/no-listing-01-cant-borrow-immutable-as-mutable/output.txt}}
```

<!--
However, there are situations in which it would be useful for a value to mutate
itself in its methods but appear immutable to other code. Code outside the
value’s methods would not be able to mutate the value. Using `RefCell<T>` is
one way to get the ability to have interior mutability. But `RefCell<T>`
doesn’t get around the borrowing rules completely: the borrow checker in the
compiler allows this interior mutability, and the borrowing rules are checked
at runtime instead. If you violate the rules, you’ll get a `panic!` instead of
a compiler error.
-->

Cependant, il existe des situations pour lesquelles il serait utile qu'une
valeur puisse se modifier elle-même dans ses propres méthodes mais qui semble
être immuable pour le reste du code. Le code à l'extérieur des méthodes de la
valeur n'est pas capable de modifier la valeur. L'utilisation de `RefCell<T>`
est une manière de pouvoir procéder à des mutations internes. Mais
`RefCell<T>` ne contourne pas complètement les règles d'emprunt : le
vérificateur d'emprunt du compilateur permet cette mutabilité interne, et les
règles d'emprunt sont plutôt vérifiées à l'exécution. Si vous violez les
règles, vous allez provoquer un `panic!` plutôt que d'avoir une erreur de
compilation.

<!--
Let’s work through a practical example where we can use `RefCell<T>` to mutate
an immutable value and see why that is useful.
-->

Voyons un exemple pratique dans lequel nous pouvons utiliser `RefCell<T>` pour
modifier une valeur immuable et voir en quoi cela est utile.

<!--
#### A Use Case for Interior Mutability: Mock Objects
-->

#### Un cas d'utilisation de la mutabilité interne : le mock object

<!--
A *test double* is the general programming concept for a type used in place of
another type during testing. *Mock objects* are specific types of test doubles
that record what happens during a test so you can assert that the correct
actions took place.
-->

Un *double de test* est un concept de programmation général pour un type
utilisé à la place d'un autre type pendant des tests. Un *mock object* est un
type particulier de double de test qui enregistre ce qui se passe lors d'un
test afin que vous puissiez vérifier que les actions se sont passées
correctement.

<!--
Rust doesn’t have objects in the same sense as other languages have objects,
and Rust doesn’t have mock object functionality built into the standard library
as some other languages do. However, you can definitely create a struct that
will serve the same purposes as a mock object.
-->

Rust n'a pas d'objets dans le sens qu'entendent les autres langages qui ont
des objets, et Rust n'a pas de fonctionnalité de mock object construit dans
la bibliothèque standard comme l'entendent d'autres langages. Cependant, vous
pouvez très bien créer une structure qui va répondre aux mêmes besoins qu'un
mock object.

<!--
Here’s the scenario we’ll test: we’ll create a library that tracks a value
against a maximum value and sends messages based on how close to the maximum
value the current value is. This library could be used to keep track of a
user’s quota for the number of API calls they’re allowed to make, for example.
-->

Voici le scénario que nous allons tester : nous allons créer une bibliothèque
qui traque une valeur par rapport à une valeur maximale et envoie des messages
en fonction de la proximité de la valeur courante. Cette bibliothèque peut
être utilisée pour suivre un quota d'un utilisateur pour le nombre d'appels
aux API qu'il est autorisé à faire, par exemple.

<!--
Our library will only provide the functionality of tracking how close to the
maximum a value is and what the messages should be at what times. Applications
that use our library will be expected to provide the mechanism for sending the
messages: the application could put a message in the application, send an
email, send a text message, or something else. The library doesn’t need to know
that detail. All it needs is something that implements a trait we’ll provide
called `Messenger`. Listing 15-20 shows the library code:
-->

Notre bibliothèque va seulement fournir la fonctionnalité de suivi en fonction
de la valeur maximale et quels seront les messages à chaque moment. Les
applications qui utiliseront notre bibliothèque devront fournir un mécanisme
pour envoyer les messages : l'application peut afficher le message dans
l'application, l'envoyer par email, l'envoyer par SMS, ou autre chose. La
bibliothèque n'a pas à se charger de ce détail. Tout ce que ce mécanisme doit
faire est d'implémenter un trait `Messager` que nous allons fournir. L'encart
15-20 propose du code pour cette bibliothèque :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-20/src/lib.rs}}
```

<!--
<span class="caption">Listing 15-20: A library to keep track of how close a
value is to a maximum value and warn when the value is at certain levels</span>
-->

<span class="caption">Encart 15-20 : une bibliothèque qui suit la proximité
d'une valeur avec une valeur maximale et avertit lorsque cette valeur atteint
un certain seuil</span>

<!--
One important part of this code is that the `Messenger` trait has one method
called `send` that takes an immutable reference to `self` and the text of the
message. This trait is the interface our mock object needs to implement so that
the mock can be used in the same way a real object is. The other important part
is that we want to test the behavior of the `set_value` method on the
`LimitTracker`. We can change what we pass in for the `value` parameter, but
`set_value` doesn’t return anything for us to make assertions on. We want to be
able to say that if we create a `LimitTracker` with something that implements
the `Messenger` trait and a particular value for `max`, when we pass different
numbers for `value`, the messenger is told to send the appropriate messages.
-->

La partie la plus importante de ce code est celle où le trait `Messager` a une
méthode qui fait appel à `envoyer` en prenant une référence immuable à `self`
ainsi que le texte du message. Ce trait est l'interface que notre mock object
doit implémenter afin que le mock puisse être utilisé de la même manière que
l'objet réel. L'autre partie importante est lorsque nous souhaitons tester le
comportement de la méthode `set_valeur` sur le `TraqueurDeLimite`. Nous pouvons
changer ce que nous envoyons dans le paramètre `valeur`, mais `set_valeur` ne
nous retourne rien qui nous permet de le vérifier. Nous voulons pouvoir dire que
si nous créons un `TraqueurDeLimite` avec quelque chose qui implémente le trait
`Messager` et une valeur précise pour `max`, lorsque nous passons différents
nombres pour `valeur`, que le messager puisse confirmer l'envoi des messages
correspondants.

<!--
We need a mock object that, instead of sending an email or text message when we
call `send`, will only keep track of the messages it’s told to send. We can
create a new instance of the mock object, create a `LimitTracker` that uses the
mock object, call the `set_value` method on `LimitTracker`, and then check that
the mock object has the messages we expect. Listing 15-21 shows an attempt to
implement a mock object to do just that, but the borrow checker won’t allow it:
-->

Nous avons besoin d'un mock object qui, au lieu d'envoyer un email ou un SMS
lorsque nous faisons appel à `envoyer`, va seulement enregistrer les messages
qu'on lui demande d'envoyer. Nous pouvons créer une nouvelle instance du mock
object, créer un `TraqueurDeLimite` qui utilise le mock object, faire appel à la
méthode `set_value` sur le `TraqueurDeLimite`, et ensuite vérifier que le mock
object a bien les messages que nous attendions. L'encart 15-21 montre une
tentative d'implémentation d'un mock object qui fait ceci, mais le vérificateur
d'emprunt ne nous autorise pas à le faire :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-21/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 15-21: An attempt to implement a `MockMessenger`
that isn’t allowed by the borrow checker</span>
-->

<span class="caption">Encart 15-21 : une tentative d'implémentation d'un
`MessagerMock` qui n'est pas autorisé par le vérificateur d'emprunt</span>

<!--
This test code defines a `MockMessenger` struct that has a `sent_messages`
field with a `Vec` of `String` values to keep track of the messages it’s told
to send. We also define an associated function `new` to make it convenient to
create new `MockMessenger` values that start with an empty list of messages. We
then implement the `Messenger` trait for `MockMessenger` so we can give a
`MockMessenger` to a `LimitTracker`. In the definition of the `send` method, we
take the message passed in as a parameter and store it in the `MockMessenger`
list of `sent_messages`.
-->

Ce code de test définit une structure `MessagerMock` qui a un champ
`messages_envoyes` qui est un `Vec` de valeurs `String`, afin d'y enregistrer
les messages qui lui sont envoyés. Nous définissons également une fonction
associée `new` pour faciliter la création de valeurs `MessagerMock` qui
commencent avec une liste vide de messages. Nous implémentons ensuite le trait
`Messager` sur `MessagerMock` afin de donner un `MessagerMock` à un
`TraqueurDeLimite`. Dans la définition de la méthode `envoyer`, nous prenons
le message envoyé en paramètre et nous le stockons dans la liste
`messages_envoyes` du `MessagerMock`.

<!--
In the test, we’re testing what happens when the `LimitTracker` is told to set
`value` to something that is more than 75 percent of the `max` value. First, we
create a new `MockMessenger`, which will start with an empty list of messages.
Then we create a new `LimitTracker` and give it a reference to the new
`MockMessenger` and a `max` value of 100. We call the `set_value` method on the
`LimitTracker` with a value of 80, which is more than 75 percent of 100. Then
we assert that the list of messages that the `MockMessenger` is keeping track
of should now have one message in it.
-->

Dans le test, nous vérifions ce qui se passe lorsque le `TraqueurDeLimite`
doit atteindre une valeur qui est supérieure à 75 pourcent de la valeur `max`.
D'abord, nous créons un nouveau `MessagerMock`, qui va démarrer avec une liste
vide de messages. Ensuite, nous créons un nouveau `TraqueurDeLimite` et nous
lui donnons une référence vers ce `MessagerMock` et une valeur `max` de 100.
Nous appelons la méthode `set_valeur` sur le `TraqueurDeLimite` avec une
valeur de 80, qui est plus grande que 75 pourcent de 100. Enfin, nous
vérifions que la liste de messages qu'a enregistré le `MessagerMock` a bien
un message à l'intérieur.

<!--
However, there’s one problem with this test, as shown here:
-->

Cependant, il reste un problème avec ce test, qui est montré ci-dessous :

<!--
```console
{{#include ../listings-sources/ch15-smart-pointers/listing-15-21/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-21/output.txt}}
```

<!--
We can’t modify the `MockMessenger` to keep track of the messages, because the
`send` method takes an immutable reference to `self`. We also can’t take the
suggestion from the error text to use `&mut self` instead, because then the
signature of `send` wouldn’t match the signature in the `Messenger` trait
definition (feel free to try and see what error message you get).
-->

Nous ne pouvons pas modifier le `MessagerMock` pour enregistrer les messages,
car la méthode `envoyer` utilise une référence immuable à `self`. Nous ne
pouvons pas non plus suivre la suggestion du texte d'erreur pour utiliser
`&mut self` à la place, car ensuite la signature de `envoyer` ne va pas
correspondre à la signature de la définition du trait `Messager` (essayez et
vous constaterez le message d'erreur que vous obtiendrez).

<!--
This is a situation in which interior mutability can help! We’ll store the
`sent_messages` within a `RefCell<T>`, and then the `send` method will be
able to modify `sent_messages` to store the messages we’ve seen. Listing 15-22
shows what that looks like:
-->

C'est une situation dans laquelle la mutabilité interne peut nous aider !
Nous allons stocker `messages_envoyes` dans une `RefCell<T>`, et ensuite la
méthode `envoyer` pourra modifier `messages_envoyes` pour stocker les
messages que nous avons avons vu. L'encart 15-22 montre à quoi cela peut
ressembler :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-22/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 15-22: Using `RefCell<T>` to mutate an inner
value while the outer value is considered immutable</span>
-->

<span class="caption">Encart 15-22 : utilisation du `RefCell<T>` pour muter
une valeur interne que les valeurs externes considèrent comme immuable
</span>

<!--
The `sent_messages` field is now of type `RefCell<Vec<String>>` instead of
`Vec<String>`. In the `new` function, we create a new `RefCell<Vec<String>>`
instance around the empty vector.
-->

Le champ `messages_envoyes` est maintenant du type `RefCell<Vec<String>>` au
lieu de `Vec<String>`. Dans la fonction `new`, nous créons une nouvelle
instance de `RefCell<Vec<String>>` autour du vecteur vide.

<!--
For the implementation of the `send` method, the first parameter is still an
immutable borrow of `self`, which matches the trait definition. We call
`borrow_mut` on the `RefCell<Vec<String>>` in `self.sent_messages` to get a
mutable reference to the value inside the `RefCell<Vec<String>>`, which is
the vector. Then we can call `push` on the mutable reference to the vector to
keep track of the messages sent during the test.
-->

En ce qui concerne l'implémentation de la méthode `envoyer`, le premier
paramètre est toujours un emprunt immuable de `self`, ce qui correspond à la
définition du trait. Nous appelons la méthode `borrow_mut` sur le
`RefCell<Vec<String>>` présent dans `self.messages_envoyes` pour obtenir une
référence mutable vers la valeur présente dans le `RefCell<Vec<String>>`, qui
correspond au vecteur. Ensuite, nous appelons `push` sur la référence mutable
vers le vecteur pour enregistrer le message envoyé pendant le test.

<!--
The last change we have to make is in the assertion: to see how many items are
in the inner vector, we call `borrow` on the `RefCell<Vec<String>>` to get an
immutable reference to the vector.
-->

Le dernier changement que nous devons appliquer est dans la vérification :
pour savoir combien d'éléments sont présents dans le vecteur, nous faisons
appel à `borrow` de `RefCell<Vec<String>>` pour obtenir une référence
immuable vers le vecteur.

<!--
Now that you’ve seen how to use `RefCell<T>`, let’s dig into how it works!
-->

Maintenant que vous avez appris à utiliser `RefCell<T>`, regardons comment il
fonctionne !

<!--
#### Keeping Track of Borrows at Runtime with `RefCell<T>`
-->

#### Suivre les emprunts à l'exécution avec `RefCell<T>`

<!--
When creating immutable and mutable references, we use the `&` and `&mut`
syntax, respectively. With `RefCell<T>`, we use the `borrow` and `borrow_mut`
methods, which are part of the safe API that belongs to `RefCell<T>`. The
`borrow` method returns the smart pointer type `Ref<T>`, and `borrow_mut`
returns the smart pointer type `RefMut<T>`. Both types implement `Deref`, so we
can treat them like regular references.
-->

Lorsque nous créons des références immuables et mutables, nous utilisons
respectivement les syntaxes `&` et `&mut`. Avec `RefCell<T>`, nous utilisons
les méthodes `borrow` et `borrow_mut`, qui font partie de l'API stable de
`RefCell<T>`. La méthode `borrow` retourne un pointeur intelligent du type
`Ref<T>` et `borrow_mut` retourne le pointeur intelligent du type `RefMut<T>`.
Les deux implémentent `Deref`, donc nous pouvons les considérer comme des
références classiques.

<!--
The `RefCell<T>` keeps track of how many `Ref<T>` and `RefMut<T>` smart
pointers are currently active. Every time we call `borrow`, the `RefCell<T>`
increases its count of how many immutable borrows are active. When a `Ref<T>`
value goes out of scope, the count of immutable borrows goes down by one. Just
like the compile-time borrowing rules, `RefCell<T>` lets us have many immutable
borrows or one mutable borrow at any point in time.
-->

Le `RefCell<T>` suit combien de pointeurs intelligents `Ref<T>` et `RefMut<T>`
sont actuellement actifs. A chaque fois que nous faisons appel à `borrow`, le
`RefCell<T>` augmente son compteur du nombre d'emprunts immuables qui existent.
Lorsqu'une valeur `Ref<T>` sort de la portée, le compteur d'emprunts immuables
est décrémenté de un. `RefCell<T>` nous permet d'avoir autant d'emprunts
immuables ou alors un seul emprunt mutable au même moment, exactement comme les
règles d'emprunt à la compilation.

<!--
If we try to violate these rules, rather than getting a compiler error as we
would with references, the implementation of `RefCell<T>` will panic at
runtime. Listing 15-23 shows a modification of the implementation of `send` in
Listing 15-22. We’re deliberately trying to create two mutable borrows active
for the same scope to illustrate that `RefCell<T>` prevents us from doing this
at runtime.
-->

Si nous ne respectons pas ces règles, l'implémentation de `RefCell<T>` va
paniquer à l'exécution plutôt que d'avoir une erreur de compilation comme nous
l'aurions avec des références classiques. L'encart 15-23 nous montre une
modification apportée à l'implémentation de `envoyer` de l'encart 15-22. Nous
essayons délibérément de créer deux emprunts mutables actifs dans la même
portée pour montrer que `RefCell<T>` nous empêche de faire ceci à l'exécution.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,panics
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```
-->

```rust,ignore,panics
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-23/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 15-23: Creating two mutable references in the
same scope to see that `RefCell<T>` will panic</span>
-->

<span class="caption">Encart 15-23 : création de deux références mutables dans
la même portée pour voir si `RefCell<T>` va paniquer</span>

<!--
We create a variable `one_borrow` for the `RefMut<T>` smart pointer returned
from `borrow_mut`. Then we create another mutable borrow in the same way in the
variable `two_borrow`. This makes two mutable references in the same scope,
which isn’t allowed. When we run the tests for our library, the code in Listing
15-23 will compile without any errors, but the test will fail:
-->

Nous créons une variable `premier_emprunt` pour le pointeur intelligent
`RefMut<T>` retourné par `borrow_mut`. Ensuite nous créons un autre emprunt de
la même manière, qui s'appelle `second_emprunt`. Cela fait deux références
mutables dans la même portée, ce qui n'est pas autorisé. Lorsque nous lançons
les tests sur notre bibliothèque, le code de l'encart 15-23 va se compiler
sans erreur, mais les tests vont échouer :

<!--
```console
{{#include ../listings-sources/ch15-smart-pointers/listing-15-23/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-23/output.txt}}
```

<!--
Notice that the code panicked with the message `already borrowed:
BorrowMutError`. This is how `RefCell<T>` handles violations of the borrowing
rules at runtime.
-->

Remarquez que le code a paniqué avec le message
`already borrowed: BorrowMutError` (NdT : `déjà emprunté`). C'est ainsi que
`RefCell<T>` gère les violations des règles d'emprunt à l'exécution.

<!--
Catching borrowing errors at runtime rather than compile time means that you
would find a mistake in your code later in the development process and possibly
not until your code was deployed to production. Also, your code would incur a
small runtime performance penalty as a result of keeping track of the borrows
at runtime rather than compile time. However, using `RefCell<T>` makes it
possible to write a mock object that can modify itself to keep track of the
messages it has seen while you’re using it in a context where only immutable
values are allowed. You can use `RefCell<T>` despite its trade-offs to get more
functionality than regular references provide.
-->

La détection des erreurs d'emprunt à l'exécution plutôt qu'à la compilation
signifie que vous allez trouver une erreur dans votre code plus tard dans le
processus de développement et peut-être même pas avant que votre code soit
déployé en production. De plus, votre code va subir une petite perte de
performances à l'exécution en raison du contrôle des emprunts à l'exécution
plutôt qu'à la compilation. Cependant, l'utilisation de `RefCell<T>` rend
possible l'écriture d'un mock object qui peut se modifier lui-même afin
d'enregistrer les messages qu'il a vu passer alors que vous l'utilisez dans un
contexte où seules les valeurs immuables sont permises. Vous pouvez utiliser
`RefCell<T>` malgré ses inconvénients pour obtenir plus de fonctionnalités
que celles qu'offre une référence classique.

<!--
### Having Multiple Owners of Mutable Data by Combining `Rc<T>` and `RefCell<T>`
-->

### Permettre plusieurs propriétaires de données mutables en combinant `Rc<T>` et `RefCell<T>`

<!--
A common way to use `RefCell<T>` is in combination with `Rc<T>`. Recall that
`Rc<T>` lets you have multiple owners of some data, but it only gives immutable
access to that data. If you have an `Rc<T>` that holds a `RefCell<T>`, you can
get a value that can have multiple owners *and* that you can mutate!
-->

Il est courant d'utiliser `RefCell<T>` en tandem avec `Rc<T>`. Rappelez-vous
que `Rc<T>` vous permet d'avoir plusieurs propriétaires d'une même donnée, mais
qu'il vous donne seulement un accès immuable à cette donnée. Si vous avez un
`Rc<T>` qui contient un `RefCell<T>`, vous pouvez obtenir une valeur qui peut
avoir plusieurs propriétaires *et* que vous pouvez modifier !

<!--
For example, recall the cons list example in Listing 15-18 where we used
`Rc<T>` to allow multiple lists to share ownership of another list. Because
`Rc<T>` holds only immutable values, we can’t change any of the values in the
list once we’ve created them. Let’s add in `RefCell<T>` to gain the ability to
change the values in the lists. Listing 15-24 shows that by using a
`RefCell<T>` in the `Cons` definition, we can modify the value stored in all
the lists:
-->

Souvenez-vous de l'exemple de la liste de construction de l'encart 15-18 où nous
avions utilisé `Rc<T>` pour permettre d'avoir plusieurs listes de se partager la
possession d'une autre liste. Comme `Rc<T>` stocke seulement des valeurs
immuables, nous ne pouvons changer aucune valeur dans la liste une fois que
nous l'avons créé. Ajoutons un `RefCell<T>` pour pouvoir changer les valeurs
dans les listes. L'encart 15-24 nous montre ceci en ajoutant un `RefCell<T>`
dans la définition de `Cons`, nous pouvons ainsi modifier les valeurs stockées
dans n'importe quelle liste :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch15-smart-pointers/listing-15-24/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch15-smart-pointers/listing-15-24/src/main.rs}}
```

<!--
<span class="caption">Listing 15-24: Using `Rc<RefCell<i32>>` to create a
`List` that we can mutate</span>
-->

<span class="caption">Encart 15-24 : utilisation de `Rc<RefCell<i32>>` pour
créer une `List` que nous pouvons modifier</span>

<!--
We create a value that is an instance of `Rc<RefCell<i32>>` and store it in a
variable named `value` so we can access it directly later. Then we create a
`List` in `a` with a `Cons` variant that holds `value`. We need to clone
`value` so both `a` and `value` have ownership of the inner `5` value rather
than transferring ownership from `value` to `a` or having `a` borrow from
`value`.
-->

Nous créons une valeur qui est une instance de `Rc<RefCell<i32>>` et nous la
stockons dans une variable `valeur` afin que nous puissions y avoir accès plus
tard. Ensuite, nous créons une `List` dans `a` avec une variante de `Cons` qui
utilise `valeur`. Nous devons utiliser clone sur `valeur` afin que `a` et
`valeur` soient toutes les deux propriétaires de la valeur interne `5` plutôt
que d'avoir à transférer la possession de `valeur` à `a` ou avoir `a` qui
emprunte `valeur`.

<!--
We wrap the list `a` in an `Rc<T>` so when we create lists `b` and `c`, they
can both refer to `a`, which is what we did in Listing 15-18.
-->

Nous insérons la liste `a` dans un `Rc<T>` pour que lorsque nous créons `b` et
`c`, elles puissent toutes les deux utiliser `a`, ce que nous avions déjà fait
dans l'encart 15-18.

<!--
After we’ve created the lists in `a`, `b`, and `c`, we add 10 to the value in
`value`. We do this by calling `borrow_mut` on `value`, which uses the
automatic dereferencing feature we discussed in Chapter 5 (see the section
[“Where’s the `->` Operator?”][wheres-the---operator]<!-- ignore -- >) to
dereference the `Rc<T>` to the inner `RefCell<T>` value. The `borrow_mut`
method returns a `RefMut<T>` smart pointer, and we use the dereference operator
on it and change the inner value.
-->

Après avoir créé les listes dans `a`, `b`, et `c`, nous ajoutons 10 à la valeur
dans `valeur`. Nous faisons cela en appelant `borrow_mut` sur `valeur`, ce qui
utilise la fonctionnalité de déréférencement automatique que nous avons vu au
chapitre 5 (voir la section
[“Où est l'opérateur -> ?”][wheres-the---operator]<!-- ignore -->) pour
déréférencer le `Rc<T>` dans la valeur interne `RefCell<T>`. La méthode
`borrow_mut` retourne un pointeur intelligent `RefMut<T>`, et nous utilisons
l'opérateur de déréférencement sur lui pour changer sa valeur interne.

<!--
When we print `a`, `b`, and `c`, we can see that they all have the modified
value of 15 rather than 5:
-->

Lorsque nous affichons `a`, `b` et `c`, nous pouvons constater qu'elles ont
toutes la valeur modifiée de 15 au lieu de 5 :

<!--
```console
{{#include ../listings-sources/ch15-smart-pointers/listing-15-24/output.txt}}
```
-->

```console
{{#include ../listings/ch15-smart-pointers/listing-15-24/output.txt}}
```

<!--
This technique is pretty neat! By using `RefCell<T>`, we have an outwardly
immutable `List` value. But we can use the methods on `RefCell<T>` that provide
access to its interior mutability so we can modify our data when we need to.
The runtime checks of the borrowing rules protect us from data races, and it’s
sometimes worth trading a bit of speed for this flexibility in our data
structures.
-->

Cette technique est plutôt ingénieuse ! En utilisant `RefCell<T>`, nous avons
une valeur `List` qui est immuable de l'extérieur. Mais nous pouvons utiliser
les méthodes de `RefCell<T>` qui nous donne accès à sa mutabilité interne afin
que nous puissions modifier notre donnée lorsque nous en avons besoin. Les
vérifications des règles d'emprunt à l'exécution nous protègent des accès
concurrents, et il est parfois intéressant de sacrifier un peu de vitesse pour
cette flexibilité dans nos structures de données.


<!--
The standard library has other types that provide interior mutability, such as
`Cell<T>`, which is similar except that instead of giving references to the
inner value, the value is copied in and out of the `Cell<T>`. There’s also
`Mutex<T>`, which offers interior mutability that’s safe to use across threads;
we’ll discuss its use in Chapter 16. Check out the standard library docs for
more details on the differences between these types.
-->

La bibliothèque standard a d'autres types qui fournissent de la mutabilité
interne, comme `Cell<T>`, qui est similaire sauf qu'au lieu de fournir des
références à la valeur interne, la valeur est copiée à l'intérieur et à
l'extérieur du `Cell<T>`. Il existe aussi `Mutex<T>` qui offre de la mutabilité
interne qui est sécurisée pour une utilisation partagée entre plusieures
tâches ; nous allons voir son utilisation au chapitre 16. Plongez-vous dans la
documentation de la bibliothèque standard pour plus de détails entre ces
différents types.

<!--
[wheres-the---operator]: ch05-03-method-syntax.html#wheres-the---operator
-->

[wheres-the---operator]: ch05-03-method-syntax.html#o%C3%B9-est-lop%C3%A9rateur---
