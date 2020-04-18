> # 🚧 Attention, peinture fraîche !
>
> Cette page a été traduite par une seule personne et n'a pas été relue et
> vérifiée par quelqu'un d'autre ! Les informations peuvent par exemple être
> erronées, être formulées maladroitement, ou contenir d'autres types de fautes.
>
> Vous pouvez contribuer à l'amélioration de cette page sur sa
> [Pull Request](https://github.com/Jimskapt/rust-book-fr/pull/228).

<!--
## Appendix G - How Rust is Made and “Nightly Rust”
-->

## Annexe G - Comment Rust est construit, et “Nightly Rust”

<!--
This appendix is about how Rust is made and how that affects you as a Rust
developer.
-->

Cette annexe va expliquer comment Rust est construit et comment cela vous
impacte en tant que développeur Rust.

<!--
### Stability Without Stagnation
-->

### La stabilité sans stagnation

<!--
As a language, Rust cares a *lot* about the stability of your code. We want
Rust to be a rock-solid foundation you can build on, and if things were
constantly changing, that would be impossible. At the same time, if we can’t
experiment with new features, we may not find out important flaws until after
their release, when we can no longer change things.
-->

En tant que langage, Rust se soucie *beaucoup* de la stabilité de votre code.
Nous voulons que Rust soit une solide fondation sur laquelle vous pouvez
construire, et si les choses changent constamment, cela serait impossible. En
même temps, si nous ne pouvions pas expérimenter de nouvelles fonctionnalités,
nous ne pourrions pas découvrir les défauts importants avant leur publication,
ce qui serait trop tard pour changer les choses.

<!--
Our solution to this problem is what we call “stability without stagnation”,
and our guiding principle is this: you should never have to fear upgrading to a
new version of stable Rust. Each upgrade should be painless, but should also
bring you new features, fewer bugs, and faster compile times.
-->

Notre solution à ce problème est ce que nous appelons la “stabilité sans
stagnation”, et notre ligne directrice est la suivante : vous ne devriez jamais
craindre de passer à nouvelle version de Rust stable. Chaque mise à jour devrait
être facile, et devrait aussi vous apporter de nouvelles fonctionnalités, moins
de bogues et un temps de compilation plus rapide.

<!--
### Choo, Choo! Release Channels and Riding the Trains
-->

### Les canaux de diffusion et sauter dans le train

<!--
Rust development operates on a *train schedule*. That is, all development is
done on the `master` branch of the Rust repository. Releases follow a software
release train model, which has been used by Cisco IOS and other software
projects. There are three *release channels* for Rust:
-->

Le développement de Rust suit un *planning ferroviaire*. Ce que cela veut dire,
c'est que tout le développement est fait sur la branche `master` du dépôt de
Rust. Les publications suivent le modèle de trains de publication de programmes,
qui a été popularisé par Cisco IOS et d'autres projets logiciels. Il y a trois
*canaux de diffusion* pour Rust :

<!--
* Nightly
* Beta
* Stable
-->

* Nightly
* Beta
* Stable

<!--
Most Rust developers primarily use the stable channel, but those who want to
try out experimental new features may use nightly or beta.
-->

La plupart des développeurs Rust utilisent principalement le canal stable, mais
ceux qui souhaitent essayer les nouvelles fonctionnalités expérimentales
utilisent nightly ou beta.

<!--
Here’s an example of how the development and release process works: let’s
assume that the Rust team is working on the release of Rust 1.5. That release
happened in December of 2015, but it will provide us with realistic version
numbers. A new feature is added to Rust: a new commit lands on the `master`
branch. Each night, a new nightly version of Rust is produced. Every day is a
release day, and these releases are created by our release infrastructure
automatically. So as time passes, our releases look like this, once a night:
-->

Voici un exemple du fonctionnement du processus de développement et de
publication : supposons que l'équipe de Rust travaille sur la publication de
Rust 1.5. Cette publication a été faite en décembre 2015, et nous permet de nous
appuyer sur des numéros de version réalistes. Une nouvelle fonctionnalité a été
ajoutée à Rust : un nouveau commit est arrivé sur la branche `master`. Chaque
nuit, une nouvelle version nightly de Rust est produite. Chaque jour voit une
nouvelle publication, et ces publications sont créées automatiquement par
l'infrastructure de publication. Ainsi, les publications ressemblent à ceci, une
fois par nuit :

```text
nightly: * - - * - - *
```

<!--
Every six weeks, it’s time to prepare a new release! The `beta` branch of the
Rust repository branches off from the `master` branch used by nightly. Now,
there are two releases:
-->

Tous les six semaines, c'est le moment de préparer une nouvelle publication ! La
branche `beta` du dépôt Rust est alors dérivée de la branche `master` utilisée
par nightly. Ainsi, il y a deux canaux de publications :

```text
nightly: * - - * - - *
                     |
beta:                *
```

<!--
Most Rust users do not use beta releases actively, but test against beta in
their CI system to help Rust discover possible regressions. In the meantime,
there’s still a nightly release every night:
-->

La plupart des utilisateurs Rust n'utilisent pas activement les publications en
beta, mais les tests en beta sur leur système d'Intégration Continue aident à
découvrir des potentielles régressions. Pendant ce temps, il continue à avoir
une publication nightly chaque nuit :

```text
nightly: * - - * - - * - - * - - *
                     |
beta:                *
```

<!--
Let’s say a regression is found. Good thing we had some time to test the beta
release before the regression snuck into a stable release! The fix is applied
to `master`, so that nightly is fixed, and then the fix is backported to the
`beta` branch, and a new release of beta is produced:
-->

Imaginons qu'une régression soit trouvée. C'est alors une bonne chose que nous
ayons du temps pour tester la publication beta avant que la régression se
retrouve dans une publication stable ! La correction est alors appliquée sur
`master`, ainsi nightly est corrigé, et ensuite la correction est reportée sur
la branche `beta`, et une nouvelle publication de beta est produite :

```text
nightly: * - - * - - * - - * - - * - - *
                     |
beta:                * - - - - - - - - *
```

<!--
Six weeks after the first beta was created, it’s time for a stable release! The
`stable` branch is produced from the `beta` branch:
-->

Six semaines après que la première beta soit créée, c'est le moment de publier
une version stable ! La branche `stable` est produite à partir de la branche
`beta` :

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |
beta:                * - - - - - - - - *
                                       |
stable:                                *
```

<!--
Hooray! Rust 1.5 is done! However, we’ve forgotten one thing: because the six
weeks have gone by, we also need a new beta of the *next* version of Rust, 1.6.
So after `stable` branches off of `beta`, the next version of `beta` branches
off of `nightly` again:
-->

Youpi ! Rust 1.5 est sorti ! Cependant, nous avons oublié quelque chose : comme
les six semaines sont passées, nous devons aussi publier une nouvelle beta de la
version *suivante* de Rust, la 1.6. Donc après que la branche `stable` soit
dérivée de la `beta`, la prochaine version de la branche `beta` doit à nouveau
être dérivée de nightly :

```text
nightly: * - - * - - * - - * - - * - - * - * - *
                     |                         |
beta:                * - - - - - - - - *       *
                                       |
stable:                                *
```

<!--
This is called the “train model” because every six weeks, a release “leaves the
station”, but still has to take a journey through the beta channel before it
arrives as a stable release.
-->

C'est appelé le “modèle ferroviaire” car toutes les six semaines, une nouvelle
publication “quitte la gare”, mais doit encore voyager dans la voie de la
beta avant d'arriver en gare de la publication stable.

<!--
Rust releases every six weeks, like clockwork. If you know the date of one Rust
release, you can know the date of the next one: it’s six weeks later. A nice
aspect of having releases scheduled every six weeks is that the next train is
coming soon. If a feature happens to miss a particular release, there’s no need
to worry: another one is happening in a short time! This helps reduce pressure
to sneak possibly unpolished features in close to the release deadline.
-->

Rust publie régulièrement toutes les six semaines, réglée comme une montre. Si
vous savez la date d'une publication Rust, vous savez la date de la suivante :
elle aura toujours lieu six semaines plus tard. Un des avantages d'avoir des
publications planifiées toutes les six semaines est que le train suivant arrive
rapidement après. Si une fonctionnalité n'est pas intégrée à une publication, il
n'y a pas à s'inquiéter : une autre arrive bientôt ! Cela aide à réduire la
pression pour faire passer en toute discrétion des fonctionnalités
éventuellement inachevées à l'approche de la date limite de diffusion.

<!--
Thanks to this process, you can always check out the next build of Rust and
verify for yourself that it’s easy to upgrade to: if a beta release doesn’t
work as expected, you can report it to the team and get it fixed before the
next stable release happens! Breakage in a beta release is relatively rare, but
`rustc` is still a piece of software, and bugs do exist.
-->

Grâce à ce processus, vous pouvez toujours découvrir la prochaine compilation de
Rust et constater par vous-même qu'il est facile de mettre à jour : si une
publication en beta ne fonctionne pas comme prévu, vous pouvez signaler cela à
l'équipe et cela sera corrigé avant que la prochaine publication stable soit
produite ! La dégradation d'une version bêta est plutôt rare, mais `rustc` reste
un logiciel, et les bogues peuvent exister malgré tout.

<!--
### Unstable Features
-->

### Les fonctionnalités instables

<!--
There’s one more catch with this release model: unstable features. Rust uses a
technique called “feature flags” to determine what features are enabled in a
given release. If a new feature is under active development, it lands on
`master`, and therefore, in nightly, but behind a *feature flag*. If you, as a
user, wish to try out the work-in-progress feature, you can, but you must be
using a nightly release of Rust and annotate your source code with the
appropriate flag to opt in.
-->

Il reste une surprise avec ce modèle de publication : les fonctionnalités
instables. Rust utilise une technique qui s'appelle les “drapeaux de
fonctionnalités” pour déterminer quelles fonctionnalités sont activées dans une
publication donnée. Si une nouvelle fonctionnalité est en développement actif,
elle va atterrir sur `master`, et ainsi, dans nightly, mais derrière un *drapeau
de fonctionnalités*. Si vous, en tant qu'utilisateur, souhaitez essayer la
fonctionnalité en cours de développement, vous pouvez, mais vous devez utiliser
une publication nightly de Rust et annoter votre code source avec le drapeau
approprié pour l'activer.

<!--
If you’re using a beta or stable release of Rust, you can’t use any feature
flags. This is the key that allows us to get practical use with new features
before we declare them stable forever. Those who wish to opt into the bleeding
edge can do so, and those who want a rock-solid experience can stick with
stable and know that their code won’t break. Stability without stagnation.
-->

Si vous utilisez une publication beta ou stable de Rust, vous ne pouvez pas
utiliser de drapeaux de fonctionnalités. C'est la clé qui permet d'obtenir une
utilisation pratique avec les nouvelles fonctionnalités avant que nous les
déclarions stables pour toujours. Ceux qui souhaitent activer ces
fonctionnalités expérimentales peuvent le faire, et ceux qui souhaitent avoir
une expérience plus solide peuvent s'en tenir au canal stable et leur code ne
sera pas cassé. C'est la stabilité sans stagnation.

<!--
This book only contains information about stable features, as in-progress
features are still changing, and surely they’ll be different between when this
book was written and when they get enabled in stable builds. You can find
documentation for nightly-only features online.
-->

Ce livre contient uniquement des informations sur des fonctionnalités stables,
car les fonctionnalités en cours de développement sont toujours en train de
changer, et elles seront sûrement différentes entre le moment où ce livre sera
écrit et lorsqu'elles seront activées dans les compilations stables. Vous pouvez
trouver la documentation pour les fonctionnalités uniquement pour nightly en
ligne.

<!--
### Rustup and the Role of Rust Nightly
-->

### Rustup et le role de Rust nightly

<!--
Rustup makes it easy to change between different release channels of Rust, on a
global or per-project basis. By default, you’ll have stable Rust installed. To
install nightly, for example:
-->

Rustup facilite les changements entre les différents canaux de publication de
Rust, de manière globale ou par projet. Par défaut, vous avez Rust stable
d'installé. Pour installer nightly, vous pouvez saisir, par exemple :

```text
$ rustup install nightly
```

<!--
You can see all of the *toolchains* (releases of Rust and associated
components) you have installed with `rustup` as well. Here’s an example on one
of your authors’ Windows computer:
-->

Vous pouvez aussi voir avec `rustup` toutes les *toolchains* (les publications
de Rust et leurs composants associés) que vous avez d'installées. Voici un
exemple d'un ordinateur sous Windows d'un des auteurs du livre :

```powershell
> rustup toolchain list
stable-x86_64-pc-windows-msvc (default)
beta-x86_64-pc-windows-msvc
nightly-x86_64-pc-windows-msvc
```

<!--
As you can see, the stable toolchain is the default. Most Rust users use stable
most of the time. You might want to use stable most of the time, but use
nightly on a specific project, because you care about a cutting-edge feature.
To do so, you can use `rustup override` in that project’s directory to set the
nightly toolchain as the one `rustup` should use when you’re in that directory:
-->

Comme vous pouvez le constater, la toolchain stable est celle par défaut. La
plupart des utilisateurs Rust utilisent celle qui est stable la plupart du
temps. Il est possible que vous souhaitiez utiliser celle qui est stable la
plupart du temps, mais que vous souhaitiez utiliser nightly sur un projet
particulier, car parce que vous vous intéressez à une fonctionnalité
expérimentale. Pour pouvoir faire cela, vous pouvez utiliser `rustup override`
dans le dossier de ce projet pour régler `rustup` pour qu'il utilise la
toolchain nightly lorsque vous vous trouvez dans ce dossier :

<!--
```text
$ cd ~/projects/needs-nightly
$ rustup override set nightly
```
-->

```text
$ cd ~/projets/necessite-nightly
$ rustup override set nightly
```

<!--
Now, every time you call `rustc` or `cargo` inside of
*~/projects/needs-nightly*, `rustup` will make sure that you are using nightly
Rust, rather than your default of stable Rust. This comes in handy when you
have a lot of Rust projects!
-->

Maintenant, à chaque fois que vous faites appel à `rustc` ou `cargo` à
l'intérieur de *~/projets/necessite-nightly*, `rustup` va s'assurer que vous
utilisez Rust nightly, plutôt que votre Rust stable par défaut. C'est très utile
lorsque vous avez beaucoup de projets Rust !

<!--
### The RFC Process and Teams
-->

### Le processus de RFC et les équipes

<!--
So how do you learn about these new features? Rust’s development model follows
a *Request For Comments (RFC) process*. If you’d like an improvement in Rust,
you can write up a proposal, called an RFC.
-->

Donc, comment en apprendre plus ces nouvelles fonctionnalités ? Le modèle de
développement de Rust suit le *processus de Request For Comments (RFC)*. Si vous
souhaitez avoir une amélioration de Rust, vous pouvez rédiger une proposition,
qu'on appelle une RFC.

<!--
Anyone can write RFCs to improve Rust, and the proposals are reviewed and
discussed by the Rust team, which is comprised of many topic subteams. There’s
a full list of the teams [on Rust’s
website](https://www.rust-lang.org/governance), which includes teams for
each area of the project: language design, compiler implementation,
infrastructure, documentation, and more. The appropriate team reads the
proposal and the comments, writes some comments of their own, and eventually,
there’s consensus to accept or reject the feature.
-->

N'importe qui peut écrire de RFC pour améliorer Rust, et les propositions sont
examinées et débattues par l'équipe de Rust, qui est composée de nombreuses
sous-équipes spécialisées dans différents domaines. Voici une liste complète des
équipes [sur le site web de Rust](https://www.rust-lang.org/governance), qui
comprend des équipes pour chaque aspect du projet : la conception du langage,
l'implémentation du compilateur, de l'infrastructure, de la documentation, et
plus encore. L'équipe appropriée lit la proposition et les commentaires, écrit
quelques commentaires la concernant, et finalement, un consensus se crée pour
accepter ou rejeter la fonctionnalité.

<!--
If the feature is accepted, an issue is opened on the Rust repository, and
someone can implement it. The person who implements it very well may not be the
person who proposed the feature in the first place! When the implementation is
ready, it lands on the `master` branch behind a feature gate, as we discussed
in the [“Unstable Features”](#unstable-features)<!-- ignore -- > section.
-->

Si la fonctionnalité est acceptée, un ticket est ouvert sur le dépôt de Rust, et
quelqu'un peut l'implémenter. La personne qui l'implémente ne peut pas être
celle qui a proposé la fonctionnalité ! Lorsque l'implémentation est prête, elle
atterrit sur la branche `master` derrière un drapeau de fonctionnalité, comme
nous l'avons vu dans la section [“Les fonctionnalités
instables”](#les-fonctionnalités-instables)<!-- ignore -->.

<!--
After some time, once Rust developers who use nightly releases have been able
to try out the new feature, team members will discuss the feature, how it’s
worked out on nightly, and decide if it should make it into stable Rust or not.
If the decision is to move forward, the feature gate is removed, and the
feature is now considered stable! It rides the trains into a new stable release
of Rust.
-->

Au bout d'un moment, une fois que les développeurs Rust qui utilisent les
publications nightly ont pu tester la nouvelle fonctionnalité, les membres de
l'équipe vont discuter de la fonctionnalité, de voir comment elle a
fonctionné sur nightly, et vont décider si elle doit être publiée sur Rust
stable ou non. Si la décision est d'avancer, le drapeau de fonctionnalité est
enlevé, et la fonctionnalité est maintenant considérée comme stable ! Elle saute
alors dans le train en direction d'une nouvelle publication stable de Rust.
