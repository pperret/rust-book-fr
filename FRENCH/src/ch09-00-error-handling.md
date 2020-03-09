> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/91).

<!--
# Error Handling
-->

# La gestion des erreurs

<!--
Rust’s commitment to reliability extends to error handling. Errors are a fact
of life in software, so Rust has a number of features for handling situations
in which something goes wrong. In many cases, Rust requires you to acknowledge
the possibility of an error and take some action before your code will compile.
This requirement makes your program more robust by ensuring that you’ll
discover errors and handle them appropriately before you’ve deployed your code
to production!
-->

L'engagement de Rust pour la fiabilité concerne aussi la gestion des erreurs.
Les erreurs font partie de la vie des programmes informatiques, c'est pourquoi
Rust a des fonctionnalités pour gérer les situations dans lesquelles quelque
chose dérape. Dans de nombreux cas, Rust exige que vous anticipiez les erreurs
possibles et que vous preniez des dispositions avant de pouvoir compiler votre
code. Cette exigence rend votre programme plus résiliant en s'assurant que vous
détectiez et gérez les erreurs correctement avant même que vous ne déployez
votre code en production !

<!--
Rust groups errors into two major categories: *recoverable* and *unrecoverable*
errors. For a recoverable error, such as a file not found error, it’s
reasonable to report the problem to the user and retry the operation.
Unrecoverable errors are always symptoms of bugs, like trying to access a
location beyond the end of an array.
-->

Rust classe les erreurs dans deux catégories principales : les erreurs
*récupérables* et *irrécupérables*. Les erreurs récupérables, comme lorsque un
fichier n'est pas trouvable, il est préférable de signaler le problème à
l'utilisateur et de relancer l'opération. Les erreurs irrécupérables sont
toujours des symptômes de bogues, comme essayer d'accéder à un élément en dehors
de l'intervalle de données d'un tableau.

<!--
Most languages don’t distinguish between these two kinds of errors and handle
both in the same way, using mechanisms such as exceptions. Rust doesn’t have
exceptions. Instead, it has the type `Result<T, E>` for recoverable errors and
the `panic!` macro that stops execution when the program encounters an
unrecoverable error. This chapter covers calling `panic!` first and then talks
about returning `Result<T, E>` values. Additionally, we’ll explore
considerations when deciding whether to try to recover from an error or to stop
execution.
-->

La plupart des langages de programmation ne font pas de distinction entre ces
deux types d'erreurs et les gèrent de la même manière, en utilisant des
fonctionnalités comme les exceptions. Rust n'a pas d'exception. À la place, il
a les types `Result<T, E>` pour les erreurs récupérables, et la macro `panic!`
qui arrête l'exécution quand le programme se heurte à des erreurs
irrécupérables. Nous allons commencer ce chapitre par expliquer l'utilisation de
`panic!`, puis ensuite nous allons voir les valeurs de retour `Result<T, E>`. De
plus, nous allons voir les éléments à prendre en compte pour décider si nous
devons essayer de rattraper une erreur ou alors arrêter l'exécution.
