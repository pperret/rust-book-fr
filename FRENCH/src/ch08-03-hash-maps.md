> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/88).

<!--
## Storing Keys with Associated Values in Hash Maps
-->

## Stocker des clés associées à des valeurs dans des tables de hachage

<!--
The last of our common collections is the *hash map*. The type `HashMap<K, V>`
stores a mapping of keys of type `K` to values of type `V`. It does this via a
*hashing function*, which determines how it places these keys and values into
memory. Many programming languages support this kind of data structure, but
they often use a different name, such as hash, map, object, hash table,
dictionary, or associative array, just to name a few.
-->

La dernière des collections les plus courantes est la *table de hachage (hash
map)*. Le type `HashMap<K, V>` stocke une association de clés de type `K` à des
valeurs de type `V`. Elle fait cela via une *fonction de hachage*, qui détermine
comment elle va ranger ces clés et valeurs dans la mémoire. De nombreux langages
de programmation prennent en charge ce genre de structure de donnée, mais elles
ont souvent un nom différent, comme hash, map, object, hash table, dictionary,
ou associative array, pour n'en nommer que quelques-uns.

<!--
Hash maps are useful when you want to look up data not by using an index, as
you can with vectors, but by using a key that can be of any type. For example,
in a game, you could keep track of each team’s score in a hash map in which
each key is a team’s name and the values are each team’s score. Given a team
name, you can retrieve its score.
-->

Les tables de hachage sont utiles lorsque vous voulez rechercher des données non
pas en utilisant des indices, comme vous pouvez le faire avec les vecteurs, mais
en utilisant une clé qui peut être de n'importe quel type. Par exemple, dans un
jeu, vous pouvez consigner le score de chaque équipe dans une table de hachage
dans laquelle chaque clé est le nom d'une équipe et la valeur est le score de
l'équipe. Lorsque vous utilisez le nom d'une équipe, vous pouvez récupérer son
score.

<!--
We’ll go over the basic API of hash maps in this section, but many more goodies
are hiding in the functions defined on `HashMap<K, V>` by the standard library.
As always, check the standard library documentation for more information.
-->

Nous allons passer en revue l'API de base des tables de hachage dans cette
section, mais bien d'autres fonctionnalités se cachent dans les fonctions
définies sur `HashMap<K, V>` par la bibliothèque standard. Comme d'habitude,
regardez la documentation de la bibliothèque standard pour plus d'informations.

<!--
### Creating a New Hash Map
-->

### Créer une nouvelle table de hachage

<!--
You can create an empty hash map with `new` and add elements with `insert`. In
Listing 8-20, we’re keeping track of the scores of two teams whose names are
Blue and Yellow. The Blue team starts with 10 points, and the Yellow team
starts with 50.
-->

Vous pouvez créer une table de hachage vide avec `new` et ajouter des éléments
avec `insert`. Dans l'encart 8-20, nous consignons les scores des deux équipes
qui s'appellent Blue et Yellow. L'équipe Blue commence avec 10 points, et
l'équipe Yellow commence avec 50.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);
```

<!--
<span class="caption">Listing 8-20: Creating a new hash map and inserting some
keys and values</span>
-->

<span class="caption">Encart 8-20 : Création d'une nouvelle table de hachage et
insertion de quelques clés et valeurs</span>

<!--
Note that we need to first `use` the `HashMap` from the collections portion of
the standard library. Of our three common collections, this one is the least
often used, so it’s not included in the features brought into scope
automatically in the prelude. Hash maps also have less support from the
standard library; there’s no built-in macro to construct them, for example.
-->

Notez que nous devons d'abord utiliser `use` sur `HashMap` de la partie des
collections de la bibliothèque standard. De nos trois collections courantes,
cette dernière est la moins utilisée, donc elle n'est pas présente dans les
fonctionnalités importées automatiquement dans la portée par l'étape
préliminaire. Les tables de hachages sont aussi moins gérées par la bibliothèque
standard ; il n'y a pas de macro intégrée pour les construire, par exemple.

<!--
Just like vectors, hash maps store their data on the heap. This `HashMap` has
keys of type `String` and values of type `i32`. Like vectors, hash maps are
homogeneous: all of the keys must have the same type, and all of the values
must have the same type.
-->

Exactement comme les vecteurs, les tables de hachage stockent leurs données sur
le tas. Cette `HashMap` a des clés de type `String` et des valeurs de type
`i32`. Et comme les vecteurs, les tables de hachage sont homogènes : toutes les
clés doivent être du même type, et toutes valeurs doivent aussi toutes être du
même type.

<!--
Another way of constructing a hash map is by using the `collect` method on a
vector of tuples, where each tuple consists of a key and its value. The
`collect` method gathers data into a number of collection types, including
`HashMap`. For example, if we had the team names and initial scores in two
separate vectors, we could use the `zip` method to create a vector of tuples
where “Blue” is paired with 10, and so forth. Then we could use the `collect`
method to turn that vector of tuples into a hash map, as shown in Listing 8-21.
-->

Une autre façon de construire une table de hachage est d'utiliser la méthode
`collect` sur un vecteur de tuples, où chaque tuple représente une clé et sa
valeur. La méthode `collect` regroupe les données dans quelques types de
collections, dont `HashMap`. Par exemple, si nous avions les noms des équipes et
les scores initiaux dans deux vecteurs séparés, nous pourrions utiliser la
méthode `zip` pour créer un vecteur de tuples où “Blue” est associé à 10, et
ainsi de suite. Ensuite nous pourrions utiliser la méthode `collect` pour
transformer ce vecteur de tuples en table de hachage, comme nous l'avons fait
dans l'encart 8-21.

```rust
use std::collections::HashMap;

let teams  = vec![String::from("Blue"), String::from("Yellow")];
let initial_scores = vec![10, 50];

let scores: HashMap<_, _> = teams.iter().zip(initial_scores.iter()).collect();
```

<!--
<span class="caption">Listing 8-21: Creating a hash map from a list of teams
and a list of scores</span>
-->

<span class="caption">Encart 8-21 : Création d'une table de hachage à partir
d'une liste d'équipe et une liste de scores</span>

<!--
The type annotation `HashMap<_, _>` is needed here because it’s possible to
`collect` into many different data structures and Rust doesn’t know which you
want unless you specify. For the parameters for the key and value types,
however, we use underscores, and Rust can infer the types that the hash map
contains based on the types of the data in the vectors.
-->

L'annotation de type `HashMap<_, _>` est nécessaire ici car `collect` peut
générer plusieurs types de structures de données et Rust ne sait pas celle que
vous souhaitez si vous ne le précisez pas. Mais pour les paramètres qui
correspondent aux types de clés et de valeur, nous utilisons des tirets bas, et
Rust peut déduire les types que la table de hachage contient en fonction des
types de données présentes dans les vecteurs.

<!--
### Hash Maps and Ownership
-->

### Les tables de hachage et la possession

<!--
For types that implement the `Copy` trait, like `i32`, the values are copied
into the hash map. For owned values like `String`, the values will be moved and
the hash map will be the owner of those values, as demonstrated in Listing 8-22.
-->

Pour les types qui implémentent le trait `Copy`, comme `i32`, les valeurs sont
copiées dans la table de hachage. Pour les valeurs qui sont possédées comme
`String`, les valeurs seront déplacées et la table de hachage sera la
propriétaire de ces valeurs, comme démontré dans l'encart 8-22.

<!--
```rust
use std::collections::HashMap;

let field_name = String::from("Favorite color");
let field_value = String::from("Blue");

let mut map = HashMap::new();
map.insert(field_name, field_value);
// field_name and field_value are invalid at this point, try using them and
// see what compiler error you get!
```
-->

```rust
use std::collections::HashMap;

let nom_champ = String::from("Couleur favorite");
let valeur_champ = String::from("Bleu");

let mut table = HashMap::new();
table.insert(nom_champ, valeur_champ);
// nom_champ et valeur_champ ne sont plus en vigueur à partir d'ici, essayez de
// les utiliser et vous verrez l'erreur du compilateur que vous obtiendrez ! 
```

<!--
<span class="caption">Listing 8-22: Showing that keys and values are owned by
the hash map once they’re inserted</span>
-->

<span class="caption">Encart 8-22 : Démonstration que les clés et les valeurs
sont possédées par la table de hachage une fois qu'elles sont insérées</span>

<!--
We aren’t able to use the variables `field_name` and `field_value` after
they’ve been moved into the hash map with the call to `insert`.
-->

Nous ne pouvons plus utiliser les variables `nom_champ` et `valeur_champ` après
qu'elles aient été déplacées dans la table de hachage lors de l'appel à
`insert`.

<!--
If we insert references to values into the hash map, the values won’t be moved
into the hash map. The values that the references point to must be valid for at
least as long as the hash map is valid. We’ll talk more about these issues in
the [“Validating References with
Lifetimes”][validating-references-with-lifetimes]<!-- ignore -- > section in
Chapter 10.
-->

Si nous utilisons des références vers les valeurs dans la table de hachage, les
valeurs ne vont pas être déplacées dans la table de hachage. Les valeurs sur
lesquelles elle pointe resteront en vigueur tant que la table de hachage le sera
aussi. Nous verrons ces problématiques dans
[une section du chapitre 10][validating-references-with-lifetimes]<!-- ignore -->.

<!--
### Accessing Values in a Hash Map
-->

### Accéder aux valeurs dans une table de hachage

<!--
We can get a value out of the hash map by providing its key to the `get`
method, as shown in Listing 8-23.
-->

Nous pouvons obtenir une valeur d'une table de hachage en donnant sa clé à la
méthode `get`, comme dans l'encart 8-23.

<!--
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let team_name = String::from("Blue");
let score = scores.get(&team_name);
```
-->

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

let nom_equipe = String::from("Blue");
let score = scores.get(&nom_equipe);
```

<!--
<span class="caption">Listing 8-23: Accessing the score for the Blue team
stored in the hash map</span>
-->

<span class="caption">Encart 8-23 : Récupérer le score de l'équipe `Blue`,
stocké dans la table de hachage</span>

<!--
Here, `score` will have the value that’s associated with the Blue team, and the
result will be `Some(&10)`. The result is wrapped in `Some` because `get`
returns an `Option<&V>`; if there’s no value for that key in the hash map,
`get` will return `None`. The program will need to handle the `Option` in one
of the ways that we covered in Chapter 6.
-->

Dans notre cas, `score` aura la valeur qui est associée à l'équipe `Blue`, et le
résultat sera `Some(&10)`. Le résultat est encapsulé dans un `Some` car `get`
retourne une `Option<&V>` : s'il n'y a pas de valeur pour cette clé dans la
table de hachage, `get` va retourner `None`. Le programme doit gérer le `Option`
d'une des manières dont nous avons parlé dans le chapitre 6.

<!--
We can iterate over each key/value pair in a hash map in a similar manner as we
do with vectors, using a `for` loop:
-->

Nous pouvons itérer sur chaque paire de clé/valeur dans une table de hachage de
la même manière que nous le faisons avec les vecteurs, en utilisant une boucle
`for` :

<!--
```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (key, value) in &scores {
    println!("{}: {}", key, value);
}
```
-->

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Yellow"), 50);

for (clee, valeur) in &scores {
    println!("{} : {}", clee, valeur);
}
```

<!--
This code will print each pair in an arbitrary order:
-->

Ce code va afficher chaque paire dans un ordre arbitraire :

<!--
```text
Yellow: 50
Blue: 10
```
-->

```text
Yellow : 50
Blue : 10
```

<!--
### Updating a Hash Map
-->

### Modifier une table de hachage

<!--
Although the number of keys and values is growable, each key can only have one
value associated with it at a time. When you want to change the data in a hash
map, you have to decide how to handle the case when a key already has a value
assigned. You could replace the old value with the new value, completely
disregarding the old value. You could keep the old value and ignore the new
value, only adding the new value if the key *doesn’t* already have a value. Or
you could combine the old value and the new value. Let’s look at how to do each
of these!
-->

Bien que le nombre de clés et de valeurs puisse augmenter, chaque clé peut
avoir seulement une seule valeur associée à elle au même moment. Lorsque vous
souhaitez changer les données dans une table de données, vous devez choisir
comment gérer le cas lorsque une clé a déjà une valeur qui lui est déjà
associée. Vous pouvez soit remplacer l'ancienne valeur avec la nouvelle valeur,
en ignorant complètement l'ancienne valeur. Vous pouvez garder l'ancienne valeur
et ignorer la nouvelle valeur, en insérant la nouvelle valeur uniquement si la
clé *n'a pas* déjà une valeur. Ou vous pouvez fusionner l'ancienne valeur et la
nouvelle. Découvrons dès maintenant comment faire chacune de ces actions !

<!--
#### Overwriting a Value
-->

#### Réécrire une valeur

<!--
If we insert a key and a value into a hash map and then insert that same key
with a different value, the value associated with that key will be replaced.
Even though the code in Listing 8-24 calls `insert` twice, the hash map will
only contain one key/value pair because we’re inserting the value for the Blue
team’s key both times.
-->

Si nous ajoutons une clé et une valeur dans une table de hachage et que nous
ajoutons à nouveau la même clé avec une valeur différente, la valeur associée
à cette clé sera remplacée. Même si le code dans l'encart 8-24 appelle deux
fois `insert`, la table de hachage contiendra un seul couple de clé/valeur car
nous ajoutons la valeur pour l'équipe `Blue` à deux reprises.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();

scores.insert(String::from("Blue"), 10);
scores.insert(String::from("Blue"), 25);

println!("{:?}", scores);
```

<!--
<span class="caption">Listing 8-24: Replacing a value stored with a particular
key</span>
-->

<span class="caption">Encart 8-24 : Remplacement d'une valeur stockée sous une
clé spécifique</span>

<!--
This code will print `{"Blue": 25}`. The original value of `10` has been
overwritten.
-->

Ce code va afficher `{"Blue": 25}`. La valeur initiale `10` a été remplacée.

<!--
#### Only Inserting a Value If the Key Has No Value
-->

#### Ajouter une valeur seulement si la clé n'a pas déjà de valeur

<!--
It’s common to check whether a particular key has a value and, if it doesn’t,
insert a value for it. Hash maps have a special API for this called `entry`
that takes the key you want to check as a parameter. The return value of the
`entry` method is an enum called `Entry` that represents a value that might or
might not exist. Let’s say we want to check whether the key for the Yellow team
has a value associated with it. If it doesn’t, we want to insert the value 50,
and the same for the Blue team. Using the `entry` API, the code looks like
Listing 8-25.
-->

Il est courant de vérifier qu'une clé spécifique a déjà une valeur, et si ce
n'est pas le cas, de lui associer une valeur. Les tables de hachage ont une API
spécifique pour ce cas-là qui s'appelle `entry` et qui prend en paramètre la
clé que vous voulez vérifier. La valeur de retour de la méthode `entry` est une
énumération qui s'appelle `Entry` qui représente une valeur qui existe ou non.
Imaginons que nous souhaitons vérifier si la clé pour l'équipe `Yellow` a une
valeur qui lui est associée. Si ce n'est pas le cas, nous voulons lui associer
la valeur 50, et faire de même pour l'équipe `Blue`. En utilisant l'API `entry`,
ce code va ressembler à l'encart 8-25.

```rust
use std::collections::HashMap;

let mut scores = HashMap::new();
scores.insert(String::from("Blue"), 10);

scores.entry(String::from("Yellow")).or_insert(50);
scores.entry(String::from("Blue")).or_insert(50);

println!("{:?}", scores);
```

<!--
<span class="caption">Listing 8-25: Using the `entry` method to only insert if
the key does not already have a value</span>
-->

<span class="caption">Encart 8-25 : Utilisation de la méthode `entry` pour
ajouter la clé uniquement si elle n'a pas déjà de valeur associée</span>

<!--
The `or_insert` method on `Entry` is defined to return a mutable reference to
the value for the corresponding `Entry` key if that key exists, and if not,
inserts the parameter as the new value for this key and returns a mutable
reference to the new value. This technique is much cleaner than writing the
logic ourselves and, in addition, plays more nicely with the borrow checker.
-->

La méthode `or_insert` sur `Entry` est conçue pour retourner une référence
mutable vers la valeur `Entry` pour la clé correspondante si cette clé existe,
et sinon, d'ajouter son paramètre comme nouvelle valeur pour cette clé et
retourner une référence mutable vers la nouvelle valeur. Cette technique est
plus propre que d'écrire la logique nous-mêmes et, de plus, elle fonctionne
mieux avec le vérificateur d'emprunt.

<!--
Running the code in Listing 8-25 will print `{"Yellow": 50, "Blue": 10}`. The
first call to `entry` will insert the key for the Yellow team with the value
50 because the Yellow team doesn’t have a value already. The second call to
`entry` will not change the hash map because the Blue team already has the
value 10.
-->

L'exécution du code de l'encart 8-25 va afficher `{"Yellow": 50, "Blue": 10}`.
Le premier appel à `entry` va ajouter la clé pour l'équipe `Yellow` avec la
valeur `50` car l'équipe `Yellow` n'a pas encore de valeur. Le second appel à
`entry` ne vas pas changer la table de hachage car l'équipe `Blue` a déjà la
valeur `10`.

<!--
#### Updating a Value Based on the Old Value
-->

#### Modifier une valeur en fonction de l'ancienne valeur

<!--
Another common use case for hash maps is to look up a key’s value and then
update it based on the old value. For instance, Listing 8-26 shows code that
counts how many times each word appears in some text. We use a hash map with
the words as keys and increment the value to keep track of how many times we’ve
seen that word. If it’s the first time we’ve seen a word, we’ll first insert
the value 0.
-->

Une autre utilisation courante avec les tables de hachage est de regarder la
valeur d'une clé et ensuite la modifier en fonction de l'ancienne valeur. Par
exemple, l'encart 8-26 contient du code qui compte combien de fois chaque mot
apparaît dans du texte. Nous utilisons une table de hachage avec les mots comme
clés et nous incrémentons la valeur pour compter combien de fois nous avons vu
ce mot. Si c'est la première fois que nous voyons un mot, nous allons insérer la
valeur `0`.

<!--
```rust
use std::collections::HashMap;

let text = "hello world wonderful world";

let mut map = HashMap::new();

for word in text.split_whitespace() {
    let count = map.entry(word).or_insert(0);
    *count += 1;
}

println!("{:?}", map);
```
-->

```rust
use std::collections::HashMap;

let texte = "bonjour le monde magnifique monde";

let mut tableau = HashMap::new();

for mot in texte.split_whitespace() {
    let compteur = map.entry(mot).or_insert(0);
    *compteur += 1;
}

println!("{:?}", tableau);
```

<!--
<span class="caption">Listing 8-26: Counting occurrences of words using a hash
map that stores words and counts</span>
-->

<span class="caption">Encart 8-26 : Comptage des occurrences des mots en
utilisant une table de hachage qui stocke les mots et leur quantité</span>

<!--
This code will print `{"world": 2, "hello": 1, "wonderful": 1}`. The
`or_insert` method actually returns a mutable reference (`&mut V`) to the value
for this key. Here we store that mutable reference in the `count` variable, so
in order to assign to that value, we must first dereference `count` using the
asterisk (`*`). The mutable reference goes out of scope at the end of the `for`
loop, so all of these changes are safe and allowed by the borrowing rules.
-->

Ce code va afficher `{"monde": 2, "bonjour": 1, "magnifique": 1, "le": 1}`. La
méthode `or_insert` retourne une référence mutable (`&mut V`) vers la valeur de
cette clé. Nous avons décidé de stocker la référence mutable dans la variable
`compteur`, donc pour affecter une valeur, nous devons d'abord déréférencer
`compteur` en utilisant l'astérisque (`*`). La référence mutable sort de la
portée à la fin de la boucle `for`, donc tous ces changements sont sûrs et
autorisés par les règles d'emprunt.

<!--
### Hashing Functions
-->

### Fonctions de hachages

<!--
By default, `HashMap` uses a “cryptographically strong”[^siphash] hashing
function that can provide resistance to Denial of Service (DoS) attacks. This
is not the fastest hashing algorithm available, but the trade-off for better
security that comes with the drop in performance is worth it. If you profile
your code and find that the default hash function is too slow for your
purposes, you can switch to another function by specifying a different
*hasher*. A hasher is a type that implements the `BuildHasher` trait. We’ll
talk about traits and how to implement them in Chapter 10. You don’t
necessarily have to implement your own hasher from scratch;
[crates.io](https://crates.io/) has libraries shared by other Rust users that
provide hashers implementing many common hashing algorithms.
-->

Par défaut, `HashMap` utilise une fonction de hachage
“robuste cryptographiquement”[^siphash] qui résiste aux attaques par Déni de
Service (DoS). Ce n'est pas l'algorithme de hachage le plus rapide qui existe,
mais le compromis entre une meilleure sécurité et la baisse de performances en
vaut la peine. Si vous analysez la performance de votre code et que vous vous
rendez compte que la fonction de hachage par défaut est trop lente pour vos
besoins, vous pouvez changer pour une autre fonction en spécifiant un *hacheur*
différent. Un hacheur est un type qui implémente le trait `BuildHasher`. Nous
verrons les traits et comment les implémenter au chapitre 10. Vous n'avez pas
forcément besoin d'implémenter votre propre hacheur à partir de zéro;
[crates.io](https://crates.io/) héberge des bibliothèques partagées par d'autres
utilisateurs de Rust qui fournissent de nombreux algorithmes de hachage
répandus.

[^siphash]: [https://www.131002.net/siphash/siphash.pdf](https://www.131002.net/siphash/siphash.pdf)

<!--
## Summary
-->

## Résumé

<!--
Vectors, strings, and hash maps will provide a large amount of functionality
necessary in programs when you need to store, access, and modify data. Here are
some exercises you should now be equipped to solve:
-->

Les vecteurs, String, et tables de hachage vont vous apporter de nombreuses
fonctionnalités nécessaires à vos programmes lorsque vous avez besoin de
stocker, accéder, et modifier des données. Voici quelques exercices pour
lesquels vous êtes maintenant en mesure de résoudre :

<!--
* Given a list of integers, use a vector and return the mean (the average
  value), median (when sorted, the value in the middle position), and mode (the
  value that occurs most often; a hash map will be helpful here) of the list.
* Convert strings to pig latin. The first consonant of each word is moved to
  the end of the word and “ay” is added, so “first” becomes “irst-fay.” Words
  that start with a vowel have “hay” added to the end instead (“apple” becomes
  “apple-hay”). Keep in mind the details about UTF-8 encoding!
* Using a hash map and vectors, create a text interface to allow a user to add
  employee names to a department in a company. For example, “Add Sally to
  Engineering” or “Add Amir to Sales.” Then let the user retrieve a list of all
  people in a department or all people in the company by department, sorted
  alphabetically.
-->

* A partir d'une liste d'entier, utiliser un vecteur et retourner la moyenne, la
  médiane (la valeur au milieu lorsque la liste est triée), et la valeur modale
  (la valeur qui apparaît le plus souvent ; une table de hachage sera utile dans
  ce cas) de la liste.
* Convertir des chaînes de caractères en Louchébem. La première syllabe de
  chaque mot est déplacée à la fin du mot et un suffixe argotique et substituée
  par un `l` (L minuscule), ainsi “bonjour” devient “*l*onjour*bem*”. Les mots
  qui commencent par un L ou une voyelle remplacent la syllabe suivante par un L
  et rétablit en fin de mot la consonne initiale avec un suffixe libre. Et
  gardez en tête les détails à propos de l'encodage UTF-8 !
* En utilisant une table de hachage et des vecteurs, créez une interface
  textuelle pour permettre à un utilisateur d'ajouter des noms d'employés dans
  un département d'une entreprise. Par exemple, “Ajouter Sally au Bureau
  d'Etudes” ou “Ajouter Amir au Service Commercial”. Ensuite, donnez la
  possibilité à l'utilisateur de récupérer une liste de toutes les personnes
  dans un département, ou tout le monde dans l'entreprise trié par département,
  et classés dans l'ordre alphabétique dans tous les cas.

<!--
The standard library API documentation describes methods that vectors, strings,
and hash maps have that will be helpful for these exercises!
-->

La documentation de l'API de la bibliothèque standard décrit les méthodes qu'ont
les vecteurs, chaînes de caractères et tables de hachage, ce qui vous sera bien
utile pour mener à bien ces exercices !

<!--
We’re getting into more complex programs in which operations can fail, so, it’s
a perfect time to discuss error handling. We’ll do that next!
-->

Nous nous lançons dans des programmes de plus en plus complexes dans lesquels
les opérations peuvent échouer, c'est donc le moment idéal pour voir comment
bien gérer les erreurs. C'est que nous allons faire au prochain chapitre !

<!--
[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html#validating-references-with-lifetimes
-->

[validating-references-with-lifetimes]:
ch10-03-lifetime-syntax.html
