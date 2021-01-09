> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/120).

<!--
## Validating References with Lifetimes
-->

## La conformité des références avec les durées de vies

<!--
One detail we didn’t discuss in the [“References and
Borrowing”][references-and-borrowing]<!-- ignore -- > section in Chapter 4 is
that every reference in Rust has a *lifetime*, which is the scope for which
that reference is valid. Most of the time, lifetimes are implicit and
inferred, just like most of the time, types are inferred. We must annotate
types when multiple types are possible. In a similar way, we must annotate
lifetimes when the lifetimes of references could be related in a few different
ways. Rust requires us to annotate the relationships using generic lifetime
parameters to ensure the actual references used at runtime will definitely be
valid.
-->

Il reste un détail que nous n'avons pas abordé dans la section [“Les références
et l'emprunt”][references-and-borrowing]<!-- ignore --> du chapitre 4, c'est que
toutes les références ont une *durée de vie* dans Rust, qui est la portée pour
laquelle cette référence est en vigueur. La plupart du temps, les durées de
vies sont implicites et sont déduit automatiquement, comme pour la plupart du
temps les types sont déduits. Nous devons renseigner le type lorsque plusieurs
types sont possibles. De la même manière, nous devons renseigner les durées de
vie lorsque les durées de vies des références peuvent être déduites de
différentes manières. Rust nécessite que nous renseignons ces relations en
utilisant des paramètres de durée de vie génériques pour s'assurer que les
références utilisées au moment de la compilation restent bien en vigueur.

<!--
The concept of lifetimes is somewhat different from tools in other programming
languages, arguably making lifetimes Rust’s most distinctive feature. Although
we won’t cover lifetimes in their entirety in this chapter, we’ll discuss
common ways you might encounter lifetime syntax so you can become familiar with
the concepts.
-->

Le concept de la durée de vie est quelque chose de radicalement différent de ce
que l'on retrouve dans les outils des autres langages de programmation, à un
tel point que la durée de vie est la fonctionnalité qui distingue Rust des
autres. Bien que nous ne puissions couvrir l'intégralité de la durée de vie dans
ce chapitre, nous allons voir les cas les plus courants où vous allez
rencontrer la syntaxe de la durée de vie, afin de vous familiariser avec ses
concepts.

<!--
### Preventing Dangling References with Lifetimes
-->

### Eviter les références pendouillantes avec les durées de vie

<!--
The main aim of lifetimes is to prevent dangling references, which cause a
program to reference data other than the data it’s intended to reference.
Consider the program in Listing 10-17, which has an outer scope and an inner
scope.
-->

L'objectif principal des durées de vies est d'éviter les références
pendouillantes qui font qu'un programme pointe des données autres que celles sur
lesquelles il était censé pointer. Admettons le programme de l'encart 10-17, qui
a une portée externe et une portée interne.

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-17: An attempt to use a reference whose value
has gone out of scope</span>
-->

<span class="caption">Encart 10-17 : tentative d'utiliser une référence vers
une valeur qui est sortie de la portée</span>

<!--
> Note: The examples in Listings 10-17, 10-18, and 10-24 declare variables
> without giving them an initial value, so the variable name exists in the
> outer scope. At first glance, this might appear to be in conflict with Rust’s
> having no null values. However, if we try to use a variable before giving it
> a value, we’ll get a compile-time error, which shows that Rust indeed does
> not allow null values.
-->

> Remarque : Les exemples dans les encarts 10-17, 10-18 et 10-24 déclarent des
> variables sans initialiser leur valeur, donc les noms de ces variables
> existent dans la portée externe. A première vue, cela semble être en conflit
> avec le fonctionnement de Rust qui n'utilise pas les valeurs nulles.
> Cependant, si nous essayons d'utiliser une variable avant de lui donner une
> valeur, nous aurons une erreur au moment de la compilation, qui confirme que
> Rust ne fonctionne pas avec des valeurs nulles.

<!--
The outer scope declares a variable named `r` with no initial value, and the
inner scope declares a variable named `x` with the initial value of 5. Inside
the inner scope, we attempt to set the value of `r` as a reference to `x`. Then
the inner scope ends, and we attempt to print the value in `r`. This code won’t
compile because the value `r` is referring to has gone out of scope before we
try to use it. Here is the error message:
-->

La portée externe déclare une variable `r` sans valeur initiale, et la portée
interne déclare une variable `x` avec la valeur initiale à `5`. Au sein de la
portée interne, nous essayons d'assigner la valeur de `r` comme étant une
référence à `x`. Puis la portée interne se ferme, et nous essayons d'afficher la
valeur dans `r`. Ce code ne va pas se compiler car la valeur `r` se réfère à
quelque chose qui est sortie de la portée avant que nous essayons de l'utiliser.
Voici le message d'erreur :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-17/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-17/output.txt}}
```

<!--
The variable `x` doesn’t “live long enough.” The reason is that `x` will be out
of scope when the inner scope ends on line 7. But `r` is still valid for the
outer scope; because its scope is larger, we say that it “lives longer.” If
Rust allowed this code to work, `r` would be referencing memory that was
deallocated when `x` went out of scope, and anything we tried to do with `r`
wouldn’t work correctly. So how does Rust determine that this code is invalid?
It uses a borrow checker.
-->

La variable `x` n'existe plus (“does not live long enough”). La raison à cela
est que `x` est sortie de la portée lorsque la portée interne s'est fermée à la
ligne 7. Mais `r` reste en vigueur dans la portée externe ; car sa portée est
plus grande, on dit qu'il “vit plus longtemps”. Si Rust avait permis à ce code de
s'exécuter, `r` pointerait sur de la mémoire désallouée dès que `x` est sortie
de la portée, ainsi tout ce que nous pourrions faire avec `r` ne fonctionnerait
pas correctement. Mais comment Rust détecte que ce code est invalide ? Il
utilise le vérificateur d'emprunt.

<!--
### The Borrow Checker
-->

### Le vérificateur d'emprunt

<!--
The Rust compiler has a *borrow checker* that compares scopes to determine
whether all borrows are valid. Listing 10-18 shows the same code as Listing
10-17 but with annotations showing the lifetimes of the variables.
-->

Le compilateur de Rust embarque un *vérificateur d'emprunt* (borrow checker) qui
compare les portées pour déterminer si les emprunts sont valides. L'encart 10-18
montre le même code que l'encart 10-17, mais avec des commentaires qui montrent
les durées de vies des variables.

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-18/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-18: Annotations of the lifetimes of `r` and
`x`, named `'a` and `'b`, respectively</span>
-->

<span class="caption">Encart 10-18 : commentaires pour montrer les durées de vie
de `r` et `x`, qui s'appellent respectivement `'a` et `'b`</span>

<!--
Here, we’ve annotated the lifetime of `r` with `'a` and the lifetime of `x`
with `'b`. As you can see, the inner `'b` block is much smaller than the outer
`'a` lifetime block. At compile time, Rust compares the size of the two
lifetimes and sees that `r` has a lifetime of `'a` but that it refers to memory
with a lifetime of `'b`. The program is rejected because `'b` is shorter than
`'a`: the subject of the reference doesn’t live as long as the reference.
-->

Ici, nous avons montré la durée de vie de `r` avec `'a` et la durée de vie de
`x` avec `'b`. Comme vous pouvez le constater, le bloc interne `'b` est bien
plus petit que le bloc externe `'a`. Au moment de la compilation, Rust compare
les tailles des deux durées de vies et constate que `r` a la durée de vie `'a`
mais fait référence à de la mémoire qui a une durée de vie de `'b`. Ce programme
est refusé car `'b` est plus court que `'a` : l'élément pointé par la référence
n'existe pas aussi longtemps que la référence.

<!--
Listing 10-19 fixes the code so it doesn’t have a dangling reference and
compiles without any errors.
-->

L'encart 10-19 résout le code afin qu'il n'ai plus de référence pendouillante et
qu'il se compile sans erreur.

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-19/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-19: A valid reference because the data has a
longer lifetime than the reference</span>
-->

<span class="caption">Encart 10-19 : la référence est valide puisque la donnée a
une durée de vie plus longue que la référence</span>

<!--
Here, `x` has the lifetime `'b`, which in this case is larger than `'a`. This
means `r` can reference `x` because Rust knows that the reference in `r` will
always be valid while `x` is valid.
-->

Ici, `x` a la durée de vie `'b`, qui est plus grand dans ce cas que `'a`. Cela
signifie que `r` peut référencer `x` car Rust sait que la référence présente
dans `r` sera toujours valide du moment que `x` est en vigueur.

<!--
Now that you know where the lifetimes of references are and how Rust analyzes
lifetimes to ensure references will always be valid, let’s explore generic
lifetimes of parameters and return values in the context of functions.
-->

Maintenant que vous savez où se situent les durées de vie des références et
comment Rust analyse les durées de vies pour s'assurer que les références soient
toujours en vigueur, découvrons les durées de vies génériques des paramètres et
des valeurs de retour dans le cas des fonctions.

<!--
### Generic Lifetimes in Functions
-->

### Les durées de vies génériques dans les fonctions

<!--
Let’s write a function that returns the longer of two string slices. This
function will take two string slices and return a string slice. After we’ve
implemented the `longest` function, the code in Listing 10-20 should print `The
longest string is abcd`.
-->

Ecrivons une fonction qui retourne la plus longue des slice d'une chaîne de
caractères. Cette fonction va prendre en argument deux slices de chaîne de
caractères et retourner une slice d'une chaîne de caractères. Après avoir
implémenté la fonction `la_plus_longue`, le code de l'encart 10-20 devrait
afficher `La plus grande chaîne est abcd`.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs}}
```
-->

```rust,ignore
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-20/src/main.rs}}
```

<!--
<span class="caption">Listing 10-20: A `main` function that calls the `longest`
function to find the longer of two string slices</span>
-->

<span class="caption">Encart 10-20 : une fonction `main` qui appelle la
fonction `la_plus_longue` pour trouver la plus grande des deux slices de chaîne
de caractères
</span>

<!--
Note that we want the function to take string slices, which are references,
because we don’t want the `longest` function to take ownership of its
parameters. Refer to the [“String Slices as
Parameters”][string-slices-as-parameters]<!-- ignore -- > section in Chapter 4
for more discussion about why the parameters we use in Listing 10-20 are the
ones we want.
-->

Remarquez que nous souhaitons que la fonction prenne deux slices de chaînes de
caractères, qui sont des références, car nous ne voulons pas que la fonction
`la_plus_longue` prenne possession de ses paramètres. Rendez-vous à la section
[“Les slices de chaînes de caractères en
paramètres”][string-slices-as-parameters]<!-- ignore --> du chapitre 4 pour
savoir pourquoi nous utilisons ce type de paramètre dans l'encart 10-20.

<!--
If we try to implement the `longest` function as shown in Listing 10-21, it
won’t compile.
-->

Si nous essayons d'implémenter la fonction `la_plus_longue` comme dans l'encart
10-21, cela ne va pas se compiler.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-21: An implementation of the `longest`
function that returns the longer of two string slices but does not yet
compile</span>
-->

<span class="caption">Encart 10-21 : une implémentation de la fonction
`la_plus_longue` qui retourne la plus longue des deux slices de chaînes de
caractères, mais ne se compile pas encore</span>

<!--
Instead, we get the following error that talks about lifetimes:
-->

A la place, nous obtenons l'erreur suivante qui nous parle de durées de vie :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-21/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-21/output.txt}}
```

<!--
The help text reveals that the return type needs a generic lifetime parameter
on it because Rust can’t tell whether the reference being returned refers to
`x` or `y`. Actually, we don’t know either, because the `if` block in the body
of this function returns a reference to `x` and the `else` block returns a
reference to `y`!
-->

La partie “help” nous explique que le type de retour a besoin d'un paramètre de
durée de vie générique car Rust ne sait pas si la référence retournée est liée à
`x` ou à `y`. Pour le moment, nous ne le savons pas nous non plus, car le bloc
`if` dans le corps de cette fonction retourne une référence à `x` et le bloc
`else` retourne une référence à `y` !

<!--
When we’re defining this function, we don’t know the concrete values that will
be passed into this function, so we don’t know whether the `if` case or the
`else` case will execute. We also don’t know the concrete lifetimes of the
references that will be passed in, so we can’t look at the scopes as we did in
Listings 10-18 and 10-19 to determine whether the reference we return will
always be valid. The borrow checker can’t determine this either, because it
doesn’t know how the lifetimes of `x` and `y` relate to the lifetime of the
return value. To fix this error, we’ll add generic lifetime parameters that
define the relationship between the references so the borrow checker can
perform its analysis.
-->

Lorsque nous définissons cette fonction, nous ne connaissons pas les valeurs
concrètes qui vont passer dans cette fonction, donc nous ne savons pas si nous
allons exécuter le cas du `if` ou du `else`. Nous ne connaissons pas non plus les
durées de vie des références qui vont passer dans la fonction, donc nous ne
pouvons pas vérifier les portées comme nous l'avons fait dans les encarts 10-18
et 10-19 pour déterminer que la référence que nous allons retourner sera
toujours en vigueur. Le vérificateur d'emprunt ne va pas pouvoir non plus
déterminer cela, car il ne sait comment les durées de vie de `x` et de `y` sont
reliées à la durée de vie de la valeur de retour. Pour résoudre cette erreur,
nous allons ajouter des paramètres de durée de vie génériques qui définissent
la relation entre les références, afin que le vérificateur d'emprunt puisse
faire cette analyse.

<!--
### Lifetime Annotation Syntax
-->

### La syntaxe pour annoter les durées de vies

<!--
Lifetime annotations don’t change how long any of the references live. Just
as functions can accept any type when the signature specifies a generic type
parameter, functions can accept references with any lifetime by specifying a
generic lifetime parameter. Lifetime annotations describe the relationships of
the lifetimes of multiple references to each other without affecting the
lifetimes.
-->

L'annotation des durées de vie ne change pas la longueur de leur durée de vie.
Comme une fonction accepte n'importe quel type lorsque la signature utilise un
paramètre de type générique, les fonctions peuvent accepter des références avec
n'importe quelle durée de vie en précisant un paramètre de durée de vie
générique. L'annotation des durées de vie décrit la relation des durées de vies
de plusieurs références entre elles sans influencer les durées de vie.

<!--
Lifetime annotations have a slightly unusual syntax: the names of lifetime
parameters must start with an apostrophe (`'`) and are usually all lowercase and
very short, like generic types. Most people use the name `'a`. We place
lifetime parameter annotations after the `&` of a reference, using a space to
separate the annotation from the reference’s type.
-->

L'annotation des durées de vies a une syntaxe un peu inhabituelle : le nom des
paramètres de durées de vies doit commencer par une apostrophe (`'`) et sont
habituellement en minuscule et très court, comme les types génériques. La
plupart des personnes utilisent le nom `'a`. Nous plaçons le paramètre de type
après le `&` d'une référence, en utilisant un espace pour séparer l'annotation
du type de la référence.

<!--
Here are some examples: a reference to an `i32` without a lifetime parameter, a
reference to an `i32` that has a lifetime parameter named `'a`, and a mutable
reference to an `i32` that also has the lifetime `'a`.
-->

Voici quelques exemples : une référence à un `i32` sans paramètre de durée de
vie, une référence à un `i32` qui a un paramètre de durée de vie `'a`, et une
référence mutable à un `i32` qui a aussi la durée de vie `'a`.

<!--
```rust,ignore
&i32        // a reference
&'a i32     // a reference with an explicit lifetime
&'a mut i32 // a mutable reference with an explicit lifetime
```
-->

```rust,ignore
&i32        // une référence
&'a i32     // une référence avec une durée de vie explicite
&'a mut i32 // une référence mutable avec une durée de vie explicite
```

<!--
One lifetime annotation by itself doesn’t have much meaning, because the
annotations are meant to tell Rust how generic lifetime parameters of multiple
references relate to each other. For example, let’s say we have a function with
the parameter `first` that is a reference to an `i32` with lifetime `'a`. The
function also has another parameter named `second` that is another reference to
an `i32` that also has the lifetime `'a`. The lifetime annotations indicate
that the references `first` and `second` must both live as long as that generic
lifetime.
-->

Une annotation de durée de vie toute seule n'a pas vraiment de sens, car les
annotations sont faites pour indiquer à Rust quels paramètres de durée de vie
génériques de plusieurs références sont liés aux autres. Par exemple, disons que
nous avons une fonction avec le paramètre `premier` qui est une référence à un
`i32` avec la durée de vie `'a`. La fonction a aussi un autre paramètre `second`
qui est une autre référence à un `i32` qui a aussi la durée de vie `'a`. Les
annotations de durée de vie indiquent que les références `premier` et `second`
doivent tous les deux exister aussi longtemps que la durée de vie générique.

<!--
### Lifetime Annotations in Function Signatures
-->

### Les annotations de durée de vie dans les signatures des fonctions

<!--
Now let’s examine lifetime annotations in the context of the `longest`
function. As with generic type parameters, we need to declare generic lifetime
parameters inside angle brackets between the function name and the parameter
list. The constraint we want to express in this signature is that all the
references in the parameters and the return value must have the same lifetime.
We’ll name the lifetime `'a` and then add it to each reference, as shown in
Listing 10-22.
-->

Maintenant, examinons les annotations de durée de vie dans contexte de la
fonction `la_plus_longue`. Comme avec les paramètres de type génériques, nous
devons déclarer les paramètres de durée de vie génériques dans des chevrons
entre le nom de la fonction et la liste des paramètres. Nous souhaitons
contraindre toutes les références dans les paramètres de cette fonction ainsi
que sa valeur de retour aient tous la même durée de vie. Nous allons appeler la
durée de vie `'a` et ensuite l'ajouter à chaque référence, comme nous le faisons
dans l'encart 10-22.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-22/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-22: The `longest` function definition
specifying that all the references in the signature must have the same lifetime
`'a`</span>
-->

<span class="caption">Encart 10-22 : définition de la fonction `la_plus_longue`
qui indique que toutes les références présentes dans la signature doivent avoir
la même durée de vie `'a`</span>

<!--
This code should compile and produce the result we want when we use it with the
`main` function in Listing 10-20.
-->

Le code devrait se compiler et devrait produire le résultat que nous souhaitions
lorsque nous l'utilisions dans la fonction `main` de l'encart 10-20.

<!--
The function signature now tells Rust that for some lifetime `'a`, the function
takes two parameters, both of which are string slices that live at least as
long as lifetime `'a`. The function signature also tells Rust that the string
slice returned from the function will live at least as long as lifetime `'a`.
In practice, it means that the lifetime of the reference returned by the
`longest` function is the same as the smaller of the lifetimes of the
references passed in. These constraints are what we want Rust to enforce.
Remember, when we specify the lifetime parameters in this function signature,
we’re not changing the lifetimes of any values passed in or returned. Rather,
we’re specifying that the borrow checker should reject any values that don’t
adhere to these constraints. Note that the `longest` function doesn’t need to
know exactly how long `x` and `y` will live, only that some scope can be
substituted for `'a` that will satisfy this signature.
-->

La signature de la fonction indique maintenant à Rust que pour la durée de vie
`'a`, la fonction prend deux paramètres, les deux étant des slices de chaîne de
caractère qui vivent aussi longtemps que la durée de vie `'a`. La signature de
la fonction indique également à Rust que la slice de chaîne de caractère qui est
retournée par la fonction vivra au moins aussi longtemps que la durée de vie
`'a`. Dans la pratique, cela veut dire que durée de vie de la référence
retournée par la fonction `la_plus_longue` est la même que celle de la plus
petite des durées de vies des références qu'on lui donne. Ces restrictions sont
celles que nous voulons que Rust fasse respecter. Souvenez-vous, lorsque nous
précisons les paramètres de durée de vie dans la signature de cette fonction,
nous ne changons pas les durées de vies des valeurs qui lui sont envoyées ou
qu'elle retourne. Ce que nous faisons, c'est plutôt indiquer au vérificateur
d'emprunt qu'il doit rejeter toute valeur qui ne répond pas à ces conditions.
Notez que la fonction `la_plus_longue` n'a pas besoin de savoir exactement
combien de temps `x` et `y` vont exister, mais seulement que cette portée peut
être substituée par `'a`, qui satisfera cette signature.

<!--
When annotating lifetimes in functions, the annotations go in the function
signature, not in the function body. Rust can analyze the code within the
function without any help. However, when a function has references to or from
code outside that function, it becomes almost impossible for Rust to figure out
the lifetimes of the parameters or return values on its own. The lifetimes
might be different each time the function is called. This is why we need to
annotate the lifetimes manually.
-->

Lorsqu'on précise les durées de vie dans les fonctions, les annotations se
placent dans la signature de la fonction, pas dans le corps de la fonction. Rust
peut analyser le code à l'intérieur du corps sans aucune aide. Cependant,
lorsqu'une fonction a des références vers du code externe ou que ce code
réutilise une référence retournée par cette fonction, il devient presque
impossible pour Rust de déduire tout seul les durées de vie des paramètres ou
des valeurs de retour. Les durées de vies peuvent être différentes à chaque fois
que la fonction est appelée. C'est pourquoi nous avons besoin d'indiquer les
durées de vie manuellement.

<!--
When we pass concrete references to `longest`, the concrete lifetime that is
substituted for `'a` is the part of the scope of `x` that overlaps with the
scope of `y`. In other words, the generic lifetime `'a` will get the concrete
lifetime that is equal to the smaller of the lifetimes of `x` and `y`. Because
we’ve annotated the returned reference with the same lifetime parameter `'a`,
the returned reference will also be valid for the length of the smaller of the
lifetimes of `x` and `y`.
-->

Lorsque nous donnons une référence concrète à `la_plus_longue`, la durée de vie
concrète qui est modélisée par `'a` est la partie de la portée de `x` qui se
chevauche avec la portée de `y`. Autrement dit, la durée vie générique `'a` aura
la durée de vie concrète qui est égale à la plus petite des durées de vies entre
`x` et `y`. Comme nous avons marqué la référence retournée avec le même
paramètre de durée de vie `'a`, la référence retournée sera toujours en vigueur
pour la durée de la plus petite des durées de vies de `x` et de `y`.

<!--
Let’s look at how the lifetime annotations restrict the `longest` function by
passing in references that have different concrete lifetimes. Listing 10-23 is
a straightforward example.
-->

Regardons comment les annotations de durée de vie restreignent la fonction
`la_plus_longue` en y passant des références qui ont des durées de vies
concrètement différentes. L'encart 10-23 en est un exemple.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-23/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-23: Using the `longest` function with
references to `String` values that have different concrete lifetimes</span>
-->

<span class="caption">Encart 10-23 : utilisation de la fonction `la_plus_longue`
sur des références à des valeurs `String` qui ont concrètement des durées de vie
différentes</span>

<!--
In this example, `string1` is valid until the end of the outer scope, `string2`
is valid until the end of the inner scope, and `result` references something
that is valid until the end of the inner scope. Run this code, and you’ll see
that the borrow checker approves of this code; it will compile and print `The
longest string is long string is long`.
-->

Dans cet exemple, `string1` est en vigueur jusqu'à la fin de la portée externe,
`string2` n'est valide que jusqu'à la fin de la portée interne, et `resultat`
est une référence vers quelque chose qui est en vigueur jusqu'à la fin de la
portée interne. Lorsque vous lancez ce code, vous constaterez que le
vérificateur d'emprunt accepte ce code ; il va se compiler et afficher
`La chaîne la plus longue est une longue chaîne est longue`.

<!--
Next, let’s try an example that shows that the lifetime of the reference in
`result` must be the smaller lifetime of the two arguments. We’ll move the
declaration of the `result` variable outside the inner scope but leave the
assignment of the value to the `result` variable inside the scope with
`string2`. Then we’ll move the `println!` that uses `result` outside the inner
scope, after the inner scope has ended. The code in Listing 10-24 will not
compile.
-->

Maintenant, essayons un exemple qui fait en sorte que la durée de vie de la
référence dans `resultat` sera plus petite que celles des deux arguments. Nous
allons déplacer la déclaration de la variable `resultat` à l'extérieur de la
portée interne mais on va laisser l'affectation de la valeur de la variable
`resultat` à l'intérieur de la portée de `string2`. Nous allons ensuite déplacer
le `println!`, qui utilise `resultat`, à l'extérieur de la portée interne, après
que la portée soit terminée. Le code de l'encart 10-24 ne va pas se compiler.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-24: Attempting to use `result` after `string2`
has gone out of scope</span>
-->

<span class="caption">Encart 10-24 : tentative d'utilisation de `resultat` après
`string2`, qui est sortie de la portée</span>

<!--
When we try to compile this code, we’ll get this error:
-->

Lorsque nous essayons de compiler ce code, nous aurons cette erreur :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-24/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-24/output.txt}}
```

<!--
The error shows that for `result` to be valid for the `println!` statement,
`string2` would need to be valid until the end of the outer scope. Rust knows
this because we annotated the lifetimes of the function parameters and return
values using the same lifetime parameter `'a`.
-->

L'erreur explique que pour que `resultat` soit en vigueur pour l'instruction
`println!`, `string2` doit toujours être valide jusqu'à la fin de la portée
externe. Rust en a déduit cela car nous avons précisé les durées de vie des
paramètres de la fonction et des valeurs de retour en utilisant le même
paramètre de durée de vie `'a`.

<!--
As humans, we can look at this code and see that `string1` is longer than
`string2` and therefore `result` will contain a reference to `string1`.
Because `string1` has not gone out of scope yet, a reference to `string1` will
still be valid for the `println!` statement. However, the compiler can’t see
that the reference is valid in this case. We’ve told Rust that the lifetime of
the reference returned by the `longest` function is the same as the smaller of
the lifetimes of the references passed in. Therefore, the borrow checker
disallows the code in Listing 10-24 as possibly having an invalid reference.
-->

En tant qu'humain, nous pouvons lire ce code et constater que `string1` est plus
grand que `string2` et ainsi que `resultat` contiendra une référence vers
`string1`. Comme `string1` n'est pas encore sorti de portée, une référence vers
`string1` sera toujours valide pour l'instruction `println!`. Cependant, le
compilateur ne peut pas déduire que la référence est valide dans notre cas. Nous
avons dit à Rust que la durée de vie de la référence qui est retournée par la
fonction `la_plus_longue` est la même que la plus petite des durées de vie des
références qu'on lui passe en argument. C'est pourquoi le vérificateur d'emprunt
rejette le code de l'encart 10-24 car il a potentiellement une référence
invalide.

<!--
Try designing more experiments that vary the values and lifetimes of the
references passed in to the `longest` function and how the returned reference
is used. Make hypotheses about whether or not your experiments will pass the
borrow checker before you compile; then check to see if you’re right!
-->

Essayez d'expérimenter d'autres situations en variant les valeurs et durées de
vie des références passées en argument de la fonction `la_plus_longue`, et
aussi comment on utilise la référence retournée. Faites des hypothèses si ces
situations vont passer ou non le vérificateur d'emprunt avant que vous
compiliez ; et vérifiez ensuite si vous avez raison !

<!--
### Thinking in Terms of Lifetimes
-->

### Penser en termes de durées de vie

<!--
The way in which you need to specify lifetime parameters depends on what your
function is doing. For example, if we changed the implementation of the
`longest` function to always return the first parameter rather than the longest
string slice, we wouldn’t need to specify a lifetime on the `y` parameter. The
following code will compile:
-->

La façon dont vous avez à préciser les paramètres de durées de vie dépend de ce
que fait votre fonction. Par exemple, si nous changions l'implémentation de la
fonction `la_plus_longue` pour qu'elle retourne systématiquement le premier
paramètre plutôt que la slice de chaîne de caractères la plus longue, nous
n'aurions pas besoin de renseigner une durée de vie sur le paramètre `y`. Le
code suivant se compile :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-08-only-one-reference-with-lifetime/src/main.rs:here}}
```

<!--
In this example, we’ve specified a lifetime parameter `'a` for the parameter
`x` and the return type, but not for the parameter `y`, because the lifetime of
`y` does not have any relationship with the lifetime of `x` or the return value.
-->

Dans cet exemple, nous avons précisé un paramètre de durée de vie `'a` sur le
paramètre `x` et sur le type de retour, mais pas sur le paramètre `y`, car la
durée de vie de `y` n'a pas de lien avec la durée de vie de `x` ou de la valeur
de retour.

<!--
When returning a reference from a function, the lifetime parameter for the
return type needs to match the lifetime parameter for one of the parameters. If
the reference returned does *not* refer to one of the parameters, it must refer
to a value created within this function, which would be a dangling reference
because the value will go out of scope at the end of the function. Consider
this attempted implementation of the `longest` function that won’t compile:
-->

Lorsqu'on retourne une référence à partir d'une fonction, le paramètre de la
durée de vie pour le type de retour doit correspondre à une des durées des
paramètres. Si la référence retournée ne se réfère *pas* à un de ses paramètres,
elle se réfère probablement à une valeur crée à l'intérieur de cette fonction,
et elle deviendra une référence pendouillante car sa valeur va sortir de la
portée à la fin de la fonction. Imaginons cette tentative d'implémentation de
la fonction `la_plus_longue` qui ne se compile pas :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,ignore,does_not_compile
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```
-->

```rust,ignore,does_not_compile
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/src/main.rs:here}}
```

<!--
Here, even though we’ve specified a lifetime parameter `'a` for the return
type, this implementation will fail to compile because the return value
lifetime is not related to the lifetime of the parameters at all. Here is the
error message we get:
-->

Ici, même si nous avons précisé un paramètre de durée de vie `'a` sur le type de
retour, cette implémentation va échouer à la compilation car la durée de vie de
la valeur de retour n'est pas du tout liée à la durée de vie des paramètres.
Voici le message d'erreur que nous obtenons :

<!--
```console
{{#include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```
-->

```console
{{#include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-09-unrelated-lifetime/output.txt}}
```

<!--
The problem is that `result` goes out of scope and gets cleaned up at the end
of the `longest` function. We’re also trying to return a reference to `result`
from the function. There is no way we can specify lifetime parameters that
would change the dangling reference, and Rust won’t let us create a dangling
reference. In this case, the best fix would be to return an owned data type
rather than a reference so the calling function is then responsible for
cleaning up the value.
-->

Le problème est que `resultat` sort de la portée et est effacée à la fin de la
fonction `la_plus_longue`. Nous avons aussi essayé de retourner une référence
vers `resultat` à partir de la fonction. Il n'existe aucune façon d'écrire les
paramètres de durée de vie de telle manière que cela changerait la référence
pendouillante, et Rust ne nous laissera pas créer une référence pendouillante.
Dans notre cas, la meilleure solution est de retourner un type de donnée dont on
va prendre possession plutôt qu'une référence, ainsi le code appelant sera
responsable du nettoyage de la valeur.

<!--
Ultimately, lifetime syntax is about connecting the lifetimes of various
parameters and return values of functions. Once they’re connected, Rust has
enough information to allow memory-safe operations and disallow operations that
would create dangling pointers or otherwise violate memory safety.
-->

Enfin, la syntaxe de la durée de vie sert à interconnecter les durées de vie de
plusieurs paramètres ainsi que les valeurs de retour des fonctions. Une fois
interconnectés, Rust a assez d'informations pour autoriser les opérations
sécurisées dans la mémoire et refuser les opérations qui pourraient créer des
pointeurs pendouillants ou alors enfreindre la sécurité de la mémoire.

<!--
### Lifetime Annotations in Struct Definitions
-->

### L'ajout des durées de vies dans les définitions des structures

<!--
So far, we’ve only defined structs to hold owned types. It’s possible for
structs to hold references, but in that case we would need to add a lifetime
annotation on every reference in the struct’s definition. Listing 10-25 has a
struct named `ImportantExcerpt` that holds a string slice.
-->

Jusqu'à présent, nous avons défini des structures pour contenir des types qui
sont possédés par elles-mêmes. Il est possible qu'une structure puisse contenir
des références, mais dans ce cas nous devons préciser une durée de vie sur
chaque référence dans la définition de la structure. L'encart 10-25 montre une
structure `ExtraitImportant` qui stocke une slice de chaîne de caractères.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-25/src/main.rs}}
```

<!--
<span class="caption">Listing 10-25: A struct that holds a reference, so its
definition needs a lifetime annotation</span>
-->

<span class="caption">Encart 10-25 : une structure qui stocke une référence,
par conséquent sa définition a besoin d'une annotation de durée de vie</span>

<!--
This struct has one field, `part`, that holds a string slice, which is a
reference. As with generic data types, we declare the name of the generic
lifetime parameter inside angle brackets after the name of the struct so we can
use the lifetime parameter in the body of the struct definition. This
annotation means an instance of `ImportantExcerpt` can’t outlive the reference
it holds in its `part` field.
-->

Cette structure a un champ, `partie`, qui stocke une slice de chaîne de
caractères, qui est une référence. Comme pour les types de données génériques,
nous déclarons le nom du paramètre de durée de vie générique entre des chevrons
après le nom de la structure pour que nous puissions utiliser le paramètre de
durée de vie dans le corps de la définition de la structure. Cette annotation
signifie qu'une instance de `ExtraitImportant` ne peut pas vivre plus longtemps
que la référence qu'elle stocke dans son champ `partie`.

<!--
The `main` function here creates an instance of the `ImportantExcerpt` struct
that holds a reference to the first sentence of the `String` owned by the
variable `novel`. The data in `novel` exists before the `ImportantExcerpt`
instance is created. In addition, `novel` doesn’t go out of scope until after
the `ImportantExcerpt` goes out of scope, so the reference in the
`ImportantExcerpt` instance is valid.
-->

La fonction `main` crée ici une instance de la structure `ExtraitImportant` qui
stocke une référence vers la première phrase de la `String` possédée par la
variable `roman`. Les données dans `roman` existent avant que l'instance de
`ExtraitImportant` soit crée. De plus, `roman` ne sort pas de la portée avant
que l'instance de `ExtraitImportant` sorte de la portée, donc la référence dans
l'instance de `ExtraitImportant` est toujours valide.

<!--
### Lifetime Elision
-->

### L'élision des durées de vie

<!--
You’ve learned that every reference has a lifetime and that you need to specify
lifetime parameters for functions or structs that use references. However, in
Chapter 4 we had a function in Listing 4-9, which is shown again in Listing
10-26, that compiled without lifetime annotations.
-->

Vous avez appris que toute référence a une durée de vie et que vous devez
renseigner des paramètres de durée de vie sur des fonctions ou des structures
qui utilisent des références. Cependant, dans le chapitre 4 nous avions une
fonction dans l'encart 4-9, qui est montrée à nouveau dans l'encart 10-26, qui
compilait sans informations de durée de vie.

<!--
<span class="filename">Filename: src/lib.rs</span>
-->

<span class="filename">Fichier : src/lib.rs</span>

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/listing-10-26/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/listing-10-26/src/main.rs:here}}
```

<!--
<span class="caption">Listing 10-26: A function we defined in Listing 4-9 that
compiled without lifetime annotations, even though the parameter and return
type are references</span>
-->

<span class="caption">Encart 10-26 : une fonction que nous avons défini dans
l'encart 4-9 qui se compilait sans avoir d'indications sur la durée de vie, même
si les paramètres et le type de retour sont des références</span>

<!--
The reason this function compiles without lifetime annotations is historical:
in early versions (pre-1.0) of Rust, this code wouldn’t have compiled because
every reference needed an explicit lifetime. At that time, the function
signature would have been written like this:
-->

La raison pour laquelle cette fonction se compile sans annotation de durée de
vie est historique : dans les premières versions de Rust (avant la 1.0), ce code
ne se serait pas compilé parce que chaque référence devait avoir une durée de
vie explicite. A l'époque, la signature de la fonction devait être écrite
ainsi :

<!--
```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```
-->

```rust,ignore
fn premier_mot<'a>(s: &'a str) -> &'a str {
```

<!--
After writing a lot of Rust code, the Rust team found that Rust programmers
were entering the same lifetime annotations over and over in particular
situations. These situations were predictable and followed a few deterministic
patterns. The developers programmed these patterns into the compiler’s code so
the borrow checker could infer the lifetimes in these situations and wouldn’t
need explicit annotations.
-->

Après avoir écrit une grande quantité de code Rust, l'équipe de Rust s'est rendu
compte que les développeurs Rust saisissaient toujours les mêmes durées de vie
encore et encore dans des situations spécifiques. Ces situations étaient
prévisibles et suivaient des schémas prédéterminés. Les développeurs ont
programmé ces schémas dans le code du compilateur afin que le vérificateur
d'emprunt puisse deviner les durées de vie dans ces situations et n'auront plus
besoin d'annotations explicites.

<!--
This piece of Rust history is relevant because it’s possible that more
deterministic patterns will emerge and be added to the compiler. In the future,
even fewer lifetime annotations might be required.
-->

Cette partie de l'histoire de Rust est intéressante car il est possible que
d'autres modèles prédéterminés émergent et soient ajoutés au compilateur. A
l'avenir, il est possible qu'encore moins d'annotations de durée de vie soient
nécessaires.

<!--
The patterns programmed into Rust’s analysis of references are called the
*lifetime elision rules*. These aren’t rules for programmers to follow; they’re
a set of particular cases that the compiler will consider, and if your code
fits these cases, you don’t need to write the lifetimes explicitly.
-->

Les schémas programmés dans l'analyse des références de Rust s'appellent les
*règles d'élision des durées de vie*. Ce ne sont pas des règles que les
développeurs doivent suivre ; c'est un jeu de cas particuliers que le
compilateur va essayer de comparer à votre code, et s'il y a une correspondance
alors vous n'aurez pas besoin d'écrire explicitement les durées de vie.

<!--
The elision rules don’t provide full inference. If Rust deterministically
applies the rules but there is still ambiguity as to what lifetimes the
references have, the compiler won’t guess what the lifetime of the remaining
references should be. In this case, instead of guessing, the compiler will give
you an error that you can resolve by adding the lifetime annotations that
specify how the references relate to each other.
-->

Les règles d'élision ne permettent pas de faire des déductions complètes. Si
Rust applique les règles de façon stricte, mais qu'il existe toujours une
ambiguïté quant à la durée de vie des références, le compilateur ne devinera pas
quel devrait être la durée de vie des autres références. Dans ce cas, au lieu de
tenter de deviner, le compilateur va vous afficher une erreur que vous devrez
résoudre en précisant les durées de vie qui clarifieront les liens entre chaque
référence.

<!--
Lifetimes on function or method parameters are called *input lifetimes*, and
lifetimes on return values are called *output lifetimes*.
-->

Les durées de vies sur les fonctions ou les paramètres des fonctions sont
appelées les *durées de vie des entrées*, et les durées de vie sur les valeurs
de retour sont appelées les *durées de vie des sorties*.

<!--
The compiler uses three rules to figure out what lifetimes references have when
there aren’t explicit annotations. The first rule applies to input lifetimes,
and the second and third rules apply to output lifetimes. If the compiler gets
to the end of the three rules and there are still references for which it can’t
figure out lifetimes, the compiler will stop with an error. These rules apply
to `fn` definitions as well as `impl` blocks.
-->

Le compilateur utilise trois règles pour déterminer quelles seraient les durées
de vie des références si cela n'est pas indiqué explicitement. La première règle
s'applique sur les durées de vie des entrées, et la seconde et troisième règle
s'appliquent sur les durées de vie des sorties. Si le compilateur arrive à la
fin des trois règles et qu'il y a encore des références pour lesquelles il ne
peut pas savoir leur durée de vie, le compilateur s'arrête avec une erreur. Ces
règles s'appliquent sur les définitions des `fn` ainsi que sur celles des blocs
`impl`.

<!--
The first rule is that each parameter that is a reference gets its own lifetime
parameter. In other words, a function with one parameter gets one lifetime
parameter: `fn foo<'a>(x: &'a i32)`; a function with two parameters gets two
separate lifetime parameters: `fn foo<'a, 'b>(x: &'a i32, y: &'b i32)`; and so
on.
-->

La première règle dit que chaque paramètre qui est une référence a sa propre
durée de vie. Autrement dit, une fonction avec un seul paramètre va avoir un
seul paramètre de durée de vie : `fn foo<'a>(x: &'a i32)` ; une fonction avec
deux paramètres va avoir deux paramètres de durée de vie séparées :
`fn foo<'a, 'b>(x: &'a i32, y: &'b i32)` ; et ainsi de suite.

<!--
The second rule is if there is exactly one input lifetime parameter, that
lifetime is assigned to all output lifetime parameters: `fn foo<'a>(x: &'a i32)
-> &'a i32`.
-->

La seconde règle dit que s'il y a exactement un seul paramètre de durée de vie
d'entrée, cette durée de vie est assignée à tous les paramètres de durée de vie
des sorties : `fn foo<'a>(x: &'a i32) -> &'a i32`.

<!--
The third rule is if there are multiple input lifetime parameters, but one of
them is `&self` or `&mut self` because this is a method, the lifetime of `self`
is assigned to all output lifetime parameters. This third rule makes methods
much nicer to read and write because fewer symbols are necessary.
-->

La troisième règle est que lorsque nous avons plusieurs paramètres de durée de
vie, mais qu'un d'entre eux est `&self` ou `&mut self` parce que c'est une
méthode, la durée de vie de `self` sera associée à tous les paramètres de durée
de vie des sorties. Cette troisième règle rend les méthodes plus faciles à lire
et à écrire car il y a moins de caractères nécessaires.

<!--
Let’s pretend we’re the compiler. We’ll apply these rules to figure out what
the lifetimes of the references in the signature of the `first_word` function
in Listing 10-26 are. The signature starts without any lifetimes associated
with the references:
-->

Imaginons que nous soyons le compilateur. Nous allons appliquer ces règles pour
déduire quelles seront les durées de vie des références dans la signature de la
fonction `premier_mot` de l'encart 10-26.

<!--
```rust,ignore
fn first_word(s: &str) -> &str {
```
-->

```rust,ignore
fn premier_mot(s: &str) -> &str {
```

<!--
Then the compiler applies the first rule, which specifies that each parameter
gets its own lifetime. We’ll call it `'a` as usual, so now the signature is
this:
-->

Ensuite, le compilateur applique la première règle, qui dit que chaque référence
a sa propre durée de vie. Appellons-la `'a` comme d'habitude, donc maintenant la
signature devient ceci :

<!--
```rust,ignore
fn first_word<'a>(s: &'a str) -> &str {
```
-->

```rust,ignore
fn premier_mot<'a>(s: &'a str) -> &str {
```

<!--
The second rule applies because there is exactly one input lifetime. The second
rule specifies that the lifetime of the one input parameter gets assigned to
the output lifetime, so the signature is now this:
-->

La seconde règle s'applique car il y a exactement une durée de vie d'entrée ici.
La seconde règle dit que la durée de vie du seul paramètre d'entrée est affectée
à la durée de vie des sorties, donc la signature est maintenant ceci :

<!--
```rust,ignore
fn first_word<'a>(s: &'a str) -> &'a str {
```
-->

```rust,ignore
fn premier_mot<'a>(s: &'a str) -> &'a str {
```

<!--
Now all the references in this function signature have lifetimes, and the
compiler can continue its analysis without needing the programmer to annotate
the lifetimes in this function signature.
-->

Maintenant, toutes les références de cette signature de fonction ont des
durées de vie, et le compilateur peut continuer son analyse sans avoir besoin
que le développeur renseigne les durées de vie dans les signatures de ces
fonctions.

<!--
Let’s look at another example, this time using the `longest` function that had
no lifetime parameters when we started working with it in Listing 10-21:
-->

Voyons un autre exemple, qui utilise cette fois la fonction `la_plus_longue` qui
n'avait pas de paramètres de durée de vie lorsque nous avons commencé à
l'utiliser dans l'encart 10-21 :

<!--
```rust,ignore
fn longest(x: &str, y: &str) -> &str {
```
-->

```rust,ignore
fn la_plus_longue(x: &str, y: &str) -> &str {
```

<!--
Let’s apply the first rule: each parameter gets its own lifetime. This time we
have two parameters instead of one, so we have two lifetimes:
-->

Appliquons la première règle : chaque référence a sa propre durée de vie. Cette
fois, nous avons avons deux références au lieu d'une seule, donc nous avons deux
durées de vie :

<!--
```rust,ignore
fn longest<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```
-->

```rust,ignore
fn la_plus_longue<'a, 'b>(x: &'a str, y: &'b str) -> &str {
```

<!--
You can see that the second rule doesn’t apply because there is more than one
input lifetime. The third rule doesn’t apply either, because `longest` is a
function rather than a method, so none of the parameters are `self`. After
working through all three rules, we still haven’t figured out what the return
type’s lifetime is. This is why we got an error trying to compile the code in
Listing 10-21: the compiler worked through the lifetime elision rules but still
couldn’t figure out all the lifetimes of the references in the signature.
-->

Vous pouvez constater que la seconde règle ne s'applique pas car il y a plus
qu'une seule durée de vie. La troisième ne s'applique pas non plus, car
`la_plus_longue` est une fonction et non pas une méthode, donc aucun de ses
paramètres ne sont `self`. Après avoir utilisé ces trois règles, nous n'avons
pas pu en déduire la durée de vie de la valeur de retour. C'est pourquoi nous
obtenons une erreur en essayant de compiler le code dans l'encart 10-21 : le
compilateur a utilisé les règles d'élision des durées de vie mais n'est pas
capable d'en déduire toutes les durées de vie des références présentes dans la
signature.

<!--
Because the third rule really only applies in method signatures, we’ll look at
lifetimes in that context next to see why the third rule means we don’t have to
annotate lifetimes in method signatures very often.
-->

Comme la troisième règle ne s'applique que sur les signatures des méthodes, nous
allons examiner les durées de vie dans ce contexte pour comprendre pourquoi la
troisième règle signifie que nous n'avons pas souvent besoin d'annoter les
durées de vie dans les signatures des méthodes.

<!--
### Lifetime Annotations in Method Definitions
-->

### Informations de durée de vie dans les définitions des méthodes

<!--
When we implement methods on a struct with lifetimes, we use the same syntax as
that of generic type parameters shown in Listing 10-11. Where we declare and
use the lifetime parameters depends on whether they’re related to the struct
fields or the method parameters and return values.
-->

Lorsque nous implémentons des méthodes sur une structure avec des durées de vie,
nous utilisons la même syntaxe que les paramètres de type génériques que nous
avons vu dans l'encart 10-11. L'endroit où nous déclarons et utilisons les
paramètres de durée de vie dépend de s'ils sont reliés aux champs des structures
ou aux paramètres de la méthode et les valeurs de retour.

<!--
Lifetime names for struct fields always need to be declared after the `impl`
keyword and then used after the struct’s name, because those lifetimes are part
of the struct’s type.
-->

Les noms des durées de vie pour les champs de structure ont toujours besoin
d'être déclarés après le mot-clé `impl` et sont ensuite utilisés après le nom de
la structure, car ces durées vie font partie du type de la structure.

<!--
In method signatures inside the `impl` block, references might be tied to the
lifetime of references in the struct’s fields, or they might be independent. In
addition, the lifetime elision rules often make it so that lifetime annotations
aren’t necessary in method signatures. Let’s look at some examples using the
struct named `ImportantExcerpt` that we defined in Listing 10-25.
-->

Sur les signatures des méthodes à l'intérieur du bloc `impl`, les références
peuvent avoir la durée de vie des références des champs de la structure, ou
elles peuvent être indépendantes. De plus, les règles d'élision des durées de
vie le font parfois, ce qui fait que l'ajout des durées de vie n'est parfois pas
nécessaire dans les signatures des méthodes. Voyons quelques exemples en
utilisant la structure `ExtraitImportant` que nous avons défini dans l'encart
10-25.

<!--
First, we’ll use a method named `level` whose only parameter is a reference to
`self` and whose return value is an `i32`, which is not a reference to anything:
-->

Premièrement, nous allons utiliser une méthode `niveau` dont le seul paramètre
est une référence à `self` et dont la valeur de retour sera un `i32`, qui n'est
pas une référence :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:1st}}
```

<!--
The lifetime parameter declaration after `impl` and its use after the type name
are required, but we’re not required to annotate the lifetime of the reference
to `self` because of the first elision rule.
-->

La déclaration du paramètre de durée de vie après `impl` et son utilisation
après le nom du type sont nécessaires, mais nous n'avons pas à préciser la durée
de vie de la référence à `self` grâce à la première règle d'élision.

<!--
Here is an example where the third lifetime elision rule applies:
-->

Voici un exemple où la troisième règle d'élision des durées de vie s'applique :

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-10-lifetimes-on-methods/src/main.rs:3rd}}
```

<!--
There are two input lifetimes, so Rust applies the first lifetime elision rule
and gives both `&self` and `announcement` their own lifetimes. Then, because
one of the parameters is `&self`, the return type gets the lifetime of `&self`,
and all lifetimes have been accounted for.
-->

Il y a deux durées de vies des entrées, donc Rust applique la première règle
d'élision des durées de vie et donne à chacun de `&self` et `annonce` leur
propre durée de vie. Ensuite, comme un des paramètres est `&self`, le type de
retour obtient la durée de vie de `&self`, et ainsi toutes les durées de vie ont
été calculées.

<!--
### The Static Lifetime
-->

### La durée de vie statique

<!--
One special lifetime we need to discuss is `'static`, which means that this
reference *can* live for the entire duration of the program. All string
literals have the `'static` lifetime, which we can annotate as follows:
-->

Une durée de vie particulière que nous devons aborder est `'static`, qui
signifie que cette référence *peut* vivre pendant la totalité de la durée du
programme. Tous les littéraux de chaînes de caractères ont la durée de vie
`'static`, que nous pouvons écrire comme ceci :

<!--
```rust
let s: &'static str = "I have a static lifetime.";
```
-->

```rust
let s: &'static str = "J'ai une durée de vie statique.";
```

<!--
The text of this string is stored directly in the program’s binary, which
is always available. Therefore, the lifetime of all string literals is
`'static`.
-->

Le texte de cette chaîne de caractères est stocké directement dans le binaire du
programme, qui est toujours disponible. C'est pourquoi la durée de vie de tous
les littéraux de chaînes de caractères est `'static`.

<!--
You might see suggestions to use the `'static` lifetime in error messages. But
before specifying `'static` as the lifetime for a reference, think about
whether the reference you have actually lives the entire lifetime of your
program or not. You might consider whether you want it to live that long, even
if it could. Most of the time, the problem results from attempting to create a
dangling reference or a mismatch of the available lifetimes. In such cases, the
solution is fixing those problems, not specifying the `'static` lifetime.
-->

Il se peut que voyez des suggestions pour utiliser la durée de vie `'static`
dans les messages d'erreur. Mais avant d'utiliser `'static` comme durée de vie
pour une référence, demandez-vous si la référence en question vit bien pendant
toute la vie de votre programme, ou non. Vous devriez vous demander si vous
voulez qu'elle vive aussi longtemps, même si si c'était possible. La plupart du
temps, le problème résulte d'une tentative de création d'une référence
pendouillante ou d'une inadéquation des durées de vie disponibles. Dans de ces
cas-là, la solution est de résoudre ces problèmes, et non pas de renseigner la
durée de vie comme étant `'static`.

<!--
## Generic Type Parameters, Trait Bounds, and Lifetimes Together
-->

## Les paramètres de type génériques, les traits liés, et les durées de vies ensemble

<!--
Let’s briefly look at the syntax of specifying generic type parameters, trait
bounds, and lifetimes all in one function!
-->

Regardons brièvement la syntaxe pour renseigner tous les paramètres de type
génériques, les traits liés, et les durées de vies sur une seule fonction !

<!--
```rust
{{#rustdoc_include ../listings-sources/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```
-->

```rust
{{#rustdoc_include ../listings/ch10-generic-types-traits-and-lifetimes/no-listing-11-generics-traits-and-lifetimes/src/main.rs:here}}
```

<!--
This is the `longest` function from Listing 10-22 that returns the longer of
two string slices. But now it has an extra parameter named `ann` of the generic
type `T`, which can be filled in by any type that implements the `Display`
trait as specified by the `where` clause. This extra parameter will be printed
before the function compares the lengths of the string slices, which is why the
`Display` trait bound is necessary. Because lifetimes are a type of generic,
the declarations of the lifetime parameter `'a` and the generic type parameter
`T` go in the same list inside the angle brackets after the function name.
-->

C'est la fonction `la_plus_longue` de l'encart 10-22 qui retourne la plus grande
de deux slices de chaînes de caractères. Mais maintenant elle a un paramètre
supplémentaire `ann` de type générique `T`, qui peut être remplacé par n'importe
quel type qui implémente le trait `Display` comme le précise la clause `where`.
Ce paramètre supplémentaire sera affiché avant que la fonction compare les
longueurs des slices de chaînes de caractères, c'est pourquoi le trait lié
`Display` est nécessaire. Comme les durées de vie sont un type de génériques,
les déclarations du paramètre de durée de vie `'a` et le paramètre de type
générique `T` vont dans la même liste à l'intérieur des chevrons après le nom de
la fonction.

<!--
## Summary
-->

## Résumé

<!--
We covered a lot in this chapter! Now that you know about generic type
parameters, traits and trait bounds, and generic lifetime parameters, you’re
ready to write code without repetition that works in many different situations.
Generic type parameters let you apply the code to different types. Traits and
trait bounds ensure that even though the types are generic, they’ll have the
behavior the code needs. You learned how to use lifetime annotations to ensure
that this flexible code won’t have any dangling references. And all of this
analysis happens at compile time, which doesn’t affect runtime performance!
-->

Nous avons vu beaucoup de choses dans ce chapitre ! Maintenant que vous en savez
plus sur les paramètres de type génériques, les traits et les traits liés, et
les paramètres de durée de vie génériques, vous pouvez maintenant écrire du code
en évitant les doublons qui va bien fonctionner dans de nombreuses situations.
Les paramètres de type génériques vous permet d'appliquer du code à différents
types. Les traits et les traits liés s'assurent que bien que les types soient
génériques, ils auront un comportement particulier sur lequel le code peut
compter. Vous avez appris comment utiliser les indications de durée de vie pour
s'assurer que ce code flexible n'aura pas de références pendouillantes. Et
toutes ces vérifications se font au moment de la compilation, ce qui n'influe
pas sur les performances au moment de l'exécution du programme !

<!--
Believe it or not, there is much more to learn on the topics we discussed in
this chapter: Chapter 17 discusses trait objects, which are another way to use
traits. There are also more complex scenarios involving lifetime annotations
that you will only need in very advanced scenarios; for those, you should read
the [Rust Reference][reference]. But next, you’ll learn how to write tests in
Rust so you can make sure your code is working the way it should.
-->

Croyez-le ou non, mais il y a encore des choses à apprendre sur les sujets que
nous avons traités dans ce chapitre : le chapitre 17 expliquera les objets de
trait, qui est une façon d'utiliser les traits. Il existe aussi des scénarios
plus complexes qui nécessitent des indications de durée de vie ainsi que
d'utiliser, uniquement pour ces ces scénarios avancés, certaines fonctionnalités
avancées du système de type ; pour ces cas-là, vous devriez consulter la
[Référence de Rust][reference]. Maintenant, nous allons voir au chapitre suivant
comment écrire des tests en Rust afin que vous puissiez vous assurer que votre
code fonctionne comme il devrait le faire.

<!--
[references-and-borrowing]:
ch04-02-references-and-borrowing.html#references-and-borrowing
[string-slices-as-parameters]:
ch04-03-slices.html#string-slices-as-parameters
[reference]: ../reference/index.html
-->

[references-and-borrowing]:
ch04-02-references-and-borrowing.html#les-références-et-lemprunt
[string-slices-as-parameters]:
ch04-03-slices.html#les-slices-de-chaînes-de-caractères-en-paramètres
[reference]: https://doc.rust-lang.org/reference/index.html
