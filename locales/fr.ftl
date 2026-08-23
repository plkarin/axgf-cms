# axgf-cms — chaînes de l'interface, français.
#
# Traduction relue. Le vocabulaire généalogique suit l'usage français :
# « union » pour union, « fiabilité » pour reliability, « degré de certitude »
# pour confidence, « acte » pour un document d'état civil.
#
# RÈGLE : ce fichier ne traduit que l'interface. Les noms, les lieux, les notes
# et les métiers viennent du fichier .axgf et restent dans leur langue et leur
# écriture d'origine.

app-name = axgf-cms

## Cadre

nav-tree = Arbre
nav-convert = Convertir un GEDCOM
nav-admin = Administration
nav-sign-in = Se connecter
nav-sign-out = Se déconnecter
footer-served-from = Servi depuis un unique fichier .axgf. Toute la logique généalogique se trouve dans axgf-rs ; le format est spécifié par axgf-spec.

## Préférences

prefs-title = Langue et apparence
prefs-language = Langue
prefs-language-note = Ceci ne change que l'interface. Les noms, les lieux et les notes sont toujours affichés dans leur propre langue et leur propre écriture.
prefs-theme = Apparence
prefs-apply = Appliquer
prefs-reviewed = relu
prefs-machine = automatique, { $coverage } %
prefs-machine-title = Traduit sans relecture par une personne dont c'est la langue maternelle. Le vocabulaire généalogique en particulier peut être erroné. Les corrections sont bienvenues — voir CONTRIBUTING.md.

theme-light = Clair
theme-dark = Sombre
theme-system = Suivre mon système
theme-high-contrast = Contraste élevé
theme-sepia = Sépia
theme-deuteranopia = Deutéranopie
theme-protanopia = Protanopie
theme-tritanopia = Tritanopie
theme-colour-blind-note = adapté au daltonisme
theme-contrast-note = contraste maximal

## Arbre

tree-title-around = Autour de { $name }
tree-title-whole = L'arbre entier
tree-lede-focused = { $ancestors ->
        [one] Un ascendant
       *[other] { $ancestors } ascendants
    }, { $descendants ->
        [one] un descendant
       *[other] { $descendants } descendants
    } et { $spouses ->
        [one] un conjoint
       *[other] { $spouses } conjoints
    }, { $depth } générations de chaque côté. Les plus anciens en bas. L'opacité des traits indique le degré de certitude de la relation — un trait pâle est une affirmation dont le dossier n'est pas sûr.
tree-lede-whole = Toutes les personnes du fichier. Les plus anciens en bas, les plus jeunes en haut. L'opacité des traits indique le degré de certitude de la relation.
tree-filter-label = Filtrer les fiches affichées
tree-filter-placeholder = Saisissez un nom…
tree-centre-on = Centrer sur
tree-depth = Générations de chaque côté
tree-show = Afficher
tree-whole-tree = Arbre entier
tree-focused = Vue centrée
tree-hidden-notice = { $n ->
        [one] Une personne est affichée sans ses informations
       *[other] { $n } personnes sont affichées sans leurs informations
    }
tree-hidden-because-role = , car leur visibilité dépasse ce que votre compte peut lire.
tree-hidden-because-anonymous = , car elles ne sont pas publiques.
tree-hidden-sign-in = Connectez-vous si vous avez un compte.
tree-restricted-card = La fiche de cette personne ne vous est pas visible
tree-width-warning = Cette zone de dessin fait { $width } pixels de large. Personne ne fait défiler autant pour retrouver un ancêtre — c'est la raison d'être de la vue centrée.
tree-empty = Ce fichier ne contient personne à dessiner.
tree-unplaced = Dans aucune famille enregistrée
tree-legend-confidence = L'opacité du trait indique le degré de certitude
tree-recentre = Centrer l'arbre ici
tree-open-record = Ouvrir la fiche complète

## La fiche

record-identity = Identité
record-life-events = Événements de la vie
record-family = Famille
record-other-relationships = Autres relations
record-occupations = Métiers
record-places = Lieux
record-sources-documents = Sources et documents
record-notes = Notes
record-history = Historique
record-raw = Entité brute
record-raw-summary-note = le JSON à partir duquel cette page a été construite

record-identity-help = Chaque nom enregistré avec son type, la période où il a été porté et la source qui l'atteste, avec l'écriture d'origine et sa translittération latine côte à côte lorsqu'elles diffèrent, ainsi que le genre, le statut vivant et la visibilité.
record-life-events-help = Naissance, décès et tous les événements auxquels cette personne a pris part, par ordre de date, chacun avec son rôle — de sorte qu'un mariage dont elle n'a été que témoin figure à côté du sien. Un fait non daté est classé en dernier plutôt que de prétendre venir en premier.
record-family-help = Parents et fratrie, puis chaque union avec son type, ses dates, son lieu, la manière dont elle a pris fin et ses enfants par ordre de naissance.
record-other-relationships-help = Chaque lien dont cette personne est l'une des extrémités, lu de son côté, de sorte que la même relation se lit « parrain de » d'un côté et « filleul de » de l'autre.
record-occupations-help = Les métiers sous forme de périodes sur un axe commun, afin que deux postes puissent être comparés d'un coup d'œil, avec des barres ouvertes lorsqu'une borne manque.
record-places-help = Chaque lieu que cette fiche touche, avec ce qui s'y est passé et l'histoire des frontières qui donne son sens à un lieu à travers le temps.
record-sources-documents-help = Chaque source nomme les faits de cette page qui reposent sur elle, classées selon la force de la preuve.
record-notes-help = Notes sur cette fiche, y compris le texte qu'aucun convertisseur n'a su interpréter et qui a été conservé mot pour mot plutôt que supprimé.
record-history-help = Chaque modification enregistrée de cette fiche, la plus récente en premier, depuis le journal des modifications tenu à côté du fichier. Qui a corrigé quoi est un fait concernant les rédacteurs de cette famille et non la famille elle-même : c'est pourquoi ce journal reste hors du .axgf et n'est montré qu'aux personnes connectées.
record-raw-help = Rien ici n'est produit pour l'affichage : voici la fiche telle que le fichier la stocke. Un format qui mérite qu'on le défende doit être lisible sans outil.
record-help-toggle = Ce que montre cette section

record-gender = Genre
record-living = Vivant
record-visibility = Visibilité
record-yes = oui
record-no = non
record-name-type = Type de nom
record-name-used = Porté
record-name-evidence = Preuve
record-name-primary = nom principal
record-transliteration = Translittération latine
record-born = Né(e)
record-died = Décédé(e)
record-parents = Parents
record-siblings = Fratrie
record-children = Enfants
record-spouse = Conjoint
record-union-ended = Fin
record-no-date = Date inconnue
record-unknown-person = [Inconnu]
record-restricted-person = Privé
record-restricted-title = La fiche de cette personne ne vous est pas visible
record-absent-person-title = Cité par ce fichier mais absent de celui-ci
record-confidence = Degré de certitude
record-source = Source
record-role = Rôle
record-download = Télécharger
record-attach-file = Joindre un fichier
record-attach-hint = Jusqu'à { $mb } Mo. Les images sont affichées dans la galerie ; tout le reste est listé avec un lien de téléchargement.
record-no-documents = Aucun fichier n'est joint à cette fiche.

## Accès

access-restricted-title = Non visible pour vous
access-restricted-signed-in = La visibilité de cette fiche dépasse ce que votre compte peut lire. Un administrateur peut modifier soit la visibilité de la fiche, soit votre rôle.
access-restricted-anonymous = Cette fiche n'est pas publique. Connectez-vous pour voir si votre compte peut la lire.
access-role-title = Pas pour votre rôle
access-role-admin = Ceci est une page d'administrateur. Votre compte peut créer et modifier des fiches, mais pas gérer les comptes, supprimer des entités ni exporter le fichier.
access-role-write = Votre compte peut lire ce fichier mais pas le modifier. Un administrateur peut vous élever au rôle de contributeur.
access-scope-title = Hors de votre branche
access-scope-named = Votre compte est limité à une branche de l'arbre, et cette fiche concerne quelqu'un qui en est hors. Chaque personne nommée par une fiche doit se trouver dans votre branche — sinon une famille comportant un conjoint extérieur permettrait de réécrire la filiation de cette personne.
access-scope-unnamed = Votre compte est limité à une branche de l'arbre, et cette fiche ne nomme personne à qui la comparer. Les sources et les lieux sont modifiés par les comptes ayant accès à l'arbre entier.

## Erreurs

error-not-found-title = Introuvable
error-not-found-detail = Cette page n'existe pas dans ce fichier.
error-no-such-person-title = Personne inconnue
error-no-such-person-detail = Ce fichier ne contient aucune personne avec cet identifiant.
error-no-such-entity-title = Entité inconnue
error-no-such-entity-detail = Ce fichier ne contient aucune entité avec cet identifiant.
error-deleted-while-editing = Ce fichier ne contient aucune entité avec cet identifiant. Elle a peut-être été supprimée pendant que vous la modifiiez.
error-no-such-file-title = Fichier introuvable
error-no-such-file-detail = Ce fichier ne comporte aucun document avec cet identifiant, ou le document est enregistré sans fichier — un document « référencé » désigne quelque chose conservé ailleurs.
error-not-an-image-title = Ce n'est pas une image
error-not-an-image-detail = Il n'y a pas de vignette pour ce document, car ce n'est pas une image que cette version sait décoder.
error-back = Retour

## Connexion

login-title = Connexion
login-lede = Les comptes sont créés par un administrateur.
login-username = Nom d'utilisateur
login-password = Mot de passe
login-submit = Se connecter
login-wrong = Ce nom d'utilisateur et ce mot de passe ne correspondent pas.
login-token-wrong = Ce jeton n'est pas correct.
login-throttled = Trop de tentatives échouées. Attendez quelques minutes et réessayez.
login-no-accounts-title = Cette installation n'a encore aucun compte.
login-no-accounts-detail = Il n'y a délibérément pas de page d'installation ici — la fenêtre entre le déploiement et la première connexion est précisément le moment où une installation n'est pas protégée : le premier administrateur est donc créé en ligne de commande.
login-no-accounts-note = Un mot de passe généré est affiché une seule fois sur la sortie d'erreur. D'ici là, le seul accès est le jeton de secours ci-dessous.
login-emergency-summary = Accès de secours
login-emergency-detail = Le jeton partagé ouvre toujours une session d'administrateur, et il n'existe que pour cela : reprendre la main quand le fichier .acl a été perdu ou que tous les administrateurs sont bloqués. Ce n'est pas un compte — il ne possède aucune préférence, et le journal des modifications l'inscrit comme emergency-token et non comme une personne. Son usage est journalisé comme un avertissement.
login-emergency-label = Jeton de secours
login-emergency-submit = Utiliser le jeton de secours
login-sign-in-prompt = Connectez-vous pour accéder à l'administration.

## Administration

admin-title = Administration
admin-lede = Modification de { $path } — { $total } entités, { $files ->
        [one] un fichier joint
       *[other] { $files } fichiers joints
    }, { $size } sur le disque. Chaque modification est écrite de façon atomique ; une modification refusée laisse le fichier intact.
admin-entities = Entités
admin-create = Créer
admin-new-kind = Nouveau : { $kind }
admin-operations = Opérations
admin-validate = Valider
admin-deduplicate = Dédoublonner
admin-export = Exporter le fichier
admin-accounts = Comptes
admin-roles-note = Valider, dédoublonner, exporter, supprimer et gérer les comptes sont réservés aux administrateurs. Un contributeur atteint toutes les autres pages.
admin-dedup-confirm = Le dédoublonnage fusionne des entités et réécrit le fichier. Continuer ?
admin-recent-changes = Modifications récentes
admin-recent-note = Les { $shown } dernières sur { $total ->
        [one] une modification enregistrée
       *[other] { $total } modifications enregistrées
    }, depuis { $path }. Le journal vit à côté du fichier, pas dedans : un .axgf est copié et publié, et qui a corrigé quoi est un fait concernant les rédacteurs de cette famille et non la famille elle-même.
admin-sessions-open = { $n ->
        [one] Une session ouverte actuellement.
       *[other] { $n } sessions ouvertes actuellement.
    }
admin-no-changes-yet = Rien n'a encore été modifié via cette application. Chaque enregistrement à partir de maintenant est consigné dans { $path }.
admin-last-validation = Dernière validation
admin-bundle-heavy = Ce fichier fait { $size }. Le fichier entier est chargé en mémoire au démarrage et y reste : au-delà d'environ { $warn }, l'application commence à coûter de la mémoire réelle et les redémarrages deviennent lents. Cette conception convient à une archive familiale, pas à une médiathèque — si les pièces jointes croissent sans limite, elles ont leur place dans un stockage de fichiers que le .axgf référencerait.

admin-fields = Champs
admin-raw-json = JSON brut
admin-raw-json-help = L'entité entière, de sorte que rien ne soit non modifiable — les listes telles que les conjoints et les enfants d'une famille, ou l'histoire des frontières d'un lieu, se trouvent ici. C'est le document de départ ; les champs ci-dessus sont ensuite écrits par-dessus les chemins qui leur appartiennent : modifiez une valeur à un endroit ou à l'autre, pas aux deux. Cela doit être du JSON valide, sinon rien n'est enregistré.
admin-save = Enregistrer
admin-cancel = Annuler
admin-delete = Supprimer
admin-not-set = — non renseigné —
admin-edit = Modifier
admin-search = Rechercher
admin-page-of = Page { $page } sur { $pages }
admin-previous = Précédent
admin-next = Suivant
admin-nothing-here = Rien de ce type n'est encore enregistré dans ce fichier.
admin-saved = Enregistré en version { $version } — { $summary }
admin-not-saved = Non enregistré
admin-created = Créé
admin-not-created = Non créé
admin-deleted = Supprimé
admin-not-deleted = Non supprimé — le fichier est inchangé
admin-delete-policy = Intégrité référentielle
admin-what-changed = ce qui a changé
admin-field = Champ
admin-from = De
admin-to = À
admin-version = version { $version }

## Comptes

accounts-title = Comptes
accounts-lede = Enregistrés dans { $path }, en mode 600, à côté du fichier et jamais dedans. Un .axgf est copié, envoyé par courriel et publié ; des empreintes de mots de passe à l'intérieur feraient de chaque copie de l'arbre familial une copie du magasin d'identifiants.
accounts-existing = Existants
accounts-username = Nom d'utilisateur
accounts-role = Rôle
accounts-status = État
accounts-branch = Branche
accounts-last-seen = Dernière connexion
accounts-change = Modifier
accounts-you = (vous)
accounts-active = actif
accounts-disabled = désactivé
accounts-never = jamais
accounts-whole-tree = arbre entier
accounts-roots = { $n ->
        [one] une racine
       *[other] { $n } racines
    }
accounts-add = Ajouter un compte
accounts-no-registration = Il n'y a délibérément ni inscription libre ni système d'invitation. Pour une archive familiale, un administrateur qui connaît tout le monde suffit, et cela supprime entièrement une surface d'abus au lieu d'avoir à la défendre.
accounts-password-hint = Laissez vide et un mot de passe est généré puis affiché une seule fois. Au moins { $min } caractères si vous le définissez vous-même.
accounts-new-password-placeholder = nouveau mot de passe (vide = inchangé)
accounts-email = Courriel
accounts-optional = (facultatif)
accounts-create = Créer le compte
accounts-role-viewer = lecteur — lit les fiches publiques et « membres »
accounts-role-contributor = contributeur — crée, modifie et téléverse également
accounts-role-admin = administrateur — gère aussi les comptes, supprime et exporte
accounts-branch-hint = Limite ce que ce compte peut modifier à ces personnes, leurs descendants et leurs conjoints. Cela ne limite pas ce qu'il peut lire — ceci relève de la visibilité de chaque fiche, et les deux sont volontairement séparés.
accounts-branch-placeholder = un identifiant de personne par ligne
accounts-ids-in-bundle = Identifiants de personnes dans ce fichier
accounts-emergency-warning = Vous êtes connecté avec le jeton de secours. Il accorde les droits d'administrateur pour cette session mais n'est pas un compte : il ne possède aucune préférence, et le journal des modifications enregistrera vos changements comme emergency-token et non comme une personne. Créez-vous un vrai compte ci-dessous et connectez-vous avec.
accounts-created-with-password = { $username } créé. Son mot de passe est { $password } — il n'est affiché qu'une fois et n'est conservé que sous forme d'empreinte Argon2id : transmettez-le maintenant.
accounts-created = { $username } créé.
accounts-updated = { $username } modifié. Toute session ouverte a été déconnectée.
accounts-username-taken = Ce nom d'utilisateur est déjà pris.
accounts-pick-role = Choisissez un rôle.
accounts-no-such = Aucun compte de ce nom.
accounts-last-admin = C'est le seul administrateur actif. Promouvez d'abord quelqu'un d'autre — une installation sans administrateur ne peut être récupérée qu'en modifiant le fichier .acl ou en utilisant le jeton de secours.
accounts-not-saved = Non enregistré : { $error }

## Conflits

conflict-title = Quelqu'un d'autre a modifié ceci avant vous
conflict-lede = { $who } a enregistré une modification de cette entité ({ $kind }) à { $when }, après que vous l'avez ouverte. Votre modification n'a pas été enregistrée, et rien n'a été écrasé.
conflict-no-merge = Rien n'est fusionné automatiquement ici. Fusionner les modifications de deux personnes produit une fiche qu'aucune des deux n'a choisie, et en généalogie, deux rédacteurs en désaccord sur une date lisent généralement des sources différentes — ce qui est une question pour une personne, pas pour un programme. Comparez les deux ci-dessous et décidez.
conflict-versions = Vous êtes parti de la version { $expected } ; le fichier contient désormais la version { $current }.
conflict-both-changed = Vous avez tous les deux modifié ceci
conflict-both-changed-detail = Ces champs ont été modifiés par vous deux. Ce que vous enregistrerez remplacera ce que { $who } y a mis :
conflict-different-fields = Vous avez modifié des champs différents : rien du travail de { $who } n'est donc contesté — mais réappliquer écrit tout de même votre entité entière par-dessus la sienne. Vérifiez les deux colonnes avant d'enregistrer.
conflict-field-by-field = Champ par champ
conflict-theirs = Ce que { $who } y a mis
conflict-yours = Ce que vous y avez mis
conflict-unchanged-by-you = non modifié par vous
conflict-unchanged-by-them = non modifié par eux
conflict-nothing-differs = Aucune des deux versions ne diffère de celle dont vous êtes parti dans un champ affiché par cette page. Le numéro de version a changé : quelqu'un a donc enregistré la fiche sans modifier ce qu'elle contient.
conflict-what-now = Et maintenant
conflict-reapply = Réappliquer votre version par-dessus la leur
conflict-reapply-hint = Voici votre modification, reportée sur la version { $version }. Modifiez-la ici pour conserver ce que vous voulez du travail de { $who }, puis enregistrez. Leur version est affichée ci-dessous pour y puiser.
conflict-save-over = Enregistrer ceci par-dessus la leur
conflict-discard = Abandonner la mienne et recommencer
conflict-their-version = La version de { $who }, telle que le fichier la contient actuellement
conflict-history-of = Historique de cette entité ({ $kind })

## Accueil

home-lede = { $family } — { $total ->
        [one] une entité
       *[other] { $total } entités
    } dans un seul fichier .axgf.
home-why-title = Pourquoi AXGF
home-what-this-bundle-has = Ce que ce fichier contient réellement
home-browse-tree = Parcourir l'arbre
home-convert-gedcom = Convertir un GEDCOM
home-see-example = Voir un exemple

## Conversion

convert-title = Convertir un GEDCOM en AXGF
convert-choose-file = Fichier GEDCOM
convert-submit = Convertir
convert-result-title = Résultat de la conversion
convert-download = Télécharger le fichier .axgf
convert-diagnostics = Ce que le convertisseur a signalé
convert-unchanged-note = La conversion ne touche jamais au fichier servi par ce site.

## Complétude

completeness-title = Complétude du fichier
completeness-recorded = renseigné
completeness-empty = vide
completeness-spec-field = Champ AXGF

## Dates

date-unknown = Date inconnue
date-not-recorded = Non renseignée
date-circa = vers { $date }
date-between = entre { $from } et { $to }
date-before = avant { $date }
date-after = après { $date }
date-preserved = enregistré tel quel : « { $text } »
date-day-month-year = { $day } { $month } { $year }
date-month-year = { $month } { $year }
date-decade = les années { $decade }
date-century = le { $century ->
        [1] Ier
       *[other] { $century }e
    } siècle
date-quarter-century = le { $quarter ->
        [1] premier
        [2] deuxième
        [3] troisième
       *[other] quatrième
    } quart du { $century ->
        [1] Ier
       *[other] { $century }e
    } siècle

month-1 = janvier
month-2 = février
month-3 = mars
month-4 = avril
month-5 = mai
month-6 = juin
month-7 = juillet
month-8 = août
month-9 = septembre
month-10 = octobre
month-11 = novembre
month-12 = décembre
