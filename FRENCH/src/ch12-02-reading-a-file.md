> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autre types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/135).

<!--
## Reading a File
-->

## Lire un fichier

<!--
Now we’ll add functionality to read the file that is specified in the
`filename` command line argument. First, we need a sample file to test it with:
the best kind of file to use to make sure `minigrep` is working is one with a
small amount of text over multiple lines with some repeated words. Listing 12-3
has an Emily Dickinson poem that will work well! Create a file called
*poem.txt* at the root level of your project, and enter the poem “I’m Nobody!
Who are you?”
-->

Maintenant, nous allons ajouter une fonctionnalité pour lire le fichier qui est
renseigné dans l'argument `nom_fichier` de la ligne de commande. D'abord, nous
avons besoin d'un fichier d'exemple pour le tester : le meilleur type de
fichier pour s'assurer que `minigrep` fonctionne est un fichier avec une petite
quantité de texte sur plusieurs lignes avec quelques mots répétés.

<span class="filename">Filename: poem.txt</span>

```text
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us - don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<!--
<span class="caption">Listing 12-3: A poem by Emily Dickinson makes a good test
case</span>
-->

<span class="caption">Encart 12-3 : Un poème Anglais d'Emily Dickinson qui fait
un bon sujet d'essai</span>

<!--
With the text in place, edit *src/main.rs* and add code to read the file, as
shown in Listing 12-4.
-->

Une fois ce texte enregistré, éditez le *src/main.rs* et ajoutez-y le code pour
lire le fichier, comme indiqué dans l'encart 12-4.

<!--
<span class="filename">Filename: src/main.rs</span>
-->

<span class="filename">Fichier : src/main.rs</span>

<!--
```rust,should_panic
use std::env;
use std::fs;

fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let query = &args[1];
#     let filename = &args[2];
#
#     println!("Searching for {}", query);
    // --snip--
    println!("In file {}", filename);

    let contents = fs::read_to_string(filename)
        .expect("Something went wrong reading the file");

    println!("With text:\n{}", contents);
}
```
-->

```rust,should_panic
use std::env;
use std::fs;

fn main() {
#     let args: Vec<String> = env::args().collect();
#
#     let recherche = &args[1];
#     let nom_fichier = &args[2];
#
#     println!("On recherche : {}", recherche);
    // -- partie masquée ici --
    println!("Dans le fichier : {}", nom_fichier);

    let contenu = fs::read_to_string(nom_fichier)
        .expect("Quelque chose s'est mal passé lors de la lecture du fichier");

    println!("Dans le texte :\n{}", contenu);
}
```

<!--
<span class="caption">Listing 12-4: Reading the contents of the file specified
by the second argument</span>
-->

<span class="caption">Encart 12-4 : Lecture du contenu du fichier renseigné en
second argument</span>

<!--
First, we add another `use` statement to bring in a relevant part of the
standard library: we need `std::fs` to handle files.
-->

Premièrement, nous ajoutons une autre instruction `use` pour importer une
partie significative de la bibliothèque standard : nous avons besoin de
`std::fs` pour manipuler les fichiers.

<!--
In `main`, we’ve added a new statement: `fs::read_to_string` takes the
`filename`, opens that file, and returns a `Result<String>` of the file’s
contents.
-->

Dans le `main`, nous avons ajouté une nouvelle instruction :
`fs::read_to_string` qui prend le `nom_fichier`, ouvre ce fichier, et retourne
un `Result<String>` du contenu du fichier.

<!--
After that statement, we’ve again added a temporary `println!` statement that
prints the value of `contents` after the file is read, so we can check that the
program is working so far.
-->

Après cette instruction, nous avons ajouté à nouveau une instruction `println!`
qui affiche la valeur de `contenu` après la lecture de ce fichier, afin que
nous puissions vérifier que ce programme fonctionne correctement.

<!--
Let’s run this code with any string as the first command line argument (because
we haven’t implemented the searching part yet) and the *poem.txt* file as the
second argument:
-->

Exécutons ce code avec n'importe quelle chaîne de caractères dans le premier
argument de la ligne de commande (car nous n'avons pas encore implémenté la
partie de recherche pour l'instant), ainsi que le fichier *poem.txt* en
second argument :

<!--
```text
$ cargo run the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep the poem.txt`
Searching for the
In file poem.txt
With text:
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```
-->

```text
$ cargo run the poem.txt
   Compiling minigrep v0.1.0 (file:///projects/minigrep)
    Finished dev [unoptimized + debuginfo] target(s) in 0.0 secs
     Running `target/debug/minigrep the poem.txt`
On recherche : the
Dans le fichier : poem.txt
Dans le texte :
I'm nobody! Who are you?
Are you nobody, too?
Then there's a pair of us — don't tell!
They'd banish us, you know.

How dreary to be somebody!
How public, like a frog
To tell your name the livelong day
To an admiring bog!
```

<!--
Great! The code read and then printed the contents of the file. But the code
has a few flaws. The `main` function has multiple responsibilities: generally,
functions are clearer and easier to maintain if each function is responsible
for only one idea. The other problem is that we’re not handling errors as well
as we could. The program is still small, so these flaws aren’t a big problem,
but as the program grows, it will be harder to fix them cleanly. It’s good
practice to begin refactoring early on when developing a program, because it’s
much easier to refactor smaller amounts of code. We’ll do that next.
-->

Très bien ! Notre code lit et affiche ensuite le contenu du fichier. Mais le
code a quelques défauts. La fonction `main` a plusieurs responsabilités :
généralement, les rôles des fonctions sont plus claires et faciles à entretenir
si chaque fonction est en charge d'une seule tâche. L'autre problème est que
nous ne gérons pas les erreurs correctement. Le programme est encore très
modeste, donc ces imperfections ne sont pas un gros problème, mais dès que le
programme va grossir, il sera plus difficile de les corriger proprement. Le
remaniement du code très tôt lors du développement d'un logiciel est une bonne
pratique, car c'est beaucoup plus facile de remanier des petites portions de
code. C'est ce que nous allons faire dès maintenant.
