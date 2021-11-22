> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/216).

<!--
## Turning Our Single-Threaded Server into a Multithreaded Server
-->

## Transformer notre serveur monotâche en serveur multitâches

<!--
Right now, the server will process each request in turn, meaning it won’t
process a second connection until the first is finished processing. If the
server received more and more requests, this serial execution would be less and
less optimal. If the server receives a request that takes a long time to
process, subsequent requests will have to wait until the long request is
finished, even if the new requests can be processed quickly. We’ll need to fix
this, but first, we’ll look at the problem in action.
-->

Jusqu'à présent, le serveur va traiter chaque requête dans l'ordre, ce qui
signifie qu'il ne va pas traiter une seconde connexion tant que la première
n'a pas fini d'être traitée. Si le serveur reçoit encore plus de requêtes,
cette exécution à la chaîne sera de moins en moins optimale. Si le serveur
reçoit une requête qui prend longtemps à traiter, les demandes suivantes
devront attendre que la longue requête à traiter soit terminée, même si les
nouvelles requêtes peuvent être traitées rapidement. Nous devons corriger cela,
mais d'abord, observons ce problème en pratique.

<!--
### Simulating a Slow Request in the Current Server Implementation
-->

### Simuler une longue requête à traiter avec l'implémentation actuelle du serveur

<!--
We’ll look at how a slow-processing request can affect other requests made to
our current server implementation. Listing 20-10 implements handling a request
to */sleep* with a simulated slow response that will cause the server to sleep
for 5 seconds before responding.
-->

Nous allons voir comment une requête longue à traiter peut influer sur le
traitement des autres requêtes par l'implémentation actuelle de notre serveur.
L'encart 20-10 rajoute le traitement d'une requête pour */pause* qui va simuler
une longue réponse qui va faire en sorte que le serveur soit en pause pendant 5
secondes avant de répondre à nouveau.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-10/src/main.rs:here}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-10/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-10: Simulating a slow request by recognizing
*/sleep* and sleeping for 5 seconds</span>
-->

<span class="caption">Encart 20-10 : simulation d'un long traitement de requête
en détectant */pause* et en faisant une pause de 5 secondes</span>

<!--
This code is a bit messy, but it’s good enough for simulation purposes. We
created a second request `sleep`, whose data our server recognizes. We added an
`else if` after the `if` block to check for the request to */sleep*. When that
request is received, the server will sleep for 5 seconds before rendering the
successful HTML page.
-->

Ce code est peu brouillon, mais est suffisant pour nos besoins de simulation.
Nous avons créé une seconde possibilité de requête `pause`, avec les données que
notre serveur va détecter. Nous avons ajouté un `else if` après le bloc `if`
pour vérifier les requêtes vers */pause*. Lorsque cette requête est reçue, le
serveur va se mettre en pause pendant 5 secondes avant de générer la page HTML
de succès.

<!--
You can see how primitive our server is: real libraries would handle the
recognition of multiple requests in a much less verbose way!
-->

Vous pouvez constater à quel point notre serveur est primitif : une
bibliothèque digne de ce nom devrait gérer la détection de différents types de
requêtes de manière bien moins verbeuse !

<!--
Start the server using `cargo run`. Then open two browser windows: one for
*http://127.0.0.1:7878/* and the other for *http://127.0.0.1:7878/sleep*. If
you enter the */* URI a few times, as before, you’ll see it respond quickly.
But if you enter */sleep* and then load */*, you’ll see that */* waits until
`sleep` has slept for its full 5 seconds before loading.
-->

Démarrez le serveur en utilisant `cargo run`. Ouvrez ensuite deux fenêtres de
navigateur web : une pour *http://127.0.0.1:7878/* et l'autre pour
*http://127.0.0.1:7878/pause*. Si vous demandez l'URI */* plusieurs fois, comme
vous l'avez fait précédemment, vous constaterez que le serveur répond
rapidement. Mais lorsque vous saisirez */pause* et que vous chargerez ensuite
*/*, vous constaterez que */* attend que `pause` ai fini sa pause des 5
secondes avant de se charger.

<!--
There are multiple ways we could change how our web server works to avoid
having more requests back up behind a slow request; the one we’ll implement is
a thread pool.
-->

Il y a plusieurs manières de changer le fonctionnement de notre serveur web
pour éviter d'accumuler des requêtes après une requête dont le traitement est
long ; celle que nous allons implémenter est un groupe de tâches.

<!--
### Improving Throughput with a Thread Pool
-->

### Améliorer le débit avec un groupe de tâches

<!--
A *thread pool* is a group of spawned threads that are waiting and ready to
handle a task. When the program receives a new task, it assigns one of the
threads in the pool to the task, and that thread will process the task. The
remaining threads in the pool are available to handle any other tasks that come
in while the first thread is processing. When the first thread is done
processing its task, it’s returned to the pool of idle threads, ready to handle
a new task. A thread pool allows you to process connections concurrently,
increasing the throughput of your server.
-->

Un *groupe de tâches* est un groupe constitué de tâches qui ont été crées et
qui attendent des missions. Lorsque le programme reçoit une nouvelle mission,
il assigne une des tâches du groupe pour cette mission, et cette tâche va
traiter la mission. Les tâches restantes dans le groupe restent disponibles
pour traiter d'autres missions qui peuvent arriver pendant que la première
tâche est en cours de traitement. Lorsque la première tâche a fini avec sa
mission, elle retourne dans le groupe de tâches inactives, prête à gérer une
nouvelle tâche. Un groupe de tâches vous permet de traiter plusieurs connexions
en simultané, ce qui augmente le débit de votre serveur.

<!--
We’ll limit the number of threads in the pool to a small number to protect us
from Denial of Service (DoS) attacks; if we had our program create a new thread
for each request as it came in, someone making 10 million requests to our
server could create havoc by using up all our server’s resources and grinding
the processing of requests to a halt.
-->

Nous allons limiter le nombre de tâches dans le groupe à un petit nombre pour
nous protéger d'attaques par déni de service (Denial of Service, DoS) ; si notre
programme créait une nouvelle tâche à chaque requête qu'il reçoit, quelqu'un qui
fait 10 millions de requêtes à notre serveur pourrait faire des ravages en
utilisant toutes les ressources de notre serveur et paralyser le traitement des
demandes.

<!--
Rather than spawning unlimited threads, we’ll have a fixed number of threads
waiting in the pool. As requests come in, they’ll be sent to the pool for
processing. The pool will maintain a queue of incoming requests. Each of the
threads in the pool will pop off a request from this queue, handle the request,
and then ask the queue for another request. With this design, we can process
`N` requests concurrently, where `N` is the number of threads. If each thread
is responding to a long-running request, subsequent requests can still back up
in the queue, but we’ve increased the number of long-running requests we can
handle before reaching that point.
-->

Plutôt que de générer des tâches en quantité illimitée, nous allons faire en
sorte qu'il y ait un nombre fixe de tâches qui seront en attente dans le
groupe. Lorsqu'une requête arrive, une tâche sera choisie dans le groupe pour
procéder au traitement. Le groupe gérera une file d'attente pour les requêtes
entrantes. Chaque tâche dans le groupe va récupérer une requête dans cette
liste d'attente, traiter cette requête, et ensuite demander une autre requête
à la file d'attente. Avec ce fonctionnement, nous pouvons traiter `N` requêtes
en concurrence, où `N` est le nombre de tâches. Si toutes les tâches répondent
chacune à une requête longue à traiter, les requêtes suivantes vont se stocker
dans la file d'attente, mais nous augmentons alors le nombre de requêtes
longues à traiter que nous devons traiter avant d'arriver à la fin.

<!--
This technique is just one of many ways to improve the throughput of a web
server. Other options you might explore are the fork/join model and the
single-threaded async I/O model. If you’re interested in this topic, you can
read more about other solutions and try to implement them in Rust; with a
low-level language like Rust, all of these options are possible.
-->

Cette technique n'est qu'une des nombreuses manières d'améliorer le débit d'un
serveur web. D'autres options que vous devriez envisager sont le modèle
fork/join et le modèle d'entrée-sortie asynchrone monotâche. Si vous êtes
intéressés par ce sujet, vous pouvez aussi en apprendre plus sur d'autres
solutions et essayer de les implémenter en Rust ; avec un langage bas niveau
comme Rust, toutes les options restent possibles.

<!--
Before we begin implementing a thread pool, let’s talk about what using the
pool should look like. When you’re trying to design code, writing the client
interface first can help guide your design. Write the API of the code so it’s
structured in the way you want to call it; then implement the functionality
within that structure rather than implementing the functionality and then
designing the public API.
-->

Avant que nous commencions l'implémentation du groupe de tâches, parlons de
l'utilisation du groupe. Lorsque vous essayez de concevoir du code, commencer
par écrire l'interface client peut vous aider à vous guider dans la conception.
Ecrivez l'API du code afin qu'il soit structuré de la manière dont vous
souhaitez l'appeler ; puis implémentez ensuite la fonctionnalité au sein de
cette structure, plutôt que d'implémenter la fonctionnalité puis de concevoir
l'API publique.

<!--
Similar to how we used test-driven development in the project in Chapter 12,
we’ll use compiler-driven development here. We’ll write the code that calls the
functions we want, and then we’ll look at errors from the compiler to determine
what we should change next to get the code to work.
-->

De la même manière que nous avons utilisé le développement piloté par les tests
dans le projet du chapitre 12, nous allons utiliser ici le développement
orienté par le compilateur. Nous allons écrire le code qui appelle les
fonctions que nous souhaitons, et ensuite nous analyserons les erreurs du
compilateur pour déterminer ce qu'il faut ensuite corriger pour que le code
fonctionne.

<!--
#### Code Structure If We Could Spawn a Thread for Each Request
-->

#### La structure du code si nous pouvions créer une tâche pour chaque requête

<!--
First, let’s explore how our code might look if it did create a new thread for
every connection. As mentioned earlier, this isn’t our final plan due to the
problems with potentially spawning an unlimited number of threads, but it is a
starting point. Listing 20-11 shows the changes to make to `main` to spawn a
new thread to handle each stream within the `for` loop.
-->

Pour commencer, voyons à quoi ressemblerait notre code s'il créait une nouvelle
tâche pour chaque connexion. Comme nous l'avons évoqué précédemment, cela ne
sera pas notre solution finale à cause des problèmes liés à la création
potentielle d'un nombre illimité de tâches, mais c'est un début. L'encart 20-11
montre les changements à apporter au `main` pour créer une nouvelle tâche pour
gérer chaque flux avec une boucle `for`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,no_run
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-11/src/main.rs:here}}
```
-->

```rust,no_run
{{#rustdoc_include ../listings/ch20-web-server/listing-20-11/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-11: Spawning a new thread for each
stream</span>
-->

<span class="caption">Encart 20-11 : création d'une nouvelle tâche pour chaque
flux</span>

<!--
As you learned in Chapter 16, `thread::spawn` will create a new thread and then
run the code in the closure in the new thread. If you run this code and load
*/sleep* in your browser, then */* in two more browser tabs, you’ll indeed see
that the requests to */* don’t have to wait for */sleep* to finish. But as we
mentioned, this will eventually overwhelm the system because you’d be making
new threads without any limit.
-->

Comme vous l'avez appris au chapitre 16, `thread::spawn` va créer une nouvelle
tâche et ensuite exécuter dans cette nouvelle tâche le code présent dans la
fermeture. Si vous exécutez ce code et chargez */pause* dans votre navigateur,
et que vous ouvrez */* dans deux nouveaux onglets, vous constaterez en effet
que les requêtes vers */* n'aurons pas à attendre que */pause* se finisse. Mais
comme nous l'avons mentionné, cela peut potentiellement surcharger le système
si vous créez des nouvelles tâches sans limite.

<!--
#### Creating a Similar Interface for a Finite Number of Threads
-->

#### Créer une interface similaire pour un nombre fini de tâches

<!--
We want our thread pool to work in a similar, familiar way so switching from
threads to a thread pool doesn’t require large changes to the code that uses
our API. Listing 20-12 shows the hypothetical interface for a `ThreadPool`
struct we want to use instead of `thread::spawn`.
-->

Nous souhaitons faire en sorte que notre groupe de tâches fonctionne de la même
manière, donc le remplacement des tâches par le groupe de tâches ne devrait pas
nécessiter de gros changements au code qui utilise notre API. L'encart 20-12
montre une interface éventuelle pour une structure `GroupeTaches` que nous
souhaitons utiliser à la place de `thread::spawn`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-12/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-12/src/main.rs:here}}
```

<!--
<span class="caption">Listing 20-12: Our ideal `ThreadPool` interface</span>
-->

<span class="caption">Encart 20-12 : Notre interface idéale `GroupeTaches`
</span>

<!--
We use `ThreadPool::new` to create a new thread pool with a configurable number
of threads, in this case four. Then, in the `for` loop, `pool.execute` has a
similar interface as `thread::spawn` in that it takes a closure the pool should
run for each stream. We need to implement `pool.execute` so it takes the
closure and gives it to a thread in the pool to run. This code won’t yet
compile, but we’ll try so the compiler can guide us in how to fix it.
-->

Nous avons utilisé `GroupeTaches::new` pour créer un nouveau groupe de tâches
avec un nombre configurable de tâches, dans notre cas, quatre. Ensuite, dans
la boucle `for`, `groupe.executer` a une interface similaire à `thread::spawn`
qui prend une fermeture que le groupe devra exécuter pour chaque flux. Nous
devons implémenter `groupe.executer` pour qu'il prenne la fermeture et la donne
à une tâche dans le groupe pour qu'elle l'exécute. Ce code ne se compile pas
encore, mais nous allons essayer de faire comme ceci pour que le compilateur
puisse nous guider dans la résolution des problèmes.

<!--
#### Building the `ThreadPool` Struct Using Compiler Driven Development
-->

#### Construire la structure `GroupeTaches` en utilisant le développement orienté par le compilateur

<!--
Make the changes in Listing 20-12 to *src/main.rs*, and then let’s use the
compiler errors from `cargo check` to drive our development. Here is the first
error we get:
-->

Faites les changements de l'encart 20-12 dans votre *src/main.rs*, et utilisez
ensuite les erreurs du compilateur lors du `cargo check` pour orienter votre
développement. Voici la première erreur que nous obtenons :

<!--
```console
{{#include ../listings-sources/ch20-web-server/listing-20-12/output.txt}}
```
-->

```console
{{#include ../listings/ch20-web-server/listing-20-12/output.txt}}
```

<!--
Great! This error tells us we need a `ThreadPool` type or module, so we’ll
build one now. Our `ThreadPool` implementation will be independent of the kind
of work our web server is doing. So, let’s switch the `hello` crate from a
binary crate to a library crate to hold our `ThreadPool` implementation. After
we change to a library crate, we could also use the separate thread pool
library for any work we want to do using a thread pool, not just for serving
web requests.
-->

Bien ! Cette erreur nous informe que nous avons besoin d'un type ou d'un module
qui s'appelle `GroupeTaches`, donc nous allons le créer. Notre implémentation
de `GroupeTaches` sera indépendante du type de travail qu'accomplira notre
serveur web. Donc, transformons la crate binaire `salutations` en crate de
bibliothèque pour y implémenter notre `GroupeTaches`. Après l'avoir changé en
crate de bibliothèque, nous pourrons utiliser ensuite cette bibliothèque de
groupe de tâches dans n'importe quel projet où nous aurons besoin d'un groupe
de tâches, et non pas seulement pour servir des requêtes web.

<!--
Create a *src/lib.rs* that contains the following, which is the simplest
definition of a `ThreadPool` struct that we can have for now:
-->

Créez un *src/lib.rs* qui contient ceci, qui est la définition la plus
simpliste d'une structure `GroupeTaches` que nous pouvons avoir pour le
moment :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/lib.rs}}
```

<!--
Then create a new directory, *src/bin*, and move the binary crate rooted in
*src/main.rs* into *src/bin/main.rs*. Doing so will make the library crate the
primary crate in the *hello* directory; we can still run the binary in
*src/bin/main.rs* using `cargo run`. After moving the *main.rs* file, edit it
to bring the library crate in and bring `ThreadPool` into scope by adding the
following code to the top of *src/bin/main.rs*:
-->

Créez ensuite un nouveau dossier, *src/bin*, et déplacez-y la crate binaire qui
est le *src/main.rs* dans *src/bin/main.rs*. Faire ceci va faire en sorte que
la crate de bibliothèque soit la crate principale dans le dossier
*salutations* ; nous pouvons quand même continuer à exécuter le binaire dans
*src/bin/main.rs* en utilisant `cargo run`. Après avoir déplacé le fichier
*main.rs*, modifiez-le pour importer la crate de bibliothèque et importer
`GroupeTaches` dans la portée en ajoutant le code suivant en haut de
*src/bin/main.rs* :

<!--
<span class="filename">Filename: src/bin/main.rs</span>
-->

<span class="filename">Fichier : src/bin/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-01-define-threadpool-struct/src/bin/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/src/bin/main.rs:here}}
```

<!--
This code still won’t work, but let’s check it again to get the next error that
we need to address:
-->

Ce code ne fonctionne toujours pas, mais vérifions-le à nouveau pour obtenir
l'erreur suivante que nous devons résoudre :

<!--
```console
{{#include ../listings-sources/ch20-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```
-->

```console
{{#include ../listings/ch20-web-server/no-listing-01-define-threadpool-struct/output.txt}}
```

<!--
This error indicates that next we need to create an associated function named
`new` for `ThreadPool`. We also know that `new` needs to have one parameter
that can accept `4` as an argument and should return a `ThreadPool` instance.
Let’s implement the simplest `new` function that will have those
characteristics:
-->

Cette erreur indique que nous devons ensuite créer une fonction associée `new`
pour `GroupeTaches`. Nous savons aussi que `new` nécessite d'avoir un paramètre
qui peut accepter `4` comme argument et doit retourner une instance de
`GroupeTaches`. Implémentons la fonction `new` la plus simple possible qui aura
ces caractéristiques :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-02-impl-threadpool-new/src/lib.rs}}
```

<!--
We chose `usize` as the type of the `size` parameter, because we know that a
negative number of threads doesn’t make any sense. We also know we’ll use this
4 as the number of elements in a collection of threads, which is what the
`usize` type is for, as discussed in the [“Integer Types”][integer-types]<!--
ignore -- > section of Chapter 3.
-->

Nous avons choisi `usize` comme type du paramètre `taille`, car nous savons
qu'un nombre négatif de tâches n'as pas de sens. Nous savons également que nous
allons utiliser ce 4 comme étant le nombre d'éléments dans une collection de
tâches, ce à quoi sert le type `usize`, comme nous l'avons vu dans la section
[“Types de nombres entiers”][integer-types]<!-- ignore --> du chapitre 3.

<!--
Let’s check the code again:
-->

Vérifions à nouveau le code :

<!--
```console
{{#include ../listings-sources/ch20-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```
-->

```console
{{#include ../listings/ch20-web-server/no-listing-02-impl-threadpool-new/output.txt}}
```

<!--
Now the error occurs because we don’t have an `execute` method on `ThreadPool`.
Recall from the [“Creating a Similar Interface for a Finite Number of
Threads”](#creating-a-similar-interface-for-a-finite-number-of-threads)<!--
ignore -- > section that we decided our thread pool should have an interface
similar to `thread::spawn`. In addition, we’ll implement the `execute` function
so it takes the closure it’s given and gives it to an idle thread in the pool
to run.
-->

Désormais, nous obtenons une erreur car nous n'avons pas implémenté la méthode
`executer` sur `GroupeTaches`. Souvenez-vous que nous avions décidé dans la
section [“Créer une interface similaire pour un nombre fini de
tâches”](#créer-une-interface-similaire-pour-un-nombre-fini-de-tâches)<!--
ignore --> que notre groupe de tâches devrait avoir une interface similaire à
`thread::spawn`. C'est pourquoi nous allons implémenter la fonction `executer`
pour qu'elle prenne en argument la fermeture qu'on lui donne et elle la passera
à une tâche inactive du groupe pour qu'elle l'exécute.

<!--
We’ll define the `execute` method on `ThreadPool` to take a closure as a
parameter. Recall from the [“Storing Closures Using Generic Parameters and the
`Fn` Traits”][storing-closures-using-generic-parameters-and-the-fn-traits]<!--
ignore -- > section in Chapter 13 that we can take closures as parameters with
three different traits: `Fn`, `FnMut`, and `FnOnce`. We need to decide which
kind of closure to use here. We know we’ll end up doing something similar to
the standard library `thread::spawn` implementation, so we can look at what
bounds the signature of `thread::spawn` has on its parameter. The documentation
shows us the following:
-->

Nous allons définir la méthode `executer` sur `GroupeTaches` pour prendre en
paramètres une fermeture. Souvenez-vous que nous avions vu dans [une section du
chapitre 13][storing-closures-using-generic-parameters-and-the-fn-traits]<!--
ignore --> que nous pouvions prendre en paramètre les fermetures avec trois
différents traits : `Fn`, `FnMut`, et `FnOnce`. Nous devons décider quel genre
de fermeture nous allons utiliser ici. Nous savons que nous allons faire quelque
chose de sensiblement identique à l'implémentation du `thread::spawn` de la
bibliothèque standard, donc nous pouvons nous inspirer de ce qui est attaché à
la signature de `thread::spawn`. La documentation nous donne ceci :

<!--
```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```
-->

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

<!--
The `F` type parameter is the one we’re concerned with here; the `T` type
parameter is related to the return value, and we’re not concerned with that. We
can see that `spawn` uses `FnOnce` as the trait bound on `F`. This is probably
what we want as well, because we’ll eventually pass the argument we get in
`execute` to `spawn`. We can be further confident that `FnOnce` is the trait we
want to use because the thread for running a request will only execute that
request’s closure one time, which matches the `Once` in `FnOnce`.
-->

Le paramètre de type `F` est celui qui nous intéresse ici ; le paramètre de
type `T` est lié à la valeur de retour, et nous ne sommes pas intéressés par
ceci. Nous pouvons constater que `spawn` utilise le trait `FnOnce` lié à `F`.
C'est probablement ce dont nous avons besoin, parce que nous allons sûrement
passer cet argument dans le `execute` de `spawn`. Nous pouvons aussi être sûr
que `FnOnce` est le trait dont nous avons besoin car la tâche qui va exécuter la
requête va exécuter le traitement la requête uniquement une seule fois, ce qui
correspond à la partie `Once` dans `FnOnce`.

<!--
The `F` type parameter also has the trait bound `Send` and the lifetime bound
`'static`, which are useful in our situation: we need `Send` to transfer the
closure from one thread to another and `'static` because we don’t know how long
the thread will take to execute. Let’s create an `execute` method on
`ThreadPool` that will take a generic parameter of type `F` with these bounds:
-->

Le paramètre de type `F` a aussi le trait lié `Send` et la durée de vie liée
`'static`, qui sont utiles dans notre situation : nous avons besoin de `Send`
pour transférer la fermeture d'une tâche vers une autre et de `'static` car nous
ne savons pas la durée d'exécution de la tâche. Créons donc une méthode
`executer` sur `GroupeTaches` qui va utiliser un paramètre générique de type `F`
avec les liens suivants :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-03-define-execute/src/lib.rs:here}}
```

<!--
We still use the `()` after `FnOnce` because this `FnOnce` represents a closure
that takes no parameters and returns the unit type `()`. Just like function
definitions, the return type can be omitted from the signature, but even if we
have no parameters, we still need the parentheses.
-->

Nous utilisons toujours le `()` après `FnOne` car ce `FnOnce` représente une
fermeture qui ne prend pas de paramètres et retourne le type unité `()`.
Exactement comme les définitions de fonctions, le type de retour peut être omis
de la signature, mais même si elle n'a pas de paramètre, nous avons tout de
même besoin des parenthèses.

<!--
Again, this is the simplest implementation of the `execute` method: it does
nothing, but we’re trying only to make our code compile. Let’s check it again:
-->

A nouveau, c'est l'implémentation la plus simpliste de la méthode `executer` :
elle ne fait rien, mais nous essayons seulement de faire en sorte que notre code
se compile. Vérifions-le à nouveau :

<!--
```console
{{#include ../listings-sources/ch20-web-server/no-listing-03-define-execute/output.txt}}
```
-->

```console
{{#include ../listings/ch20-web-server/no-listing-03-define-execute/output.txt}}
```

<!--
It compiles! But note that if you try `cargo run` and make a request in the
browser, you’ll see the errors in the browser that we saw at the beginning of
the chapter. Our library isn’t actually calling the closure passed to `execute`
yet!
-->

Cela se compile ! Mais remarquez que si vous lancez `cargo run` et faites la
requête dans votre navigateur web, vous verrez l'erreur dans le navigateur que
nous avions tout au début du chapitre. Notre bibliothèque n'exécute pas encore
la fermeture envoyée à `executer` !

<!--
> Note: A saying you might hear about languages with strict compilers, such as
> Haskell and Rust, is “if the code compiles, it works.” But this saying is not
> universally true. Our project compiles, but it does absolutely nothing! If we
> were building a real, complete project, this would be a good time to start
> writing unit tests to check that the code compiles *and* has the behavior we
> want.
-->

> Remarque : un dicton que vous avez probablement déjà entendu à propos des
> compilateurs strictes, comme Haskell et Rust, est que “si le code se compile,
> il fonctionne”. Mais ce dicton n'est pas vrai universellement. Notre projet se
> compile, mais il ne fait absolument rien ! Si nous construisions un vrai
> projet, complexe, il serait bon de commencer à écrire des tests unitaires pour
> vérifier que ce code compile *et* qu'il suit le comportement que nous
> souhaitons.

<!--
#### Validating the Number of Threads in `new`
-->

#### Valider le nombre de tâches envoyé à `new`

<!--
We aren’t doing anything with the parameters to `new` and `execute`. Let’s
implement the bodies of these functions with the behavior we want. To start,
let’s think about `new`. Earlier we chose an unsigned type for the `size`
parameter, because a pool with a negative number of threads makes no sense.
However, a pool with zero threads also makes no sense, yet zero is a perfectly
valid `usize`. We’ll add code to check that `size` is greater than zero before
we return a `ThreadPool` instance and have the program panic if it receives a
zero by using the `assert!` macro, as shown in Listing 20-13.
-->

Nous ne faisons rien avec les paramètres `new` et `executer`. Implémentons le
corps de ces fonctions avec le comportement que nous souhaitons. Pour commencer,
réfléchissons à `new`. Précédemment, nous avions choisi un type sans signe pour
le paramètre `taille`, car un groupe avec un nombre négatif de tâches n'a pas de
sens. Cependant, un groupe avec aucune tâche n'a pas non plus de sens, alors que
zéro est une valeur parfaitement valide pour `usize`. Nous allons ajouter du
code pour vérifier que `taille` est plus grand que zéro avant de retourner une
instance de `GroupeTaille` et faire en sorte que le programme panique s'il
reçoit un zéro, en utilisant la macro `assert!` comme dans l'encart 20-13.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Filename : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-13/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-13/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-13: Implementing `ThreadPool::new` to panic if
`size` is zero</span>
-->

<span class="caption">Encart 20-13 : implémentation de `GroupeTaches::new` qui
devrait paniquer si `taille` vaut zéro</span>

<!--
We’ve added some documentation for our `ThreadPool` with doc comments. Note
that we followed good documentation practices by adding a section that calls
out the situations in which our function can panic, as discussed in Chapter 14.
Try running `cargo doc --open` and clicking the `ThreadPool` struct to see what
the generated docs for `new` look like!
-->

Nous avons ajouté un peu de documentation pour notre `GroupeTaches` avec des
commentaires de documentation. Remarquez que nous avons suivi les pratiques de
bonne documentation en ajoutant une section qui liste les situations pour
lesquelles notre fonction peut paniquer, comme nous l'avons vu dans le
chapitre 14. Essayez de lancer `cargo doc --open` et de cliquer sur la structure
`GroupeTaches` pour voir à quoi ressemble la documentation générée pour `new` !

<!--
Instead of adding the `assert!` macro as we’ve done here, we could make `new`
return a `Result` like we did with `Config::new` in the I/O project in Listing
12-9. But we’ve decided in this case that trying to create a thread pool
without any threads should be an unrecoverable error. If you’re feeling
ambitious, try to write a version of `new` with the following signature to
compare both versions:
-->

Au lieu d'ajouter la macro `assert!` comme nous venons de faire, nous aurions pu
faire en sorte que `new` retourne un `Result` comme nous l'avions fait avec
`Config::new` dans le projet d'entrée/sortie dans l'encart 12-9. Mais nous avons
décidé que dans le cas d'une création d'un groupe de tâche sans aucune tâche
devrait être une erreur irrécupérable. Si vous en sentez l'envie, essayez
d'écrire une version de `new` avec la signature suivante, pour comparer les deux
versions :

<!--
```rust,ignore
pub fn new(size: usize) -> Result<ThreadPool, PoolCreationError> {
```
-->

```rust,ignore
pub fn new(taille: usize) -> Result<GroupeTaches, ErreurGroupeTaches> {
```

<!--
#### Creating Space to Store the Threads
-->

#### Créer l'espace de rangement des tâches

<!--
Now that we have a way to know we have a valid number of threads to store in
the pool, we can create those threads and store them in the `ThreadPool` struct
before returning it. But how do we “store” a thread? Let’s take another look at
the `thread::spawn` signature:
-->

Maintenant que nous avons une manière de savoir si nous avons un nombre valide
de tâches à stocker dans le groupe, nous pouvons créer ces tâches et les stocker
dans la structure `GroupeTaches` avant de la retourner. Mais comment “stocker”
une tâche ? Regardons à nouveau la signature de `thread::spawn` :

<!--
```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```
-->

```rust,ignore
pub fn spawn<F, T>(f: F) -> JoinHandle<T>
    where
        F: FnOnce() -> T,
        F: Send + 'static,
        T: Send + 'static,
```

<!--
The `spawn` function returns a `JoinHandle<T>`, where `T` is the type that the
closure returns. Let’s try using `JoinHandle` too and see what happens. In our
case, the closures we’re passing to the thread pool will handle the connection
and not return anything, so `T` will be the unit type `()`.
-->

La fonction `spawn` retourne un `JoinHandle<T>`, où `T` est le type que retourne
notre fermeture. Essayons d'utiliser nous aussi `JoinHandle` pour voir ce qu'il
se passe. Dans notre cas, les fermetures que nous passons dans le groupe de
tâches vont traiter les connexions mais ne vont rien retourner, donc `T` sera le
type unité, `()`.

<!--
The code in Listing 20-14 will compile but doesn’t create any threads yet.
We’ve changed the definition of `ThreadPool` to hold a vector of
`thread::JoinHandle<()>` instances, initialized the vector with a capacity of
`size`, set up a `for` loop that will run some code to create the threads, and
returned a `ThreadPool` instance containing them.
-->

Le code de l'encart 20-14 va se compiler mais ne va pas encore créer de tâches
pour le moment. Nous avons changé la définition du `GroupeTaches` pour qu'il
possède un vecteur d'instances `thread::JoinHandle<()>`, nous avons initialisé
le vecteur avec une capacité de la valeur de `taille`, mis en place une boucle
`for` qui va exécuter du code pour créer les tâches, et nous avons retourné une
instance de `GroupeTaches` qui les contient.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-14/src/lib.rs:here}}
```
-->

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-14/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-14: Creating a vector for `ThreadPool` to hold
the threads</span>
-->

<span class="caption">Encart 20-14 : création d'un vecteur pour `GroupeTaches`
pour stocker les tâches</span>

<!--
We’ve brought `std::thread` into scope in the library crate, because we’re
using `thread::JoinHandle` as the type of the items in the vector in
`ThreadPool`.
-->

Nous avons importé `std::thread` dans la portée de la crate de bibliothèque, car
nous utilisons `thread::JoinHandle` comme étant le type des éléments du vecteur
dans `GroupeTaches`.

<!--
Once a valid size is received, our `ThreadPool` creates a new vector that can
hold `size` items. We haven’t used the `with_capacity` function in this book
yet, which performs the same task as `Vec::new` but with an important
difference: it preallocates space in the vector. Because we know we need to
store `size` elements in the vector, doing this allocation up front is slightly
more efficient than using `Vec::new`, which resizes itself as elements are
inserted.
-->

Une fois qu'une taille valide est reçue, notre `GroupeTaches` crée un nouveau
vecteur qui peut stocker `taille` éléments. Nous n'avons pas encore utilisé la
fonction `with_capacity` dans ce livre, qui fait la même chose que `Vec::new`
mais avec une grosse différence : elle pré-alloue l'espace dans le vecteur.
Comme nous savons que nous avons besoin de stocker `taille` éléments dans le
vecteur, faire cette allocation en amont est bien plus efficace que d'utiliser
`Vec::new`, qui va se redimentionner lorsque des éléments lui seront rajoutés.

<!--
When you run `cargo check` again, you’ll get a few more warnings, but it should
succeed.
-->

Lorsque vous lancez à nouveau `cargo check`, vous devriez avoir quelques
avertissements en plus, mais cela devrait être un succès.

<!-- markdownlint-disable -->
<!--
#### A `Worker` Struct Responsible for Sending Code from the `ThreadPool` to a Thread
-->
<!-- markdownlint-enable -->

#### Une structure `Operateur` chargé d'envoyer le code de `GroupeTaches` à une tâche

<!--
We left a comment in the `for` loop in Listing 20-14 regarding the creation of
threads. Here, we’ll look at how we actually create threads. The standard
library provides `thread::spawn` as a way to create threads, and
`thread::spawn` expects to get some code the thread should run as soon as the
thread is created. However, in our case, we want to create the threads and have
them *wait* for code that we’ll send later. The standard library’s
implementation of threads doesn’t include any way to do that; we have to
implement it manually.
-->

Nous avons laissé un commentaire dans la boucle `for` dans l'encart 20-14 qui
concernait la création des tâches. Ici, nous allons voir comment nous créer les
tâches. La bibliothèque standard fournit une manière de créer les tâches avec
`thread::spawn`, et `thread::spawn` doit recevoir du code que la tâche doit
exécuter dès que la tâche est créée. Cependant, dans notre cas, nous souhaitons
créer les tâches et qu'elles *attendent* du code que nous leur enverrons plus
tard. L'implémentation des tâches de la bibliothèque standard n'offre pas les
moyens de faire ceci ; nous devons implémenter cela manuellement.

<!--
We’ll implement this behavior by introducing a new data structure between the
`ThreadPool` and the threads that will manage this new behavior. We’ll call
this data structure `Worker`, which is a common term in pooling
implementations. Think of people working in the kitchen at a restaurant: the
workers wait until orders come in from customers, and then they’re responsible
for taking those orders and filling them.
-->

Nous allons implémenter ce comportement en introduisant une nouvelle structure
de données entre le `GroupeTaches` et les tâches qui va gérer ce nouveau
comportement. Nous allons appeler cette structure `Operateur`, qui est souvent
appelé `Worker` dans les implémentations de groupe. C'est comme des personnes
qui travaillent dans la cuisine d'un restaurant : les opérateurs attendent les
commandes des clients, et ils sont chargés de prendre en charge ces commandes et
d'y répondre.

<!--
Instead of storing a vector of `JoinHandle<()>` instances in the thread pool,
we’ll store instances of the `Worker` struct. Each `Worker` will store a single
`JoinHandle<()>` instance. Then we’ll implement a method on `Worker` that will
take a closure of code to run and send it to the already running thread for
execution. We’ll also give each worker an `id` so we can distinguish between
the different workers in the pool when logging or debugging.
-->

Au lieu de stocker un vecteur d'instances `JoinHandle<()>` dans le groupe de
tâches, nous allons stocker les instances de structure `Operateur`. Chaque
`Operateur` va stocker une seule instance de `JoinHandle<()>`. Ensuite nous
implémenterons une méthode sur `Operateur` qui va prendre en argument une
fermeture de code à exécuter et l'envoyer à une tâche qui fonctionne déjà pour
exécution. Nous allons aussi donner à chacun des opérateurs un identifiant `id`
afin que nous puissions distinguer les différents opérateurs dans le groupe
dans les journaux ou lors de déboguages.

<!--
Let’s make the following changes to what happens when we create a `ThreadPool`.
We’ll implement the code that sends the closure to the thread after we have
`Worker` set up in this way:
-->

Appliquons ces changements à l'endroit où nous créons un `GroupeTaches`. Nous
allons implémenter le code de `Operateur` qui envoie la fermeture à la tâche
selon ces instructions :

<!--
1. Define a `Worker` struct that holds an `id` and a `JoinHandle<()>`.
2. Change `ThreadPool` to hold a vector of `Worker` instances.
3. Define a `Worker::new` function that takes an `id` number and returns a
   `Worker` instance that holds the `id` and a thread spawned with an empty
   closure.
4. In `ThreadPool::new`, use the `for` loop counter to generate an `id`, create
   a new `Worker` with that `id`, and store the worker in the vector.
-->

1. Définir une structure `Operateur` qui possède un `id` et un `JoinHandle<()>`.
2. Changer le `GroupeTaches` pour posséder un vecteur d'instances de
   `Operateur`.
3. Définir une fonction `Operateur::new` qui prend en argument un nombre `id`
   et retourne une instance de `Operateur` qui contient le `id` et une tâche
   créée avec une fermeture vide.
4. Dans `GroupeTaches::new`, utilisons le compteur de la boucle `for` pour
   générer un `id`, créer un nouveau `Operateur` avec cet `id`, et stocker
   l'opérateur dans le vecteur.

<!--
If you’re up for a challenge, try implementing these changes on your own before
looking at the code in Listing 20-15.
-->

Si vous vous sentez prêt(e) à relever le défi, essayez de faire ces changements
de votre côté avant de regarder le code de l'encart 20-15.

<!--
Ready? Here is Listing 20-15 with one way to make the preceding modifications.
-->

Vous êtes prêt(e) ? Voici l'encart 20-15 qui propose une solution pour procéder
aux changements listés précédemment.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-15/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-15/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-15: Modifying `ThreadPool` to hold `Worker`
instances instead of holding threads directly</span>
-->

<span class="caption">Encart 20-15 : modification de `GroupeTaches` pour
stocker des instances de `Operateur` plutôt que de stocker directement des
tâches</span>

<!--
We’ve changed the name of the field on `ThreadPool` from `threads` to `workers`
because it’s now holding `Worker` instances instead of `JoinHandle<()>`
instances. We use the counter in the `for` loop as an argument to
`Worker::new`, and we store each new `Worker` in the vector named `workers`.
-->

Nous avons changé le nom du champ `taches` sur `GroupeTaches` par `operateurs`
car il stocke maintenant des instances de `Operateur` plutôt que des instances
de `JoinHandle<()>`. Nous utilisons le compteur de la boucle `for` en argument
de `Operateur::new`, et nous stockons chacun des nouveaux `Operateur` dans le
vecteur `operateurs`.

<!--
External code (like our server in *src/bin/main.rs*) doesn’t need to know the
implementation details regarding using a `Worker` struct within `ThreadPool`,
so we make the `Worker` struct and its `new` function private. The
`Worker::new` function uses the `id` we give it and stores a `JoinHandle<()>`
instance that is created by spawning a new thread using an empty closure.
-->

Le code externe (comme celui de notre serveur dans *src/bin/main.rs*) n'a pas
besoin de connaître les détails de l'implémentation qui utilise une structure
`Operateur` dans `GroupeTaches`, donc nous faisons en sorte que la structure
`Operateur` et sa fonction `new` restent privées. La fonction `Operateur::new`
utilise le `id` que nous lui donnons et stocke une instance de `JoinHandle<()>`
qui est créée en créant une nouvelle tâche en utilisant une fermeture vide.

<!--
This code will compile and will store the number of `Worker` instances we
specified as an argument to `ThreadPool::new`. But we’re *still* not processing
the closure that we get in `execute`. Let’s look at how to do that next.
-->

Ce code va se compiler et stocker le nombre d'instances de `Operateur` que nous
avons renseigné en argument de `GroupeTaches::new`. Mais nous n'exécutons
*toujours pas* la fermeture que nous obtenons de `executer`. Voyons désormais
comment faire cela.

<!--
#### Sending Requests to Threads via Channels
-->

#### Envoyer des requêtes à des tâches via les canaux

<!--
Now we’ll tackle the problem that the closures given to `thread::spawn` do
absolutely nothing. Currently, we get the closure we want to execute in the
`execute` method. But we need to give `thread::spawn` a closure to run when we
create each `Worker` during the creation of the `ThreadPool`.
-->

Maintenant nous allons nous pencher sur le problème qui fait que les fermetures
passées à `thread::spawn` ne font absolument rien. Actuellement, nous obtenons
la fermeture que nous souhaitons exécuter dans la méthode `executer`. Mais nous
avons besoin de donner une fermeture à `thread::spawn` pour qu'elle l'exécute
lorsque nous créons chaque `Operateur` pendant la création de `GroupeTaches`.

<!--
We want the `Worker` structs that we just created to fetch code to run from a
queue held in the `ThreadPool` and send that code to its thread to run.
-->

Nous souhaitons que les structures `Operateur` que nous venons de créer
récupèrent du code à exécuter dans une liste d'attente présente dans le
`GroupeTaches` et renvoient ce code à leur tâche pour l'exécuter.

<!--
In Chapter 16, you learned about *channels*—a simple way to communicate between
two threads—that would be perfect for this use case. We’ll use a channel to
function as the queue of jobs, and `execute` will send a job from the
`ThreadPool` to the `Worker` instances, which will send the job to its thread.
Here is the plan:
-->

Dans le chapitre 16, vous avez appris les *canaux* (une manière simple de
communiquer entre deux tâches) qui seront parfaits pour ce cas d'emploi. Nous
allons utiliser un canal pour les fonctions pour créer la liste d'attente des
missions, et `executer` devrait envoyer une mission de `GroupeTaches` vers les
instances `Operateur`, qui vont passer la mission à leurs tâches. Voici le
plan :

<!--
1. The `ThreadPool` will create a channel and hold on to the sending side of
   the channel.
2. Each `Worker` will hold on to the receiving side of the channel.
3. We’ll create a new `Job` struct that will hold the closures we want to send
   down the channel.
4. The `execute` method will send the job it wants to execute down the sending
   side of the channel.
5. In its thread, the `Worker` will loop over its receiving side of the channel
   and execute the closures of any jobs it receives.
-->

1. Le `GroupeTaches` va créer un canal et conserver la partie d'envoi du canal.
2. Chaque `Operateur` va conserver la partie de réception du canal.
3. Nous allons créer une nouvelle structure `Mission` qui va stocker les
   fermetures que nous souhaitons envoyer dans le canal.
4. La méthode `executer` va envoyer la mission qu'elle souhaite executer dans
   la zone d'envoi du canal.
5. Dans sa propre tâche, le `Operateur` va vérifier en permanence la partie
   réception du canal et exécuter les fermetures des missions qu'il va
   recevoir.

<!--
Let’s start by creating a channel in `ThreadPool::new` and holding the sending
side in the `ThreadPool` instance, as shown in Listing 20-16. The `Job` struct
doesn’t hold anything for now but will be the type of item we’re sending down
the channel.
-->

Commençons par créer un canal dans `GroupeTaches::new` et stocker la partie
d'envoi dans l'instance de `GroupeTaches`, comme dans l'encart 20-16. La
structure `Mission` ne contient rien pour le moment, mais sera le type
d'éléments que nous enverrons dans le canal.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-16/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-16/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-16: Modifying `ThreadPool` to store the
sending end of a channel that sends `Job` instances</span>
-->

<span class="caption">Encart 20-16 : modification de `GroupeTaches` pour
stocker la partie d'envoi du canal qui envoie des instances de `Mission`</span>

<!--
In `ThreadPool::new`, we create our new channel and have the pool hold the
sending end. This will successfully compile, still with warnings.
-->

Dans `GroupeTaches::new`, nous créons notre nouveau canal et faisons en sorte
que le groupe stocke la partie d'envoi. Cela devrait pouvoir se compiler, mais
il subsiste des avertissements.

<!--
Let’s try passing a receiving end of the channel into each worker as the thread
pool creates the channel. We know we want to use the receiving end in the
thread that the workers spawn, so we’ll reference the `receiver` parameter in
the closure. The code in Listing 20-17 won’t quite compile yet.
-->

Essayons de donner la partie réceptrice du canal à chacun des opérateurs
lorsque le groupe de tâches crée le canal. Nous savons que nous voulons
utiliser la partie réceptrice dans la tâche que l'opérateur utilise, donc nous
allons créer une référence vers le paramètre `reception` dans la fermeture. Le
code de l'encart 20-17 ne se compile pas encore.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-17/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-17/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-17: Passing the receiving end of the channel
to the workers</span>
-->

<span class="caption">Encart 20-17 : envoi de la partie réceptrice du canal aux
opérateurs</span>

<!--
We’ve made some small and straightforward changes: we pass the receiving end of
the channel into `Worker::new`, and then we use it inside the closure.
-->

Nous avons fait des petites et simples modifications : nous envoyons la partie
réceptrice du canal dans `Operateur::new`, et ensuite nous l'utilisons dans la
fermeture.

<!--
When we try to check this code, we get this error:
-->

Lorsque nous essayons de vérifier ce code, nous obtenons cette erreur :

<!--
```console
{{#include ../listings-sources/ch20-web-server/listing-20-17/output.txt}}
```
-->

```console
{{#include ../listings/ch20-web-server/listing-20-17/output.txt}}
```

<!--
The code is trying to pass `receiver` to multiple `Worker` instances. This
won’t work, as you’ll recall from Chapter 16: the channel implementation that
Rust provides is multiple *producer*, single *consumer*. This means we can’t
just clone the consuming end of the channel to fix this code. Even if we could,
that is not the technique we would want to use; instead, we want to distribute
the jobs across threads by sharing the single `receiver` among all the workers.
-->

Le code essaye d'envoyer `reception` dans plusieurs instances de `Operateur`.
Ceci ne fonctionne pas, comme vous l'avez appris au chapitre 16 :
l'implémentation du canal que fournit Rust est du type plusieurs *producteurs*,
un seul *consommateur*. Cela signifie que nous ne pouvons pas simplement cloner
la partie réceptrice du canal pour corriger ce code. Même si nous aurions pu le
faire, ce n'est pas la solution que nous souhaitons utiliser ; nous voulons
plutôt distribuer les missions entre les tâches en partageant la même réception
entre tous les opérateurs.

<!--
Additionally, taking a job off the channel queue involves mutating the
`receiver`, so the threads need a safe way to share and modify `receiver`;
otherwise, we might get race conditions (as covered in Chapter 16).
-->

De plus, obtenir une mission de la file d'attente du canal implique de muter le
`reception`, donc les tâches ont besoin d'une méthode sécurisée pour partager
et modifier `reception` ; autrement, nous allons avoir des situations de
concurrence (comme nous l'avons vu dans le chapitre 16).

<!--
Recall the thread-safe smart pointers discussed in Chapter 16: to share
ownership across multiple threads and allow the threads to mutate the value, we
need to use `Arc<Mutex<T>>`. The `Arc` type will let multiple workers own the
receiver, and `Mutex` will ensure that only one worker gets a job from the
receiver at a time. Listing 20-18 shows the changes we need to make.
-->

Souvenez-vous des pointeurs intelligents conçus pour les échanges entre les
tâches que nous avons vus au chapitre 16 : pour partager la possession entre
plusieurs tâches et permettre aux tâches de muter la valeur, nous avons besoin
d'utiliser `Arc<Mutex<T>>`. Le type `Arc` va permettre à plusieurs opérateurs
de posséder la réception, et `Mutex` va s'assurer que seulement un seul
opérateur obtienne la mission dans la réception au même moment. L'encart 20-18
montre les changements que nous devons apporter.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-18/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-18/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-18: Sharing the receiving end of the channel
among the workers using `Arc` and `Mutex`</span>
-->

<span class="caption">Encart 20-18 : partage de la partie réceptrice du canal
entre les opérateurs en utilisant `Arc` et `Mutex`</span>

<!--
In `ThreadPool::new`, we put the receiving end of the channel in an `Arc` and a
`Mutex`. For each new worker, we clone the `Arc` to bump the reference count so
the workers can share ownership of the receiving end.
-->

Dans `GroupeTaches::new`, nous installons la partie réceptrice du canal dans un
`Arc` et un `Mutex`. Pour chaque nouvel opérateur, nous clonons le `Arc` pour
augmenter le compteur de références afin que les opérateurs puissent se
partager la possession de la partie réceptrice.

<!--
With these changes, the code compiles! We’re getting there!
-->

Grâce à ces changements, le code se compile ! Nous touchons au but !

<!--
#### Implementing the `execute` Method
-->

#### Implémenter la méthode `executer`

<!--
Let’s finally implement the `execute` method on `ThreadPool`. We’ll also change
`Job` from a struct to a type alias for a trait object that holds the type of
closure that `execute` receives. As discussed in the [“Creating Type Synonyms
with Type Aliases”][creating-type-synonyms-with-type-aliases]<!-- ignore -- >
section of Chapter 19, type aliases allow us to make long types shorter. Look
at Listing 20-19.
-->

Finissons par implémenter la méthode `executer` sur `GroupeTaches`. Nous allons
aussi modifier la structure `Mission` pour devenir un alias de type pour un
objet trait qui contiendra le type de la fermeture que `executer` recevra.
Comme nous l'avons vu dans [une section du
chapitre 19][creating-type-synonyms-with-type-aliases]<!-- ignore -->, les
alias de type nous permettent de raccourcir les types un peu trop longs.
Voyez cela dans l'encart 20-19.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-19/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-19/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-19: Creating a `Job` type alias for a `Box`
that holds each closure and then sending the job down the channel</span>
-->

<span class="caption">Encart 20-19 : création d'un alias de type `Mission`
pour une `Box` qui contient chaque fermeture et qui transportera la mission
dans le canal</span>

<!--
After creating a new `Job` instance using the closure we get in `execute`, we
send that job down the sending end of the channel. We’re calling `unwrap` on
`send` for the case that sending fails. This might happen if, for example, we
stop all our threads from executing, meaning the receiving end has stopped
receiving new messages. At the moment, we can’t stop our threads from
executing: our threads continue executing as long as the pool exists. The
reason we use `unwrap` is that we know the failure case won’t happen, but the
compiler doesn’t know that.
-->

Après avoir créé une nouvelle instance `Mission` en utilisant la fermeture que
nous obtenons dans `executer`, nous envoyons cette mission dans le canal via la
partie émettrice. Nous utilisons `unwrap` sur `send` pour les cas où l'envoi
échoue. Cela peut arriver si, par exemple, nous stoppons l'exécution de toutes
les tâches, ce qui signifiera que les parties réceptrices auront finis de
recevoir des nouveaux messages. Pour le moment, nous ne pouvons pas stopper
l'exécution de nos tâches : nos tâches continuerons à s'exécuter aussi
longtemps que le groupe existe. La raison pour laquelle nous utilisons `unwrap`
est que nous savons que le cas d'échec ne va pas se produire, mais le
compilateur ne le sait pas.

<!--
But we’re not quite done yet! In the worker, our closure being passed to
`thread::spawn` still only *references* the receiving end of the channel.
Instead, we need the closure to loop forever, asking the receiving end of the
channel for a job and running the job when it gets one. Let’s make the change
shown in Listing 20-20 to `Worker::new`.
-->

Mais nous n'avons pas encore fini ! Dans l'opérateur, notre fermeture envoyée
à `thread::spawn` ne fait que *référencer* la sortie du canal. Nous avons
plutôt besoin d'une fermeture pour faire une boucle à l'infini, qui demandera
une mission à la sortie du canal et exécuter cette mission lorsqu'il en obtient
un. Appliquons les changements montrés dans l'encart 20-20 à `Operateur::new`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-20/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/listing-20-20/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-20: Receiving and executing the jobs in the
worker’s thread</span>
-->

<span class="caption">Encart 20-20 : réception et exécution des missions dans
la tâche de l'opérateur</span>

<!--
Here, we first call `lock` on the `receiver` to acquire the mutex, and then we
call `unwrap` to panic on any errors. Acquiring a lock might fail if the mutex
is in a *poisoned* state, which can happen if some other thread panicked while
holding the lock rather than releasing the lock. In this situation, calling
`unwrap` to have this thread panic is the correct action to take. Feel free to
change this `unwrap` to an `expect` with an error message that is meaningful to
you.
-->

Ici, nous faisons d'abord appel à `lock` sur `reception` pour obtenir le mutex,
et ensuite nous faisons appel à `unwrap` pour paniquer dès qu'il y a une
erreur. L'acquisition d'un verrou peut échouer si le mutex est dans un état
*empoisonné*, ce qui peut arriver si d'autres tâches ont paniqué pendant
qu'elles avaient le verrou, au lieu de le rendre. Dans cette situation, l'appel
à `unwrap` fera paniquer la tâche, ce qui est la bonne chose à faire. Vous
pouvez aussi changer ce `unwrap` en un `expect` avec un message d'erreur qui
vous est plus explicite.

<!--
If we get the lock on the mutex, we call `recv` to receive a `Job` from the
channel. A final `unwrap` moves past any errors here as well, which might occur
if the thread holding the sending side of the channel has shut down, similar to
how the `send` method returns `Err` if the receiving side shuts down.
-->

Si nous obtenons le verrou du mutex, nous faisons appel à `recv` pour recevoir
une `Mission` provenant du canal. Un `unwrap` final s'occupe lui aussi des cas
d'erreurs, qui peuvent se produire si la tâche qui contient la partie émettrice
du canal se termine, de la même manière que la méthode `send` enverrait `Err`
si la partie réceptrice se fermerait.

<!--
The call to `recv` blocks, so if there is no job yet, the current thread will
wait until a job becomes available. The `Mutex<T>` ensures that only one
`Worker` thread at a time is trying to request a job.
-->

L'appel à `recv` bloque l'exécution, donc s'il n'y a pas encore de mission, la
tâche courante va attendre jusqu'à ce qu'une mission soit disponible. Le
`Mutex<T>` s'assure qu'une seule tâche d'`Operateur` obtienne une même mission
à la fois.

<!--
With the implementation of this trick, our thread pool is in a working state!
Give it a `cargo run` and make some requests:
-->

Avec l'implémentation de cette astuce, notre groupe de tâches est en état de
fonctionner ! Faites un `cargo run` et faites quelques requêtes :

<!--
<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-20
cargo run
make some requests to 127.0.0.1:7878
Can't automate because the output depends on making requests
-- >
-->

<!--
```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
warning: field is never read: `workers`
 -- > src/lib.rs:7:5
  |
7 |     workers: Vec<Worker>,
  |     ^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: field is never read: `id`
  -- > src/lib.rs:48:5
   |
48 |     id: usize,
   |     ^^^^^^^^^

warning: field is never read: `thread`
  -- > src/lib.rs:49:5
   |
49 |     thread: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/main`
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
Worker 1 got a job; executing.
Worker 3 got a job; executing.
Worker 0 got a job; executing.
Worker 2 got a job; executing.
```
-->

```console
$ cargo run
   Compiling salutations v0.1.0 (file:///projects/salutations)
warning: field is never read: `operateurs`
 -- > src/lib.rs:7:5
  |
7 |     operateurs: Vec<Operateur>,
  |     ^^^^^^^^^^^^^^^^^^^^^^^^^^
  |
  = note: `#[warn(dead_code)]` on by default

warning: field is never read: `id`
  -- > src/lib.rs:48:5
   |
48 |     id: usize,
   |     ^^^^^^^^^

warning: field is never read: `tache`
  -- > src/lib.rs:49:5
   |
49 |     tache: thread::JoinHandle<()>,
   |     ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^

warning: 3 warnings emitted

    Finished dev [unoptimized + debuginfo] target(s) in 1.40s
     Running `target/debug/main`
L'opérateur 0 a obtenu une mission ; il l'exécute.
L'opérateur 2 a obtenu une mission ; il l'exécute.
L'opérateur 1 a obtenu une mission ; il l'exécute.
L'opérateur 3 a obtenu une mission ; il l'exécute.
L'opérateur 0 a obtenu une mission ; il l'exécute.
L'opérateur 2 a obtenu une mission ; il l'exécute.
L'opérateur 1 a obtenu une mission ; il l'exécute.
L'opérateur 3 a obtenu une mission ; il l'exécute.
L'opérateur 0 a obtenu une mission ; il l'exécute.
L'opérateur 2 a obtenu une mission ; il l'exécute.
```

<!--
Success! We now have a thread pool that executes connections asynchronously.
There are never more than four threads created, so our system won’t get
overloaded if the server receives a lot of requests. If we make a request to
*/sleep*, the server will be able to serve other requests by having another
thread run them.
-->

Parfait ! Nous avons maintenant un groupe de tâches qui exécute des connexions
de manière asynchrone. Il n'y a pas plus que quatre tâches qui sont créées,
donc notre système ne sera pas surchargé si le serveur reçoit beaucoup de
requêtes. Si nous faisons une requête vers */pause*, le serveur sera toujours
capable de servir les autres requêtes grâce aux autres tâches qui pourront les
exécuter.

<!--
> Note: if you open */sleep* in multiple browser windows simultaneously, they
> might load one at a time in 5 second intervals. Some web browsers execute
> multiple instances of the same request sequentially for caching reasons. This
> limitation is not caused by our web server.
-->

> Remarque : si vous ouvrez */pause* dans plusieurs fenêtres de navigation en
> simultané, elles peuvent parfois être chargées une par une avec 5 secondes
> d'intervalle. Certains navigateurs web exécutent plusieurs instances de la
> même requête de manière séquentielle pour des raisons de cache. Cette
> limitation n'est pas la faute de notre serveur web.

<!--
After learning about the `while let` loop in Chapter 18, you might be wondering
why we didn’t write the worker thread code as shown in Listing 20-21.
-->

Après avoir appris la boucle `while let` dans le chapitre 18, vous pourriez
vous demander pourquoi nous n'avons pas écrit le code des tâches des opérateurs
comme dans l'encart 20-21.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-21/src/lib.rs:here}}
```
-->

```rust,ignore,not_desired_behavior
{{#rustdoc_include ../listings/ch20-web-server/listing-20-21/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-21: An alternative implementation of
`Worker::new` using `while let`</span>
-->

<span class="caption">Encart 20-21 : une implémentation alternative de
`Operateur::new` qui utilise `while let`</span>

<!--
This code compiles and runs but doesn’t result in the desired threading
behavior: a slow request will still cause other requests to wait to be
processed. The reason is somewhat subtle: the `Mutex` struct has no public
`unlock` method because the ownership of the lock is based on the lifetime of
the `MutexGuard<T>` within the `LockResult<MutexGuard<T>>` that the `lock`
method returns. At compile time, the borrow checker can then enforce the rule
that a resource guarded by a `Mutex` cannot be accessed unless we hold the
lock. But this implementation can also result in the lock being held longer
than intended if we don’t think carefully about the lifetime of the
`MutexGuard<T>`.
-->

Ce code se compile et s'exécute mais ne se comporte pas comme nous
souhaiterions que les tâches se comportent : une requête lente à traiter va
continuer à faire en sorte que les autres requêtes vont attendre d'être
traitées. La raison à cela est subtile : la structure `Mutex` n'a pas de
méthode publique `unlock` car la propriété du verrou se base sur la durée de
vie du `MutexGuard<T>` au sein du `LockResult<MutexGuard<T>>` que retourne la
méthode `lock`. A la compilation, le vérificateur d'emprunt peut ensuite
vérifier la règle qui dit qu'une ressource gardée par un `Mutex` ne peut pas
être accessible que si nous avons ce verrou. Mais cette implémentation peut
aussi faire en sorte que nous gardions le verrou plus longtemps que prévu si
nous ne réfléchissons pas avec attention sur la durée de vie du
`MutexGuard<T>`.

<!--
The code in Listing 20-20 that uses `let job =
receiver.lock().unwrap().recv().unwrap();` works because with `let`, any
temporary values used in the expression on the right hand side of the equals
sign are immediately dropped when the `let` statement ends. However, `while
let` (and `if let` and `match`) does not drop temporary values until the end of
the associated block. In Listing 20-21, the lock remains held for the duration
of the call to `job()`, meaning other workers cannot receive jobs.
-->

Le code de l'encart 20-20 qui utilise `let mission =
reception.lock().unwrap().recv().unwrap();` fonctionne, car avec `let`, toute
valeur temporaire utilisée dans la partie droite du signe égal est libérée
immédiatement lorsque l'instruction `let` se termine. Cependant, `while let` (
ainsi que `if let` et `match`) ne libèrent pas les valeurs temporaires avant la
fin du bloc associé. Dans l'encart 20-21, le verrou continue à être maintenu
pendant toute la durée de l'appel à `mission()`, ce qui veut dire que les
autres opérateurs ne peuvent pas recevoir des tâches.

<!--
[creating-type-synonyms-with-type-aliases]:
ch19-04-advanced-types.html#creating-type-synonyms-with-type-aliases
[integer-types]: ch03-02-data-types.html#integer-types
[storing-closures-using-generic-parameters-and-the-fn-traits]:
ch13-01-closures.html#storing-closures-using-generic-parameters-and-the-fn-traits
-->

[creating-type-synonyms-with-type-aliases]: ch19-04-advanced-types.html
[integer-types]: ch03-02-data-types.html
[storing-closures-using-generic-parameters-and-the-fn-traits]:
ch13-01-closures.html
