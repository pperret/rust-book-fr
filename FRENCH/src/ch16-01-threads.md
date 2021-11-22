> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/197).

<!--
## Using Threads to Run Code Simultaneously
-->

## Utiliser les tâches pour exécuter simultanément du code

<!--
In most current operating systems, an executed program’s code is run in a
*process*, and the operating system manages multiple processes at once. Within
your program, you can also have independent parts that run simultaneously. The
features that run these independent parts are called *threads*.
-->

Dans la plupart des systèmes d'exploitation actuels, le code d'un programme
est exécuté dans un *processus*, et le système d'exploitation gère plusieurs
processus à la fois. Dans votre programme, vous pouvez vous aussi avoir des
parties indépendantes qui s'exécutent simultanément. Les éléments qui font
fonctionner ces parties indépendantes sont appelés les *tâches*.

<!--
Splitting the computation in your program into multiple threads can improve
performance because the program does multiple tasks at the same time, but it
also adds complexity. Because threads can run simultaneously, there’s no
inherent guarantee about the order in which parts of your code on different
threads will run. This can lead to problems, such as:
-->

Le découpage des calculs de votre programme dans plusieurs tâches peut
améliorer sa performance car le programme fait plusieurs choses à la fois, mais
cela rajoute aussi de la complexité. Comme les tâches peuvent s'exécuter de
manière simultanée, il n'y a pas de garantie absolue sur l'ordre d'exécution
de vos différentes parties de votre code. Cela peut poser des problèmes,
comme :

<!--
* Race conditions, where threads are accessing data or resources in an
  inconsistent order
* Deadlocks, where two threads are waiting for each other to finish using a
  resource the other thread has, preventing both threads from continuing
* Bugs that happen only in certain situations and are hard to reproduce and fix
  reliably
-->

* Les situations de concurrence, durant lesquelles les tâches accèdent à une
  donnée ou ressource dans un ordre incohérent
* Des interblocages, durant lesquels deux tâches attendent mutuellement que
  l'autre finisse d'utiliser une ressource que l'autre tâche utilise, bloquant
  la progression des deux tâches
* Des bogues qui surgissent uniquement dans certaines situations et qui sont
  difficiles à reproduire et corriger durablement

<!--
Rust attempts to mitigate the negative effects of using threads, but
programming in a multithreaded context still takes careful thought and requires
a code structure that is different from that in programs running in a single
thread.
-->

Rust cherche à atténuer les effets indésirables de l'utilisation des tâches,
mais le développement dans un contexte multitâches exige toujours une attention
particulière et nécessite une structure de code différente de celle pour des
programmes qui s'exécutent dans une seule tâche.

<!--
Programming languages implement threads in a few different ways. Many operating
systems provide an API for creating new threads. This model where a language
calls the operating system APIs to create threads is sometimes called *1:1*,
meaning one operating system thread per one language thread.
-->

Les langages de programmation implémentent les tâches de différentes manières.
De nombreux systèmes d'exploitation offrent des API pour créer des nouvelles
tâches. L'appel à cet API du système d'exploitation pour créer des tâches par
un langage est parfois qualifié de *1:1*, ce qui signifie une tâche du système
d'exploitation par tâche dans le langage de programmation.

<!--
Many programming languages provide their own special implementation of threads.
Programming language-provided threads are known as *green* threads, and
languages that use these green threads will execute them in the context of a
different number of operating system threads. For this reason, the
green-threaded model is called the *M:N* model: there are `M` green threads per
`N` operating system threads, where `M` and `N` are not necessarily the same
number.
-->

De nombreux langages de programmation fournissent leur propre implémentation
spéciale des tâches. Les tâches fournies par un langage de programmation
s'appelle une tâche *virtuelle*, et les langages qui utilisent ces tâches
virtuelles vont les exécuter dans différentes tâches du système d'exploitation.
C'est pourquoi le modèle des tâches virtuelles est appelé modèle *M:N* : il y a
`M` tâches virtuelles pour `N` tâches du système d'exploitation, dans lequel
`M` et `N` ne sont pas nécessairement le même nombre.

<!--
Each model has its own advantages and trade-offs, and the trade-off most
important to Rust is runtime support. *Runtime* is a confusing term and can
have different meanings in different contexts.
-->

Chaque modèle a ses propres avantages et compromis, et le compromis le plus
important pour Rust est la prise en charge de l'environnement d'exécution.
*Environnement d'exécution* est un terme qui peut prêter à confusion et avoir
différentes significations dans différents contextes.

<!--
In this context, by *runtime* we mean code that is included by the language in
every binary. This code can be large or small depending on the language, but
every non-assembly language will have some amount of runtime code. For that
reason, colloquially when people say a language has “no runtime,” they often
mean “small runtime.” Smaller runtimes have fewer features but have the
advantage of resulting in smaller binaries, which make it easier to combine the
language with other languages in more contexts. Although many languages are
okay with increasing the runtime size in exchange for more features, Rust needs
to have nearly no runtime and cannot compromise on being able to call into C to
maintain performance.
-->

Dans ce contexte, lorsque nous parlons *d'environnement exécution*, nous
entendons le code qui est intégré par le langage dans chaque binaire. Ce code
peut être plus ou moins vaste en fonction du langage, mais chaque langage non
assembleur aura une certaine quantité de code d'environnement exécution. Pour
cette raison, lorsque les gens disent couramment d'un langage n'a pas
“d'environnement d'exécution”, ils entendent très souvent “faible environnement
d'exécution”. Les faibles environnements d'exécution ont moins de
fonctionnalités mais ont l'avantage d'avoir des bibliothèques plus petites, ce
qui facilite la combinaison du langage avec un autre et dans plus de contextes.
Contrairement à de nombreux langages de programmation qui acceptent d'augmenter
la taille de l'environnement d'exécution pour plus de fonctionnalités, Rust a
besoin d'avoir un environnement d'exécution presque inexistant et ne doit pas
faire de compromis sur ses capacités à faire appel au C, afin de conserver ses
performances.

<!--
The green-threading M:N model requires a larger language runtime to manage
threads. As such, the Rust standard library only provides an implementation of
1:1 threading. Because Rust is such a low-level language, there are crates that
implement M:N threading if you would rather trade overhead for aspects such as
more control over which threads run when and lower costs of context switching,
for example.
-->

Le modèle de tâches virtuelles M:N nécessite un plus grand environnement
d'exécution pour gérer les tâches. C'est pourquoi la bibliothèque standard de
Rust fournit seulement une implémentation 1:1. Comme Rust est un langage
bas-niveau, il existe des crates qui implémentent des tâches M:N si vous
préférez compenser des pertes de performances pour plus de maîtrise dans
l'exécution des tâches et moins de conséquences pour les changements de
contextes.

<!--
Now that we’ve defined threads in Rust, let’s explore how to use the
thread-related API provided by the standard library.
-->

Maintenant que nous avons défini ce qu'étaient les tâches en Rust, découvrons
comment utiliser les API liées aux tâches fournies par la bibliothèque
standard.

<!--
### Creating a New Thread with `spawn`
-->

### Créer une nouvelle tâche avec `spawn`

<!--
To create a new thread, we call the `thread::spawn` function and pass it a
closure (we talked about closures in Chapter 13) containing the code we want to
run in the new thread. The example in Listing 16-1 prints some text from a main
thread and other text from a new thread:
-->

Pour créer une nouvelle tâche, nous appelons la fonction `thread::spawn` et
nous lui passons une fermeture (nous avons vu les fermetures au chapitre 13)
qui contient le code que nous souhaitons exécuter dans la nouvelle tâche.
L'exemple dans l'encart 16-1 affiche du texte à partir de la tâche principale
et un autre texte à partir d'une nouvelle tâche :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-01/src/main.rs}}
```

<!--
<span class="caption">Listing 16-1: Creating a new thread to print one thing
while the main thread prints something else</span>
-->

<span class="caption">Encart 16-1 : création d'une nouvelle tâche pour afficher
une chose pendant que la tâche principale affiche autre chose</span>

<!--
Note that with this function, the new thread will be stopped when the main
thread ends, whether or not it has finished running. The output from this
program might be a little different every time, but it will look similar to the
following:
-->

Remarquez qu'avec cette fonction, la nouvelle tâche s'arrêtera lorsque la tâche
principale s'arrêtera, qu'elle ai fini ou non de s'exécuter. La sortie de ce
programme peut être différente à chaque fois, mais il devrait ressembler à
ceci :

<!--
<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -- >
-->

<!--
```text
hi number 1 from the main thread!
hi number 1 from the spawned thread!
hi number 2 from the main thread!
hi number 2 from the spawned thread!
hi number 3 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the main thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
```
-->

```text
Bonjour n°1 à partir de la tâche principale !
Bonjour n°1 à partir de la nouvelle tâche !
Bonjour n°2 à partir de la tâche principale !
Bonjour n°2 à partir de la nouvelle tâche !
Bonjour n°3 à partir de la tâche principale !
Bonjour n°3 à partir de la nouvelle tâche !
Bonjour n°4 à partir de la tâche principale !
Bonjour n°4 à partir de la nouvelle tâche !
Bonjour n°5 à partir de la nouvelle tâche !
```

<!--
The calls to `thread::sleep` force a thread to stop its execution for a short
duration, allowing a different thread to run. The threads will probably take
turns, but that isn’t guaranteed: it depends on how your operating system
schedules the threads. In this run, the main thread printed first, even though
the print statement from the spawned thread appears first in the code. And even
though we told the spawned thread to print until `i` is 9, it only got to 5
before the main thread shut down.
-->

L'appel à `thread::sleep` force une tâche à mettre en pause son exécution
pendant une petite durée, permettant à une autre tâche de s'exécuter. Les
tâches se relaieront probablement, mais ce n'est pas garanti : cela dépend de
comment votre système d'exploitation agence les tâches. Lors de cette
exécution, la tâche principale a d'abord écris, même si l'instruction
d'écriture de la nouvelle tâche apparaît en premier dans le code. Et même si
nous avons demandé à la nouvelle tâche d'écrire jusqu'à ce que `i` vaille `9`,
elle l'a fait seulement jusqu'à `5`, avant que la tâche principale s'arrête.

<!--
If you run this code and only see output from the main thread, or don’t see any
overlap, try increasing the numbers in the ranges to create more opportunities
for the operating system to switch between the threads.
-->

Si vous exécutez ce code et que vous ne voyez que du texte provenant de la
tâche principale, ou que vous ne voyez aucun chevauchement, essayez d'augmenter
les nombres dans les intervalles pour donner plus d'opportunités au système
d'exploitation pour basculer entre les tâches.

<!--
### Waiting for All Threads to Finish Using `join` Handles
-->

### Attendre que toutes les tâches aient fini en utilisant `join`

<!--
The code in Listing 16-1 not only stops the spawned thread prematurely most of
the time due to the main thread ending, but also can’t guarantee that the
spawned thread will get to run at all. The reason is that there is no guarantee
on the order in which threads run!
-->

Le code dans l'encart 16-1 non seulement stoppe la nouvelle tâche prématurément
la plupart du temps à cause de la fin de la tâche principale, mais elle ne
garantit pas non plus que la nouvelle tâche va s'exécuter une seule fois. La
raison à cela est qu'il n'y a pas de garantie sur l'ordre dans lequel les
tâches vont s'exécuter !

<!--
We can fix the problem of the spawned thread not getting to run, or not getting
to run completely, by saving the return value of `thread::spawn` in a variable.
The return type of `thread::spawn` is `JoinHandle`. A `JoinHandle` is an owned
value that, when we call the `join` method on it, will wait for its thread to
finish. Listing 16-2 shows how to use the `JoinHandle` of the thread we created
in Listing 16-1 and call `join` to make sure the spawned thread finishes before
`main` exits:
-->

Nous pouvons régler le problème des nouvelles tâches qui ne s'exécutent pas, ou
pas complètement, en sauvegardant la valeur de retour de `thread::spawn` dans
une variable. Le type de retour de `thread::spawn` est `JoinHandle`. Un
`JoinHandle` est une valeur possédée qui, lorsque nous appelons la méthode
`join` sur elle, va attendre que ses tâches finissent. L'encart 16-2 montre
comment utiliser le `JoinHandle` de la tâche que nous avons créé dans l'encart
16-1 en appelant la méthode `join` pour s'assurer que la nouvelle tâche finit
bien avant que `main` se termine :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-02/src/main.rs}}
```

<!--
<span class="caption">Listing 16-2: Saving a `JoinHandle` from `thread::spawn`
to guarantee the thread is run to completion</span>
-->

<span class="caption">Encart 16-2 : sauvegarde d'un `JoinHandle` d'un
`thread::spawn` pour garantir que la tâche est exécutée jusqu'à la fin</span>

<!--
Calling `join` on the handle blocks the thread currently running until the
thread represented by the handle terminates. *Blocking* a thread means that
thread is prevented from performing work or exiting. Because we’ve put the call
to `join` after the main thread’s `for` loop, running Listing 16-2 should
produce output similar to this:
-->

L'appel à `join` sur le manipulateur bloque la tâche qui s'exécute actuellement
jusqu'à ce que la tâche représentée par le manipulateur se termine. *Bloquer*
une tâche signifie que cette tâche est privée d'accomplir un quelconque travail
ou de se terminer. Comme nous avons inséré l'appel à `join` après la boucle
`for` de la tâche principale, l'exécution de l'encart 16-2 devrait produire un
résultat similaire à celui-ci :

<!--
<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -- >
-->

<!--
```text
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 1 from the spawned thread!
hi number 3 from the main thread!
hi number 2 from the spawned thread!
hi number 4 from the main thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
```
-->

```text
Bonjour n°1 à partir de la tâche principale !
Bonjour n°2 à partir de la tâche principale !
Bonjour n°1 à partir de la nouvelle tâche !
Bonjour n°3 à partir de la tâche principale !
Bonjour n°2 à partir de la nouvelle tâche !
Bonjour n°4 à partir de la tâche principale !
Bonjour n°3 à partir de la nouvelle tâche !
Bonjour n°4 à partir de la nouvelle tâche !
Bonjour n°5 à partir de la nouvelle tâche !
Bonjour n°6 à partir de la nouvelle tâche !
Bonjour n°7 à partir de la nouvelle tâche !
Bonjour n°8 à partir de la nouvelle tâche !
Bonjour n°9 à partir de la nouvelle tâche !
```

<!--
The two threads continue alternating, but the main thread waits because of the
call to `handle.join()` and does not end until the spawned thread is finished.
-->

Les deux tâches continuent à alterner, mais la tâche principale attends à cause
de l'appel à `manipulateur.join()` et ne se termine pas avant que la nouvelle
tâche soit finie.

<!--
But let’s see what happens when we instead move `handle.join()` before the
`for` loop in `main`, like this:
-->

Mais voyons maintenant ce qui se passe lorsque nous déplaçons le
`manipulateur.join()` avant la boucle `for` du `main` comme ceci :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/no-listing-01-join-too-early/src/main.rs}}
```

<!--
The main thread will wait for the spawned thread to finish and then run its
`for` loop, so the output won’t be interleaved anymore, as shown here:
-->

La tâche principale va attendre que la nouvelle tâche se finisse et ensuite
exécuter sa boucle `for`, ainsi la sortie ne sera plus chevauchée, comme
ci-dessous :

<!--
<!-- Not extracting output because changes to this output aren't significant;
the changes are likely to be due to the threads running differently rather than
changes in the compiler -- >
-->

<!--
```text
hi number 1 from the spawned thread!
hi number 2 from the spawned thread!
hi number 3 from the spawned thread!
hi number 4 from the spawned thread!
hi number 5 from the spawned thread!
hi number 6 from the spawned thread!
hi number 7 from the spawned thread!
hi number 8 from the spawned thread!
hi number 9 from the spawned thread!
hi number 1 from the main thread!
hi number 2 from the main thread!
hi number 3 from the main thread!
hi number 4 from the main thread!
```
-->

```text
Bonjour n°1 à partir de la nouvelle tâche !
Bonjour n°2 à partir de la nouvelle tâche !
Bonjour n°3 à partir de la nouvelle tâche !
Bonjour n°4 à partir de la nouvelle tâche !
Bonjour n°5 à partir de la nouvelle tâche !
Bonjour n°6 à partir de la nouvelle tâche !
Bonjour n°7 à partir de la nouvelle tâche !
Bonjour n°8 à partir de la nouvelle tâche !
Bonjour n°9 à partir de la nouvelle tâche !
Bonjour n°1 à partir de la tâche principale !
Bonjour n°2 à partir de la tâche principale !
Bonjour n°3 à partir de la tâche principale !
Bonjour n°4 à partir de la tâche principale !
```

<!--
Small details, such as where `join` is called, can affect whether or not your
threads run at the same time.
-->

Des petits détails, comme l'endroit où `join` est appelé, peuvent déterminer si
vos tâches peuvent être exécutées ou non en même temps.

<!--
### Using `move` Closures with Threads
-->

### Utiliser les fermetures `move` avec les tâches

<!--
The `move` closure is often used alongside `thread::spawn` because it allows
you to use data from one thread in another thread.
-->

La fermeture `move` est souvent utilisé avec `thread::spawn` car elle vous
permet d'utiliser une donnée d'une tâche dans une autre tâche.

<!--
In Chapter 13, we mentioned we can use the `move` keyword before the parameter
list of a closure to force the closure to take ownership of the values it uses
in the environment. This technique is especially useful when creating new
threads in order to transfer ownership of values from one thread to another.
-->

Au chapitre 13, nous avons évoqué que nous pouvions utiliser le mot-clé `move`
avant la liste des paramètres d'une fermeture pour forcer la fermeture à
prendre possession des valeurs de son environnement qu'elle utilise. Cette
technique est particulièrement utile lorsque nous créons des nouvelles tâches
pour pouvoir transférer la possession des valeurs d'une tâche à une autre.

<!--
Notice in Listing 16-1 that the closure we pass to `thread::spawn` takes no
arguments: we’re not using any data from the main thread in the spawned
thread’s code. To use data from the main thread in the spawned thread, the
spawned thread’s closure must capture the values it needs. Listing 16-3 shows
an attempt to create a vector in the main thread and use it in the spawned
thread. However, this won’t yet work, as you’ll see in a moment.
-->

Remarquez dans l'encart 16-1 que la fermeture que nous donnons à `thread::spawn`
ne prends pas d'arguments : nous n'utilisons aucune donnée de la tâche
principale dans le code de la nouvelle tâche. Pour utiliser des données de la
tâche principale dans la nouvelle tâche, la fermeture de la nouvelle tâche doit
capturer les valeurs dont elle a besoin. L'encart 16-3 montre une tentative de
création d'un vecteur dans la tâche principale et l'utilisation dans la
nouvelle tâche. Cependant, cela ne fonctionne pas encore, comme vous allez le
constater bientôt.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-03/src/main.rs}}
```

<!--
<span class="caption">Listing 16-3: Attempting to use a vector created by the
main thread in another thread</span>
-->

<span class="caption">Encart 16-3 : tentative d'utilisation d'un vecteur créé
par la tâche principale dans une autre tâche</span>

<!--
The closure uses `v`, so it will capture `v` and make it part of the closure’s
environment. Because `thread::spawn` runs this closure in a new thread, we
should be able to access `v` inside that new thread. But when we compile this
example, we get the following error:
-->

La fermeture utilise `v`, donc elle va capturer `v` et l'intégrer dans son
environnement. Comme `thread::spawn` exécute cette fermeture dans une nouvelle
tâche, nous devrions pouvoir accéder à `v` dans cette nouvelle tâche. Mais
lorsque nous compilons cet exemple, nous obtenons l'erreur suivante :

<!--
```console
{{#include ../listings-sources/ch16-fearless-concurrency/listing-16-03/output.txt}}
```
-->

```console
{{#include ../listings/ch16-fearless-concurrency/listing-16-03/output.txt}}
```

<!--
Rust *infers* how to capture `v`, and because `println!` only needs a reference
to `v`, the closure tries to borrow `v`. However, there’s a problem: Rust can’t
tell how long the spawned thread will run, so it doesn’t know if the reference
to `v` will always be valid.
-->

Rust *déduit* comment capturer `v`, et comme `println!` n'a besoin que d'une
référence à `v`, la fermeture essaye d'emprunter `v`. Cependant, il y a un
problème : Rust ne peut pas savoir combien de temps la tâche va s'exécuter,
donc il ne peut pas savoir si la référence à `v` sera toujours valide.

<!--
Listing 16-4 provides a scenario that’s more likely to have a reference to `v`
that won’t be valid:
-->

L'encart 16-4 propose un scénario qui est plus enclin à avoir une référence à
`v` qui ne sera plus valide :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-04/src/main.rs}}
```

<!--
<span class="caption">Listing 16-4: A thread with a closure that attempts to
capture a reference to `v` from a main thread that drops `v`</span>
-->

<span class="caption">Encart 16-4 : une tâche dont la fermeture essaye de
capturer une référence à `v` à partir de la tâche principale, qui va ensuite
libérer `v`</span>

<!--
If we were allowed to run this code, there’s a possibility the spawned thread
would be immediately put in the background without running at all. The spawned
thread has a reference to `v` inside, but the main thread immediately drops
`v`, using the `drop` function we discussed in Chapter 15. Then, when the
spawned thread starts to execute, `v` is no longer valid, so a reference to it
is also invalid. Oh no!
-->

Si nous étions autorisés à exécuter ce code, il y aurait une possibilité que
la nouvelle tâche serait immédiatement placée en arrière-plan sans être
exécutée du tout. La nouvelle tâche a une référence à `v` en son sein, mais la
tâche principale libère immédiatement `v`, en utilisant la fonction `drop` que
nous avons vu au chapitre 15. Ensuite, lorsque la nouvelle tâche commence à
s'exécuter, `v` n'est plus en vigueur, donc une référence à cette dernière est
elle aussi invalide !

<!--
To fix the compiler error in Listing 16-3, we can use the error message’s
advice:
-->

Pour corriger l'erreur de compilation de l'encart 16-3, nous pouvons appliquer
le conseil du message d'erreur :

<!--
<!-- manual-regeneration
after automatic regeneration, look at listings/ch16-fearless-concurrency/listing-16-03/output.txt and copy the relevant part
-- >
-->

<!--
```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let handle = thread::spawn(move || {
  |                                ^^^^^^^
```
-->

```text
help: to force the closure to take ownership of `v` (and any other referenced variables), use the `move` keyword
  |
6 |     let manipulateur = thread::spawn(move || {
  |                                      ^^^^^^^
```

<!--
By adding the `move` keyword before the closure, we force the closure to take
ownership of the values it’s using rather than allowing Rust to infer that it
should borrow the values. The modification to Listing 16-3 shown in Listing
16-5 will compile and run as we intend:
-->

En ajoutant le mot-clé `move` avant la fermeture, nous forçons la fermeture à
prendre possession des valeurs qu'elle utilise au lieu de laisser Rust en
déduire qu'il doit emprunter les valeurs. Les modifications à l'encart 16-3
proposées dans l'encart 16-5 devraient se compiler et s'exécuter comme prévu :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch16-fearless-concurrency/listing-16-05/src/main.rs}}
```

<!--
<span class="caption">Listing 16-5: Using the `move` keyword to force a closure
to take ownership of the values it uses</span>
-->

<span class="caption">Encart 16-5 : utilisation du mot-clé `move` pour forcer
une fermeture à prendre possession des valeurs qu'elle utilise</span>

<!--
What would happen to the code in Listing 16-4 where the main thread called
`drop` if we use a `move` closure? Would `move` fix that case? Unfortunately,
no; we would get a different error because what Listing 16-4 is trying to do
isn’t allowed for a different reason. If we added `move` to the closure, we
would move `v` into the closure’s environment, and we could no longer call
`drop` on it in the main thread. We would get this compiler error instead:
-->

Qu'est-ce qui arriverait au code de l'encart 16-4 dans lequel la tâche
principale fait appel à `drop` si nous utilisions la fermeture avec `move` ?
Est-ce que le `move` va résoudre ce problème ? Malheureusement, non ; nous
obtiendrions une erreur différente parce que ce que l'encart 16-4 essaye de
faire n'est pas autorisé pour différentes raisons. Si nous ajoutions `move` à la
fermeture, nous déplacerions `v` dans l'environnement de la fermeture, et nous
ne pourrions plus appeler `drop` sur `v` dans la tâche principale. Nous
obtiendrons à la place cette erreur de compilation :

<!--
```console
{{#include ../listings-sources/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```
-->

```console
{{#include ../listings/ch16-fearless-concurrency/output-only-01-move-drop/output.txt}}
```

```text
error[E0382]: use of moved value: `v`
  -- > src/main.rs:10:10
   |
6  |     let manipulateur = thread::spawn(move || {
   |                                      ------- value moved (into closure) here
...
10 |     drop(v); // oh non, le vecteur est libéré !
   |          ^ value used here after move
   |
   = note: move occurs because `v` has type `std::vec::Vec<i32>`, which does
   not implement the `Copy` trait
```

<!--
Rust’s ownership rules have saved us again! We got an error from the code in
Listing 16-3 because Rust was being conservative and only borrowing `v` for the
thread, which meant the main thread could theoretically invalidate the spawned
thread’s reference. By telling Rust to move ownership of `v` to the spawned
thread, we’re guaranteeing Rust that the main thread won’t use `v` anymore. If
we change Listing 16-4 in the same way, we’re then violating the ownership
rules when we try to use `v` in the main thread. The `move` keyword overrides
Rust’s conservative default of borrowing; it doesn’t let us violate the
ownership rules.
-->

Les règles de possession de Rust nous ont encore sauvé la mise ! Nous obtenons
une erreur du code l'encart 16-3 car Rust a été conservateur et a juste emprunté
`v` à la tâche, ce qui signifie que la tâche principale peut théoriquement
neutraliser la référence de la tâche crée. En demandant à Rust de déplacer la
possession de `v` à la nouvelle tâche, nous avons garanti à Rust que la tâche
principale n'utiliserait plus `v`. Si nous changeons l'encart 16-4 de la même
manière, nous violons les règles de possession lorsque nous essayons d'utiliser
`v` dans la tâche principale. Le mot-clé `move` remplace le comportement
d'emprunt conservateur par défaut ; il ne nous laisse pas enfreindre les règles
d'emprunt.

<!--
With a basic understanding of threads and the thread API, let’s look at what we
can *do* with threads.
-->

Avec cette connaissance de base des tâches et de leur API, découvrons ce que
nous pouvons *faire* avec les tâches.
