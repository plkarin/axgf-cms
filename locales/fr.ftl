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
account-emergency = Jeton d’urgence

## Préférences

settings-title = Réglages
edit-mode-label = Mode édition — ce que vous enregistrez modifie les fiches de la famille
display-menu-label = Langue et affichage
display-settings = Thème et affichage…
settings-tab-theme = Thème
settings-tab-language = Langue
settings-tab-appearance = Apparence
settings-done = Terminé
prefs-language = Langue
prefs-theme = Thème
prefs-background-on = Un voile de couleur léger derrière la page
prefs-apply = Appliquer
busy-saving = Enregistrement…
busy-working = En cours…
prefs-machine = automatique, { $coverage } %

theme-canopee = Canopée
theme-canopee-note = sombre, vert forêt · par défaut
theme-fusion = Fusion
theme-fusion-note = sombre, néon · un arbre vivant
theme-galaxy = Galaxie
theme-galaxy-note = espace profond · un système solaire
theme-summit = Sommet
theme-summit-note = sombre, aube dorée · une ascension

## Arbre

tree-title-around = Autour de { $name }
tree-title-whole = L'arbre entier
tree-lede-focused = { $ancestors ->
        [0] Aucun ascendant
        [one] Un ascendant
       *[other] { $ancestors } ascendants
    }, { $descendants ->
        [0] aucun descendant
        [one] un descendant
       *[other] { $descendants } descendants
    } et { $spouses ->
        [0] aucun conjoint
        [one] un conjoint
       *[other] { $spouses } conjoints
    }, { $depth } générations de chaque côté.
tree-filter-label = Filtrer les fiches affichées
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

record-identity-help = Chaque nom enregistré avec son type, la période où il a été porté et la source qui l'atteste, avec l'écriture d'origine et sa translittération latine côte à côte lorsqu'elles diffèrent, ainsi que le genre, le statut vivant et la visibilité.
record-life-events-help = Naissance, décès et tous les événements auxquels cette personne a pris part, par ordre de date, chacun avec son rôle — de sorte qu'un mariage dont elle n'a été que témoin figure à côté du sien. Un fait non daté est classé en dernier plutôt que de prétendre venir en premier.
record-family-help = Parents et fratrie, puis chaque union avec son type, ses dates, son lieu, la manière dont elle a pris fin et ses enfants par ordre de naissance.
record-other-relationships-help = Chaque lien dont cette personne est l'une des extrémités, lu de son côté, de sorte que la même relation se lit « parrain de » d'un côté et « filleul de » de l'autre.
record-places-help = Chaque lieu que cette fiche touche, avec ce qui s'y est passé et l'histoire des frontières qui donne son sens à un lieu à travers le temps.
record-sources-documents-help = Chaque source nomme les faits de cette page qui reposent sur elle, classées selon la force de la preuve.
record-notes-help = Notes sur cette fiche, y compris le texte qu'aucun convertisseur n'a su interpréter et qui a été conservé mot pour mot plutôt que supprimé.
record-history-help = Chaque modification enregistrée sur cette fiche, la plus récente en premier. Qui a corrigé quoi est un fait sur les personnes qui tiennent l’arbre, non sur la famille qui s’y trouve : c’est donc tenu hors de l’archive exportée et montré seulement aux proches connectés.
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
conf-percent = { $percent } %
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
error-merged-title = Cette fiche n’existe plus.
error-merged-detail = Elle a été fusionnée avec { $name } le { $date }. Les liens vers l’ancienne adresse mènent désormais vers la fiche fusionnée.
error-merged-open = Ouvrir la fiche fusionnée
error-back-to-river = Retour à la rivière
view-tree-back = Retour à l’arbre
view-galaxy-back = Retour à la galaxie
view-summit-back = Retour à la montagne
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
setup-title = Mettre en route cette archive
setup-lede = Cette installation ne contient encore aucune famille. Choisissez comment elle commence. Rien ici n’est jamais écrit par-dessus une famille : cette page se ferme dès qu’il y en a une.
setup-door-archive-title = À partir d’une archive AXGF
setup-door-archive-detail = Un fichier .axgf venu d’une autre installation ou d’une sauvegarde. Ses personnes, ses sources et ses photographies arrivent telles quelles.
setup-door-archive-file = Archive (.axgf)
setup-door-archive-go = Mettre en route avec cette archive
setup-limit-gb = Jusqu’à { $n } Go, écrits directement sur le disque.
setup-door-scratch-title = À partir de rien
setup-door-scratch-detail = Une archive vide. Ajoutez la première personne, puis sa famille, fiche après fiche.
setup-door-scratch-go = Ajouter la première personne
setup-door-gedcom-title = À partir d’un fichier GEDCOM
setup-door-gedcom-detail = Le fichier que tout logiciel de généalogie exporte : GEDCOM 5.5, 5.5.1, 5.5.5 ou 7.0, en .ged ou en .gdz, quel que soit son encodage. Vous voyez comment il a été lu et ce qui passe avant que rien ne soit gardé.
setup-door-gedcom-file = Fichier GEDCOM (.ged, .gdz)
setup-door-gedcom-go = Lire ce fichier
setup-never-over = Pour remplacer une famille déjà présente, restaurez une sauvegarde en ligne de commande (axgf-cms restore) ; aucune page n’écrit par-dessus.
setup-adopt-title = En faire la famille de l’archive
setup-adopt-detail = Fiches qui deviendraient la famille de cette installation : { $total }. Rien n’est gardé avant que vous n’appuyiez sur le bouton.
setup-adopt-go = Mettre en route l’archive avec cette famille
setup-adopt-back = Choisir une autre entrée
setup-done-title = L’archive est en route
setup-done-detail = Fiches dans la famille de cette installation désormais : { $total }.
setup-done-photos = Photographies et documents arrivés avec elle : { $n }.
setup-done-mismatches = Fichiers dont le contenu ne correspond pas à l’empreinte enregistrée : { $n }. Ils sont gardés ; vérifiez-les.
setup-done-river = Ouvrir la rivière
view-tree-open = Ouvrir l’arbre
view-galaxy-open = Ouvrir la galaxie
view-summit-open = Ouvrir la montagne
setup-done-admin = Administration
setup-refused-title = Ceci n’a pas pu devenir l’archive
setup-refused-detail = Rien n’a été écrit. Ce qui l’a empêché :
setup-closed-title = Cette archive est déjà en route
setup-closed-detail = Elle contient une famille, et aucune page n’écrit par-dessus. Pour la remplacer, restaurez une sauvegarde en ligne de commande (axgf-cms restore).
setup-not-archive = Ce n’est pas une archive .axgf.
setup-archive-too-large = Cette archive dépasse la taille d’un envoi.
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
admin-roles-note = Valider, dédoublonner, exporter, supprimer et gérer les comptes : réservé aux administrateurs.
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
admin-no-changes-yet = Aucune modification pour l’instant. Chaque enregistrement est consigné dans { $path }.
admin-last-validation = Dernière validation
admin-bundle-heavy = Cette archive pèse { $size }. L’ensemble est chargé au démarrage et gardé en mémoire : au-delà d’environ { $warn }, le site commence à coûter de la mémoire réelle et les redémarrages deviennent lents. Cela convient à une archive familiale, pas à une médiathèque — si les pièces jointes grossissent sans limite, conservez-les dans un espace de fichiers et faites-les pointer depuis l’archive.

admin-fields = Champs
admin-raw-json = JSON brut
form-more-fields = { $n ->
        [one] Un autre champ
        [many] { $n } autres champs
       *[other] { $n } autres champs
    }
admin-raw-json-help = L'entité entière, de sorte que rien ne soit non modifiable — les listes telles que les conjoints et les enfants d'une famille, ou l'histoire des frontières d'un lieu, se trouvent ici. C'est le document de départ ; les champs ci-dessus sont ensuite écrits par-dessus les chemins qui leur appartiennent : modifiez une valeur à un endroit ou à l'autre, pas aux deux. Cela doit être du JSON valide, sinon rien n'est enregistré.
admin-save = Enregistrer
admin-cancel = Annuler
place-editor-title = Modifier un lieu
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
person-tab-life = Événements
person-tab-media = Médias
person-tab-tree = Arbre
person-tab-history = Historique
person-tab-afg = AFG
person-tab-status = Statut
status-check = À vérifier
status-know = À savoir
status-fix = Corriger
status-open = Ouvrir
status-dismissed = Marqués « normal chez nous » : { $n }
status-panel = À vérifier : { $n }
person-afg-title = Demander à l’AFG à propos de { $name }
person-afg-title-short = Conversations avec l’AFG
person-afg-lede = Posez vos questions sur la vie de { $name }, sa famille, son époque. Vos conversations ne sont qu’à vous.
person-afg-reads = Lu à chaque message : la fiche, les notes ({ $notes }), les proches ({ $relatives }), la famille ({ $persons }) · environ { $tokens } tokens.
person-afg-ask = Votre question sur { $name }
person-afg-example = Par exemple : « Dans quel régiment se trouvait { $name } lors de la bataille dont parlent les notes ? »
person-afg-conversations = Vos conversations
person-afg-none = Aucune conversation pour l’instant : posez une première question.
person-afg-all = Toutes vos conversations
person-afg-back = Retour à vos conversations
person-afg-gone = Cette conversation n’existe pas, ou n’est pas la vôtre.
person-afg-delete-all = Supprimer toutes vos conversations sur { $name }
person-afg-delete-all-confirm = Je veux toutes les supprimer.
person-afg-deleted-all = Vos conversations sur cette personne sont supprimées.

admin-delete = Supprimer
admin-not-set = — non renseigné —
admin-edit = Modifier
list-select-row = Sélectionner { $name }
list-kind-label = Type
list-select-page = Tout sélectionner sur cette page
list-selected = { $n ->
        [one] { $n } sélectionnée
        [many] { $n } sélectionnées
       *[other] { $n } sélectionnées
    }
list-export = Exporter
list-escape = Échap pour désélectionner
bulk-confirm-title = { $n ->
        [one] Supprimer { $n } fiche ?
        [many] Supprimer { $n } fiches ?
       *[other] Supprimer { $n } fiches ?
    }
bulk-confirm-detail = Une fiche à laquelle une autre renvoie encore est refusée, et alors rien n’est supprimé.
bulk-confirm-go = Les supprimer
bulk-deleted = { $n ->
        [one] { $n } fiche supprimée.
        [many] { $n } fiches supprimées.
       *[other] { $n } fiches supprimées.
    }
bulk-refused = Rien n’a été supprimé : { $name } est encore cité ailleurs. Retirez d’abord ce renvoi, puis recommencez.
bulk-changed = { $n ->
        [one] { $n } fiche a changé depuis l’affichage de la liste, donc rien n’a été supprimé. Rechargez la liste et recommencez.
        [many] { $n } fiches ont changé depuis l’affichage de la liste, donc rien n’a été supprimé. Rechargez la liste et recommencez.
       *[other] { $n } fiches ont changé depuis l’affichage de la liste, donc rien n’a été supprimé. Rechargez la liste et recommencez.
    }
admin-range = { $from }–{ $to } sur { $total }
list-filiation = Filiation
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
accounts-lede = Enregistré dans { $path } (mode 600), à côté de l’archive, jamais dedans.
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
accounts-no-registration = Les visiteurs ne peuvent pas ouvrir de compte eux-mêmes ; l’inscription se règle plus haut.
accounts-password-hint = Vide : un mot de passe est généré et affiché une fois. Sinon, { $min } caractères au moins.
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
accounts-emergency-warning = Vous êtes connecté avec le jeton de secours, pas avec un compte : créez un vrai compte ci-dessous et utilisez-le.
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
tree-contradicts-detail = Ces liens sont écartés de la numérotation ; certaines rangées peuvent être mal placées. Corrigez la fiche fautive.
tree-contradicts-pair = Couple et parent-enfant à la fois :
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

home-empty = Rien d’enregistré pour l’instant.
home-count = { $total ->
        [one] Une fiche
       *[other] { $total } fiches
    }, dans un seul fichier qui appartient à la famille.
home-showcase-title = Là où cet arbre dit déjà plus que des noms et des dates
home-showcase-example = Voir un exemple →
home-nothing-title = Rien à montrer pour l'instant.
home-nothing-admin = Cette installation ne contient encore aucune famille : mettez-la en route à partir d’une archive, d’un fichier GEDCOM, ou de rien.
home-nothing-visitor = Cette archive de famille est en cours de préparation.
home-eyebrow = Archive familiale
home-hero-1 = Chaque vie est une source.
home-hero-2 = Chaque famille, une rivière.
view-tree-hero-1 = Chaque vie est une feuille.
view-tree-hero-2 = Chaque famille, un arbre.
view-galaxy-hero-1 = Chaque vie est une étoile.
view-galaxy-hero-2 = Chaque famille, une galaxie.
view-summit-hero-1 = Chaque vie est un pas.
view-summit-hero-2 = Chaque famille, une ascension.
home-cta-river = Explorer la rivière
view-tree-cta = Explorer l’arbre
view-galaxy-cta = Explorer la galaxie
view-summit-cta = Gravir la montagne
home-cta-tree = Voir l’arbre
home-scroll = Défiler pour explorer
home-trust-title = Confiance des filiations
home-trust-certain = pratiquement certaines
home-trust-high = bien étayées
home-trust-medium = plausibles
home-trust-low = hypothétiques
home-trust-unrated = pas encore évaluées
home-showcase-eyebrow = Au-delà des noms et des dates
home-cta-title = Ensemble, gardons la mémoire de la famille.
home-cta-detail = Chaque nom ajouté, chaque source vérifiée rend la rivière plus juste pour ceux qui viendront.
view-tree-cta-detail = Chaque nom ajouté, chaque source vérifiée rend l’arbre plus juste pour ceux qui viendront.
view-galaxy-cta-detail = Chaque nom ajouté, chaque source vérifiée rend la galaxie plus juste pour ceux qui viendront.
view-summit-cta-detail = Chaque nom ajouté, chaque source vérifiée rend la montagne plus juste pour ceux qui viendront.
home-cta-add = Ajouter une personne

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
record-living-yes = { $gender ->
        [F] vivante
       *[other] vivant
    }
record-deceased = { $gender ->
        [F] décédée
       *[other] décédé
    }
record-open-full-page = Ouvrir la page entière ↗
record-open-full-title = Ouvrir la page autonome et partageable
record-edit = Modifier
record-see-in-river = Voir dans la rivière
view-tree-see = Voir dans l’arbre
view-galaxy-see = Voir dans la galaxie
view-summit-see = Voir sur la montagne
panel-empty = Sélectionnez une fiche pour voir ici le dossier complet de cette personne.
person-visibility-inline = visibilité :
person-age-at-death = { $gender ->
        [F] { $n ->
            [one] morte à { $n } an
           *[other] morte à { $n } ans
        }
       *[other] { $n ->
            [one] mort à { $n } an
           *[other] mort à { $n } ans
        }
    }
person-age-now = { $n ->
        [one] { $n } an
       *[other] { $n } ans
    }
person-born-in = { $gender ->
        [F] née à { $place }
       *[other] né à { $place }
    }
person-died-in = { $gender ->
        [F] morte à { $place }
       *[other] mort à { $place }
    }
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
record-upload-help-short = Jusqu'à { $mb } Mo. Le SVG est refusé.
record-verbatim-note = Conservé exactement tel que le dossier l'énonçait, parce qu'aucun convertisseur n'a su l'interpréter.
record-file-to-attach = Fichier à joindre
record-document-type = Type de document
record-caption = Légende
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }
record-occupations-help-undated = Un métier s’enregistre avec un début et une fin, ce qui permet d’en comparer plusieurs sur un même axe. Cette archive contient les intitulés mais aucune date — c’est courant après une importation, la plupart des fichiers familiaux n’ayant nulle part où les mettre — donc il n’y a pas encore d’échelle à dessiner.
record-occupations-help-axis = Un métier est un état avec une durée, non un événement à date unique. Toutes les périodes partagent un même axe, { $from }-{ $to }.
record-birth-order = ordre de naissance
record-start-not-recorded = début non enregistré
record-end-not-recorded = fin non enregistrée
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
completeness-dates-note = Les dates sont gardées telles que données, au jour ou à la décennie ; un texte qui n’est pas une date est gardé mot pour mot.
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
convert-lede = Reprenez un arbre depuis un fichier GEDCOM. Rien n’est stocké, et l’arbre du site ne change pas.
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
tree-level-self = Même génération
tree-level-up = { $n ->
        [1] Enfants
        [2] Petits-enfants
        [3] Arrière-petits-enfants
       *[other] Descendants, { $n }ᵉ génération
    }
tree-level-down = { $n ->
        [1] Parents
        [2] Grands-parents
        [3] Arrière-grands-parents
       *[other] Ancêtres, { $n }ᵉ génération
    }
tree-card-centre = Centrer l’arbre ici
tree-card-same = La même personne, dessinée une seconde fois
# The tree page's legend: what a line to a child says (2026-10-28).
tree-key-lines = Liens :
tree-key-son = Fils
tree-key-daughter = Fille
tree-key-adoptive = Adoption
tree-key-foster = Accueil ou tutelle
tree-key-step = Par alliance
tree-key-union = Union
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

note-links = Relations hors de la famille : { $n }
note-occupations = Métiers datés : { $n }
note-birth-imprecise = Date de naissance imprécise dans la source
note-death-imprecise = Date de décès imprécise dans la source
note-names = Noms enregistrés : { $n }
note-transliteration = Un nom dans son écriture d’origine, avec sa translittération
note-witnessed = Événements comme témoin : { $n }

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
completeness-metric-confidence-many = { $with } faits sur { $slots } portent un degré : { $assessed } s’écartent de { $modal }, sur { $distinct } niveaux.

completeness-metric-parentage = À quel point chaque lien parent-enfant est sûr
completeness-metric-parentage-none = Aucune filiation ici ne dit à quel point elle est sûre. Adoptions, lignes contestées, reconstitutions à partir d'une seule mention : c'est précisément là qu'une famille a besoin d'enregistrer le doute — et l'arbre dessine un lien moins certain d'un trait plus pâle.
completeness-metric-parentage-some = Filiations avec leur propre degré : { $n }

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
completeness-metric-occupations-some = { $span } sur { $total } ont un début ou une fin.

completeness-metric-sources = Sources classées selon leur fiabilité
completeness-metric-sources-none = Aucune source enregistrée. Indiquer d'où vient un fait est ce qui permet à un proche de le vérifier plus tard — ou de le contester en disant pourquoi.
completeness-metric-sources-some = { $graded } sur { $total } indiquent leur solidité.
footer-credit = © 2026 · Conçu et développé par { $author }
footer-format-link = Le format AXGF sur GitHub
home-in-this-tree = Ce que la famille a enregistré jusqu’ici
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
admin-export-health-note = L’export simple écarte les catégories sensibles et le comportement des vivants. Cochez ce que ce fichier doit contenir.
avatar-picker-title = Choisir une image
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
ai-badge = IA
ai-portraits-title = Portraits retravaillés par IA
ai-portraits-band = Retouches
ai-portraits-upload = Déposer les images retravaillées via IA
ai-portraits-manage = Gérer les images IA
ai-portraits-intro = Des images qu'une IA a faites à partir des photographies de cette personne : ce à quoi elle pouvait ressembler. Elles apparaissent à côté du visage sur la fiche, marquées « IA », et seuls les membres connectés les voient. Elles ne sont jamais l'image de la personne, jamais parmi ses fichiers, jamais une preuve.
ai-portraits-current = Sur la fiche
ai-portraits-add = Ajouter des images — place pour { $n }
ai-portraits-files = Images (PNG, JPEG, WebP ou GIF), plusieurs à la fois
ai-portraits-generator = Réalisées avec
ai-portraits-from = D'après
ai-portraits-from-none = Non précisé
ai-portraits-remove = Retirer
ai-portraits-alt = Portrait de { $name } retravaillé par une IA
ai-portraits-not-evidence = Faites par une IA d'après les photographies : pas des images d'époque.
ai-portraits-full = La fiche a ses trois images IA. Retirez-en une pour en ajouter une autre.
ai-portraits-too-many = Une fiche porte trois images IA au plus. Retirez-en une avant d'en ajouter une autre.
ai-portraits-images-only = Un portrait IA doit être une image : PNG, JPEG, WebP ou GIF.
ai-portraits-stale = La fiche a changé pendant que cette page était ouverte. Rien n'a été enregistré : rouvrez-la et recommencez.
ai-portraits-back = Retour aux images IA

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
identity-edit-link = Modifier les noms et l'identité

## Union types, statuses and date precision, said out loud

union-type-marriage = mariage
union-type-civil_union = union civile
union-type-cohabitation = concubinage
union-type-religious_only = union religieuse
union-type-polygamous = polygame
union-type-unknown = non renseigné
union-role-spouse = conjoint
union-role-partner = partenaire
union-role-husband = mari
union-role-wife = épouse
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
events-edit-link = Modifier les événements

## Presentation styles: density, never colour

prefs-style = Densité
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
profile-group-filled = { $n ->
        [one] { $n } renseigné
        [many] { $n } renseignés
       *[other] { $n } renseignés
    }
profile-problems = Une partie de la saisie n’a pas pu être enregistrée. Chaque problème est indiqué à côté de son champ, et rien n’a été écrit.
profile-editor-withheld = Cette rubrique contient aussi, pour cette personne, des données de type { $classes } auxquelles vous n’avez pas accès. Elles ne sont pas affichées ici, et enregistrer ce formulaire les laisse telles quelles.
profile-living-class-note = Cette personne est enregistrée comme vivante. Ce que vous saisissez ici dans une catégorie sensible n’est visible que des administrateurs.
profile-relationships-elsewhere = Proches, parrains et témoins sont dans les familles, liens et événements : les modifier ici modifie aussi ces fiches.
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

## Interface

confidence-certain = Degré de certitude { $percent } % — pratiquement certain
confidence-high = Degré de certitude { $percent } % — bien étayé
confidence-medium = Degré de certitude { $percent } % — plausible mais non confirmé
confidence-low = Degré de certitude { $percent } % — hypothétique
tree-edge-union-between = { $from } et { $to } — { $confidence }
tree-edge-parentage-of = { $from }, parent de { $to } — { $confidence }
record-note-biography = Biographie
record-note-birth-date-as-recorded = Date de naissance, telle qu’enregistrée
record-note-death-date-as-recorded = Date de décès, telle qu’enregistrée
record-note-event-date-as-recorded = Date de « { $event } », telle qu’enregistrée
record-unknown-source = [Source inconnue]
record-untitled-source = [Source sans titre]
record-unnamed = [Sans nom]
record-untitled = [Sans intitulé]
record-period-from = à partir de { $date }
record-period-until = jusqu’en { $date }
record-dates-unrecorded = dates non enregistrées
record-link-unlabelled = lié à
record-link-reverse = { $label } (de)
record-place-worked-as = A travaillé comme { $title }
record-place-married-to = Mariage avec { $name }
record-place-married = Mariage
record-source-use-name = le nom « { $name } »
record-source-use-working-as = l’activité de { $title }
record-source-use-union-with = l’union avec { $name }
record-source-use-union = l’union
record-lifespan-born = n. { $year }
record-lifespan-died = déc. { $year }
size-bytes = { $n ->
        [one] { $n } octet
       *[other] { $n } octets
    }
size-kb = { $n } Ko
size-mb = { $n } Mo
size-gb = { $n } Go
calendar-gregorian = grégorien
calendar-julian = julien
calendar-hebrew = hébraïque
calendar-hijri = hégirien
calendar-persian = persan
calendar-chinese = chinois
calendar-ethiopian = éthiopien
calendar-japanese_era = ère japonaise
calendar-republican_french = républicain
calendar-roman = romain
diff-summary-none = aucun champ modifié
diff-summary-one = a modifié { $a }
diff-summary-two = a modifié { $a } et { $b }
diff-summary-many = a modifié { $a }, { $b } et { $n ->
        [one] un autre
       *[other] { $n } autres
    }
diff-saved-none = aucun champ modifié
diff-saved-one = { $a } modifié
diff-saved-two = { $a } et { $b } modifiés
diff-saved-many = { $a }, { $b } et { $n ->
        [one] un autre champ modifiés
       *[other] { $n } autres champs modifiés
    }
diff-section-other = un autre champ
diff-section-avatar = Photo de portrait
diff-section-partners = Partenaires
diff-section-ai = Assistance par IA
diff-section-birth = Naissance
diff-section-civil-status = État civil
diff-section-documents = Documents joints
diff-section-extensions = Données de l’application
diff-section-children = Enfants
diff-section-union = Union
diff-section-date = Date
diff-section-participants = Participants
diff-section-from = Lien depuis
diff-section-to = Lien vers
diff-section-valid-from = Valable à partir de
diff-section-valid-until = Valable jusqu’à
diff-section-visibility = Visibilité
diff-section-employer = Employeur
diff-section-title-normalized = Titre normalisé
diff-section-conflicts = Sources contradictoires
diff-section-dna = ADN
diff-section-document = Document
diff-section-language = Langue
diff-section-place = Lieu
diff-section-repository = Dépôt d’archives
diff-section-script = Écriture
diff-section-coordinates = Coordonnées
diff-section-country-history = Historique des pays
diff-section-identifiers = Identifiants
diff-section-names = Noms
diff-section-file = Fichier
diff-section-linked-to = Rattaché à
diff-section-ocr = Texte reconnu
diff-section-format-version = Version du format
history-created = a créé
history-deleted = a supprimé
history-merged = a fusionné dans une autre fiche
history-setup = a mis en route l’archive
history-attached = a joint un fichier
admin-raw-json-unparsed = Le JSON brut n’a pas pu être lu ({ $error }). Rien n’a été enregistré.
admin-raw-json-dangling = Le JSON brut fait pointer { $field } vers { $target }, qui ne figure pas dans cette archive. Choisissez-le avec un champ ci-dessus ou corrigez l’identifiant. Rien n’a été enregistré.
conflict-someone = Quelqu’un
conflict-unrecorded-time = un moment que personne n’a noté
dedup-merged-persons = { $n ->
        [one] une personne fusionnée
       *[other] { $n } personnes fusionnées
    }
dedup-merged-families = { $n ->
        [one] une famille fusionnée
       *[other] { $n } familles fusionnées
    }
dedup-manual-review = { $n ->
        [one] un cas laissé à l’examen d’une personne
       *[other] { $n } cas laissés à l’examen d’une personne
    }
dedup-nothing = Rien à signaler.
record-union-duplicate = Un seul couple, plusieurs fiches.
record-union-duplicate-detail = Le paquet conserve des fiches Famille distinctes pour ces deux personnes. C’est un défaut des données, non une seconde union.
dedup-pair-merged = La paire concernée ne forme plus qu’une fiche.
dedup-pair-refused = La paire concernée n’a pas été fusionnée : la bibliothèque a refusé et l’a laissée à l’examen d’une personne.

validate-errors = { $n ->
        [one] une erreur
       *[other] { $n } erreurs
    }
validate-warnings = { $n ->
        [one] un avertissement
       *[other] { $n } avertissements
    }
validate-notes = { $n ->
        [one] une remarque
       *[other] { $n } remarques
    }
validate-nothing = Rien à signaler.
list-separator = { ", " }
result-written = L’archive a été écrite sur le disque.
result-refused = La bibliothèque a refusé cette opération. L’archive sur le disque est inchangée.
convert-error-no-file = Aucun fichier n’a été envoyé. Choisissez d’abord un fichier .ged.
convert-error-file-too-large = Ce fichier pèse { $size } Mo et la limite est de { $limit } Mo. Rien n’a été converti.
convert-error-too-large = Cet envoi dépasse la limite de { $limit } Mo. Rien n’a été converti.
convert-error-unreadable = L’envoi n’a pas pu être lu ({ $error }). Rien n’a été converti.
convert-error-not-gedcom = Ce fichier ne ressemble pas à un fichier GEDCOM : un fichier GEDCOM 5.5.1 commence par une ligne « 0 HEAD ». Rien n’a été converti.
gedcom-read-title = Comment le fichier a été lu
gedcom-read-as = Lu comme du GEDCOM { $version }, en { $encoding }.
gedcom-read-as-unversioned = Lu comme du GEDCOM (son en-tête ne donne pas de version), en { $encoding }.
gedcom-read-declared = Son en-tête annonce { $declared } ; ses octets disent autre chose, et ont été lus tels qu’ils sont.
gedcom-read-gedzip = Une archive GEDCOM 7 (.gdz) : son jeu de données a été lu. Autres fichiers qu’elle contient (photographies, scans), non repris : { $n }.
gedcom-read-crs = Lignes terminées à l’ancienne manière Macintosh, réparées : { $n }.
gedcom-read-voids = Pointeurs vers @VOID@ — le « aucun enregistrement » de GEDCOM 7 — écartés plutôt que lus comme des personnes : { $n }.
gedcom-read-notes = Notes partagées (SNOTE) lues comme des notes : { $n }.
gedcom-read-stray = Octets qui n’étaient pas de l’UTF-8 dans ce fichier UTF-8, lus un à un en Windows-1252 : { $n }.
gedcom-read-tidied = Lignes disposées d’une façon que le convertisseur ne lit pas (indentées, tabulations ou espaces en trop entre les champs, balise en minuscules, espace avant une jointure CONC, caractère de contrôle, marque de calendrier grégorien), réparées : { $n }.
gedcom-read-too-deep = Lignes imbriquées plus profondément qu’aucune structure GEDCOM, mises de côté avec ce qu’elles contenaient : { $n }.
convert-error-no-dataset = Ce zip ne contient ni gedcom.ged, ni un unique fichier .ged à lire à sa place.
convert-error-refused = Le convertisseur a refusé ce fichier. Les diagnostics ci-dessous disent pourquoi. Rien n’a été gardé.
convert-error-packaging = Le fichier a été converti mais n’a pas pu être empaqueté ({ $error }).
completeness-fraction = { $part } sur { $whole }
event-category-adoption = Adoption
event-category-migration = Migration
event-category-naturalization = Naturalisation
event-category-incarceration = Emprisonnement
event-category-name_change = Changement de nom
event-category-legal = Affaire judiciaire
event-category-religious = Événement religieux
event-category-social = Événement social
event-category-historical = Événement historique
precision-quarter_century = au quart de siècle près
source-type-birth_certificate = acte de naissance
source-type-death_certificate = acte de décès
source-type-marriage_certificate = acte de mariage
source-type-census = recensement
source-type-baptism_record = acte de baptême
source-type-burial_record = acte de sépulture
source-type-will = testament
source-type-land_record = acte foncier
source-type-military_record = registre militaire
source-type-immigration_record = registre d’immigration
source-type-naturalization = dossier de naturalisation
source-type-passport = passeport
source-type-photograph = photographie
source-type-letter = lettre
source-type-diary = journal intime
source-type-newspaper = journal
source-type-oral_tradition = tradition orale
source-type-dna = test ADN
source-type-family_bible = bible familiale
source-type-gravestone = pierre tombale
source-type-published_genealogy = généalogie publiée
source-type-other = autre source
source-status-verified = vérifiée sur l’original
source-status-unverified = pas encore vérifiée
source-status-lost = perdue
source-status-known_missing = connue pour manquer
document-type-birth_certificate = acte de naissance
document-type-death_certificate = acte de décès
document-type-marriage_certificate = acte de mariage
document-type-census_page = page de recensement
document-type-baptism_record = acte de baptême
document-type-military_record = registre militaire
document-type-will = testament
document-type-land_record = acte foncier
document-type-diary = journal intime
document-type-newspaper_clipping = coupure de presse
document-type-gravestone_photo = photographie de pierre tombale
document-type-family_tree_drawing = arbre généalogique dessiné
document-type-audio = enregistrement sonore
document-type-video = enregistrement vidéo
document-status-present = conservé ici
document-status-referenced = cité, conservé ailleurs
document-status-known_missing = connu pour manquer
document-status-lost = perdu
document-status-unknown = emplacement inconnu
diag-unsupported_spec_version = L’archive déclare une version d’AXGF que cette version du logiciel ne sait pas lire.
diag-invalid_json = Un contenu qui devrait être du JSON ne peut pas être lu.
diag-invalid_bundle_structure = L’archive n’est pas organisée comme AXGF l’exige.
diag-schema_validation_failed = Un enregistrement ne correspond pas au schéma AXGF.
diag-dangling_reference = Un enregistrement renvoie à un autre qui ne figure pas dans l’archive.
diag-duplicate_entity_id = Deux enregistrements partagent le même identifiant.
diag-duplicate_unique_ref = Deux enregistrements revendiquent la même référence, qui devrait être unique.
diag-cycle_detected = Les liens familiaux tournent en rond : quelqu’un serait son propre ancêtre.
diag-chronology_conflict = Des dates se contredisent, par exemple un enfant né avant son parent.
diag-out_of_vocabulary = Une valeur ne fait pas partie des termes que sa liste autorise.
diag-claim_inconsistent = Une affirmation se contredit ou contredit une autre affirmation sur le même point.
diag-spec_version_mismatch = La version d’AXGF déclarée par un enregistrement ne correspond pas à son contenu.
diag-unknown_attribute = Un enregistrement porte un attribut qu’AXGF ne définit pas.
diag-entity_not_found = L’enregistrement à modifier ne figure pas dans l’archive.
diag-entity_already_exists = Un enregistrement portant cet identifiant existe déjà.
diag-unknown_entity_kind = Ce n’est pas un type d’enregistrement qu’AXGF connaît.
diag-delete_blocked_by_reference = L’enregistrement ne peut pas être supprimé tant que d’autres y renvoient.
diag-manual_review_required = Une personne doit examiner ce point ; il n’a pas été modifié automatiquement.
diag-zip_read_error = Le fichier d’archive n’a pas pu être lu.
diag-zip_write_error = Le fichier d’archive n’a pas pu être écrit.
diag-payloads_external = Les fichiers joints sont conservés hors des données de l’archive.
diag-payload_source_failed = Un fichier joint n’a pas pu être lu.
diag-payload_sink_failed = Un fichier joint n’a pas pu être écrit.
diag-gedcom_parse_error = Une ligne du fichier GEDCOM n’a pas pu être comprise.
diag-gedcom_unrecognized_tag = Le fichier GEDCOM utilise une balise que l’import ne connaît pas ; cette entrée n’a pas été reprise.
diag-internal = Une erreur s’est produite à l’intérieur de la bibliothèque.
diag-merge_refused = Les deux fiches ne peuvent pas être fusionnées en l’état.
diag-merge_unresolved = Un désaccord entre les deux fiches n’a pas été tranché.
diag-merge_unsupported = Les fiches de ce type ne peuvent pas être fusionnées.
diag-stale_plan = L’une des fiches a changé depuis le plan : refaites-le.
field-person-display-name = Nom affiché
field-person-display-name-hint = Le nom montré partout sur le site.
field-person-gender = Genre
field-person-living = En vie
field-person-birth-date = Date de naissance
field-date-value-hint = 1923, 1923-04 ou 1923-04-12 ; vide si inconnue.
field-person-birth-precision = Précision de la naissance
field-precision-hint = Avec quelle précision la source l’établit.
field-person-birth-circa = Naissance approximative
field-circa-hint = Affiché « vers 1923 » plutôt que comme une affirmation exacte.
field-person-birth-place = Identifiant du lieu de naissance
field-person-birth-confidence = Certitude de la naissance
field-person-confidence-hint = À quel point vous en êtes sûr. C’est ce que le site dessine.
field-person-death-date = Date de décès
field-person-death-precision = Précision du décès
field-person-death-circa = Décès approximatif
field-person-death-place = Identifiant du lieu de décès
field-person-death-confidence = Certitude du décès
field-person-death-cause = Cause du décès
field-person-bio = Biographie
field-notes = Notes
field-family-name = Nom de la famille
field-description = Description
field-family-union-type = Type d’union
field-family-union-status = État de l’union
field-family-union-confidence = Certitude de l’union
field-family-union-confidence-hint = Règle l’intensité du trait entre les partenaires dans l’arbre.
field-family-union-start = Début de l’union
field-family-union-end = Fin de l’union
field-family-notes-hint = Les partenaires et les enfants sont des listes : modifiez-les dans le JSON brut ci-dessous, ou sur la page des relations de la personne.
field-category = Catégorie
field-required-hint = Obligatoire.
field-event-subcategory = Sous-catégorie
field-date = Date
field-event-date-hint = Exigée par le schéma.
field-precision = Précision
field-circa = Approximative
field-place-id = Identifiant du lieu
field-confidence = Degré de certitude
field-source-id = Identifiant de la source
field-link-from-type = Type au départ
field-link-from-id = Identifiant au départ
field-link-to-type = Type à l’arrivée
field-link-to-id = Identifiant à l’arrivée
field-link-label = Intitulé
field-link-label-hint = Se lit dans le sens du lien : « parrain », « employeur », « témoin ». Obligatoire.
field-link-label-reverse = Intitulé inverse
field-link-label-reverse-hint = Comment il se lit depuis l’autre bout : « filleul », « employé ».
field-link-bidirectional = Se lit pareil dans les deux sens
field-valid-from = Valable à partir de
field-link-valid-from-hint = Quand la relation a commencé.
field-valid-until = Valable jusqu’à
field-link-confidence-hint = « Sûr à 85 %, d’après une lettre de famille » — ce que GEDCOM ne sait pas dire.
field-note = Note
field-occupation-person-id = Identifiant de la personne
field-occupation-title = Activité
field-occupation-title-hint = Obligatoire, par exemple Instituteur.
field-occupation-title-latin = Activité (écriture latine)
field-occupation-employer = Employeur
field-occupation-from = Depuis
field-occupation-from-hint = Une activité est une période. Donner ses deux bornes permet de la dessiner comme une barre.
field-occupation-until = Jusqu’à
field-source-title = Titre
field-source-type = Type de source
field-source-reliability = Fiabilité
field-source-reliability-hint = Obligatoire. Affichée en pastille à côté de chaque fait qui repose sur cette source.
field-source-status = État de la source
field-source-repository = Lieu de conservation
field-source-repository-reference = Cote dans le lieu de conservation
field-source-transcription = Transcription
field-place-name = Nom principal
field-place-name-lang = Langue du nom
field-place-name-lang-hint = Une étiquette de langue, par exemple en, fr ou pl.
field-place-type = Type de lieu
field-place-region = Région
field-place-country-current = Pays actuel
field-place-country-current-hint = L’histoire de ses frontières est une liste : modifiez-la dans le JSON brut ci-dessous.
field-document-filename = Nom du fichier
field-document-mime-type = Type de média
field-document-mime-type-hint = Obligatoire, par exemple image/jpeg.
field-document-type = Type de document
field-document-status = État du fichier
field-document-url = Adresse web
field-document-caption = Légende
lang-zh-Hans = Chinois simplifié
family-lineage = Filiation
links-relation = Nature du lien
occupations-position = Poste occupé
field-link-relation = Nature du lien
field-link-relation-hint = L’un des liens qu’AXGF 1.1 nomme. L’intitulé ci-dessus garde les mots de l’acte.
field-occupation-position = Poste occupé
field-occupation-position-hint = Le poste au sein de l’activité : Directrice, quand l’activité est Institutrice.
error-delete-changed-title = Modifié depuis que vous l’avez consulté
error-delete-changed-detail = Cet enregistrement a été enregistré de nouveau après l’affichage de la page d’où vous l’avez supprimé ; il est maintenant en version { $version }. Rien n’a été supprimé. Regardez-le tel qu’il est avant de décider à nouveau.
error-delete-changed-look = Le regarder de nouveau
documents-files = Fichiers joints ici
documents-files-help = Modifiez les détails d’un fichier, ou supprimez le fichier lui-même. La suppression retire le document et ses octets de l’archive, pour toutes les personnes auxquelles il est joint ; pour le retirer de cette seule personne, videz sa ligne ci-dessus.
documents-edit-details = Modifier les détails
documents-delete = Supprimer ce fichier

## Charts

radar-section = Graphiques tirés de la fiche
radar-section-help = Trois lectures de ce que contient cette fiche, chaque axe de 0 à 100. Chaque nombre est calculé à partir des faits indiqués à côté, selon des règles écrites dans la documentation de l’application ; rien n’est enregistré, rien n’est deviné, et un axe sans rien à lire reste vide au lieu de recevoir une note moyenne.
radar-physique = Physique
radar-mind = Tempérament et esprit
radar-vitality = Santé et vitalité
radar-axis-stature = Taille
radar-axis-build = Corpulence
radar-axis-lean-mass = Masse maigre
radar-axis-posture = Posture
radar-axis-gait = Démarche
radar-axis-dentition = Dents
radar-axis-openness = Ouverture
radar-axis-conscientiousness = Conscienciosité
radar-axis-extraversion = Extraversion
radar-axis-agreeableness = Agréabilité
radar-axis-stability = Stabilité émotionnelle
radar-axis-cognition = Cognition
radar-axis-circulation = Circulation
radar-axis-breathing = Respiration
radar-axis-metabolism = Métabolisme
radar-axis-illness = Absence de maladie
radar-axis-senses = Sens
radar-axis-rest = Sommeil et humeur
radar-folded-open = Afficher ce graphique
radar-folded-why = Cette personne est enregistrée comme vivante. Le portrait du tempérament d’une personne vivante reste replié jusqu’à ce que quelqu’un qui peut le lire demande à le voir.
radar-empty = Rien dans cette fiche ne permet encore de remplir ce graphique.
radar-table-caption = { $chart } : chaque axe, sa note et ce dont elle est tirée
radar-col-axis = Axe
radar-col-score = Note
radar-col-from = Tiré de
radar-no-score = pas de note
radar-from-none = rien
record-link-outgoing = de cette personne
record-link-incoming = vers cette personne
# L'avertissement d'exploitation sur le tableau de bord.
health-attention = Demande votre attention :
health-standing-token = Un jeton d’urgence est encore défini et passe outre chaque compte. Retirez-le du fichier d’environnement du service, puis redémarrez.
health-bundle-invalid = { $errors ->
        [one] Les données familiales ne passent plus la validation : une erreur, indiquée sous « Valider » ci-dessous.
        [many] Les données familiales ne passent plus la validation : { $errors } erreurs, indiquées sous « Valider » ci-dessous.
       *[other] Les données familiales ne passent plus la validation : { $errors } erreurs, indiquées sous « Valider » ci-dessous.
    }
health-disk-unknown = L'espace libre de cette machine n'a pas pu être lu ; rien ici ne peut donc promettre que le prochain enregistrement tiendra.
health-disk-no-room-to-save = Enregistrer est impossible : il reste { $free } libres, et reconstruire ce lot en demande { $need }. Rien n'est perdu, et rien ne peut être modifié avant qu'il y ait de la place.
health-disk-critical = Le disque est libre à { $percent } % — { $free } restants. L'enregistrement cessera bientôt de fonctionner.
health-disk-low = Le disque est libre à { $percent } % — { $free } restants. À traiter avant que cela devienne urgent.
health-backup-unconfigured = Rien n'est sauvegardé. Un seul disque défaillant emporterait tous les enregistrements.
health-backup-never = Un répertoire de sauvegarde est défini, mais aucune sauvegarde n'y a jamais été écrite.
health-backup-stale = { $days ->
        [one] La sauvegarde la plus récente date d'un jour. Il faudrait en écrire une chaque jour.
        [many] La sauvegarde la plus récente date de { $days } jours. Il faudrait en écrire une chaque jour.
       *[other] La sauvegarde la plus récente date de { $days } jours. Il faudrait en écrire une chaque jour.
    }
health-cache-missing = { $missing ->
        [one] Un fichier joint sur { $declared } manque dans le cache et ne pourra être téléchargé qu'après sa reconstruction au prochain enregistrement.
        [many] { $missing } fichiers joints sur { $declared } manquent dans le cache et ne pourront être téléchargés qu'après leur reconstruction au prochain enregistrement.
       *[other] { $missing } fichiers joints sur { $declared } manquent dans le cache et ne pourront être téléchargés qu'après leur reconstruction au prochain enregistrement.
    }

## River

nav-river = Rivière
nav-home = Famille
river-aria = La rivière autour de { $name } : les ancêtres en bas, les descendants en haut.
river-band-living = VIVANTS
river-circa = v.
river-trail = Itinéraire
river-back = ← Retour
river-range = Portée
river-motion = Mouvement
river-travel = voyage
river-cut = coupe
river-legend-attested = attesté
river-legend-inferred = déduit
river-legend-speculative = supposé
river-legend-lost = fin du relevé
river-legend-repeat = ×n atteint par n lignes, dessiné sous chacune
river-legend-again = ↔ redessiné auprès d’un partenaire suivant
river-legend-discharge = débit
river-legend-direction = ↓ vers la source · ↑ vers les vivants
view-tree-direction = ↓ les racines : les ancêtres · ↑ la couronne : les descendants
view-galaxy-direction = ↓ les ancêtres sous le soleil · ↑ les descendants au-dessus
view-summit-direction = ↓ les ancêtres dans la vallée · ↑ les descendants vers le sommet
river-legend-generation = une génération ≈ 29 ans
river-zoom = Zoom
river-still = Figer le paysage
view-tree-still = Figer l’arbre
view-galaxy-still = Figer le ciel
view-summit-still = Figer la montagne
river-whole = Toute la famille
river-names-all = Tous les noms
# A tap on a person in a view, on a touch screen: the drawing moves onto them.
river-pick-centre = Centrer ici
river-zoom-hint = Zoom : Ctrl + molette, pincer ou + / − · zoomé, glisser pour se déplacer
river-tab-sources = Sources
river-legend-title = Lire la rivière
view-tree-name = Arbre vivant
view-tree-legend = Lire l’arbre
view-galaxy-name = Galaxie
view-galaxy-legend = Lire la galaxie
view-summit-name = Ascension
view-summit-legend = Lire la montagne
hud-ancestors = Ancêtres
hud-descendants = Descendants
hud-generations = Générations
hud-living = Vivants
hud-attested = Liens attestés

# Masthead search
search-label = Rechercher une personne
search-short = Personne
date-qualifier = Qualificatif
date-qualifier-circa = vers
search-title = Recherche
search-heading = Personnes correspondant à « { $q } »
search-none = Personne de visible ne porte ce nom.
search-open-record = Ouvrir la fiche
search-hint = Entrée fait voyager la rivière jusqu’à elle ; Maj+Entrée ouvre sa fiche.
view-tree-search-hint = Entrée fait pousser l’arbre autour d’elle ; Maj+Entrée ouvre sa fiche.
view-galaxy-search-hint = Entrée en fait le soleil ; Maj+Entrée ouvre sa fiche.
view-summit-search-hint = Entrée vous y conduit ; Maj+Entrée ouvre sa fiche.
search-summary = { $n ->
        [one] { $n } personne
        [many] { $n } personnes
       *[other] { $n } personnes
    } · ↵ voyager · ⇧↵ ouvrir la fiche
search-empty-query = Saisissez un nom pour rechercher.

# Data quality (/quality)
nav-quality = Qualité des données
quality-title = Choses à regarder
quality-none = Rien à regarder : toutes les vérifications sont passées.
quality-intro = Des points à vérifier ; il y a souvent une bonne raison. Ouvrez la fiche, ou marquez le point comme vérifié.
quality-groups-label = Types de constats
quality-all = Tout
quality-show-label = Afficher
quality-show-open-option = À vérifier
quality-download = Télécharger
quality-group-names = Noms
quality-group-relations = Liens familiaux
quality-group-dates = Dates
quality-group-places = Lieux
quality-group-evidence = Preuves
quality-export-csv = Télécharger en CSV
quality-export-json = Télécharger en JSON
quality-open = Ouvrir
quality-none-file = Rien à regarder dans ce fichier.
quality-show-list = Afficher la liste
quality-after-import = Rien n’a été modifié dans le fichier. Une fois qu’il sera l’archive du site, la page Qualité des données listera les mêmes points avec un lien vers chaque fiche.
quality-file-list-cut = Seules les { $shown } premières sont listées ici. Une fois le fichier devenu l’archive du site, la page Qualité des données les liste toutes.
quality-rule-name-sentence = Une phrase dans un champ de nom
quality-name-sentence = « { $value } » ressemble à une phrase, pas à un nom.
quality-rule-union-parent-child = Un couple qui est aussi parent et enfant
quality-union-parent-child = { $parent } et { $child } : couple et parent-enfant à la fois.
quality-rule-child-listed-twice = Un enfant inscrit deux fois
quality-child-listed-twice = { $child } figure deux fois parmi les enfants.
quality-rule-duplicate-union = Le même couple enregistré deux fois
quality-duplicate-union = { $a } et { $b } : { $count } unions enregistrées.
quality-rule-partner-twice = Une même personne des deux côtés d’une union
quality-partner-twice = { $person } est des deux côtés de l’union.
quality-rule-foreign-surnames = Un fils dont le nom ne correspond à aucun parent
quality-foreign-surnames = Nom différent de ceux des parents : { $who }.
quality-rule-death-before-birth = Décès avant la naissance
quality-death-before-birth = { $person } : décès ({ $death }) avant la naissance ({ $birth }).
quality-rule-child-before-parent = Un enfant né avant un parent
quality-child-before-parent = Naissance de { $child } ({ $childborn }) avant celle de son parent { $parent } ({ $parentborn }).
quality-rule-parent-too-young = Un parent de moins de douze ans
quality-parent-too-young = { $parent } avait { $age } ans à la naissance de { $child }.
quality-rule-parent-too-old = Un parent de plus de soixante-cinq ans
quality-parent-too-old = { $parent } avait { $age } ans à la naissance de { $child }.
quality-rule-marriage-before-birth = Un mariage avant la naissance d’un conjoint
quality-marriage-before-birth = Mariage ({ $married }) avant la naissance de { $person } ({ $born }).
quality-rule-isolated-record = Une fiche sans date ni lien
quality-isolated-record = { $person } : ni date ni lien.
quality-rule-place-no-region = Un lieu sans région
quality-place-no-region = { $place } : pas de région.
quality-rule-region-free-text = Une région qui ressemble à une note
quality-region-free-text = Région de { $place } : « { $value } » ressemble à une note.
quality-rule-place-near-duplicate = Des lieux qui sont peut-être le même
quality-place-near-duplicate = Peut-être un seul lieu : { $names }.
quality-rule-not-a-place = Une entrée qui n’est peut-être pas un lieu
quality-not-a-place = « { $place } » n’est peut-être pas un lieu.
quality-rule-certain-without-source = Certain, sans source
quality-certain-without-source = { $record } : certain ({ $claims }) sans source.
quality-dismiss = Vérifié — normal chez nous
quality-restore = Le réafficher
quality-note-label = Pourquoi (facultatif)
quality-show-dismissed = Vérifiés et conservés
quality-show-open = Retour à la liste ouverte
quality-dismissed-by = Vérifié par { $by } le { $at }
quality-gone-title = Ce constat n’existe plus
quality-gone-detail = La fiche a changé depuis l’ouverture de la liste, et le constat n’est plus le même. Rechargez la liste et regardez à nouveau.
quality-summary = { $n ->
    [one] { $n } chose à regarder
   *[other] { $n } choses à regarder
}
quality-summary-file = { $n ->
    [one] { $n } chose à regarder dans ce fichier
   *[other] { $n } choses à regarder dans ce fichier
}
quality-dismissed-summary = { $n ->
    [one] { $n } constat vérifié et conservé
   *[other] { $n } constats vérifiés et conservés
}
quality-dismiss-all = Tout marquer comme vérifié ({ $n })
quality-restore-all = Tout réafficher ({ $n })
quality-bulk-conflict = Fiches modifiées depuis l’ouverture de la liste : { $n }. Rien n’a été marqué ; rechargez la liste et recommencez.

## The record's editor (v0.4.2)

edit-tab-changed = modifié, pas encore enregistré
edit-remove = Retirer
edit-add-partner = Ajouter un conjoint
edit-add-child = Ajouter un enfant
edit-union-with = Union avec { $names }
edit-union-alone = Union sans conjoint enregistré
edit-child-of = Enfant de { $names }
edit-new-family = Créer une famille
edit-cite-title = Ce qui repose sur une source
edit-attach-to = La citer pour
edit-add-source = Ajouter une source
edit-error-source-title = Une nouvelle source doit avoir un titre. Rien n'a été enregistré.
edit-detach-file = Retirer de cette fiche
edit-no-sources = Aucun fait propre à cette personne ne cite encore de source.
edit-no-files = Aucun fichier n'est encore attaché à cette fiche.
edit-saved = { $n ->
        [one] Enregistré : une fiche modifiée.
        [many] Enregistré : { $n } fiches modifiées en une fois.
       *[other] Enregistré : { $n } fiches modifiées en une fois.
    }
edit-saved-none = Rien n'avait changé : rien n'a été enregistré.
edit-back-to-record = Retour à la fiche
edit-avatar = Photo de profil
edit-avatar-frame = Cadrer le visage
edit-avatar-upload = Utiliser comme photo de profil
edit-stale = Quelqu'un d'autre a modifié { $what } depuis l'ouverture de cette page. Rien n'a été enregistré ; tout ce que vous avez saisi est conservé.
edit-refused = Rien n'a été enregistré : { $reason }

# Administration console (v0.5.0, docs/adr/0011)
error-cross-site-title = Refusé : envoyé depuis un autre site
error-cross-site-detail = Ce formulaire vient d’une page d’un autre site, il n’a donc pas été pris en compte. Si vous l’avez envoyé depuis ce site, rechargez la page et renvoyez-le.
admin-nav-label = Rubriques de l’administration
admin-nav-overview = Vue d’ensemble
admin-nav-bundle = Archive AXGF
admin-nav-infrastructure = Infrastructure
admin-nav-accounts = Comptes
admin-nav-connectivity = Connectivité

# Administration console: the AXGF bundle (v0.5.0, docs/adr/0011)
bundle-title = Archive AXGF
bundle-lede = L’archive de la famille : { $path }. Sa conformité, ce qui semble faux, et comment en prendre une copie.
bundle-unset-title = Pas encore de famille.
bundle-unset-detail = Cette installation tient une archive vide : il n’y a encore rien à vérifier. Mettez-la en route à partir d’une archive, d’un fichier GEDCOM ou de rien.
bundle-kpi-conformity = Conformité
bundle-verdict-ok = Conforme
bundle-verdict-warn = À surveiller
bundle-verdict-fail = Non conforme
findings-tally = Au vert : { $ok } · Avertissements : { $warn } · Critiques : { $fail }
bundle-kpi-entities = Entités
bundle-kpi-entities-note = { $persons ->
    [one] 1 personne
    *[other] { $persons } personnes
    } · { $files ->
    [one] 1 fichier joint
    *[other] { $files } fichiers joints
    }
bundle-kpi-size = Taille sur le disque
bundle-kpi-members = { $n ->
    [one] 1 fichier dans l’archive
    *[other] { $n } fichiers dans l’archive
    }
bundle-kpi-format = Format
bundle-kpi-format-note = Cette version lit et écrit AXGF { $current }.
bundle-conformity = Conformité
bundle-conformity-intro = Neuf vérifications, faites sur l’archive telle qu’elle est maintenant ; aucune ne la modifie. Ouvrez une ligne pour voir quoi faire.
bundle-area-format = Format
bundle-area-integrity = Intégrité
bundle-area-privacy = Vie privée
bundle-area-content = Contenu
level-ok = Au vert
level-warn = Avertissement
level-fail = Critique
findings-what-to-do = Quoi faire
findings-go = Y aller →
bundle-anomalies = Anomalies
bundle-anomalies-intro = Ce que trouve la page Qualité des données, règle par règle : encore ouvert, et déjà marqué vérifié.
bundle-anomalies-rule = Règle
bundle-anomalies-open = Ouvertes
bundle-anomalies-checked = Vérifiées
bundle-anomalies-none = La page Qualité des données ne trouve rien à regarder.
bundle-diagnostics = Diagnostics du validateur
bundle-facts = Caractéristiques
bundle-fact-path = Fichier
bundle-fact-size = Taille
bundle-fact-spec = Format
bundle-fact-schema = Schéma joint
bundle-fact-none = aucun
bundle-fact-created = Créée
bundle-fact-updated = Dernière écriture
bundle-fact-generator = Écrite par
bundle-fact-gedcom = Convertie depuis
bundle-export = Exporter
bundle-export-raw = L’archive telle quelle (.axgf)
bundle-export-raw-go = Télécharger l’archive
bundle-export-sealed = Chiffrée pour une personne (.axgf.gpg)
bundle-export-sealed-intro = La même archive, sous forme de message OpenPGP que seul le détenteur de la clé privée peut ouvrir. Collez sa clé publique : le texte qui commence par « -----BEGIN PGP PUBLIC KEY BLOCK----- ».
bundle-export-sealed-go = Télécharger chiffrée
seal-recipient = Pour
seal-fingerprint = Empreinte
seal-algorithm = Algorithmes
seal-fips-yes = dans l’ensemble approuvé par la FIPS
seal-fips-no = hors de l’ensemble approuvé par la FIPS
seal-expires = Expiration de la clé
seal-never = jamais
seal-key-label = La clé publique du destinataire
seal-key-hint = Une clé RSA d’au moins 2048 bits, ou une clé à courbe elliptique. Vérifiez l’empreinte avec son détenteur avant d’envoyer quoi que ce soit.
seal-remember = Garder cette clé pour la prochaine fois
seal-check = Lire la clé seulement
seal-fips-note = L’archive est chiffrée en AES-256 vers la clé du destinataire. Les algorithmes approuvés par la FIPS sont RSA de 3072 bits ou plus et les courbes NIST P-256, P-384 et P-521 : demandez P-384. La bibliothèque utilisée ici n’est pas elle-même un module validé FIPS 140-3.
seal-decrypt-note = Le destinataire l’ouvre avec GnuPG :
seal-forget = Oublier la clé gardée
seal-key-kept = La clé { $fingerprint } est gardée pour le prochain export chiffré.
seal-error-no-key = Collez d’abord la clé publique du destinataire.
seal-error-not-a-key = Ce n’est pas une clé publique OpenPGP. Collez le bloc entier, de sa ligne BEGIN à sa ligne END, et jamais une clé privée.
seal-error-bad-signatures = Les signatures de cette clé ne se vérifient pas : elle a peut-être été modifiée. Demandez-en une copie neuve à son détenteur.
seal-error-expired = Cette clé, ou chacune de ses clés capables de chiffrer, a expiré. Demandez à son détenteur de la prolonger ou d’en envoyer une nouvelle.
seal-error-revoked = Son détenteur a révoqué cette clé. Demandez-lui la nouvelle.
seal-error-no-encryption-key = Cette clé sait signer mais pas chiffrer : elle n’a pas de sous-clé de chiffrement. Demandez-en une à son détenteur (gpg --quick-add-key … encr).
seal-error-too-weak = Cette clé RSA fait { $bits } bits ; un export en demande au moins { $min }.
seal-error-unsupported = L’algorithme de cette clé n’est pas utilisé par l’export : clés RSA ou à courbe elliptique seulement.
seal-error-io = La copie chiffrée n’a pas pu être faite : { $error }
date-time-utc = { $date } à { $time } UTC

# Administration console: the bundle's conformity (v0.5.0, docs/adr/0011)
bundle-check-format-title = Version du format
bundle-check-format-ok = AXGF { $version }, la version que celle-ci lit et écrit.
bundle-check-format-missing = Le manifeste ne nomme aucune version d’AXGF.
bundle-check-format-missing-fix = L’archive n’a pas été écrite par un outil AXGF, ou elle a été abîmée. Restaurez la dernière sauvegarde qui validait (axgf-cms restore).
bundle-check-format-older = Écrite en AXGF { $version } ; le prochain enregistrement l’écrira en AXGF { $current }.
bundle-check-format-older-fix = Rien à faire : la prochaine modification la réécrit. Gardez une copie de l’ancien fichier si un autre programme ne lit encore que la { $version }.
bundle-check-format-newer = Écrite en AXGF { $version }, plus récente que la { $current } de cette version : enregistrer pourrait perdre ce qu’elle ne comprend pas.
bundle-check-format-newer-fix = Mettez axgf-cms à jour vers une version qui lit AXGF { $version } avant toute modification (Infrastructure, Version).
bundle-check-manifest-title = Les comptes du manifeste
bundle-check-manifest-ok = Les comptes du manifeste correspondent au contenu de l’archive.
bundle-check-manifest-drift = Le manifeste déclare { $declared } là où l’archive en tient { $actual } ({ $collection }).
bundle-check-manifest-fix = Rien n’est perdu : les comptes sont réécrits au prochain enregistrement. Si cette archive vient d’ailleurs, vérifiez qu’elle a été copiée en entier.
bundle-check-validation-title = Validation
bundle-check-validation-ok = Le validateur de la bibliothèque ne trouve rien d’anormal.
bundle-check-validation-errors = { $errors ->
    [one] 1 erreur
    *[other] { $errors } erreurs
    } : l’archive ne valide pas.
bundle-check-validation-errors-fix = Chaque diagnostic ci-dessous nomme la fiche et le champ. Corrigez la fiche, ou restaurez la dernière sauvegarde qui validait.
bundle-check-validation-warnings = { $warnings ->
    [one] 1 avertissement
    *[other] { $warnings } avertissements
    }. Ils ne bloquent jamais un enregistrement ; chacun signale que quelque chose cloche peut-être.
bundle-check-validation-warnings-fix = Lisez-les ci-dessous. La plupart sont des renvois à des fiches absentes de l’archive, qu’un import GEDCOM laisse souvent.
bundle-check-payloads-title = Fichiers joints
bundle-check-payloads-none = Aucun fichier joint.
bundle-check-payloads-ok = { $declared ->
    [one] Le fichier est présent.
    *[other] Les { $declared } fichiers sont tous présents.
    }
bundle-check-payloads-missing = { $missing } fichiers joints sur { $declared } manquent dans le cache.
bundle-check-payloads-fix = Le prochain enregistrement reconstruit le cache à partir de l’archive. S’ils manquent dans l’archive elle-même, restaurez-les depuis une sauvegarde.
bundle-check-binding-title = Comptes et archive
bundle-check-binding-ok = Le fichier des comptes a été créé pour cette famille.
bundle-check-binding-mismatch = Le fichier des comptes a été créé pour { $expected } ; cette archive est { $found }.
bundle-check-binding-fix = Si l’archive a été restaurée ou renommée exprès, rien à faire. Sinon, les comptes d’une famille s’appliquent à l’arbre d’une autre : remettez le bon fichier .acl à côté.
bundle-check-living-title = Les vivants, en public
bundle-check-living-ok = Aucune personne enregistrée comme vivante n’est visible des visiteurs.
bundle-check-living-public = { $n ->
    [one] 1 personne enregistrée comme vivante est marquée
    *[other] { $n } personnes enregistrées comme vivantes sont marquées
    } publiques : tout visiteur lit leur fiche.
bundle-check-living-fix = Sauf s’ils l’ont accepté, réglez leur visibilité sur membres (Modifier, Identité).
bundle-check-special-title = Catégories particulières
bundle-check-special-none = Aucune fiche ne contient de données de santé, biométriques, génomiques, pénales ou comportementales.
bundle-check-special-some = { $n ->
    [one] 1 fiche contient
    *[other] { $n } fiches contiennent
    } des données de santé, biométriques, génomiques, pénales ou comportementales. Pour les vivants, elles restent aux administrateurs, quoi que dise la fiche.
bundle-check-quality-title = Qualité des données
bundle-check-quality-ok = Rien à regarder.
bundle-check-quality-open = { $n ->
    [one] 1 chose à regarder
    *[other] { $n } choses à regarder
    }, dans { $groups ->
    [one] 1 groupe
    *[other] { $groups } groupes
    }.
bundle-check-quality-fix = Ouvrez la page Qualité des données : chaque ligne nomme la fiche et ce qui semble faux. Corrigez-la, ou marquez-la vérifiée.
bundle-check-size-title = Taille
bundle-check-size-ok = { $size }, sous les { $warn } au-delà desquels cette page avertit.
bundle-check-size-heavy = { $size }, au-delà de { $warn }.
bundle-check-size-fix = L’archive est à la famille, libre de grandir. Surveillez le disque et les sauvegardes (Infrastructure, Volumes), et relevez --size-warn-mb si cette taille est attendue.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

# Administration console: the infrastructure's checks (v0.5.0, docs/adr/0011 §3)
infra-area-security = Sécurité
infra-area-access = Accès
infra-area-data = Données
infra-area-capacity = Capacité
infra-area-release = Version
infra-area-platform = Plateforme
infra-verdict-ok = En bonne santé
infra-verdict-warn = À surveiller
infra-verdict-fail = Critique
duration-days = { $n ->
    [one] { $n } jour
    [many] { $n } jours
    *[other] { $n } jours
    }
duration-hours = { $n ->
    [one] { $n } heure
    [many] { $n } heures
    *[other] { $n } heures
    }
duration-minutes = { $n ->
    [one] { $n } minute
    [many] { $n } minutes
    *[other] { $n } minutes
    }
infra-transport-title = Comment le site est joint
infra-transport-ok = Cette page est arrivée en HTTPS.
infra-transport-exposed = Cette page est arrivée en clair, et le serveur écoute sur { $bind }, au-delà de cette machine : mots de passe et cookies de session traversent le réseau en clair.
infra-transport-exposed-fix = Placez un proxy inverse HTTPS (Caddy, nginx, HAProxy) devant, et liez de nouveau le service à 127.0.0.1 : relancez bootstrap.sh avec --bind 127.0.0.1:8080, ou retirez le drop-in qui l’a élargi.
infra-transport-plain = En clair, depuis { $bind } : acceptable sur cette machine seulement.
infra-transport-plain-fix = Passez par l’adresse HTTPS du site. Si un proxy termine bien le TLS, faites-lui envoyer X-Forwarded-Proto: https pour que ce serveur le sache.
infra-forwarded-title = Adresses transmises
infra-forwarded-ok = Le serveur n’écoute que sur cette machine : l’adresse du client que transmet un proxy est fiable pour ralentir les échecs de connexion répétés.
infra-forwarded-exposed = Le serveur est joignable directement, et il fait confiance à l’en-tête X-Forwarded-For : un client qui pose lui-même cet en-tête échappe au ralentissement après des échecs de connexion.
infra-forwarded-fix = Liez le service à 127.0.0.1 derrière le proxy, ou filtrez le port pour que seul le proxy l’atteigne.
infra-cross-site-title = Envois depuis d’autres sites
infra-cross-site-ok = Aucun formulaire n’a été envoyé ici depuis un autre site au cours du dernier jour.
infra-cross-site-refused = { $n ->
    [one] Un formulaire envoyé depuis un autre site a été refusé au cours du dernier jour.
    [many] { $n } formulaires envoyés depuis d’autres sites ont été refusés au cours du dernier jour.
    *[other] { $n } formulaires envoyés depuis d’autres sites ont été refusés au cours du dernier jour.
    }
infra-cross-site-fix = Rien n’est passé. Si ces refus sont les vôtres — le même site joint par une seconde adresse —, réglez l’adresse publique dans Connectivité, ou n’utilisez que celle-là.
infra-sso-title = Services de connexion
infra-sso-none = Aucun service de connexion n’est activé ; les comptes se connectent avec un mot de passe.
infra-sso-ok = { $n ->
    [one] Un service de connexion est activé, et l’adresse publique est en HTTPS.
    [many] { $n } services de connexion sont activés, et l’adresse publique est en HTTPS.
    *[other] { $n } services de connexion sont activés, et l’adresse publique est en HTTPS.
    }
infra-sso-no-https = Un service de connexion est activé, mais l’adresse publique n’est pas en HTTPS : les fournisseurs refusent d’envoyer leur réponse à une adresse en clair, et la connexion échoue.
infra-sso-fix = Dans Connectivité, réglez l’adresse publique sur l’adresse https:// qu’utilisent les lecteurs.
infra-admins-title = Administrateurs
infra-admins-none = Aucun compte actif ne peut administrer ce site ; seul le jeton d’urgence le peut.
infra-admins-none-fix = Connectez-vous avec le jeton d’urgence, créez un administrateur, puis retirez le jeton.
infra-admins-one = Un seul administrateur. S’il est absent ou perd son mot de passe, personne ne peut gérer le site.
infra-admins-one-fix = Donnez le rôle d’administrateur à une seconde personne de confiance, dans Comptes.
infra-admins-ok = { $n ->
    [one] { $n } administrateur peut se connecter.
    [many] { $n } administrateurs peuvent se connecter.
    *[other] { $n } administrateurs peuvent se connecter.
    }
infra-token-title = Jeton d’urgence
infra-token-generated = Aucun jeton d’urgence n’est défini.
infra-token-only-way = Le jeton d’urgence est la seule porte d’entrée tant qu’aucun compte administrateur n’existe.
infra-token-standing = Un jeton d’urgence est toujours défini. Il ouvre la session par-dessus tout compte et toute permission.
infra-token-fix = Retirez AXGF_CMS_ADMIN_TOKEN du fichier d’environnement du service, puis redémarrez le service.
infra-registration-title = Inscription libre
infra-registration-closed = Les visiteurs ne peuvent pas créer de compte ; seul un administrateur le peut.
infra-registration-secret = Les visiteurs peuvent créer un compte membre s’ils connaissent le secret d’inscription.
infra-registration-open = N’importe quel visiteur peut créer un compte membre.
infra-registration-open-fix = Cela convient à un site voulu ouvert. Sinon, exigez le secret d’inscription, ou fermez l’inscription, dans Comptes.
infra-failures-title = Échecs de connexion
infra-failures-ok = { $n ->
    [one] { $n } échec de connexion au cours du dernier jour.
    [many] { $n } échecs de connexion au cours du dernier jour.
    *[other] { $n } échecs de connexion au cours du dernier jour.
    }
infra-failures-many = { $n ->
    [one] { $n } échec de connexion au cours du dernier jour : quelqu’un essaie peut-être de deviner des mots de passe.
    [many] { $n } échecs de connexion au cours du dernier jour : quelqu’un essaie peut-être de deviner des mots de passe.
    *[other] { $n } échecs de connexion au cours du dernier jour : quelqu’un essaie peut-être de deviner des mots de passe.
    }
infra-failures-fix = Chaque adresse est déjà ralentie après des échecs répétés. Si le compte continue de grimper, trouvez d’où viennent les tentatives dans le journal du proxy et bloquez-les là, et assurez-vous que chaque administrateur a un long mot de passe ou se connecte par un service de connexion.
infra-health-bundle-title = Données de la famille
infra-health-bundle-ok = L’archive s’ouvre et passe la validation.
infra-health-bundle-fix = Ouvrez la page Archive AXGF pour voir les erreurs et corrigez-les, ou restaurez la dernière sauvegarde qui validait (axgf-cms restore).
infra-health-cache-title = Cache des pièces jointes
infra-health-cache-ok = Chaque fichier joint est dans le cache.
infra-health-cache-fix = Enregistrez une fiche une fois : l’enregistrement reconstruit le cache. Les fichiers encore absents ensuite manquent dans l’archive elle-même.
infra-health-backup-title = Sauvegardes
infra-health-backup-ok = { $count ->
    [one] Une sauvegarde récente existe ; { $count } archive est conservée.
    [many] Une sauvegarde récente existe ; { $count } archives sont conservées.
    *[other] Une sauvegarde récente existe ; { $count } archives sont conservées.
    }
infra-health-backup-fix = Sans répertoire de sauvegarde, relancez bootstrap.sh avec --backup-dir. Avec un, vérifiez le minuteur (systemctl status axgf-cms-backup.timer) et son journal (journalctl -u axgf-cms-backup).
infra-disk-title = Espace disque
infra-disk-ok = { $free } libres sur { $total } ({ $percent } % libres).
infra-disk-fix = Libérez de la place sur la partition qui contient le répertoire des données — anciennes sauvegardes aussi conservées ailleurs, journaux, fichiers d’autres programmes —, ou déplacez les données sur un disque plus grand.
infra-growth-title = Croissance
infra-growth-ok = { $days ->
    [one] Croît d’environ { $rate } par mois ; de la place pour encore { $days } jour à ce rythme.
    [many] Croît d’environ { $rate } par mois ; de la place pour encore { $days } jours à ce rythme.
    *[other] Croît d’environ { $rate } par mois ; de la place pour encore { $days } jours à ce rythme.
    }
infra-growth-soon = { $days ->
    [one] Croît d’environ { $rate } par mois : à ce rythme, le disque est plein dans { $days } jour.
    [many] Croît d’environ { $rate } par mois : à ce rythme, le disque est plein dans { $days } jours.
    *[other] Croît d’environ { $rate } par mois : à ce rythme, le disque est plein dans { $days } jours.
    }
infra-growth-fix = Prévoyez la place dès maintenant : un disque plus grand, moins de sauvegardes sur le même disque, ou des sauvegardes déplacées sur un autre.
infra-growth-flat = Les sauvegardes n’ont pas grossi ; il n’y a rien à prévoir.
infra-growth-unknown = Pas encore assez de sauvegardes pour mesurer la croissance : il faut au moins un jour entre la plus ancienne et la plus récente.
infra-memory-title = Mémoire
infra-memory-ok = { $used } utilisés sur le plafond de { $max } du service ({ $percent } %).
infra-memory-high = { $used } utilisés sur le plafond de { $max } du service ({ $percent } %) : près du seuil où le système arrête le service.
infra-memory-fix = Relevez MemoryMax= dans un drop-in pour axgf-cms.service, ou trouvez ce qui retient la mémoire — un très gros import, une grosse pièce jointe — et redémarrez le service.
infra-memory-unlimited = { $used } utilisés, sans plafond fixé pour le service.
infra-version-title = Version
infra-version-ok = v{ $current } est la version publiée la plus récente.
infra-version-behind = v{ $current } tourne ; { $latest } est publiée.
infra-version-behind-fix = Lisez les notes de version, puis mettez à jour depuis la section Version ci-dessous. Une sauvegarde est faite d’abord, et la version précédente revient d’elle-même si la nouvelle ne démarre pas.
infra-version-unknown = On ne sait pas s’il existe une version plus récente : la liste des versions n’a pas été récupérée.
infra-version-unknown-fix = Utilisez « Rechercher de nouvelles versions » ci-dessous. Le bac à sable de production ne peut pas joindre GitHub lui-même ; une fois l’assistant de mise à jour installé, c’est lui qui récupère la liste chaque jour.
infra-version-stale = { $days ->
    [one] La liste des versions a été récupérée il y a { $days } jour.
    [many] La liste des versions a été récupérée il y a { $days } jours.
    *[other] La liste des versions a été récupérée il y a { $days } jours.
    }
infra-upgrades-title = Mises à jour
infra-upgrades-ok = L’assistant de mise à jour est installé : le bouton ci-dessous peut changer de version.
infra-upgrades-manual = L’assistant de mise à jour n’est pas installé : la version se change à la main sur le serveur.
infra-upgrades-fix = Lancez une fois le bootstrap.sh actuel en root : il installe l’assistant avec le service.
infra-sandbox-title = Bac à sable
infra-sandbox-ok = Le processus tourne avec no-new-privileges et un filtre d’appels système.
infra-sandbox-unknown = Ce système n’indique pas le bac à sable du processus.
infra-sandbox-off = Le processus tourne sans une partie de son bac à sable : no-new-privileges ou le filtre d’appels système est désactivé.
infra-sandbox-fix = Faites tourner le service depuis l’unité qu’écrit bootstrap.sh, sans drop-in qui l’assouplisse. Sur une machine de développement, c’est attendu.
infra-clock-title = Horloge
infra-clock-ok = L’horloge système indique { $year }, ce qui est plausible.
infra-clock-wrong = L’horloge système indique { $year } : dates, âges des sauvegardes et sessions sont tous faux.
infra-clock-fix = Activez la synchronisation de l’heure : timedatectl set-ntp true.

# Administration console: the Infrastructure page (v0.5.0, docs/adr/0011)
infra-title = Infrastructure
infra-lede = La machine de ce site : accès, clés, données, version et bac à sable, vérifiés à chaque ouverture de la page.
infra-kpi-health = Santé
infra-kpi-version = Version
infra-kpi-version-newer = { $version } disponible
infra-kpi-version-current = À jour
infra-kpi-version-unknown = Pas encore vérifiée
infra-kpi-disk = Espace libre
infra-kpi-disk-note = { $percent } % de { $total } utilisés
infra-kpi-compute = Machine
infra-kpi-cpus = { $n ->
    [one] { $n } cœur
    [many] { $n } cœurs
    *[other] { $n } cœurs
    }
infra-kpi-load = Charge { $load }
infra-kpi-rss = { $size } utilisés
infra-unknown = Inconnu
infra-health = Santé
infra-health-intro = Vert : tout va bien ; avertissement : à regarder ; critique : à traiter tout de suite.
infra-json = Les mêmes vérifications en JSON, pour une supervision
infra-version = Version
infra-version-running = En service
infra-version-latest = Dernière publiée
infra-version-up-to-date = à jour
infra-version-checked = Liste récupérée
infra-version-never = jamais
infra-source-direct = par ce serveur
infra-source-helper = par l’assistant de mise à jour
infra-version-token = Dépôt privé
infra-token-env-set = Un jeton GitHub est défini pour lister et télécharger les versions.
infra-token-env-unset = Aucun jeton. Si le dépôt est privé, placez-en un dans { $name }, lisible par root seul : l’assistant de mise à jour le lit là.
infra-releases-error = La dernière tentative de récupération de la liste a échoué :
infra-releases-refresh = Rechercher de nouvelles versions
infra-releases-asked = L’assistant de mise à jour a été prié de récupérer une liste fraîche. Rechargez cette page dans une minute.
infra-releases-not-asked = La demande n’a pas pu être écrite : { $error }
infra-upgrade-status-running = Passage de { $from } à { $to } en cours…
infra-upgrade-status-succeeded = Passée de { $from } à { $to }.
infra-upgrade-status-failed = Le passage de { $from } à { $to } a échoué ; { $from } est toujours la version en service.
infra-upgrade-status-refused = L’assistant de mise à jour a refusé le passage à { $to }.
infra-upgrade-log = Sortie de l’assistant
infra-upgrade-pending = { $by } a demandé { $version } le { $at } ; l’assistant de mise à jour ne l’a pas encore pris en charge.
infra-releases = Versions publiées
infra-releases-tag = Version
infra-releases-published = Publiée
infra-releases-relation = Par rapport à celle-ci
infra-releases-notes = Notes de version
infra-releases-open = Lire
infra-relation-newer = plus récente
infra-relation-current = celle-ci
infra-relation-older = plus ancienne
infra-prerelease = version candidate
infra-releases-none = Pas encore de liste des versions.
infra-upgrade = Changer de version
infra-upgrade-intro = L’assistant de mise à jour sauvegarde, installe la version vérifiée, redémarre et contrôle ; en cas d’échec, la version précédente revient.
infra-upgrade-version = Version à installer
infra-upgrade-confirm = Pour revenir à une version plus ancienne, retapez son étiquette
infra-upgrade-confirm-hint = Inutile pour une version plus récente. Une version plus ancienne peut ne pas lire des données écrites par une plus récente : lisez d’abord ses notes.
infra-upgrade-go = Installer cette version
infra-upgrade-safety = Le site est indisponible quelques secondes pendant le redémarrage du service.
infra-upgrade-manual = L’assistant de mise à jour n’est pas installé sur ce serveur : cette page ne peut pas changer de version. En root, sur le serveur :
infra-upgrade-manual-helper = Lancez une fois le bootstrap.sh actuel pour installer l’assistant.
infra-upgrade-asked = { $version } a été demandée. L’assistant de mise à jour la prend en charge en quelques secondes ; cette page indique comment cela s’est passé.
infra-upgrade-no-helper = L’assistant de mise à jour n’est pas installé : { $version } ne peut pas être installée d’ici.
infra-upgrade-unknown-version = { $version } n’est pas dans la liste des versions publiées.
infra-upgrade-same-version = { $version } est la version qui tourne déjà.
infra-upgrade-confirm-older = { $version } est plus ancienne que la version en service. Tapez son étiquette dans le champ de confirmation pour y revenir.
infra-volumes = Stockage
infra-volumes-intro = La partition qui contient le répertoire des données ({ $path }), et ce que ce site y conserve.
infra-volumes-partition = Partition
infra-of = { $used } sur { $total }
infra-volumes-ours = Ce site
infra-volumes-breakdown = Ce que ce site conserve
infra-part-bundle = Archive de la famille
infra-part-cache = Cache des pièces jointes
infra-part-backups = Sauvegardes
infra-part-binary = Programme
infra-part-journal = Journal des modifications
infra-part-accounts = Comptes et réglages
infra-files = { $n ->
    [one] { $n } fichier
    [many] { $n } fichiers
    *[other] { $n } fichiers
    }
infra-growth = Croissance
infra-growth-month = Par mois
infra-growth-not-enough = pas assez de sauvegardes pour le dire
infra-growth-left = Place restante à ce rythme
infra-backups = Sauvegardes
infra-backups-dir = Répertoire
infra-backups-count = Archives
infra-backups-newest = La plus récente
infra-backups-oldest = La plus ancienne
infra-backups-room = Place là où elles sont gardées
infra-free-of = { $free } libres sur { $total }
infra-paths = Chemins
infra-path-data = Répertoire des données
infra-path-binary = Programme
infra-compute = Calcul
infra-compute-hidden = Le bac à sable du service lui cache les chiffres de la machine elle-même ; ce qui s’affiche ici, c’est ce que le processus peut voir.
infra-compute-memory = Mémoire de la machine
infra-available-of = { $available } libres sur { $total }
infra-compute-ceiling = Mémoire du service
infra-compute-no-ceiling = { $used }, sans plafond
infra-compute-os = Système
infra-compute-kernel = Noyau
infra-compute-arch = Architecture
infra-compute-cpu = Processeur
infra-compute-cpus = Cœurs
infra-compute-load = Charge (1, 5, 15 min)
infra-compute-rss = Ce processus
infra-compute-threads = Fils d’exécution
infra-compute-files = Fichiers ouverts
infra-compute-uptime = Service en marche depuis
infra-compute-sys-uptime = Machine en marche depuis
infra-compute-sandbox = Bac à sable
infra-compute-nnp-on = aucun nouveau privilège
infra-compute-nnp-off = privilèges acquérables
infra-compute-seccomp-on = appels système filtrés
infra-compute-seccomp-off = appels système non filtrés
infra-activity = Activité
infra-activity-intro = Sur { $span } ; repart de zéro au redémarrage du service.
infra-activity-sessions = Sessions ouvertes
infra-activity-signed-in = Connexions
infra-activity-failed = Échecs de connexion
infra-activity-registered = Comptes créés
infra-activity-cross-site = Envois d’autres sites refusés

# Sign-in services: what the reader is told when one fails (v0.5.0, docs/adr/0012)
sso-error-not-configured = Ce service de connexion n’est pas configuré sur ce site.
sso-error-no-public-url = Se connecter par un service demande l’adresse publique de ce site, qui n’est pas encore définie. Un administrateur la règle dans Connectivité.
sso-error-unreachable = Le service de connexion n’a pas pu être joint. Réessayez dans un instant, ou connectez-vous autrement.
sso-error-refused = Le service de connexion ne vous a pas laissé entrer, ou la connexion y a été annulée.
sso-error-stale = Cette connexion avait expiré, ou a été commencée dans un autre navigateur. Recommencez.
sso-error-invalid = La réponse du service de connexion n’a pas passé les vérifications de ce site : elle n’a pas été utilisée. Recommencez ; si cela se reproduit, prévenez un administrateur.
infra-upgrade-already-pending = { $version } attend déjà l’assistant de mise à jour. Attendez la fin avant de demander une autre version.

# Administration console: Connectivity, sign-in services and registration (v0.5.0, docs/adr/0012)
connectivity-title = Connectivité
connectivity-lede = Comment les lecteurs joignent ce site et se connectent : adresse publique, services de connexion et règles.
connectivity-kpi-address = Adresse publique
connectivity-address-unset = Non définie
connectivity-address-https = HTTPS
connectivity-address-plain = Non chiffrée
connectivity-address-unset-note = Tout service de connexion en a besoin
connectivity-kpi-services = Services de connexion
connectivity-kpi-services-note = { $n ->
    [one] prêt à proposer
    [many] prêts à proposer
    *[other] prêts à proposer
    }
connectivity-kpi-passwords = Mots de passe
connectivity-policy-short-everyone = Tout le monde
connectivity-policy-short-non-admins = Sauf les administrateurs
connectivity-policy-short-nobody = Personne
connectivity-policy-note-everyone = Chaque compte peut se connecter avec son mot de passe, et aussi par un service lié.
connectivity-policy-note-non-admins = Les administrateurs ne se connectent que par un service ; les autres peuvent encore utiliser un mot de passe. Un mot de passe d’administrateur volé n’ouvre alors plus rien.
connectivity-policy-note-nobody = Tout le monde se connecte par un service ; un compte sans service lié ne peut pas se connecter.
connectivity-kpi-admins-linked = { $n ->
    [one] { $n } administrateur avec un service lié
    [many] { $n } administrateurs avec un service lié
    *[other] { $n } administrateurs avec un service lié
    }
connectivity-kpi-registration = Inscription
connectivity-kpi-registration-go = Se règle dans Comptes →
registration-short-closed = Administrateurs seulement
registration-short-secret = Avec le secret de la famille
registration-short-open = Ouverte aux visiteurs
connectivity-address = Adresse publique
connectivity-address-intro = L’adresse publique que tapent les lecteurs, comme https://family.example ; les services de connexion y renvoient.
connectivity-address-label = Adresse publique
connectivity-address-hint = Laissez-la vide tant que personne ne se connecte par un service.
connectivity-address-invalid = Ce n’est pas une adresse http ou https avec un nom d’hôte.
connectivity-address-saved = L’adresse publique est enregistrée.
connectivity-policy = Règles de connexion
connectivity-policy-intro = Elles valent pour tous les comptes ; une règle qui ne laisserait aucun administrateur se connecter est refusée.
connectivity-policy-passwords = Qui peut se connecter avec un mot de passe
connectivity-policy-without-way = { $n ->
    [one] { $n } compte actif n’a ni service lié ni adresse : avec « Personne », il ne pourrait plus se connecter.
    [many] { $n } comptes actifs n’ont ni service lié ni adresse : avec « Personne », ils ne pourraient plus se connecter.
    *[other] { $n } comptes actifs n’ont ni service lié ni adresse : avec « Personne », ils ne pourraient plus se connecter.
    }
connectivity-link-by-email = Lier un service au compte qui a la même adresse vérifiée
connectivity-link-by-email-note = Créez le compte d’un proche avec son adresse : sa première connexion par un service qui la vérifie le relie.
connectivity-domains = Domaines autorisés à ouvrir un compte
connectivity-domains-hint = Un par ligne (family.example). Vide : tous. Sinon, seules les adresses vérifiées de ces domaines peuvent ouvrir un compte.
connectivity-domain-invalid = « { $domain } » n’est pas un nom de domaine.
connectivity-policy-unknown = Choisissez qui peut se connecter avec un mot de passe.
connectivity-policy-lockout = Refusé : avec ce changement, aucun administrateur actif ne pourrait se connecter. Liez d’abord un service de connexion au compte d’un administrateur, en vous connectant une fois par lui, ou gardez les mots de passe pour les administrateurs.
connectivity-policy-saved = Les règles de connexion sont enregistrées.
connectivity-not-saved = Non enregistré : { $error }
connectivity-services = Services de connexion
connectivity-services-intro = Enregistrez ce site dans la console du service avec l’adresse de redirection indiquée, puis collez ici l’identifiant client et le secret.
connectivity-outbound-note = Se connecter par un service exige les connexions sortantes : sudo systemctl edit axgf-cms, ajoutez IPAddressAllow=any sous [Service], puis redémarrez.
connectivity-service-ready = Prêt
connectivity-service-incomplete = Incomplet
connectivity-service-off = Désactivé
connectivity-service-enable = Proposer ce service sur les pages de connexion et d’inscription
connectivity-service-issuer = Émetteur
connectivity-service-issuer-hint = L’adresse du fournisseur, par exemple https://auth.family.example/realms/family ; ses réglages sont lus dans /.well-known/openid-configuration sous cette adresse.
connectivity-service-label = Libellé du bouton
connectivity-service-tenant = Annuaire (tenant)
connectivity-service-tenant-hint = common pour tout compte Microsoft, organizations pour les comptes professionnels et scolaires, consumers pour les comptes personnels, ou l’identifiant de votre annuaire.
connectivity-service-client-id = Identifiant client
connectivity-service-secret = Secret client
connectivity-service-secret-set = Un secret est défini. Laissez ce champ vide pour le garder.
connectivity-service-secret-unset = Aucun secret n’est encore défini.
connectivity-service-secret-clear = Retirer le secret
connectivity-service-redirect = Adresse de redirection à enregistrer
connectivity-service-redirect-needs-address = Définissez d’abord l’adresse publique
connectivity-service-console = Où l’enregistrer
connectivity-service-console-open = Console de { $name }
connectivity-service-test = Tester la connexion
connectivity-issuer-invalid = L’émetteur doit être une adresse https://.
connectivity-tenant-invalid = Un annuaire est un nom ou un identifiant : lettres, chiffres, points et tirets.
connectivity-provider-incomplete = { $name } n’est pas complet : il faut un identifiant client et un secret client, et un émetteur pour un fournisseur générique.
connectivity-provider-saved = { $name } est enregistré.
connectivity-test-ok = { $name } a répondu : { $answer }.
connectivity-test-failed = { $name } n’a pas pu être joint depuis ce serveur : { $detail }
registration-title = Inscription
registration-intro = Si un visiteur peut ouvrir lui-même un compte membre ; un administrateur peut lui donner un autre rôle ci-dessous.
registration-who = Qui peut ouvrir un compte
registration-note-closed = Seul un administrateur crée des comptes, sur cette page.
registration-note-secret = Qui connaît le secret de la famille peut ouvrir un compte membre. Partagez-le comme un code de porte.
registration-note-open = N’importe quel visiteur peut ouvrir un compte membre. Les membres lisent les fiches réservées aux membres.
registration-rotate = Tirer un nouveau secret
registration-secret = Le secret de la famille
registration-secret-note = Quiconque l’a peut ouvrir un compte tant que ce mode est actif. En tirer un nouveau ferme la porte à l’ancien ; les comptes déjà ouverts restent.
registration-link = Page d’inscription
registration-quick = Accès rapides
registration-quick-none = Aucun service de connexion n’est prêt : configurez-en un dans Connectivité
registration-changed = Modifiée en dernier par { $by }, { $at }.
registration-unknown = Choisissez qui peut ouvrir un compte.
accounts-kpi-total = Comptes actifs
accounts-kpi-total-note = { $disabled ->
    [one] { $disabled } désactivé
    [many] { $disabled } désactivés
    *[other] { $disabled } désactivés
    }
accounts-kpi-admins = Administrateurs
accounts-kpi-admins-few = Ajoutez-en un deuxième
accounts-kpi-admins-ok = Plus d’un, comme il se doit
accounts-kpi-linked = Liés à un service
accounts-kpi-linked-note = { $n ->
    [one] { $n } service prêt
    [many] { $n } services prêts
    *[other] { $n } services prêts
    }
accounts-kpi-registered = { $n ->
    [one] { $n } ouvert par un visiteur
    [many] { $n } ouverts par des visiteurs
    *[other] { $n } ouverts par des visiteurs
    }
accounts-roles = Rôles
accounts-sign-in = Se connecte avec
accounts-sign-in-password = mot de passe
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = authentification unique
accounts-sign-in-none = aucun moyen d’entrer pour l’instant
accounts-created-via-register = ouvert par le visiteur
accounts-created-via-sso = ouvert par un service
accounts-add-also-register = Les visiteurs peuvent aussi ouvrir eux-mêmes des comptes membres, comme réglé plus haut. Un compte créé ici peut avoir n’importe quel rôle.
login-password-not-allowed = Les règles de connexion de ce site n’autorisent pas ce compte à utiliser un mot de passe : connectez-vous par un service.
login-register = Pas encore de compte ? Créez-en un
register-title = Créer un compte
register-lede = Un compte membre lit les fiches réservées aux membres de la famille. Le plus rapide est un compte que vous avez déjà.
register-closed = Ici, les comptes sont créés par un administrateur. Demandez-lui le vôtre.
register-have-account = Vous avez déjà un compte ? Connectez-vous
register-or = ou
register-secret = Le secret de la famille
register-secret-hint = Un administrateur vous l’a donné, ou quelqu’un de la famille.
register-secret-hint-services = Tapez-le avant de choisir un service, sinon l’étape suivante vous le demandera.
register-domains = Seules les adresses de { $domains } peuvent ouvrir un compte.
register-username-hint = De 2 à 64 lettres, chiffres, points, tirets ou tirets bas.
register-submit = Créer le compte
register-member-note = Aucune adresse n’est demandée : rien ne vous est envoyé, et personne ne peut s’approprier la vôtre.
register-form-closed = Ce site n’ouvre pas de compte avec ce formulaire.
register-throttled = Trop de tentatives depuis cet endroit. Attendez un quart d’heure et réessayez.
register-secret-wrong = Ce n’est pas le secret de la famille.
register-username-invalid = Un nom d’utilisateur compte de 2 à 64 lettres, chiffres, points, tirets ou tirets bas.
register-password-short = Le mot de passe doit compter au moins { $min } caractères.
register-username-taken = Ce nom d’utilisateur est pris. Choisissez-en un autre.
register-not-saved = Le compte n’a pas pu être enregistré. Réessayez dans un instant.
register-finish-title = Encore une étape
register-finish-lede = Le service a confirmé qui vous êtes. Pour ouvrir un compte ici, tapez le secret de la famille.
register-finish-lede-email = Le service vous a confirmé comme { $email }. Pour ouvrir un compte ici, tapez le secret de la famille.
sso-continue = Continuer
sso-continue-with = Continuer avec { $name }
sso-error-title = La connexion n’a pas abouti
sso-account-disabled = Le compte lié à cette connexion est désactivé. Adressez-vous à un administrateur.
sso-no-account = Aucun compte ici ne correspond à cette connexion, et ce site n’ouvre pas de compte de lui-même. Demandez à un administrateur de créer le vôtre avec cette adresse : votre prochaine connexion s’y liera.
sso-domain-refused = Ici, un compte ne peut être ouvert qu’avec une adresse des domaines de la famille, confirmée par le service.

# Administration console: the overview's tiles (v0.5.0, docs/adr/0011)
overview-accounts-note = { $admins ->
    [one] { $admins } administrateur
    [many] { $admins } administrateurs
    *[other] { $admins } administrateurs
    }
register-password-hint = Au moins { $min } caractères. Une phrase de plusieurs mots se retient mieux qu’un fouillis de signes.

# Clarity: the light theme and its funnel (v0.5.0, docs/adr/0013)
theme-clarity = Clarté
theme-clarity-note = clair, blanc et orange · un entonnoir
view-funnel-back = Retour à l’entonnoir
view-funnel-open = Ouvrir l’entonnoir
view-funnel-hero-1 = Chaque génération est un anneau.
view-funnel-hero-2 = Chaque famille, un entonnoir.
view-funnel-cta = Explorer l’entonnoir
view-funnel-cta-detail = Chaque nom ajouté et chaque source vérifiée rendent l’entonnoir plus juste pour ceux qui viendront après.
view-funnel-see = Voir dans l’entonnoir
view-funnel-direction = ↓ les plus anciens au bout étroit · ↑ les plus récents à l’embouchure
view-funnel-still = Figer l’entonnoir
view-funnel-name = Entonnoir
view-funnel-legend = Comment lire l’entonnoir
view-funnel-search-hint = Entrée les amène au premier plan ; Maj+Entrée ouvre leur fiche.
theme-genealogy = Généalogie
theme-genealogy-note = vert sombre et crème · un arbre à gravir
view-scroll-back = Retour à l’arbre généalogique
view-scroll-open = Ouvrir l’arbre généalogique
view-scroll-hero-1 = Chaque vie est un portrait.
view-scroll-hero-2 = Chaque famille, un arbre à gravir.
view-scroll-cta = Explorer l’arbre généalogique
view-scroll-cta-detail = Chaque nom ajouté, chaque source vérifiée rend l’arbre généalogique plus juste pour ceux qui viendront.
view-scroll-see = Voir dans l’arbre généalogique
view-scroll-direction = ↓ vers les racines : les ancêtres · ↑ vers la cime : les descendants
view-scroll-still = Figer les feuilles
view-scroll-name = Arbre généalogique
view-scroll-legend = Lire l’arbre généalogique
view-scroll-search-hint = Entrée la place sur le tronc ; Maj+Entrée ouvre sa fiche.
bundle-binding-unnamed = une archive sans nom
bundle-binding-created = une archive créée le { $date }
bundle-binding-named = « { $family } »
bundle-binding-named-created = « { $family } », créée le { $date }

# Notes and contributions (docs/adr/0014)
notes-section-title = Notes et contributions
notes-section-intro = Ce que les membres de la famille ont mis par écrit : souvenirs, recherches, lettres, documents. Cherchez-les, parcourez-les par lettre, ou ouvrez-en un pour le lire et l'enregistrer en PDF.
notes-search-label = Chercher dans les notes
notes-search-go = Chercher
notes-all = Toutes
notes-letters-label = Les notes par première lettre de leur titre
notes-shown = Affichées : { $shown } sur { $total }
notes-none = Aucune note n'a encore été écrite.
notes-none-found = Aucune note ne répond à cette recherche.
notes-write = Écrire une note
notes-family-wide = Toute la famille
notes-by = par { $author }
notes-about = Au sujet de : { $names }
notes-about-prefix = Au sujet de
notes-about-family = Au sujet de toute la famille
notes-pdf = Télécharger en PDF
notes-edit = Modifier
notes-close = Fermer
notes-back = Retour aux notes
notes-add-about = Écrire une note sur cette personne
notes-sign-in = Les notes et contributions de la famille sont réservées à ses membres. Connectez-vous pour les lire.
notes-admin-title = Notes et contributions
notes-admin-intro = Écrivez ce que les fiches ne peuvent pas contenir : un souvenir, une recherche, une lettre, avec des images. Une note rattachée à des personnes apparaît sur leurs fiches ; une note rattachée à personne appartient à toute la famille. Chaque membre les lit toutes sur la page Famille.
notes-deleted = La note a été supprimée.
notes-col-title = Titre
notes-col-category = Catégorie
notes-col-date = Date
notes-col-about = Au sujet de
notes-col-author = Auteur
notes-col-actions = Actions
notes-new-title = Nouvelle note
notes-edit-title = Modifier la note
notes-section-about = La note
notes-field-title = Titre
notes-field-category = Catégorie
notes-field-category-hint = Texte libre ; les catégories déjà utilisées sont proposées.
notes-field-date = Date
notes-field-date-hint = 1952, 1952-06 ou 1952-06-14 ; vide s’il n’y en a pas.
notes-field-people = Personnes concernées
notes-field-people-hint = Tapez un nom et choisissez-le. Personne de nommé : la note est celle de toute la famille.
notes-person-add = Ajouter
notes-person-remove = Retirer cette personne
notes-field-text = Texte
notes-text-hint = Markdown, ou les boutons ci-dessus. Collez ou déposez des images, { $mb } Mo au plus chacune.
notes-tool-bold = Gras
notes-tool-italic = Italique
notes-tool-heading = Intertitre
notes-tool-list = Liste
notes-tool-quote = Citation
notes-tool-link = Lien
notes-tool-image = Ajouter une image
notes-tool-preview = Aperçu
notes-tool-write = Revenir à l'écriture
notes-uploading = Ajout de l'image…
notes-save = Enregistrer la note
notes-cancel = Annuler
notes-delete = Supprimer cette note
notes-delete-confirm = La note et ses images seront retirées de l'archive.
notes-category-suggestions = Souvenir, Recherche, Lettre, Récit, Recette, Document
notes-error-title = La note a besoin d'un titre. Rien n'a été enregistré.
notes-error-title-long = Le titre est trop long : 200 caractères au plus. Rien n'a été enregistré.
notes-error-text-long = Le texte est trop long. Rien n'a été enregistré.
notes-error-date = Cette date n'est pas comprise. Écrivez une année, un mois ou un jour : 1952, 1952-06 ou 1952-06-14. Rien n'a été enregistré.
notes-error-person = Personnes concernées : « { $name } ».
notes-error-image-gone = Une image collée n'attendait plus d'être enregistrée ; les images attendent six heures. Ajoutez-la de nouveau. Rien n'a été enregistré.
notes-error-too-many-images = Une note contient 40 images au plus. Rien n'a été enregistré.
notes-error-stale = Quelqu'un a modifié cette note pendant que vous y travailliez. Rouvrez-la pour voir sa version, puis refaites votre modification. Rien n'a été enregistré.
notes-error-refused = L'archive a refusé cette modification. Rien n'a été enregistré.
notes-error-image = Ce fichier n'est pas une image que ce site sait afficher : PNG, JPEG, GIF ou WebP.
notes-error-image-large = Cette image est trop lourde.
notes-stale-title = La note n'a pas été modifiée
notes-missing-title = Note introuvable
notes-missing = Il n'y a pas de note ici, ou elle ne vous est pas ouverte.
notes-not-yours = Un contributeur ne modifie que les notes qu'il a écrites. Un administrateur peut modifier celle-ci.
notes-pdf-failed-title = Le PDF n'a pas pu être fabriqué
notes-pdf-failed = Un problème est survenu en fabriquant le PDF de cette note. Lisez-la sur sa page en attendant, et prévenez un administrateur.
notes-dashboard-count = Notes écrites : { $n }
notes-dashboard-open = Écrire et gérer les notes

# The family the archive describes: the bundle room's form (2026-10-23)
kind-bundle = archive
manifest-title = La famille
manifest-intro = Ce que l’archive dit de sa famille. Son nom figure sur la page Famille, les titres et les PDF des notes.
manifest-name = Nom de la famille
manifest-name-hint = Affiché sur la page Famille et dans les titres. Les comptes suivent le nouveau nom.
manifest-description = Description
manifest-culture = Culture principale
manifest-culture-hint = Une étiquette de langue : fr, pl, pt-BR…
manifest-place = Lieu principal
manifest-place-hint = Le continent où la famille a surtout vécu.
manifest-span = Période couverte
manifest-span-hint = De la naissance la plus ancienne à la plus récente ; calculé à l’enregistrement.
manifest-license = Licence
manifest-license-type = Licence
manifest-save = Enregistrer les informations de la famille
manifest-saved = Les informations de la famille sont enregistrées. Les comptes suivent son nom.
manifest-error-long = Un champ est trop long : 120 caractères pour le nom, 2 000 pour la description. Rien n'a été enregistré.
manifest-error-culture = La culture est une étiquette de langue, comme fr, pl ou pt-BR. Rien n'a été enregistré.
manifest-error-stale = Quelqu'un a modifié les informations de la famille pendant que vous les modifiiez. Votre version est ci-dessous : l'enregistrer de nouveau remplace la sienne. Rien n'a été enregistré.
manifest-error-io = L'archive n'a pas pu être écrite ; le journal du serveur dit pourquoi. Rien n'a été enregistré.

# The family form: places, licences and the period (2026-10-23)
manifest-none = Non précisé
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = Aucune date de naissance enregistrée pour l'instant.
manifest-license-pick = Choisissez une licence pour voir ce qu'elle permet aux autres de faire de l'archive.
manifest-license-author = Auteur
manifest-license-author-value = { $who }, { $at }
manifest-license-author-none = Enregistré quand une licence est choisie.
manifest-error-place = Choisissez le lieu principal dans la liste. Rien n'a été enregistré.
manifest-error-licence = Choisissez la licence dans la liste. Rien n'a été enregistré.
continent-africa = Afrique
continent-antarctica = Antarctique
continent-asia = Asie
continent-europe = Europe
continent-north-america = Amérique du Nord
continent-south-america = Amérique du Sud
continent-oceania = Océanie
continent-several = Plusieurs continents
licence-all-rights-reserved-name = Tous droits réservés
licence-all-rights-reserved-about = Personne ne peut copier, partager ni réutiliser l'archive sans l'accord de la famille. C'est ce qui s'applique quand rien n'est dit.
licence-family-private-name = Privée, pour la famille
licence-family-private-about = Réservée aux membres de la famille : ni publiée, ni transmise hors de la famille.
licence-cc0-name = CC0 1.0 — domaine public
licence-cc0-about = Chacun peut copier, modifier et utiliser l'archive à toute fin, même commerciale, sans demander ni citer personne.
licence-cc-by-name = CC BY 4.0 — Attribution
licence-cc-by-about = Chacun peut partager et adapter l'archive, même commercialement, à condition de citer ses auteurs et d'indiquer ce qui a été modifié.
licence-cc-by-sa-name = CC BY-SA 4.0 — Attribution, Partage dans les mêmes conditions
licence-cc-by-sa-about = Chacun peut la partager et l'adapter, même commercialement, en citant ses auteurs ; ce qui en est tiré doit être partagé sous la même licence.
licence-cc-by-nc-name = CC BY-NC 4.0 — Attribution, Pas d'utilisation commerciale
licence-cc-by-nc-about = Chacun peut la partager et l'adapter en citant ses auteurs, mais pas à des fins commerciales.
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — Attribution, Pas d'utilisation commerciale, Partage dans les mêmes conditions
licence-cc-by-nc-sa-about = Chacun peut la partager et l'adapter en citant ses auteurs, sans usage commercial, et seulement sous la même licence.
licence-cc-by-nd-name = CC BY-ND 4.0 — Attribution, Pas de modification
licence-cc-by-nd-about = Chacun peut la partager telle quelle, même commercialement, en citant ses auteurs, mais pas en diffuser une version modifiée.
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — Attribution, Pas d'utilisation commerciale, Pas de modification
licence-cc-by-nc-nd-about = Chacun peut la partager telle quelle en citant ses auteurs, mais ni à des fins commerciales ni dans une version modifiée : la licence Creative Commons la plus stricte.
licence-odbl-name = ODbL 1.0 — Open Database License
licence-odbl-about = Faite pour les bases de données : chacun peut utiliser, partager et adapter les données en citant leur source, et une base qui en est tirée doit rester ouverte aux mêmes conditions.

# An account's own page and its record in the tree (2026-10-23)
nav-account = Mon compte
account-title = Mon compte
account-who = Le compte
account-email = Adresse e-mail
account-since = Depuis
account-person-title = Ma fiche dans l'arbre
account-person-intro = Quelle personne de l’arbre êtes-vous ? À vous de choisir, sans vérification ; modifiable à tout moment.
account-person-is = Ce compte dit être
account-person-unlink = Retirer le lien
account-person-pick = Une personne de l'arbre
account-person-link = Lier mon compte à cette fiche
account-person-change = Me lier plutôt à cette fiche
account-person-linked = Votre compte est lié à sa fiche.
account-person-unlinked = Votre compte n'est plus lié à une fiche.
account-person-error = Cette fiche n'a pas pu être liée :
account-person-io = Le compte n'a pas pu être enregistré ; le journal du serveur dit pourquoi. Rien n'a changé.
account-emergency-page = Le jeton d'urgence n'est pas un compte : il n'a ni page à lui ni fiche dans l'arbre.
accounts-person = Fiche :

## The merge screen (src/routes/merge.rs): two records a person says are one.
merge-action = Fusionner
merge-needs-two = Cochez exactement deux fiches pour les fusionner.
merge-title = Fusionner deux fiches
merge-lede = Rien ne change avant votre confirmation. La première fiche est conservée ; la seconde y est fusionnée, et gardée telle quelle dans le journal des modifications.
merge-keep = Conservée
merge-absorb = Fusionnée dans la première
merge-swap = Conserver l’autre
merge-refused = Cette fusion est refusée :
merge-refusal-ancestor = l’une est l’ancêtre de l’autre
merge-refusal-spouse-and-child = dans une même famille, l’une est conjoint et l’autre enfant
merge-refusal-spouses = ce sont les deux conjoints d’une même union
merge-refusal-siblings = elles sont enregistrées comme enfants des mêmes parents
merge-refusal-different-couples = les deux familles sont des couples différents
merge-refusal-child-is-spouse = un enfant de l’une des familles est conjoint dans l’autre
merge-refusal-same-record = c’est deux fois la même fiche
merge-conflicts = Ce qui diffère
merge-nothing = Rien à choisir : elles ne se contredisent pas.
merge-choice-keep = Fiche conservée
merge-choice-absorb = Autre fiche
merge-choice-both = Les deux
merge-because-higher-confidence = Suggéré : la valeur au degré de certitude le plus élevé.
merge-because-sourced = Suggéré : la valeur qui cite une source.
merge-because-kept-record = Suggéré : la valeur de la fiche conservée.
merge-because-more-restrictive = Suggéré : la visibilité la plus restrictive.
merge-because-protective = Suggéré : vivant, la lecture qui protège.
merge-because-both-kept = Suggéré : garder les deux textes.
merge-becomes-alias = Le nom non retenu est gardé comme autre nom.
merge-becomes-event = La valeur non retenue est gardée comme événement.
merge-agreements = Ce sur quoi elles s’accordent
merge-from-keep = de la fiche conservée
merge-from-absorb = de l’autre fiche
merge-from-both = identique des deux côtés
merge-from-joined = listes réunies
merge-from-combined = combinés
merge-effects = Ce qui change aussi
merge-effect-references = Références réécrites
merge-effect-self-links = Liens vers elle-même supprimés
merge-effect-listed-twice = Listes qui la nommeraient deux fois
merge-effect-same-couple = Familles du même couple
merge-confirm-label = Il peut quand même s’agir d’une seule personne. Pour fusionner malgré tout, tapez le nom exactement : { $name }
merge-go = Fusionner
merge-missing = L’une des deux fiches n’existe plus.
merge-done-title = Fusion faite
merge-done-lede = { $absorbed } fait désormais partie de { $kept }.
merge-report-references = Références réécrites
merge-report-events = Événements créés à partir d’une valeur non retenue
merge-report-source-conflicts = Désaccords notés sur une source
merge-report-links = Liens supprimés
merge-report-tidied = Entrées remises en ordre
merge-absorbed-record = La fiche fusionnée, telle qu’elle était
merge-open-kept = Ouvrir la fiche conservée

## Parenté (docs/adr/0020)

kin-tab = Parenté
kin-parent = { $adoptive ->
        [yes] { $sex ->
            [M] père adoptif
            [F] mère adoptive
           *[other] parent adoptif
        }
       *[no] { $sex ->
            [M] père
            [F] mère
           *[other] parent
        }
    }
kin-ancestor = { $n ->
        [2] { $sex ->
            [M] grand-père
            [F] grand-mère
           *[other] grand-parent
        }
        [3] { $sex ->
            [M] arrière-grand-père
            [F] arrière-grand-mère
           *[other] arrière-grand-parent
        }
        [4] { $sex ->
            [M] arrière-arrière-grand-père
            [F] arrière-arrière-grand-mère
           *[other] arrière-arrière-grand-parent
        }
       *[other] ancêtre à la { $n }e génération
    }
kin-child = { $adoptive ->
        [yes] { $sex ->
            [M] fils adoptif
            [F] fille adoptive
           *[other] enfant adoptif
        }
       *[no] { $sex ->
            [M] fils
            [F] fille
           *[other] enfant
        }
    }
kin-descendant = { $n ->
        [2] { $sex ->
            [M] petit-fils
            [F] petite-fille
           *[other] petit-enfant
        }
        [3] { $sex ->
            [M] arrière-petit-fils
            [F] arrière-petite-fille
           *[other] arrière-petit-enfant
        }
        [4] { $sex ->
            [M] arrière-arrière-petit-fils
            [F] arrière-arrière-petite-fille
           *[other] arrière-arrière-petit-enfant
        }
       *[other] { $sex ->
            [F] descendante à la { $n }e génération
           *[other] descendant à la { $n }e génération
        }
    }
kin-sibling = { $half ->
        [yes] { $sex ->
            [M] demi-frère
            [F] demi-sœur
           *[other] demi-frère ou demi-sœur
        }
       *[no] { $adoptive ->
            [yes] { $sex ->
                [M] frère adoptif
                [F] sœur adoptive
               *[other] frère ou sœur adoptifs
            }
           *[no] { $sex ->
                [M] frère
                [F] sœur
               *[other] frère ou sœur
            }
        }
    }
kin-uncle = { $half ->
        [yes] demi-
       *[no] {""}
    }{ $n ->
        [1] { $sex ->
            [M] oncle
            [F] tante
           *[other] oncle ou tante
        }
        [2] { $sex ->
            [M] grand-oncle
            [F] grand-tante
           *[other] grand-oncle ou grand-tante
        }
        [3] { $sex ->
            [M] arrière-grand-oncle
            [F] arrière-grand-tante
           *[other] arrière-grand-oncle ou arrière-grand-tante
        }
       *[other] { $sex ->
            [F] sœur d’un ancêtre
           *[other] frère d’un ancêtre
        }
    }
kin-nephew = { $half ->
        [yes] demi-
       *[no] {""}
    }{ $n ->
        [1] { $sex ->
            [M] neveu
            [F] nièce
           *[other] neveu ou nièce
        }
        [2] { $sex ->
            [M] petit-neveu
            [F] petite-nièce
           *[other] petit-neveu ou petite-nièce
        }
        [3] { $sex ->
            [M] arrière-petit-neveu
            [F] arrière-petite-nièce
           *[other] arrière-petit-neveu ou arrière-petite-nièce
        }
       *[other] { $sex ->
            [F] descendante d’un frère ou d’une sœur
           *[other] descendant d’un frère ou d’une sœur
        }
    }
kin-cousin = { $half ->
        [yes] demi-
       *[no] {""}
    }{ $sex ->
        [F] { $k ->
            [1] cousine germaine
            [2] cousine issue de germain
           *[other] cousine de { $k }e rang
        }
       *[other] { $k ->
            [1] cousin germain
            [2] cousin issu de germain
           *[other] cousin de { $k }e rang
        }
    }
kin-cousin-removed = { $older ->
        [yes] { $cousin } { $forebear }
       *[no] { $descendant } { $ofcousin }
    }
kin-of-forebear = { $n ->
        [1] { $sex ->
            [M] du père
            [F] de la mère
           *[other] d’un parent
        }
        [2] { $sex ->
            [M] du grand-père
            [F] de la grand-mère
           *[other] d’un grand-parent
        }
        [3] { $sex ->
            [M] de l’arrière-grand-père
            [F] de l’arrière-grand-mère
           *[other] d’un arrière-grand-parent
        }
       *[other] d’un ancêtre
    }
kin-descendant-word = { $n ->
        [1] { $sex ->
            [M] fils
            [F] fille
           *[other] enfant
        }
        [2] { $sex ->
            [M] petit-fils
            [F] petite-fille
           *[other] petit-enfant
        }
        [3] { $sex ->
            [M] arrière-petit-fils
            [F] arrière-petite-fille
           *[other] arrière-petit-enfant
        }
       *[other] { $sex ->
            [F] descendante
           *[other] descendant
        }
    }
kin-of-cousin = { $sex ->
        [F] d’une { $half ->
            [yes] demi-
           *[no] {""}
        }{ $k ->
            [1] cousine germaine
            [2] cousine issue de germain
           *[other] cousine de { $k }e rang
        }
       *[other] d’un { $half ->
            [yes] demi-
           *[no] {""}
        }{ $k ->
            [1] cousin germain
            [2] cousin issu de germain
           *[other] cousin de { $k }e rang
        }
    }
kin-partner = { $ended ->
        [yes] { $married ->
            [yes] { $sex ->
                [M] ex-époux
                [F] ex-épouse
               *[other] ex-conjoint
            }
           *[no] { $sex ->
                [M] ex-compagnon
                [F] ex-compagne
               *[other] ex-conjoint
            }
        }
       *[no] { $married ->
            [yes] { $sex ->
                [M] époux
                [F] épouse
               *[other] conjoint
            }
           *[no] { $sex ->
                [M] compagnon
                [F] compagne
               *[other] conjoint
            }
        }
    }
kin-parent-in-law = { $sex ->
        [M] beau-père
        [F] belle-mère
       *[other] beau-parent
    }
kin-child-in-law = { $sex ->
        [M] gendre
        [F] belle-fille
       *[other] gendre ou belle-fille
    }
kin-sibling-in-law = { $sex ->
        [M] beau-frère
        [F] belle-sœur
       *[other] beau-frère ou belle-sœur
    }
kin-step-parent = { $sex ->
        [M] beau-père
        [F] belle-mère
       *[other] beau-parent
    }
kin-step-child = { $sex ->
        [M] beau-fils
        [F] belle-fille
       *[other] enfant du conjoint
    }
kin-note-step-parent = { $sex ->
        [F] conjointe
       *[other] conjoint
    } { $next ->
        [M] du père
        [F] de la mère
       *[other] d’un parent
    }
kin-note-step-child = enfant { $via ->
        [F] de la conjointe
       *[other] du conjoint
    }
kin-foster-parent = { $sex ->
        [M] père d’accueil
        [F] mère d’accueil
       *[other] parent d’accueil
    }
kin-foster-child = { $sex ->
        [F] enfant accueillie
       *[other] enfant accueilli
    }
kin-guardian = { $sex ->
        [F] tutrice
       *[other] tuteur
    }
kin-ward = pupille
kin-relative-of-partner = { $relative } { $via ->
        [F] de la conjointe
       *[other] du conjoint
    }
kin-partner-of-relative = { $sex ->
        [M] époux
        [F] épouse
       *[other] conjoint
    } { $rsex ->
        [F] d’une
       *[other] d’un
    } { $relative }
kin-affine = { $sex ->
        [F] alliée par mariage
       *[other] allié par mariage
    }
kin-related = { $sex ->
        [F] reliée par les fiches de la famille
       *[other] relié par les fiches de la famille
    }
kin-note-adoptive = par adoption
kin-of = { $vowel ->
        [yes] d’{ $name }
       *[no] de { $name }
    }
kin-hop-parent = { $line ->
        [adoption] { $sex ->
            [M] père adoptif
            [F] mère adoptive
           *[other] parent adoptif
        }
        [foster] { $sex ->
            [M] père d’accueil
            [F] mère d’accueil
           *[other] parent d’accueil
        }
        [guardianship] { $sex ->
            [F] tutrice
           *[other] tuteur
        }
        [step] { $sex ->
            [M] beau-père
            [F] belle-mère
           *[other] beau-parent
        }
       *[other] { $sex ->
            [M] père
            [F] mère
           *[other] parent
        }
    }
kin-hop-child = { $line ->
        [adoption] { $sex ->
            [M] fils adoptif
            [F] fille adoptive
           *[other] enfant adoptif
        }
        [foster] { $sex ->
            [F] enfant accueillie
           *[other] enfant accueilli
        }
        [guardianship] pupille
        [step] { $sex ->
            [M] beau-fils
            [F] belle-fille
           *[other] enfant du conjoint
        }
       *[other] { $sex ->
            [M] fils
            [F] fille
           *[other] enfant
        }
    }
kin-group-partners = Conjoints
kin-group-ancestors = Ascendants
kin-group-descendants = Descendants
kin-group-siblings = Frères et sœurs
kin-group-uncles = Oncles et tantes
kin-group-nephews = Neveux et nièces
kin-group-cousins = Cousins
kin-group-in-laws = Par alliance
kin-group-step = Famille recomposée
kin-group-care = Accueil et tutelle
kin-group-rites = Rites et coutumes
kin-group-others = Autrement reliés
kin-find-title = Quel lien avec… ?
kin-find-lede = Choisissez une personne : son lien avec { $name }, étape par étape.
kin-find-label = Personne
kin-find-choose = Choisir une personne…
kin-find-go = Voir le lien
kin-find-none = Les fiches de la famille ne relient pas { $a } et { $b }.
kin-path = Le lien, étape par étape
kin-degree = parenté au { $n ->
        [1] 1er
       *[other] { $n }e
    } degré
kin-all-title = Toute la parenté de { $name }
kin-all-count = { $n ->
        [one] { $n } personne
       *[other] { $n } personnes
    }
kin-common-list = Ancêtres communs : { $names }
kin-also-list = aussi { $list }
kin-rites-help = Parrains, marraines et autres liens créés par une tradition ; l’arbre les dessine en pointillé.
kin-rite-recorded = La fiche dit « { $label } ».
kin-hidden = une personne masquée
kin-native = Dans les mots de la tradition
kin-legend-rite = Parrain, marraine, rite ou coutume
kin-tree-root = { $term } { $of }
kin-editor-rite = Rite ou coutume
kin-editor-rite-none = Aucun : un simple lien
kin-editor-rite-hint = Ce que cette personne est pour l’autre. Le choix remplit la relation et les mots, et la fiche garde le rite.
kin-editor-rite-option = { $rite } : { $word }

kin-tradition-christian = Christianisme
kin-tradition-orthodox = Christianisme orthodoxe
kin-tradition-hispanic = Monde hispanique et Philippines
kin-tradition-jewish = Judaïsme
kin-tradition-islamic = Islam
kin-tradition-south-asian = Asie du Sud
kin-tradition-east-asian = Asie de l’Est
kin-tradition-african = Afrique
kin-tradition-indigenous = Peuples autochtones et Pacifique
kin-tradition-civil = Civil et laïque

kin-rite-godparent = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [M] parrain
            [F] marraine
           *[other] parrain ou marraine
        }
    }
kin-rite-godparent-name = Parrainage
kin-rite-godparent-note = Parrain ou marraine, comme la fiche le dit.
kin-rite-baptism = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [M] parrain
            [F] marraine
           *[other] parrain ou marraine
        }
    }
kin-rite-baptism-name = Baptême
kin-rite-baptism-note = Parrain ou marraine de baptême.
kin-rite-confirmation = { $role ->
        [sponsored] { $sex ->
            [F] confirmande
           *[other] confirmand
        }
       *[other] { $sex ->
            [F] marraine de confirmation
           *[other] parrain de confirmation
        }
    }
kin-rite-confirmation-name = Confirmation
kin-rite-confirmation-note = Parrain ou marraine de confirmation, ou de chrismation.
kin-rite-wedding-sponsor = { $role ->
        [sponsored] { $sex ->
            [F] filleule de mariage
           *[other] filleul de mariage
        }
       *[other] { $sex ->
            [F] marraine de mariage
           *[other] parrain de mariage
        }
    }
kin-rite-wedding-sponsor-name = Parrainage de mariage
kin-rite-wedding-sponsor-note = Parrain ou marraine d’un mariage : padrino de boda, ninong, koumbaros.
kin-rite-kum = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [M] parrain
            [F] marraine
           *[other] parrain ou marraine
        }
    }
kin-rite-kum-name = Kumstvo
kin-rite-kum-note = Parrain ou témoin de mariage dans la tradition orthodoxe slave ; le kumstvo lie deux familles.
kin-rite-koumbaros = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [M] parrain
            [F] marraine
           *[other] parrain ou marraine
        }
    }
kin-rite-koumbaros-name = Koumbaros
kin-rite-koumbaros-note = Parrain de mariage ou de baptême dans la tradition grecque orthodoxe.
kin-rite-padrino = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [M] parrain
            [F] marraine
           *[other] parrain ou marraine
        }
    }
kin-rite-padrino-name = Padrino
kin-rite-padrino-note = Padrino ou madrina : au baptême, à la première communion, à la quinceañera.
kin-rite-compadre = { $sex ->
        [M] compère
        [F] commère
       *[other] compère ou commère
    }
kin-rite-compadre-name = Compadrazgo
kin-rite-compadre-note = Le lien entre les parents d’un enfant et ses parrain et marraine.
kin-rite-ninong = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [M] parrain
            [F] marraine
           *[other] parrain ou marraine
        }
    }
kin-rite-ninong-name = Ninong
kin-rite-ninong-note = Ninong ou ninang : parrain ou marraine de baptême, de confirmation ou de mariage aux Philippines.
kin-rite-sandek = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] sandek
    }
kin-rite-sandek-name = Brit milah : sandek
kin-rite-sandek-note = Tient l’enfant pendant la brit milah.
kin-rite-kvatter = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [F] kvatterin
           *[other] kvatter
        }
    }
kin-rite-kvatter-name = Brit milah : kvatter
kin-rite-kvatter-note = Porte l’enfant jusqu’à la brit milah.
kin-rite-milk-parent = { $role ->
        [sponsored] { $sex ->
            [M] fils de lait
            [F] fille de lait
           *[other] enfant de lait
        }
       *[other] { $sex ->
            [M] père de lait
            [F] mère de lait
           *[other] parent de lait
        }
    }
kin-rite-milk-parent-name = Parenté de lait
kin-rite-milk-parent-note = A allaité l’enfant : en droit musulman, la parenté de lait fait une famille et interdit le mariage, comme la naissance.
kin-rite-milk-sibling = { $sex ->
        [M] frère de lait
        [F] sœur de lait
       *[other] frère ou sœur de lait
    }
kin-rite-milk-sibling-name = Fratrie de lait
kin-rite-milk-sibling-note = Allaités par la même femme.
kin-rite-kafala = { $role ->
        [sponsored] pupille (kafala)
       *[other] { $sex ->
            [F] tutrice (kafala)
           *[other] tuteur (kafala)
        }
    }
kin-rite-kafala-name = Kafala
kin-rite-kafala-note = Recueille et élève un enfant sans la filiation d’une adoption, en droit musulman.
kin-rite-rakhi = { $sex ->
        [M] frère de rakhi
        [F] sœur de rakhi
       *[other] frère ou sœur de rakhi
    }
kin-rite-rakhi-name = Rakhi
kin-rite-rakhi-note = À Raksha Bandhan, une sœur noue un rakhi au poignet d’un frère, qui promet de la protéger.
kin-rite-guru = { $role ->
        [sponsored] disciple
       *[other] guru
    }
kin-rite-guru-name = Guru
kin-rite-guru-note = Maître spirituel et disciple.
kin-rite-gan-parent = { $role ->
        [sponsored] { $sex ->
            [M] filleul
            [F] filleule
           *[other] filleul ou filleule
        }
       *[other] { $sex ->
            [M] parrain
            [F] marraine
           *[other] parrain ou marraine
        }
    }
kin-rite-gan-parent-name = Gan qin
kin-rite-gan-parent-note = Parrain ou marraine choisis en Chine et dans les familles chinoises, souvent pour protéger l’enfant.
kin-rite-sworn-sibling = { $sex ->
        [M] frère juré
        [F] sœur jurée
       *[other] frère ou sœur jurés
    }
kin-rite-sworn-sibling-name = Fraternité jurée
kin-rite-sworn-sibling-note = Frères ou sœurs par serment (jiebai).
kin-rite-name-giver = { $role ->
        [sponsored] { $sex ->
            [F] enfant nommée
           *[other] enfant nommé
        }
       *[other] { $sex ->
            [F] donneuse du nom
           *[other] donneur du nom
        }
    }
kin-rite-name-giver-name = Don du nom
kin-rite-name-giver-note = A donné son nom à l’enfant (nazuke-oya au Japon).
kin-rite-nakodo = { $role ->
        [sponsored] { $sex ->
            [F] mariée par son entremise
           *[other] marié par son entremise
        }
       *[other] { $sex ->
            [F] entremetteuse
           *[other] entremetteur
        }
    }
kin-rite-nakodo-name = Nakōdo
kin-rite-nakodo-note = A arrangé le mariage et en a été témoin (nakōdo au Japon).
kin-rite-namesake = homonyme
kin-rite-namesake-name = Homonyme
kin-rite-namesake-note = Porte le même nom : en Afrique centrale et de l’Est, un lien de cadeaux et de soin (ndoyi, somo).
kin-rite-initiation = { $role ->
        [sponsored] { $sex ->
            [F] initiée
           *[other] initié
        }
       *[other] { $sex ->
            [F] marraine d’initiation
           *[other] parrain d’initiation
        }
    }
kin-rite-initiation-name = Initiation
kin-rite-initiation-note = A guidé la personne dans une initiation.
kin-rite-blood-pact = { $sex ->
        [M] frère de sang
        [F] sœur de sang
       *[other] frère ou sœur de sang
    }
kin-rite-blood-pact-name = Pacte de sang
kin-rite-blood-pact-note = Liés par un pacte de sang.
kin-rite-hanai = { $role ->
        [sponsored] { $sex ->
            [M] fils hānai
            [F] fille hānai
           *[other] enfant hānai
        }
       *[other] { $sex ->
            [M] père hānai
            [F] mère hānai
           *[other] parent hānai
        }
    }
kin-rite-hanai-name = Hānai
kin-rite-hanai-note = Élevé comme leur enfant selon la coutume hawaïenne, sans adoption légale.
kin-rite-hunka = { $role ->
        [sponsored] { $sex ->
            [M] fils hunka
            [F] fille hunka
           *[other] enfant hunka
        }
       *[other] { $sex ->
            [M] père hunka
            [F] mère hunka
           *[other] parent hunka
        }
    }
kin-rite-hunka-name = Hunka
kin-rite-hunka-note = Fait parent par la cérémonie lakota du hunka.
kin-rite-civil-sponsor = { $role ->
        [sponsored] { $sex ->
            [F] filleule civile
           *[other] filleul civil
        }
       *[other] { $sex ->
            [F] marraine civile
           *[other] parrain civil
        }
    }
kin-rite-civil-sponsor-name = Parrainage civil
kin-rite-civil-sponsor-note = Parrain ou marraine d’un parrainage civil, en mairie.
kin-rite-guardian = { $role ->
        [sponsored] pupille
       *[other] { $sex ->
            [F] tutrice
           *[other] tuteur
        }
    }
kin-rite-guardian-name = Tutelle
kin-rite-guardian-note = Tuteur ou tutrice, comme la fiche le dit.

## Polygamous unions (docs/adr/0021): one marriage per spouse, the
## co-spouses, the editor's two fields.
kin-co-spouse = { $sex ->
        [M] coépoux
        [F] coépouse
       *[other] coépoux
    }
union-co-spouses = { $sex ->
        [F] { $n ->
            [one] Coépouse
           *[other] Coépouses
        }
        [M] { $n ->
            [one] Coépoux
           *[other] Coépoux
        }
       *[other] { $n ->
            [one] Coépoux
           *[other] Coépoux
        }
    }
tree-key-polygamous = Union polygame
family-primary = Au centre de l’union
family-primary-hint = Dans une union polygame, la personne mariée à chacun des autres. Les autres ne sont pas mariés entre eux.
family-child-parent = Autre parent
family-child-parent-hint = Dans une union polygame, le conjoint qui est l’autre parent de cet enfant. Sans choix, la fiche ne le dit pas.

## Emblems (docs/adr/0022): the picture that stands for a family, and for
## the archive's.
emblem-family-edit = Emblème ou blason
emblem-family-title = L’emblème de la famille
emblem-archive-title = L’emblème de la famille de l’archive
emblem-archive-edit = Emblème ou blason de la famille
emblem-saved = L’emblème est enregistré.
emblem-choose = L’image qui représente la famille
emblem-help = Un blason, une marque, une photographie : ce qui représente cette famille. Il est montré en entier, près de l’union sur chaque fiche et sur le nœud du couple dans l’arbre.
emblem-archive-help = Un blason, une marque, une photographie : ce qui représente la famille dont parle cette archive. Il est montré en entier en tête de chaque page et sur la vue d’ensemble, à tous les visiteurs.
emblem-mode-auto-note = L’image que cette famille désigne comme son emblème, s’il y en a une.
emblem-mode-none = Pas d’emblème
emblem-no-images = Pas encore d’image : ajoutez-en une ci-dessous.
emblem-upload-title = Ajouter une image
emblem-upload-button = L’ajouter et l’utiliser
emblem-not-an-image = Un emblème est une image : PNG, JPEG, GIF, WebP ou BMP.
emblem-back = Retour à l’emblème
emblem-no-family-title = Famille introuvable
emblem-no-family-detail = Cette famille n’est pas dans l’archive, ou vous ne pouvez pas la voir.

## The Artificial family genealogist, AFG (docs/adr/0023): its room, where
## its computing comes from, and a test.
admin-nav-afg = AFG
afg-title = Généalogiste familial artificiel
afg-lede = D’où vient le calcul de l’AFG, et un essai.
afg-kpi-source = Calcul
afg-ready = prêt
afg-off-note = Choisissez d’où vient le calcul.
afg-not-ready = incomplet
afg-kpi-sent = Fichiers envoyés
afg-kpi-sent-note = { $kb } Ko, environ { $tokens } tokens
afg-kpi-last = Dernier essai
afg-never = aucun encore
afg-run-open = L’ouvrir
afg-source = D’où vient le calcul
afg-source-intro = Une adresse ; une adresse avec une clé et un modèle ; ou un agent connecté sur cette machine.
afg-kind = Mode de calcul
afg-kind-hint = Un service se joint par le réseau ; un agent tourne sur cette machine et doit y être connecté.
afg-kind-off = Aucun
afg-kind-anthropic = Claude (API d’Anthropic)
afg-kind-ollama = Ollama
afg-kind-openai = OpenAI
afg-kind-mistral = Mistral
afg-kind-vllm = vLLM
afg-kind-gemini = Gemini
afg-kind-claude-code = Claude Code
afg-kind-codex = Codex
afg-kind-opencode = OpenCode
afg-group-address = Une adresse
afg-group-service = Une adresse, une clé d’API et un modèle
afg-group-agent = Un agent de code sur cette machine
afg-url = Adresse
afg-url-hint = Laissée vide : l’adresse habituelle du service. Appel :
afg-key = Clé d’API
afg-key-set = Une clé est conservée, qui finit par { $end }.
afg-key-unset = Aucune clé n’est conservée.
afg-key-hint = Vide : la clé gardée reste, et ne s’affiche plus jamais.
afg-key-clear = Oublier la clé
afg-model = Modèle
afg-model-hint = Le nom du modèle tel que l’écrit le service. Tout nom est envoyé tel quel ; vide, celui par défaut.
afg-model-unlisted = Hors des suggestions : envoyé tel qu’il est écrit.
afg-effort = Effort
afg-effort-hint = L’intensité de réflexion du modèle. Tout mot est envoyé tel quel ; vide, celle par défaut.
afg-effort-unlisted = Hors des efforts de ce modèle : envoyé tel qu’il est écrit.
afg-effort-default = par défaut
afg-program = Programme
afg-program-found = Trouvé :
afg-program-missing = Introuvable sur cette machine.
afg-program-hint = Vide : cherché là où son installateur le met. Sinon, le chemin complet du programme.
afg-sensitive = Envoyer aussi les classes sensibles
afg-sensitive-hint = Décochée, santé, biométrie, génétique, dossiers judiciaires et comportement restent hors de l’envoi, comme dans un export partagé.
afg-timeout = Temps accordé (secondes)
afg-timeout-hint = Le temps qu’une réponse peut prendre avant qu’on y renonce.
afg-test = Essai
afg-test-intro = Envoie l’archive (JSON, sans photos) et votre cerveau avec une question ; la réponse s’ouvre dans une fenêtre.
afg-sent-files = Fichiers
afg-sent-files-value = { $n ->
        [one] { $n } fichier JSON
       *[other] { $n } fichiers JSON
    }, { $kb } Ko
afg-sent-tokens = Tokens, environ
afg-sent-records = Fiches
afg-sent-records-value = { $persons ->
        [one] { $persons } personne
       *[other] { $persons } personnes
    }, { $families ->
        [one] { $families } famille
       *[other] { $families } familles
    }
afg-sent-classes = Classes sensibles
afg-sent-classes-in = envoyées
afg-sent-classes-out = laissées de côté
afg-sent-question = Question
afg-privacy = L’essai envoie les fiches de la famille, vivants compris, au calcul choisi ci-dessus.
afg-test-button = Envoyer l’essai
afg-test-needs = Enregistrez d’abord un mode de calcul complet.
afg-test-question = Donne-moi le top 3 des familles disposant du plus grand nombre d’individus déclarés.
afg-test-empty = Bundle vide
afg-saved = Enregistré.
afg-error-not-saved = Non enregistré : { $error }
afg-error-off = Aucun mode de calcul n’est choisi.
afg-error-archive = L’archive n’a pas pu être lue : { $error }
afg-error-url = L’adresse doit commencer par http:// ou https://.
afg-error-key = Ce service demande une clé d’API.
afg-error-model = Un modèle est demandé ici, écrit avec des lettres, des chiffres et . _ : / @ [ ] + -
afg-error-effort = Un effort s’écrit avec des lettres, des chiffres et . _ : / @ [ ] + -
afg-error-program = Le programme doit être celui de l’agent (claude, codex ou opencode), par son chemin complet.
afg-run-state-running = En attente de la réponse
afg-run-state-done = Répondu
afg-run-state-failed = Échec
afg-run-seconds = { $s } s
afg-run-usage = Tokens lus
afg-run-usage-out = { $n } écrits
afg-run-cost = { $usd } USD
afg-run-waiting = L’archive est envoyée ; la réponse s’affichera ici. Cette page se recharge toutes les trois secondes.
afg-run-model = Modèle :
afg-run-failed-hint = Vérifiez le mode de calcul, le réseau que ce site peut joindre et, pour un agent, qu’il est installé et connecté sous le compte qui fait tourner ce site.
afg-run-gone-detail = Les essais sont gardés le temps de vie du processus, les dix derniers. Envoyez-en un nouveau.

## The AFG's brains, its window and who may use it (docs/adr/0024).
afg-kpi-brain = Votre cerveau
afg-brain-exchanges = { $n ->
        [one] { $n } échange
       *[other] { $n } échanges
    }
afg-brain-note = { $facts ->
        [one] { $facts } fait retenu
       *[other] { $facts } faits retenus
    } · { $brains ->
        [one] { $brains } cerveau sur le disque
       *[other] { $brains } cerveaux sur le disque
    }
afg-brain-no-account = Cette session n’a pas de compte, donc pas de cerveau.
afg-access-count = Comptes qui peuvent parler à l’AFG : { $n }
afg-access-set = À régler sur la page Comptes
afg-result-ok = Succès
afg-result-error = Erreur
afg-result-close = Fermer
afg-result-remembered = Retenu dans le cerveau de { $name } :
afg-result-nothing-kept = Rien de nouveau retenu dans le cerveau de { $name }.
accounts-afg = AFG
accounts-afg-allowed = Peut parler à l’AFG
accounts-afg-yes = oui
accounts-afg-no = non
accounts-afg-hint = Désactivé tant que la case n’est pas cochée. Sa mémoire est gardée à part, à côté de l’archive.

## An account's AFG brain on its own page: seen and forgotten (docs/adr/0024 §6).
account-brain-title = Le cerveau de votre AFG
account-brain-intro = Ce que l’AFG retient de vous, rappelé à chaque question. Effacez ce que vous voulez ; personne d’autre ne le voit.
account-brain-no-access = L’AFG n’est pas ouvert à votre compte : c’est un administrateur qui en décide, sur la page Comptes.
account-brain-none = L’AFG ne retient encore rien de vous.
account-brain-facts = Ce qu’il retient
account-brain-facts-none = Il n’a gardé aucun fait sur vous.
account-brain-exchanges = Vos échanges avec lui
account-brain-exchanges-none = Aucun échange n’est gardé.
account-brain-older = { $n ->
        [one] { $n } échange plus ancien est gardé et n’est pas listé ici.
       *[other] { $n } échanges plus anciens sont gardés et ne sont pas listés ici.
    }
account-brain-context-test = Essai dans la salle de l’AFG
account-brain-answer = Sa réponse
account-brain-forget = Oublier
account-brain-forget-all = Tout oublier…
account-brain-forget-all-confirm = Oui, oublier tout ce que l’AFG retient de moi
account-brain-forget-all-button = Tout oublier
account-brain-forgotten = Oublié : l’AFG ne le saura plus.
account-brain-forgotten-all = L’AFG ne retient plus rien de vous.
account-brain-already = C’était déjà oublié.
account-brain-confirm-needed = Cochez la case pour tout oublier.
account-brain-error = Rien n’a été oublié : { $error }
afg-brain-see = Voir et oublier

## My personal assistant: projects, conversations, code of conduct, second brain (docs/adr/0026).
assistant-title = Mon assistant personnel
assistant-entry = Vos projets et conversations avec l’AFG.
assistant-lede = Vos projets avec l’AFG, chacun avec son code de conduite, sa mémoire et ses conversations.
assistant-shut-no-account = Cette session n’a pas de compte, donc pas d’assistant personnel. Connectez-vous avec votre propre compte.
assistant-shut-not-open = L’AFG n’est pas ouvert à votre compte : c’est un administrateur qui en décide, sur la page Comptes.
assistant-shut-broken = Votre espace n’a pas pu être ouvert. Réessayez dans un instant ; si cela dure, prévenez un administrateur.
assistant-shut-gone = Il n’y a rien ici : ce projet ou cette conversation n’existe pas, ou n’est pas le vôtre.
assistant-back-account = Retour à mon compte
assistant-new-project = Nouveau projet
assistant-not-ready = Le calcul de l’AFG n’est pas encore réglé : lecture seule pour l’instant.
assistant-conversations-count = { $n ->
        [one] { $n } conversation
       *[other] { $n } conversations
    }
assistant-conduct-set = code de conduite défini
assistant-empty = Aucun projet pour l’instant. Un projet réunit les conversations sur un même sujet.
assistant-project-name = Nom du projet
assistant-create = Créer le projet
assistant-facts-count = { $n ->
        [one] { $n } fait retenu
       *[other] { $n } faits retenus
    }
assistant-archive-on = avec l’archive de la famille
assistant-start = Commencer une conversation
assistant-conversations = Conversations
assistant-no-conversation = Aucune conversation pour l’instant : votre première question en ouvre une.
assistant-conduct = Code de conduite
assistant-conduct-label = Comment l’AFG doit se comporter dans ce projet
assistant-conduct-hint = Vos consignes pour toutes les conversations de ce projet : le ton, la langue, ce qu’il faut regarder, ce qu’il faut laisser de côté. L’AFG les suit, sauf si elles lui demandent de vous tromper ou de nuire.
assistant-save = Enregistrer
assistant-memory = Mémoire du projet
assistant-memory-intro = Ce que l’AFG a choisi de retenir des conversations de ce projet : son second cerveau. Elle lui est redonnée à chaque message du projet. Faites-lui oublier ce que vous ne voulez plus qu’il sache.
assistant-forget = Oublier
assistant-forget-all = Tout oublier…
assistant-forget-all-confirm = Oui, oublier tout ce que l’AFG retient de ce projet
assistant-forget-all-button = Tout oublier
assistant-memory-none = L’AFG ne retient encore rien de ce projet.
assistant-settings = Réglages
assistant-archive = Donner l’archive de la famille à l’AFG dans ce projet
assistant-archive-hint = Toutes les fiches de l’archive, vivants compris, partent alors avec chaque message vers le calcul de l’AFG. Les classes sensibles restent dehors, sauf si un administrateur en a décidé autrement.
assistant-archive-size = Fichiers JSON : { $files } ; jetons par message : environ { $tokens }.
assistant-avatar-upload = Une image à vous comme avatar
assistant-avatar-send = Utiliser cette image
assistant-delete-project = Supprimer le projet
assistant-delete-project-confirm = Oui, supprimer ce projet, ses conversations, leurs fichiers et sa mémoire
assistant-delete = Supprimer
assistant-conversation-settings = Réglages de la conversation
assistant-conversation-title = Titre de la conversation
assistant-delete-conversation = Supprimer la conversation
assistant-delete-conversation-confirm = Oui, supprimer cette conversation et ses messages
assistant-thread = La conversation
assistant-reply = Votre message
assistant-sending = Envoi…
assistant-attach = Joindre
assistant-send = Envoyer
assistant-composer-hint = Entrée envoie, Maj+Entrée va à la ligne. Documents, photos, son ou vidéo : déposez-les, collez-les ou joignez-les, { $n } fichiers au plus, { $mb } Mo chacun.
assistant-avatar-choose = Choisir son avatar
assistant-avatar = Avatar
assistant-avatar-auto = Choisi pour lui
assistant-avatar-own = Votre image
assistant-kept = { $n ->
        [one] Retenu dans la mémoire du projet : { $n } fait
       *[other] Retenu dans la mémoire du projet : { $n } faits
    }
assistant-error-answer = L’AFG n’a pas pu répondre.
assistant-retry = Redemander
assistant-unanswered = Ce message est resté sans réponse.
assistant-writing = L’AFG écrit
assistant-saved = Enregistré.
assistant-project-deleted = Le projet est supprimé.
assistant-conversation-deleted = La conversation est supprimée.
assistant-memory-forgotten = Oublié : l’AFG ne le recevra plus.
assistant-memory-forgotten-all = L’AFG ne retient plus rien de ce projet.
assistant-avatar-saved = L’avatar est enregistré.
assistant-error-name = Un nom est nécessaire.
assistant-error-too-many = Il y en a déjà autant qu’un compte peut en garder.
assistant-error-not-saved = Non enregistré : le serveur n’a pas pu l’écrire. Réessayez dans un instant.
assistant-error-conduct-long = Le code de conduite est trop long.
assistant-error-confirm = Cochez la case pour confirmer.
assistant-error-too-big = Le message est trop lourd.
assistant-error-file-big = Un fichier est trop lourd.
assistant-error-too-many-files = Trop de fichiers dans un seul message.
assistant-error-text-long = Le message est trop long.
assistant-error-empty = Écrivez un message ou joignez un fichier.
assistant-error-busy = L’AFG répond déjà à autant de conversations qu’il peut. Réessayez dans un instant.
assistant-error-wait = L’AFG est encore en train de répondre dans cette conversation.
assistant-error-picture = Ce fichier n’est pas une image que ce site sait lire.
assistant-domain-genealogy = Généalogie
assistant-domain-medicine = Santé et médecine
assistant-domain-work = Travail et carrière
assistant-domain-money = Argent
assistant-domain-law = Droit et papiers
assistant-domain-learning = Apprendre
assistant-domain-home = Maison
assistant-domain-cooking = Cuisine
assistant-domain-travel = Voyages
assistant-domain-nature = Nature et jardin
assistant-domain-wellbeing = Bien-être
assistant-domain-family = Vie de famille
assistant-domain-arts = Arts et culture
assistant-domain-sports = Sport
assistant-domain-tech = Technologie
assistant-domain-science = Sciences
assistant-domain-history = Histoire et mémoire
assistant-domain-admin = Démarches
assistant-domain-pets = Animaux
assistant-domain-rites = Fêtes et rites

## The AFG on a person's record: a face or a photograph restored, a document read, the summary (docs/adr/0029).

summary-title = Résumé
summary-write = Rédiger le résumé
summary-edit = Modifier le résumé
summary-language = Langue
summary-by-ai = Rédigé par l'IA ({ $generator }) : à relire.
summary-note-title = Résumé — { $name }
summary-category = Résumé
summary-edit-title = Le résumé de la personne
afg-summary = Rédiger avec l'IA
afg-summary-busy = Rédaction…
afg-retouch = Retouche IA
afg-retouch-busy = Retouche…
afg-retouched = Restaurée par une IA : pas une photographie d'époque.
afg-read = Lire le document
afg-read-busy = Lecture…
afg-read-note-title = Transcription — { $name }
afg-read-category = Transcription
afg-rework = Retravailler l'image avec IA
afg-rework-busy = Retouche…
afg-images-off-admin = Aucun service d'images : à choisir dans Administration › AFG › Images.
notes-manage-in-media = Les modifier ou les supprimer dans l'onglet Médias
notes-delete-short = Supprimer
notes-none-yet = Aucune note sur cette personne pour l'instant.
afg-record-title = L'IA sur la fiche
afg-record-back = Retour à la fiche
afg-record-no-access = L'AFG n'est pas ouverte à votre compte. Un administrateur peut l'ouvrir sur la page Comptes.
afg-record-images-off = Aucun service d'images n'est configuré. Un administrateur le choisit dans Administration › AFG › Images.
afg-record-text-off = Le calcul de l'AFG n'est pas configuré. Un administrateur le choisit dans Administration › AFG.
afg-record-no-face = Cette personne n'a pas de photographie à retravailler.
afg-record-no-document = Ce document n'est pas parmi les fichiers de cette personne, son fichier n'est pas conservé ici, ou c'est déjà une copie faite par une IA.
afg-record-not-a-picture = Seule une image peut être retouchée.
afg-record-not-readable = Seuls une image ou un PDF peuvent être lus.
afg-record-pdf-unread = Le calcul actuel de l'AFG ne sait pas lire un PDF. OpenAI, Claude, Gemini, Claude Code et OpenCode le savent.
afg-record-failed = L'IA n'a pas abouti : { $reason }
afg-record-bad-picture = Le service n'a pas renvoyé une image que ce site puisse conserver.
afg-record-no-text = L'IA n'a trouvé aucun texte dans ce document.
afg-record-stale = La fiche a changé entre-temps. Rechargez la page et recommencez.
afg-record-refused = L'archive l'a refusé : { $reason }
afg-record-summary-not-yours = Un autre membre a écrit ce résumé : seuls son auteur ou un administrateur peuvent le remplacer.
afg-images = Images
afg-images-intro = D’où viennent les images de l’AFG : un service payant (OpenAI, Google, Canva) ou une machine de la famille avec carte graphique.
afg-image-kind = Service d'images
afg-image-kind-hint = Les modèles d'OpenAI et de Google, l'IA de Canva par votre compte, ou un serveur de la famille : Ollama, tout serveur compatible OpenAI, ComfyUI.
afg-image-kind-off = Aucun
afg-image-kind-openai = OpenAI (GPT Image)
afg-image-kind-gemini = Google Gemini (Nano Banana)
afg-image-url-hint = Vide : l'adresse habituelle du service. Pour un serveur à vous : son adresse (par ex. http://192.168.1.20:8000/v1).
afg-image-key-hint = Changer de service oublie la clé : donnez celle du nouveau en même temps.
afg-image-model-hint = Vide : le premier proposé ; un autre modèle est envoyé tel qu'il est écrit.
afg-images-privacy = La photographie part vers ce service à chaque demande d'un membre. Choisissez un service à qui votre famille confie ses images.
afg-images-saved = Le service d'images est enregistré.
afg-images-ready = Les images sont prêtes : les boutons de la fiche peuvent en demander.
afg-images-not-ready = Aucun service d'images : les boutons d'image de la fiche sont grisés.
afg-images-choose = Choisir le service d'images

## Canva and the family's own servers as the AFG's picture service (docs/adr/0030).

afg-image-kind-canva = Canva (son IA, par votre compte)
afg-image-kind-ollama = Ollama (expérimental)
afg-image-kind-openai-compatible = Un serveur à vous (compatible OpenAI)
afg-image-kind-comfyui = ComfyUI (votre workflow)
afg-image-workflow = Workflow ComfyUI (format API)
afg-image-workflow-hint = Exporté de ComfyUI au format API (« Save (API Format) »). La photo est placée dans chaque nœud LoadImage, et ce repère, dans un texte, devient la consigne de restauration :
afg-image-key-optional-hint = Facultative : seulement si votre serveur en demande une.
afg-images-at-home = La photo reste sur votre réseau : ce service tourne sur une machine de la famille.
afg-images-saved-canva = Canva est choisi : connectez maintenant un compte.
afg-canva-connect = Connecter Canva
afg-canva-connected = Connecté au compte Canva { $account }.
afg-canva-connected-unnamed = Un compte Canva est connecté.
afg-canva-not-connected = Aucun compte Canva n'est connecté.
afg-canva-disconnect = Déconnecter Canva
afg-canva-disconnect-confirm = Déconnecter ce compte Canva ? Les images déjà faites restent dans l'archive.
afg-canva-step-open = 1. Ouvrir Canva et accepter
afg-canva-step-paste = 2. Collez ici l'adresse de la page où Canva vous a renvoyé (http://localhost…)
afg-canva-paste-hint = Cette page ne s'affiche pas : c'est normal. Son adresse suffit, et elle ne sert qu'une fois.
afg-canva-finish = Terminer la connexion
afg-canva-finish-wait = La connexion prend jusqu'à une minute : elle prépare aussi, dans Canva, la page sur laquelle chaque photo est retouchée.
afg-canva-done-ready = Canva est connecté, et sa page de travail est prête dans le dossier « ax-genealogy ». Canva peut ensuite suspendre ses crédits IA quelques minutes : si la première retouche est refusée, réessayez un peu plus tard.
afg-canva-cooldown = Canva a suspendu les crédits IA de ce compte pour quelques minutes. Réessayez dans un moment.
afg-canva-done = Canva est connecté.
afg-canva-gone = Canva est déconnecté.
afg-canva-save-first = Choisissez d'abord Canva comme service d'images, et enregistrez.
afg-canva-policy = Une seule connexion sert toute la famille : les retouches passent par ce compte Canva, ses crédits et son dossier « ax-genealogy ». La politique d'usage de Canva prévoit que chacun connecte son propre compte ; en connectant le vôtre pour l'AFG, vous en prenez la responsabilité.
afg-error-workflow = Le workflow ComfyUI manque, ou n'est pas au format API (il lui faut au moins un nœud LoadImage).
afg-error-canva = Canva n'est pas connecté : utilisez « Connecter Canva ».
