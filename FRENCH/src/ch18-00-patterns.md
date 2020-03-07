> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Push Request](https://github.com/Jimskapt/rust-book-fr/pull/203).

<!--
# Patterns and Matching
-->

# Les motifs et le filtrage par motif

<!--
Patterns are a special syntax in Rust for matching against the structure of
types, both complex and simple. Using patterns in conjunction with `match`
expressions and other constructs gives you more control over a program’s
control flow. A pattern consists of some combination of the following:
-->

Les motifs ont une syntaxe spéciale en Rust pour pouvoir les filtrer sur la
structure des types, qu'elle soit simple ou complexe. L'utilisation de motifs
en collaboration avec l'expression `match` et d'autres constructions vous donne
plus contrôle sur le flux de contrôle de votre programme. Un motif concerne :

<!--
* Literals
* Destructured arrays, enums, structs, or tuples
* Variables
* Wildcards
* Placeholders
-->

* les littéraux
* les tableaux déstructurés, les énumérations, les structures, ou les tuples
* les variables
* les jokers
* les espaces réservés

<!--
These components describe the shape of the data we’re working with, which we
then match against values to determine whether our program has the correct data
to continue running a particular piece of code.
-->

Les composants décrivent la forme d'une donnée avec laquelle nous travaillons,
que nous filtrons avec motifs de valeurs pour savoir si notre programme a la
donnée correcte pour continuer à exécuter un code qui en a besoin.

<!--
To use a pattern, we compare it to some value. If the pattern matches the
value, we use the value parts in our code. Recall the `match` expressions in
Chapter 6 that used patterns, such as the coin-sorting machine example. If the
value fits the shape of the pattern, we can use the named pieces. If it
doesn’t, the code associated with the pattern won’t run.
-->

Pour utiliser un motif, nous le comparons à quelques valeurs. Si le motif
correspond à la valeur, nous utilisons les éléments présents dans la valeur
pour notre code. Rappelez-vous que les expressions `match` du chapitre 6
utilisaient les motifs, comme pour la machine à trier la monnaie par exemple.
Si la valeur correspond à la forme d'un motif, nous pouvions utiliser le nom de
la pièce. Sinon, le code lié au motif n'était pas exécuté.

<!--
This chapter is a reference on all things related to patterns. We’ll cover the
valid places to use patterns, the difference between refutable and irrefutable
patterns, and the different kinds of pattern syntax that you might see. By the
end of the chapter, you’ll know how to use patterns to express many concepts in
a clear way.
-->

Ce chapitre sert de référence sur ce qui concerne les motifs. Nous allons voir
les moments appropriés pour utiliser les motifs, les différences entre les
motifs réfutables et irréfutables, et les différentes syntaxes de motifs que
vous pouvez rencontrer. A la fin de ce chapitre, vous saurez comment utiliser
les motifs pour exprimer clairement de nombreux concepts.
