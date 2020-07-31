> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/199).

<!--
## Shared-State Concurrency
-->

## Le partage d'état en concurrence

<!--
Message passing is a fine way of handling concurrency, but it’s not the only
one. Consider this part of the slogan from the Go language documentation again:
“do not communicate by sharing memory.”
-->

L'envoi de messages est un assez bon moyen de gestion de la concurrence, mais il
n'y a pas qu'un seul. Repensons à cette partie du slogan de la documentation du
langage Go : “ne communiquez pas en partageant la mémoire”.

<!--
What would communicating by sharing memory look like? In addition, why would
message-passing enthusiasts not use it and do the opposite instead?
-->

A quoi ressemble le partage de mémoire pour communiquer ? De plus, pourquoi les
partisans de l'envoi de messages ne devraient pas l'utiliser et faire plutôt
le contraire ?

<!--
In a way, channels in any programming language are similar to single ownership,
because once you transfer a value down a channel, you should no longer use that
value. Shared memory concurrency is like multiple ownership: multiple threads
can access the same memory location at the same time. As you saw in Chapter 15,
where smart pointers made multiple ownership possible, multiple ownership can
add complexity because these different owners need managing. Rust’s type system
and ownership rules greatly assist in getting this management correct. For an
example, let’s look at mutexes, one of the more common concurrency primitives
for shared memory.
-->

De manière générale, les canaux dans les langages de programmation ressemble à
la possession unique, car une fois que vous avez transféré une valeur dans un
canal, vous ne pouvez plus utiliser cette valeur. Le partage de mémoire en
concurrence est comme de la possession multiple : plusieurs tâches peuvent
accéder au même endroit de la mémoire en même temps. Comme vous l'avez vu au
chapitre 15, dans lequel les pointeurs intelligents rendent possible la
possession multiple, la possession multiple peut rajouter de la complexité car
ces différents propriétaires ont besoin d'être gérés. Le système de type de Rust
et les règles de possession aident beaucoup à les gérer correctement. Par
exemple, découvrons les mutex, une des primitives les plus courantes pour
partager la mémoire.

<!--
### Using Mutexes to Allow Access to Data from One Thread at a Time
-->

### Utiliser les mutex pour permettre l'accès à la donnée à une seule tâche à la fois

<!--
*Mutex* is an abbreviation for *mutual exclusion*, as in, a mutex allows only
one thread to access some data at any given time. To access the data in a
mutex, a thread must first signal that it wants access by asking to acquire the
mutex’s *lock*. The lock is a data structure that is part of the mutex that
keeps track of who currently has exclusive access to the data. Therefore, the
mutex is described as *guarding* the data it holds via the locking system.
-->

*Mutex* est une abréviation pour *mutual exclusion*, ce qui veut dire qu'un
mutex ne permet qu'à une seule tâche d'accéder à une donnée en même temps. Pour
accéder à la donnée dans un mutex, une tâche doit d'abord signaler qu'elle
souhaite y accéder en demandant l'obtention du *verrou* du mutex. Le verrou est
une structure de donnée qui fait partie du mutex qui assure le suivi de qui a
actuellement accès à la donnée. Par conséquent, le mutex est qualifié de
*gardien* de la donnée qu'il renferme via le système de verrou.

<!--
Mutexes have a reputation for being difficult to use because you have to
remember two rules:
-->

Les mutex ont la réputation d'être difficiles à utiliser car vous devez veiller
à deux règles :

<!--
* You must attempt to acquire the lock before using the data.
* When you’re done with the data that the mutex guards, you must unlock the
  data so other threads can acquire the lock.
-->

* Vous devez obtenir le verrou avant d'utiliser la donnée.
* Lorsque vous avez fini avec la donnée que le mutex garde, vous devez
  déverrouiller la donnée afin que d'autres tâches puissent obtenir le verrou.

<!--
For a real-world metaphor for a mutex, imagine a panel discussion at a
conference with only one microphone. Before a panelist can speak, they have to
ask or signal that they want to use the microphone. When they get the
microphone, they can talk for as long as they want to and then hand the
microphone to the next panelist who requests to speak. If a panelist forgets to
hand the microphone off when they’re finished with it, no one else is able to
speak. If management of the shared microphone goes wrong, the panel won’t work
as planned!
-->

Pour faire une métaphore de la vie courante d'un mutex, imaginez une table ronde
lors d'une conférence avec un seul microphone. Avant qu'un participant puisse
parler, il doit demander ou signaler qu'il veut utiliser le micro. Lorsqu'il
obtient le micro, il peut parler aussi longtemps qu'il le souhaite et ensuite
passer le micro au prochain participant qui a demandé à pouvoir parler. Si un
participant oublie de rendre le micro après avoir fini de parler, personne
d'autre ne peut parler. Si la gestion du micro partagé se passe mal, la table
ronde ne fonctionnera pas comme prévu !

<!--
Management of mutexes can be incredibly tricky to get right, which is why so
many people are enthusiastic about channels. However, thanks to Rust’s type
system and ownership rules, you can’t get locking and unlocking wrong.
-->

La gestion des mutex peut devenir incroyable compliquée, c'est pourquoi tant de
personnes sont partisanes des canaux. Cependant, grâce au système de type de
Rust et aux règles de possession, vous ne pouvez pas vous tromper dans le
verrouillage et déverrouillage.

<!--
#### The API of `Mutex<T>`
-->

#### L'API des `Mutex<T>`

<!--
As an example of how to use a mutex, let’s start by using a mutex in a
single-threaded context, as shown in Listing 16-12:
-->

Pour illustrer l'utilisation d'un mutex, commençons par utiliser un mutex dans
le contexte d'une seule tâche, comme dans l'encart 16-12 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-12/src/main.rs}}
```

<!--
<span class="caption">Listing 16-12: Exploring the API of `Mutex<T>` in a
single-threaded context for simplicity</span>
-->

<span class="caption">Encart 16-12 : découverte de l'API de `Mutex<T>` dans le
contexte d'une seule tâche pour raison de simplicité</span>

<!--
As with many types, we create a `Mutex<T>` using the associated function `new`.
To access the data inside the mutex, we use the `lock` method to acquire the
lock. This call will block the current thread so it can’t do any work until
it’s our turn to have the lock.
-->

Comme avec beaucoup de types, nous créons un `Mutex<T>` en utilisant la
fonction associée `new`. Pour accéder à la donnée dans le mutex, nous utilisons
la méthode `lock` pour obtenir le verrou. Cela va bloquer la tâche courante,
donc elle ne s'exécutera plus tant que ce n'est à notre tour d'avoir le verrou.

<!--
The call to `lock` would fail if another thread holding the lock panicked. In
that case, no one would ever be able to get the lock, so we’ve chosen to
`unwrap` and have this thread panic if we’re in that situation.
-->

L'appel à `lock` va échouer dans le cas où une autre tâche qui avait le verrou
paniquerait. Dans ce cas, personne ne pourra obtenir le verrou, donc nous avons
choisi d'utiliser `unwrap` pour faire en sorte que cette tâche panique si elle
est dans cette situation.

<!--
After we’ve acquired the lock, we can treat the return value, named `num` in
this case, as a mutable reference to the data inside. The type system ensures
that we acquire a lock before using the value in `m`: `Mutex<i32>` is not an
`i32`, so we *must* acquire the lock to be able to use the `i32` value. We
can’t forget; the type system won’t let us access the inner `i32` otherwise.
-->

Après avoir obtenu le verrou, nous pouvons utiliser la valeur de retour comme
une référence mutable vers la donnée, qui s'appellera `nombre` dans ce cas. Le
système de type s'assure que nous obtenons le verrou avant d'utiliser la valeur
présente dans `m` : le `Mutex<i32>` n'est pas un `i32`, donc nous *devons*
obtenir le verrou pour pouvoir utiliser la valeur `i32`. Nous ne pouvons pas
l'oublier ; le système de type ne nous laissera pas accéder au `i32` à
l'intérieur de toute façon.

<!--
As you might suspect, `Mutex<T>` is a smart pointer. More accurately, the call
to `lock` *returns* a smart pointer called `MutexGuard`, wrapped in a
`LockResult` that we handled with the call to `unwrap`. The `MutexGuard` smart
pointer implements `Deref` to point at our inner data; the smart pointer also
has a `Drop` implementation that releases the lock automatically when a
`MutexGuard` goes out of scope, which happens at the end of the inner scope in
Listing 16-12. As a result, we don’t risk forgetting to release the lock and
blocking the mutex from being used by other threads because the lock release
happens automatically.
-->

Comme vous pouvez vous en douter, `Mutex<T>` est un pointeur intelligent. Plus
précisément, l'appel à `lock` *retourne* un pointeur intelligent `MutexGuard`,
intégré dans un `LockResult` que nous avons géré en faisant appel à `unwrap`.
Le pointeur intelligent `MutexGuard` implémente `Deref` pour pouvoir pointer
sur la donnée interne ; ce pointeur intelligent implémente aussi `Drop` qui
libère le verrou automatiquement lorsqu'un `MutexGuard` sort de la portée, ce
qui arrive à la fin de la portée interne dans l'encart 16-12. Au final, nous ne
risquons d'oublier de rendre le verrou et ainsi bloquer l'utilisation du mutex
par les autres tâches car la libération du verrou se produit automatiquement.

<!--
After dropping the lock, we can print the mutex value and see that we were able
to change the inner `i32` to 6.
-->

Après avoir libéré le verrou, nous pouvons afficher la valeur dans le mutex et
constater que nous avons pu changer la valeur interne du `i32` à `6`.

<!--
#### Sharing a `Mutex<T>` Between Multiple Threads
-->

#### Partager un `Mutex<T>` entre plusieurs tâches

<!--
Now, let’s try to share a value between multiple threads using `Mutex<T>`.
We’ll spin up 10 threads and have them each increment a counter value by 1, so
the counter goes from 0 to 10. The next example in Listing 16-13 will have
a compiler error, and we’ll use that error to learn more about using
`Mutex<T>` and how Rust helps us use it correctly.
-->

Essayons maintenant de partager une valeur entre plusieurs tâches en utilisant
`Mutex<T>`. Nous allons faire fonctionner 10 tâches et faire en sorte que
chacune augmente la valeur du compteur de 1, donc le compteur va passer de 0
à 10. Le prochain exemple dans l'encart 16-13 débouchera sur une erreur de
compilation, et nous allons utiliser cette erreur pour en apprendre plus sur
l'utilisation de `Mutex<T>` et sur comment Rust nous aide à l'utiliser
correctement.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-13/src/main.rs}}
```

<!--
<span class="caption">Listing 16-13: Ten threads each increment a counter
guarded by a `Mutex<T>`</span>
-->

<span class="caption">Encart 16-13 : dix tâches qui augmentent chacune un
compteur gardé par un `Mutex<T>`</span>

<!--
We create a `counter` variable to hold an `i32` inside a `Mutex<T>`, as we
did in Listing 16-12. Next, we create 10 threads by iterating over a range
of numbers. We use `thread::spawn` and give all the threads the same closure,
one that moves the counter into the thread, acquires a lock on the `Mutex<T>`
by calling the `lock` method, and then adds 1 to the value in the mutex. When a
thread finishes running its closure, `num` will go out of scope and release the
lock so another thread can acquire it.
-->

Nous avons créé une variable `compteur` pour stocker un `i32` dans un
`Mutex<T>`, comme nous l'avons fait dans l'encart 16-12. Ensuite, nous créons
10 tâches en itérant sur un intervalle de nombres. Nous utilisons
`thread::spawn` et nous donnons à toutes les tâches la même fermeture, qui
déplace le compteur dans la tâche, obtient le verrou sur le `Mutex<T>` en
faisant appel à la méthode `lock`, et on ajoute ensuite 1 à la valeur présente
dans le mutex. Lorsqu'une tâche finit d'exécuter sa fermeture, `nombre` va
sortir de la portée et va libérer le verrou afin qu'une autre tâche puisse
l'obtenir.

<!--
In the main thread, we collect all the join handles. Then, as we did in Listing
16-2, we call `join` on each handle to make sure all the threads finish. At
that point, the main thread will acquire the lock and print the result of this
program.
-->

Dans la tâche principale, nous collectons tous les manipulateurs. Ensuite,
comme nous l'avions fait dans l'encart 16-2, nous faisons appel à `join` sur
chaque manipulateur pour s'assurer que toutes les tâches ont fini. Une fois que
c'est le cas, la tâche principale va obtenir le verrou et afficher le résultat
de ce programme.

<!--
We hinted that this example wouldn’t compile. Now let’s find out why!
-->

Nous avions mentionné que cet exemple ne se compilerait pas. Découvrons
maintenant pourquoi !

<!--
```console
{{#include ../listings-sources/ch16-fearless-concurrency/listing-16-13/output.txt}}
```
-->

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-13/output.txt}}
```

<!--
The error message states that the `counter` value was moved in the previous
iteration of the loop. So Rust is telling us that we can’t move the ownership
of lock `counter` into multiple threads. Let’s fix the compiler error with a
multiple-ownership method we discussed in Chapter 15.
-->

Le message d'erreur signale que la valeur `compteur` a été déplacée dans
l'itération précédente de la boucle. Donc Rust nous explique qu'il ne peut
pas déplacer la possession du verrou de `compteur` dans plusieurs tâches.
Corrigeons cette erreur de compilation avec une méthode pour avoir plusieurs
propriétaires que nous avons vu au chapitre 15.

<!--
#### Multiple Ownership with Multiple Threads
-->

#### Plusieurs propriétaires avec plusieurs tâches

<!--
In Chapter 15, we gave a value multiple owners by using the smart pointer
`Rc<T>` to create a reference counted value. Let’s do the same here and see
what happens. We’ll wrap the `Mutex<T>` in `Rc<T>` in Listing 16-14 and clone
the `Rc<T>` before moving ownership to the thread. Now that we’ve seen the
errors, we’ll also switch back to using the `for` loop, and we’ll keep the
`move` keyword with the closure.
-->

Dans le chapitre 15, nous avons assigné plusieurs propriétaires à une valeur
en utilisant le pointeur intelligent `Rc<T>` pour créer un compteur de
référence. Faisons la même chose ici et voyons ce qui se passe. Nous allons
intégrer le `Mutex<T>` dans un `Rc<T>` dans l'encart 16-14 et cloner le `Rc<T>`
avant de déplacer sa possession à la tâche. Maintenant que nous avons vu les
erreurs, nous allons aussi revenir à l'utilisation de la boucle `for` et nous
allons garder le mot-clé `move` dans la fermeture.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-14/src/main.rs}}
```

<!--
<span class="caption">Listing 16-14: Attempting to use `Rc<T>` to allow
multiple threads to own the `Mutex<T>`</span>
-->

<span class="caption">Encart 16-14 : tentative d'utilisation d'un `Rc<T>` pour
nous permettre d'utiliser plusieurs tâches qui posséderont le `Mutex<T>`</span>

<!--
Once again, we compile and get... different errors! The compiler is teaching us
a lot.
-->

A nouveau, nous compilons et nous obtenons ... une erreur différente ! Le
compilateur nous en apprend beaucoup.

<!--
```console
{{#include ../listings-sources/ch16-fearless-concurrency/listing-16-14/output.txt}}
```
-->

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-14/output.txt}}
```

<!--
Wow, that error message is very wordy! Here’s the important part to focus
on: `` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. The compiler
is also telling us the reason why: ``the trait `Send` is not implemented for
`Rc<Mutex<i32>>` ``. We’ll talk about `Send` in the next section: it’s one of
the traits that ensures the types we use with threads are meant for use in
concurrent situations.
-->

Ouah, ce message d'erreur est très bavard ! Voici la partie la plus importante
sur laquelle se concentrer :
`` `Rc<Mutex<i32>>` cannot be sent between threads safely ``. Le compilateur
nous indique aussi pour quelle raison :
``the trait `Send` is not implemented for `Rc<Mutex<i32>>` ``. Nous allons voir
`Send` dans la prochaine section : c'est l'un des traits qui garantissent que le
type que nous utilisons avec les tâches est prévu pour être utilisé dans des
situations de concurrence.

<!--
Unfortunately, `Rc<T>` is not safe to share across threads. When `Rc<T>`
manages the reference count, it adds to the count for each call to `clone` and
subtracts from the count when each clone is dropped. But it doesn’t use any
concurrency primitives to make sure that changes to the count can’t be
interrupted by another thread. This could lead to wrong counts—subtle bugs that
could in turn lead to memory leaks or a value being dropped before we’re done
with it. What we need is a type exactly like `Rc<T>` but one that makes changes
to the reference count in a thread-safe way.
-->

Malheureusement, `Rc<T>` n'est pas sûr pour l'utilisation entre des tâches.
Lorsque `Rc<T>` gère le compteur de références, il incrémente le compteur autant
de fois que nous avons fait appel à `clone` et décrémente le compteur à chaque
fois qu'un clone est libéré. Mais il n'utilise pas de primitives de concurrence
pour s'assurer que les changements faits au compteur ne peuvent pas être
interrompus par une autre tâche. Cela pourrait provoquer subtilement des bogues
à cause d'une mauvaise gestion du compteur, qui pourraient provoquer des fuites
de mémoire ou faire en sorte qu'une valeur soit libérée avant que nous ayons
fini de l'utiliser. Nous avons besoin d'un type exactement comme `Rc<T>` mais
qui procède aux changements du compteur de références de manière sûr dans des
situations concurrentes.

<!--
#### Atomic Reference Counting with `Arc<T>`
-->

#### Compteur de référence atomique avec `Arc<T>`

<!--
Fortunately, `Arc<T>` *is* a type like `Rc<T>` that is safe to use in
concurrent situations. The *a* stands for *atomic*, meaning it’s an *atomically
reference counted* type. Atomics are an additional kind of concurrency
primitive that we won’t cover in detail here: see the standard library
documentation for `std::sync::atomic` for more details. At this point, you just
need to know that atomics work like primitive types but are safe to share
across threads.
-->

Heureusement, `Arc<T>` *est* un type comme `Rc<T>` qui est sûr lors de
situations concurrentes. Le *A* signifie *atomique*, ce qui signifie que c'est
le type *compteur de références atomique*. L'atome est une sorte de primitive
concurrente que nous n'allons pas aborder en détails ici : rendez-vous dans la
documentation de la bibliothèque standard sur `std::sync::atomic` pour en
savoir plus. Pour le moment, vous avez juste besoin de retenir que les atomes
fonctionnent comme les types primitifs mais qui son sûrs pour l'échange entre
les tâches.

<!--
You might then wonder why all primitive types aren’t atomic and why standard
library types aren’t implemented to use `Arc<T>` by default. The reason is that
thread safety comes with a performance penalty that you only want to pay when
you really need to. If you’re just performing operations on values within a
single thread, your code can run faster if it doesn’t have to enforce the
guarantees atomics provide.
-->

Vous vous demandez pourquoi tous les types primitifs ne sont pas atomiques et
pourquoi les types de la bibliothèque standard ne sont pas implémentés en
utilisant `Arc<T>` par défaut. La raison à cela est que la sécurité entre les
tâches a un coût sur les performances que vous n'êtes prêt à payer que lorsque
vous en avez besoin. Si vous procédez à des opérations sur des valeurs
uniquement dans une seule tâche, votre code va s'exécuter plus vite car il n'a
pas besoin de garantir de que les atomes fournissent.

<!--
Let’s return to our example: `Arc<T>` and `Rc<T>` have the same API, so we fix
our program by changing the `use` line, the call to `new`, and the call to
`clone`. The code in Listing 16-15 will finally compile and run:
-->

Retournons à notre exemple : `Arc<T>` et `Rc<T>` ont la même API, donc
corrigeons notre programme en changeant la ligne `use`, l'appel à `new`, et
l'appel à `clone`. Le code dans l'encart 16-15 va finalement se compiler et
s'exécuter :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-15/src/main.rs}}
```

<!--
<span class="caption">Listing 16-15: Using an `Arc<T>` to wrap the `Mutex<T>`
to be able to share ownership across multiple threads</span>
-->

<span class="caption">Encart 16-15 : utilisation d'un `Arc<T>` pour englober
le `Mutex<T>` afin de partager la possession entre plusieurs tâches</span>

<!--
This code will print the following:
-->

Ce code va finalement afficher ceci :

<!--
<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -- >
-->

<!--
```text
Result: 10
```
-->

```text
Resultat : 10
```

<!--
We did it! We counted from 0 to 10, which may not seem very impressive, but it
did teach us a lot about `Mutex<T>` and thread safety. You could also use this
program’s structure to do more complicated operations than just incrementing a
counter. Using this strategy, you can divide a calculation into independent
parts, split those parts across threads, and then use a `Mutex<T>` to have each
thread update the final result with its part.
-->

Nous y sommes arrivés ! Nous avons compté de 0 à 10, ce qui ne semble pas très
impressionnant, mais cela nous a appris beaucoup sur `Mutex<T>` et la sécurité
entre les tâches. Vous pouvez aussi utiliser cette structure de programme pour
procéder à des opérations plus complexes que simplement incrémenter un
compteur. En utilisant cette stratégie, vous pouvez diviser un calcul en
différentes parties, répartir ces parties sur des tâches, et ensuite utiliser
un `Mutex<T>` pour faire en sorte que chaque tâche mette à jour le résultat
final avec sa propre partie.

<!--
### Similarities Between `RefCell<T>`/`Rc<T>` and `Mutex<T>`/`Arc<T>`
-->

### Similarités entre `RefCell<T>`/`Rc<T>` et `Mutex<T>`/`Arc<T>`

<!--
You might have noticed that `counter` is immutable but we could get a mutable
reference to the value inside it; this means `Mutex<T>` provides interior
mutability, as the `Cell` family does. In the same way we used `RefCell<T>` in
Chapter 15 to allow us to mutate contents inside an `Rc<T>`, we use `Mutex<T>`
to mutate contents inside an `Arc<T>`.
-->

Vous avez peut-être constaté que `compteur` est immuable mais que nous pouvons
obtenir une référence mutable vers la valeur qu'il renferme ; cela signifie que
`Mutex<T>` a une mutabilité interne, comme le fait la famille des `Cell`. De la
même manière que nous avons utilisé `RefCell<T>` au chapitre 15 pour nous
permettre de changer le contenu dans un `Rc<T>`, nous utilisons `Mutex<T>` pour
modifier le contenu d'un `Arc<T>`.

<!--
Another detail to note is that Rust can’t protect you from all kinds of logic
errors when you use `Mutex<T>`. Recall in Chapter 15 that using `Rc<T>` came
with the risk of creating reference cycles, where two `Rc<T>` values refer to
each other, causing memory leaks. Similarly, `Mutex<T>` comes with the risk of
creating *deadlocks*. These occur when an operation needs to lock two resources
and two threads have each acquired one of the locks, causing them to wait for
each other forever. If you’re interested in deadlocks, try creating a Rust
program that has a deadlock; then research deadlock mitigation strategies for
mutexes in any language and have a go at implementing them in Rust. The
standard library API documentation for `Mutex<T>` and `MutexGuard` offers
useful information.
-->

Un autre détail à souligner est Rust ne peut pas vous protéger de tous les
genres d'erreurs de logique lorsque vous utilisez `Mutex<T>`. Souvenez-vous
que le chapitre 15 utilisait `Rc<T>` avec le risque de créer des boucles de
références, dans lesquelles deux valeurs `Rc<T>` se référeraient l'une à
l'autre, ce qui provoquait des fuites de mémoire. De la même manière,
l'utilisation de `Mutex<T>` risque de créer des *interblocages*. Cela se produit
lorsqu'une opération nécessite de verrouiller deux ressources et que deux tâches
ont chacune un des deux verrous, ce qui fait qu'elles s'attendent mutuellement
pour toujours. Si vous êtes intéressés par les interblocages, essayez de créer
un programme Rust qui a un interblocage ; recherchez ensuite des stratégies pour
pallier aux interblocages dans n'importe quel langage et implémentez-les en
Rust. La documentation de l'API de la bibliothèque standard pour `Mutex<T>` et
`MutexGuard` offre des informations précieuses à ce sujet.

<!--
We’ll round out this chapter by talking about the `Send` and `Sync` traits and
how we can use them with custom types.
-->

Nous allons terminer ce chapitre en parlant des traits `Send` et `Sync` et
voir comment nous pouvons les utiliser sur des types personnalisés.
