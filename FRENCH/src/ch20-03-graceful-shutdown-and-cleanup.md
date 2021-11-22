> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/217).

<!--
## Graceful Shutdown and Cleanup
-->

## Arrêt propre et nettoyage

<!--
The code in Listing 20-20 is responding to requests asynchronously through the
use of a thread pool, as we intended. We get some warnings about the `workers`,
`id`, and `thread` fields that we’re not using in a direct way that reminds us
we’re not cleaning up anything. When we use the less elegant <span
class="keystroke">ctrl-c</span> method to halt the main thread, all other
threads are stopped immediately as well, even if they’re in the middle of
serving a request.
-->

Le code de l'encart 20-20 réponds aux requêtes de manière asynchrone grâce à
l'utilisation du groupe de tâches, comme nous l'espérions. Nous avons quelques
avertissements sur les champs `operateurs`, `id` et `tâche` que nous
n'utilisons pas directement qui nous rappelle que nous ne nettoyons rien.
Lorsque nous utilisons la méthode <span class="keystroke">ctrl-c</span> pour
terminer la tâche principale, toutes les autres tâches sont aussi stoppées
immédiatement, même si nous sommes en train de servir une requête.

<!--
Now we’ll implement the `Drop` trait to call `join` on each of the threads in
the pool so they can finish the requests they’re working on before closing.
Then we’ll implement a way to tell the threads they should stop accepting new
requests and shut down. To see this code in action, we’ll modify our server to
accept only two requests before gracefully shutting down its thread pool.
-->

Nous allons implémenter le trait `Drop` pour faire appel à `join` sur chacune
des tâches dans le groupe afin qu'elles puissent finir les requêtes qu'elles
sont en train de traiter avant l'arrêt. Ensuite, nous allons implémenter un
moyen de demander aux tâches d'arrêter d'accepter des nouvelles requêtes et de
s'arrêter. Pour voir ce code à l'action, nous allons modifier notre serveur
pour accepter seulement deux requêtes avant d'arrêter proprement son groupe de
tâches.

<!--
### Implementing the `Drop` Trait on `ThreadPool`
-->

### Implémenter le trait `Drop` sur `GroupeTaches`

<!--
Let’s start with implementing `Drop` on our thread pool. When the pool is
dropped, our threads should all join to make sure they finish their work.
Listing 20-22 shows a first attempt at a `Drop` implementation; this code won’t
quite work yet.
-->

Commençons par implémenter `Drop` sur notre groupe de tâches. Lorsque le groupe
est nettoyé, nos tâches doivent toutes faire appel à `join` pour s'assurer
qu'elles finissent leur travail. L'encart 20-22 montre une première tentative
d'implémentation de `Drop` ; ce code ne fonctionne pas encore tout à fait.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-22/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/listing-20-22/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-22: Joining each thread when the thread pool
goes out of scope</span>
-->

<span class="caption">Encart 20-22 : utilisation de `join` sur chaque tâche
lorsque le groupe de tâches sort de la portée</span>

<!--
First, we loop through each of the thread pool `workers`. We use `&mut` for
this because `self` is a mutable reference, and we also need to be able to
mutate `worker`. For each worker, we print a message saying that this
particular worker is shutting down, and then we call `join` on that worker’s
thread. If the call to `join` fails, we use `unwrap` to make Rust panic and go
into an ungraceful shutdown.
-->

D'abord, nous faisons une boucle sur les `operateurs`. Nous utilisons `&mut`
pour cela car `self` est une référence mutable, et nous avons aussi besoin de
pouvoir muter `operateur`. Pour chaque opérateur, nous affichons un message qui
indique que cet opérateur s'arrête, et ensuite nous faisons appel à `join` sur
la tâche de cet opérateur. Si l'appel à `join` échoue, nous utilisons `unwrap`
pour faire paniquer Rust et ainsi procéder à un arrêt brutal.

<!--
Here is the error we get when we compile this code:
-->

Voici l'erreur que nous obtenons lorsque nous compilons ce code :

<!--
```console
{{#include ../listings-sources/ch20-web-server/listing-20-22/output.txt}}
```
-->

```console
{{#include ../listings/ch20-web-server/listing-20-22/output.txt}}
```

<!--
The error tells us we can’t call `join` because we only have a mutable borrow
of each `worker` and `join` takes ownership of its argument. To solve this
issue, we need to move the thread out of the `Worker` instance that owns
`thread` so `join` can consume the thread. We did this in Listing 17-15: if
`Worker` holds an `Option<thread::JoinHandle<()>>` instead, we can call the
`take` method on the `Option` to move the value out of the `Some` variant and
leave a `None` variant in its place. In other words, a `Worker` that is running
will have a `Some` variant in `thread`, and when we want to clean up a
`Worker`, we’ll replace `Some` with `None` so the `Worker` doesn’t have a
thread to run.
-->

L'erreur nous informe que nous ne pouvons pas faire appel à `join` car nous
faisons seulement un emprunt mutable pour chacun des `operateur` et que `join`
prend possession de son argument. Pour résoudre ce problème, nous devons
sortir la tâche de l'instance de `Operateur` qui possède la `tache` afin que
`join` puisse consommer la tâche. Nous faisons ceci dans l'encart 17-15 : si
`Operateur` contient un `Option<thread::JoinHandle<()>>`, nous pouvons utiliser
la méthode `take` sur `Option` pour sortir la valeur de la variante `Some` et
la remplacer par la variante `None` à la place. Autrement dit, un `Operateur`
qui est en cours d'exécution aura une variante `Some` dans `tache`, et lorsque
nous souhaitons nettoyer `Operateur`, nous remplacerons `Some` par `None` afin
que `Operateur` n'ai pas de tâche à exécuter.

<!--
So we know we want to update the definition of `Worker` like this:
-->

Donc nous savons que nous voulons modifier la définition de `Operateur` comme
ceci :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-04-update-worker-definition/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch20-web-server/no-listing-04-update-worker-definition/src/lib.rs:here}}
```

<!--
Now let’s lean on the compiler to find the other places that need to change.
Checking this code, we get two errors:
-->

Maintenant, aidons-nous du compilateur pour trouver les autres endroits qui ont
besoin de changer. En vérifiant ce code, nous obtenons deux erreurs :

<!--
```console
{{#include ../listings-sources/ch20-web-server/no-listing-04-update-worker-definition/output.txt}}
```
-->

```console
{{#include ../listings/ch20-web-server/no-listing-04-update-worker-definition/output.txt}}
```

<!--
Let’s address the second error, which points to the code at the end of
`Worker::new`; we need to wrap the `thread` value in `Some` when we create a
new `Worker`. Make the following changes to fix this error:
-->

Corrigeons la seconde erreur, qui se situe dans le code à la fin de
`Operateur::new` ; nous devons intégrer la valeur de `tache` dans un `Some`
lorsque nous créons un nouvel `Operateur`. Faites les changements suivants pour
corriger cette erreur :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-05-fix-worker-new/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-05-fix-worker-new/src/lib.rs:here}}
```


<!--
The first error is in our `Drop` implementation. We mentioned earlier that we
intended to call `take` on the `Option` value to move `thread` out of `worker`.
The following changes will do so:
-->

La première erreur se situe dans notre implémentation de `Drop`. Nous avions
mentionné plus tôt que nous voulions faire appel à `take` sur la valeur de
`Option` pour déplacer `tache` en dehors de `operateur`. Voici les changements
à apporter pour ceci :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-06-fix-threadpool-drop/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-06-fix-threadpool-drop/src/lib.rs:here}}
```

<!--
As discussed in Chapter 17, the `take` method on `Option` takes the `Some`
variant out and leaves `None` in its place. We’re using `if let` to destructure
the `Some` and get the thread; then we call `join` on the thread. If a worker’s
thread is already `None`, we know that worker has already had its thread
cleaned up, so nothing happens in that case.
-->

Comme nous l'avons vu au chapitre 17, la méthode `take` sur `Option` sort la
variante `Some` et laisse un `None` à la place. Nous utilisons `if let` pour
destructurer le `Some` et obtenir la tâche ; ensuite nous faisons appel à `join`
sur cette tâche. Si la tâche d'un opérateur est déjà un `None`, nous savons que
cet opérateur a déjà nettoyé sa tâche, donc nous ne faisons rien dans ce cas.

<!--
### Signaling to the Threads to Stop Listening for Jobs
-->

### Demander aux tâches d'arrêter d'attendre des missions

<!--
With all the changes we’ve made, our code compiles without any warnings. But
the bad news is this code doesn’t function the way we want it to yet. The key
is the logic in the closures run by the threads of the `Worker` instances: at
the moment, we call `join`, but that won’t shut down the threads because they
`loop` forever looking for jobs. If we try to drop our `ThreadPool` with our
current implementation of `drop`, the main thread will block forever waiting
for the first thread to finish.
-->

Avec tous ces changements, notre code se compile sans aucun avertissement. Mais
la mauvaise nouvelle est que ce code ne fonctionne pas de la façon dont nous
souhaitons, pour l'instant. La cause se situe dans la logique des fermetures
qui sont exécutées par les tâches des instances de `Operateur` : pour le
moment, nous faisons appel à `join`, mais cela ne va pas arrêter les
tâches car elles font une boucle infinie avec `loop` pour attendre des
missions. Si nous essayons de nettoyer notre `GroupeTaches` avec
l'implémentation actuelle de `drop`, la tâche principale va se bloquer à
l'infini en attendant que la première tâche se termine.

<!--
To fix this problem, we’ll modify the threads so they listen for either a `Job`
to run or a signal that they should stop listening and exit the infinite loop.
Instead of `Job` instances, our channel will send one of these two enum
variants.
-->

Pour corriger ce problème, nous allons modifier les tâches pour qu'elles
attendent soit une `Mission` à exécuter, ou le signal qui leur dit qu'elles
doivent arrêter d'attendre des missions et arrêter la boucle infinie. Notre
canal va envoyer une de ces deux variantes d'énumération au lieu d'uniquement
des instances de `Job`.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-07-define-message-enum/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-07-define-message-enum/src/lib.rs:here}}
```

<!--
This `Message` enum will either be a `NewJob` variant that holds the `Job` the
thread should run, or it will be a `Terminate` variant that will cause the
thread to exit its loop and stop.
-->

Cette énumération `Message` aura pour valeurs une variante `NouvelleMission`
qui contiendra la `Mission` que la tâche devra exécuter, ou la variante
`Extinction` qui va faire en sorte que la tâche sorte de sa boucle et se
termine.

<!--
We need to adjust the channel to use values of type `Message` rather than type
`Job`, as shown in Listing 20-23.
-->

Nous devons corriger le canal pour utiliser les valeurs du type `Message`
plutôt que le type `Mission`, comme dans l'encart 20-23.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-23/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-23/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-23: Sending and receiving `Message` values and
exiting the loop if a `Worker` receives `Message::Terminate`</span>
-->

<span class="caption">Encart 20-23 : envoi et réception de valeurs de `Message`
et sortie de la boucle si un `Operateur` reçoit `Message:Extinction`</span>

<!--
To incorporate the `Message` enum, we need to change `Job` to `Message` in two
places: the definition of `ThreadPool` and the signature of `Worker::new`. The
`execute` method of `ThreadPool` needs to send jobs wrapped in the
`Message::NewJob` variant. Then, in `Worker::new` where a `Message` is received
from the channel, the job will be processed if the `NewJob` variant is
received, and the thread will break out of the loop if the `Terminate` variant
is received.
-->

Pour intégrer l'énumération `Message`, nous devons changer `Mission` par
`Message` à deux endroits : la définition de `GroupeTaches` et la signature de
`Operateur::new`. La méthode `executer` de `GroupeTaches` doit envoyer des
missions intégrées dans des variantes de `Message::NouvelleTache`. Ensuite,
dans `Operateur::new` où nous recevons des `Message` du canal, la mission sera
traitée si la variante `NouvelleTache` est reçue, et la tâche arrêtera la
boucle si la variante `Extinction` est reçue.

<!--
With these changes, the code will compile and continue to function in the same
way as it did after Listing 20-20. But we’ll get a warning because we aren’t
creating any messages of the `Terminate` variety. Let’s fix this warning by
changing our `Drop` implementation to look like Listing 20-24.
-->

Grâce à ces changements, le code va se compiler et continuer de fonctionner de
la même manière qu'il le faisait après l'encart 20-20. Mais nous allons obtenir
un avertissement car nous ne créons aucun message de la variante `Extinction`.
Corrigeons cet avertissement en corrigeant l'implémentation de notre `Drop`
pour qu'elle ressemble à l'encart 20-24.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-24/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-24/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 20-24: Sending `Message::Terminate` to the
workers before calling `join` on each worker thread</span>
-->

<span class="caption">Encart 20-24 : envoi de `Message::Extinction` aux
opérateurs avant de faire appel à `join` sur toutes les tâches des
opérateurs</span>

<!--
We’re now iterating over the workers twice: once to send one `Terminate`
message for each worker and once to call `join` on each worker’s thread. If we
tried to send a message and `join` immediately in the same loop, we couldn’t
guarantee that the worker in the current iteration would be the one to get the
message from the channel.
-->

Nous itérons deux fois sur les opérateurs : une fois pour envoyer un message
`Extinction` pour chaque opérateur, et une fois pour utiliser `join` sur la
tâche de chacun des opérateurs. Si nous essayons d'envoyer le message et
d'utiliser immédiatement `join` dans la même boucle, nous ne pouvons pas
garantir que l'opérateur dans l'itération en cours sera celui qui obtiendra le
message envoyé dans le canal.

<!--
To better understand why we need two separate loops, imagine a scenario with
two workers. If we used a single loop to iterate through each worker, on the
first iteration a terminate message would be sent down the channel and `join`
called on the first worker’s thread. If that first worker was busy processing a
request at that moment, the second worker would pick up the terminate message
from the channel and shut down. We would be left waiting on the first worker to
shut down, but it never would because the second thread picked up the terminate
message. Deadlock!
-->

Pour mieux comprendre pourquoi nous avons besoin de séparer les boucles,
imaginez un scénario avec deux opérateurs. Si nous avions utilisé une seule
boucle pour itérer sur chacun des opérateurs, dans la première itération un
message d'extinction sera envoyé dans le canal et `join` sera utilisé sur la
tâche du premier opérateur. Si ce premier opérateur était occupé à traiter une
requête à ce moment-là, le second opérateur devrait alors récupérer le message
d'extinction dans le canal et s'arrêter. Nous resterons alors à attendre que le
premier opérateur s'arrête, mais cela ne sera jamais le cas car c'est la
seconde tâche qui aura obtenu le message d'extinction. Nous serions alors dans
une situation d'interblocage !

<!--
To prevent this scenario, we first put all of our `Terminate` messages on the
channel in one loop; then we join on all the threads in another loop. Each
worker will stop receiving requests on the channel once it gets a terminate
message. So, we can be sure that if we send the same number of terminate
messages as there are workers, each worker will receive a terminate message
before `join` is called on its thread.
-->

Pour éviter ce scénario, nous allons commencer par insérer tous nos messages
`Extinction` dans le canal dans une boucle ; et ensuite nous utiliserons `join`
sur toutes les tâches dans une autre boucle. Chaque opérateur va arrêter de
recevoir des nouvelles requêtes du canal dès qu'ils recevront le message
d'extinction. Donc, nous pouvons nous assurer que si nous envoyons la même
quantité de messages d'extinction qu'il y a d'opérateurs, chaque opérateur
devrait recevoir un message d'extinction avant que `join` soit utilisé sur leur
tâche.

<!--
To see this code in action, let’s modify `main` to accept only two requests
before gracefully shutting down the server, as shown in Listing 20-25.
-->

Pour observer ce code en action, modifions notre `main` pour accepter
uniquement deux requêtes avant d'arrêter proprement le serveur, comme dans
l'encart 20-25.

<!--
<span class="filename">Filename: src/bin/main.rs</span>
-->

<span class="filename">Fichier : src/bin/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch20-web-server/listing-20-25/src/bin/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/listing-20-25/src/bin/main.rs:here}}
```

<!--
<span class="caption">Listing 20-25: Shut down the server after serving two
requests by exiting the loop</span>
-->

<span class="caption">Encart 20-25 : arrêt du serveur après avoir servi deux
requêtes en sortant de la boucle</span>

<!--
You wouldn’t want a real-world web server to shut down after serving only two
requests. This code just demonstrates that the graceful shutdown and cleanup is
in working order.
-->

Dans la réalité on ne voudrait pas qu'un serveur web s'arrête après avoir servi
seulement deux requêtes. Ce code sert uniquement à montrer que l'arrêt et le
nettoyage s'effectuent bien proprement.

<!--
The `take` method is defined in the `Iterator` trait and limits the iteration
to the first two items at most. The `ThreadPool` will go out of scope at the
end of `main`, and the `drop` implementation will run.
-->

La méthode `take` est définie dans le trait `Iterator` et limite l'itération
aux deux premiers éléments au maximum. Le `GroupeTaches` va sortir de la portée
à la fin du `main`, et l'implémentation de `drop` va s'exécuter.

<!--
Start the server with `cargo run`, and make three requests. The third request
should error, and in your terminal you should see output similar to this:
-->

Démarrez le serveur avec `cargo run`, et faites trois requêtes. La troisième
requête devrait faire une erreur, et dans votre terminal vous devriez avoir une
sortie similaire à ceci :

<!--
<!-- manual-regeneration
cd listings/ch20-web-server/listing-20-25
cargo run
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
curl http://127.0.0.1:7878
third request will error because server will have shut down
copy output below
Can't automate because the output depends on making requests
-- >
-->

<!--
```console
$ cargo run
   Compiling hello v0.1.0 (file:///projects/hello)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/main`
Worker 0 got a job; executing.
Worker 3 got a job; executing.
Shutting down.
Sending terminate message to all workers.
Shutting down all workers.
Shutting down worker 0
Worker 1 was told to terminate.
Worker 2 was told to terminate.
Worker 0 was told to terminate.
Worker 3 was told to terminate.
Shutting down worker 1
Shutting down worker 2
Shutting down worker 3
```
-->

```console
$ cargo run
   Compiling salutations v0.1.0 (file:///projects/salutations)
    Finished dev [unoptimized + debuginfo] target(s) in 1.0s
     Running `target/debug/main`
L'opérateur 0 a reçu une mission ; il l'exécute.
L'opérateur 3 a reçu une mission ; il l'exécute.
Arrêt.
Envoi du message d'extinction à tous les opérateurs.
Arrêt de tous les opérateurs.
Arrêt de l'opérateur 0
L'opérateur 1 a reçu l'instruction d'arrêt.
L'opérateur 2 a reçu l'instruction d'arrêt.
L'opérateur 0 a reçu l'instruction d'arrêt.
L'opérateur 3 a reçu l'instruction d'arrêt.
Arrêt de l'opérateur 1
Arrêt de l'opérateur 2
Arrêt de l'opérateur 3
```

<!--
You might see a different ordering of workers and messages printed. We can see
how this code works from the messages: workers 0 and 3 got the first two
requests, and then on the third request, the server stopped accepting
connections. When the `ThreadPool` goes out of scope at the end of `main`, its
`Drop` implementation kicks in, and the pool tells all workers to terminate.
The workers each print a message when they see the terminate message, and then
the thread pool calls `join` to shut down each worker thread.
-->

Vous devriez voir un ordre différent entre les opérateurs et les messages
affichés. Nous pouvons constater comment ce code fonction avec les messages :
les opérateurs 0 et 3 obtiennent les deux premières requêtes, et ensuite lors
de la troisième requête, le serveur arrête d'accepter des connexions. Lorsque
le `GroupeTaches` sort de la portée à la fin du `main`, son implémentation de
`Drop` rentre en action, et le groupe demande à tous les opérateurs de
s'arrêter. Chaque opérateur va afficher un message lorsqu'il recevra le message
d'extinction, et ensuite le groupe de tâche utilisera `join` pour arrêter
chaque tâche de chaque opérateur.

<!--
Notice one interesting aspect of this particular execution: the `ThreadPool`
sent the terminate messages down the channel, and before any worker received
the messages, we tried to join worker 0. Worker 0 had not yet received the
terminate message, so the main thread blocked waiting for worker 0 to finish.
In the meantime, each of the workers received the termination messages. When
worker 0 finished, the main thread waited for the rest of the workers to
finish. At that point, they had all received the termination message and were
able to shut down.
-->

Remarquez un aspect intéressant spécifique à cette exécution : le
`GroupeTaches` a envoyé les messages d'extinction dans le canal, et avant que
tous les opérateurs aient reçu les messages, nous avons essayé d'utiliser
`join` sur l'opérateur 0. L'opérateur 0 n'avait pas encore reçu le message
d'extinction, donc la tâche principale attendait que l'opérateur 0 finisse.
Pendant ce temps, tous les autres opérateurs ont reçu les messages
d'extinction. Lorsque l'opérateur 0 a fini, la tâche principale a attendu que
les opérateurs restant se terminent. A partir de là, ils ont tous reçu le
message d'extinction et sont disposés à s'arrêter.

<!--
Congrats! We’ve now completed our project; we have a basic web server that uses
a thread pool to respond asynchronously. We’re able to perform a graceful
shutdown of the server, which cleans up all the threads in the pool.
-->

Félicitations ! Nous avons maintenant terminé notre projet ; nous avons un
serveur web basique qui utilise un groupe de tâches pour répondre de manière
asynchrone. Nous pouvons demander un arrêt propre du serveur, qui va alors
nettoyer les tâches dans le groupe.

<!--
Here’s the full code for reference:
-->

Voici le code complet pour pouvoir vous y référer :

<!--
<span class="filename">Filename: src/bin/main.rs</span>
-->

<span class="filename">Fichier : src/bin/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-08-final-code/src/bin/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch20-web-server/no-listing-08-final-code/src/bin/main.rs}}
```

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch20-web-server/no-listing-08-final-code/src/lib.rs}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch20-web-server/no-listing-08-final-code/src/lib.rs}}
```

<!--
We could do more here! If you want to continue enhancing this project, here are
some ideas:
-->

Nous aurions pu faire bien plus là-dedans ! Si vous souhaitez continuer à
améliorer ce projet, voici quelques idées :

<!--
* Add more documentation to `ThreadPool` and its public methods.
* Add tests of the library’s functionality.
* Change calls to `unwrap` to more robust error handling.
* Use `ThreadPool` to perform some task other than serving web requests.
* Find a thread pool crate on [crates.io](https://crates.io/) and implement a
  similar web server using the crate instead. Then compare its API and
  robustness to the thread pool we implemented.
-->

* Ajouter de la documentation sur `GroupeTaches` et les méthodes publiques.
* Ajouter des tests sur les fonctionnalités de la bibliothèque.
* Corriger les appels à `unwrap` pour une meilleure gestion des erreurs.
* Utiliser `GroupeTaches` pour faire d'autres tâches que de répondre à des
  requêtes web.
* Trouver une crate de groupe tâches *(NdT : thread pool)* sur
  [crates.io](https://crates.io/) et implémenter un serveur web similaire en
  utilisant plutôt cette crate. Comparer ensuite son API et sa robustesse au
  groupe de tâches que nous avons implémenté.

<!--
## Summary
-->

## Résumé

<!--
Well done! You’ve made it to the end of the book! We want to thank you for
joining us on this tour of Rust. You’re now ready to implement your own Rust
projects and help with other peoples’ projects. Keep in mind that there is a
welcoming community of other Rustaceans who would love to help you with any
challenges you encounter on your Rust journey.
-->

Bravo ! Vous êtes arrivé à la fin du livre ! Nous tenons à vous remercier
chaleureusement pour nous avoir accompagné pendant cette présentation de Rust.
Vous êtes maintenant fin prêt(e) à créer vos propres projets Rust et aider les
projets des autres développeurs. Rappelez-vous qu'il existe une communauté
chaleureuse de Rustacés qui est prête à vous aider dans tous les défis que vous
rencontrerez dans votre aventure avec Rust.
