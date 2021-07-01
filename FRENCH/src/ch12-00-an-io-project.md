> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/130).

<!--
# An I/O Project: Building a Command Line Program
-->

# Un projet d'entrée/sortie : construire un programme en ligne de commande

<!--
This chapter is a recap of the many skills you’ve learned so far and an
exploration of a few more standard library features. We’ll build a command line
tool that interacts with file and command line input/output to practice some of
the Rust concepts you now have under your belt.
-->

Ce chapitre est un résumé de toutes les nombreuses compétences que vous avez
apprises précédemment et une découverte de quelques fonctionnalités
supplémentaires de la bibliothèque standard. Nous allons construire un outil en
ligne de commande qui interagit avec des fichiers et les entrées/sorties de la
ligne de commande pour mettre en pratique certains concepts Rust que vous avez
maintenant en votre connaissance.

<!--
Rust’s speed, safety, single binary output, and cross-platform support make it
an ideal language for creating command line tools, so for our project, we’ll
make our own version of the classic command line tool `grep` (**g**lobally
search a **r**egular **e**xpression and **p**rint). In the simplest use case,
`grep` searches a specified file for a specified string. To do so, `grep` takes
as its arguments a filename and a string. Then it reads the file, finds lines
in that file that contain the string argument, and prints those lines.
-->

Les fonctionnalités de rapidité, sécurité, de sortie binaire unique, et du
support multi-plateforme de Rust en font le langage idéal pour créer des outils
en ligne de commande, donc pour notre projet, nous allons construire notre
version de l'outil en ligne de commande `grep` (qui signifie **g**lobally
search a **r**egular **e**xpression and **p**rint, soit *recherche globale et
affichage d'une expression régulière*). Dans des cas d'usage très simple,
`grep` recherche une chaîne de caractère précise dans un fichier précis. Pour
faire ainsi, `grep` prend en argument un nom de fichier et une chaîne de
caractères. Ensuite, il lit le fichier, trouve les lignes de ce fichier qui
contient la chaîne de caractères envoyé en argument, et affiche ces lignes.

<!--
Along the way, we’ll show how to make our command line tool use features of the
terminal that many command line tools use. We’ll read the value of an
environment variable to allow the user to configure the behavior of our tool.
We’ll also print error messages to the standard error console stream (`stderr`)
instead of standard output (`stdout`), so, for example, the user can redirect
successful output to a file while still seeing error messages onscreen.
-->

Sur la route, nous allons vous montrer comment utiliser dans votre outil en
ligne de commande les fonctionnalités des terminaux que de nombreux outils en
ligne de commande utilisent. Nous allons lire la valeur d'une variable
d'environnement pour permettre à l'utilisateur de configurer le comportement de
notre outil. Nous allons aussi afficher des messages d'erreur vers le flux
d'erreur standard de la console (`stderr`) plutôt que la sortie standard
(`stdout`), pour, par exemple, que l'utilisateur puisse rediriger la sortie
fructueuse vers un fichier, tout en affichant les messages d'erreur à l'écran.

<!--
One Rust community member, Andrew Gallant, has already created a fully
featured, very fast version of `grep`, called `ripgrep`. By comparison, our
version of `grep` will be fairly simple, but this chapter will give you some of
the background knowledge you need to understand a real-world project such as
`ripgrep`.
-->

Un membre de la communauté Rust, Andrew Gallant, a déjà créé une version
complète et très performante de `grep`, qu'il a appelé `ripgrep`. En
comparaison, notre version de `grep` sera plutôt simple, mais ce chapitre va
vous donner les connaissances de base dont vous avez besoin pour appréhender
un projet réel comme `ripgrep`.

<!--
Our `grep` project will combine a number of concepts you’ve learned so far:
-->

Notre projet `grep` va combiner un certain nombre de concepts que vous avez
acquis jusqu'à présent :

<!--
* Organizing code (using what you learned about modules in [Chapter 7][ch7]<!--
  ignore -- >)
* Using vectors and strings (collections, [Chapter 8][ch8]<!-- ignore -- >)
* Handling errors ([Chapter 9][ch9]<!-- ignore -- >)
* Using traits and lifetimes where appropriate ([Chapter 10][ch10]<!-- ignore
  -- >)
* Writing tests ([Chapter 11][ch11]<!-- ignore -- >)
-->

* Organiser le code (en utilisant ce que vous avez appris sur les modules au
[chapitre 7][ch7]<!-- ignore -->)
* Utiliser les vecteurs et les chaînes de caractères (les collections du
[chapitre 8][ch8]<!-- ignore -->)
* Gérer les erreurs ([chapitre 9][ch9]<!-- ignore -->)
* Utiliser les traits et les durées de vie lorsque c'est approprié
([chapitre 10][ch10]<!-- ignore -->)
* Ecrire les tests ([chapitre 11][ch11]<!-- ignore -->)

<!--
We’ll also briefly introduce closures, iterators, and trait objects, which
Chapters [13][ch13]<!-- ignore -- > and [17][ch17]<!-- ignore -- > will cover in
detail.
-->

Nous vous présenterons aussi brièvement les fermetures, les itérateurs, et les
objets de trait, que les chapitres [13][ch13]<!-- ignore --> et
[17][ch17]<!-- ignore --> vont traiter en détails.

<!--
[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch17]: ch17-00-oop.html
-->

[ch7]: ch07-00-managing-growing-projects-with-packages-crates-and-modules.html
[ch8]: ch08-00-common-collections.html
[ch9]: ch09-00-error-handling.html
[ch10]: ch10-00-generics.html
[ch11]: ch11-00-testing.html
[ch13]: ch13-00-functional-features.html
[ch17]: ch17-00-oop.html
