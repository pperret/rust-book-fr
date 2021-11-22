> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/202).

<!--
## Extensible Concurrency with the `Sync` and `Send` Traits
-->

## Etendre la concurrence avec les traits `Sync` et `Send`

<!--
Interestingly, the Rust language has *very* few concurrency features. Almost
every concurrency feature we’ve talked about so far in this chapter has been
part of the standard library, not the language. Your options for handling
concurrency are not limited to the language or the standard library; you can
write your own concurrency features or use those written by others.
-->

Curieusement, le langage Rust a *très* peu de fonctionnalités de concurrence.
La plupart des fonctionnalités de concurrence que nous avons vu précédemment
dans ce chapitre font partie de la bibliothèque standard, pas du langage. Vos
options pour gérer la concurrence ne sont pas limitées à celles du langage ou
de la bibliothèque standard ; vous pouvez aussi écrire vos propres
fonctionnalités de concurrence ou utiliser celles qui ont été écrites par
d'autres.

<!--
However, two concurrency concepts are embedded in the language: the
`std::marker` traits `Sync` and `Send`.
-->

Cependant, deux concepts de concurrence sont intégrés dans le langage : les
traits `Sync` et `Send` de `std::marker`.

<!--
### Allowing Transference of Ownership Between Threads with `Send`
-->

### Permettre le transfert de possession entre les tâches avec `Send`

<!--
The `Send` marker trait indicates that ownership of values of the type implementing
`Send` can be transferred between threads. Almost every Rust type is `Send`,
but there are some exceptions, including `Rc<T>`: this cannot be `Send` because
if you cloned an `Rc<T>` value and tried to transfer ownership of the clone to
another thread, both threads might update the reference count at the same time.
For this reason, `Rc<T>` is implemented for use in single-threaded situations
where you don’t want to pay the thread-safe performance penalty.
-->

Le trait `Send` indique que la possession des valeurs du type qui implémente
`Send` peut être transféré entre plusieurs tâches. Presque tous les types de
Rust implémentent `Send`, mais il subsiste quelques exceptions, comme `Rc<T>` :
il ne peut pas implémenter `Send` car si vous clonez une valeur `Rc<T>` et que
vous essayez de transférer la possession de ce clone à une autre tâche, les
deux tâches peuvent modifier le compteur de référence en même temps. Pour cette
raison, `Rc<T>` est prévu pour une utilisation dans des situations qui
nécessitent qu'une seule tâche dans lesquelles vous n'avez pas besoin de subir
le coût sur la performance imposé par la sécurité entre les tâches.

<!--
Therefore, Rust’s type system and trait bounds ensure that you can never
accidentally send an `Rc<T>` value across threads unsafely. When we tried to do
this in Listing 16-14, we got the error `the trait Send is not implemented for
Rc<Mutex<i32>>`. When we switched to `Arc<T>`, which is `Send`, the code
compiled.
-->

Toutefois, le système de type et de traits liés de Rust garantit que vous ne
pourrez jamais envoyer accidentellement en toute insécurité une valeur `Rc<T>`
entre des tâches. Lorsque nous avons essayé de faire cela dans l'encart 16-14,
nous avons obtenu l'erreur
`the trait Send is not implemented for Rc<Mutex<i32>>`. Lorsque nous l'avons
changé pour un `Arc<T>`, qui implémente `Send`, le code s'est compilé.

<!--
Any type composed entirely of `Send` types is automatically marked as `Send` as
well. Almost all primitive types are `Send`, aside from raw pointers, which
we’ll discuss in Chapter 19.
-->

Tous les types composés entièrement d'autres types qui implémentent `Send` sont
automatiquement marqués comme `Send` eux-aussi. Presque la plupart des types
primitifs sont `Send`, à part les pointeurs bruts, ce que nous verrons au
chapitre 19.

<!--
### Allowing Access from Multiple Threads with `Sync`
-->

### Permettre l'accès à plusieurs tâches avec `Sync`

<!--
The `Sync` marker trait indicates that it is safe for the type implementing
`Sync` to be referenced from multiple threads. In other words, any type `T` is
`Sync` if `&T` (an immutable reference to `T`) is `Send`, meaning the reference
can be sent safely to another thread. Similar to `Send`, primitive types are
`Sync`, and types composed entirely of types that are `Sync` are also `Sync`.
-->

Le trait `Sync` indique qu'il est sûr d'avoir une référence dans plusieurs
tâches vers le type qui implémente `Sync`. Autrement dit, n'importe quel type
`T` implémente `Sync` si `&T` (une référence immuable vers `T`) implémente
`Send`, ce qui signifie que la référence peut être envoyée en toute sécurité à
une autre tâche. De la même manière que `Send`, les types primitifs
implémentent `Sync`, et les types composés entièrement d'autres types qui
implémentent `Sync` sont eux-mêmes `Sync`.

<!--
The smart pointer `Rc<T>` is also not `Sync` for the same reasons that it’s not
`Send`. The `RefCell<T>` type (which we talked about in Chapter 15) and the
family of related `Cell<T>` types are not `Sync`. The implementation of borrow
checking that `RefCell<T>` does at runtime is not thread-safe. The smart
pointer `Mutex<T>` is `Sync` and can be used to share access with multiple
threads as you saw in the [“Sharing a `Mutex<T>` Between Multiple
Threads”][sharing-a-mutext-between-multiple-threads]<!-- ignore -- > section.
-->

Le pointeur intelligent `Rc<T>` n'implémente pas non plus `Sync` pour les mêmes
raisons qu'il n'implémente pas `Send`. Le type `RefCell<T>` (que nous avons vu
au chapitre 15) et la famille liée aux types `Cell<T>` n'implémentent pas `Sync`.
L'implémentation du vérificateur d'emprunt que fait `RefCell<T>` à l'exécution
n'est pas sûre entre plusieurs tâches. Le pointeur intelligent `Mutex<T>`
implémente `Sync` et peut être utilisé pour partager l'accès entre plusieurs
tâches, comme vous l'avez vu dans la section précédente.

<!--
### Implementing `Send` and `Sync` Manually Is Unsafe
-->

### Implémenter manuellement `Send` et `Sync` n'est pas sûr

<!--
Because types that are made up of `Send` and `Sync` traits are automatically
also `Send` and `Sync`, we don’t have to implement those traits manually. As
marker traits, they don’t even have any methods to implement. They’re just
useful for enforcing invariants related to concurrency.
-->

Comme les types qui sont constitués de types implémentant les traits `Send` et
`Sync` sont automatiquement des `Send` et `Sync`, nous n'avons pas à
implémenter manuellement ces traits. Comme ce sont des traits de marquage, ils
n'ont même pas de méthodes à implémenter. Ils sont uniquement utiles pour
appliquer les règles de concurrence.

<!--
Manually implementing these traits involves implementing unsafe Rust code.
We’ll talk about using unsafe Rust code in Chapter 19; for now, the important
information is that building new concurrent types not made up of `Send` and
`Sync` parts requires careful thought to uphold the safety guarantees. [“The
Rustonomicon”][nomicon] has more information about these guarantees and how to
uphold them.
-->

L'implémentation manuelle de ces traits implique de faire du code Rust non
sécurisé. Nous allons voir le code Rust non sécurisé dans le chapitre 19 ; pour
l'instant l'information à retenir est que construire de nouveaux types
pour la concurrence constitués d'éléments qui n'implémentent pas `Send` et
`Sync` nécessite une réflexion approfondie pour respecter les garanties de
sécurité. [“The Rustonomicon”][nomicon] contient plus d'informations à propos de
ces garanties et comment les faire appliquer.

[The Rustonomicon]: https://doc.rust-lang.org/stable/nomicon/

<!--
## Summary
-->

## Résumé

<!--
This isn’t the last you’ll see of concurrency in this book: the project in
Chapter 20 will use the concepts in this chapter in a more realistic situation
than the smaller examples discussed here.
-->

Ce n'est pas la dernière fois que vous allez rencontrer de la concurrence dans
ce livre : le projet du chapitre 20 va utiliser les concepts de ce chapitre dans
une situation plus réaliste que les petits exemples que nous avons utilisés ici.

<!--
As mentioned earlier, because very little of how Rust handles concurrency is
part of the language, many concurrency solutions are implemented as crates.
These evolve more quickly than the standard library, so be sure to search
online for the current, state-of-the-art crates to use in multithreaded
situations.
-->

Nous l'avons dit précédemment, comme les outils pour gérer la concurrence de
Rust ne sont pas directement intégrés dans le langage, de nombreuses solutions
pour de la concurrence sont implémentées dans des crates. Elles évoluent plus
rapidement que la bibliothèque standard, donc assurez-vous de rechercher en
ligne des crates modernes et à la pointe de la technologie à utiliser dans des
situations multitâches.

<!--
The Rust standard library provides channels for message passing and smart
pointer types, such as `Mutex<T>` and `Arc<T>`, that are safe to use in
concurrent contexts. The type system and the borrow checker ensure that the
code using these solutions won’t end up with data races or invalid references.
Once you get your code to compile, you can rest assured that it will happily
run on multiple threads without the kinds of hard-to-track-down bugs common in
other languages. Concurrent programming is no longer a concept to be afraid of:
go forth and make your programs concurrent, fearlessly!
-->

La bibliothèque standard de Rust fournit les canaux pour l'envoi de messages et
les types de pointeurs intelligents, comme `Mutex<T>` et `Arc<T>`, qui sont sûr
à utiliser en concurrence. Le système de type et le vérificateur d'emprunt sont
là pour s'assurer que le code utilisé dans ces solutions ne vont pas conduire à
des situations de concurrence ou utiliser des références qui ne sont plus en
vigueur. Une fois que votre code se compile, vous pouvez être assuré qu'il
fonctionnera bien sur plusieurs tâches sans avoir les genres de bogues
*difficiles à traquer* qui sont monnaie courante dans les autres langages. Le
développement en concurrence n'est un domaine qui ne devrait plus faire peur :
lancez-vous et appliquez la concurrence à vos programmes sans en avoir crainte !

<!--
Next, we’ll talk about idiomatic ways to model problems and structure solutions
as your Rust programs get bigger. In addition, we’ll discuss how Rust’s idioms
relate to those you might be familiar with from object-oriented programming.
-->

Au chapitre suivant, nous allons voir des techniques adaptées pour modéliser des
problèmes et structurer votre solution au fur et à mesure que vos programmes en
Rust grandissent. De plus, nous analyserons les liens qui peuvent exister entre
les idées de Rust et celles avec lesquelles vous êtes peut-être familier en
programmation orientée objet.

<!--
[sharing-a-mutext-between-multiple-threads]:
ch16-03-shared-state.html#sharing-a-mutext-between-multiple-threads
[nomicon]: ../nomicon/index.html
-->

[sharing-a-mutext-between-multiple-threads]: ch16-03-shared-state.html
[nomicon]: https://doc.rust-lang.org/nomicon/index.html
