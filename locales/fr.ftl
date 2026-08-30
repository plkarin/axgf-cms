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
nav-convert = Importer
nav-admin = Administration
nav-sign-in = Se connecter
nav-sign-out = Se déconnecter

## Préférences

prefs-title = Langue et apparence
prefs-language = Langue
prefs-language-note = Ceci ne change que l'interface. Les noms, les lieux et les notes sont toujours affichés dans leur propre langue et leur propre écriture.
prefs-theme = Apparence
prefs-apply = Appliquer
prefs-reviewed = relu
prefs-machine = automatique, { $coverage } %
prefs-machine-title = Traduit sans relecture par une personne dont c’est la langue maternelle. Le vocabulaire généalogique en particulier peut être fautif — les mots pour une union, un parrain ou une source primaire varient selon la tradition archivistique de chaque pays. Les corrections sont bienvenues ; CONTRIBUTING.md indique par où commencer.

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
tree-lede-whole = Toutes les personnes de l’arbre. Les plus anciennes en bas, les plus jeunes en haut. L’opacité d’un trait est le degré de certitude du lien.
tree-filter-label = Filtrer les fiches affichées
tree-filter-placeholder = Saisissez un nom…
tree-centre-on = Centrer sur
tree-depth = Générations de chaque côté
tree-show = Afficher
tree-hidden-notice = { $n ->
        [one] Une personne est affichée sans ses informations
       *[other] { $n } personnes sont affichées sans leurs informations
    }
tree-hidden-because-role = , car leur visibilité dépasse ce que votre compte peut lire.
tree-hidden-because-anonymous = , car elles ne sont pas publiques.
tree-hidden-sign-in = Connectez-vous si vous avez un compte.
tree-restricted-card = La fiche de cette personne ne vous est pas visible
tree-empty = Il n’y a encore personne à dessiner.
tree-unplaced = Dans aucune famille enregistrée

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
record-history-help = Chaque modification enregistrée sur cette fiche, la plus récente en premier. Qui a corrigé quoi est un fait sur les personnes qui tiennent l’arbre, non sur la famille qui s’y trouve : c’est donc tenu hors de l’archive exportée et montré seulement aux proches connectés.
record-raw-help = Rien ici n’est produit pour l’affichage : c’est la fiche telle qu’elle est enregistrée, jusqu’aux noms des champs. Si vous deviez un jour lire l’archive sans ce site, voici ce que vous verriez.
record-help-toggle = Ce que montre cette section

record-gender = Genre
record-living = Vivant
record-visibility = Visibilité
record-yes = oui
record-no = non
record-name-type = Type de nom
record-name-used = Porté
record-name-evidence = Preuve
record-transliteration = Translittération latine
record-born = Né(e)
record-died = Décédé(e)
record-parents = Parents
record-siblings = Fratrie
record-children = Enfants
record-unknown-person = [Inconnu]
record-restricted-person = Privé
record-restricted-title = La fiche de cette personne ne vous est pas visible
record-absent-person-title = Nommée dans cet arbre mais sans fiche
record-confidence = Degré de certitude
record-source = Source
record-download = Télécharger

## Accès

access-restricted-title = Non visible pour vous
access-restricted-signed-in = La visibilité de cette fiche dépasse ce que votre compte peut lire. Un administrateur peut modifier soit la visibilité de la fiche, soit votre rôle.
access-restricted-anonymous = Cette fiche n'est pas publique. Connectez-vous pour voir si votre compte peut la lire.
access-role-title = Pas pour votre rôle
access-role-admin = Ceci est une page d’administrateur. Votre compte peut créer et modifier des fiches, mais pas gérer les comptes, supprimer des fiches ni exporter l’archive.
access-role-write = Votre compte peut lire cet arbre mais pas le modifier. Un administrateur peut vous passer contributeur.
access-scope-title = Hors de votre branche
access-scope-named = Votre compte est limité à une branche de l'arbre, et cette fiche concerne quelqu'un qui en est hors. Chaque personne nommée par une fiche doit se trouver dans votre branche — sinon une famille comportant un conjoint extérieur permettrait de réécrire la filiation de cette personne.
access-scope-unnamed = Votre compte est limité à une branche de l'arbre, et cette fiche ne nomme personne à qui la comparer. Les sources et les lieux sont modifiés par les comptes ayant accès à l'arbre entier.

## Erreurs

error-not-found-title = Introuvable
error-not-found-detail = Cette page n’existe pas ici.
error-no-such-person-title = Personne inconnue
error-no-such-person-detail = Aucune personne ici ne porte cet identifiant.
error-no-such-entity-title = Entité inconnue
error-no-such-entity-detail = Aucune fiche ici ne porte cet identifiant.
error-deleted-while-editing = Aucune fiche ici ne porte cet identifiant. Elle a peut-être été supprimée pendant que vous la modifiiez.
error-no-such-file-title = Fichier introuvable
error-no-such-file-detail = Aucun document ici ne porte cet identifiant, ou le document est enregistré sans fichier — un document référencé désigne quelque chose conservé ailleurs.
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
admin-export = Exporter l’archive
admin-accounts = Comptes
admin-roles-note = Valider, dédoublonner, exporter, supprimer et gérer les comptes sont réservés aux administrateurs. Un contributeur atteint toutes les autres pages.
admin-dedup-confirm = La déduplication fusionne des fiches et réécrit l’archive. Continuer ?
admin-recent-changes = Modifications récentes
admin-recent-note = Les { $shown } dernières sur { $total ->
        [one] une modification enregistrée
       *[other] { $total } modifications enregistrées
    }, depuis { $path }. Le journal est conservé à côté de l’archive plutôt que dedans : une archive se copie, s’envoie et se publie, et qui a corrigé quoi est un fait sur les personnes qui tiennent l’arbre, non sur la famille qui s’y trouve.
admin-sessions-open = { $n ->
        [one] Une session ouverte actuellement.
       *[other] { $n } sessions ouvertes actuellement.
    }
admin-no-changes-yet = Rien n'a encore été modifié via cette application. Chaque enregistrement à partir de maintenant est consigné dans { $path }.
admin-last-validation = Dernière validation
admin-bundle-heavy = Cette archive pèse { $size }. L’ensemble est chargé au démarrage et gardé en mémoire : au-delà d’environ { $warn }, le site commence à coûter de la mémoire réelle et les redémarrages deviennent lents. Cela convient à une archive familiale, pas à une médiathèque — si les pièces jointes grossissent sans limite, conservez-les dans un espace de fichiers et faites-les pointer depuis l’archive.

admin-fields = Champs
admin-raw-json = JSON brut
admin-raw-json-help = L'entité entière, de sorte que rien ne soit non modifiable — les listes telles que les conjoints et les enfants d'une famille, ou l'histoire des frontières d'un lieu, se trouvent ici. C'est le document de départ ; les champs ci-dessus sont ensuite écrits par-dessus les chemins qui leur appartiennent : modifiez une valeur à un endroit ou à l'autre, pas aux deux. Cela doit être du JSON valide, sinon rien n'est enregistré.
admin-save = Enregistrer
admin-cancel = Annuler
admin-delete = Supprimer
admin-not-set = — non renseigné —
admin-edit = Modifier
admin-page-of = Page { $page } sur { $pages }
admin-previous = Précédent
admin-next = Suivant
admin-saved = Enregistré en version { $version } — { $summary }
admin-not-saved = Non enregistré
admin-created = Créé
admin-not-created = Non créé
admin-deleted = Supprimé
admin-not-deleted = Non supprimé — rien n’a été modifié
admin-what-changed = ce qui a changé
admin-field = Champ
admin-from = De
admin-to = À
admin-version = version { $version }

## Comptes

accounts-title = Comptes
accounts-lede = Enregistré dans { $path }, en mode 600, à côté de l’archive et jamais dedans. Une archive se copie, s’envoie et se publie ; des empreintes de mots de passe voyageant à l’intérieur feraient de chaque copie de l’arbre familial une copie des identifiants.
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
accounts-ids-in-bundle = Identifiants de personnes dans cet arbre
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
conflict-versions = Vous êtes parti de la version { $expected } ; la fiche porte maintenant la version { $current }.
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
conflict-their-version = La version de { $who }, telle qu’elle est actuellement
conflict-history-of = Historique de cette entité ({ $kind })

## Conversion

convert-title = Importer un fichier familial
convert-submit = Importer
convert-result-title = Rapport d’importation
convert-download = Télécharger l’archive

## Dates

date-unknown = Date inconnue
date-not-recorded = Non renseignée
date-circa = vers { $date }
date-between = entre { $from } et { $to }
date-before = avant { $date }
date-after = après { $date }
date-preserved = enregistré tel quel : « { $text } »
date-day-month-year = { $day } { $month ->
        [1] janvier
        [2] février
        [3] mars
        [4] avril
        [5] mai
        [6] juin
        [7] juillet
        [8] août
        [9] septembre
        [10] octobre
        [11] novembre
        [12] décembre
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] janvier
        [2] février
        [3] mars
        [4] avril
        [5] mai
        [6] juin
        [7] juillet
        [8] août
        [9] septembre
        [10] octobre
        [11] novembre
        [12] décembre
        *[other] { $month }
    } { $year }
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

## Erreurs, suite

error-back-to-start = Retour à l'accueil
error-payload-missing-title = Fichier introuvable
error-payload-missing-detail = Le contenu de ce document n'est pas dans le cache.
error-payload-unopenable-detail = Le contenu de ce document n'a pas pu être ouvert.
error-no-such-document-detail = Aucun document ici ne porte cet identifiant.
error-bad-preference-title = Ce n'est pas un des choix proposés
error-bad-preference-detail = Ce n'est ni une langue ni un thème que ce site propose. Rien n'a été modifié.
error-unknown-kind-title = Type inconnu
error-unknown-kind-detail = « { $kind } » n’est pas un type de fiche. Cette archive contient : { $kinds }.
error-io-title = Enregistrement impossible
error-io-detail = { $error }. Rien n’a été modifié sur le disque.
error-upload-too-large = Ce fichier dépasse la limite de { $mb } Mo. Rien n'a été enregistré et le fichier est inchangé.
error-upload-refused = La bibliothèque a refusé le document : { $reason }. Le fichier est inchangé.
error-back-to-person = Retour à la fiche
error-no-such-person-to-attach = Aucune personne ici ne porte cet identifiant : il n’y a donc rien à quoi rattacher un document.
error-upload-title = Ce téléversement n'a pas été enregistré
error-download-expired-title = Ce téléchargement a expiré
error-download-expired-detail = Une importation est conservée quinze minutes, puis supprimée. Importez le fichier à nouveau.
error-upload-none = Aucun fichier n'a été envoyé. Choisissez-en un d'abord.
error-upload-unsupported = Ce type de fichier n'est pas conservé par cette archive. Les images, les PDF, le texte brut, l'audio et la vidéo sont acceptés ; le type est déterminé d'après les octets du fichier lui-même, si bien que renommer un exécutable ne le fait pas passer. Le SVG est refusé sans exception, car un SVG peut porter du script.
error-export-unreadable-title = Impossible de lire l’archive exportée
error-export-unreadable-detail = { $error }

## Arbre, suite

tree-title-suffix = arbre
tree-back-to-focused = Revenir à la vue centrée
tree-show-all = Afficher les { $n }
tree-width-notice = Cette vue fait { $width } px de large. Chaque génération occupe une ligne, et c'est la plus large qui fixe cette largeur — sur un écran de 1500 px cela représente { $screens ->
        [one] un écran
       *[other] { $screens } écrans
    } de défilement horizontal. La vue centrée montre plutôt quelques dizaines de personnes autour d'une seule, et chaque fiche la recentre.
tree-confidence-label = Degré de certitude :
tree-band-certain = certain
tree-band-high = élevé
tree-band-medium = moyen
tree-band-low = spéculatif
tree-counts = { $drawn } personnes sur { $total } · { $generations ->
        [one] une génération
       *[other] { $generations } générations
    }
tree-unplaced-count = { $n } sans place
tree-contradicts-title = Cet arbre se contredit.
tree-contradicts-detail = Quelqu'un y est enregistré comme son propre ancêtre, ou deux personnes d'une même ligne de descendance y forment un couple. Aucune disposition des lignes ne peut satisfaire cela : la relation fautive a donc été écartée de la numérotation des générations et certaines lignes peuvent être erronées. Lancez le validateur depuis le tableau de bord pour la trouver.
tree-no-people = Il n’y a encore personne dans cet arbre.
tree-no-people-cta = Importez un fichier familial, ou ajoutez la première personne.
tree-nobody-selected = Personne à dessiner pour cette sélection.
tree-nobody-selected-cta = Repartir de la vue par défaut.
tree-click-hint = Cliquez sur une fiche pour ouvrir la personne dans le panneau ; « Centrer l'arbre ici », dans le panneau, redéfinit la racine de la vue.
tree-edge-union = Une union enregistrée
tree-edge-parentage = Une filiation enregistrée

## Accueil, suite

home-empty = Rien d’enregistré pour l’instant. Importez un fichier familial pour reprendre un arbre existant, ou ajoutez la première personne à la main.
home-count = { $total ->
        [one] Une fiche
       *[other] { $total } fiches
    }, dans un seul fichier qui appartient à la famille.
home-browse = Parcourir l’arbre
home-convert = Importer un fichier familial
home-showcase-title = Là où cet arbre dit déjà plus que des noms et des dates
home-showcase-note = Chacun de ces points vient de ce qui est réellement enregistré ici, non d’une liste de ce que le site saurait faire.
home-showcase-example = Voir un exemple →
home-nothing-title = Rien à montrer pour l'instant.
home-nothing-detail = Importez un fichier familial pour reprendre un arbre existant, ou partez de rien et ajoutez vous-même la première personne.

showcase-links-title = { $n ->
        [one] Une relation hors de la famille
       *[other] { $n } relations hors de la famille
    }
showcase-links-detail = Parrains, employeurs, témoins et mentors, chacun avec ses propres dates, sa source et votre degré de certitude.
showcase-occupations-title = { $n ->
        [one] Un métier avec un début et une fin
       *[other] { $n } métiers avec un début et une fin
    }
showcase-occupations-detail = « Institutrice, 1948-1978 » garde sa durée : cela se dessine comme une barre à travers les années, et non comme une seule ligne datée.
showcase-uncertain-title = { $n ->
        [one] Une date laissée aussi incertaine qu’elle a été donnée
       *[other] { $n } dates laissées aussi incertaines qu’elles ont été données
    }
showcase-uncertain-detail = Vers, avant, après et entre restent quatre affirmations distinctes. Une date que la source n’a pas su fixer n’est jamais présentée comme si elle l’avait été.
showcase-preserved-title = { $n ->
        [one] Une date conservée dans les mots où elle a été écrite
       *[other] { $n } dates conservées dans les mots où elles ont été écrites
    }
showcase-preserved-detail = Une formulation que personne n’a su lire comme une date est conservée telle quelle, plutôt que discrètement supprimée.
showcase-sources-title = { $n ->
        [one] Une source dont la fiabilité est enregistrée
       *[other] { $n } sources dont la fiabilité est enregistrée
    }
showcase-sources-detail = { $primary ->
        [one] Une source primaire.
       *[other] { $primary } primaires.
    } Chaque fait indique sur quelle preuve il repose, et la force de cette preuve.
showcase-places-title = { $n ->
        [one] Un lieu dont les frontières ont bougé
       *[other] { $n } lieux dont les frontières ont bougé
    }
showcase-places-detail = Une ville peut appartenir à différents pays selon l'époque, et le dossier dit lequel s'appliquait quand.

## Fiche, détails

record-also-recorded-as = également enregistré comme
record-borders-moved = Frontières déplacées :
record-display-name = nom d'usage
record-read-as = se lit
record-note = Note
record-living-yes = vivant
record-deceased = décédé
record-centre-tree-here = Centrer l'arbre ici
record-centre-tree-title = Déplacer l'arbre pour le centrer sur cette personne
record-open-full-page = Ouvrir la page entière ↗
record-open-full-title = Ouvrir la page autonome et partageable
record-edit = Modifier
panel-empty = Sélectionnez une fiche pour voir ici le dossier complet de cette personne.
person-see-in-tree = Voir cette personne dans l'arbre
person-visibility-inline = visibilité :
person-sections-label = Sections de cette page

result-diagnostics = Diagnostics
result-diagnostics-note = Tous les diagnostics renvoyés par la bibliothèque, y compris les avertissements qui n'ont pas bloqué l'opération. Aucun n'est filtré.
result-no-diagnostics = La bibliothèque n'a renvoyé aucun diagnostic.
result-continue = Continuer
result-dashboard = Tableau de bord

record-name = Nom
record-type = Type
record-cause = Cause :
record-as = comme
record-partner-not-recorded = Conjoint non enregistré
record-union-from = À partir de
record-union-at = à
record-union-until = jusqu'à
record-occupation-from = de
record-occupation-until = à
record-source-reliability = Fiabilité
record-source-supports = Étaye
record-photographs = Photographies
record-documents = Documents
record-file = Fichier
record-status = État
record-size = Taille
record-absent-document = Nommé par cette personne mais absent d’ici.
record-no-file = aucun fichier
record-attach-document = Joindre un document
record-doc-photo = photographie
record-doc-certificate = acte
record-doc-letter = lettre
record-doc-record = registre
record-doc-newspaper = journal
record-doc-other = autre
record-upload = Téléverser
record-upload-help = Jusqu’à { $mb } Mo par fichier. Les pièces jointes sont conservées à côté de l’arbre et réintégrées dans l’archive à l’export, de sorte qu’une photographie voyage avec la famille à laquelle elle appartient. Le type est lu dans le contenu du fichier, non dans son nom : images, PDF, texte brut, audio et vidéo sont acceptés. Le SVG est refusé, car un SVG peut porter un script.
record-upload-help-short = Jusqu'à { $mb } Mo. Le SVG est refusé.
record-verbatim-note = Conservé exactement tel que le dossier l'énonçait, parce qu'aucun convertisseur n'a su l'interpréter. L'autre solution aurait été de le supprimer.
record-file-to-attach = Fichier à joindre
record-document-type = Type de document
record-caption = Légende
record-caption-placeholder = Légende (facultative)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }
record-occupations-help-undated = Un métier s’enregistre avec un début et une fin, ce qui permet d’en comparer plusieurs sur un même axe. Cette archive contient les intitulés mais aucune date — c’est courant après une importation, la plupart des fichiers familiaux n’ayant nulle part où les mettre — donc il n’y a pas encore d’échelle à dessiner.
record-occupations-help-axis = Un métier est un état avec une durée, non un événement à date unique. Toutes les périodes partagent un même axe, { $from }-{ $to }.
record-birth-order = ordre de naissance
record-start-not-recorded = début non enregistré
record-end-not-recorded = fin non enregistrée
record-document-no-file = Le document est enregistré ici, mais le fichier lui-même n’est pas conservé
panel-selected-person = Personne sélectionnée

## Types d'entités

kind-person = personne
kind-family = famille
kind-event = événement
kind-link = lien
kind-occupation = métier
kind-source = source
kind-place = lieu
kind-document = document

kind-person-plural = { $n ->
        [one] personne
       *[other] personnes
    }
kind-family-plural = { $n ->
        [one] famille
       *[other] familles
    }
kind-event-plural = { $n ->
        [one] événement
       *[other] événements
    }
kind-link-plural = { $n ->
        [one] lien
       *[other] liens
    }
kind-occupation-plural = { $n ->
        [one] métier
       *[other] métiers
    }
kind-source-plural = { $n ->
        [one] source
       *[other] sources
    }
kind-place-plural = { $n ->
        [one] lieu
       *[other] lieux
    }
kind-document-plural = { $n ->
        [one] document
       *[other] documents
    }

## Listes

list-matching = { $total ->
        [one] Un résultat
       *[other] { $total } résultats
    }, { $per_page } par page.
list-filter-placeholder = Filtrer par nom ou identifiant
list-filter = Filtrer
list-clear = Effacer
list-summary = Résumé
list-id = Identifiant
list-actions = Actions
list-nothing = Rien ici.
list-nothing-matching = Rien ici ne correspond à « { $q } ».
list-delete-confirm = Supprimer ce/cette { $kind } ? Choisissez ce qu'il advient des entités qui le référencent :
list-policy-reject = Refuser
list-policy-reject-detail = — refuser tant que quelque chose le référence encore. Rien n'est perdu.
list-policy-cascade = Cascade
list-policy-cascade-detail = — le supprimer et retirer physiquement chaque référence à lui.
list-policy-orphan = Orphelin
list-policy-orphan-detail = — le supprimer mais conserver les dossiers qui le citent, le lien mis à null.

## Complétude, suite

completeness-dates-title = Les dates, selon la forme qu'elles ont réellement
completeness-no-dates = Aucune date enregistrée pour l’instant.
completeness-dates-note = Une date que quelqu'un a pu fixer au jour près et une date qu'il n'a pu situer que dans une décennie sont deux affirmations différentes, et toutes deux sont conservées telles quelles. Un texte qu'on n'a pas pu lire comme une date est préservé mot pour mot plutôt que supprimé.
completeness-shape-exact = exacte
completeness-shape-exact-note = un jour de calendrier complet
completeness-shape-approximate = approximative
completeness-shape-approximate-note = vers, ou une année ou décennie seulement
completeness-shape-ranged = par bornes
completeness-shape-ranged-note = avant, après ou entre
completeness-shape-preserved = préservée
completeness-shape-preserved-note = texte illisible, conservé mot pour mot
completeness-shape-unknown = inconnue
completeness-shape-unknown-note = enregistrée comme non connue

## Conversion, suite

convert-page-title = Importer un fichier familial
convert-lede = Reprenez un arbre existant depuis un fichier GEDCOM — l’export que produisent la plupart des logiciels de généalogie. Vous récupérez une archive à conserver. Rien n’est stocké ici, et l’arbre que ce site montre déjà reste exactement tel quel.
convert-file-label = Fichier familial (.ged)
convert-file-hint = Jusqu’à { $mb } Mo. Un arbre de 767 personnes pèse environ 320 Ko.
convert-confidence-label = Degré de certitude de départ
convert-confidence-hint = Le fichier importé ne dit pas à quel point quelqu’un était sûr : chaque fait a donc besoin d’un point de départ. Choisissez une valeur basse pour un arbre monté rapidement, plus haute pour un arbre travaillé sur documents. La lecture honnête de ce nombre est « importé, et vérifié par personne depuis » — vous pourrez relever ou abaisser chaque fait ensuite, un par un.
convert-lang-label = Langue des noms de lieux
convert-lang-hint = Une étiquette comme en, fr ou pl. Un lieu peut porter son nom en plusieurs langues ; ceci indique dans quelle langue sont écrits les noms de votre fichier.
convert-what-you-get = Ce que l’importation ajoute
convert-what-you-get-1 = Chaque fait reçoit un degré de certitude que vous pourrez ajuster plus tard : un doute s’écrit au lieu de se perdre. Les dates gardent leur forme : vers 1500, avant 1430 et entre 1920 et 1925 restent trois affirmations différentes, et une formulation que personne n’a su lire comme une date est conservée mot pour mot. Un métier devient une durée, avec un début et une fin. Chaque lieu devient une entrée à part entière, si bien qu’une ville ayant changé de pays garde cette histoire.
convert-no-way-back = Réécrire un fichier .ged n’est pas proposé. Ce format n’a nulle part où mettre le degré de certitude d’un fait, une relation hors de la famille, la durée d’un métier ou une date que personne n’a su fixer — le trajet retour les perdrait donc en silence. Votre archive s’exporte entière, en un seul fichier.
convert-failed = L’importation n’a pas abouti
convert-try-another = Essayer un autre fichier
convert-converted = { $filename } importé
convert-result-lede = { $total ->
        [one] Une fiche
       *[other] { $total } fiches
    }, { $size } Ko. Tout est entré avec un degré de certitude de { $confidence }, les noms de lieux étant lus comme du { $lang }. L’arbre que montre ce site n’a pas été touché.
convert-produced = Ce qui est passé
convert-skipped-title = { $n ->
        [one] Une entrée illisible
       *[other] { $n } entrées illisibles
    }
convert-skipped-note = Ces entrées ne contenaient rien qui puisse être repris. Elles sont listées plutôt qu’escamotées : savoir exactement ce qui est resté en arrière fait la différence entre une importation à laquelle on peut se fier et une autre non.
convert-other-diagnostics = { $n ->
        [one] Une autre chose à savoir
       *[other] { $n } autres choses à savoir
    }
convert-clean = Rien n’est resté en arrière — chaque entrée du fichier est passée.
convert-download-title = Téléchargement
convert-download-named = Télécharger { $name }
convert-download-note = Conservé ici quinze minutes puis supprimé : téléchargez-le maintenant. Ce fichier unique, c’est tout l’arbre ; gardez-le en lieu sûr.
convert-another = Importer un autre fichier

## Administration, suite

admin-history-on = sur
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Une erreur
       *[other] { $errors } erreurs
    }, { $warnings ->
        [one] un avertissement
       *[other] { $warnings } avertissements
    }, { $infos ->
        [one] une note
       *[other] { $infos } notes
    }.
admin-warnings-never-block = Les avertissements ne bloquent jamais — ce sont des informations, pas des barrières.
admin-validator-clean = Le validateur n'a rien signalé.
admin-validation-report = Rapport de validation
admin-dedup-complete = Dédoublonnage terminé
admin-dedup-refused = Dédoublonnage refusé
admin-value-not-set = non renseigné

## Bandes de l'arbre

tree-band-generation = Génération { $g }
tree-band-people = { $n ->
        [one] une personne
       *[other] { $n } personnes
    }
tree-band-unplaced = Sans place
tree-band-unplaced-note = { $n ->
        [one] une personne dans aucune famille — affichée plutôt qu'omise
       *[other] { $n } personnes dans aucune famille — affichées plutôt qu'omises
    }

## Vocabulaire contrôlé

gender-M = Masculin
gender-F = Féminin
gender-NB = Non binaire
gender-unrecorded = Non renseigné

name-part-given_name = prénom
name-part-family_name = nom de famille
name-part-patronymic = patronyme
name-part-matronymic = matronyme
name-part-middle_name = deuxième prénom
name-part-nickname = surnom
name-part-prefix = particule initiale
name-part-suffix = suffixe
name-part-particle = particule
name-part-part = élément

name-type-primary = principal
name-type-other = autre
name-type-alias = alias
name-type-birth = de naissance
name-type-married = d'épouse
name-type-religious = de religion
name-type-transliteration = translittération
name-type-nickname = surnom

## Notes de mise en avant

note-links = { $n ->
        [one] une relation hors de la famille, avec ses propres dates et sources
       *[other] { $n } relations hors de la famille, avec leurs propres dates et sources
    }
note-occupations = { $n ->
        [one] un métier enregistré avec un début et une fin
       *[other] { $n } métiers enregistrés avec un début et une fin
    }
note-birth-imprecise = une date de naissance que la source n'a pas su fixer, montrée telle qu'enregistrée
note-death-imprecise = une date de décès que la source n'a pas su fixer, montrée telle qu'enregistrée
note-names = { $n ->
        [one] un nom enregistré
       *[other] { $n } noms enregistrés
    }
note-transliteration = un nom dans son écriture d'origine à côté de sa translittération latine
note-witnessed = { $n ->
        [one] un événement dont elle a été témoin sans en être le sujet
       *[other] { $n } événements dont elle a été témoin sans en être le sujet
    }

visibility-public = public
visibility-members = membres
visibility-contributors = contributeurs
visibility-private = privé

## Résumés des listes d'administration

family-label-couple = { $children ->
        [0] { $a } & { $b }
        [one] { $a } & { $b } — un enfant
       *[other] { $a } & { $b } — { $children } enfants
    }
family-label-half = { $children ->
        [0] { $a } & { $unknown }
        [one] { $a } & { $unknown } — un enfant
       *[other] { $a } & { $unknown } — { $children } enfants
    }
family-label-children = { $others ->
        [0] { $first } — parents non enregistrés
        [one] { $first } et un frère ou une sœur — parents non enregistrés
       *[other] { $first } et { $others } frères et sœurs — parents non enregistrés
    }
family-label-empty = Famille sans personne enregistrée

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } & { $b }
event-more-people = { $a } & { $b } et { $others ->
        [one] une autre personne
       *[other] { $others } autres personnes
    }

link-label = { $label } : { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } sans titre
list-unnamed = { $kind } sans nom

event-category-birth = Naissance
event-category-death = Décès
event-category-marriage = Mariage
event-category-divorce = Divorce
event-category-baptism = Baptême
event-category-burial = Inhumation
event-category-immigration = Immigration
event-category-emigration = Émigration
event-category-census = Recensement
event-category-residence = Résidence
event-category-military = Service militaire
event-category-education = Études
event-category-other = Événement

reliability-primary = source primaire
reliability-secondary = source secondaire
reliability-tertiary = source tertiaire
reliability-recollection = souvenir
reliability-derivative = travail dérivé
reliability-authored = ouvrage d’auteur
reliability-oral = tradition orale
reliability-unknown = fiabilité inconnue

document-type-photo = photographie
document-type-certificate = acte
document-type-letter = lettre
document-type-record = registre
document-type-newspaper = coupure de presse
document-type-other = document

## Là où l'arbre pourrait en dire plus

completeness-title = Là où cet arbre pourrait en dire plus
completeness-intro = Ce qui est enregistré et ce qui reste vide. Rien ici n'est une erreur : une ligne vide est un endroit où le dossier peut grandir, non quelque chose qui a échoué.
completeness-import-title = Ce que l'importation a apporté
completeness-import-intro = Compté à partir du fichier que vous venez d'envoyer. Une ligne vide correspond à ce que le fichier d'origine n'enregistrait pas — ce n'est pas quelque chose que l'importation a perdu.
completeness-what-is-recorded = Ce que le dossier peut dire
completeness-in-this-tree = Dans cet arbre
completeness-not-yet = pas encore enregistré

completeness-headline-full = Chaque type de détail ci-dessous est enregistré quelque part dans cet arbre.
completeness-headline-empty = { $total ->
        [one] Le seul type de détail ci-dessous n'est encore enregistré nulle part.
       *[other] Aucun des { $total } types de détail ci-dessous n'est encore enregistré.
    } Chacun est un endroit où le dossier pourrait en dire plus.
completeness-headline-partial = { $carried ->
        [one] Un type de détail ci-dessous est enregistré
       *[other] { $carried } types de détail ci-dessous sont enregistrés
    } ; { $empty ->
        [one] un reste vide
       *[other] { $empty } restent vides
    }.

completeness-metric-confidence = À quel point chaque fait est sûr
completeness-metric-confidence-none = Aucun des { $slots } faits ici ne dit à quel point il est sûr. Une date relevée sur un acte et une date supposée se ressemblent, jusqu'au jour où ce n'est plus le cas.
completeness-metric-confidence-uniform = { $with } faits sur { $slots } portent un degré de certitude, et tous la même valeur ({ $modal }). C'est ce que laisse une importation en masse : une valeur par défaut que personne n'a reprise. Aucun n'a encore été jugé individuellement.
completeness-metric-confidence-some = { $with } faits sur { $slots } portent un degré de certitude. { $modal_count } partagent une même valeur ({ $modal }) ; { $assessed } s'en écartent et ont donc été examinés un par un.
completeness-metric-confidence-many = { $with } faits sur { $slots } portent un degré de certitude, dont { $assessed } s'écartent de la valeur la plus courante ({ $modal }), sur { $distinct } niveaux distincts. Cet arbre enregistre une incertitude réelle et variée.

completeness-metric-parentage = À quel point chaque lien parent-enfant est sûr
completeness-metric-parentage-none = Aucune filiation ici ne dit à quel point elle est sûre. Adoptions, lignes contestées, reconstitutions à partir d'une seule mention : c'est précisément là qu'une famille a besoin d'enregistrer le doute — et l'arbre dessine un lien moins certain d'un trait plus pâle.
completeness-metric-parentage-some = { $n ->
        [one] Une filiation porte son propre degré de certitude
       *[other] { $n } filiations portent leur propre degré de certitude
    }, de sorte qu'une ligne spéculative est visiblement plus faible qu'une ligne documentée.

completeness-metric-links = Relations hors du sang et du mariage
completeness-metric-links-none = Parrains, employeurs, témoins, mentors, tuteurs. Aucun n'est encore enregistré. Chacun peut porter ses propres dates, sa source et votre degré de certitude.
completeness-metric-links-some = { $n ->
        [one] Une enregistrée, avec ses propres dates, sa source et votre degré de certitude.
       *[other] { $n } enregistrées, chacune avec ses propres dates, sa source et votre degré de certitude.
    }

completeness-metric-occupations = Métiers enregistrés avec un début et une fin
completeness-metric-occupations-none = Aucun métier enregistré. Un métier exercé trente ans en dit plus sur une vie qu'une seule entrée datée.
completeness-metric-occupations-undated = { $total ->
        [one] Un métier est enregistré, sans dates
       *[other] { $total } métiers sont enregistrés, sans dates
    }. Ajoutez un début et une fin et ils pourront être comparés côte à côte sur un même axe.
completeness-metric-occupations-some = { $span } sur { $total } ont un début ou une fin, et peuvent donc être comparés côte à côte sur un même axe.

completeness-metric-sources = Sources classées selon leur fiabilité
completeness-metric-sources-none = Aucune source enregistrée. Indiquer d'où vient un fait est ce qui permet à un proche de le vérifier plus tard — ou de le contester en disant pourquoi.
completeness-metric-sources-some = { $graded } sur { $total } indiquent leur solidité, de sorte qu'une affirmation reposant sur un acte de naissance ne se confond pas avec une autre reposant sur un souvenir.
footer-open-format = L’archive de votre famille tient dans un seul fichier qui vous appartient, écrit dans un format ouvert : il s’ouvrira encore longtemps après ce site.
footer-open-format-link = À propos du format
home-what-title = Ce que cela apporte à une famille
home-what-archive-title = Une seule place pour toute l’archive
home-what-archive-body = L’arbre, les documents et les photographies tiennent ensemble. Le scan d’un acte de mariage est rattaché au mariage lui-même, et non perdu dans la boîte mail de quelqu’un ; une photographie nomme les personnes qui s’y trouvent.
home-what-together-title = Plusieurs proches, des rôles différents
home-what-together-body = Une tante qui tient ses notes depuis trente ans et un cousin qui veut seulement corriger une orthographe n’ont pas besoin des mêmes pouvoirs. Chaque proche est invité avec son propre rôle, et chaque modification garde le nom de son auteur et sa date.
home-what-privacy-title = La confidentialité, personne par personne
home-what-privacy-body = Un proche vivant peut être visible pour la famille et invisible pour les visiteurs, tandis que son arrière-grand-mère reste ouverte à tous. Le choix se fait pour chaque personne, et non une fois pour tout l’arbre.
home-what-languages-title = Onze langues
home-what-languages-body = Chaque proche lit le site dans sa langue — y compris le russe, langue dans laquelle ont été tenus les registres d’état civil de la moitié de l’Europe centrale et orientale. Un nom reste dans son écriture d’origine à côté de sa transcription ; rien n’a besoin d’être ramené à un seul alphabet pour que le site fonctionne.
home-what-export-title = L’archive reste la vôtre
home-what-export-body = Exportez l’ensemble en un seul fichier quand vous le voulez — personnes, relations, documents et photographies réunis. Si vous décidez un jour de partir, vous partez avec l’archive entière.
home-in-this-tree = Ce que la famille a enregistré jusqu’ici
record-notes-title = À noter sur cette fiche :
home-unnamed-family = Cet arbre familial
prefs-machine-complete = complète, pas encore relue

## Rôles tenus par un participant dans un événement

role-spouse = époux
role-spouse_1 = premier époux
role-spouse_2 = second époux
role-subject = personne concernée
role-participant = participant
role-witness = témoin
role-officiant = officiant
role-informant = déclarant
role-godparent = parrain ou marraine
