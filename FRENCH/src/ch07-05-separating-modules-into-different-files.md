> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/79).

<!--
## Separating Modules into Different Files
-->

## Séparer les modules dans différents fichiers

<!--
So far, all the examples in this chapter defined multiple modules in one file.
When modules get large, you might want to move their definitions to a separate
file to make the code easier to navigate.
-->

Jusqu'à présent, tous les exemples de ce chapitre ont défini plusieurs modules
dans un seul fichier. Quand les modules vont grossir, il est probable que vous
vouliez déplacer leurs définitions dans un fichier séparé pour faciliter le
parcours de votre code.

<!--
For example, let’s start from the code in Listing 7-17 and move the
`front_of_house` module to its own file *src/front_of_house.rs* by changing the
crate root file so it contains the code shown in Listing 7-21. In this case,
the crate root file is *src/lib.rs*, but this procedure also works with binary
crates whose crate root file is *src/main.rs*.
-->

Par exemple, basons-nous sur le code de l'encart 7-17 et déplaçons le module
`salle_a_manger` dans son propre fichier *src/front_of_house.rs* en changeant le
fichier de la crate racine afin qu'il corresponde au code de l'encart 7-21. Dans
notre cas, le fichier de la crate racine est *src/lib.rs*, mais cette procédure
fonctionne aussi avec les crates binaires dans lesquels le fichier de la crate
racine est *src/main.rs*.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
mod front_of_house;

pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
    hosting::add_to_waitlist();
}
```
-->

```rust,ignore
mod salle_a_manger;

pub use crate::salle_a_manger::accueil;

pub fn manger_au_restaurant() {
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
    accueil::ajouter_a_la_liste_attente();
}
```

<!--
<span class="caption">Listing 7-21: Declaring the `front_of_house` module whose
body will be in *src/front_of_house.rs*</span>
-->

<span class="caption">Encart 7-21 : Déclaration du module `salle_a_manger` dont
le corps sera dans *src/salle_a_manger.rs*</span>

<!--
And *src/front_of_house.rs* gets the definitions from the body of the
`front_of_house` module, as shown in Listing 7-22.
-->

Et *src/salle_a_manger.rs* contiendra la définition du corps du module
`salle_a_manger`, comme dans l'encart 7-22.

<!--
<span class="filename">Filename: src/front_of_house.rs</span>
-->

<span class="filename">Fichier : src/salle_a_manger.rs</span>

<!--
```rust
pub mod hosting {
    pub fn add_to_waitlist() {}
}
```
-->

```rust
pub mod accueil {
    pub fn ajouter_a_la_liste_attente() {}
}
```

<!--
<span class="caption">Listing 7-22: Definitions inside the `front_of_house`
module in *src/front_of_house.rs*</span>
-->

<span class="caption">Encart 7-22 : Les définitions à l'intérieur du module
`salle_a_manger` dans *src/salle_a_manger.rs*</span>

<!--
Using a semicolon after `mod front_of_house` rather than using a block tells
Rust to load the contents of the module from another file with the same name as
the module. To continue with our example and extract the `hosting` module to
its own file as well, we change *src/front_of_house.rs* to contain only the
declaration of the `hosting` module:
-->

Utiliser un point-virgule après `mod salle_a_manger` plutôt que de créer un bloc
indique à Rust de charger le contenu du module à partir d'un autre fichier avec
le même nom que le module. Pour continuer avec notre exemple et déplacer
également le module `accueil` dans son propre fichier, nous modifions
*src/salle_a_manger.rs* pour avoir uniquement la déclaration du module
`accueil` :

<!--
<span class="filename">Filename: src/front_of_house.rs</span>
-->

<span class="filename">Fichier : src/salle_a_manger.rs</span>

<!--
```
pub mod hosting;
```
-->

```
pub mod accueil;
```

<!--
Then we create a *src/front_of_house* directory and a file
*src/front_of_house/hosting.rs* to contain the definitions made in the
`hosting` module:
-->

Ensuite, nous créons un dossier *src/salle_a_manger* et un fichier
*src/salle_a_manger/accueil.rs* qui contiendra les définitions du module
`accueil` :

<!--
<span class="filename">Filename: src/front_of_house/hosting.rs</span>
-->

<span class="filename">Fichier : src/salle_a_manger/accueil.rs</span>

<!--
```
pub fn add_to_waitlist() {}
```
-->

```
pub fn ajouter_a_la_liste_attente() {}
```

<!--
The module tree remains the same, and the function calls in `eat_at_restaurant`
will work without any modification, even though the definitions live in
different files. This technique lets you move modules to new files as they grow
in size.
-->

L'arborescence des modules reste identique, et les appels aux fonctions de
`manger_au_restaurant` vont continuer à fonctionner sans aucune modification,
même si les définitions se retrouvent dans des fichiers différents. Cette
technique vous permet de déplacer des modules dans de nouveaux fichiers au fur
et à mesure qu'ils s'agrandissent.

<!--
Note that the `pub use crate::front_of_house::hosting` statement in
*src/lib.rs* also hasn’t changed, nor does `use` have any impact on what files
are compiled as part of the crate. The `mod` keyword declares modules, and Rust
looks in a file with the same name as the module for the code that goes into
that module.
-->

Remarquez que l'instruction `pub use crate::salle_a_manger::accueil` dans
*src/lib.rs* n'a pas changée, et que `use` n'a aucun impact sur quels fichiers
sont compilés pour constituer la crate. Le mot-clé `mod` déclare un module, et
Rust recherche un fichier avec le même nom que le module qui contiendrait du
code qui correspondrait à ce module.

<!--
## Summary
-->

## Résumé

<!--
Rust lets you split a package into multiple crates and a crate into modules
so you can refer to items defined in one module from another module. You can do
this by specifying absolute or relative paths. These paths can be brought into
scope with a `use` statement so you can use a shorter path for multiple uses of
the item in that scope. Module code is private by default, but you can make
definitions public by adding the `pub` keyword.
-->

Rust vous permet de décomposer un paquet dans plusieurs crates et un crate en
modules afin que vous puissiez réutiliser vos éléments d'un module à un autre.
Vous pouvez faire cela en utilisant des chemins absolus ou relatifs. Ces chemins
peuvent être importés dans la portée avec une instruction `use` afin d'avoir un
chemin plus court pour pouvoir l'utiliser plusieurs fois cet élément dans la
portée. Le code du module est privé par défaut, mais vous pouvez rendre public
des définitions en ajoutant le mot-clé `pub`.

<!--
In the next chapter, we’ll look at some collection data structures in the
standard library that you can use in your neatly organized code.
-->

Au prochain chapitre, nous allons nous intéresser à quelques structures de
collections de données de la bibliothèque standard que vous pourrez utiliser
dans votre code soigneusement organisé.
