> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/143).

<!--
## Writing Error Messages to Standard Error Instead of Standard Output
-->

## Ecrire les messages d'erreur sur la sortie d'erreurs standard au lieu de la sortie normale

<!--
At the moment, we’re writing all of our output to the terminal using the
`println!` macro. Most terminals provide two kinds of output: *standard
output* (`stdout`) for general information and *standard error* (`stderr`)
for error messages. This distinction enables users to choose to direct the
successful output of a program to a file but still print error messages to the
screen.
-->

Pour l'instant, nous avons écrit toutes nos sorties du terminal en utilisant
la macro `println!`. La plupart des terminaux fournissent deux genres de
sorties : la *sortie standard* (`stdout`) pour les informations générales
et *la sortie d'erreur standard* (`stderr`) pour les messages d'erreur. Cette
distinction permet à l'utilisateur de choisir de rediriger la sortie des
réussites d'un programme vers un fichier mais continuer à afficher les messages
d'erreur à l'écran.

<!--
The `println!` macro is only capable of printing to standard output, so we
have to use something else to print to standard error.
-->

La macro `println!` ne peut écrire que sur la sortie standard, donc nous
devons utiliser autre chose pour écrire sur la sortie d'erreur standard.

<!--
### Checking Where Errors Are Written
-->

### Vérifier où sont écris les erreurs

<!--
First, let’s observe how the content printed by `minigrep` is currently being
written to standard output, including any error messages we want to write to
standard error instead. We’ll do that by redirecting the standard output stream
to a file while also intentionally causing an error. We won’t redirect the
standard error stream, so any content sent to standard error will continue to
display on the screen.
-->

Commençons par observer comment le contenu écris par `minigrep` est actuellement
écris sur la sortie standard, y compris les messages d'erreur que nous
souhaitons plutôt écrire sur la sortie d'erreur standard. Nous allons faire cela
en redirigeant le flux de sortie standard vers un fichier pendant que nous
déclencherons intentionnellement une erreur. Nous ne redirigerons pas le flux
de sortie d'erreur standard, donc n'importe quel contenu envoyé à la sortie
d'erreur standard va continuer à s'afficher à l'écran.

<!--
Command line programs are expected to send error messages to the standard error
stream so we can still see error messages on the screen even if we redirect the
standard output stream to a file. Our program is not currently well-behaved:
we’re about to see that it saves the error message output to a file instead!
-->

Les programmes en ligne de commande sont censés envoyer leurs messages d'erreur
dans le flux d'erreurs standard afin que nous puissions continuer à voir les
messages d'erreurs à l'écran même si nous redirigeons le flux de la sortie
standard dans un fichier. Notre programme ne se comporte pas comme il le
devrait : nous allons voir qu'à la place, il envoie les messages d'erreur
dans le fichier !

<!--
The way to demonstrate this behavior is by running the program with `>` and the
filename, *output.txt*, that we want to redirect the standard output stream to.
We won’t pass any arguments, which should cause an error:
-->

Pour démontrer ce comportement, il faut exécuter le programme avec `>` suivi du
nom du fichier, *sortie.txt*, dans lequel nous souhaitons rediriger le flux de
sortie standard. Nous ne fournissons aucun argument, ce qui va causer une
erreur :

<!--
```console
$ cargo run > output.txt
```
-->

```console
$ cargo run > sortie.txt
```

<!--
The `>` syntax tells the shell to write the contents of standard output to
*output.txt* instead of the screen. We didn’t see the error message we were
expecting printed to the screen, so that means it must have ended up in the
file. This is what *output.txt* contains:
-->

La syntaxe indique à l'invite de commande d'écrire le contenu de la sortie
standard dans *sortie.txt* plutôt qu'à l'écran. Nous n'avons pas vu le
message d'erreur que nous nous attendions de voir à l'écran, ce qui veut
dire qu'il a dû finir dans le fichier. Voici ce que *sortie.txt* contient :

<!--
```text
Problem parsing arguments: not enough arguments
```
-->

```text
Problème rencontré lors de l'interprétation des arguments : il n'y a pas assez d'arguments
```

<!--
Yup, our error message is being printed to standard output. It’s much more
useful for error messages like this to be printed to standard error so only
data from a successful run ends up in the file. We’ll change that.
-->

Effectivement, notre message d'erreur est écris sur la sortie standard. Il
est bien plus utile que les messages d'erreur comme celui-ci soient écris
dans la sortie d'erreur standard afin qu'uniquement les données d'une
exécution fructueuse finissent dans le fichier. Nous allons corriger cela.

<!--
### Printing Errors to Standard Error
-->

### Ecrire les erreurs sur la sortie d'erreur standard

<!--
We’ll use the code in Listing 12-24 to change how error messages are printed.
Because of the refactoring we did earlier in this chapter, all the code that
prints error messages is in one function, `main`. The standard library provides
the `eprintln!` macro that prints to the standard error stream, so let’s change
the two places we were calling `println!` to print errors to use `eprintln!`
instead.
-->

Nous allons utiliser le code de l'encart 12-24 pour changer la manière dont les
messages d'erreur sont écris. Grâce au remaniement que nous avons fait plus tôt
dans ce chapitre, tout le code qui écris les messages d'erreurs se trouve dans
une seule fonction, `main`. La bibliothèque standard fournit la macro
`eprintln!` qui écris dans le flux d'erreur standard, donc changeons les deux
endroits où nous appelons `println!` afin d'utiliser `eprintln!` à la place.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-24/src/main.rs:here}}
```

<!--
<span class="caption">Listing 12-24: Writing error messages to standard error
instead of standard output using `eprintln!`</span>
-->

<span class="caption">Encart 12-24 : Ecrire les messages d'erreur sur la sortie
d'erreur standard au lieu de la sortie standard en utilisant `eprintln!`</span>

<!--
After changing `println!` to `eprintln!`, let’s run the program again in the
same way, without any arguments and redirecting standard output with `>`:
-->

Après avoir changé `println!` en `eprintln!`, exécutons le programme à nouveau
de la même manière, sans aucun argument et en redirigeant la sortie standard
avec `>` :

<!--
```console
$ cargo run > output.txt
Problem parsing arguments: not enough arguments
```
-->

```console
$ cargo run > sortie.txt
Problème rencontré lors de l'interprétation des arguments : il n'y a pas assez d'arguments
```

<!--
Now we see the error onscreen and *output.txt* contains nothing, which is the
behavior we expect of command line programs.
-->

Désormais nous pouvons voir l'erreur à l'écran et *sortie.txt* ne contient rien,
ce qui est le comportement de que nous attendons d'un programme en ligne de
commande.

<!--
Let’s run the program again with arguments that don’t cause an error but still
redirect standard output to a file, like so:
-->

Exécutons le programme à nouveau avec des arguments qui ne causent pas d'erreurs
mais nous continuons à rediriger la sortie standard, comme ceci :

<!--
```console
$ cargo run to poem.txt > output.txt
```
-->

```console
$ cargo run to poem.txt > sortie.txt
```

<!--
We won’t see any output to the terminal, and *output.txt* will contain our
results:
-->

Nous ne voyons rien sur la sortie du terminal, et *sortie.txt* devrait contenir
notre résultat :

<!--
<span class="filename">Filename: output.txt</span>
-->

<span class="filename">Fichier : sortie.txt</span>

<!--
```text
Are you nobody, too?
How dreary to be somebody!
```
-->

```text
Are you nobody, too?
How dreary to be somebody!
```

<!--
This demonstrates that we’re now using standard output for successful output
and standard error for error output as appropriate.
-->

Ceci prouve que nous utilisons maintenant la sortie standard pour la sortie
réussie et l'erreur standard pour la sortie d'erreur, en fonction des
circonstances.

<!--
## Summary
-->

## Résumé

<!--
This chapter recapped some of the major concepts you’ve learned so far and
covered how to perform common I/O operations in Rust. By using command line
arguments, files, environment variables, and the `eprintln!` macro for printing
errors, you’re now prepared to write command line applications. By using the
concepts in previous chapters, your code will be well organized, store data
effectively in the appropriate data structures, handle errors nicely, and be
well tested.
-->

Ce chapitre a résumé certains des concepts majeurs que vous avez appris
précédemment et expliqué comment procéder à des opérations courantes sur les
entrées/sorties en Rust. En utilisant les arguments en ligne de commande, les
fichiers, les variables d'environnement, et la macro `eprintln!` pour écrire les
erreurs, vous pouvez désormais écrire des applications en ligne de commande. En
suivant les concepts vus dans les chapitres précédents, votre code restera bien
organisé, stockera les données dans les bonnes structures de données, gérera
correctement les erreurs, et sera correctement testé.

<!--
Next, we’ll explore some Rust features that were influenced by functional
languages: closures and iterators.
-->

Maintenant, nous allons découvrir quelques fonctionnalités de Rust qui ont été
influencées par les langages fonctionnels : les closures et les itérateurs.
