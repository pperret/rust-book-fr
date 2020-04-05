> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/99).

<!--
## Recoverable Errors with `Result`
-->

## Des erreurs récupérables avec `Result`

<!--
Most errors aren’t serious enough to require the program to stop entirely.
Sometimes, when a function fails, it’s for a reason that you can easily
interpret and respond to. For example, if you try to open a file and that
operation fails because the file doesn’t exist, you might want to create the
file instead of terminating the process.
-->

La plupart des erreurs ne sont pas assez graves au point d'arrêter complètement
le programme. Parfois, lorsque une fonction échoue, c'est pour une raison que
vous pouvez facilement comprendre et agir en conséquence. Par exemple, si vous
essayez d'ouvrir un fichier et que l'opération échoue parce que le fichier
n'existe pas, vous pourriez vouloir créer le fichier plutôt que d'arrêter le
processus.

<!--
Recall from [“Handling Potential Failure with the `Result`
Type”][handle_failure]<!-- ignore -- > in Chapter 2 that the `Result` enum is
defined as having two variants, `Ok` and `Err`, as follows:
-->

Souvenez-vous de la section
“[Gérer les erreurs potentielles avec le type `Result`][handle_failure]<!-- ignore -->”
du chapitre 2 lorsque l'énumération `Result` pouvait avoir deux variantes, `Ok`
et `Err`, comme ci-dessous :

<!--
[handle_failure]: ch02-00-guessing-game-tutorial.html#handling-potential-failure-with-the-result-type
-->

[handle_failure]: ch02-00-guessing-game-tutorial.html#gérer-les-erreurs-potentielles-avec-le-type-result

```rust
enum Result<T, E> {
    Ok(T),
    Err(E),
}
```

<!--
The `T` and `E` are generic type parameters: we’ll discuss generics in more
detail in Chapter 10. What you need to know right now is that `T` represents
the type of the value that will be returned in a success case within the `Ok`
variant, and `E` represents the type of the error that will be returned in a
failure case within the `Err` variant. Because `Result` has these generic type
parameters, we can use the `Result` type and the functions that the standard
library has defined on it in many different situations where the successful
value and error value we want to return may differ.
-->

Le `T` et `E` sont des paramètres de type génériques : nous allons parler plus
en détail des génériques au chapitre 10. Tout ce que vous avez besoin de savoir
pour le moment, c'est que `T` représente le type de valeur imbriquée dans la
variante `Ok` qui sera retournée dans le cas d'un succès, et `E` représente le
type d'erreur imbriquée dans la variante `Err` qui sera retournée dans le cas
d'un échec. Comme `Result` a ces types de paramètres génériques, nous pouvons
utiliser le type `Result` et les fonctions que la bibliothèque standard qui lui
ont été associées pour différentes situations où la valeur de succès et la
valeur d'erreur peuvent être différentes.

<!--
Let’s call a function that returns a `Result` value because the function could
fail. In Listing 9-3 we try to open a file.
-->

Utilisons une fonction qui retourne une valeur de type `Result` car la fonction
peut échouer. Dans l'encart 9-3 nous essayons d'ouvrir un fichier :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");
}
```

<!--
<span class="caption">Listing 9-3: Opening a file</span>
-->

<span class="caption">Encart 9-3 : Ouverture d'un fichier</span>

<!--
How do we know `File::open` returns a `Result`? We could look at the [standard
library API documentation](../std/index.html)<!-- ignore -- >, or we could ask
the compiler! If we give `f` a type annotation that we know is *not* the return
type of the function and then try to compile the code, the compiler will tell
us that the types don’t match. The error message will then tell us what the
type of `f` *is*. Let’s try it! We know that the return type of `File::open`
isn’t of type `u32`, so let’s change the `let f` statement to this:
-->

Comment savons-nous que `File::open` retourne un `Result` ? Nous pouvons
regarder la
[documentation de l'API de la bibliothèque standard](https://doc.rust-lang.org/std/index.html)<!-- ignore -->,
ou nous pouvons demander au compilateur ! Si nous faisons une annotation de type
à `f` dont nous savons que le type de retour de la fonction n'est *pas* correcte
et que nous essayons ensuite de compiler le code, le compilateur va nous dire
que les types ne correspondent pas. Le message d'erreur va ensuite nous dire
ensuite *quel est le type* de `f`. Essayons cela ! Nous savons que le retour de
`File::open` n'est pas du type `u32`, alors essayons de changer l'instruction
`let f` par ceci :

```rust,ignore,does_not_compile
let f: u32 = File::open("hello.txt");
```

<!--
Attempting to compile now gives us the following output:
-->

La compilation nous donne maintenant le résultat suivant :

```text
error[E0308]: mismatched types
 -- > src/main.rs:4:18
  |
4 |     let f: u32 = File::open("hello.txt");
  |                  ^^^^^^^^^^^^^^^^^^^^^^^ expected u32, found enum
`std::result::Result`
  |
  = note: expected type `u32`
             found type `std::result::Result<std::fs::File, std::io::Error>`
```

<!--
This tells us the return type of the `File::open` function is a `Result<T, E>`.
The generic parameter `T` has been filled in here with the type of the success
value, `std::fs::File`, which is a file handle. The type of `E` used in the
error value is `std::io::Error`.
-->

Cela nous dit que le retour de la fonction `File::open` est du type
`Result<T, E>`. Le paramètre générique `T` a été remplacé dans ce cas par le
type en cas de succès, `std::fs::File`, qui permet d'interagir avec le fichier.
Le `E` utilisé pour la valeur d'erreur est du type `std::io::Error`.

<!--
This return type means the call to `File::open` might succeed and return a file
handle that we can read from or write to. The function call also might fail:
for example, the file might not exist, or we might not have permission to
access the file. The `File::open` function needs to have a way to tell us
whether it succeeded or failed and at the same time give us either the file
handle or error information. This information is exactly what the `Result` enum
conveys.
-->

Ce type de retour veut dire que l'appel à `File::open` peut réussir et nous
retourner un manipulateur de fichier qui peut nous permettre de le lire ou
l'écrire. L'utilisation de cette fonction peut aussi échouer : par exemple, le
fichier peut ne pas exister, ou nous n'avons pas le droit d'accéder au fichier.
La fonction `File::open` doit avoir un moyen de nous dire si son utilisation a
réussi ou échoué et en même temps nous fournir soit le manipulateur de fichier
soit des informations sur l'erreur. C'est exactement ces informations que
l'énumération `Result` se charge de nous transmettre.

<!--
In the case where `File::open` succeeds, the value in the variable `f` will be
an instance of `Ok` that contains a file handle. In the case where it fails,
the value in `f` will be an instance of `Err` that contains more information
about the kind of error that happened.
-->

Dans le cas où `File::open` réussit, la valeur que nous obtenons dans la
variable `f` sera une instance de `Ok` qui contiendra un manipulateur de
fichier. Dans le cas où cela échoue, la valeur dans `f` sera une instance de
`Err` qui contiendra plus d'information sur le type d'erreur qui a eu lieu.

<!--
We need to add to the code in Listing 9-3 to take different actions depending
on the value `File::open` returns. Listing 9-4 shows one way to handle the
`Result` using a basic tool, the `match` expression that we discussed in
Chapter 6.
-->

Nous avons besoin d'ajouter différentes actions dans le code de l'encart 9-3 en
fonction de la valeur que `File::open` a retourné. L'encart 9-4 montre une façon
de gérer `Result` en utilisant un outil basique, l'expression `match` que nous
avons vu au chapitre 6.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => {
            panic!("Problem opening the file: {:?}", error)
        },
    };
}
```
-->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(fichier) => fichier,
        Err(erreur) => {
            panic!("Erreur lors de l'ouverture du fichier : {:?}", erreur)
        },
    };
}
```

<!--
<span class="caption">Listing 9-4: Using a `match` expression to handle the
`Result` variants that might be returned</span>
-->

<span class="caption">Encart 9-4: Utilisation de l'expression `match` pour
gérer les variantes que `Result` pourrait retourner</span>

<!--
Note that, like the `Option` enum, the `Result` enum and its variants have been
brought into scope by the prelude, so we don’t need to specify `Result::`
before the `Ok` and `Err` variants in the `match` arms.
-->

Remarquez que comme l'énumération `Option`, l'énumération `Result` et ses
variantes ont été importés par l'étape préliminaire, donc vous n'avez pas
besoin de préciser `Result::` devant les variantes `Ok` et `Err` dans les
branches du `match`.

<!--
Here we tell Rust that when the result is `Ok`, return the inner `file` value
out of the `Ok` variant, and we then assign that file handle value to the
variable `f`. After the `match`, we can use the file handle for reading or
writing.
-->

Ici nous indiquons à Rust que quand le résultat est `Ok`, il faut sortir la
valeur `fichier` de la variante `Ok`, et nous assignons ensuite cette valeur à
la variable `f`. Après le `match`, nous pourrons ensuite utiliser le
manipulateur de fichier pour lire ou écrire.

<!--
The other arm of the `match` handles the case where we get an `Err` value from
`File::open`. In this example, we’ve chosen to call the `panic!` macro. If
there’s no file named *hello.txt* in our current directory and we run this
code, we’ll see the following output from the `panic!` macro:
-->

L'autre branche du bloc `match` gère le cas où nous obtenons un `Err` à l'appel
de `File::open`. Dans cet exemple, nous avons choisi de faire appel à la macro
`panic!`. S'il n'y a pas de fichier qui s'appelle *hello.txt* dans notre
répertoire actuel et que nous exécutons ce code, nous allons voir la sortie
suivante suite à l'appel de la macro `panic!` :

<!--
```text
thread 'main' panicked at 'Problem opening the file: Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```
-->

```text
thread 'main' panicked at 'Erreur lors de l'ouverture du fichier : Error { repr:
Os { code: 2, message: "No such file or directory" } }', src/main.rs:9:12
```

<!--
As usual, this output tells us exactly what has gone wrong.
-->

Comme d'habitude, cette sortie nous explique avec précision ce qui s'est mal
passé.

<!--
### Matching on Different Errors
-->

### Tester les différentes erreurs

<!--
The code in Listing 9-4 will `panic!` no matter why `File::open` failed. What
we want to do instead is take different actions for different failure reasons:
if `File::open` failed because the file doesn’t exist, we want to create the
file and return the handle to the new file. If `File::open` failed for any
other reason—for example, because we didn’t have permission to open the file—we
still want the code to `panic!` in the same way as it did in Listing 9-4. Look
at Listing 9-5, which adds an inner `match` expression.
-->

Le code dans l'encart 9-4 va faire un `panic!` peu importe la raison de l'échec
de `File::open`. Ce que nous voudrions plutôt faire est de régir différemment en
fonction de différents cas d'erreurs : si `File::open` a échoué parce que le
fichier n'existe pas, nous voulons créer le fichier et renvoyer le manipulateur
de fichier pour ce nouveau fichier. Si `File::open` échoue pour toute autre
raison, par exemple si nous n'avons pas l'autorisation d'ouvrir le fichier,
nous voulons quand même que le code lance un `panic!` de la même manière qu'il
l'a fait dans l'encart 9-4. Dans l'encart 9-5, nous avons ajouté un nouveau cas
au bloc `match` :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!-- ignore this test because otherwise it creates hello.txt which causes other
tests to fail lol -->

<!--
```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(file) => file,
        Err(error) => match error.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            },
            other_error => panic!("Problem opening the file: {:?}", other_error),
        },
    };
}
```
-->

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt");

    let f = match f {
        Ok(fichier) => fichier,
        Err(erreur) => match erreur.kind() {
            ErrorKind::NotFound => match File::create("hello.txt") {
                Ok(fc) => fc,
                Err(e) => panic!("Erreur lors de la création du fichier : {:?}", e),
            },
            autre_erreur => panic!("Erreur lors de l'ouverture du fichier : {:?}", autre_erreur),
        },
    };
}
```

<!--
<span class="caption">Listing 9-5: Handling different kinds of errors in
different ways</span>
-->

<span class="caption">Encart 9-5 : Gestion des différents cas d'erreurs avec des
actions différentes.</span>

<!--
The type of the value that `File::open` returns inside the `Err` variant is
`io::Error`, which is a struct provided by the standard library. This struct
has a method `kind` that we can call to get an `io::ErrorKind` value. The enum
`io::ErrorKind` is provided by the standard library and has variants
representing the different kinds of errors that might result from an `io`
operation. The variant we want to use is `ErrorKind::NotFound`, which indicates
the file we’re trying to open doesn’t exist yet. So we match on `f`, but we
also have an inner match on `error.kind()`.
-->

La valeur de retour de `File::open` logée dans la variante `Err` est de type
`io::Error`, qui est une structure fournie par la bibliothèque standard. Cette
structure a une méthode `kind` que nous pouvons utiliser pour obtenir un retour
de type `io::ErrorKind`. L'énumération `io::ErrorKind` est fournie elle aussi
par la bibliothèque standard qui embarque des variantes qui représentent
différents types d'erreurs qui pourraient résulter d'une opération provenant du
module `io`. La variante que nous voulons utiliser est `ErrorKind::NotFound`,
qui nous informe que le fichier que nous essayons d'ouvrir n'existe pas encore.
Donc nous utilisons `match` sur `f`, mais nous avons dans celle-ci une autre
`match` sur `erreur.kind()`.

<!--
The condition we want to check in the inner match is whether the value returned
by `error.kind()` is the `NotFound` variant of the `ErrorKind` enum. If it is,
we try to create the file with `File::create`. However, because `File::create`
could also fail, we need a second arm in the inner `match` expression. When the
file can’t be created, a different error message is printed. The second arm of
the outer `match` stays the same, so the program panics on any error besides
the missing file error.
-->

Nous souhaitons vérifier dans le `match` interne si la valeur de retour de
`error.kind()` est la variante `NotFound` de l'énumération `ErrorKind`. Si c'est
le cas, nous essayons de créer le fichier avec `File::create`. Cependant, comme
`File::create` peut aussi échouer, nous avons besoin d'une seconde branche dans
le `match` à l'intérieur. Lorsque le fichier ne peut pas être créé, un message
d'erreur différent est affiché. La seconde branche du `match` principal reste
inchangé, donc le programme panique lorsqu'on rencontre une autre erreur que
l'absence de fichier.

<!--
That’s a lot of `match`! The `match` expression is very useful but also very
much a primitive. In Chapter 13, you’ll learn about closures; the `Result<T,
E>` type has many methods that accept a closure and are implemented using
`match` expressions. Using those methods will make your code more concise. A
more seasoned Rustacean might write this code instead of Listing 9-5:
-->

Cela commence à faire beaucoup de `match` ! L'expression `match` est très utile
mais est aussi assez primitif. Dans le chapitre 13, vous allez en apprendre plus
sur les fermetures ; le type `Result<T, E>` a de nombreuses méthodes qui
acceptent une fermeture et qui sont implémentés en utilisant des expressions
`match`. L'utilisation de ces méthodes vont rendre votre code plus concis. Un
Rustacé plus habitué écrira ce code plutôt que celui de l'encart 9-5 :

<!--
```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|error| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|error| {
                panic!("Problem creating the file: {:?}", error);
            })
        } else {
            panic!("Problem opening the file: {:?}", error);
        }
    });
}
```
-->

```rust,ignore
use std::fs::File;
use std::io::ErrorKind;

fn main() {
    let f = File::open("hello.txt").unwrap_or_else(|erreur| {
        if error.kind() == ErrorKind::NotFound {
            File::create("hello.txt").unwrap_or_else(|erreur| {
                panic!("Erreur lors de la création du fichier : {:?}", erreur);
            })
        } else {
            panic!("Erreur lors de l'ouverture du fichier : {:?}", erreur);
        }
    });
}
```

<!--
Although this code has the same behavior as Listing 9-5, it doesn’t contain any
`match` expressions and is cleaner to read. Come back to this example after
you’ve read Chapter 13, and look up the `unwrap_or_else` method in the standard
library documentation. Many more of these methods can clean up huge nested
`match` expressions when you’re dealing with errors.
-->

Bien que ce code ait le même comportement que celui de l'encart 9-5, il ne
contient aucune expression `match` et est plus facile à lire. Revenez sur cet
exemple après avoir lu le chapitre 13, et renseignez-vous sur la méthode
`unwrap_or_else` dans la documentation de la bibliothèque standard. De
nombreuses méthodes de ce type peuvent clarifier de grosses expressions `match`
lorsque vous traitez les erreurs.

<!--
### Shortcuts for Panic on Error: `unwrap` and `expect`
-->

### Raccourci pour faire un Panic lors d'une erreur : `unwrap` et `expect`

<!--
Using `match` works well enough, but it can be a bit verbose and doesn’t always
communicate intent well. The `Result<T, E>` type has many helper methods
defined on it to do various tasks. One of those methods, called `unwrap`, is a
shortcut method that is implemented just like the `match` expression we wrote in
Listing 9-4. If the `Result` value is the `Ok` variant, `unwrap` will return
the value inside the `Ok`. If the `Result` is the `Err` variant, `unwrap` will
call the `panic!` macro for us. Here is an example of `unwrap` in action:
-->

L'utilisation de `match` fonctionne assez bien, mais elle peut être un peu
verbeuse et ne communique pas forcément bien son intention. Le type
`Result<T, R>` a de nombreuses méthodes qui lui ont été définies pour différents
cas. Une de ces méthodes, qui s'appelle `unwrap`, a été implémentée comme
l'expression `match` que nous avons écrit dans l'encart 9-4. Si la valeur de
`Result` est une variante de `Ok`, `unwrap` va retourner la valeur dans le
`Ok`. Si le `Result` est une variante de `Err`, `unwrap` va appeler la macro
`panic!` pour nous. Voici un exemple de `unwrap` à l'action :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").unwrap();
}
```

<!--
If we run this code without a *hello.txt* file, we’ll see an error message from
the `panic!` call that the `unwrap` method makes:
-->

Si nous exécutons ce code sans le fichier *hello.txt*, nous allons voir un
message d'erreur suite à l'appel à `panic!` que la méthode `unwrap` a fait :

```text
thread 'main' panicked at 'called `Result::unwrap()` on an `Err` value: Error {
repr: Os { code: 2, message: "No such file or directory" } }',
src/libcore/result.rs:906:4
```

<!--
Another method, `expect`, which is similar to `unwrap`, lets us also choose the
`panic!` error message. Using `expect` instead of `unwrap` and providing good
error messages can convey your intent and make tracking down the source of a
panic easier. The syntax of `expect` looks like this:
-->

L'autre méthode, `expect`, qui ressemble à `unwrap`, nous donne la possibilité
de définir le message d'erreur du `panic!`. Utiliser `expect` plutôt que
`unwrap` et lui fournir un bon message d'erreur permet de mieux exprimer le
problème et faciliter la recherche de la source d'erreur. La syntaxe de `expect`
est la suivante :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Failed to open hello.txt");
}
```
-->

```rust,should_panic
use std::fs::File;

fn main() {
    let f = File::open("hello.txt").expect("Echec à l'ouverture de hello.txt");
}
```

<!--
We use `expect` in the same way as `unwrap`: to return the file handle or call
the `panic!` macro. The error message used by `expect` in its call to `panic!`
will be the parameter that we pass to `expect`, rather than the default
`panic!` message that `unwrap` uses. Here’s what it looks like:
-->

Nous utilisons `expect` de la même manière que `unwrap` : pour retourner le
manipulateur de fichier ou appeler la macro `panic!`. Le message d'erreur
utilisé par `expect` lors de son appel au `panic!` sera le paramètre que nous
avons passé à `expect`, plutôt que le message par défaut de `panic!`
qu'utilise `unwrap`. Voici ce que cela donne :

<!--
```text
thread 'main' panicked at 'Failed to open hello.txt: Error { repr: Os { code:
2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```
-->

```text
thread 'main' panicked at 'Echec à l'ouverture de hello.txt: Error { repr: Os {
code: 2, message: "No such file or directory" } }', src/libcore/result.rs:906:4
```

<!--
Because this error message starts with the text we specified, `Failed to open
hello.txt`, it will be easier to find where in the code this error message is
coming from. If we use `unwrap` in multiple places, it can take more time to
figure out exactly which `unwrap` is causing the panic because all `unwrap`
calls that panic print the same message.
-->

Comme ce message d'erreur commence par le texte que nous avons précisé, `Echec à
l'ouverture de hello.txt`, ce sera plus facile de trouver où se situe ce message
d'erreur dans le code. Si nous utilisons `unwrap` à plusieurs endroits, cela
peut prendre plus de temps de comprendre exactement quel `unwrap` a causé la
panique, car tous les appels aux `unwrap` vont afficher le même message.

<!--
### Propagating Errors
-->

### Propager les Erreurs

<!--
When you’re writing a function whose implementation calls something that might
fail, instead of handling the error within this function, you can return the
error to the calling code so that it can decide what to do. This is known as
*propagating* the error and gives more control to the calling code, where there
might be more information or logic that dictates how the error should be
handled than what you have available in the context of your code.
-->

Lorsque vous écrivez une fonction dont l'implémentation utilise quelque chose
qui peut échouer, au lieu de gérer l'erreur dans cette fonction, vous pouvez
retourner cette erreur au code qui l'appelle pour qu'il décide quoi faire. C'est
ce que l'on appelle *propager* l'erreur et donne ainsi plus de contrôle au code
qui appelle la fonction, dans lequel il peut y avoir plus d'informations ou
d'instructions pour traiter l'erreur que dans le contexte de votre code.

<!--
For example, Listing 9-6 shows a function that reads a username from a file. If
the file doesn’t exist or can’t be read, this function will return those errors
to the code that called this function.
-->

Par exemple, l'encart 9-6 montre une fonction qui lit le nom d'utilisateur à
partir d'un fichier. Si ce fichier n'existe pas ou ne peut pas être lu, cette
fonction va retourner ces erreurs au code qui a appelé cette fonction.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let f = File::open("hello.txt");

    let mut f = match f {
        Ok(file) => file,
        Err(e) => return Err(e),
    };

    let mut s = String::new();

    match f.read_to_string(&mut s) {
        Ok(_) => Ok(s),
        Err(e) => Err(e),
    }
}
```

<!--
<span class="caption">Listing 9-6: A function that returns errors to the
calling code using `match`</span>
-->

<span class="caption">Encart 9-6 : une fonction qui retourne les erreurs au
code qui l'appelle en utilisant `match`</span>

<!--
This function can be written in a much shorter way, but we’re going to start by
doing a lot of it manually in order to explore error handling; at the end,
we’ll show the shorter way. Let’s look at the return type of the function first:
`Result<String, io::Error>`. This means the function is returning a value of
the type `Result<T, E>` where the generic parameter `T` has been filled in
with the concrete type `String` and the generic type `E` has been filled in
with the concrete type `io::Error`. If this function succeeds without any
problems, the code that calls this function will receive an `Ok` value that
holds a `String`—the username that this function read from the file. If this
function encounters any problems, the code that calls this function will
receive an `Err` value that holds an instance of `io::Error` that contains
more information about what the problems were. We chose `io::Error` as the
return type of this function because that happens to be the type of the error
value returned from both of the operations we’re calling in this function’s
body that might fail: the `File::open` function and the `read_to_string`
method.
-->

Cette fonction peut être écrite de façon plus concise, mais nous avons décidé de
commencer par faire un maximum de choses manuellement pour découvrir la gestion
d'erreurs ; mais à la fin, nous verrons comment raccourcir le code. Commençons par
regarder le type de retour de la fonction : `Result<String, io::Error>`. Cela
signifie que la fonction retourne une valeur de type `Result<T, E>` où le
paramètre générique `T` a été remplacé par le type `String` et le paramètre
générique `E` a été remplacé par le type `io::Error`. Si cette fonction réussit
avec succès, le code qui appelle cette fonction va obtenir une valeur `Ok` qui
contient un `String`, le nom d'utilisateur que cette fonction lit dans le
fichier. Si cette fonction rencontre un problème, le code qui appelle cette
fonction va obtenir une valeur `Err` qui contient une instance de `io::Error`
qui donne plus d'informations sur la raison du problème. Nous avons choisi
`io::Error` comme type de retour de cette fonction parce qu'il se trouve que
c'est le type d'erreur de retour pour toutes les opérations qui peuvent échouer
que l'on utilise dans le corps de cette fonction : la fonction `File::open` et
la méthode `read_to_string`.

<!--
The body of the function starts by calling the `File::open` function. Then we
handle the `Result` value returned with a `match` similar to the `match` in
Listing 9-4, only instead of calling `panic!` in the `Err` case, we return
early from this function and pass the error value from `File::open` back to the
calling code as this function’s error value. If `File::open` succeeds, we store
the file handle in the variable `f` and continue.
-->

Le corps de la fonction commence par appeler la fonction `File::open`. Ensuite,
nous gérons la valeur `Result` retourné, avec un `match` similaire au `match`
dans l'encart 9-4, mais, au lieu d'appeler `panic!` dans le cas de `Err`, nous
retournons prématurément le résultat de la fonction avec la valeur d'erreur de
`File::open` au code appelant avec la valeur d'erreur de cette fonction. Si
`File::open` réussit, nous enregistrons le manipulateur de fichier dans la
variable `f` et nous continuons.

<!--
Then we create a new `String` in variable `s` and call the `read_to_string`
method on the file handle in `f` to read the contents of the file into `s`. The
`read_to_string` method also returns a `Result` because it might fail, even
though `File::open` succeeded. So we need another `match` to handle that
`Result`: if `read_to_string` succeeds, then our function has succeeded, and we
return the username from the file that’s now in `s` wrapped in an `Ok`. If
`read_to_string` fails, we return the error value in the same way that we
returned the error value in the `match` that handled the return value of
`File::open`. However, we don’t need to explicitly say `return`, because this
is the last expression in the function.
-->

Ensuite, nous créons un nouveau `String` dans la variable `s` et nous appelons
la méthode `read_to_string` sur le manipulateur de fichier `f` pour extraire le
contenu du fichier dans `s`. La méthode `read_to_string` retourne aussi un
`Result` car elle peut échouer, même si `File::open` réussit. Nous avons donc
besoin d'un nouveau `match` pour gérer ce `Result` : si `read_to_string`
réussit, alors notre fonction a réussi, et nous retournons le nom d'utilisateur
présent dans le contenu du fichier qui est maintenant intégré dans un `Ok`,
lui-même stocké dans `s`. Si `read_to_string` échoue, nous retournons la valeur
d'erreur de la même façon que nous avons retourné la valeur d'erreur dans le
`match` qui gérait la valeur de retour de `File::open`. Cependant, nous n'avons
pas besoin d'écrire explicitement `return`, car c'est la dernière expression
de la fonction.

<!--
The code that calls this code will then handle getting either an `Ok` value
that contains a username or an `Err` value that contains an `io::Error`. We
don’t know what the calling code will do with those values. If the calling code
gets an `Err` value, it could call `panic!` and crash the program, use a
default username, or look up the username from somewhere other than a file, for
example. We don’t have enough information on what the calling code is actually
trying to do, so we propagate all the success or error information upward for
it to handle appropriately.
-->

Le code qui appelle ce code va devoir ensuite gérer les cas où il récupère une
valeur `Ok` qui contient un nom d'utilisateur, ou une valeur `Err` qui contient
une `io::Error`. Nous ne savons pas ce que va faire le code appelant avec ces
valeurs. Si le code appelant obtient une valeur `Err`, il peut appeler `panic!`
et faire planter le programme, utiliser un nom d'utilisateur par défaut, ou
chercher le nom d'utilisateur autre part que dans ce fichier, par exemple. Nous
n'avons pas assez d'informations sur ce que le code appelant a l'intention de
faire, donc nous remontons toutes les informations de succès ou d'erreur vers le
haut pour qu'elles soient gérées correctement.

<!--
This pattern of propagating errors is so common in Rust that Rust provides the
question mark operator `?` to make this easier.
-->

Cette façon de propager les erreurs est si courante en Rust que Rust fournit
l'opérateur du point d'interrogation `?` pour faciliter ceci.

<!--
#### A Shortcut for Propagating Errors: the `?` Operator
-->

#### Un raccourci pour propager les erreurs : l'opérateur `?`

<!--
Listing 9-7 shows an implementation of `read_username_from_file` that has the
same functionality as it had in Listing 9-6, but this implementation uses the
`?` operator.
-->

L'encart 9-7 montre une implémentation de `read_username_from_file` qui a les
mêmes fonctionnalités qu'elle a dans l'encart 9-6, mais cette implémentation
utilise l'opérateur point d'interrogation :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut f = File::open("hello.txt")?;
    let mut s = String::new();
    f.read_to_string(&mut s)?;
    Ok(s)
}
```

<!--
<span class="caption">Listing 9-7: A function that returns errors to the
calling code using the `?` operator</span>
-->

<span class="caption">Encart 9-7 : Une fonction qui retourne les erreurs au code
appelant en utilisant l'opérateur `?`</span>

<!--
The `?` placed after a `Result` value is defined to work in almost the same way
as the `match` expressions we defined to handle the `Result` values in Listing
9-6. If the value of the `Result` is an `Ok`, the value inside the `Ok` will
get returned from this expression, and the program will continue. If the value
is an `Err`, the `Err` will be returned from the whole function as if we had
used the `return` keyword so the error value gets propagated to the calling
code.
-->

Le `?` placé après une valeur `Result` est conçu pour fonctionner presque de la
même manière que les expressions `match` que nous avons défini pour gérer les
valeurs `Result` dans l'encart 9-6. Si la valeur du `Result` est un `Ok`, la
valeur dans le `Ok` sera retournée par cette expression et le programme
continuera. Si la valeur est une `Err`, la `Err` sera retournée par la fonction
comme si nous avions utilisé le mot-clé `result` afin que la valeur d'erreur
soit propagée au code appelant.

<!--
There is a difference between what the `match` expression from Listing 9-6 and
the `?` operator do: error values that have the `?` operator called on them go
through the `from` function, defined in the `From` trait in the standard
library, which is used to convert errors from one type into another. When the
`?` operator calls the `from` function, the error type received is converted
into the error type defined in the return type of the current function. This is
useful when a function returns one error type to represent all the ways a
function might fail, even if parts might fail for many different reasons. As
long as each error type implements the `from` function to define how to convert
itself to the returned error type, the `?` operator takes care of the
conversion automatically.
-->

Il y a une différence entre ce que fait l'expression `match` de l'encart 9-6 et
ce que fait l'opérateur `?` : les valeurs d'erreurs sur lesquelles sont
utilisées l'opérateur `?` passent par la fonction `from`, définie dans le trait
`From` de la bibliothèque standard, qui est utilisée pour convertir les erreurs
d'un type à un autre. Lorsque l'opérateur `?` utilise la fonction `from`, le
type d'erreur reçu est converti dans le type d'erreur déclaré dans le type de
retour de la fonction concernée. C'est utile lorsque une fonction retourne un
type d'erreur qui peut couvrir tous les cas d'échec de la fonction, même si
certaines de ses parties peuvent échouer pour différentes raisons. A partir du
moment que chaque type d'erreur implémente la fonction `from` pour expliquer
comment se convertir elle-même dans le type d'erreur retourné, l'opérateur `?`
se charge de faire la conversion automatiquement.

<!--
In the context of Listing 9-7, the `?` at the end of the `File::open` call will
return the value inside an `Ok` to the variable `f`. If an error occurs, the
`?` operator will return early out of the whole function and give any `Err`
value to the calling code. The same thing applies to the `?` at the end of the
`read_to_string` call.
-->

Dans le cas de l'encart 9-7, le `?` à la fin de l'appel à `File::open` va
retourner la valeur à l'intérieur d'un `Ok` à la variable `f`. Si une erreur se
produit, l'opérateur `?` va retourner prématurément la fonction et fournir une
valeur `Err` au code appelant. La même chose se produira au `?` à ma fin de
l'appel à `read_to_string`.

<!--
The `?` operator eliminates a lot of boilerplate and makes this function’s
implementation simpler. We could even shorten this code further by chaining
method calls immediately after the `?`, as shown in Listing 9-8.
-->

L'opérateur `?` épargne de l'écriture de code et facilite l'implémentation de la
fonction. Nous pouvons même encore plus réduire ce code en enchaînant
immédiatement les appels aux méthodes après le `?` comme dans l'encart 9-8 :

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
use std::io;
use std::io::Read;
use std::fs::File;

fn read_username_from_file() -> Result<String, io::Error> {
    let mut s = String::new();

    File::open("hello.txt")?.read_to_string(&mut s)?;

    Ok(s)
}
```

<!--
<span class="caption">Listing 9-8: Chaining method calls after the `?`
operator</span>
-->

<span class="caption">Encart 9-8 : Enchaînement des appels aux méthodes après
l'opérateur `?`</span>

<!--
We’ve moved the creation of the new `String` in `s` to the beginning of the
function; that part hasn’t changed. Instead of creating a variable `f`, we’ve
chained the call to `read_to_string` directly onto the result of
`File::open("hello.txt")?`. We still have a `?` at the end of the
`read_to_string` call, and we still return an `Ok` value containing the
username in `s` when both `File::open` and `read_to_string` succeed rather than
returning errors. The functionality is again the same as in Listing 9-6 and
Listing 9-7; this is just a different, more ergonomic way to write it.
-->

Nous avons déplacé la création de la nouvelle `String` dans `s` au début de la
fonction ; cette partie n'a pas changée. Au lieu de créer la variable `f`, nous
enchaînons directement l'appel à `read_to_string` sur le résultat de
`File::open("hello.txt")?`. Nous avons toujours le `?` à la fin de l'appel à
`read_to_string`, et nous retournons toujours une valeur `Ok` contenant le nom
d'utilisateur dans `s` lorsque `File::open` et `read_to_string` réussissent
toutes les deux plutôt que de retourner des erreurs. Cette fonctionnalité est
toujours la même que dans l'encart 9-6 et l'encart 9-7 ; c'est juste une façon
différente et plus ergonomique de l'écrire.

<!--
Speaking of different ways to write this function, Listing 9-9 shows that
there’s a way to make this even shorter.
-->

En parlant de différentes façons d'écrire cette fonction, l'encart 9-9 nous
montre qu'il y a une façon d'écrire encore moins de code.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

```rust
use std::io;
use std::fs;

fn read_username_from_file() -> Result<String, io::Error> {
    fs::read_to_string("hello.txt")
}
```

<!--
<span class="caption">Listing 9-9: Using `fs::read_to_string` instead of
opening and then reading the file</span>
-->

<span class="caption">Encart 9-9: Utilisation de `fs::read_to_string` plutôt que
d'ouvrir puis lire le fichier</span>

<!--
Reading a file into a string is a fairly common operation, so Rust provides the
convenient `fs::read_to_string` function that opens the file, creates a new
`String`, reads the contents of the file, puts the contents into that `String`,
and returns it. Of course, using `fs::read_to_string` doesn’t give us the
opportunity to explain all the error handling, so we did it the longer way
first.
-->

Récupérer le contenu d'un fichier dans une `String` est une opération assez
courante, donc Rust fournir la fonction `fs::read_to_string` assez pratique, qui
ouvre le fichier, crée une nouvelle `String`, lit de contenu du fichier, envoie
le contenu dans cette `String`, et la retourne. Evidemment, l'utilisation de
`fs:read_to_string` ne nous offre pas l'occasion d'expliquer toute la gestion
des erreurs, donc nous utiliserons d'abord la façon qui est plus longue.

<!--
#### The `?` Operator Can Be Used in Functions That Return `Result`
-->

#### L'opérateur `?` peut être utilisé dans des fonctions qui retournent un `Result`

<!--
The `?` operator can be used in functions that have a return type of
`Result`, because it is defined to work in the same way as the `match`
expression we defined in Listing 9-6. The part of the `match` that requires a
return type of `Result` is `return Err(e)`, so the return type of the function
can be a `Result` to be compatible with this `return`.
-->

L'opérateur `?` peut être utilisé dans des fonctions qui ont un type de retour
`Result`, car il est conçu pour fonctionner de la même manière que l'expression
`match` que nous avons utilisé dans l'encart 9-6. La partie du `match` qui
nécessite le type de retour `Result` est `return Err(e)`, donc le type de retour
de cette fonction peut être `Result` pour être compatible avec ce `return`.

<!--
Let’s look at what happens if we use the `?` operator in the `main` function,
which you’ll recall has a return type of `()`:
-->

Voyons ce que ce passe si nous utilisons l'opérateur `?` dans la fonction
`main`, pour laquelle vous devriez vous souvenir qu'elle a un type de retour
`()` :

```rust,ignore,does_not_compile
use std::fs::File;

fn main() {
    let f = File::open("hello.txt")?;
}
```

<!--
When we compile this code, we get the following error message:
-->

Lorsque nous compilons ce code, nous obtenons le message d'erreur suivant :

```text
error[E0277]: the `?` operator can only be used in a function that returns
`Result` or `Option` (or another type that implements `std::ops::Try`)
 -- > src/main.rs:4:13
  |
4 |     let f = File::open("hello.txt")?;
  |             ^^^^^^^^^^^^^^^^^^^^^^^^ cannot use the `?` operator in a
  function that returns `()`
  |
  = help: the trait `std::ops::Try` is not implemented for `()`
  = note: required by `std::ops::Try::from_error`
```

<!--
This error points out that we’re only allowed to use the `?` operator in a
function that returns `Result` or `Option` or another type that implements
`std::ops::Try`. When you’re writing code in a function that doesn’t return
one of these types, and you want to use `?` when you call other functions that
return `Result<T, E>`, you have two choices to fix this problem. One technique
is to change the return type of your function to be `Result<T, E>` if you have
no restrictions preventing that. The other technique is to use a `match` or one
of the `Result<T, E>` methods to handle the `Result<T, E>` in whatever way is
appropriate.
-->

Cette erreur explique que nous sommes uniquement autorisés à utiliser
l'opérateur `?` dans une fonction qui retourne `Result` ou `Option` ou un autre
type qui implémente `std::ops::Try`. Lorsque vous écrivez du code dans une
fonction qui ne retourne pas un de ces types, et que vous souhaitez utiliser `?`
lorsque vous appelez d'autres fonctions qui retournent `Result<T, E>`, vous avez
deux façons de régler le problème. La première est de changer le type de retour
de votre fonction en `Result<T, E>` si vous pouvez le faire. L'autre solution
est d'utiliser un `match` ou une des méthodes de `Result<T, E>` pour gérer le
`Result<T, E>` de la manière la plus appropriée.

<!--
The `main` function is special, and there are restrictions on what its return
type must be. One valid return type for main is `()`, and conveniently, another
valid return type is `Result<T, E>`, as shown here:
-->

La fonction `main` est spéciale, et il y a des restrictions sur ce que doit être
son type de retour. Une type de retour correct pour `main` est `()`, et il
existe aussi un autre type de retour acceptable qui est `Result<T, E>`, comme
ci-dessous :

```rust,ignore
use std::error::Error;
use std::fs::File;

fn main() -> Result<(), Box<dyn Error>> {
    let f = File::open("hello.txt")?;

    Ok(())
}
```

<!--
The `Box<dyn Error>` type is called a trait object, which we’ll talk about in
the [“Using Trait Objects that Allow for Values of Different
Types”][trait-objects]<!-- ignore -- > section in Chapter 17. For now, you can
read `Box<dyn Error>` to mean “any kind of error.” Using `?` in a `main`
function with this return type is allowed.
-->

Le type `Box<dyn Error>` est ce qu'on appelle un objet trait, que nous allons
voir dans une section du [chapitre 17][trait-objects]<!-- ignore -->. Pour
l'instant vous pouvez interpréter `Box<dyn Error>` en “tout type d'erreur”.
L'utilisation de `?` dans la fonction `main` avec ce type de retour est donc
autorisée.

<!--
Now that we’ve discussed the details of calling `panic!` or returning `Result`,
let’s return to the topic of how to decide which is appropriate to use in which
cases.
-->

Maintenant que nous avons vu les détails pour utiliser `panic!` ou retourner
`Result`, voyons maintenant comment choisir ce qu'il faut faire en fonction des
cas.

<!--
[trait-objects]: ch17-02-trait-objects.html#using-trait-objects-that-allow-for-values-of-different-types
-->

[trait-objects]: ch17-02-trait-objects.html
