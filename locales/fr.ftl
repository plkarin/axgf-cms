# axgf-cms — chaînes de l'interface, français.
#
# Traduction relue. Le vocabulaire généalogique suit l'usage français :
# « union » pour union, « fiabilité » pour reliability, « degré de certitude »
# pour confidence, « acte » pour un document d'état civil.
#
# RÈGLE : ce fichier ne traduit que l'interface. Les noms, les lieux, les notes
# et les métiers viennent du fichier .axgf et restent dans leur langue et leur
# écriture d'origine.

app-name = ax-genealogy

## Cadre

nav-tree = Arbre
nav-convert = Importer
nav-admin = Administration
nav-sign-in = Se connecter
nav-sign-out = Se déconnecter

## Préférences

prefs-title = Langue et apparence
prefs-language = Langue
prefs-theme = Apparence
prefs-background = Arrière-plan
prefs-background-on = Un voile de couleur léger derrière la page
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
    }, { $depth } générations de chaque côté.
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
    }, depuis { $path }.
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
place-editor-title = Modifier un lieu
place-add-detail = Compléter ce lieu
place-names = Noms
place-name-primary = Principal
place-name-lang = Langue
place-name-value = Nom
place-names-hint = Une ligne par nom enregistré. Un lieu administré par trois empires porte trois noms ; le principal est celui affiché partout ailleurs.
place-where = Situation
place-type = Type
place-region = Région
place-country-current = Pays actuel
place-country-hint = ISO 3166-1 alpha-2, par ex. PL, FR, DE.
place-country-history = Histoire des frontières
place-history-country = État
place-history-from = De
place-history-until = Jusqu’à
place-country-history-hint = Quel État a détenu ce lieu et pendant quelle période. Généalogiquement significatif : un acte écrit en russe en 1880 et un acte écrit en polonais en 1930 peuvent nommer le même village.
place-coordinates = Coordonnées
place-lat = Latitude
place-lon = Longitude
place-precision = Précision
place-identifiers = Identifiants
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } est utilisé par { $n ->
        [one] un autre enregistrement
       *[other] { $n } autres enregistrements
    }.
place-error-no-name = Un lieu doit porter au moins un nom.
place-error-coords-pair = Latitude et longitude vont ensemble : les deux, ou aucune.
place-error-coords-number = La latitude et la longitude doivent être des nombres.
place-error-coords-range = La latitude va de -90 à 90 et la longitude de -180 à 180.
place-type-continent = continent
place-type-country = pays
place-type-region = région
place-type-department = département
place-type-city = ville
place-type-village = village
place-type-district = quartier
place-type-street = rue
place-type-building = bâtiment
place-type-farm = ferme
place-type-island = île
place-type-historical = historique
place-type-unknown = inconnu
place-precision-exact = exacte
place-precision-building = bâtiment
place-precision-street = rue
place-precision-city_center = centre de la ville
place-precision-region_center = centre de la région
place-precision-country_center = centre du pays
place-precision-approximate = approximative

place-coordinates-hint = La saisie manuelle est la voie habituelle. Beaucoup de lieux enregistrés sous une administration ancienne restent introuvables par une recherche moderne.
place-geocode-search = Rechercher ce nom
place-geocode-hint = Envoie le nom, la région et le pays au service de géocodage, un lieu à la fois. Rien n'est enregistré tant que vous n'enregistrez pas.
place-geocode-off = La recherche de noms est désactivée. Elle exige une adresse de contact permettant au service d'identifier cette installation ; démarrez le serveur avec --geocoder-contact pour l'activer.
place-geocode-query = Recherche effectuée : { $q }
place-geocode-error = Le service de recherche est injoignable. Les champs de coordonnées ci-dessus fonctionnent toujours.
place-geocode-none = Aucun résultat. Pour un village enregistré sous administration russe, prussienne ou autrichienne, c'est le cas ordinaire ; saisissez la position à la main.
place-geocode-not-a-place = pas une localité
place-geocode-use = Utiliser celui-ci
place-geocode-attribution = Résultats d'OpenStreetMap via Nominatim, sous licence Open Database.

place-paste = Coller une position
place-paste-placeholder = un lien de carte, ou 52.0782795, 21.2508068
place-paste-read = Lire
place-paste-hint = Un lien Google Maps ou OpenStreetMap, une URI geo:, une paire de nombres, ou des degrés-minutes-secondes comme 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Lu dans les champs ci-dessus. Vérifiez, puis enregistrez.
place-paste-unreadable = Ce n'est pas une position lisible ici. Les champs ci-dessus acceptent toujours une simple paire de nombres.

place-map-hint = Cliquez sur la carte pour poser le point, ou faites glisser l'épingle. Ce sont les champs ci-dessus qui font foi.
place-map-clear = Effacer le point
place-open-in-map = Chercher ce lieu dans OpenStreetMap, puis recoller le lien

person-tab-record = Fiche
person-tab-life = Vie
person-tab-media = Documents
person-tab-tree = Arbre
person-tree-depth = { $n } générations de chaque côté. L'arbre entier se trouve plus bas.
person-tree-alone = Cette fiche ne nomme ni parents, ni conjoints, ni enfants : il n'y a donc aucune forme à dessiner autour d'elle.

record-no-evidence = Rien n'est joint à cette fiche — ni source ni document. C'est l'état ordinaire d'un fichier converti, non un défaut : le GEDCOM emporte les faits et laisse derrière lui ce qui les prouvait.
record-no-evidence-signed-out = Connectez-vous pour en joindre un.
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
accounts-branch-hint = Limite ce que ce compte peut modifier à ces personnes, leurs descendants et leurs conjoints.
accounts-branch-reading = Cela ne limite pas ce qu'il peut lire — ceci relève de la visibilité de chaque fiche, et les deux sont volontairement séparés.
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
tree-width-notice = Cette vue fait { $width } px de large — sur un écran de 1500 px, cela représente { $screens ->
        [one] un écran
       *[other] { $screens } écrans
    } de défilement horizontal.
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
tree-contradicts-detail = Aucune disposition des rangées ne peut satisfaire cela : la relation ci-dessous a donc été écartée de la numérotation des générations et certaines rangées peuvent être mal placées. Corrigez celle des deux fiches qui est fautive.
tree-contradicts-pair = Enregistrés à la fois comme couple et comme parent et enfant :
tree-contradicts-more = { $n ->
        [one] Une autre contradiction n’est pas listée.
       *[other] { $n } autres contradictions ne sont pas listées.
    }
tree-no-people = Il n’y a encore personne dans cet arbre.
tree-no-people-cta = Importez un fichier familial, ou ajoutez la première personne.
tree-nobody-selected = Personne à dessiner pour cette sélection.
tree-nobody-selected-cta = Repartir de la vue par défaut.
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

## Vocabulary the structured editors offer

name-part-nasab = nasab (filiation)
name-part-laqab = laqab (épithète)
name-part-kunya = kunya (téknonyme)
name-part-nisbah = nisbah (origine)
name-part-alias = alias
name-part-religious_name = nom religieux
name-part-pen_name = nom de plume
name-type-pen_name = nom de plume
gender-U = Non renseigné

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
person-age-at-death = mort à { $n } ans
person-age-now = { $n } ans
person-born-in = né à { $place }
person-died-in = mort à { $place }
person-children-count = { $n ->
        [one] un enfant
       *[other] { $n } enfants
    }
person-generations-below = { $n ->
        [one] une génération en dessous
       *[other] { $n } générations en dessous
    }
person-portrait-of = Photographie de { $name }
person-no-portrait = Aucune photographie enregistrée
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
record-verbatim-note = Conservé exactement tel que le dossier l'énonçait, parce qu'aucun convertisseur n'a su l'interpréter.
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
convert-lede = Reprenez un arbre existant depuis un fichier GEDCOM — l’export que produisent la plupart des logiciels de généalogie. Rien n’est stocké ici, et l’arbre que ce site montre déjà reste exactement tel quel.
convert-file-label = Fichier familial (.ged)
convert-file-hint = Jusqu’à { $mb } Mo. Un arbre de 767 personnes pèse environ 320 Ko.
convert-confidence-label = Degré de certitude de départ
convert-confidence-hint = Le fichier importé ne dit pas à quel point quelqu’un était sûr : chaque fait a donc besoin d’un point de départ. Choisissez une valeur basse pour un arbre monté rapidement, plus haute pour un arbre travaillé sur documents. La lecture honnête de ce nombre est « importé, et vérifié par personne depuis » — vous pourrez relever ou abaisser chaque fait ensuite, un par un.
convert-lang-label = Langue des noms de lieux
convert-lang-hint = Une étiquette comme en, fr ou pl.
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
convert-skipped-note = Ces entrées ne contenaient rien qui puisse être repris.
convert-other-diagnostics = { $n ->
        [one] Une autre chose à savoir
       *[other] { $n } autres choses à savoir
    }
convert-clean = Rien n’est resté en arrière — chaque entrée du fichier est passée.
convert-download-title = Téléchargement
convert-download-named = Télécharger { $name }
convert-download-note = Conservé ici quinze minutes puis supprimé : téléchargez-le maintenant.
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
completeness-intro = Ce qui est enregistré et ce qui reste vide.
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

phys-no-source = sans source
phys-col-date = Quand
phys-col-source = Source
phys-col-confidence = Confiance
phys-col-note = Note
phys-field-height-cm = Taille
phys-field-weight-kg = Poids
phys-field-eye-colour = Couleur des yeux
phys-field-hair-colour = Couleur des cheveux
phys-field-build = Corpulence
phys-field-handedness = Latéralité
phys-field-features = Signes particuliers
phys-field-military = Service militaire
phys-field-languages = Langues parlées
phys-field-blood-group = Groupe sanguin
phys-field-conditions = Affections connues
phys-field-operations = Opérations et blessures
phys-field-cause-of-death = Cause du décès
phys-field-religion = Religion ou appartenance
phys-field-health-notes = Notes
admin-export-health-note = L’export simple laisse de côté chaque catégorie sensible — santé et convictions, biométrie, données génomiques, casier judiciaire — ainsi que le profil comportemental de toute personne vivante : un fichier envoyé à un parent n’en contient aucune. Cochez ce qu’un fichier donné doit contenir ; l’archive indique elle-même les catégories omises.
avatar-picker-title = Choisir une image
avatar-choose-link = Choisir l'image
avatar-choose = Quelle image représente cette personne
avatar-mode-auto = Laisser le logiciel choisir
avatar-mode-auto-note = Le premier portrait, à défaut la première image liée à cette fiche.
avatar-mode-none = Afficher les initiales
avatar-mode-none-note = Pour une fiche dont les images sont des documents et non des visages.
avatar-focal-hint = Cliquez une image pour la choisir, puis cliquez à nouveau sur la partie à garder dans le cadre — un avatar est carré, la plupart des numérisations ne le sont pas.
avatar-no-images = Aucune image n'est encore liée à cette fiche.
avatar-upload-title = Envoyer une image et l'utiliser
avatar-upload-button = Envoyer et utiliser comme image
avatar-not-available-title = Cette image n'est pas disponible
avatar-not-available-detail = Le fichier choisi n'est pas lié à cette personne, ou vous n'avez pas le droit de le lire.

record-history-withheld = non communiqué

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Statut
record-presumed-deceased = décès présumé
record-presumed-short = présumé
record-presumed-why = Aucun décès n'est enregistré et la naissance remonte à plus de { $years } ans : la fiche ne peut donc pas être exacte. L'archive n'est pas modifiée — c'est ce que la page déduit, non ce que la source dit.

## La figure engendrée à côté d'une fiche
#
# Ce n'est pas un portrait. Tous les messages qui suivent existent pour que
# cela reste sans ambiguïté.

silhouette-label = Âge et taille relevés, non une apparence
silhouette-not-a-likeness = Ce n'est pas un portrait : rien n'en est tiré d'une photographie.
silhouette-proportions-infant = Proportions d'un nourrisson, environ quatre hauteurs de tête.
silhouette-proportions-child = Proportions d'un jeune enfant, environ six hauteurs de tête.
silhouette-proportions-adolescent = Proportions d'un adolescent, environ sept hauteurs de tête.
silhouette-proportions-adult = Proportions d'un adulte, environ sept hauteurs de tête et demie.
silhouette-proportions-elderly = Proportions d'un adulte, qui cessent de changer vers vingt ans : la figure ne distingue pas une personne âgée d'un adulte plus jeune, alors que les dates ci-dessus le font.
silhouette-to-scale = Dessinée à l'échelle, par rapport à une ligne de référence à { $ref } cm.
silhouette-no-height = Aucune taille n'est enregistrée : la figure est dessinée à une taille nominale pour sa tranche d'âge, et non à l'échelle.
silhouette-several-heights = Plusieurs tailles sont enregistrées ; la figure dessine la plus récente.

## The identity editor

identity-editor-title = Noms et identité
identity-primary-name = Le nom affiché partout
identity-primary-help = Ce qu'utilisent la carte de l'arbre, le titre et toutes les listes. Les autres noms ci-dessous sont ceux qu'une source a employés à un autre moment.
identity-display = Nom
identity-display-latin = En écriture latine
identity-culture = Langue
identity-direction = Sens d'écriture
identity-direction-ltr = de gauche à droite
identity-direction-rtl = de droite à gauche
identity-direction-auto = selon le texte
identity-components = Parties du nom
identity-components-help = Quelle partie est le prénom et laquelle le nom de famille, dans l'ordre où ils s'écrivent. Une fiche sans parties s'affiche quand même : les parties sont ce sur quoi une recherche peut porter.
identity-part = Partie
identity-value = Texte
identity-other-names = Autres noms
identity-other-help = Un nom d'épouse, un nom religieux, un nom employé par une fiche plus tardive. Chacun porte sa période d'usage et la source qui l'atteste.
identity-name-type = Type de nom
identity-valid-from = En usage depuis
identity-valid-until = En usage jusqu'à
identity-about = À propos de la personne
identity-living-help = C'est l'indicateur posé par la source. La page présume par ailleurs un décès quand la naissance est trop ancienne, et cette présomption ne modifie jamais cette case ni l'archive.
identity-error-no-display = Une fiche a besoin d'un nom pour être affichée. Rien n'a été enregistré.
editor-blank-to-remove = Videz le nom pour supprimer cette entrée.
# A BCP 47 tag, shown as an example in the language field.
identity-culture-placeholder = fr
identity-edit-link = Modifier les noms et l'identité

## Union types, statuses and date precision, said out loud

union-type-marriage = mariage
union-type-civil_union = union civile
union-type-cohabitation = concubinage
union-type-religious_only = union religieuse
union-type-polygamous = polygame
union-type-unknown = non renseigné
union-status-active = en cours
union-status-ended_by_death = terminée par un décès
union-status-ended_by_divorce = terminée par un divorce
union-status-ended_by_separation = terminée par une séparation
union-status-annulled = annulée
union-status-unknown = non renseigné
union-status-ended = terminée
union-status-ended-by = terminée par { $reason }
union-reason-death_of_spouse = le décès d'un conjoint
precision-exact = au jour près
precision-year = à l'année près
precision-month = au mois près
precision-decade = à la décennie près
precision-century = au siècle près
precision-unknown = inconnue
record-precision = Précision
record-approximate = Approximatif
record-place = Lieu

## The relationships editor

family-editor-title = Famille et relations
family-unions = Unions
family-no-unions = Aucune union n'est enregistrée pour cette personne.
family-union-legend = Union { $n }
family-writes-family = Enregistrer modifie la fiche de famille #{ $id }, partagée par les deux personnes. La page de l'autre personne change aussi.
family-partners = Partenaires
family-partner = Partenaire
family-role = Rôle
family-children = Enfants
family-children-help = L'ordre de naissance est ce que la fiche affirme elle-même. Laissé vide, rien n'est affirmé : un numéro tiré de la position de la ligne serait un fait que personne n'a écrit.
family-child = Enfant
family-birth-order = Ordre de naissance
family-the-union = L'union elle-même
family-type = Type d'union
family-status = Statut
family-started = Début
family-ended = Fin
family-leave = Retirer cette personne de cette union
family-open-entity = Ouvrir la fiche de famille
family-new-union = Une nouvelle union
family-new-union-help = Cela crée une nouvelle fiche de famille contenant cette personne. Le partenaire est facultatif : un parent que la fiche nomme sans aucun partenaire est une union à une personne.
family-create-union = Créer l'union
family-parents = Parents
family-no-parents = Cette personne n'est enregistrée comme enfant d'aucune famille.
family-child-of = Enfant de cette famille
family-detach-child = Retirer cette personne de cette famille
family-attach-parents = Rattacher à des parents
family-attach-help = Choisissez la famille dont cette personne est l'enfant. Cela l'ajoute à cette fiche de famille, et la modification apparaît aussi sur les pages des parents.
family-the-family = La famille
family-attach = Rattacher
family-error-last-partner = Une union doit contenir au moins une personne. Supprimez plutôt la fiche de famille, ce qui demande quoi faire de tout ce qui s'y réfère.
family-error-no-family = Aucune famille n'a été choisie. Rien n'a été enregistré.
family-error-already-child = Cette personne est déjà l'enfant de cette famille.
pick-error-empty = Aucune personne n'a été nommée. Rien n'a été enregistré.
pick-error-not-found = Aucune personne de ce nom dans cette archive. Rien n'a été enregistré.
pick-error-ambiguous = Plusieurs personnes répondent à cela. Choisissez-en une dans la liste pour que la fiche dise laquelle. Rien n'a été enregistré.

## Links and occupations

links-editor-title = Liens
links-editor-help = Les relations qui ne sont pas familiales : un parrain, un employeur, un témoin, un régiment. Chacune est une fiche à part nommant deux personnes ; la modifier ici change aussi ce que l'autre fiche affiche.
links-none = Aucun lien n'est enregistré pour cette personne.
links-new = Un nouveau lien
links-create = Créer le lien
links-remove = Supprimer ce lien
links-other-end = L'autre extrémité
links-label = Ce que c'est
links-label-reverse = Dans l'autre sens
links-category = Catégorie
links-bidirectional = Se lit pareil dans les deux sens
links-from = À partir de
links-until = Jusqu'à
links-reversed = Ce lien a été créé depuis l'autre fiche. Le modifier ici change la même entité.
link-error-no-label = Un lien doit dire ce qu'il est. Rien n'a été enregistré.
occupations-editor-title = Professions
occupations-editor-help = Une profession est une période avec un début et une fin, non un intitulé de poste. Chacune porte ses propres dates et sa propre source.
occupations-none = Aucune profession n'est enregistrée pour cette personne.
occupations-new = Une nouvelle profession
occupations-create = Créer la profession
occupations-remove = Supprimer cette profession
occupations-title = Ce qu'elle faisait
occupations-employer = Pour qui
occupations-employer-place = Où ils étaient
occupations-from = À partir de
occupations-until = Jusqu'à
occupation-error-no-title = Une profession doit dire ce que quelqu'un faisait. Rien n'a été enregistré.
link-category-spiritual = spirituel
link-category-professional = professionnel
link-category-social = social
link-category-legal = juridique
link-category-medical = médical
link-category-educational = éducatif
link-category-conflict = conflit
link-category-other = autre
links-edit-link = Modifier les liens
occupations-edit-link = Modifier les professions
family-edit-link = Modifier la famille et les relations

## Events and documents

events-editor-title = Événements
events-editor-help = Un événement nomme plusieurs personnes à la fois — un mariage, un baptême, un recensement — c'est donc une fiche à part, qui apparaît sur chaque page qu'elle nomme.
events-none = Aucun événement ne nomme cette personne.
events-new = Un nouvel événement
events-new-help = Cette personne y est ajoutée comme sujet si vous n'en nommez aucune autre. Un événement sans personne n'est qu'une date.
events-create = Créer l'événement
events-remove = Supprimer cet événement
events-category = Ce qui s'est passé
events-subcategory = Plus précisément
events-description = Description
events-participants = Qui était présent
events-participants-help = Enregistrer modifie la fiche de l'événement, que toutes les autres personnes nommées affichent aussi.
events-who = Qui
event-error-no-category = Un événement doit dire ce qui s'est passé. Rien n'a été enregistré.
documents-editor-title = Documents
documents-editor-help = Quels fichiers cette fiche désigne, et ce que chacun est pour elle. Vider une ligne détache le fichier : le document et ses octets restent dans l'archive.
documents-attached = Attachés à cette fiche
documents-upload = Téléverser un fichier
documents-upload-help = Jusqu'à { $mb } Mo. Le fichier est rangé dans l'archive et attaché à cette fiche.
documents-caption = Légende
documents-edit-link = Attacher et détacher des documents
events-edit-link = Modifier les événements

## Presentation styles: density, never colour

prefs-style = Densité
prefs-style-help = L'espace que prend la page. Distinct du thème, qui ne concerne que la couleur : vous pouvez combiner l'un et l'autre librement.
style-comfortable = Confortable
style-comfortable-note = par défaut, avec de l'air pour lire
style-compact = Compacte
style-compact-note = plus de fiche par écran, pour en parcourir plusieurs
style-paper = Papier
style-paper-note = une police à empattements et des filets au lieu de cartes, pour lire posément ou imprimer

## Sensitive classes

admin-export-choose = Inclure dans cet export
scope-health = Santé et convictions
scope-biometrics = Biométrie
scope-genomics = Données génomiques
scope-legal = Casier judiciaire
scope-behaviour = Profils comportementaux des personnes vivantes
admin-export-with-chosen = Exporter avec les éléments cochés

## Profile

pg-identity = Identité et état civil
pg-identity-intro = Qui le dossier dit que la personne était, et ce que les registres d’état civil ont consigné.
pg-morphology = Morphologie
pg-morphology-intro = Le corps tel qu’il a été mesuré et décrit.
pg-biometrics = Biométrie
pg-biometrics-intro = La voix, les mains et les sens, et les gabarits qui identifient une personne.
pg-health = Santé
pg-health-intro = Affections, traitements, mesures et résultats.
pg-genomics = Génomique
pg-genomics-intro = Tests ADN, haplogroupes, variants et autres résultats moléculaires.
pg-death = Décès
pg-death-intro = Comment, quand et où une vie a pris fin, et ce qu’il est advenu du corps.
pg-residence = Résidence et nationalité
pg-residence-intro = Où la personne a vécu, quels États la comptaient parmi leurs ressortissants, et les langues qu’elle parlait.
pg-education = Éducation et travail
pg-education-intro = Scolarité, diplômes, revenus et biens.
pg-military = Service militaire et distinctions
pg-military-intro = Service, grades, unités et distinctions.
pg-legal = Justice
pg-legal-intro = Procédures pénales et leur issue.
pg-belief = Convictions et appartenances
pg-belief-intro = Religion, rites, convictions et adhésions.
pg-personality = Personnalité et comportement
pg-personality-intro = Tempérament, habitudes et loisirs, tels que les sources les décrivent.
pg-relationships = Relations
pg-relationships-intro = Parents, conjoints, enfants et les autres personnes d’une vie.
pg-digital-legacy = Héritage numérique
pg-digital-legacy-intro = Numérisations, modèles, enregistrements et archives qui représentent une personne.
pa-identity-titles = Titres
pa-identity-sex-at-birth = Sexe à la naissance
pa-identity-gender-identity = Identité de genre
pa-birth-time = Heure de naissance
pa-birth-coordinates = Lieu de naissance, en coordonnées
pa-civil-status-birth-certificate-number = Numéro de l’acte de naissance
pa-civil-status-register-entries = Actes d’état civil
pa-civil-status-marginal-annotations = Mentions marginales
pa-morphology-height = Taille
pa-morphology-weight = Poids
pa-morphology-bmi = Indice de masse corporelle
pa-morphology-body-composition = Composition corporelle
pa-morphology-build = Corpulence
pa-morphology-eye-colour = Couleur des yeux
pa-morphology-eye-shape = Forme des yeux
pa-morphology-eye-spacing = Écartement des yeux
pa-morphology-hair-colour = Couleur naturelle des cheveux
pa-morphology-hair-texture = Nature des cheveux
pa-morphology-hairline = Implantation des cheveux
pa-morphology-facial-hair = Pilosité faciale
pa-morphology-body-hair = Pilosité corporelle
pa-morphology-skin-tone = Phototype (Fitzpatrick)
pa-morphology-skin-undertone = Sous-ton de la peau
pa-morphology-freckles = Taches de rousseur
pa-morphology-pigmentation = Marques pigmentaires
pa-morphology-scars = Cicatrices
pa-morphology-tattoos = Tatouages
pa-morphology-moles = Grains de beauté
pa-morphology-facial-asymmetries = Asymétries du visage
pa-morphology-face-shape = Forme du visage
pa-morphology-nose-shape = Forme du nez
pa-morphology-ear-shape = Forme des oreilles
pa-morphology-lip-shape = Forme des lèvres
pa-morphology-dentition = Denture
pa-morphology-malocclusion = Malocclusion (classe d’Angle)
pa-morphology-posture = Posture
pa-morphology-gait = Démarche
pa-morphology-distinguishing-features = Signes particuliers
pa-biometrics-fingerprints = Empreintes digitales
pa-biometrics-retinal-print = Empreinte rétinienne
pa-biometrics-voice-signature = Empreinte vocale
pa-biometrics-voice-frequency = Fréquence fondamentale de la voix
pa-biometrics-vocal-timbre = Timbre de voix
pa-biometrics-spoken-accent = Accent
pa-biometrics-speech-rate = Débit de parole
pa-biometrics-verbal-tics = Tics verbaux
pa-biometrics-frequent-vocabulary = Vocabulaire fréquent
pa-biometrics-speech-register = Registre de langue
pa-biometrics-motor-tics = Tics moteurs
pa-biometrics-handedness = Latéralité
pa-biometrics-hearing = Audition
pa-biometrics-visual-acuity = Acuité visuelle
pa-biometrics-optical-correction = Correction optique
pa-health-blood-group = Groupe sanguin (ABO)
pa-health-rhesus = Rhésus (RhD)
pa-health-blood-pressure = Tension artérielle
pa-health-resting-heart-rate = Fréquence cardiaque au repos
pa-health-respiratory-capacity = Capacité respiratoire
pa-health-conditions = Affections
pa-health-surgeries = Antécédents chirurgicaux
pa-health-injuries = Blessures
pa-health-deformities = Malformations
pa-health-amputations = Amputations
pa-health-prostheses = Prothèses
pa-health-implants = Implants
pa-health-devices = Dispositifs implantés
pa-health-medications = Médicaments
pa-health-allergies = Allergies
pa-health-vaccinations = Vaccinations
pa-health-serology = Sérologie
pa-health-lab-results = Résultats de laboratoire
pa-health-deficiencies = Carences
pa-health-sleep-disorders = Troubles du sommeil
pa-health-mental-health-assessments = Évaluations de santé mentale
pa-genomics-autosomal-mapping = Test ADN autosomique
pa-genomics-y-haplogroup = Haplogroupe du chromosome Y
pa-genomics-mt-haplogroup = Haplogroupe mitochondrial
pa-genomics-whole-genome-sequencing = Séquençage du génome complet
pa-genomics-risk-variants = Variants à risque
pa-genomics-hereditary-conditions = Maladies héréditaires
pa-genomics-predispositions = Prédispositions
pa-genomics-epigenetic-markers = Marqueurs épigénétiques
pa-genomics-epigenetic-age = Âge épigénétique
pa-genomics-gut-microbiome = Microbiote intestinal
pa-genomics-skin-microbiome = Microbiote cutané
pa-genomics-toxicological-sensitivities = Sensibilités aux médicaments et aux toxiques
pa-death-time = Heure du décès
pa-death-coordinates = Lieu du décès, en coordonnées
pa-death-causes = Causes du décès
pa-death-contributing-factors = Facteurs contributifs
pa-death-autopsy = Autopsie
pa-death-disposition = Devenir du corps
pa-death-grave = Sépulture
pa-residence-addresses = Adresses
pa-residence-nationality-of-origin = Nationalité d’origine
pa-residence-acquired-nationalities = Nationalités acquises
pa-residence-mother-tongue = Langue maternelle
pa-residence-spoken-languages = Langues parlées
pa-education-level = Niveau d’études
pa-education-diplomas = Diplômes et grades
pa-education-institutions = Écoles et établissements
pa-education-income = Revenus
pa-education-real-estate = Biens immobiliers
pa-military-distinctions = Distinctions
pa-military-citations = Citations
pa-military-ranks = Grades
pa-military-units = Unités
pa-military-service-numbers = Numéros matricules
pa-legal-criminal-record = Casier judiciaire
pa-belief-religions = Religion
pa-belief-sacraments = Sacrements et rites
pa-belief-beliefs = Convictions
pa-belief-political-leanings = Orientation politique
pa-belief-memberships = Adhésions
pa-personality-big-five = Scores Big Five
pa-personality-mbti = Type MBTI
pa-personality-introversion-extraversion = Introversion et extraversion
pa-personality-stress-tolerance = Tolérance au stress
pa-personality-decision-style = Style de décision
pa-personality-interests = Centres d’intérêt
pa-personality-hobbies = Loisirs
pa-personality-sports = Sports
pa-personality-dietary-habits = Régime alimentaire
pa-personality-dependencies = Dépendances
pa-digital-legacy-body-models = Modèles du corps
pa-digital-legacy-skin-textures = Textures de la peau
pa-digital-legacy-rigs = Squelettes d’animation
pa-digital-legacy-voice-corpora = Enregistrements pour la synthèse vocale
pa-digital-legacy-text-corpora = Écrits pour un modèle de langue
pa-digital-legacy-digital-traces = Traces numériques
pa-digital-legacy-carbon-footprint = Empreinte carbone
pa-digital-legacy-behaviour-models = Modèles de comportement
pf-identity-titles-text = Titre tel qu’écrit
pf-identity-titles-kind = Type de titre
pf-civil-status-marginal-annotations-text = Mention
pf-morphology-pigmentation-kind = Type de marque
pf-biometrics-spoken-accent-description = Comment il est décrit
pf-biometrics-optical-correction-kind = Correction
pf-health-amputations-level = Niveau d’amputation
pf-health-prostheses-kind = Prothèse
pf-health-implants-kind = Implant
pf-health-devices-kind = Dispositif
pf-health-allergies-type = Type d’allergie
pf-health-vaccinations-status = Statut vaccinal
pf-health-sleep-disorders-category = Catégorie de trouble
pf-death-autopsy-kind = Autopsie
pf-education-institutions-name = Nom de l’établissement
pf-military-distinctions-name = Nom de la distinction
pf-military-distinctions-kind = Type de distinction
pf-military-citations-text = Citation
pf-military-ranks-category = Catégorie de grade
pf-belief-political-leanings-position = Position sur l’axe gauche–droite
pf-belief-memberships-kind = Type d’organisation
pf-digital-legacy-carbon-footprint-method = Méthode d’estimation
pf-age-years = Âge en années
pf-agreeableness = Agréabilité
pf-allergen = Allergène
pf-amount = Montant
pf-analyte = Analyte
pf-artefact-type = Type d’artefact
pf-autoimmune = Auto-immune
pf-body-region = Région du corps
pf-bone-percent = Os
pf-carrier-status = Statut de porteur
pf-cause = Cause
pf-chronic = Chronique
pf-clock = Horloge
pf-condition = Affection
pf-conferred-by = Décernée par
pf-congenital = Congénitale
pf-conscientiousness = Conscienciosité
pf-consent = Consentement
pf-coordinates = Coordonnées
pf-corrected = Avec correction
pf-country = Pays
pf-court = Juridiction
pf-coverage = Couverture
pf-currency = Devise
pf-decimal = Acuité (décimale)
pf-denomination = Confession
pf-derived-from-id = Dérivé de
pf-description = Description
pf-details = Précisions
pf-diagnosis = Diagnostic
pf-diameter-mm = Diamètre
pf-diastolic = Diastolique
pf-diet = Régime
pf-document-id = Document
pf-dose = Dose
pf-ear = Oreille
pf-entry-number = Numéro d’acte
pf-extraversion = Extraversion
pf-eye = Œil
pf-fat-percent = Graisse
pf-fev1-fvc-ratio = Rapport VEMS/CVF
pf-fev1-litres = VEMS
pf-file-format = Format de fichier
pf-findings = Constatations
pf-flag = Signalement
pf-format = Format
pf-fracture = Fracture
pf-fvc-litres = CVF
pf-gene = Gène
pf-generator = Réalisé avec
pf-grade = Degré
pf-iccs-section = Section d’infraction (ICCS)
pf-icd10-chapter = Chapitre de la CIM-10
pf-indication = Indication
pf-inheritance = Transmission
pf-inscription = Inscription
pf-institution = Établissement
pf-instrument = Instrument
pf-isced-level = Niveau CITE
pf-jurisdiction = Juridiction nationale
pf-language = Langue
pf-lat = Latitude
pf-level = Niveau
pf-lines = Adresse
pf-location = Emplacement
pf-lon = Longitude
pf-major = Haplogroupe principal
pf-marker = Marqueur
pf-metaboliser-status = Statut de métaboliseur
pf-method = Méthode
pf-mode = Mode d’acquisition
pf-muscle-percent = Muscle
pf-neuroticism = Névrosisme
pf-number = Numéro
pf-nutrient = Nutriment
pf-offence = Infraction
pf-office = Bureau
pf-openness = Ouverture
pf-organisation = Organisation
pf-outcome = Issue
pf-pace = Vitesse de vieillissement
pf-page = Page
pf-panel = Bilan
pf-party = Parti
pf-pathogen = Agent pathogène
pf-pattern = Mode de consommation
pf-percentile = Centile
pf-period = Période de paie
pf-place-id = Lieu
pf-plot = Emplacement
pf-polygenic-score = Score polygénique
pf-postal-code = Code postal
pf-precision = Précision
pf-prescription = Prescription
pf-proficiency = Niveau de maîtrise
pf-provider = Prestataire
pf-quintile = Quintile de revenu
pf-rank = Grade
pf-rank-text = Grade tel qu’écrit
pf-reaction = Réaction
pf-reference-build = Génome de référence
pf-reference-high = Valeur de référence haute
pf-reference-low = Valeur de référence basse
pf-register-type = Type d’acte
pf-result = Résultat
pf-role = Rôle
pf-sacrament = Sacrement ou rite
pf-score = Score
pf-sentence = Peine
pf-sequence = Rang dans l’enchaînement
pf-service = Armée
pf-severity = Gravité
pf-shannon-diversity = Diversité de Shannon
pf-shape = Forme
pf-significance = Signification clinique
pf-snp-count = SNP analysés
pf-sport = Sport
pf-subclade = Sous-clade
pf-substance = Substance
pf-summary = Synthèse
pf-systolic = Systolique
pf-tenure = Régime de propriété
pf-test = Test
pf-threshold-db = Seuil auditif
pf-title = Intitulé
pf-tonnes-co2e-per-year = Émissions
pf-tradition = Tradition
pf-tree-version = Version de l’arbre
pf-unit = Unité
pf-use = Usage
pf-variant = Variant
pf-volume = Registre
pf-zygosity = Zygotie
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n } %
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = { $n } mots/min
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = { $n } bpm
pu-litres = { $n } L
pu-coverage = { $n }×
pu-years = { $n } ans
pu-t-co2e-yr = { $n } t éq. CO₂ par an
pv-sensitive-class-health = Santé et convictions
pv-sensitive-class-biometrics = Biométrie
pv-sensitive-class-genomics = Données génomiques
pv-sensitive-class-legal = Casier judiciaire
pv-laterality-left = Gauche
pv-laterality-right = Droite
pv-laterality-both = Les deux
pv-body-region-head = Tête
pv-body-region-face = Visage
pv-body-region-neck = Cou
pv-body-region-left-shoulder = Épaule gauche
pv-body-region-right-shoulder = Épaule droite
pv-body-region-left-arm = Bras gauche
pv-body-region-right-arm = Bras droit
pv-body-region-left-hand = Main gauche
pv-body-region-right-hand = Main droite
pv-body-region-chest = Thorax
pv-body-region-abdomen = Abdomen
pv-body-region-upper-back = Haut du dos
pv-body-region-lower-back = Bas du dos
pv-body-region-pelvis = Bassin et hanches
pv-body-region-left-leg = Jambe gauche
pv-body-region-right-leg = Jambe droite
pv-body-region-left-foot = Pied gauche
pv-body-region-right-foot = Pied droit
pv-body-region-internal = Interne
pv-body-region-whole-body = Corps entier
pv-body-region-other = Autre région
pv-artefact-type-mesh = Maillage
pv-artefact-type-point-cloud = Nuage de points
pv-artefact-type-skin-texture-map = Carte de texture de la peau
pv-artefact-type-skeletal-rig = Squelette d’animation
pv-artefact-type-voice-corpus = Corpus vocal
pv-artefact-type-text-corpus = Corpus de textes
pv-artefact-type-trace-archive = Archive d’activité en ligne
pv-artefact-type-behaviour-model = Modèle de comportement
pv-artefact-type-fingerprint-card = Fiche d’empreintes
pv-artefact-type-fingerprint-template = Gabarit d’empreinte
pv-artefact-type-retinal-image = Image rétinienne
pv-artefact-type-voiceprint = Empreinte vocale
pv-consent-given = Donné
pv-consent-given-by-estate = Donné par les ayants droit
pv-consent-refused = Refusé
pv-consent-withdrawn = Retiré
pv-consent-not-asked = Non demandé
pv-consent-unknown = Inconnu
pv-sex-at-birth-female = Féminin
pv-sex-at-birth-male = Masculin
pv-sex-at-birth-intersex = Intersexe
pv-sex-at-birth-undetermined = Indéterminé
pv-sex-at-birth-unknown = Inconnu
pv-gender-identity-woman = Femme
pv-gender-identity-man = Homme
pv-gender-identity-non-binary = Non binaire
pv-gender-identity-other = Autre
pv-gender-identity-undisclosed = Non communiquée
pv-gender-identity-unknown = Inconnue
pv-title-kind-nobility = Noblesse
pv-title-kind-academic = Universitaire
pv-title-kind-professional = Professionnel
pv-title-kind-religious = Religieux
pv-title-kind-military = Militaire
pv-title-kind-civic = Civique ou honorifique
pv-title-kind-courtesy = De courtoisie
pv-title-kind-other = Autre
pv-register-type-birth = Naissance
pv-register-type-baptism = Baptême
pv-register-type-marriage = Mariage
pv-register-type-death = Décès
pv-register-type-burial = Inhumation
pv-register-type-divorce = Divorce
pv-register-type-recognition = Reconnaissance d’enfant
pv-register-type-legitimation = Légitimation
pv-register-type-adoption = Adoption
pv-register-type-name-change = Changement de nom
pv-register-type-other = Autre
pv-build-slight = Menue
pv-build-slim = Mince
pv-build-average = Moyenne
pv-build-sturdy = Robuste
pv-build-stout = Corpulente
pv-build-heavy = Forte
pv-eye-colour-light-blue = Bleu clair
pv-eye-colour-blue = Bleu
pv-eye-colour-dark-blue = Bleu foncé
pv-eye-colour-grey = Gris
pv-eye-colour-blue-grey = Bleu-gris
pv-eye-colour-green = Vert
pv-eye-colour-grey-green = Gris-vert
pv-eye-colour-hazel = Noisette
pv-eye-colour-amber = Ambre
pv-eye-colour-light-brown = Marron clair
pv-eye-colour-brown = Marron
pv-eye-colour-dark-brown = Marron foncé
pv-eye-colour-black = Noir
pv-eye-colour-mixed = Mélangé
pv-eye-colour-other = Autre
pv-eye-shape-almond = En amande
pv-eye-shape-round = Ronds
pv-eye-shape-hooded = Paupière tombante
pv-eye-shape-monolid = Paupière unique
pv-eye-shape-deep-set = Enfoncés
pv-eye-shape-protruding = Saillants
pv-eye-shape-upturned = Relevés
pv-eye-shape-downturned = Tombants
pv-eye-shape-other = Autre
pv-eye-spacing-close-set = Rapprochés
pv-eye-spacing-average = Moyen
pv-eye-spacing-wide-set = Écartés
pv-hair-colour-black = Noir
pv-hair-colour-dark-brown = Brun foncé
pv-hair-colour-brown = Brun
pv-hair-colour-light-brown = Châtain clair
pv-hair-colour-auburn = Auburn
pv-hair-colour-red = Roux
pv-hair-colour-strawberry-blond = Blond vénitien
pv-hair-colour-dark-blond = Blond foncé
pv-hair-colour-blond = Blond
pv-hair-colour-light-blond = Blond clair
pv-hair-colour-grey = Gris
pv-hair-colour-white = Blanc
pv-hair-colour-none = Sans cheveux
pv-hair-colour-other = Autre
pv-hair-texture-straight = Raides
pv-hair-texture-wavy = Ondulés
pv-hair-texture-curly = Bouclés
pv-hair-texture-coily = Crépus
pv-hair-texture-other = Autre
pv-hairline-straight = Droite
pv-hairline-rounded = Arrondie
pv-hairline-widows-peak = En pointe
pv-hairline-m-shaped = En M
pv-hairline-bell-shaped = En cloche
pv-hairline-uneven = Irrégulière
pv-hairline-receding = Dégarnie
pv-hairline-bald = Chauve
pv-facial-hair-none = Aucune
pv-facial-hair-stubble = Barbe de quelques jours
pv-facial-hair-moustache = Moustache
pv-facial-hair-goatee = Bouc
pv-facial-hair-full-beard = Barbe complète
pv-facial-hair-sideburns = Favoris
pv-facial-hair-other = Autre
pv-body-hair-none = Aucune
pv-body-hair-sparse = Clairsemée
pv-body-hair-moderate = Modérée
pv-body-hair-dense = Dense
pv-skin-tone-type-i = Type I — brûle toujours, ne bronze jamais
pv-skin-tone-type-ii = Type II — brûle souvent, bronze peu
pv-skin-tone-type-iii = Type III — brûle parfois, bronze uniformément
pv-skin-tone-type-iv = Type IV — brûle rarement, bronze bien
pv-skin-tone-type-v = Type V — brûle très rarement
pv-skin-tone-type-vi = Type VI — ne brûle jamais
pv-skin-undertone-cool = Froid
pv-skin-undertone-neutral = Neutre
pv-skin-undertone-warm = Chaud
pv-skin-undertone-olive = Olive
pv-freckles-none = Aucune
pv-freckles-few = Quelques-unes
pv-freckles-moderate = Modérées
pv-freckles-many = Nombreuses
pv-pigmentation-mark-birthmark = Tache de naissance
pv-pigmentation-mark-port-wine-stain = Angiome plan
pv-pigmentation-mark-cafe-au-lait-spot = Tache café au lait
pv-pigmentation-mark-depigmented-patch = Tache dépigmentée
pv-pigmentation-mark-hyperpigmented-patch = Tache hyperpigmentée
pv-pigmentation-mark-other = Autre
pv-mole-shape-round = Rond
pv-mole-shape-oval = Ovale
pv-mole-shape-irregular = Irrégulier
pv-mole-shape-other = Autre
pv-face-shape-oval = Ovale
pv-face-shape-round = Rond
pv-face-shape-square = Carré
pv-face-shape-oblong = Allongé
pv-face-shape-heart = En cœur
pv-face-shape-diamond = En losange
pv-face-shape-triangular = Triangulaire
pv-nose-shape-straight = Droit
pv-nose-shape-aquiline = Aquilin
pv-nose-shape-snub = Camus
pv-nose-shape-upturned = Retroussé
pv-nose-shape-flat = Aplati
pv-nose-shape-broad = Large
pv-nose-shape-bulbous = Bulbeux
pv-nose-shape-crooked = Dévié
pv-nose-shape-other = Autre
pv-ear-shape-free-lobe = Lobes détachés
pv-ear-shape-attached-lobe = Lobes attachés
pv-ear-shape-protruding = Décollées
pv-ear-shape-close-set = Plaquées
pv-ear-shape-pointed = Pointues
pv-ear-shape-other = Autre
pv-lip-shape-thin = Fines
pv-lip-shape-medium = Moyennes
pv-lip-shape-full = Charnues
pv-lip-shape-bow-shaped = En arc
pv-lip-shape-wide = Larges
pv-lip-shape-downturned = Tombantes
pv-lip-shape-other = Autre
pv-dentition-primary = Dents de lait
pv-dentition-mixed = Mixte
pv-dentition-permanent-complete = Définitive, complète
pv-dentition-permanent-partial-loss = Définitive, incomplète
pv-dentition-edentulous = Édenté
pv-dentition-partial-denture = Prothèse partielle
pv-dentition-full-denture = Prothèse complète
pv-dentition-implants = Implants dentaires
pv-malocclusion-normal = Occlusion normale
pv-malocclusion-class-i = Classe I
pv-malocclusion-class-ii-division-1 = Classe II, division 1
pv-malocclusion-class-ii-division-2 = Classe II, division 2
pv-malocclusion-class-iii = Classe III
pv-posture-ideal = Idéale
pv-posture-kyphotic-lordotic = Cyphose-lordose
pv-posture-flat-back = Dos plat
pv-posture-sway-back = Dos creux
pv-posture-stooped = Voûtée
pv-posture-scoliotic = Scoliotique
pv-posture-other = Autre
pv-gait-brisk = Vive
pv-gait-average = Moyenne
pv-gait-slow = Lente
pv-gait-shuffling = Traînante
pv-gait-limping = Boiteuse
pv-gait-waddling = Dandinante
pv-gait-unsteady = Instable
pv-gait-stiff = Raide
pv-gait-other = Autre
pv-vocal-timbre-bright = Clair
pv-vocal-timbre-dark = Sombre
pv-vocal-timbre-warm = Chaleureux
pv-vocal-timbre-breathy = Voilé
pv-vocal-timbre-nasal = Nasal
pv-vocal-timbre-hoarse = Rauque
pv-vocal-timbre-resonant = Sonore
pv-vocal-timbre-thin = Grêle
pv-vocal-timbre-other = Autre
pv-speech-register-frozen = Figé
pv-speech-register-formal = Soutenu
pv-speech-register-consultative = Courant
pv-speech-register-casual = Familier
pv-speech-register-intimate = Intime
pv-handedness-left = Gaucher
pv-handedness-right = Droitier
pv-handedness-ambidextrous = Ambidextre
pv-handedness-mixed = Mixte
pv-handedness-unknown = Inconnue
pv-hearing-grade-normal = Normale
pv-hearing-grade-mild = Légère
pv-hearing-grade-moderate = Moyenne
pv-hearing-grade-moderately-severe = Moyennement sévère
pv-hearing-grade-severe = Sévère
pv-hearing-grade-profound = Profonde
pv-hearing-grade-complete = Totale
pv-optical-correction-none = Aucune
pv-optical-correction-glasses = Lunettes
pv-optical-correction-contact-lenses = Lentilles de contact
pv-optical-correction-glasses-and-contact-lenses = Lunettes et lentilles
pv-optical-correction-refractive-surgery = Chirurgie réfractive
pv-optical-correction-intraocular-lens = Lentille intraoculaire
pv-optical-correction-other = Autre
pv-rhesus-positive = RhD positif
pv-rhesus-negative = RhD négatif
pv-rhesus-weak-d = D faible
pv-rhesus-unknown = Inconnu
pv-icd10-chapter-infectious-parasitic = I Maladies infectieuses et parasitaires
pv-icd10-chapter-neoplasms = II Tumeurs
pv-icd10-chapter-blood-immune = III Sang et système immunitaire
pv-icd10-chapter-endocrine-metabolic = IV Endocriniennes, nutritionnelles et métaboliques
pv-icd10-chapter-mental-behavioural = V Troubles mentaux et du comportement
pv-icd10-chapter-nervous-system = VI Système nerveux
pv-icd10-chapter-eye-adnexa = VII Œil et annexes
pv-icd10-chapter-ear-mastoid = VIII Oreille et apophyse mastoïde
pv-icd10-chapter-circulatory = IX Appareil circulatoire
pv-icd10-chapter-respiratory = X Appareil respiratoire
pv-icd10-chapter-digestive = XI Appareil digestif
pv-icd10-chapter-skin = XII Peau et tissu cellulaire sous-cutané
pv-icd10-chapter-musculoskeletal = XIII Système ostéo-articulaire
pv-icd10-chapter-genitourinary = XIV Appareil génito-urinaire
pv-icd10-chapter-pregnancy-childbirth = XV Grossesse et accouchement
pv-icd10-chapter-perinatal = XVI Affections périnatales
pv-icd10-chapter-congenital = XVII Malformations congénitales
pv-icd10-chapter-ill-defined = XVIII Symptômes et causes mal définies
pv-icd10-chapter-injury-poisoning = XIX Lésions traumatiques et empoisonnements
pv-icd10-chapter-external-causes = XX Causes externes
pv-icd10-chapter-health-factors = XXI Facteurs influant sur l’état de santé
pv-icd10-chapter-special-purposes = XXII Codes d’utilisation particulière
pv-diagnosis-status-diagnosed = Diagnostiquée
pv-diagnosis-status-suspected = Suspectée
pv-diagnosis-status-self-reported = Déclarée
pv-diagnosis-status-unknown = Inconnu
pv-prosthesis-kind-limb = Membre
pv-prosthesis-kind-joint = Articulation
pv-prosthesis-kind-ocular = Oculaire
pv-prosthesis-kind-dental = Dentaire
pv-prosthesis-kind-auditory = Auditive
pv-prosthesis-kind-breast = Mammaire
pv-prosthesis-kind-other = Autre
pv-implant-kind-orthopaedic = Orthopédique
pv-implant-kind-dental = Dentaire
pv-implant-kind-cochlear = Cochléaire
pv-implant-kind-breast = Mammaire
pv-implant-kind-intraocular-lens = Lentille intraoculaire
pv-implant-kind-contraceptive = Contraceptif
pv-implant-kind-cosmetic = Esthétique
pv-implant-kind-other = Autre
pv-device-kind-pacemaker = Stimulateur cardiaque
pv-device-kind-implantable-defibrillator = Défibrillateur implantable
pv-device-kind-cardiac-resynchronisation = Dispositif de resynchronisation
pv-device-kind-ventricular-assist = Assistance ventriculaire
pv-device-kind-neurostimulator = Neurostimulateur
pv-device-kind-insulin-pump = Pompe à insuline
pv-device-kind-drug-port = Chambre implantable
pv-device-kind-shunt = Dérivation
pv-device-kind-stent = Stent
pv-device-kind-other = Autre
pv-allergy-type-drug = Médicament
pv-allergy-type-food = Aliment
pv-allergy-type-environmental = Environnement
pv-allergy-type-insect-venom = Venin d’insecte
pv-allergy-type-latex = Latex
pv-allergy-type-other = Autre
pv-allergy-severity-mild = Légère
pv-allergy-severity-moderate = Modérée
pv-allergy-severity-severe = Sévère
pv-allergy-severity-anaphylactic = Anaphylactique
pv-allergy-severity-unknown = Inconnue
pv-pathogen-diphtheria = Diphtérie
pv-pathogen-tetanus = Tétanos
pv-pathogen-pertussis = Coqueluche
pv-pathogen-poliomyelitis = Poliomyélite
pv-pathogen-measles = Rougeole
pv-pathogen-mumps = Oreillons
pv-pathogen-rubella = Rubéole
pv-pathogen-varicella = Varicelle
pv-pathogen-smallpox = Variole
pv-pathogen-tuberculosis = Tuberculose
pv-pathogen-hepatitis-a = Hépatite A
pv-pathogen-hepatitis-b = Hépatite B
pv-pathogen-hepatitis-c = Hépatite C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae de type b
pv-pathogen-pneumococcal = Infections à pneumocoque
pv-pathogen-meningococcal = Infections à méningocoque
pv-pathogen-human-papillomavirus = Papillomavirus humain
pv-pathogen-influenza = Grippe
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotavirus
pv-pathogen-yellow-fever = Fièvre jaune
pv-pathogen-typhoid = Typhoïde
pv-pathogen-cholera = Choléra
pv-pathogen-rabies = Rage
pv-pathogen-japanese-encephalitis = Encéphalite japonaise
pv-pathogen-tick-borne-encephalitis = Encéphalite à tiques
pv-pathogen-hiv = VIH
pv-pathogen-syphilis = Syphilis
pv-pathogen-toxoplasmosis = Toxoplasmose
pv-pathogen-cytomegalovirus = Cytomégalovirus
pv-pathogen-epstein-barr = Virus d’Epstein-Barr
pv-pathogen-other = Autre
pv-vaccination-status-vaccinated = Vacciné
pv-vaccination-status-partially-vaccinated = Partiellement vacciné
pv-vaccination-status-unvaccinated = Non vacciné
pv-vaccination-status-contraindicated = Contre-indiqué
pv-vaccination-status-unknown = Inconnu
pv-serology-result-positive = Positive
pv-serology-result-negative = Négative
pv-serology-result-equivocal = Douteuse
pv-serology-result-unknown = Inconnue
pv-lab-panel-basic-metabolic = Bilan métabolique de base
pv-lab-panel-lipid = Bilan lipidique
pv-lab-panel-liver = Bilan hépatique
pv-lab-panel-renal = Bilan rénal
pv-lab-panel-glycated-haemoglobin = Hémoglobine glyquée
pv-lab-panel-iron = Bilan martial
pv-lab-analyte-sodium = Sodium
pv-lab-analyte-potassium = Potassium
pv-lab-analyte-chloride = Chlore
pv-lab-analyte-bicarbonate = Bicarbonates
pv-lab-analyte-urea = Urée
pv-lab-analyte-creatinine = Créatinine
pv-lab-analyte-glucose = Glucose
pv-lab-analyte-calcium = Calcium
pv-lab-analyte-total-cholesterol = Cholestérol total
pv-lab-analyte-ldl-cholesterol = Cholestérol LDL
pv-lab-analyte-hdl-cholesterol = Cholestérol HDL
pv-lab-analyte-triglycerides = Triglycérides
pv-lab-analyte-non-hdl-cholesterol = Cholestérol non-HDL
pv-lab-analyte-alt = Alanine aminotransférase (ALAT)
pv-lab-analyte-ast = Aspartate aminotransférase (ASAT)
pv-lab-analyte-alp = Phosphatases alcalines (PAL)
pv-lab-analyte-ggt = Gamma-glutamyl-transférase (GGT)
pv-lab-analyte-total-bilirubin = Bilirubine totale
pv-lab-analyte-direct-bilirubin = Bilirubine conjuguée
pv-lab-analyte-albumin = Albumine
pv-lab-analyte-total-protein = Protéines totales
pv-lab-analyte-egfr = DFG estimé
pv-lab-analyte-uric-acid = Acide urique
pv-lab-analyte-phosphate = Phosphate
pv-lab-analyte-urine-albumin-creatinine-ratio = Rapport albumine/créatinine urinaire
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Fer sérique
pv-lab-analyte-ferritin = Ferritine
pv-lab-analyte-transferrin = Transferrine
pv-lab-analyte-transferrin-saturation = Coefficient de saturation de la transferrine
pv-lab-analyte-tibc = Capacité totale de fixation du fer
pv-lab-flag-low = Bas
pv-lab-flag-normal = Normal
pv-lab-flag-high = Élevé
pv-lab-flag-critical-low = Critique bas
pv-lab-flag-critical-high = Critique haut
pv-nutrient-vitamin-a = Vitamine A
pv-nutrient-thiamine = Thiamine (B1)
pv-nutrient-riboflavin = Riboflavine (B2)
pv-nutrient-niacin = Niacine (B3)
pv-nutrient-vitamin-b6 = Vitamine B6
pv-nutrient-folate = Folates (B9)
pv-nutrient-vitamin-b12 = Vitamine B12
pv-nutrient-vitamin-c = Vitamine C
pv-nutrient-vitamin-d = Vitamine D
pv-nutrient-vitamin-e = Vitamine E
pv-nutrient-vitamin-k = Vitamine K
pv-nutrient-iron = Fer
pv-nutrient-zinc = Zinc
pv-nutrient-magnesium = Magnésium
pv-nutrient-calcium = Calcium
pv-nutrient-iodine = Iode
pv-nutrient-selenium = Sélénium
pv-nutrient-copper = Cuivre
pv-nutrient-potassium = Potassium
pv-nutrient-phosphorus = Phosphore
pv-nutrient-other = Autre
pv-sleep-disorder-insomnia = Insomnie
pv-sleep-disorder-sleep-related-breathing = Trouble respiratoire du sommeil
pv-sleep-disorder-central-hypersomnolence = Hypersomnolence centrale
pv-sleep-disorder-circadian-rhythm = Trouble du rythme circadien
pv-sleep-disorder-parasomnia = Parasomnie
pv-sleep-disorder-sleep-related-movement = Trouble moteur du sommeil
pv-sleep-disorder-other = Autre
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Entretien clinique
pv-assessment-instrument-other = Autre
pv-assessment-severity-none-minimal = Nulle ou minime
pv-assessment-severity-mild = Légère
pv-assessment-severity-moderate = Modérée
pv-assessment-severity-moderately-severe = Modérément sévère
pv-assessment-severity-severe = Sévère
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Données brutes de puce
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Autre
pv-zygosity-heterozygous = Hétérozygote
pv-zygosity-homozygous = Homozygote
pv-zygosity-hemizygous = Hémizygote
pv-zygosity-compound-heterozygous = Hétérozygote composite
pv-clinical-significance-pathogenic = Pathogène
pv-clinical-significance-likely-pathogenic = Probablement pathogène
pv-clinical-significance-uncertain-significance = Signification incertaine
pv-clinical-significance-likely-benign = Probablement bénin
pv-clinical-significance-benign = Bénin
pv-inheritance-pattern-autosomal-dominant = Autosomique dominante
pv-inheritance-pattern-autosomal-recessive = Autosomique récessive
pv-inheritance-pattern-x-linked-dominant = Dominante liée à l’X
pv-inheritance-pattern-x-linked-recessive = Récessive liée à l’X
pv-inheritance-pattern-y-linked = Liée à l’Y
pv-inheritance-pattern-mitochondrial = Mitochondriale
pv-inheritance-pattern-multifactorial = Multifactorielle
pv-inheritance-pattern-unknown = Inconnue
pv-carrier-status-affected = Atteint
pv-carrier-status-carrier = Porteur
pv-carrier-status-not-carrier = Non porteur
pv-carrier-status-unknown = Inconnu
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Autre
pv-metaboliser-status-poor = Métaboliseur lent
pv-metaboliser-status-intermediate = Métaboliseur intermédiaire
pv-metaboliser-status-normal = Métaboliseur normal
pv-metaboliser-status-rapid = Métaboliseur rapide
pv-metaboliser-status-ultrarapid = Métaboliseur ultrarapide
pv-autopsy-not-performed = Non pratiquée
pv-autopsy-clinical = Clinique
pv-autopsy-forensic = Médico-légale
pv-autopsy-external-examination = Examen externe seulement
pv-autopsy-unknown = Inconnue
pv-disposition-burial = Inhumation
pv-disposition-cremation = Crémation
pv-disposition-entombment = Mise au tombeau
pv-disposition-burial-at-sea = Immersion en mer
pv-disposition-natural-burial = Inhumation naturelle
pv-disposition-body-donation = Don du corps à la science
pv-disposition-other = Autre
pv-disposition-unknown = Inconnu
pv-address-use-principal = Résidence principale
pv-address-use-secondary = Résidence secondaire
pv-address-use-temporary = Résidence temporaire
pv-address-use-postal = Adresse postale
pv-address-use-other = Autre
pv-nationality-mode-descent = Par filiation
pv-nationality-mode-birth-in-territory = Par naissance sur le territoire
pv-nationality-mode-naturalisation = Par naturalisation
pv-nationality-mode-marriage = Par mariage
pv-nationality-mode-registration = Par déclaration
pv-nationality-mode-restoration = Par réintégration
pv-nationality-mode-state-succession = Par changement de souveraineté
pv-nationality-mode-other = Autre
pv-language-proficiency-a1 = A1 Découverte
pv-language-proficiency-a2 = A2 Survie
pv-language-proficiency-b1 = B1 Seuil
pv-language-proficiency-b2 = B2 Avancé
pv-language-proficiency-c1 = C1 Autonome
pv-language-proficiency-c2 = C2 Maîtrise
pv-language-proficiency-native = Langue première
pv-isced-level-isced-0 = 0 Petite enfance
pv-isced-level-isced-1 = 1 Primaire
pv-isced-level-isced-2 = 2 Premier cycle du secondaire
pv-isced-level-isced-3 = 3 Second cycle du secondaire
pv-isced-level-isced-4 = 4 Post-secondaire non supérieur
pv-isced-level-isced-5 = 5 Supérieur de cycle court
pv-isced-level-isced-6 = 6 Licence ou équivalent
pv-isced-level-isced-7 = 7 Master ou équivalent
pv-isced-level-isced-8 = 8 Doctorat ou équivalent
pv-income-quintile-q1 = Cinquième le plus bas
pv-income-quintile-q2 = Deuxième cinquième
pv-income-quintile-q3 = Cinquième médian
pv-income-quintile-q4 = Quatrième cinquième
pv-income-quintile-q5 = Cinquième le plus élevé
pv-pay-period-hourly = Par heure
pv-pay-period-daily = Par jour
pv-pay-period-weekly = Par semaine
pv-pay-period-monthly = Par mois
pv-pay-period-annual = Par an
pv-tenure-owned = Propriété
pv-tenure-co-owned = Copropriété
pv-tenure-leasehold = Bail emphytéotique
pv-tenure-rented = Location
pv-tenure-usufruct = Usufruit
pv-tenure-other = Autre
pv-distinction-kind-order = Ordre
pv-distinction-kind-decoration = Décoration
pv-distinction-kind-medal = Médaille
pv-distinction-kind-title = Titre honorifique
pv-distinction-kind-other = Autre
pv-military-service-army = Armée de terre
pv-military-service-navy = Marine
pv-military-service-air-force = Armée de l’air
pv-military-service-marines = Infanterie de marine
pv-military-service-gendarmerie = Gendarmerie
pv-military-service-border-guard = Garde-frontières
pv-military-service-national-guard = Garde nationale
pv-military-service-other = Autre
pv-rank-category-enlisted = Militaire du rang
pv-rank-category-non-commissioned = Sous-officier
pv-rank-category-warrant = Adjudant ou équivalent
pv-rank-category-officer-cadet = Élève officier
pv-rank-category-junior-officer = Officier subalterne
pv-rank-category-senior-officer = Officier supérieur
pv-rank-category-general-officer = Officier général
pv-iccs-section-acts-leading-to-death = 01 Actes ayant entraîné la mort
pv-iccs-section-acts-causing-harm = 02 Actes causant un préjudice
pv-iccs-section-sexual-acts = 03 Actes préjudiciables de nature sexuelle
pv-iccs-section-property-with-violence = 04 Contre les biens, avec violence
pv-iccs-section-property-only = 05 Contre les biens uniquement
pv-iccs-section-controlled-substances = 06 Substances contrôlées
pv-iccs-section-fraud-deception-corruption = 07 Fraude, tromperie ou corruption
pv-iccs-section-public-order-and-state = 08 Contre l’ordre public et l’État
pv-iccs-section-public-safety-and-security = 09 Contre la sécurité publique
pv-iccs-section-natural-environment = 10 Contre l’environnement naturel
pv-iccs-section-other-criminal-acts = 11 Autres actes criminels
pv-case-outcome-convicted = Condamné
pv-case-outcome-acquitted = Acquitté
pv-case-outcome-dismissed = Non-lieu
pv-case-outcome-conviction-quashed = Condamnation annulée
pv-case-outcome-pardoned = Gracié
pv-case-outcome-amnestied = Amnistié
pv-case-outcome-expunged = Effacé du casier
pv-case-outcome-pending = En cours
pv-case-outcome-unknown = Inconnue
pv-religion-buddhism = Bouddhisme
pv-religion-christianity-catholic = Christianisme : catholique
pv-religion-christianity-orthodox = Christianisme : orthodoxe
pv-religion-christianity-protestant = Christianisme : protestant
pv-religion-christianity-other = Christianisme : autre
pv-religion-hinduism = Hindouisme
pv-religion-islam-sunni = Islam : sunnite
pv-religion-islam-shia = Islam : chiite
pv-religion-islam-other = Islam : autre
pv-religion-jainism = Jaïnisme
pv-religion-judaism = Judaïsme
pv-religion-sikhism = Sikhisme
pv-religion-bahai = Foi bahá’íe
pv-religion-shinto = Shintoïsme
pv-religion-taoism = Taoïsme
pv-religion-zoroastrianism = Zoroastrisme
pv-religion-traditional = Religion traditionnelle ou populaire
pv-religion-other = Autre
pv-religion-none = Sans religion
pv-religion-unknown = Inconnue
pv-sacrament-baptism = Baptême
pv-sacrament-confirmation = Confirmation
pv-sacrament-first-communion = Première communion
pv-sacrament-reconciliation = Réconciliation
pv-sacrament-anointing-of-the-sick = Onction des malades
pv-sacrament-holy-orders = Ordre
pv-sacrament-matrimony = Mariage
pv-sacrament-other-rite = Autre rite
pv-political-position-far-left = Extrême gauche
pv-political-position-left = Gauche
pv-political-position-centre-left = Centre gauche
pv-political-position-centre = Centre
pv-political-position-centre-right = Centre droit
pv-political-position-right = Droite
pv-political-position-far-right = Extrême droite
pv-political-position-apolitical = Apolitique
pv-political-position-other = Hors de cet axe
pv-political-position-unknown = Inconnue
pv-membership-kind-trade-union = Syndicat
pv-membership-kind-political-party = Parti politique
pv-membership-kind-professional-body = Ordre professionnel
pv-membership-kind-religious-order = Ordre religieux
pv-membership-kind-religious-association = Association religieuse
pv-membership-kind-fraternal-order = Ordre fraternel
pv-membership-kind-veterans-association = Association d’anciens combattants
pv-membership-kind-sports-club = Club sportif
pv-membership-kind-cultural-association = Association culturelle
pv-membership-kind-charitable-association = Association caritative
pv-membership-kind-other = Autre
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Évalué par un proche
pv-personality-instrument-inferred = Déduit des archives
pv-personality-instrument-other = Autre
pv-introversion-extraversion-strongly-introverted = Très introverti
pv-introversion-extraversion-introverted = Introverti
pv-introversion-extraversion-ambiverted = Ambiverti
pv-introversion-extraversion-extraverted = Extraverti
pv-introversion-extraversion-strongly-extraverted = Très extraverti
pv-stress-tolerance-very-low = Très faible
pv-stress-tolerance-low = Faible
pv-stress-tolerance-moderate = Modérée
pv-stress-tolerance-high = Élevée
pv-stress-tolerance-very-high = Très élevée
pv-decision-style-rational = Rationnel
pv-decision-style-intuitive = Intuitif
pv-decision-style-dependent = Dépendant
pv-decision-style-avoidant = Évitant
pv-decision-style-spontaneous = Spontané
pv-sport-level-recreational = Loisir
pv-sport-level-amateur-competitive = Compétition amateur
pv-sport-level-semi-professional = Semi-professionnel
pv-sport-level-professional = Professionnel
pv-diet-omnivore = Omnivore
pv-diet-flexitarian = Flexitarien
pv-diet-pescatarian = Pescétarien
pv-diet-vegetarian = Végétarien
pv-diet-vegan = Végétalien
pv-diet-other = Autre
pv-substance-tobacco = Tabac et nicotine
pv-substance-alcohol = Alcool
pv-substance-cannabis = Cannabis
pv-substance-opioids = Opioïdes
pv-substance-stimulants = Stimulants
pv-substance-sedatives-hypnotics = Sédatifs et hypnotiques
pv-substance-hallucinogens = Hallucinogènes
pv-substance-inhalants = Solvants inhalés
pv-substance-gambling = Jeux d’argent
pv-substance-gaming = Jeux vidéo
pv-substance-other = Autre
pv-use-pattern-occasional-use = Usage occasionnel
pv-use-pattern-regular-use = Usage régulier
pv-use-pattern-harmful-use = Usage nocif
pv-use-pattern-dependence = Dépendance
pv-use-pattern-in-remission = En rémission
pv-lineage-biological = Biologique
pv-lineage-adoptive = Adoptive
pv-lineage-foster = D’accueil
pv-lineage-step = Par alliance
pv-lineage-guardianship = Sous tutelle
pv-lineage-unknown = Inconnue
pv-link-relation-godparent = Parrain ou marraine
pv-link-relation-godchild = Filleul
pv-link-relation-witness = Témoin
pv-link-relation-officiant = Officiant
pv-link-relation-business-partner = Associé
pv-link-relation-employer = Employeur
pv-link-relation-employee = Employé
pv-link-relation-mentor = Mentor
pv-link-relation-apprentice = Apprenti
pv-link-relation-close-friend = Ami proche
pv-link-relation-neighbour = Voisin
pv-link-relation-guardian = Tuteur
pv-link-relation-ward = Pupille
pv-link-relation-other = Autre
pv-country-AD = Andorre
pv-country-AE = Émirats arabes unis
pv-country-AF = Afghanistan
pv-country-AG = Antigua-et-Barbuda
pv-country-AI = Anguilla
pv-country-AL = Albanie
pv-country-AM = Arménie
pv-country-AO = Angola
pv-country-AQ = Antarctique
pv-country-AR = Argentine
pv-country-AS = Samoa américaines
pv-country-AT = Autriche
pv-country-AU = Australie
pv-country-AW = Aruba
pv-country-AX = Îles Åland
pv-country-AZ = Azerbaïdjan
pv-country-BA = Bosnie-Herzégovine
pv-country-BB = Barbade
pv-country-BD = Bangladesh
pv-country-BE = Belgique
pv-country-BF = Burkina Faso
pv-country-BG = Bulgarie
pv-country-BH = Bahreïn
pv-country-BI = Burundi
pv-country-BJ = Bénin
pv-country-BL = Saint-Barthélemy
pv-country-BM = Bermudes
pv-country-BN = Brunei
pv-country-BO = Bolivie
pv-country-BQ = Pays-Bas caribéens
pv-country-BR = Brésil
pv-country-BS = Bahamas
pv-country-BT = Bhoutan
pv-country-BV = Île Bouvet
pv-country-BW = Botswana
pv-country-BY = Biélorussie
pv-country-BZ = Belize
pv-country-CA = Canada
pv-country-CC = Îles Cocos
pv-country-CD = Congo-Kinshasa
pv-country-CF = République centrafricaine
pv-country-CG = Congo-Brazzaville
pv-country-CH = Suisse
pv-country-CI = Côte d’Ivoire
pv-country-CK = Îles Cook
pv-country-CL = Chili
pv-country-CM = Cameroun
pv-country-CN = Chine
pv-country-CO = Colombie
pv-country-CR = Costa Rica
pv-country-CU = Cuba
pv-country-CV = Cap-Vert
pv-country-CW = Curaçao
pv-country-CX = Île Christmas
pv-country-CY = Chypre
pv-country-CZ = Tchéquie
pv-country-DE = Allemagne
pv-country-DJ = Djibouti
pv-country-DK = Danemark
pv-country-DM = Dominique
pv-country-DO = République dominicaine
pv-country-DZ = Algérie
pv-country-EC = Équateur
pv-country-EE = Estonie
pv-country-EG = Égypte
pv-country-EH = Sahara occidental
pv-country-ER = Érythrée
pv-country-ES = Espagne
pv-country-ET = Éthiopie
pv-country-FI = Finlande
pv-country-FJ = Fidji
pv-country-FK = Îles Malouines
pv-country-FM = Micronésie
pv-country-FO = Îles Féroé
pv-country-FR = France
pv-country-GA = Gabon
pv-country-GB = Royaume-Uni
pv-country-GD = Grenade
pv-country-GE = Géorgie
pv-country-GF = Guyane française
pv-country-GG = Guernesey
pv-country-GH = Ghana
pv-country-GI = Gibraltar
pv-country-GL = Groenland
pv-country-GM = Gambie
pv-country-GN = Guinée
pv-country-GP = Guadeloupe
pv-country-GQ = Guinée équatoriale
pv-country-GR = Grèce
pv-country-GS = Géorgie du Sud-et-les Îles Sandwich du Sud
pv-country-GT = Guatemala
pv-country-GU = Guam
pv-country-GW = Guinée-Bissau
pv-country-GY = Guyana
pv-country-HK = R.A.S. chinoise de Hong Kong
pv-country-HM = Îles Heard-et-MacDonald
pv-country-HN = Honduras
pv-country-HR = Croatie
pv-country-HT = Haïti
pv-country-HU = Hongrie
pv-country-ID = Indonésie
pv-country-IE = Irlande
pv-country-IL = Israël
pv-country-IM = Île de Man
pv-country-IN = Inde
pv-country-IO = Territoire britannique de l’océan Indien
pv-country-IQ = Irak
pv-country-IR = Iran
pv-country-IS = Islande
pv-country-IT = Italie
pv-country-JE = Jersey
pv-country-JM = Jamaïque
pv-country-JO = Jordanie
pv-country-JP = Japon
pv-country-KE = Kenya
pv-country-KG = Kirghizstan
pv-country-KH = Cambodge
pv-country-KI = Kiribati
pv-country-KM = Comores
pv-country-KN = Saint-Christophe-et-Niévès
pv-country-KP = Corée du Nord
pv-country-KR = Corée du Sud
pv-country-KW = Koweït
pv-country-KY = Îles Caïmans
pv-country-KZ = Kazakhstan
pv-country-LA = Laos
pv-country-LB = Liban
pv-country-LC = Sainte-Lucie
pv-country-LI = Liechtenstein
pv-country-LK = Sri Lanka
pv-country-LR = Liberia
pv-country-LS = Lesotho
pv-country-LT = Lituanie
pv-country-LU = Luxembourg
pv-country-LV = Lettonie
pv-country-LY = Libye
pv-country-MA = Maroc
pv-country-MC = Monaco
pv-country-MD = Moldavie
pv-country-ME = Monténégro
pv-country-MF = Saint-Martin
pv-country-MG = Madagascar
pv-country-MH = Îles Marshall
pv-country-MK = Macédoine du Nord
pv-country-ML = Mali
pv-country-MM = Myanmar (Birmanie)
pv-country-MN = Mongolie
pv-country-MO = R.A.S. chinoise de Macao
pv-country-MP = Îles Mariannes du Nord
pv-country-MQ = Martinique
pv-country-MR = Mauritanie
pv-country-MS = Montserrat
pv-country-MT = Malte
pv-country-MU = Maurice
pv-country-MV = Maldives
pv-country-MW = Malawi
pv-country-MX = Mexique
pv-country-MY = Malaisie
pv-country-MZ = Mozambique
pv-country-NA = Namibie
pv-country-NC = Nouvelle-Calédonie
pv-country-NE = Niger
pv-country-NF = Île Norfolk
pv-country-NG = Nigeria
pv-country-NI = Nicaragua
pv-country-NL = Pays-Bas
pv-country-NO = Norvège
pv-country-NP = Népal
pv-country-NR = Nauru
pv-country-NU = Niue
pv-country-NZ = Nouvelle-Zélande
pv-country-OM = Oman
pv-country-PA = Panama
pv-country-PE = Pérou
pv-country-PF = Polynésie française
pv-country-PG = Papouasie-Nouvelle-Guinée
pv-country-PH = Philippines
pv-country-PK = Pakistan
pv-country-PL = Pologne
pv-country-PM = Saint-Pierre-et-Miquelon
pv-country-PN = Îles Pitcairn
pv-country-PR = Porto Rico
pv-country-PS = Territoires palestiniens
pv-country-PT = Portugal
pv-country-PW = Palaos
pv-country-PY = Paraguay
pv-country-QA = Qatar
pv-country-RE = La Réunion
pv-country-RO = Roumanie
pv-country-RS = Serbie
pv-country-RU = Russie
pv-country-RW = Rwanda
pv-country-SA = Arabie saoudite
pv-country-SB = Îles Salomon
pv-country-SC = Seychelles
pv-country-SD = Soudan
pv-country-SE = Suède
pv-country-SG = Singapour
pv-country-SH = Sainte-Hélène
pv-country-SI = Slovénie
pv-country-SJ = Svalbard et Jan Mayen
pv-country-SK = Slovaquie
pv-country-SL = Sierra Leone
pv-country-SM = Saint-Marin
pv-country-SN = Sénégal
pv-country-SO = Somalie
pv-country-SR = Suriname
pv-country-SS = Soudan du Sud
pv-country-ST = Sao Tomé-et-Principe
pv-country-SV = Salvador
pv-country-SX = Saint-Martin (partie néerlandaise)
pv-country-SY = Syrie
pv-country-SZ = Eswatini
pv-country-TC = Îles Turques-et-Caïques
pv-country-TD = Tchad
pv-country-TF = Terres australes françaises
pv-country-TG = Togo
pv-country-TH = Thaïlande
pv-country-TJ = Tadjikistan
pv-country-TK = Tokelau
pv-country-TL = Timor oriental
pv-country-TM = Turkménistan
pv-country-TN = Tunisie
pv-country-TO = Tonga
pv-country-TR = Turquie
pv-country-TT = Trinité-et-Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taïwan
pv-country-TZ = Tanzanie
pv-country-UA = Ukraine
pv-country-UG = Ouganda
pv-country-UM = Îles mineures éloignées des États-Unis
pv-country-US = États-Unis
pv-country-UY = Uruguay
pv-country-UZ = Ouzbékistan
pv-country-VA = État de la Cité du Vatican
pv-country-VC = Saint-Vincent-et-les Grenadines
pv-country-VE = Venezuela
pv-country-VG = Îles Vierges britanniques
pv-country-VI = Îles Vierges des États-Unis
pv-country-VN = Viêt Nam
pv-country-VU = Vanuatu
pv-country-WF = Wallis-et-Futuna
pv-country-WS = Samoa
pv-country-YE = Yémen
pv-country-YT = Mayotte
pv-country-ZA = Afrique du Sud
pv-country-ZM = Zambie
pv-country-ZW = Zimbabwe
pv-country-SU = Union soviétique
pv-country-DD = Allemagne de l’Est
pv-country-YU = Yougoslavie
pv-country-CS = Tchécoslovaquie
pv-country-OT = Empire ottoman
lang-aa = Afar
lang-ab = Abkhaze
lang-ae = Avestique
lang-af = Afrikaans
lang-ak = Akan
lang-am = Amharique
lang-an = Aragonais
lang-ar = Arabe
lang-as = Assamais
lang-av = Avar
lang-ay = Aymara
lang-az = Azerbaïdjanais
lang-ba = Bachkir
lang-be = Biélorusse
lang-bg = Bulgare
lang-bi = Bichelamar
lang-bm = Bambara
lang-bn = Bengali
lang-bo = Tibétain
lang-br = Breton
lang-bs = Bosniaque
lang-ca = Catalan
lang-ce = Tchétchène
lang-ch = Chamorro
lang-co = Corse
lang-cr = Cree
lang-cs = Tchèque
lang-cu = Slavon d’église
lang-cv = Tchouvache
lang-cy = Gallois
lang-da = Danois
lang-de = Allemand
lang-dv = Maldivien
lang-dz = Dzongkha
lang-ee = Éwé
lang-el = Grec
lang-en = Anglais
lang-eo = Espéranto
lang-es = Espagnol
lang-et = Estonien
lang-eu = Basque
lang-fa = Persan
lang-ff = Peul
lang-fi = Finnois
lang-fj = Fidjien
lang-fo = Féroïen
lang-fr = Français
lang-fy = Frison occidental
lang-ga = Irlandais
lang-gd = Gaélique écossais
lang-gl = Galicien
lang-gn = Guarani
lang-gu = Goudjarati
lang-gv = Mannois
lang-ha = Haoussa
lang-he = Hébreu
lang-hi = Hindi
lang-ho = Hiri motu
lang-hr = Croate
lang-ht = Créole haïtien
lang-hu = Hongrois
lang-hy = Arménien
lang-hz = Héréro
lang-ia = Interlingua
lang-id = Indonésien
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Yi du Sichuan
lang-ik = Inupiaq
lang-io = Ido
lang-is = Islandais
lang-it = Italien
lang-iu = Inuktitut
lang-ja = Japonais
lang-jv = Javanais
lang-ka = Géorgien
lang-kg = Kikongo
lang-ki = Kikuyu
lang-kj = Kuanyama
lang-kk = Kazakh
lang-kl = Groenlandais
lang-km = Khmer
lang-kn = Kannada
lang-ko = Coréen
lang-kr = Kanouri
lang-ks = Cachemiri
lang-ku = Kurde
lang-kv = Komi
lang-kw = Cornique
lang-ky = Kirghize
lang-la = Latin
lang-lb = Luxembourgeois
lang-lg = Ganda
lang-li = Limbourgeois
lang-ln = Lingala
lang-lo = Lao
lang-lt = Lituanien
lang-lu = Luba-katanga (kiluba)
lang-lv = Letton
lang-mg = Malgache
lang-mh = Marshallais
lang-mi = Maori
lang-mk = Macédonien
lang-ml = Malayalam
lang-mn = Mongol
lang-mr = Marathi
lang-ms = Malais
lang-mt = Maltais
lang-my = Birman
lang-na = Nauruan
lang-nb = Norvégien bokmål
lang-nd = Ndébélé du Nord
lang-ne = Népalais
lang-ng = Ndonga
lang-nl = Néerlandais
lang-nn = Norvégien nynorsk
lang-no = Norvégien
lang-nr = Ndébélé du Sud
lang-nv = Navajo
lang-ny = Chewa
lang-oc = Occitan
lang-oj = Ojibwa
lang-om = Oromo
lang-or = Odia
lang-os = Ossète
lang-pa = Pendjabi
lang-pi = Pali
lang-pl = Polonais
lang-ps = Pachto
lang-pt = Portugais
lang-qu = Quechua
lang-rm = Romanche
lang-rn = Roundi
lang-ro = Roumain
lang-ru = Russe
lang-rw = Kinyarwanda
lang-sa = Sanskrit
lang-sc = Sarde
lang-sd = Sindhi
lang-se = Same du Nord
lang-sg = Sango
lang-sh = Serbo-croate
lang-si = Cingalais
lang-sk = Slovaque
lang-sl = Slovène
lang-sm = Samoan
lang-sn = Shona
lang-so = Somali
lang-sq = Albanais
lang-sr = Serbe
lang-ss = Swati
lang-st = Sotho du Sud
lang-su = Soundanais
lang-sv = Suédois
lang-sw = Swahili
lang-ta = Tamoul
lang-te = Télougou
lang-tg = Tadjik
lang-th = Thaï
lang-ti = Tigrigna
lang-tk = Turkmène
lang-tl = Tagalog
lang-tn = Tswana
lang-to = Tongien
lang-tr = Turc
lang-ts = Tsonga
lang-tt = Tatar
lang-tw = Twi
lang-ty = Tahitien
lang-ug = Ouïghour
lang-uk = Ukrainien
lang-ur = Ourdou
lang-uz = Ouzbek
lang-ve = Venda
lang-vi = Vietnamien
lang-vo = Volapük
lang-wa = Wallon
lang-wo = Wolof
lang-xh = Xhosa
lang-yi = Yiddish
lang-yo = Yoruba
lang-za = Zhuang
lang-zh = Chinois
lang-zu = Zoulou
person-tab-profile = Profil
profile-groups-label = Rubriques du profil
profile-group-withheld = Une partie de cette rubrique ne vous est pas accessible
profile-withheld = Consigné pour cette personne et non accessible pour vous : { $classes }.
profile-empty = Rien n’est encore consigné dans cette rubrique.
profile-earlier = ancien formulaire
profile-earlier-title = Consigné par une version antérieure de cette application, dans un champ qui n’a pas de place dans AXGF 1.1. Il est conservé tel qu’il a été écrit.
profile-other-names = { $n ->
        [one] et un autre nom
       *[other] et { $n } autres noms
    }
profile-edit-group = Modifier « { $group } »
profile-summary-link = { $n ->
        [one] Un fait dans le profil
       *[other] { $n } faits dans le profil
    }
profile-from = depuis
profile-until = jusqu’à
profile-yes = Oui
profile-no = Non
profile-value = Valeur
profile-editor-title = Profil
profile-problems = Une partie de la saisie n’a pas pu être enregistrée. Chaque problème est indiqué à côté de son champ, et rien n’a été écrit.
profile-editor-withheld = Cette rubrique contient aussi, pour cette personne, des données de type { $classes } auxquelles vous n’avez pas accès. Elles ne sont pas affichées ici, et enregistrer ce formulaire les laisse telles quelles.
profile-living-class-note = Cette personne est enregistrée comme vivante. Ce que vous saisissez ici dans une catégorie sensible n’est visible que des administrateurs.
profile-relationships-elsewhere = Les parents, conjoints, enfants, parrains et témoins ne sont pas enregistrés sur cette personne. Ce sont des familles, des liens et des événements qui la nomment — chaque modification ici change donc aussi la fiche de chacune des autres personnes concernées.
profile-documents-first = Un artefact renvoie à un document rattaché à cette personne. Joignez d’abord le fichier.
profile-editor-nothing = Rien dans cette rubrique ne peut être modifié par vous.
profile-new-entry = Nouvelle entrée
profile-provenance = Date, source et degré de confiance
profile-from-date = Vrai à partir de
profile-until-date = Vrai jusqu’à
profile-remove-entry = Supprimer cette entrée
profile-add-entry = Ajouter une autre entrée
profile-no-such-group-title = Rubrique introuvable
profile-no-such-group-detail = Le profil ne comporte pas de rubrique de ce nom.
profile-error-number = Une valeur de ce champ doit être un nombre.
profile-error-integer = Une valeur de ce champ doit être un nombre entier.
profile-error-range = Un nombre sort de la plage admise pour cet attribut.
profile-error-term = Une valeur ne fait pas partie des choix proposés.
profile-error-required = Il manque à une entrée un champ indispensable.
profile-error-one-of = Une entrée doit comporter au moins un de ses champs principaux.
profile-error-confidence = La confiance va de 0 à 1, par exemple 0,8.
profile-error-time = Une heure s’écrit en heures et minutes, par exemple 05:40.
profile-error-currency = Une devise s’écrit avec son code à trois lettres, par exemple EUR.
profile-error-language = Une langue s’écrit avec son code, par exemple fr ou zh-Hans.
profile-error-coordinates = Des coordonnées demandent une latitude entre −90 et 90 et une longitude entre −180 et 180.
profile-error-rank-country = Le grade appartient à un autre pays que celui choisi.
record-unknown-place = [Lieu inconnu]
record-missing-document = [Document manquant]
