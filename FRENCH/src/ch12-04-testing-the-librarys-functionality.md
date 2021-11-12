> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/139).

<!--
## Developing the Library’s Functionality with Test-Driven Development
-->

## Développer les fonctionnalités de la bibliothèque avec le TDD

<!--
Now that we’ve extracted the logic into *src/lib.rs* and left the argument
collecting and error handling in *src/main.rs*, it’s much easier to write tests
for the core functionality of our code. We can call functions directly with
various arguments and check return values without having to call our binary
from the command line. Feel free to write some tests for the functionality in
the `Config::new` and `run` functions on your own.
-->

Maintenant que nous avons extrait la logique dans *src/lib.rs* et que nous
avons laissé la récupération des arguments et la gestion des erreurs dans
*src/main.rs*, il est bien plus facile d'écrire les tests pour les
fonctionnalités de base de notre code. Nous pouvons appeler les fonctions
directement avec différents arguments et vérifier les valeurs de retour sans
avoir à appeler notre binaire dans la ligne de commande. N'hésitez pas à écrire
quelques tests par vous-même pour les fonctions `Config::new` et `run`.

<!--
In this section, we’ll add the searching logic to the `minigrep` program by
using the Test-driven development (TDD) process. This software development
technique follows these steps:
-->

Dans cette section, nous allons ajouter la logique de recherche au programme
`minigrep` en utilisant le processus de développement orienté par les tests
(c'est le TDD : *Test-Driven Development*). Cette technique de développement
de logiciels suit ces trois étapes :

<!--
1. Write a test that fails and run it to make sure it fails for the reason you
   expect.
2. Write or modify just enough code to make the new test pass.
3. Refactor the code you just added or changed and make sure the tests
   continue to pass.
4. Repeat from step 1!
-->

1. Ecrire un test qui échoue et lancez-le pour vous assurer qu'il va échouer
   pour la raison que vous attendiez.
2. Ecrire ou modifier juste assez de code pour faire réussir ce nouveau test.
3. Remanier le code que vous venez d'ajouter ou de changer pour vous assurer
   que les tests continuent à réussir.
4. Recommencer à l'étape 1 !

<!--
This process is just one of many ways to write software, but TDD can help drive
code design as well. Writing the test before you write the code that makes the
test pass helps to maintain high test coverage throughout the process.
-->

Ce processus n'est qu'une des différentes manières d'écrire des programmes,
mais le TDD peut aussi aider à piloter sa conception. Ecrire les tests avant
d'écrire le code qui fait réussir les tests aide à maintenir une haute
couverture de tests tout le long du processus.

<!--
We’ll test drive the implementation of the functionality that will actually do
the searching for the query string in the file contents and produce a list of
lines that match the query. We’ll add this functionality in a function called
`search`.
-->

Nous allons expérimenter cela avec l'implémentation de la fonctionnalité qui va
rechercher la chaîne de caractères demandée dans le contenu du fichier et
générer une liste de lignes qui correspond à cette recherche. Nous ajouterons
cette fonctionnalité dans une fonction `rechercher`.

<!--
### Writing a Failing Test
-->

### Ecrire un test qui échoue

<!--
Because we don’t need them anymore, let’s remove the `println!` statements from
*src/lib.rs* and *src/main.rs* that we used to check the program’s behavior.
Then, in *src/lib.rs*, we’ll add a `tests` module with a test function, as we
did in [Chapter 11][ch11-anatomy]<!-- ignore -- >. The test function specifies
the behavior we want the `search` function to have: it will take a query and
the text to search for the query in, and it will return only the lines from the
text that contain the query. Listing 12-15 shows this test, which won’t compile
yet.
-->

Comme nous n'en avons plus besoin, enlevons les instructions `println!` de
*src/lib.rs* et *src/main.rs* que nous avions utilisé pour vérifier le bon
comportement du programme. Ensuite, dans *src/lib.rs*, nous allons ajouter un
module `tests` avec une fonction de test, comme nous l'avions fait dans le
[chapitre 11][ch11-anatomy]<!-- ignore -->. La fonction de test définit le
comportement que nous voulons qu'ait la fonction `rechercher` : elle va prendre
en arguments une recherche et le texte dans lequel rechercher, et elle va
retourner seulement les lignes du texte qui correspondent à la recherche.
L'encart 12-15 montre ce test, qui ne se compile pas encore.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-15/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-15: Creating a failing test for the `search`
function we wish we had</span>
-->

<span class="caption">Encart 12-15 : Création d'un test qui échoue pour la
fonction `rechercher` que nous souhaitons concevoir</span>

<!--
This test searches for the string `"duct"`. The text we’re searching is three
lines, only one of which contains `"duct"` (Note that the backslash after the
opening double quote tells Rust not to put a newline character at the beginning
of the contents of this string literal). We assert that the value returned from
the `search` function contains only the line we expect.
-->

Ce test recherche la chaîne de caractères `"duct"`. Le texte dans lequel nous
recherchons fait trois lignes, et seulement une d'entre elles contient `"duct"`
(remarquez que l'antislash après la double-guillet ouvrante indique à Rust de
ne pas insérer un caractère de nouvelle ligne au début du contenu de ce litéral
de chaîne de caractère). Nous vérifions que la valeur retournée par la fonction
`rechercher` contient seulement la ligne que nous avions prévu.

<!--
We aren’t able to run this test and watch it fail because the test doesn’t even
compile: the `search` function doesn’t exist yet! So now we’ll add just enough
code to get the test to compile and run by adding a definition of the `search`
function that always returns an empty vector, as shown in Listing 12-16. Then
the test should compile and fail because an empty vector doesn’t match a vector
containing the line `"safe, fast, productive."`
-->

Nous ne pouvons pas encore exécuter ce test et vérifier s'il échoue car même le
test ne peut pas se compiler : la fonction `rechercher` n'existe pas encore !
Donc pour le moment nous allons ajouter juste assez de code pour que le test
puisse compiler et s'exécuter en ajoutant une définition de la fonction
`rechercher` qui retourne un vecteur vide, comme dans l'encart 12-16. Ensuite
le test va compiler et échouer car un vecteur vide ne correspond pas au vecteur
qui contient la ligne `"sécurité, rapidité, productivité."`

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,noplayground
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```
-->

```rust,noplayground
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-16/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-16: Defining just enough of the `search`
function so our test will compile</span>
-->

<span class="caption">Encart 12-16 : Définition du strict minimum de la fonction
`rechercher` pour que notre test puisse compiler</span>

<!--
Notice that we need an explicit lifetime `'a` defined in the signature of
`search` and used with the `contents` argument and the return value. Recall in
[Chapter 10][ch10-lifetimes]<!-- ignore -- > that the lifetime parameters
specify which argument lifetime is connected to the lifetime of the return
value. In this case, we indicate that the returned vector should contain string
slices that reference slices of the argument `contents` (rather than the
argument `query`).
-->

Remarquez que nous avons besoin de préciser explicitement une durée de vie `'a`
définie dans la signature de `rechercher` et l'utiliser sur l'argument `contenu`
et la valeur de retour. Rappelez-vous que dans le
[chapitre 10][ch10-lifetimes]<!-- ignore --> nous avions vu que les paramètres
de durée de vie renseignent à quelle durée de vie des arguments sont connectées
aux durées de vie des valeurs de retour. Dans notre cas, nous indiquons que le
vecteur retourné devrait contenir des slices de chaînes de caractères qui sont
des slices de l'argument `contenu` (et non pas de l'argument `recherche`).

<!--
In other words, we tell Rust that the data returned by the `search` function
will live as long as the data passed into the `search` function in the
`contents` argument. This is important! The data referenced *by* a slice needs
to be valid for the reference to be valid; if the compiler assumes we’re making
string slices of `query` rather than `contents`, it will do its safety checking
incorrectly.
-->

Autrement dit, nous disons à Rust que les données retournées par la fonction
`rechercher` vont vivre aussi longtemps que la donnée dans l'argument `contenu`
de la fonction `rechercher`. C'est très important ! Les données sur lesquelles
pointent les slices doivent être en vigueur pour que la référence reste valide ;
si le compilateur croit que nous créons des slices de `recherche` plutôt que de
`contenu`, ses vérifications de sécurité seront incorrectes.

<!--
If we forget the lifetime annotations and try to compile this function, we’ll
get this error:
-->

Si nous oublions les annotations de durée de vie et que nous essayons de
compiler cette fonction, nous allons obtenir cette erreur :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/output-only-02-missing-lifetimes/output.txt}}
```

<!--
Rust can’t possibly know which of the two arguments we need, so we need to tell
it. Because `contents` is the argument that contains all of our text and we
want to return the parts of that text that match, we know `contents` is the
argument that should be connected to the return value using the lifetime syntax.
-->

Rust ne peut pas deviner lequel des deux arguments nous allons utiliser, donc
nous devons lui dire. Comme `contenu` est l'argument qui contient tout notre
texte et que nous voulons retourner des extraits de ce texte qui correspondent à
la recherche, nous savons que `contenu` est l'argument qui doit être connecté à
la valeur de retour, en utilisant la syntaxe de durée de vie.

<!--
Other programming languages don’t require you to connect arguments to return
values in the signature. Although this might seem strange, it will get easier
over time. You might want to compare this example with the [“Validating
References with Lifetimes”][validating-references-with-lifetimes]<!-- ignore
-- > section in Chapter 10.
-->

Les autres langages de programmation n'ont pas besoin que vous connectiez les
arguments aux valeurs de retour dans la signature. Bien que cela puisse paraître
étrange, cela devient plus facile au fil du temps. Vous devriez peut-être
comparer cet exemple à la
[section 3 du chapitre 10][validating-references-with-lifetimes]<!-- ignore -->.

<!--
Now let’s run the test:
-->

Maintenant, exécutons le test :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/listing-12-16/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-16/output.txt}}
```

<!--
Great, the test fails, exactly as we expected. Let’s get the test to pass!
-->

Très bien, le test a échoué, comme nous nous y attendions. Faisons maintenant en
sorte qu'il réussisse !

<!--
### Writing Code to Pass the Test
-->

### Ecrire du code pour réussir au test

<!--
Currently, our test is failing because we always return an empty vector. To fix
that and implement `search`, our program needs to follow these steps:
-->

Pour le moment, notre test échoue car nous retournons toujours un vecteur vide.
Pour corriger cela et implémenter `rechercher`, notre programme doit suivre les
étapes suivantes :

<!--
* Iterate through each line of the contents.
* Check whether the line contains our query string.
* If it does, add it to the list of values we’re returning.
* If it doesn’t, do nothing.
* Return the list of results that match.
-->

* Itérer sur chacune des lignes de `contenu`.
* Vérifier si la ligne contient la chaîne de caractères recherchée.
* Si c'est le cas, l'ajouter à la liste des valeurs que nous retournerons.
* Si ce n'est pas le cas, ne rien faire.
* Retourner la liste des résultats qui ont été trouvés.

<!--
Let’s work through each step, starting with iterating through lines.
-->

Travaillons sur chacune de ces étapes, en commençant par l'itération sur les
lignes.

<!--
#### Iterating Through Lines with the `lines` Method
-->

#### Itérer sur chacune des lignes avec la méthode `lines`

<!--
Rust has a helpful method to handle line-by-line iteration of strings,
conveniently named `lines`, that works as shown in Listing 12-17. Note this
won’t compile yet.
-->

Rust a une méthode très pratique pour gérer l'itération ligne-par-ligne des
chaînes de caractères, judicieusement appelée `lines`, qui fonctionne comme dans
l'encart 12-17. Notez que cela ne se compile pas encore.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-17/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-17: Iterating through each line in `contents`
</span>
-->

<span class="caption">Encart 12-17 : Itération sur chacune des lignes de
`contenu` </span>

<!-- markdownlint-disable -->
<!--
The `lines` method returns an iterator. We’ll talk about iterators in depth in
[Chapter 13][ch13-iterators]<!-- ignore -- >, but recall that you saw this way of using an
iterator in [Listing 3-5][ch3-iter]<!-- ignore -- >, where we used a `for` loop
with an iterator to run some code on each item in a collection.
-->
<!-- markdownlint-restore -->

La méthode `lines` retourne un itérateur. Nous verrons plus tard les itérateurs
dans le [chapitre 13][ch13-iterators]<!-- ignore -->, mais souvenez-vous que
vous avez vu cette façon d'utiliser un itérateur dans
[l'encart 3-5][ch3-iter]<!-- ignore -->, dans lequel nous avions utilisé une
boucle `for` sur un itérateur pour exécuter du code sur chaque élément d'une
collection.

<!--
#### Searching Each Line for the Query
-->

#### Trouver chaque ligne correspondante à la recherche

<!--
Next, we’ll check whether the current line contains our query string.
Fortunately, strings have a helpful method named `contains` that does this for
us! Add a call to the `contains` method in the `search` function, as shown in
Listing 12-18. Note this still won’t compile yet.
-->

Ensuite, nous allons vérifier que la ligne courante contient la chaîne de
caractères que nous recherchons. Heureusement, les chaînes de caractères ont une
méthode `contains` assez pratique qui fait cela pour nous ! Ajoutez l'appel à
la méthode `contains` dans la fonction `rechercher`, comme dans l'encart 12-18.
Notez qu'ici non plus nous ne pouvons pas encore compiler.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-18/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-18: Adding functionality to see whether the
line contains the string in `query`</span>
-->

<span class="caption">Encart 12-18 : Ajout d'une fonctionnalité pour trouver
quelle ligne contient la chaîne de caractères `recherche`</span>

<!--
#### Storing Matching Lines
-->

#### Stocker les lignes trouvées

<!--
We also need a way to store the lines that contain our query string. For that,
we can make a mutable vector before the `for` loop and call the `push` method
to store a `line` in the vector. After the `for` loop, we return the vector, as
shown in Listing 12-19.
-->

Nous avons aussi besoin d'un moyen de stocker les lignes qui contienent la
chaîne de caractères que nous recherchons. Pour cela, nous pouvons créer un
vecteur mutable avant la boucle `for` et appeler la méthode `push` pour
enregistrer la `ligne` dans le vecteur. Après la boucle `for`, nous retournons
le vecteur, comme dans l'encart 12-19 :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/listing-12-19/src/lib.rs:here}}
```

<!--
<span class="caption">Listing 12-19: Storing the lines that match so we can
return them</span>
-->

<span class="caption">Encart 12-19 : Enregistrement des lignes qui sont
trouvées afin que nous puissions les retourner</span>

<!--
Now the `search` function should return only the lines that contain `query`,
and our test should pass. Let’s run the test:
-->

Maintenant, notre fonction `rechercher` retourne uniquement les lignes qui
contiennent `recherche`, et notre test devrait réussir. Exécutons le test :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/listing-12-19/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/listing-12-19/output.txt}}
```

<!--
Our test passed, so we know it works!
-->

Notre test a réussi, donc nous savons que cela fonctionne !

<!--
At this point, we could consider opportunities for refactoring the
implementation of the search function while keeping the tests passing to
maintain the same functionality. The code in the search function isn’t too bad,
but it doesn’t take advantage of some useful features of iterators. We’ll
return to this example in [Chapter 13][ch13-iterators]<!-- ignore -- >, where we’ll
explore iterators in detail, and look at how to improve it.
-->

Arrivé à ce stade, nous pourrions envisager des pistes de remaniement pour
l'implémentation de la fonction de recherche tout en faisant en sorte que les
tests réussissent toujours afin de conserver les mêmes fonctionnalités. Le code
de la fonction de recherche n'est pas mauvais, mais il ne profite pas de
quelques fonctionnalités utiles des itérateurs. Nous retrouverons cet exemple dans
le [chapitre 13][ch13-iterators]<!-- ignore -->, dans lequel nous explorerons
les itérateurs en détail, et ainsi découvrir comment nous pourrions l'améliorer.

<!--
#### Using the `search` Function in the `run` Function
-->

#### Utiliser la fonction `rechercher` dans la fonction `run`

<!--
Now that the `search` function is working and tested, we need to call `search`
from our `run` function. We need to pass the `config.query` value and the
`contents` that `run` reads from the file to the `search` function. Then `run`
will print each line returned from `search`:
-->

Maintenant que la fonction `rechercher` fonctionne et est testée, nous devons
appeler `rechercher` dans notre fonction `run`. Nous devons passer à
`rechercher` la valeur de `config.recherche`  et le `contenu` que `run` obtient
en lisant le fichier. Ensuite, `run` devra afficher chaque ligne retournée par
`rechercher` :

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/src/lib.rs:here}}
```

<!--
We’re still using a `for` loop to return each line from `search` and print it.
-->

Nous utilisons ici aussi une boucle `for` pour récupérer chaque ligne provenant
de `rechercher` et l'afficher.

<!--
Now the entire program should work! Let’s try it out, first with a word that
should return exactly one line from the Emily Dickinson poem, “frog”:
-->

Maintenant, l'intégralité du programme devrait fonctionner ! Essayons-le, pour
commencer avec un mot qui devrait retourner exactement une seule ligne du poème
d'Emily Dickinson, “frog” :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/no-listing-02-using-search-in-run/output.txt}}
```

<!--
Cool! Now let’s try a word that will match multiple lines, like “body”:
-->

Super ! Maintenant, essayons un mot qui devrait retourner plusieurs lignes,
comme “body” :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/output-only-03-multiple-matches/output.txt}}
```

<!--
And finally, let’s make sure that we don’t get any lines when we search for a
word that isn’t anywhere in the poem, such as “monomorphization”:
-->

Et enfin, assurons-nous que nous n'obtenons aucune ligne lorsque nous cherchons
un mot qui n'est nulle part dans le poème, comme “monomorphization” :

<!--
```console
{{#include ../listings-sources/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```
-->

```console
{{#include ../listings/ch12-an-io-project/output-only-04-no-matches/output.txt}}
```

<!--
Excellent! We’ve built our own mini version of a classic tool and learned a lot
about how to structure applications. We’ve also learned a bit about file input
and output, lifetimes, testing, and command line parsing.
-->

Très bien ! Nous avons construit notre propre mini-version d'un outil classique
et nous avons beaucoup appris à structurer nos applications. Nous avons aussi
appris un peu sur les entrées et sorties des fichiers, les durées de vie, les
tests, et l'interprétation de la ligne de commande.

<!--
To round out this project, we’ll briefly demonstrate how to work with
environment variables and how to print to standard error, both of which are
useful when you’re writing command line programs.
-->

Pour clôturer ce projet, nous allons brièvement voir comment travailler avec les
variables d'environnement et comment écrire sur la sortie standard des erreurs,
qui sont tous les deux utiles lorsque vous écrivez des programmes en ligne de
commande.

<!--
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
[ch11-anatomy]: ch11-01-writing-tests.html#the-anatomy-of-a-test-function
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html#looping-through-a-collection-with-for
[ch13-iterators]: ch13-02-iterators.html
-->

[validating-references-with-lifetimes]: ch10-03-lifetime-syntax.html
[ch11-anatomy]: ch11-01-writing-tests.html
[ch10-lifetimes]: ch10-03-lifetime-syntax.html
[ch3-iter]: ch03-05-control-flow.html
[ch13-iterators]: ch13-02-iterators.html
