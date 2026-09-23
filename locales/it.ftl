# axgf-cms — testi dell'interfaccia, italiano.
#
# QUALITÀ AUTOMATICA — non riletto da una persona di madrelingua italiana. Il
# lessico genealogico ha corrispondenze consolidate che variano secondo la
# tradizione archivistica, e questa traduzione può essere sbagliata. Le
# correzioni sono benvenute — si veda CONTRIBUTING.md.
#
# Scelte adottate (da contestare pure):
#   union → unione · link → legame · confidence → certezza
#   reliability → affidabilità · source → fonte
#   primary source → fonte primaria · occupation → occupazione
#   record → scheda · archive → archivio · godparent → padrino/madrina
#   witness → testimone · speculative → ipotetico
#
# Plurale: regole CLDR one / other. Mai sostituirle con una logica propria.
#
# Date: «12 aprile 1923» — giorno, mese in minuscolo, anno senza virgola. La
# tabella dei mesi sta dentro lo schema della data.
#
# REGOLA: questo file traduce solo l'interfaccia. Nomi, luoghi, note e
# mestieri vengono dall'archivio e restano nella loro lingua e scrittura.

app-name = ax-genealogy

## Testata e piè di pagina

nav-tree = Albero
nav-convert = Importa
nav-admin = Amministrazione
nav-sign-in = Accedi
nav-sign-out = Esci
footer-open-format = L'archivio della vostra famiglia è un solo file che resta vostro, scritto in un formato aperto: si aprirà ancora molto tempo dopo che questo sito sarà sparito.
footer-open-format-link = Sul formato

## Preferenze

settings-title = Impostazioni
settings-tabs-label = Sezioni delle impostazioni
settings-tab-theme = Tema
settings-tab-language = Lingua
settings-tab-appearance = Aspetto
settings-done = Fatto
prefs-language = Lingua
prefs-theme = Tema
prefs-background = Sfondo
prefs-background-on = Una velatura di colore dietro la pagina
prefs-apply = Applica
prefs-reviewed = riletta
prefs-machine = automatica, { $coverage }%
prefs-machine-complete = completa, non ancora riletta
prefs-machine-title = Tradotta senza rilettura da una persona di madrelingua. Il lessico genealogico in particolare può essere sbagliato: le parole per un'unione, un padrino o una fonte primaria cambiano secondo la tradizione archivistica di ogni paese. Le correzioni sono benvenute, e CONTRIBUTING.md dice da dove cominciare.

theme-light = Chiaro
theme-dark = Scuro
theme-system = Come il sistema
theme-high-contrast = Contrasto elevato
theme-sepia = Seppia
theme-deuteranopia = Deuteranopia
theme-protanopia = Protanopia
theme-tritanopia = Tritanopia
theme-colour-blind-note = adatto al daltonismo
theme-contrast-note = contrasto massimo

## Albero

tree-title-around = Attorno a { $name }
tree-title-whole = L'albero intero
tree-lede-focused = { $ancestors ->
        [one] Un antenato
       *[other] { $ancestors } antenati
    }, { $descendants ->
        [one] un discendente
       *[other] { $descendants } discendenti
    } e { $spouses ->
        [one] un partner
       *[other] { $spouses } partner
    }, { $depth } generazioni per parte.
tree-filter-label = Filtra le schede visibili
tree-filter-placeholder = Digita un nome…
tree-centre-on = Centra su
tree-depth = Generazioni per parte
tree-show = Mostra
tree-hidden-notice = { $n ->
        [one] Una persona è mostrata senza i suoi dati
       *[other] { $n } persone sono mostrate senza i loro dati
    }
tree-hidden-because-role = , perché la loro visibilità supera quanto il vostro account può leggere.
tree-hidden-because-anonymous = , perché non sono pubbliche.
tree-hidden-sign-in = Accedete, se avete un account.
tree-restricted-card = Questa scheda non vi è visibile
tree-empty = Non c'è ancora nessuno da disegnare.
tree-unplaced = In nessuna famiglia registrata

## La scheda

record-identity = Identità
record-life-events = Eventi della vita
record-family = Famiglia
record-other-relationships = Altre relazioni
record-occupations = Occupazioni
record-places = Luoghi
record-sources-documents = Fonti e documenti
record-notes = Note
record-history = Storia delle modifiche
record-raw = Dati grezzi
record-raw-summary-note = il JSON da cui è costruita questa pagina

record-identity-help = Ogni nome registrato con il suo tipo, il periodo in cui fu usato e la fonte che lo sostiene, con la scrittura originale accanto alla traslitterazione latina dove differiscono, più genere, stato in vita e visibilità.
record-life-events-help = Nascita, morte e ogni evento a cui questa persona ha preso parte, in ordine di data, ciascuno con il suo ruolo — così un matrimonio a cui fece solo da testimone sta accanto al proprio. Un fatto senza data va in fondo, invece di fingere di venire per primo.
life-nothing-recorded = Per questa persona non risultano eventi, mestieri né luoghi. In un file convertito è la norma: GEDCOM trasporta ciò che qualcuno ha annotato, e quasi tutte le voci sono un nome e una data.
life-add-first = Registra il primo evento
record-family-help = Genitori e fratelli, poi ogni unione con il suo tipo, le date, il luogo, il modo in cui finì e i figli in ordine di nascita.
record-other-relationships-help = Ogni legame che ha questa persona a uno dei due capi, letto dal suo lato: la stessa scheda appare come «padrino di» da un capo e «figlioccio di» dall'altro.
record-occupations-help = Le occupazioni come periodi su un unico asse condiviso, così due impieghi si confrontano a occhio; dove manca un estremo la barra resta aperta.
record-places-help = Ogni luogo che questa scheda tocca, con quel che vi accadde e con la storia dei confini che rende un luogo comprensibile nel tempo.
record-sources-documents-help = Ogni fonte elenca i fatti di questa pagina che vi si appoggiano, in ordine di forza della prova.
record-notes-help = Note su questa scheda, compreso il testo che nessun convertitore ha saputo interpretare e che è stato conservato alla lettera invece di essere scartato.
record-history-help = Ogni modifica salvata di questa scheda, dalla più recente. Chi ha corretto cosa è un fatto sulle persone che tengono l'albero, non sulla famiglia che vi si trova: resta perciò fuori dall'archivio esportato ed è mostrato solo ai parenti che hanno effettuato l'accesso.
record-raw-help = Qui non c'è nulla prodotto per la visualizzazione: questa è la scheda esattamente come è conservata, fino ai nomi dei campi. Se un giorno doveste leggere l'archivio senza questo sito, vedreste proprio questo.
record-help-toggle = Che cosa mostra questa sezione

record-gender = Genere
record-living = In vita
record-visibility = Visibilità
record-yes = sì
record-no = no
record-name-type = Tipo di nome
record-name-used = In uso
record-name-evidence = Prova
record-transliteration = Traslitterazione latina
record-born = Nato/a
record-died = Morto/a
record-parents = Genitori
record-siblings = Fratelli e sorelle
record-children = Figli
record-unknown-person = [Ignoto]
record-restricted-person = Riservata
record-restricted-title = Questa scheda non vi è visibile
record-absent-person-title = Nominato in quest'albero ma senza scheda propria
record-confidence = Certezza
record-source = Fonte
record-download = Scarica

## Accesso

access-restricted-title = Non visibile a voi
access-restricted-signed-in = La visibilità di questa scheda supera quanto il vostro account può leggere. Un amministratore può cambiare o la visibilità della scheda o il vostro ruolo.
access-restricted-anonymous = Questa scheda non è pubblica. Accedete per vedere se il vostro account può leggerla.
access-role-title = Non per il vostro ruolo
access-role-admin = Questa è una pagina da amministratore. Il vostro account può creare e modificare schede, ma non gestire account, eliminare schede o esportare l'archivio.
access-role-write = Il vostro account può leggere quest'albero ma non modificarlo. Un amministratore può portare il vostro ruolo a collaboratore.
access-scope-title = Fuori dal vostro ramo
access-scope-named = Il vostro account è limitato a un ramo dell'albero, e questa scheda riguarda qualcuno che ne sta fuori. Ogni persona nominata in una scheda deve stare dentro il vostro ramo: altrimenti una famiglia con un partner esterno sarebbe un modo per riscrivere la discendenza di quella persona.
access-scope-unnamed = Il vostro account è limitato a un ramo dell'albero, e questa scheda non nomina nessuno con cui confrontarla. Fonti e luoghi li modificano gli account che hanno accesso all'albero intero.

## Errori

error-not-found-title = Non trovato
error-not-found-detail = Questa pagina qui non esiste.
error-no-such-person-title = Nessuna persona simile
error-no-such-person-detail = Qui non c'è nessuna persona con quell'identificativo.
error-no-such-entity-title = Nessun elemento simile
error-no-such-entity-detail = Qui non c'è nessuna scheda con quell'identificativo.
error-deleted-while-editing = Qui non c'è nessuna scheda con quell'identificativo. Può darsi che sia stata eliminata mentre la modificavate.
error-no-such-file-title = Nessun file simile
error-no-such-file-detail = Qui non c'è nessun documento con quell'identificativo, oppure il documento è registrato senza file — un documento richiamato indica qualcosa che sta altrove.
error-not-an-image-title = Non è un'immagine
error-not-an-image-detail = Per questo documento non c'è anteprima, perché non è un'immagine che questa versione sappia decodificare.
error-back = Indietro

## Accesso al sito

login-title = Accedi
login-lede = Gli account li crea un amministratore.
login-username = Nome utente
login-password = Password
login-submit = Accedi
login-wrong = Quel nome utente e quella password non corrispondono.
login-token-wrong = Quel token non è corretto.
login-throttled = Troppi tentativi falliti. Attendete qualche minuto e riprovate.
login-no-accounts-title = Questa installazione non ha ancora nessun account.
login-no-accounts-detail = Non c'è di proposito una pagina di configurazione: l'intervallo fra la messa in opera e il primo accesso è esattamente il momento in cui un'installazione è indifesa, perciò il primo amministratore si crea dalla riga di comando.
login-no-accounts-note = Stampa una password generata su stderr una volta sola e mai più. Fino ad allora l'unica via d'ingresso è il token d'emergenza qui sotto.
login-emergency-summary = Accesso d'emergenza
login-emergency-detail = Il token condiviso apre ancora una sessione da amministratore ed esiste per una cosa sola: rientrare quando il file .acl è andato perduto o tutti gli amministratori sono chiusi fuori. Non è un account: non ha preferenze proprie, e il registro delle modifiche lo annota come emergency-token invece che come persona. Il suo uso è registrato come avviso.
login-emergency-label = Token d'emergenza
login-emergency-submit = Usa il token d'emergenza
login-sign-in-prompt = Accedete per entrare nel pannello di amministrazione.

## Amministrazione

admin-title = Amministrazione
admin-lede = Si modifica { $path } — { $total } elementi, { $files ->
        [one] un file allegato
       *[other] { $files } file allegati
    }, { $size } su disco. Ogni modifica è scritta in un colpo solo; una modifica rifiutata lascia il file intatto.
admin-entities = Elementi
admin-create = Crea
admin-new-kind = Nuovo: { $kind }
admin-operations = Operazioni
admin-validate = Verifica
admin-deduplicate = Unisci i duplicati
admin-export = Esporta l'archivio
admin-accounts = Account
admin-roles-note = Verifica, unione dei duplicati, esportazione, eliminazione e gestione degli account sono riservate all'amministratore. Un collaboratore raggiunge ogni altra pagina di qui.
admin-dedup-confirm = L'unione dei duplicati fonde schede e riscrive l'archivio. Continuare?
admin-recent-changes = Modifiche recenti
admin-recent-note = Le ultime { $shown } di { $total ->
        [one] una modifica registrata
       *[other] { $total } modifiche registrate
    }, da { $path }.
admin-sessions-open = { $n ->
        [one] Una sessione aperta adesso.
       *[other] { $n } sessioni aperte adesso.
    }
admin-no-changes-yet = Con questa applicazione non è ancora stato cambiato nulla. Ogni salvataggio d'ora in poi è annotato in { $path }.
admin-last-validation = Ultima verifica
admin-bundle-heavy = Quest'archivio pesa { $size }. Viene caricato per intero all'avvio e tenuto in memoria, quindi oltre circa { $warn } il sito comincia a costare memoria vera e i riavvii diventano lenti. Va bene per un archivio di famiglia, non per una mediateca: se gli allegati crescono senza limite, teneteli in un deposito di file e fate che l'archivio vi punti.

admin-fields = Campi
admin-raw-json = JSON grezzo
admin-raw-json-help = L'elemento intero, così nulla è immodificabile: elenchi come i partner e i figli di una famiglia, o la storia dei confini di un luogo, stanno proprio qui. Questo è il documento di partenza; i campi qui sopra vengono poi scritti sui percorsi che possiedono, quindi modificate un valore o in un posto o nell'altro, non in entrambi. Deve leggersi come JSON, altrimenti non si salva nulla.
admin-save = Salva
admin-cancel = Annulla
place-editor-title = Modifica un luogo
place-add-detail = Completa questo luogo
place-names = Nomi
place-name-primary = Principale
place-name-lang = Lingua
place-name-value = Nome
place-names-hint = Una riga per ogni nome registrato. Un luogo amministrato da tre imperi porta tre nomi; il principale è quello mostrato ovunque.
place-where = Posizione
place-type = Tipo
place-region = Regione
place-country-current = Paese oggi
place-country-hint = ISO 3166-1 alpha-2, per esempio PL, FR, DE.
place-country-history = Storia dei confini
place-history-country = Stato
place-history-from = Da
place-history-until = Fino a
place-country-history-hint = Quale Stato ha tenuto questo luogo e in quale periodo. Conta per la genealogia: un atto scritto in russo nel 1880 e uno scritto in polacco nel 1930 possono nominare lo stesso villaggio.
place-coordinates = Coordinate
place-lat = Latitudine
place-lon = Longitudine
place-precision = Precisione
place-identifiers = Identificatori
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } è usato da { $n ->
        [one] un altro record
       *[other] altri { $n } record
    }.
place-error-no-name = Un luogo ha bisogno di almeno un nome.
place-error-coords-pair = Latitudine e longitudine vanno insieme: indicatele entrambe o nessuna.
place-error-coords-number = Latitudine e longitudine devono essere numeri.
place-error-coords-range = La latitudine va da -90 a 90 e la longitudine da -180 a 180.
place-type-continent = continente
place-type-country = Stato
place-type-region = regione
place-type-department = dipartimento
place-type-city = città
place-type-village = villaggio
place-type-district = quartiere
place-type-street = via
place-type-building = edificio
place-type-farm = cascina
place-type-island = isola
place-type-historical = storico
place-type-unknown = sconosciuto
place-precision-exact = esatta
place-precision-building = edificio
place-precision-street = via
place-precision-city_center = centro città
place-precision-region_center = centro della regione
place-precision-country_center = centro del paese
place-precision-approximate = approssimativa

place-coordinates-hint = Inserirle a mano è la via consueta. Molti luoghi registrati sotto un'amministrazione passata non si trovano affatto con una ricerca moderna.
place-geocode-search = Cerca questo nome
place-geocode-hint = Invia nome, regione e paese al servizio di geocodifica, un luogo per volta. Nulla viene salvato finché non salvi tu.
place-geocode-off = La ricerca dei nomi è disattivata. Richiede un indirizzo di contatto con cui il servizio identifichi questa installazione; avvia il server con --geocoder-contact per attivarla.
place-geocode-query = Cercato: { $q }
place-geocode-error = Il servizio di ricerca non è raggiungibile. I campi delle coordinate qui sopra funzionano ancora.
place-geocode-none = Nessun risultato. Per un villaggio registrato sotto amministrazione russa, prussiana o austriaca questo è l'esito ordinario; inserisci la posizione a mano.
place-geocode-not-a-place = non è un centro abitato
place-geocode-use = Usa questo
place-geocode-attribution = Risultati da OpenStreetMap tramite Nominatim, con licenza Open Database.

place-paste = Incolla una posizione
place-paste-placeholder = un collegamento a una mappa, o 52.0782795, 21.2508068
place-paste-read = Leggila
place-paste-hint = Un collegamento Google Maps o OpenStreetMap, un URI geo:, una coppia di numeri, o gradi-minuti-secondi come 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Letta nei campi qui sopra. Controllala, poi salva.
place-paste-unreadable = Questa non è una posizione leggibile qui. I campi qui sopra accettano ancora una coppia di numeri.

place-map-hint = Fai clic sulla mappa per posare il punto, oppure trascina lo spillo. Fanno fede i campi qui sopra.
place-map-clear = Togli il punto
place-open-in-map = Cerca questo luogo in OpenStreetMap e incolla qui il collegamento

person-tab-record = Scheda
person-tab-life = Vita
person-tab-media = Materiali
person-tab-tree = Albero
person-tab-history = Modifiche
person-tree-depth = { $n } generazioni per lato. L'albero intero è più sotto.
person-tree-alone = Questa scheda non nomina genitori, coniugi né figli, quindi non c'è forma da disegnare attorno.

record-no-evidence = A questa scheda non è allegato nulla: né una fonte né un documento. È lo stato ordinario di un file convertito, non un suo difetto: il GEDCOM porta con sé i fatti e lascia indietro ciò che li provava.
record-no-evidence-signed-out = Accedi per allegarne uno.
admin-delete = Elimina
admin-not-set = — non impostato —
admin-edit = Modifica
admin-page-of = Pagina { $page } di { $pages }
admin-previous = Precedente
admin-next = Successiva
admin-saved = Salvato come versione { $version } — { $summary }
admin-not-saved = Non salvato
admin-created = Creato
admin-not-created = Non creato
admin-deleted = Eliminato
admin-not-deleted = Non eliminato — non è cambiato nulla
admin-what-changed = che cosa è cambiato
admin-field = Campo
admin-from = Da
admin-to = A
admin-version = versione { $version }

## Account

accounts-title = Account
accounts-lede = Conservati in { $path }, con permessi 600, accanto all'archivio e mai dentro. Un archivio si copia, si spedisce e si pubblica; le impronte delle password che viaggiassero al suo interno farebbero di ogni copia dell'albero una copia delle credenziali.
accounts-existing = Esistenti
accounts-username = Nome utente
accounts-role = Ruolo
accounts-status = Stato
accounts-branch = Ramo
accounts-last-seen = Ultimo accesso
accounts-change = Cambia
accounts-you = (voi)
accounts-active = attivo
accounts-disabled = disattivato
accounts-never = mai
accounts-whole-tree = albero intero
accounts-roots = { $n ->
        [one] una radice
       *[other] { $n } radici
    }
accounts-add = Aggiungi un account
accounts-no-registration = Non c'è di proposito né iscrizione autonoma né invito. Per un archivio di famiglia basta un amministratore che conosca tutti, e questo toglie del tutto una superficie d'abuso invece di doverla difendere.
accounts-password-hint = Lasciate vuoto e ne verrà generata una, mostrata una volta sola. Almeno { $min } caratteri se la impostate voi.
accounts-new-password-placeholder = nuova password (vuoto = invariata)
accounts-email = Email
accounts-optional = (facoltativo)
accounts-create = Crea l'account
accounts-role-viewer = lettore — legge le schede pubbliche e quelle di famiglia
accounts-role-contributor = collaboratore — inoltre crea, modifica e carica file
accounts-role-admin = amministratore — inoltre gestisce gli account, elimina ed esporta
accounts-branch-hint = Limita ciò che questo account può modificare a quelle persone, ai loro discendenti e ai loro coniugi.
accounts-branch-reading = Non limita ciò che può leggere: quello lo governa la visibilità di ogni scheda, e le due cose restano separate di proposito.
accounts-branch-placeholder = un identificativo di persona per riga
accounts-ids-in-bundle = Identificativi delle persone in quest'albero
accounts-emergency-warning = Siete entrati con il token d'emergenza. Concede i diritti d'amministratore per questa sessione ma non è un account: non ha preferenze proprie, e il registro annoterà le vostre modifiche come emergency-token invece che come persona. Createvi qui sotto un account vero ed entrate con quello.
accounts-created-with-password = Creato { $username }. La password è { $password }: viene mostrata una volta sola ed è conservata solo come impronta Argon2id, quindi passatela adesso.
accounts-created = Creato { $username }.
accounts-updated = Aggiornato { $username }. Ogni sessione aperta di quell'account è stata chiusa.
accounts-username-taken = Quel nome utente è già preso.
accounts-pick-role = Scegliete un ruolo.
accounts-no-such = Quell'account non esiste.
accounts-last-admin = È l'unico amministratore attivo. Promuovete prima qualcun altro: un'installazione senza amministratore si recupera solo modificando il file .acl o usando il token d'emergenza.
accounts-not-saved = Non salvato: { $error }

## Conflitti

conflict-title = Qualcun altro l'ha cambiato prima
conflict-lede = { $who } ha salvato una modifica a questo elemento ({ $kind }) alle { $when }, dopo che l'avevate aperto. La vostra modifica non è stata salvata e non è stato sovrascritto nulla.
conflict-no-merge = Qui non si unisce nulla in automatico. Fondere le modifiche di due persone produce una scheda che non ha scelto nessuna delle due, e in genealogia due redattori in disaccordo su una data di solito stanno leggendo fonti diverse — che è una domanda per una persona, non per un programma. Confrontate qui sotto e decidete.
conflict-versions = Siete partiti dalla versione { $expected }; la scheda ora è alla versione { $current }.
conflict-both-changed = L'avete cambiato entrambi
conflict-both-changed-detail = Questi campi li avete modificati entrambi. Qualunque cosa salviate sostituirà ciò che vi ha messo { $who }:
conflict-different-fields = Avete modificato campi diversi, quindi nulla del lavoro di { $who } è in discussione — ma riapplicare scrive comunque il vostro elemento intero sopra il loro. Controllate le due colonne prima di salvare.
conflict-field-by-field = Campo per campo
conflict-theirs = In che cosa l'ha cambiato { $who }
conflict-yours = In che cosa l'avete cambiato voi
conflict-unchanged-by-you = non modificato da voi
conflict-unchanged-by-them = non modificato da loro
conflict-nothing-differs = Nessuna delle due versioni differisce da quella da cui siete partiti in alcun campo mostrato in questa pagina. Il numero di versione è avanzato, quindi qualcuno ha salvato la scheda senza cambiare nulla di ciò che contiene.
conflict-what-now = E adesso
conflict-reapply = Riapplica la vostra versione sopra la loro
conflict-reapply-hint = Questa è la vostra modifica, riportata sulla versione { $version }. Correggetela qui per tenere ciò che volete del lavoro di { $who }, poi salvate. La loro versione è mostrata sotto, da cui copiare.
conflict-save-over = Salva questa sopra la loro
conflict-discard = Scarta la mia e ricomincia
conflict-their-version = La versione di { $who }, come sta adesso
conflict-history-of = Storia di questo elemento ({ $kind })

## Importazione

convert-title = Importa un file di famiglia
convert-submit = Importa
convert-result-title = Rapporto d'importazione
convert-download = Scarica l'archivio

## Date

date-unknown = Data ignota
date-not-recorded = Non registrata
date-circa = circa { $date }
date-between = fra il { $from } e il { $to }
date-before = prima del { $date }
date-after = dopo il { $date }
date-preserved = registrata come «{ $text }»
date-day-month-year = { $day } { $month ->
        [1] gennaio
        [2] febbraio
        [3] marzo
        [4] aprile
        [5] maggio
        [6] giugno
        [7] luglio
        [8] agosto
        [9] settembre
        [10] ottobre
        [11] novembre
        [12] dicembre
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] gennaio
        [2] febbraio
        [3] marzo
        [4] aprile
        [5] maggio
        [6] giugno
        [7] luglio
        [8] agosto
        [9] settembre
        [10] ottobre
        [11] novembre
        [12] dicembre
        *[other] { $month }
    } { $year }
date-decade = gli anni { $decade }
date-century = il { $century }° secolo
date-quarter-century = il { $quarter ->
        [1] primo
        [2] secondo
        [3] terzo
       *[other] quarto
    } quarto del { $century }° secolo

## Altre pagine di errore

error-back-to-start = Torna all'inizio
error-payload-missing-title = Nessun file simile
error-payload-missing-detail = Il contenuto di quel documento non è nella cache.
error-payload-unopenable-detail = Il contenuto di quel documento non si è potuto aprire.
error-no-such-document-detail = Qui non c'è nessun documento con quell'identificativo.
error-bad-preference-title = Non è una delle scelte
error-bad-preference-detail = Non è né una lingua né un aspetto che questo sito offra. Non è stato cambiato nulla.
error-unknown-kind-title = Tipo sconosciuto
error-unknown-kind-detail = «{ $kind }» non è un tipo di scheda. Quest'archivio contiene: { $kinds }.
error-io-title = Salvataggio impossibile
error-io-detail = { $error }. Su disco non è cambiato nulla.
error-upload-too-large = Quel file supera il limite di { $mb } MB. Non è stato conservato nulla e l'archivio è invariato.
error-upload-refused = Il documento è stato rifiutato: { $reason }. L'archivio è invariato.
error-back-to-person = Torna alla scheda
error-no-such-person-to-attach = Qui non c'è nessuna persona con quell'identificativo, quindi non c'è nulla a cui allegare un documento.
error-upload-title = Quel caricamento non è stato conservato
error-download-expired-title = Quel download è scaduto
error-download-expired-detail = Un'importazione si conserva quindici minuti, poi viene scartata. Importate di nuovo il file.
error-upload-none = Non è stato caricato alcun file. Sceglietene prima uno.
error-upload-unsupported = Quel tipo di file l'archivio non lo conserva. Si accettano immagini, PDF, testo semplice, audio e video; il tipo si legge dai byte stessi del file, quindi rinominare un eseguibile non serve. L'SVG è rifiutato senz'altro, perché un SVG può contenere uno script.
error-export-unreadable-title = Impossibile leggere l'archivio esportato
error-export-unreadable-detail = { $error }

## Pagina dell'albero, séguito

tree-title-suffix = albero
tree-back-to-focused = Torna alla vista attorno a una persona
tree-show-all = Mostra tutte e { $n }
tree-width-notice = Questa vista è larga { $width } pixel: su uno schermo da 1500 pixel fa { $screens ->
        [one] uno schermo
       *[other] { $screens } schermi
    } di scorrimento orizzontale.
tree-confidence-label = Certezza:
tree-band-certain = certo
tree-band-high = alta
tree-band-medium = media
tree-band-low = ipotetico
tree-counts = { $drawn } persone su { $total } · { $generations ->
        [one] una generazione
       *[other] { $generations } generazioni
    }
tree-unplaced-count = { $n } senza posto
tree-contradicts-title = Quest'albero si contraddice.
tree-contradicts-detail = Nessuna disposizione di righe può soddisfarlo, perciò il legame qui sotto è stato lasciato fuori dalla numerazione delle generazioni e qualche riga può essere disegnata nel posto sbagliato. Correggete quella delle due schede che è sbagliata.
tree-contradicts-pair = Registrati sia come coppia sia come genitore e figlio:
tree-contradicts-more = { $n ->
        [one] Un'altra contraddizione non è elencata.
       *[other] Altre { $n } contraddizioni non sono elencate.
    }
tree-no-people = In quest'albero non c'è ancora nessuno.
tree-no-people-cta = Importate un file di famiglia, oppure aggiungete la prima persona.
tree-nobody-selected = Per questa selezione non c'è nessuno da disegnare.
tree-nobody-selected-cta = Cominciate dalla vista predefinita.
tree-edge-union = Un'unione registrata
tree-edge-parentage = Una discendenza registrata

## Pagina iniziale

home-empty = Ancora nulla di registrato. Importate un file di famiglia per portare qui un albero già esistente, oppure aggiungete a mano la prima persona.
home-count = { $total ->
        [one] Una scheda
       *[other] { $total } schede
    }, in un solo file che appartiene alla famiglia.
home-browse = Sfoglia l'albero
home-convert = Importa un file di famiglia
home-unnamed-family = Quest'albero di famiglia
home-in-this-tree = Che cosa ha registrato finora la famiglia
home-showcase-title = Dove quest'albero dice già più di nomi e date
home-showcase-example = Vedi un esempio →
home-nothing-title = Ancora nulla da mostrare.
home-nothing-detail = Importate un file di famiglia per portare qui un albero già esistente, oppure partite da zero e aggiungete voi stessi la prima persona.

## Schede di panoramica

showcase-links-title = { $n ->
        [one] Una relazione fuori dalla famiglia
       *[other] { $n } relazioni fuori dalla famiglia
    }
showcase-links-detail = Padrini, datori di lavoro, testimoni e maestri, ciascuno con date proprie, fonte propria e il vostro grado di certezza.
showcase-occupations-title = { $n ->
        [one] Un'occupazione con un inizio e una fine
       *[other] { $n } occupazioni con un inizio e una fine
    }
showcase-occupations-detail = «Maestra elementare, 1948-1978» conserva la sua durata ed è disegnata come una barra attraverso gli anni, non come una sola riga datata.
showcase-uncertain-title = { $n ->
        [one] Una data lasciata incerta quanto è stata tramandata
       *[other] { $n } date lasciate incerte quanto sono state tramandate
    }
showcase-uncertain-detail = Circa, prima, dopo e fra restano quattro affermazioni diverse. Una data che la fonte non ha saputo fissare non è mai mostrata come se l'avesse fatto.
showcase-preserved-title = { $n ->
        [one] Una data conservata nelle parole in cui fu scritta
       *[other] { $n } date conservate nelle parole in cui furono scritte
    }
showcase-preserved-detail = Una formulazione che nessuno ha saputo leggere come data resta esattamente com'è scritta, invece di essere scartata in silenzio.
showcase-sources-title = { $n ->
        [one] Una fonte con la sua affidabilità registrata
       *[other] { $n } fonti con la loro affidabilità registrata
    }
showcase-sources-detail = { $primary ->
        [one] Una fonte primaria.
       *[other] { $primary } primarie.
    } Ogni fatto mostra su quale prova si regge e quanto quella prova è forte.
showcase-places-title = { $n ->
        [one] Un luogo i cui confini si sono spostati
       *[other] { $n } luoghi i cui confini si sono spostati
    }
showcase-places-detail = Una città può appartenere a stati diversi in tempi diversi, e la scheda dice quale valeva quando.

## Dettagli della scheda

record-also-recorded-as = registrato anche come
record-borders-moved = Confini spostati:
record-display-name = nome mostrato
record-read-as = letto come
record-note = Nota
record-living-yes = in vita
record-deceased = defunto/a
record-centre-tree-here = Centra l'albero qui
record-centre-tree-title = Sposta l'albero per centrarlo su questa persona
record-open-full-page = Apri la pagina intera ↗
record-open-full-title = Apri la pagina autonoma e condivisibile
record-edit = Modifica
panel-empty = Scegliete una scheda per vedere qui il documento completo di quella persona.
person-see-in-tree = Vedi questa persona nell'albero
person-visibility-inline = visibilità:
person-age-at-death = morto a { $n } anni
person-age-now = { $n } anni
person-born-in = nato a { $place }
person-died-in = morto a { $place }
person-children-count = { $n ->
        [one] un figlio
       *[other] { $n } figli
    }
person-generations-below = { $n ->
        [one] una generazione sotto
       *[other] { $n } generazioni sotto
    }
person-portrait-of = Fotografia di { $name }
person-no-portrait = Nessuna fotografia registrata

## Esiti delle operazioni

result-diagnostics = Segnalazioni
result-diagnostics-note = Ogni segnalazione restituita dalla libreria, comprese le avvertenze che non hanno bloccato l'operazione. Non se ne filtra nessuna.
result-no-diagnostics = La libreria non ha restituito segnalazioni.
result-continue = Avanti
result-dashboard = Quadro generale
person-sections-label = Sezioni di questa pagina

## Vocabulary the structured editors offer

name-part-nasab = nasab (discendenza)
name-part-laqab = laqab (epiteto)
name-part-kunya = kunya (teknonimo)
name-part-nisbah = nisbah (origine)
name-part-alias = alias
name-part-religious_name = nome religioso
name-part-pen_name = pseudonimo
name-type-pen_name = pseudonimo
gender-U = Non registrato

## Sezioni della scheda, dettagli

record-notes-title = Da notare su questa scheda:
record-name = Nome
record-type = Tipo
record-cause = Causa:
record-as = come
record-partner-not-recorded = Partner non registrato
record-union-from = Dal
record-union-at = a
record-union-until = fino al
record-occupation-from = dal
record-occupation-until = fino al
record-source-reliability = Affidabilità
record-source-supports = Sostiene
record-photographs = Fotografie
record-documents = Documenti
record-file = File
record-status = Stato
record-size = Dimensione
record-absent-document = Nominato da questa persona ma non conservato qui.
record-no-file = nessun file
record-attach-document = Allega un documento
record-upload = Carica
record-upload-help = Fino a { $mb } MB per file. Gli allegati stanno accanto all'albero e vengono riscritti nell'archivio all'esportazione, così una fotografia viaggia insieme alla famiglia a cui appartiene. Il tipo di file si legge dal suo contenuto e non dal nome: si accettano immagini, PDF, testo semplice, audio e video. L'SVG è rifiutato, perché un SVG può contenere uno script.
record-upload-help-short = Fino a { $mb } MB. L'SVG è rifiutato.
record-verbatim-note = Conservato esattamente come lo dava la scheda, perché nessun convertitore ha saputo interpretarlo.
record-file-to-attach = File da allegare
record-document-type = Tipo di documento
record-caption = Didascalia
record-caption-placeholder = Didascalia (facoltativa)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Tipi di elemento

kind-person = persona
kind-family = famiglia
kind-event = evento
kind-link = legame
kind-occupation = occupazione
kind-source = fonte
kind-place = luogo
kind-document = documento

kind-person-plural = { $n ->
        [one] persona
       *[other] persone
    }
kind-family-plural = { $n ->
        [one] famiglia
       *[other] famiglie
    }
kind-event-plural = { $n ->
        [one] evento
       *[other] eventi
    }
kind-link-plural = { $n ->
        [one] legame
       *[other] legami
    }
kind-occupation-plural = { $n ->
        [one] occupazione
       *[other] occupazioni
    }
kind-source-plural = { $n ->
        [one] fonte
       *[other] fonti
    }
kind-place-plural = { $n ->
        [one] luogo
       *[other] luoghi
    }
kind-document-plural = { $n ->
        [one] documento
       *[other] documenti
    }

## Elenchi

list-matching = { $total ->
        [one] Una corrispondenza
       *[other] { $total } corrispondenze
    }, { $per_page } per pagina.
list-filter-placeholder = Filtra per nome o identificativo
list-filter = Filtra
list-clear = Azzera
list-summary = Descrizione
list-id = Identificativo
list-actions = Azioni
list-nothing = Qui non c'è nulla.
list-nothing-matching = Qui non c'è nulla che corrisponda a «{ $q }».
list-delete-confirm = Eliminare questo elemento ({ $kind })? Scegliete che cosa ne sarà degli elementi che lo richiamano:
list-policy-reject = Rifiuta
list-policy-reject-detail = — rifiuta se qualcosa lo richiama ancora. Non si perde nulla.
list-policy-cascade = A cascata
list-policy-cascade-detail = — eliminalo e togli davvero ogni richiamo a esso.
list-policy-orphan = Lascia orfani
list-policy-orphan-detail = — eliminalo ma conserva le schede che lo richiamano, con il richiamo azzerato.

## Completezza

completeness-dates-title = Le date secondo la forma che hanno davvero
completeness-no-dates = Nessuna data registrata finora.
completeness-dates-note = Una data che qualcuno ha saputo fissare al giorno e una che qualcuno ha saputo collocare solo in un decennio sono due affermazioni diverse, ed entrambe restano come sono state date. Il testo che non si è potuto leggere affatto come data si conserva parola per parola invece di essere scartato.
completeness-shape-exact = esatta
completeness-shape-exact-note = un giorno di calendario intero
completeness-shape-approximate = approssimata
completeness-shape-approximate-note = circa, oppure solo un anno o un decennio
completeness-shape-ranged = delimitata
completeness-shape-ranged-note = prima, dopo o fra
completeness-shape-preserved = alla lettera
completeness-shape-preserved-note = testo non interpretabile, conservato tale e quale
completeness-shape-unknown = ignota
completeness-shape-unknown-note = registrata come non nota

## Pagina d'importazione

convert-page-title = Importa un file di famiglia
convert-lede = Portate qui un albero già esistente da un file GEDCOM, l'esportazione che produce la maggior parte dei programmi di genealogia. Qui non si conserva nulla, e l'albero che questo sito già mostra resta esattamente com'era.
convert-file-label = File di famiglia (.ged)
convert-file-hint = Fino a { $mb } MB. Un albero di 767 persone pesa circa 320 KB.
convert-confidence-label = Quanto sono certi questi fatti, per cominciare
convert-confidence-hint = Il file che state importando non dice quanto qualcuno fosse sicuro, perciò ogni fatto ha bisogno di un punto di partenza. Mettetelo basso per un albero messo insieme in fretta, più alto per uno lavorato sui documenti. La lettura onesta di questo numero è «importato, e da allora non controllato da nessuno»: potrete poi alzare o abbassare ogni fatto, uno alla volta.
convert-lang-label = Lingua dei nomi di luogo
convert-lang-hint = Una sigla come en, fr o it.

## Rapporto d'importazione

convert-failed = L'importazione non è andata a buon fine
convert-try-another = Prova un altro file
convert-converted = Importato { $filename }
convert-result-lede = { $total ->
        [one] Una scheda
       *[other] { $total } schede
    }, { $size } KB. È entrato tutto con una certezza di { $confidence }, con i nomi di luogo letti come { $lang }. L'albero che questo sito mostra non è stato toccato.
convert-produced = Che cosa è passato
convert-skipped-title = { $n ->
        [one] Una voce che non si è potuta leggere
       *[other] { $n } voci che non si sono potute leggere
    }
convert-skipped-note = Queste voci non contenevano nulla che si potesse portare qui.
convert-other-diagnostics = { $n ->
        [one] Un'altra cosa da sapere
       *[other] Altre { $n } cose da sapere
    }
convert-clean = Non è rimasto indietro nulla: ogni voce del file è passata.
convert-download-title = Scaricamento
convert-download-named = Scarica { $name }
convert-download-note = Conservato qui quindici minuti e poi scartato, quindi scaricatelo adesso.
convert-another = Importa un altro file
admin-history-on = il
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Un errore
       *[other] { $errors } errori
    }, { $warnings ->
        [one] un'avvertenza
       *[other] { $warnings } avvertenze
    }, { $infos ->
        [one] una nota
       *[other] { $infos } note
    }.
admin-warnings-never-block = Le avvertenze non bloccano mai: sono informazione, non sbarramento.
admin-validator-clean = La verifica non ha segnalato nulla.
record-occupations-help-undated = Un'occupazione si registra con un inizio e una fine, così più d'una si può confrontare su un'unica linea del tempo. Quest'archivio ha i nomi dei mestieri ma non le date — cosa consueta dopo un'importazione, perché la maggior parte dei file di famiglia non ha dove tenerle — quindi non c'è ancora una scala da disegnare.
record-occupations-help-axis = Un'occupazione è uno stato con una durata, non un evento in una sola data. Tutti i tratti condividono un asse, { $from }–{ $to }.
admin-value-not-set = non impostato
admin-validation-report = Rapporto di verifica
admin-dedup-complete = Unione dei duplicati conclusa
admin-dedup-refused = Unione dei duplicati rifiutata
record-birth-order = ordine di nascita
record-start-not-recorded = inizio non registrato
record-end-not-recorded = fine non registrata
record-document-no-file = Il documento è registrato qui, ma il file stesso non c'è
panel-selected-person = Persona scelta

## Fasce delle generazioni

tree-band-generation = Generazione { $g }
tree-band-people = { $n ->
        [one] una persona
       *[other] { $n } persone
    }
tree-band-unplaced = Senza posto
tree-band-unplaced-note = { $n ->
        [one] una persona senza famiglia — mostrata invece che omessa
       *[other] { $n } persone senza famiglia — mostrate invece che omesse
    }

## Vocabolario controllato

gender-M = Maschile
gender-F = Femminile
gender-NB = Non binario
gender-unrecorded = Non registrato

name-part-given_name = nome di battesimo
name-part-family_name = cognome
name-part-patronymic = patronimico
name-part-matronymic = matronimico
name-part-middle_name = secondo nome
name-part-nickname = soprannome
name-part-prefix = prefisso
name-part-suffix = suffisso
name-part-particle = particella
name-part-part = elemento

name-type-primary = principale
name-type-other = altro
name-type-alias = d'uso
name-type-birth = di nascita
name-type-married = da coniugata
name-type-religious = religioso
name-type-transliteration = traslitterazione
name-type-nickname = soprannome

## Annotazioni sulla scheda

note-links = { $n ->
        [one] una relazione fuori dalla famiglia, con date e fonti proprie
       *[other] { $n } relazioni fuori dalla famiglia, con date e fonti proprie
    }
note-occupations = { $n ->
        [one] un mestiere registrato con un inizio e una fine
       *[other] { $n } mestieri registrati con un inizio e una fine
    }
note-birth-imprecise = una data di nascita che la fonte non ha saputo fissare, mostrata com'è registrata
note-death-imprecise = una data di morte che la fonte non ha saputo fissare, mostrata com'è registrata
note-names = { $n ->
        [one] un nome registrato
       *[other] { $n } nomi registrati
    }
note-transliteration = un nome nella propria scrittura accanto alla traslitterazione latina
note-witnessed = { $n ->
        [one] un evento di cui fu testimone e non protagonista
       *[other] { $n } eventi di cui fu testimone e non protagonista
    }

visibility-public = pubblica
visibility-members = familiari
visibility-contributors = collaboratori
visibility-private = riservata

## Descrizioni di riga negli elenchi di amministrazione

family-label-couple = { $children ->
        [0] { $a } e { $b }
        [one] { $a } e { $b } — un figlio
       *[other] { $a } e { $b } — { $children } figli
    }
family-label-half = { $children ->
        [0] { $a } e { $unknown }
        [one] { $a } e { $unknown } — un figlio
       *[other] { $a } e { $unknown } — { $children } figli
    }
family-label-children = { $others ->
        [0] { $first } — genitori non registrati
        [one] { $first } e un fratello — genitori non registrati
       *[other] { $first } e { $others } fratelli — genitori non registrati
    }
family-label-empty = Famiglia senza nessuno registrato

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } e { $b }
event-more-people = { $a } e { $b } e { $others ->
        [one] un altro
       *[other] altri { $others }
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } senza titolo
list-unnamed = { $kind } senza nome

## Vocabolari della specifica negli elenchi

event-category-birth = Nascita
event-category-death = Morte
event-category-marriage = Matrimonio
event-category-divorce = Divorzio
event-category-baptism = Battesimo
event-category-burial = Sepoltura
event-category-immigration = Immigrazione
event-category-emigration = Emigrazione
event-category-census = Censimento
event-category-residence = Residenza
event-category-military = Servizio militare
event-category-education = Istruzione
event-category-other = Evento

reliability-primary = fonte primaria
reliability-secondary = fonte secondaria
reliability-tertiary = fonte terziaria
reliability-recollection = testimonianza orale
reliability-derivative = opera derivata
reliability-authored = opera d’autore
reliability-oral = tradizione orale
reliability-unknown = affidabilità ignota

document-type-photo = fotografia
document-type-certificate = atto
document-type-letter = lettera
document-type-record = registrazione d'archivio
document-type-newspaper = ritaglio di giornale
document-type-other = documento

## Dove questa scheda potrebbe dire di più

completeness-title = Dove quest'albero potrebbe dire di più
completeness-intro = Che cosa è registrato e che cosa è ancora vuoto.
completeness-import-title = Che cosa ha portato l'importazione
completeness-import-intro = Contato sul file che avete appena caricato. Una riga vuota è qualcosa che il file di partenza non registrava — non qualcosa che l'importazione ha perso.

completeness-headline-full = Ogni tipo di dettaglio qui sotto è registrato da qualche parte in quest'albero.
completeness-headline-empty = { $total ->
        [one] L'unico tipo di dettaglio qui sotto non è ancora registrato da nessuna parte.
       *[other] Nessuno dei { $total } tipi di dettaglio qui sotto è ancora registrato.
    } Ciascuno è un punto in cui la scheda potrebbe dire di più.
completeness-headline-partial = { $carried ->
        [one] Un tipo di dettaglio qui sotto è registrato
       *[other] { $carried } tipi di dettaglio qui sotto sono registrati
    }; { $empty ->
        [one] uno è ancora vuoto
       *[other] { $empty } sono ancora vuoti
    }.

completeness-metric-confidence = Quanto è certo ogni fatto
completeness-metric-confidence-none = Nessuno dei { $slots } fatti qui dice quanto sia certo. Una data letta su un atto e una tirata a indovinare si somigliano, finché non si somigliano più.
completeness-metric-confidence-uniform = { $with } fatti su { $slots } portano un punteggio, e ognuno è lo stesso numero ({ $modal }). È quel che lascia dietro di sé un'importazione in blocco: un valore di comodo su cui nessuno è tornato. Nessuno è ancora stato giudicato uno per uno.
completeness-metric-confidence-some = { $with } fatti su { $slots } portano un punteggio. { $modal_count } condividono un unico valore ({ $modal }); { $assessed } se ne discostano e sono quindi stati guardati uno alla volta.
completeness-metric-confidence-many = { $with } fatti su { $slots } portano un punteggio, e { $assessed } di essi si discostano dal valore più frequente ({ $modal }), su { $distinct } livelli distinti. Quest'albero registra un'incertezza vera e variata.

completeness-metric-parentage = Quanto è certo ogni legame genitore-figlio
completeness-metric-parentage-none = Nessuna discendenza qui dice quanto sia certa. Adozioni, linee contestate e ricostruzioni da una sola menzione sono proprio i punti in cui una famiglia ha bisogno di registrare il dubbio — e l'albero disegna un legame meno certo con una linea più pallida.
completeness-metric-parentage-some = { $n ->
        [one] Una discendenza porta un punteggio proprio
       *[other] { $n } discendenze portano un punteggio proprio
    }, così una linea ipotetica è visibilmente più debole di una documentata.

completeness-metric-links = Relazioni oltre il sangue e il matrimonio
completeness-metric-links-none = Padrini, datori di lavoro, testimoni, maestri, tutori. Non ne è registrata ancora nessuna. Ognuna può portare date proprie, la sua fonte e il vostro grado di certezza.
completeness-metric-links-some = { $n ->
        [one] Una registrata, con date proprie, fonte propria e il vostro grado di certezza.
       *[other] { $n } registrate, ciascuna con date proprie, fonte propria e il vostro grado di certezza.
    }

completeness-metric-occupations = Mestieri registrati con un inizio e una fine
completeness-metric-occupations-none = Nessuna occupazione registrata. Un mestiere esercitato per trent'anni dice di una vita più di una sola voce datata.
completeness-metric-occupations-undated = { $total ->
        [one] È registrata un'occupazione, senza date
       *[other] Sono registrate { $total } occupazioni, senza date
    }. Aggiungete un inizio e una fine e si potranno confrontare fianco a fianco su un'unica linea del tempo.
completeness-metric-occupations-some = { $span } su { $total } hanno un inizio o una fine, quindi si possono confrontare fianco a fianco su un'unica linea del tempo.

completeness-metric-sources = Fonti con il grado di affidabilità registrato
completeness-metric-sources-none = Nessuna fonte registrata. Dire da dove viene un fatto è ciò che permette a un parente di verificarlo più tardi — o di non essere d'accordo e dire perché.
completeness-metric-sources-some = { $graded } su { $total } dicono quanto sono forti, così un'affermazione che si regge su un atto di nascita non è visibilmente la stessa cosa di una che si regge su un ricordo.

completeness-what-is-recorded = Che cosa la scheda può dire
completeness-in-this-tree = In quest'albero
completeness-not-yet = non ancora registrato

## Ruoli di un partecipante in un evento

role-spouse = coniuge
role-spouse_1 = primo coniuge
role-spouse_2 = secondo coniuge
role-subject = persona interessata
role-participant = partecipante
role-witness = testimone
role-officiant = celebrante
role-informant = dichiarante
role-godparent = padrino o madrina

phys-no-source = senza fonte
phys-col-date = Quando
phys-col-source = Fonte
phys-col-confidence = Attendibilità
phys-col-note = Nota
phys-field-height-cm = Statura
phys-field-weight-kg = Peso
phys-field-eye-colour = Colore degli occhi
phys-field-hair-colour = Colore dei capelli
phys-field-build = Corporatura
phys-field-handedness = Lateralità
phys-field-features = Segni particolari
phys-field-military = Servizio militare
phys-field-languages = Lingue parlate
phys-field-blood-group = Gruppo sanguigno
phys-field-conditions = Patologie note
phys-field-operations = Operazioni e ferite
phys-field-cause-of-death = Causa della morte
phys-field-religion = Religione o appartenenza
phys-field-health-notes = Note
admin-export-health-note = L’esportazione semplice esclude ogni categoria sensibile — salute e convinzioni, dati biometrici, dati genomici e casellario giudiziale — oltre al profilo comportamentale di chiunque sia in vita, così un file inviato a un parente non ne contiene nessuna. Spunta ciò che un determinato file deve contenere; l’archivio stesso registra quali categorie sono state escluse.
avatar-picker-title = Scegli un'immagine
avatar-choose-link = Scegli l'immagine
avatar-choose = Quale immagine rappresenta questa persona
avatar-mode-auto = Lascia scegliere al programma
avatar-mode-auto-note = Il primo ritratto, altrimenti la prima immagine collegata a questa scheda.
avatar-mode-none = Mostra le iniziali
avatar-mode-none-note = Per una scheda le cui immagini sono documenti e non volti.
avatar-focal-hint = Fai clic su un'immagine per sceglierla, poi di nuovo sulla parte da tenere nell'inquadratura: un avatar è quadrato, la maggior parte delle scansioni no.
avatar-no-images = A questa scheda non è ancora collegata alcuna immagine.
avatar-upload-title = Carica un'immagine e usala
avatar-upload-button = Carica e usa come immagine
avatar-not-available-title = Quell'immagine non è disponibile
avatar-not-available-detail = Il file scelto non è collegato a questa persona, oppure non ti è dato leggerlo.

record-history-withheld = non mostrato a te

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Stato
record-presumed-deceased = decesso presunto
record-presumed-short = presunto
record-presumed-why = Non è registrata alcuna morte e la nascita risale a più di { $years } anni fa, quindi questa scheda non può essere esatta. L'archivio non è modificato: è ciò che la pagina deduce, non ciò che dice la fonte.

## The identity editor

identity-editor-title = Nomi e identità
identity-primary-name = Il nome mostrato ovunque
identity-primary-help = Quello che usano la scheda dell'albero, il titolo e ogni elenco. Gli altri nomi sotto sono quelli con cui una fonte ha chiamato questa persona in un altro momento.
identity-display = Nome
identity-display-latin = In caratteri latini
identity-culture = Lingua
identity-direction = Direzione
identity-direction-ltr = da sinistra a destra
identity-direction-rtl = da destra a sinistra
identity-direction-auto = dal testo
identity-components = Parti del nome
identity-components-help = Quale parte è il nome e quale il cognome, nell'ordine in cui si scrivono. Una scheda senza parti si mostra lo stesso: le parti sono ciò su cui una ricerca può agire.
identity-part = Parte
identity-value = Testo
identity-other-names = Altri nomi
identity-other-help = Un cognome da coniugata, un nome religioso, un nome usato da una scheda più tarda. Ciascuno porta il periodo d'uso e la fonte che lo attesta.
identity-name-type = Tipo di nome
identity-valid-from = In uso dal
identity-valid-until = In uso fino al
identity-about = Sulla persona
identity-living-help = Questo è il contrassegno posto dalla fonte. La pagina presume separatamente un decesso quando la nascita è troppo lontana, e quella presunzione non cambia mai questa casella né l'archivio.
identity-error-no-display = Una scheda ha bisogno di un nome con cui mostrarsi. Non è stato salvato nulla.
editor-blank-to-remove = Svuota il nome per rimuovere questa voce.
# A BCP 47 tag, shown as an example in the language field.
identity-culture-placeholder = it
identity-edit-link = Modifica nomi e identità

## Union types, statuses and date precision, said out loud

union-type-marriage = matrimonio
union-type-civil_union = unione civile
union-type-cohabitation = convivenza
union-type-religious_only = unione religiosa
union-type-polygamous = poligamo
union-type-unknown = non registrato
union-role-spouse = coniuge
union-role-partner = convivente
union-role-husband = marito
union-role-wife = moglie
union-status-active = in corso
union-status-ended_by_death = terminata per decesso
union-status-ended_by_divorce = terminata per divorzio
union-status-ended_by_separation = terminata per separazione
union-status-annulled = annullata
union-status-unknown = non registrato
union-status-ended = terminata
union-status-ended-by = terminata per { $reason }
union-reason-death_of_spouse = il decesso di un coniuge
precision-exact = al giorno
precision-year = all'anno
precision-month = al mese
precision-decade = al decennio
precision-century = al secolo
precision-unknown = non nota
record-precision = Precisione
record-approximate = Approssimativo
record-place = Luogo

## The relationships editor

family-editor-title = Famiglia e relazioni
family-unions = Unioni
family-no-unions = Per questa persona non è registrata alcuna unione.
family-union-legend = Unione { $n }
family-writes-family = Salvare modifica la scheda di famiglia #{ $id }, condivisa dalle due persone. Anche la pagina dell'altra cambia.
family-partners = Partner
family-partner = Partner
family-role = Ruolo
family-children = Figli
family-children-help = L'ordine di nascita è l'affermazione della scheda stessa. Lasciato vuoto non afferma nulla: un numero preso dalla posizione della riga sarebbe un fatto che nessuno ha scritto.
family-child = Figlio
family-birth-order = Ordine di nascita
family-the-union = L'unione stessa
family-type = Tipo di unione
family-status = Stato
family-started = Inizio
family-ended = Fine
family-leave = Togli questa persona da questa unione
family-open-entity = Apri la scheda di famiglia
family-new-union = Una nuova unione
family-new-union-help = Questo crea una nuova scheda di famiglia con questa persona. Il partner è facoltativo: un genitore che la scheda nomina senza alcun partner è un'unione di uno.
family-create-union = Crea l'unione
family-parents = Genitori
family-no-parents = Questa persona non è registrata come figlio di alcuna famiglia.
family-child-of = Figlio di questa famiglia
family-detach-child = Togli questa persona da questa famiglia
family-attach-parents = Collega ai genitori
family-attach-help = Scegli la famiglia di cui questa persona è figlio. La aggiunge a quella scheda di famiglia, quindi compare anche sulle pagine dei genitori.
family-the-family = La famiglia
family-attach = Collega
family-error-last-partner = Un'unione ha bisogno di almeno una persona. Elimina invece la scheda di famiglia, che chiede cosa fare di tutto ciò che vi si riferisce.
family-error-no-family = Non è stata scelta alcuna famiglia. Non è stato salvato nulla.
family-error-already-child = Questa persona è già figlio di quella famiglia.
pick-error-empty = Non è stata indicata alcuna persona. Non è stato salvato nulla.
pick-error-not-found = In questo archivio non c'è nessuna persona con quel nome. Non è stato salvato nulla.
pick-error-ambiguous = Più di una persona corrisponde. Scegline una dall'elenco perché la scheda dica quale. Non è stato salvato nulla.

## Links and occupations

links-editor-title = Collegamenti
links-editor-help = Le relazioni che non sono di famiglia: un padrino, un datore di lavoro, un testimone, un reggimento. Ognuna è una scheda a sé che nomina due persone, quindi modificarla qui cambia anche l'altra scheda.
links-none = Per questa persona non è registrato alcun collegamento.
links-new = Un nuovo collegamento
links-create = Crea il collegamento
links-remove = Rimuovi questo collegamento
links-other-end = L'altro capo
links-label = Che cos'è
links-label-reverse = Nell'altro verso
links-category = Categoria
links-bidirectional = Si legge uguale in entrambi i versi
links-from = Da
links-until = Fino a
links-reversed = Questo collegamento è stato creato dall'altra scheda. Modificarlo qui cambia la stessa entità.
link-error-no-label = Un collegamento deve dire che cos'è. Non è stato salvato nulla.
occupations-editor-title = Occupazioni
occupations-editor-help = Un'occupazione è un periodo con un inizio e una fine, non un titolo di lavoro. Ciascuna porta le proprie date e la propria fonte.
occupations-none = Per questa persona non è registrata alcuna occupazione.
occupations-new = Una nuova occupazione
occupations-create = Crea l'occupazione
occupations-remove = Rimuovi questa occupazione
occupations-title = Che cosa faceva
occupations-employer = Per chi
occupations-employer-place = Dove stavano
occupations-from = Da
occupations-until = Fino a
occupation-error-no-title = Un'occupazione deve dire che cosa faceva qualcuno. Non è stato salvato nulla.
link-category-spiritual = spirituale
link-category-professional = professionale
link-category-social = sociale
link-category-legal = legale
link-category-medical = medica
link-category-educational = educativa
link-category-conflict = conflitto
link-category-other = altro
links-edit-link = Modifica i collegamenti
occupations-edit-link = Modifica le occupazioni
family-edit-link = Modifica famiglia e relazioni

## Events and documents

events-editor-title = Eventi
events-editor-help = Un evento nomina più persone insieme — un matrimonio, un battesimo, un censimento — quindi ciascuno è una scheda a sé e compare su ogni pagina che nomina.
events-none = Nessun evento nomina questa persona.
events-new = Un nuovo evento
events-new-help = Questa persona vi è aggiunta come soggetto se non ne nomini altre. Un evento senza nessuno è solo una data.
events-create = Crea l'evento
events-remove = Rimuovi questo evento
events-category = Che cosa è successo
events-subcategory = Più precisamente
events-description = Descrizione
events-participants = Chi c'era
events-participants-help = Salvare modifica la scheda dell'evento, che mostra anche ogni altra persona nominata.
events-who = Chi
event-error-no-category = Un evento deve dire che cosa è successo. Non è stato salvato nulla.
documents-editor-title = Documenti
documents-editor-help = A quali file punta questa scheda e che cosa è ciascuno per essa. Svuotare una riga stacca il file: il documento e i suoi byte restano nell'archivio.
documents-attached = Allegati a questa scheda
documents-upload = Carica un file
documents-upload-help = Fino a { $mb } MB. Il file è riposto nell'archivio e allegato a questa scheda.
documents-caption = Didascalia
documents-edit-link = Allega e stacca documenti
events-edit-link = Modifica gli eventi

## Presentation styles: density, never colour

prefs-style = Densità
prefs-style-help = Quanto spazio prende la pagina. Separata dal tema, che riguarda solo il colore, quindi ogni combinazione è possibile.
style-comfortable = Comoda
style-comfortable-note = predefinita, con spazio per leggere
style-compact = Compatta
style-compact-note = più scheda per schermo, per scorrerne diverse
style-paper = Carta
style-paper-note = un carattere con grazie e filetti invece di schede, per leggere con calma o stampare

## Sensitive classes

admin-export-choose = Includi in questa esportazione
scope-health = Salute e convinzioni
scope-biometrics = Dati biometrici
scope-genomics = Dati genomici
scope-legal = Casellario giudiziale
scope-behaviour = Profili comportamentali delle persone viventi
admin-export-with-chosen = Esporta con le voci spuntate

## Profile

pg-identity = Identità e stato civile
pg-identity-intro = Chi era la persona secondo i documenti e che cosa hanno annotato i registri dello stato civile.
pg-morphology = Morfologia
pg-morphology-intro = Il corpo come è stato misurato e descritto.
pg-biometrics = Biometria
pg-biometrics-intro = La voce, le mani e i sensi, e i modelli che permettono di riconoscere una persona.
pg-health = Salute
pg-health-intro = Malattie, cure, misurazioni e referti.
pg-genomics = Genomica
pg-genomics-intro = Test del DNA, aplogruppi, varianti e altri risultati molecolari.
pg-death = Morte
pg-death-intro = Come, quando e dove si è conclusa una vita, e che cosa è stato del corpo.
pg-residence = Residenza e cittadinanza
pg-residence-intro = Dove la persona ha vissuto, quali Stati la consideravano loro cittadina e quali lingue parlava.
pg-education = Istruzione e lavoro
pg-education-intro = Studi, titoli, redditi e beni.
pg-military = Servizio militare e onorificenze
pg-military-intro = Servizio, gradi, reparti e onorificenze.
pg-legal = Procedimenti penali
pg-legal-intro = Procedimenti penali e loro esito.
pg-belief = Fede e appartenenze
pg-belief-intro = Religione, riti, convinzioni e iscrizioni.
pg-personality = Personalità e comportamento
pg-personality-intro = Temperamento, abitudini e passatempi, come li descrivono le fonti.
pg-relationships = Relazioni
pg-relationships-intro = Genitori, coniugi, figli e le altre persone di una vita.
pg-digital-legacy = Eredità digitale
pg-digital-legacy-intro = Scansioni, modelli, registrazioni e archivi che rappresentano una persona.
pa-identity-titles = Titoli
pa-identity-sex-at-birth = Sesso alla nascita
pa-identity-gender-identity = Identità di genere
pa-birth-time = Ora di nascita
pa-birth-coordinates = Luogo di nascita in coordinate
pa-civil-status-birth-certificate-number = Numero dell’atto di nascita
pa-civil-status-register-entries = Atti di stato civile
pa-civil-status-marginal-annotations = Annotazioni a margine
pa-morphology-height = Statura
pa-morphology-weight = Peso
pa-morphology-bmi = Indice di massa corporea
pa-morphology-body-composition = Composizione corporea
pa-morphology-build = Corporatura
pa-morphology-eye-colour = Colore degli occhi
pa-morphology-eye-shape = Forma degli occhi
pa-morphology-eye-spacing = Distanza tra gli occhi
pa-morphology-hair-colour = Colore naturale dei capelli
pa-morphology-hair-texture = Tipo di capelli
pa-morphology-hairline = Attaccatura dei capelli
pa-morphology-facial-hair = Barba e baffi
pa-morphology-body-hair = Peluria corporea
pa-morphology-skin-tone = Fototipo (Fitzpatrick)
pa-morphology-skin-undertone = Sottotono della pelle
pa-morphology-freckles = Lentiggini
pa-morphology-pigmentation = Macchie della pelle
pa-morphology-scars = Cicatrici
pa-morphology-tattoos = Tatuaggi
pa-morphology-moles = Nei
pa-morphology-facial-asymmetries = Asimmetrie del viso
pa-morphology-face-shape = Forma del viso
pa-morphology-nose-shape = Forma del naso
pa-morphology-ear-shape = Forma delle orecchie
pa-morphology-lip-shape = Forma delle labbra
pa-morphology-dentition = Dentatura
pa-morphology-malocclusion = Malocclusione (classe di Angle)
pa-morphology-posture = Postura
pa-morphology-gait = Andatura
pa-morphology-distinguishing-features = Segni particolari
pa-biometrics-fingerprints = Impronte digitali
pa-biometrics-retinal-print = Impronta retinica
pa-biometrics-voice-signature = Impronta vocale
pa-biometrics-voice-frequency = Frequenza fondamentale della voce
pa-biometrics-vocal-timbre = Timbro della voce
pa-biometrics-spoken-accent = Accento
pa-biometrics-speech-rate = Velocità di eloquio
pa-biometrics-verbal-tics = Intercalari
pa-biometrics-frequent-vocabulary = Lessico ricorrente
pa-biometrics-speech-register = Registro linguistico
pa-biometrics-motor-tics = Tic motori
pa-biometrics-handedness = Lateralità manuale
pa-biometrics-hearing = Udito
pa-biometrics-visual-acuity = Acuità visiva
pa-biometrics-optical-correction = Correzione ottica
pa-health-blood-group = Gruppo sanguigno (AB0)
pa-health-rhesus = Fattore Rh (RhD)
pa-health-blood-pressure = Pressione arteriosa
pa-health-resting-heart-rate = Frequenza cardiaca a riposo
pa-health-respiratory-capacity = Funzionalità respiratoria
pa-health-conditions = Patologie
pa-health-surgeries = Interventi chirurgici
pa-health-injuries = Lesioni
pa-health-deformities = Deformità
pa-health-amputations = Amputazioni
pa-health-prostheses = Protesi
pa-health-implants = Impianti
pa-health-devices = Dispositivi impiantati
pa-health-medications = Farmaci
pa-health-allergies = Allergie
pa-health-vaccinations = Vaccinazioni
pa-health-serology = Sierologia
pa-health-lab-results = Esami di laboratorio
pa-health-deficiencies = Carenze
pa-health-sleep-disorders = Disturbi del sonno
pa-health-mental-health-assessments = Valutazioni della salute mentale
pa-genomics-autosomal-mapping = Test del DNA autosomico
pa-genomics-y-haplogroup = Aplogruppo del cromosoma Y
pa-genomics-mt-haplogroup = Aplogruppo mitocondriale
pa-genomics-whole-genome-sequencing = Sequenziamento dell’intero genoma
pa-genomics-risk-variants = Varianti di rischio
pa-genomics-hereditary-conditions = Malattie ereditarie
pa-genomics-predispositions = Predisposizioni
pa-genomics-epigenetic-markers = Marcatori epigenetici
pa-genomics-epigenetic-age = Età epigenetica
pa-genomics-gut-microbiome = Microbioma intestinale
pa-genomics-skin-microbiome = Microbioma cutaneo
pa-genomics-toxicological-sensitivities = Sensibilità a farmaci e sostanze tossiche
pa-death-time = Ora del decesso
pa-death-coordinates = Luogo del decesso in coordinate
pa-death-causes = Cause di morte
pa-death-contributing-factors = Fattori concomitanti
pa-death-autopsy = Autopsia
pa-death-disposition = Destinazione della salma
pa-death-grave = Sepoltura
pa-residence-addresses = Indirizzi
pa-residence-nationality-of-origin = Cittadinanza d’origine
pa-residence-acquired-nationalities = Cittadinanze acquisite
pa-residence-mother-tongue = Lingua madre
pa-residence-spoken-languages = Lingue parlate
pa-education-level = Livello di istruzione
pa-education-diplomas = Diplomi e titoli di studio
pa-education-institutions = Scuole e istituti
pa-education-income = Reddito
pa-education-real-estate = Beni immobili
pa-military-distinctions = Onorificenze
pa-military-citations = Encomi
pa-military-ranks = Gradi
pa-military-units = Reparti
pa-military-service-numbers = Numeri di matricola
pa-legal-criminal-record = Casellario giudiziale
pa-belief-religions = Religione
pa-belief-sacraments = Sacramenti e riti
pa-belief-beliefs = Convinzioni
pa-belief-political-leanings = Orientamento politico
pa-belief-memberships = Iscrizioni
pa-personality-big-five = Punteggi Big Five
pa-personality-mbti = Tipo MBTI
pa-personality-introversion-extraversion = Introversione ed estroversione
pa-personality-stress-tolerance = Tolleranza allo stress
pa-personality-decision-style = Stile decisionale
pa-personality-interests = Interessi
pa-personality-hobbies = Hobby
pa-personality-sports = Sport
pa-personality-dietary-habits = Alimentazione
pa-personality-dependencies = Dipendenze
pa-digital-legacy-body-models = Modelli del corpo
pa-digital-legacy-skin-textures = Texture della pelle
pa-digital-legacy-rigs = Rig scheletrici
pa-digital-legacy-voice-corpora = Registrazioni per la sintesi vocale
pa-digital-legacy-text-corpora = Scritti per un modello linguistico
pa-digital-legacy-digital-traces = Tracce digitali
pa-digital-legacy-carbon-footprint = Impronta di carbonio
pa-digital-legacy-behaviour-models = Modelli di comportamento
pf-identity-titles-text = Titolo come scritto
pf-identity-titles-kind = Tipo di titolo
pf-civil-status-marginal-annotations-text = Annotazione
pf-morphology-pigmentation-kind = Tipo di macchia
pf-biometrics-spoken-accent-description = Come viene descritto
pf-biometrics-optical-correction-kind = Correzione
pf-health-amputations-level = Livello dell’amputazione
pf-health-prostheses-kind = Protesi
pf-health-implants-kind = Impianto
pf-health-devices-kind = Dispositivo
pf-health-allergies-type = Tipo di allergia
pf-health-vaccinations-status = Stato vaccinale
pf-health-sleep-disorders-category = Categoria del disturbo
pf-death-autopsy-kind = Autopsia
pf-education-institutions-name = Nome dell’istituto
pf-military-distinctions-name = Nome dell’onorificenza
pf-military-distinctions-kind = Tipo di onorificenza
pf-military-citations-text = Testo dell’encomio
pf-military-ranks-category = Categoria del grado
pf-belief-political-leanings-position = Posizione sull’asse sinistra–destra
pf-belief-memberships-kind = Tipo di organizzazione
pf-digital-legacy-carbon-footprint-method = Metodo di stima
pf-age-years = Età in anni
pf-agreeableness = Gradevolezza
pf-allergen = Allergene
pf-amount = Importo
pf-analyte = Analita
pf-artefact-type = Tipo di artefatto
pf-autoimmune = Autoimmune
pf-body-region = Parte del corpo
pf-bone-percent = Ossa
pf-carrier-status = Stato di portatore
pf-cause = Causa
pf-chronic = Cronica
pf-clock = Orologio
pf-condition = Patologia
pf-conferred-by = Conferita da
pf-congenital = Congenita
pf-conscientiousness = Coscienziosità
pf-consent = Consenso
pf-coordinates = Coordinate
pf-corrected = Con correzione
pf-country = Paese
pf-court = Tribunale
pf-coverage = Copertura
pf-currency = Valuta
pf-decimal = Acuità (decimale)
pf-denomination = Confessione
pf-derived-from-id = Derivato da
pf-description = Descrizione
pf-details = Dettagli
pf-diagnosis = Diagnosi
pf-diameter-mm = Diametro
pf-diastolic = Diastolica
pf-diet = Dieta
pf-document-id = Documento
pf-dose = Dose
pf-ear = Orecchio
pf-entry-number = Numero dell’atto
pf-extraversion = Estroversione
pf-eye = Occhio
pf-fat-percent = Grasso
pf-fev1-fvc-ratio = Rapporto FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = Formato del file
pf-findings = Referto
pf-flag = Segnalazione
pf-format = Formato
pf-fracture = Frattura
pf-fvc-litres = FVC
pf-gene = Gene
pf-generator = Realizzato con
pf-grade = Grado
pf-iccs-section = Sezione del reato (ICCS)
pf-icd10-chapter = Capitolo ICD-10
pf-indication = Indicazione
pf-inheritance = Modalità di trasmissione
pf-inscription = Iscrizione
pf-institution = Istituto
pf-instrument = Strumento
pf-isced-level = Livello ISCED
pf-jurisdiction = Giurisdizione
pf-language = Lingua
pf-lat = Latitudine
pf-level = Livello
pf-lines = Indirizzo
pf-location = Posizione
pf-lon = Longitudine
pf-major = Aplogruppo principale
pf-marker = Marcatore
pf-metaboliser-status = Fenotipo metabolizzatore
pf-method = Metodo
pf-mode = Modalità di acquisto
pf-muscle-percent = Muscoli
pf-neuroticism = Nevroticismo
pf-number = Numero
pf-nutrient = Nutriente
pf-offence = Reato
pf-office = Ufficio
pf-openness = Apertura mentale
pf-organisation = Organizzazione
pf-outcome = Esito
pf-pace = Ritmo di invecchiamento
pf-page = Pagina
pf-panel = Profilo
pf-party = Partito
pf-pathogen = Agente patogeno
pf-pattern = Modalità d’uso
pf-percentile = Percentile
pf-period = Periodo di retribuzione
pf-place-id = Luogo
pf-plot = Settore
pf-polygenic-score = Punteggio poligenico
pf-postal-code = Codice postale
pf-precision = Precisione
pf-prescription = Prescrizione
pf-proficiency = Livello di competenza
pf-provider = Fornitore
pf-quintile = Quintile di reddito
pf-rank = Grado
pf-rank-text = Grado come scritto
pf-reaction = Reazione
pf-reference-build = Genoma di riferimento
pf-reference-high = Limite superiore di riferimento
pf-reference-low = Limite inferiore di riferimento
pf-register-type = Tipo di atto
pf-result = Risultato
pf-role = Ruolo
pf-sacrament = Sacramento o rito
pf-score = Punteggio
pf-sentence = Pena
pf-sequence = Posto nella catena causale
pf-service = Forza armata
pf-severity = Gravità
pf-shannon-diversity = Diversità di Shannon
pf-shape = Forma
pf-significance = Significato clinico
pf-snp-count = SNP analizzati
pf-sport = Sport
pf-subclade = Sottoclade
pf-substance = Sostanza
pf-summary = Sintesi
pf-systolic = Sistolica
pf-tenure = Titolo di possesso
pf-test = Test
pf-threshold-db = Soglia uditiva
pf-title = Titolo
pf-tonnes-co2e-per-year = Emissioni
pf-tradition = Tradizione
pf-tree-version = Versione dell’albero
pf-unit = Unità
pf-use = Uso
pf-variant = Variante
pf-volume = Volume
pf-zygosity = Zigosità
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n }%
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = { $n } parole/min
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = { $n } bpm
pu-litres = { $n } l
pu-coverage = { $n }×
pu-years = { $n } anni
pu-t-co2e-yr = { $n } t CO₂e all’anno
pv-sensitive-class-health = Salute e convinzioni
pv-sensitive-class-biometrics = Dati biometrici
pv-sensitive-class-genomics = Dati genomici
pv-sensitive-class-legal = Casellario giudiziale
pv-laterality-left = Sinistro
pv-laterality-right = Destro
pv-laterality-both = Entrambi
pv-body-region-head = Testa
pv-body-region-face = Viso
pv-body-region-neck = Collo
pv-body-region-left-shoulder = Spalla sinistra
pv-body-region-right-shoulder = Spalla destra
pv-body-region-left-arm = Braccio sinistro
pv-body-region-right-arm = Braccio destro
pv-body-region-left-hand = Mano sinistra
pv-body-region-right-hand = Mano destra
pv-body-region-chest = Torace
pv-body-region-abdomen = Addome
pv-body-region-upper-back = Parte alta della schiena
pv-body-region-lower-back = Parte bassa della schiena
pv-body-region-pelvis = Bacino e anche
pv-body-region-left-leg = Gamba sinistra
pv-body-region-right-leg = Gamba destra
pv-body-region-left-foot = Piede sinistro
pv-body-region-right-foot = Piede destro
pv-body-region-internal = Interna
pv-body-region-whole-body = Tutto il corpo
pv-body-region-other = Altra parte
pv-artefact-type-mesh = Mesh
pv-artefact-type-point-cloud = Nuvola di punti
pv-artefact-type-skin-texture-map = Mappa della texture cutanea
pv-artefact-type-skeletal-rig = Rig scheletrico
pv-artefact-type-voice-corpus = Corpus vocale
pv-artefact-type-text-corpus = Corpus di testi
pv-artefact-type-trace-archive = Archivio dell’attività online
pv-artefact-type-behaviour-model = Modello di comportamento
pv-artefact-type-fingerprint-card = Cartellino dattiloscopico
pv-artefact-type-fingerprint-template = Modello di impronta
pv-artefact-type-retinal-image = Immagine retinica
pv-artefact-type-voiceprint = Impronta vocale
pv-consent-given = Dato
pv-consent-given-by-estate = Dato dagli eredi
pv-consent-refused = Negato
pv-consent-withdrawn = Revocato
pv-consent-not-asked = Non richiesto
pv-consent-unknown = Non noto
pv-sex-at-birth-female = Femminile
pv-sex-at-birth-male = Maschile
pv-sex-at-birth-intersex = Intersessuale
pv-sex-at-birth-undetermined = Indeterminato
pv-sex-at-birth-unknown = Non noto
pv-gender-identity-woman = Donna
pv-gender-identity-man = Uomo
pv-gender-identity-non-binary = Non binario
pv-gender-identity-other = Altro
pv-gender-identity-undisclosed = Non dichiarata
pv-gender-identity-unknown = Non nota
pv-title-kind-nobility = Nobiliare
pv-title-kind-academic = Accademico
pv-title-kind-professional = Professionale
pv-title-kind-religious = Religioso
pv-title-kind-military = Militare
pv-title-kind-civic = Onorifico
pv-title-kind-courtesy = Di cortesia
pv-title-kind-other = Altro
pv-register-type-birth = Nascita
pv-register-type-baptism = Battesimo
pv-register-type-marriage = Matrimonio
pv-register-type-death = Morte
pv-register-type-burial = Sepoltura
pv-register-type-divorce = Divorzio
pv-register-type-recognition = Riconoscimento di figlio
pv-register-type-legitimation = Legittimazione
pv-register-type-adoption = Adozione
pv-register-type-name-change = Cambio di nome
pv-register-type-other = Altro
pv-build-slight = Minuta
pv-build-slim = Snella
pv-build-average = Media
pv-build-sturdy = Robusta
pv-build-stout = Tarchiata
pv-build-heavy = Pesante
pv-eye-colour-light-blue = Azzurro chiaro
pv-eye-colour-blue = Azzurro
pv-eye-colour-dark-blue = Blu scuro
pv-eye-colour-grey = Grigio
pv-eye-colour-blue-grey = Grigio-azzurro
pv-eye-colour-green = Verde
pv-eye-colour-grey-green = Grigio-verde
pv-eye-colour-hazel = Nocciola
pv-eye-colour-amber = Ambra
pv-eye-colour-light-brown = Castano chiaro
pv-eye-colour-brown = Castano
pv-eye-colour-dark-brown = Castano scuro
pv-eye-colour-black = Nero
pv-eye-colour-mixed = Misto
pv-eye-colour-other = Altro
pv-eye-shape-almond = A mandorla
pv-eye-shape-round = Rotondi
pv-eye-shape-hooded = Palpebra cadente
pv-eye-shape-monolid = Senza piega palpebrale
pv-eye-shape-deep-set = Infossati
pv-eye-shape-protruding = Sporgenti
pv-eye-shape-upturned = All’insù
pv-eye-shape-downturned = All’ingiù
pv-eye-shape-other = Altro
pv-eye-spacing-close-set = Ravvicinati
pv-eye-spacing-average = Nella media
pv-eye-spacing-wide-set = Distanziati
pv-hair-colour-black = Neri
pv-hair-colour-dark-brown = Castano scuro
pv-hair-colour-brown = Castani
pv-hair-colour-light-brown = Castano chiaro
pv-hair-colour-auburn = Ramati
pv-hair-colour-red = Rossi
pv-hair-colour-strawberry-blond = Biondo fragola
pv-hair-colour-dark-blond = Biondo scuro
pv-hair-colour-blond = Biondi
pv-hair-colour-light-blond = Biondo chiaro
pv-hair-colour-grey = Grigi
pv-hair-colour-white = Bianchi
pv-hair-colour-none = Senza capelli
pv-hair-colour-other = Altro
pv-hair-texture-straight = Lisci
pv-hair-texture-wavy = Mossi
pv-hair-texture-curly = Ricci
pv-hair-texture-coily = Crespi
pv-hair-texture-other = Altro
pv-hairline-straight = Diritta
pv-hairline-rounded = Arrotondata
pv-hairline-widows-peak = A punta
pv-hairline-m-shaped = A M
pv-hairline-bell-shaped = A campana
pv-hairline-uneven = Irregolare
pv-hairline-receding = Stempiata
pv-hairline-bald = Calvizie
pv-facial-hair-none = Nessuna
pv-facial-hair-stubble = Barba incolta
pv-facial-hair-moustache = Baffi
pv-facial-hair-goatee = Pizzetto
pv-facial-hair-full-beard = Barba folta
pv-facial-hair-sideburns = Basette
pv-facial-hair-other = Altro
pv-body-hair-none = Assente
pv-body-hair-sparse = Rada
pv-body-hair-moderate = Moderata
pv-body-hair-dense = Folta
pv-skin-tone-type-i = Tipo I — si scotta sempre, non si abbronza mai
pv-skin-tone-type-ii = Tipo II — si scotta spesso, si abbronza poco
pv-skin-tone-type-iii = Tipo III — a volte si scotta, si abbronza uniformemente
pv-skin-tone-type-iv = Tipo IV — si scotta di rado, si abbronza bene
pv-skin-tone-type-v = Tipo V — si scotta molto di rado
pv-skin-tone-type-vi = Tipo VI — non si scotta mai
pv-skin-undertone-cool = Freddo
pv-skin-undertone-neutral = Neutro
pv-skin-undertone-warm = Caldo
pv-skin-undertone-olive = Olivastro
pv-freckles-none = Nessuna
pv-freckles-few = Poche
pv-freckles-moderate = Moderate
pv-freckles-many = Molte
pv-pigmentation-mark-birthmark = Voglia
pv-pigmentation-mark-port-wine-stain = Angioma piano
pv-pigmentation-mark-cafe-au-lait-spot = Macchia caffellatte
pv-pigmentation-mark-depigmented-patch = Macchia depigmentata
pv-pigmentation-mark-hyperpigmented-patch = Macchia iperpigmentata
pv-pigmentation-mark-other = Altro
pv-mole-shape-round = Rotondo
pv-mole-shape-oval = Ovale
pv-mole-shape-irregular = Irregolare
pv-mole-shape-other = Altro
pv-face-shape-oval = Ovale
pv-face-shape-round = Rotondo
pv-face-shape-square = Squadrato
pv-face-shape-oblong = Allungato
pv-face-shape-heart = A cuore
pv-face-shape-diamond = A diamante
pv-face-shape-triangular = Triangolare
pv-nose-shape-straight = Dritto
pv-nose-shape-aquiline = Aquilino
pv-nose-shape-snub = Camuso
pv-nose-shape-upturned = All’insù
pv-nose-shape-flat = Schiacciato
pv-nose-shape-broad = Largo
pv-nose-shape-bulbous = Bulboso
pv-nose-shape-crooked = Storto
pv-nose-shape-other = Altro
pv-ear-shape-free-lobe = Lobi staccati
pv-ear-shape-attached-lobe = Lobi attaccati
pv-ear-shape-protruding = Sporgenti
pv-ear-shape-close-set = Aderenti
pv-ear-shape-pointed = A punta
pv-ear-shape-other = Altro
pv-lip-shape-thin = Sottili
pv-lip-shape-medium = Medie
pv-lip-shape-full = Carnose
pv-lip-shape-bow-shaped = Ad arco
pv-lip-shape-wide = Larghe
pv-lip-shape-downturned = Rivolte in giù
pv-lip-shape-other = Altro
pv-dentition-primary = Decidua
pv-dentition-mixed = Mista
pv-dentition-permanent-complete = Permanente, completa
pv-dentition-permanent-partial-loss = Permanente, incompleta
pv-dentition-edentulous = Edentula
pv-dentition-partial-denture = Protesi parziale
pv-dentition-full-denture = Protesi totale
pv-dentition-implants = Impianti dentali
pv-malocclusion-normal = Occlusione normale
pv-malocclusion-class-i = Classe I
pv-malocclusion-class-ii-division-1 = Classe II, divisione 1
pv-malocclusion-class-ii-division-2 = Classe II, divisione 2
pv-malocclusion-class-iii = Classe III
pv-posture-ideal = Corretta
pv-posture-kyphotic-lordotic = Cifo-lordotica
pv-posture-flat-back = Schiena piatta
pv-posture-sway-back = Schiena insellata
pv-posture-stooped = Curva
pv-posture-scoliotic = Scoliotica
pv-posture-other = Altro
pv-gait-brisk = Svelta
pv-gait-average = Normale
pv-gait-slow = Lenta
pv-gait-shuffling = Strascicata
pv-gait-limping = Zoppicante
pv-gait-waddling = Ondeggiante
pv-gait-unsteady = Instabile
pv-gait-stiff = Rigida
pv-gait-other = Altro
pv-vocal-timbre-bright = Chiaro
pv-vocal-timbre-dark = Scuro
pv-vocal-timbre-warm = Caldo
pv-vocal-timbre-breathy = Soffiato
pv-vocal-timbre-nasal = Nasale
pv-vocal-timbre-hoarse = Rauco
pv-vocal-timbre-resonant = Sonoro
pv-vocal-timbre-thin = Esile
pv-vocal-timbre-other = Altro
pv-speech-register-frozen = Aulico
pv-speech-register-formal = Formale
pv-speech-register-consultative = Neutro
pv-speech-register-casual = Colloquiale
pv-speech-register-intimate = Intimo
pv-handedness-left = Mancino
pv-handedness-right = Destrimane
pv-handedness-ambidextrous = Ambidestro
pv-handedness-mixed = Mista
pv-handedness-unknown = Non nota
pv-hearing-grade-normal = Normale
pv-hearing-grade-mild = Lieve
pv-hearing-grade-moderate = Moderata
pv-hearing-grade-moderately-severe = Moderatamente grave
pv-hearing-grade-severe = Grave
pv-hearing-grade-profound = Profonda
pv-hearing-grade-complete = Totale
pv-optical-correction-none = Nessuna
pv-optical-correction-glasses = Occhiali
pv-optical-correction-contact-lenses = Lenti a contatto
pv-optical-correction-glasses-and-contact-lenses = Occhiali e lenti a contatto
pv-optical-correction-refractive-surgery = Chirurgia refrattiva
pv-optical-correction-intraocular-lens = Lente intraoculare
pv-optical-correction-other = Altro
pv-rhesus-positive = RhD positivo
pv-rhesus-negative = RhD negativo
pv-rhesus-weak-d = D debole
pv-rhesus-unknown = Non noto
pv-icd10-chapter-infectious-parasitic = I Malattie infettive e parassitarie
pv-icd10-chapter-neoplasms = II Tumori
pv-icd10-chapter-blood-immune = III Sangue e sistema immunitario
pv-icd10-chapter-endocrine-metabolic = IV Malattie endocrine, nutrizionali e metaboliche
pv-icd10-chapter-mental-behavioural = V Disturbi psichici e comportamentali
pv-icd10-chapter-nervous-system = VI Sistema nervoso
pv-icd10-chapter-eye-adnexa = VII Occhio e annessi
pv-icd10-chapter-ear-mastoid = VIII Orecchio e apofisi mastoide
pv-icd10-chapter-circulatory = IX Sistema circolatorio
pv-icd10-chapter-respiratory = X Sistema respiratorio
pv-icd10-chapter-digestive = XI Apparato digerente
pv-icd10-chapter-skin = XII Cute e tessuto sottocutaneo
pv-icd10-chapter-musculoskeletal = XIII Sistema osteomuscolare
pv-icd10-chapter-genitourinary = XIV Apparato genitourinario
pv-icd10-chapter-pregnancy-childbirth = XV Gravidanza e parto
pv-icd10-chapter-perinatal = XVI Condizioni perinatali
pv-icd10-chapter-congenital = XVII Malformazioni congenite
pv-icd10-chapter-ill-defined = XVIII Sintomi e cause mal definite
pv-icd10-chapter-injury-poisoning = XIX Traumatismi e avvelenamenti
pv-icd10-chapter-external-causes = XX Cause esterne
pv-icd10-chapter-health-factors = XXI Fattori che influenzano la salute
pv-icd10-chapter-special-purposes = XXII Codici per scopi speciali
pv-diagnosis-status-diagnosed = Diagnosticata
pv-diagnosis-status-suspected = Sospetta
pv-diagnosis-status-self-reported = Riferita
pv-diagnosis-status-unknown = Non noto
pv-prosthesis-kind-limb = Arto
pv-prosthesis-kind-joint = Articolare
pv-prosthesis-kind-ocular = Oculare
pv-prosthesis-kind-dental = Dentale
pv-prosthesis-kind-auditory = Acustica
pv-prosthesis-kind-breast = Mammaria
pv-prosthesis-kind-other = Altro
pv-implant-kind-orthopaedic = Ortopedico
pv-implant-kind-dental = Dentale
pv-implant-kind-cochlear = Cocleare
pv-implant-kind-breast = Mammario
pv-implant-kind-intraocular-lens = Lente intraoculare
pv-implant-kind-contraceptive = Contraccettivo
pv-implant-kind-cosmetic = Estetico
pv-implant-kind-other = Altro
pv-device-kind-pacemaker = Pacemaker
pv-device-kind-implantable-defibrillator = Defibrillatore impiantabile
pv-device-kind-cardiac-resynchronisation = Dispositivo di resincronizzazione
pv-device-kind-ventricular-assist = Assistenza ventricolare
pv-device-kind-neurostimulator = Neurostimolatore
pv-device-kind-insulin-pump = Microinfusore di insulina
pv-device-kind-drug-port = Port vascolare
pv-device-kind-shunt = Shunt
pv-device-kind-stent = Stent
pv-device-kind-other = Altro
pv-allergy-type-drug = Farmaco
pv-allergy-type-food = Alimento
pv-allergy-type-environmental = Ambientale
pv-allergy-type-insect-venom = Veleno di insetti
pv-allergy-type-latex = Lattice
pv-allergy-type-other = Altro
pv-allergy-severity-mild = Lieve
pv-allergy-severity-moderate = Moderata
pv-allergy-severity-severe = Grave
pv-allergy-severity-anaphylactic = Anafilattica
pv-allergy-severity-unknown = Non nota
pv-pathogen-diphtheria = Difterite
pv-pathogen-tetanus = Tetano
pv-pathogen-pertussis = Pertosse
pv-pathogen-poliomyelitis = Poliomielite
pv-pathogen-measles = Morbillo
pv-pathogen-mumps = Parotite
pv-pathogen-rubella = Rosolia
pv-pathogen-varicella = Varicella
pv-pathogen-smallpox = Vaiolo
pv-pathogen-tuberculosis = Tubercolosi
pv-pathogen-hepatitis-a = Epatite A
pv-pathogen-hepatitis-b = Epatite B
pv-pathogen-hepatitis-c = Epatite C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae di tipo b
pv-pathogen-pneumococcal = Pneumococco
pv-pathogen-meningococcal = Meningococco
pv-pathogen-human-papillomavirus = Papillomavirus umano
pv-pathogen-influenza = Influenza
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotavirus
pv-pathogen-yellow-fever = Febbre gialla
pv-pathogen-typhoid = Febbre tifoide
pv-pathogen-cholera = Colera
pv-pathogen-rabies = Rabbia
pv-pathogen-japanese-encephalitis = Encefalite giapponese
pv-pathogen-tick-borne-encephalitis = Encefalite da zecche
pv-pathogen-hiv = HIV
pv-pathogen-syphilis = Sifilide
pv-pathogen-toxoplasmosis = Toxoplasmosi
pv-pathogen-cytomegalovirus = Citomegalovirus
pv-pathogen-epstein-barr = Virus di Epstein-Barr
pv-pathogen-other = Altro
pv-vaccination-status-vaccinated = Vaccinato
pv-vaccination-status-partially-vaccinated = Parzialmente vaccinato
pv-vaccination-status-unvaccinated = Non vaccinato
pv-vaccination-status-contraindicated = Controindicato
pv-vaccination-status-unknown = Non noto
pv-serology-result-positive = Positivo
pv-serology-result-negative = Negativo
pv-serology-result-equivocal = Dubbio
pv-serology-result-unknown = Non noto
pv-lab-panel-basic-metabolic = Profilo metabolico di base
pv-lab-panel-lipid = Profilo lipidico
pv-lab-panel-liver = Funzionalità epatica
pv-lab-panel-renal = Funzionalità renale
pv-lab-panel-glycated-haemoglobin = Emoglobina glicata
pv-lab-panel-iron = Metabolismo del ferro
pv-lab-analyte-sodium = Sodio
pv-lab-analyte-potassium = Potassio
pv-lab-analyte-chloride = Cloro
pv-lab-analyte-bicarbonate = Bicarbonati
pv-lab-analyte-urea = Urea
pv-lab-analyte-creatinine = Creatinina
pv-lab-analyte-glucose = Glucosio
pv-lab-analyte-calcium = Calcio
pv-lab-analyte-total-cholesterol = Colesterolo totale
pv-lab-analyte-ldl-cholesterol = Colesterolo LDL
pv-lab-analyte-hdl-cholesterol = Colesterolo HDL
pv-lab-analyte-triglycerides = Trigliceridi
pv-lab-analyte-non-hdl-cholesterol = Colesterolo non-HDL
pv-lab-analyte-alt = Alanina aminotransferasi (ALT)
pv-lab-analyte-ast = Aspartato aminotransferasi (AST)
pv-lab-analyte-alp = Fosfatasi alcalina (ALP)
pv-lab-analyte-ggt = Gamma-glutamiltransferasi (GGT)
pv-lab-analyte-total-bilirubin = Bilirubina totale
pv-lab-analyte-direct-bilirubin = Bilirubina diretta
pv-lab-analyte-albumin = Albumina
pv-lab-analyte-total-protein = Proteine totali
pv-lab-analyte-egfr = eGFR stimato
pv-lab-analyte-uric-acid = Acido urico
pv-lab-analyte-phosphate = Fosfato
pv-lab-analyte-urine-albumin-creatinine-ratio = Rapporto albumina/creatinina urinario
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Sideremia
pv-lab-analyte-ferritin = Ferritina
pv-lab-analyte-transferrin = Transferrina
pv-lab-analyte-transferrin-saturation = Saturazione della transferrina
pv-lab-analyte-tibc = Capacità totale di legare il ferro
pv-lab-flag-low = Basso
pv-lab-flag-normal = Normale
pv-lab-flag-high = Alto
pv-lab-flag-critical-low = Criticamente basso
pv-lab-flag-critical-high = Criticamente alto
pv-nutrient-vitamin-a = Vitamina A
pv-nutrient-thiamine = Tiamina (B1)
pv-nutrient-riboflavin = Riboflavina (B2)
pv-nutrient-niacin = Niacina (B3)
pv-nutrient-vitamin-b6 = Vitamina B6
pv-nutrient-folate = Folati (B9)
pv-nutrient-vitamin-b12 = Vitamina B12
pv-nutrient-vitamin-c = Vitamina C
pv-nutrient-vitamin-d = Vitamina D
pv-nutrient-vitamin-e = Vitamina E
pv-nutrient-vitamin-k = Vitamina K
pv-nutrient-iron = Ferro
pv-nutrient-zinc = Zinco
pv-nutrient-magnesium = Magnesio
pv-nutrient-calcium = Calcio
pv-nutrient-iodine = Iodio
pv-nutrient-selenium = Selenio
pv-nutrient-copper = Rame
pv-nutrient-potassium = Potassio
pv-nutrient-phosphorus = Fosforo
pv-nutrient-other = Altro
pv-sleep-disorder-insomnia = Insonnia
pv-sleep-disorder-sleep-related-breathing = Disturbo respiratorio nel sonno
pv-sleep-disorder-central-hypersomnolence = Ipersonnia centrale
pv-sleep-disorder-circadian-rhythm = Disturbo del ritmo circadiano
pv-sleep-disorder-parasomnia = Parasonnia
pv-sleep-disorder-sleep-related-movement = Disturbo del movimento nel sonno
pv-sleep-disorder-other = Altro
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Colloquio clinico
pv-assessment-instrument-other = Altro
pv-assessment-severity-none-minimal = Assente o minima
pv-assessment-severity-mild = Lieve
pv-assessment-severity-moderate = Moderata
pv-assessment-severity-moderately-severe = Moderatamente grave
pv-assessment-severity-severe = Grave
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Dati grezzi di microarray
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Altro
pv-zygosity-heterozygous = Eterozigote
pv-zygosity-homozygous = Omozigote
pv-zygosity-hemizygous = Emizigote
pv-zygosity-compound-heterozygous = Eterozigote composto
pv-clinical-significance-pathogenic = Patogenetica
pv-clinical-significance-likely-pathogenic = Probabilmente patogenetica
pv-clinical-significance-uncertain-significance = Significato incerto
pv-clinical-significance-likely-benign = Probabilmente benigna
pv-clinical-significance-benign = Benigna
pv-inheritance-pattern-autosomal-dominant = Autosomica dominante
pv-inheritance-pattern-autosomal-recessive = Autosomica recessiva
pv-inheritance-pattern-x-linked-dominant = Dominante legata all’X
pv-inheritance-pattern-x-linked-recessive = Recessiva legata all’X
pv-inheritance-pattern-y-linked = Legata all’Y
pv-inheritance-pattern-mitochondrial = Mitocondriale
pv-inheritance-pattern-multifactorial = Multifattoriale
pv-inheritance-pattern-unknown = Non nota
pv-carrier-status-affected = Affetto
pv-carrier-status-carrier = Portatore
pv-carrier-status-not-carrier = Non portatore
pv-carrier-status-unknown = Non noto
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Altro
pv-metaboliser-status-poor = Metabolizzatore lento
pv-metaboliser-status-intermediate = Metabolizzatore intermedio
pv-metaboliser-status-normal = Metabolizzatore normale
pv-metaboliser-status-rapid = Metabolizzatore rapido
pv-metaboliser-status-ultrarapid = Metabolizzatore ultrarapido
pv-autopsy-not-performed = Non eseguita
pv-autopsy-clinical = Clinica
pv-autopsy-forensic = Medico-legale
pv-autopsy-external-examination = Solo ispezione esterna
pv-autopsy-unknown = Non noto
pv-disposition-burial = Inumazione
pv-disposition-cremation = Cremazione
pv-disposition-entombment = Tumulazione
pv-disposition-burial-at-sea = Sepoltura in mare
pv-disposition-natural-burial = Sepoltura naturale
pv-disposition-body-donation = Donazione del corpo alla scienza
pv-disposition-other = Altro
pv-disposition-unknown = Non noto
pv-address-use-principal = Residenza principale
pv-address-use-secondary = Residenza secondaria
pv-address-use-temporary = Domicilio temporaneo
pv-address-use-postal = Recapito postale
pv-address-use-other = Altro
pv-nationality-mode-descent = Per discendenza
pv-nationality-mode-birth-in-territory = Per nascita sul territorio
pv-nationality-mode-naturalisation = Per naturalizzazione
pv-nationality-mode-marriage = Per matrimonio
pv-nationality-mode-registration = Per dichiarazione
pv-nationality-mode-restoration = Per riacquisto
pv-nationality-mode-state-succession = Per mutamento di sovranità
pv-nationality-mode-other = Altro
pv-language-proficiency-a1 = A1 Principiante
pv-language-proficiency-a2 = A2 Elementare
pv-language-proficiency-b1 = B1 Intermedio
pv-language-proficiency-b2 = B2 Intermedio superiore
pv-language-proficiency-c1 = C1 Avanzato
pv-language-proficiency-c2 = C2 Padronanza
pv-language-proficiency-native = Prima lingua
pv-isced-level-isced-0 = 0 Prima infanzia
pv-isced-level-isced-1 = 1 Primaria
pv-isced-level-isced-2 = 2 Secondaria di primo grado
pv-isced-level-isced-3 = 3 Secondaria di secondo grado
pv-isced-level-isced-4 = 4 Post-secondaria non terziaria
pv-isced-level-isced-5 = 5 Terziaria di ciclo breve
pv-isced-level-isced-6 = 6 Laurea triennale o equivalente
pv-isced-level-isced-7 = 7 Laurea magistrale o equivalente
pv-isced-level-isced-8 = 8 Dottorato o equivalente
pv-income-quintile-q1 = Quinto più basso
pv-income-quintile-q2 = Secondo quinto
pv-income-quintile-q3 = Quinto centrale
pv-income-quintile-q4 = Quarto quinto
pv-income-quintile-q5 = Quinto più alto
pv-pay-period-hourly = All’ora
pv-pay-period-daily = Al giorno
pv-pay-period-weekly = Alla settimana
pv-pay-period-monthly = Al mese
pv-pay-period-annual = All’anno
pv-tenure-owned = Proprietà
pv-tenure-co-owned = Comproprietà
pv-tenure-leasehold = Enfiteusi
pv-tenure-rented = Locazione
pv-tenure-usufruct = Usufrutto
pv-tenure-other = Altro
pv-distinction-kind-order = Ordine
pv-distinction-kind-decoration = Decorazione
pv-distinction-kind-medal = Medaglia
pv-distinction-kind-title = Titolo onorifico
pv-distinction-kind-other = Altro
pv-military-service-army = Esercito
pv-military-service-navy = Marina
pv-military-service-air-force = Aeronautica
pv-military-service-marines = Fanteria di marina
pv-military-service-gendarmerie = Carabinieri o gendarmeria
pv-military-service-border-guard = Guardia di frontiera
pv-military-service-national-guard = Guardia nazionale
pv-military-service-other = Altro
pv-rank-category-enlisted = Truppa
pv-rank-category-non-commissioned = Sottufficiali
pv-rank-category-warrant = Marescialli o equivalenti
pv-rank-category-officer-cadet = Allievi ufficiali
pv-rank-category-junior-officer = Ufficiali inferiori
pv-rank-category-senior-officer = Ufficiali superiori
pv-rank-category-general-officer = Ufficiali generali
pv-iccs-section-acts-leading-to-death = 01 Atti che causano la morte
pv-iccs-section-acts-causing-harm = 02 Atti che causano danni alla persona
pv-iccs-section-sexual-acts = 03 Atti lesivi di natura sessuale
pv-iccs-section-property-with-violence = 04 Contro il patrimonio con violenza
pv-iccs-section-property-only = 05 Contro il patrimonio
pv-iccs-section-controlled-substances = 06 Sostanze controllate
pv-iccs-section-fraud-deception-corruption = 07 Frode, inganno o corruzione
pv-iccs-section-public-order-and-state = 08 Contro l’ordine pubblico e lo Stato
pv-iccs-section-public-safety-and-security = 09 Contro la sicurezza pubblica
pv-iccs-section-natural-environment = 10 Contro l’ambiente
pv-iccs-section-other-criminal-acts = 11 Altri reati
pv-case-outcome-convicted = Condannato
pv-case-outcome-acquitted = Assolto
pv-case-outcome-dismissed = Archiviato
pv-case-outcome-conviction-quashed = Condanna annullata
pv-case-outcome-pardoned = Graziato
pv-case-outcome-amnestied = Amnistiato
pv-case-outcome-expunged = Estinto
pv-case-outcome-pending = In corso
pv-case-outcome-unknown = Non noto
pv-religion-buddhism = Buddhismo
pv-religion-christianity-catholic = Cristianesimo: cattolico
pv-religion-christianity-orthodox = Cristianesimo: ortodosso
pv-religion-christianity-protestant = Cristianesimo: protestante
pv-religion-christianity-other = Cristianesimo: altro
pv-religion-hinduism = Induismo
pv-religion-islam-sunni = Islam: sunnita
pv-religion-islam-shia = Islam: sciita
pv-religion-islam-other = Islam: altro
pv-religion-jainism = Giainismo
pv-religion-judaism = Ebraismo
pv-religion-sikhism = Sikhismo
pv-religion-bahai = Fede bahá’í
pv-religion-shinto = Shintoismo
pv-religion-taoism = Taoismo
pv-religion-zoroastrianism = Zoroastrismo
pv-religion-traditional = Religione tradizionale o popolare
pv-religion-other = Altra
pv-religion-none = Nessuna religione
pv-religion-unknown = Non nota
pv-sacrament-baptism = Battesimo
pv-sacrament-confirmation = Cresima
pv-sacrament-first-communion = Prima comunione
pv-sacrament-reconciliation = Confessione
pv-sacrament-anointing-of-the-sick = Unzione degli infermi
pv-sacrament-holy-orders = Ordine sacro
pv-sacrament-matrimony = Matrimonio
pv-sacrament-other-rite = Altro rito
pv-political-position-far-left = Estrema sinistra
pv-political-position-left = Sinistra
pv-political-position-centre-left = Centro-sinistra
pv-political-position-centre = Centro
pv-political-position-centre-right = Centro-destra
pv-political-position-right = Destra
pv-political-position-far-right = Estrema destra
pv-political-position-apolitical = Apolitico
pv-political-position-other = Fuori da questo asse
pv-political-position-unknown = Non nota
pv-membership-kind-trade-union = Sindacato
pv-membership-kind-political-party = Partito politico
pv-membership-kind-professional-body = Ordine professionale
pv-membership-kind-religious-order = Ordine religioso
pv-membership-kind-religious-association = Associazione religiosa
pv-membership-kind-fraternal-order = Confraternita
pv-membership-kind-veterans-association = Associazione di reduci
pv-membership-kind-sports-club = Società sportiva
pv-membership-kind-cultural-association = Associazione culturale
pv-membership-kind-charitable-association = Associazione benefica
pv-membership-kind-other = Altro
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Valutazione di chi la conosceva
pv-personality-instrument-inferred = Dedotto dai documenti
pv-personality-instrument-other = Altro
pv-introversion-extraversion-strongly-introverted = Molto introverso
pv-introversion-extraversion-introverted = Introverso
pv-introversion-extraversion-ambiverted = Ambiverso
pv-introversion-extraversion-extraverted = Estroverso
pv-introversion-extraversion-strongly-extraverted = Molto estroverso
pv-stress-tolerance-very-low = Molto bassa
pv-stress-tolerance-low = Bassa
pv-stress-tolerance-moderate = Moderata
pv-stress-tolerance-high = Alta
pv-stress-tolerance-very-high = Molto alta
pv-decision-style-rational = Razionale
pv-decision-style-intuitive = Intuitivo
pv-decision-style-dependent = Dipendente
pv-decision-style-avoidant = Evitante
pv-decision-style-spontaneous = Spontaneo
pv-sport-level-recreational = Amatoriale
pv-sport-level-amateur-competitive = Agonistico dilettantistico
pv-sport-level-semi-professional = Semiprofessionistico
pv-sport-level-professional = Professionistico
pv-diet-omnivore = Onnivora
pv-diet-flexitarian = Flessitariana
pv-diet-pescatarian = Pescetariana
pv-diet-vegetarian = Vegetariana
pv-diet-vegan = Vegana
pv-diet-other = Altro
pv-substance-tobacco = Tabacco e nicotina
pv-substance-alcohol = Alcol
pv-substance-cannabis = Cannabis
pv-substance-opioids = Oppioidi
pv-substance-stimulants = Stimolanti
pv-substance-sedatives-hypnotics = Sedativi e ipnotici
pv-substance-hallucinogens = Allucinogeni
pv-substance-inhalants = Inalanti
pv-substance-gambling = Gioco d’azzardo
pv-substance-gaming = Videogiochi
pv-substance-other = Altro
pv-use-pattern-occasional-use = Uso occasionale
pv-use-pattern-regular-use = Uso regolare
pv-use-pattern-harmful-use = Uso dannoso
pv-use-pattern-dependence = Dipendenza
pv-use-pattern-in-remission = In remissione
pv-lineage-biological = Biologica
pv-lineage-adoptive = Adottiva
pv-lineage-foster = Affidataria
pv-lineage-step = Acquisita
pv-lineage-guardianship = Tutela
pv-lineage-unknown = Non nota
pv-link-relation-godparent = Padrino o madrina
pv-link-relation-godchild = Figlioccio
pv-link-relation-witness = Testimone
pv-link-relation-officiant = Celebrante
pv-link-relation-business-partner = Socio d’affari
pv-link-relation-employer = Datore di lavoro
pv-link-relation-employee = Dipendente
pv-link-relation-mentor = Mentore
pv-link-relation-apprentice = Apprendista
pv-link-relation-close-friend = Amico intimo
pv-link-relation-neighbour = Vicino
pv-link-relation-guardian = Tutore
pv-link-relation-ward = Pupillo
pv-link-relation-other = Altro
pv-country-AD = Andorra
pv-country-AE = Emirati Arabi Uniti
pv-country-AF = Afghanistan
pv-country-AG = Antigua e Barbuda
pv-country-AI = Anguilla
pv-country-AL = Albania
pv-country-AM = Armenia
pv-country-AO = Angola
pv-country-AQ = Antartide
pv-country-AR = Argentina
pv-country-AS = Samoa americane
pv-country-AT = Austria
pv-country-AU = Australia
pv-country-AW = Aruba
pv-country-AX = Isole Åland
pv-country-AZ = Azerbaigian
pv-country-BA = Bosnia ed Erzegovina
pv-country-BB = Barbados
pv-country-BD = Bangladesh
pv-country-BE = Belgio
pv-country-BF = Burkina Faso
pv-country-BG = Bulgaria
pv-country-BH = Bahrein
pv-country-BI = Burundi
pv-country-BJ = Benin
pv-country-BL = Saint-Barthélemy
pv-country-BM = Bermuda
pv-country-BN = Brunei
pv-country-BO = Bolivia
pv-country-BQ = Caraibi olandesi
pv-country-BR = Brasile
pv-country-BS = Bahamas
pv-country-BT = Bhutan
pv-country-BV = Isola Bouvet
pv-country-BW = Botswana
pv-country-BY = Bielorussia
pv-country-BZ = Belize
pv-country-CA = Canada
pv-country-CC = Isole Cocos (Keeling)
pv-country-CD = Congo - Kinshasa
pv-country-CF = Repubblica Centrafricana
pv-country-CG = Congo-Brazzaville
pv-country-CH = Svizzera
pv-country-CI = Costa d’Avorio
pv-country-CK = Isole Cook
pv-country-CL = Cile
pv-country-CM = Camerun
pv-country-CN = Cina
pv-country-CO = Colombia
pv-country-CR = Costa Rica
pv-country-CU = Cuba
pv-country-CV = Capo Verde
pv-country-CW = Curaçao
pv-country-CX = Isola Christmas
pv-country-CY = Cipro
pv-country-CZ = Cechia
pv-country-DE = Germania
pv-country-DJ = Gibuti
pv-country-DK = Danimarca
pv-country-DM = Dominica
pv-country-DO = Repubblica Dominicana
pv-country-DZ = Algeria
pv-country-EC = Ecuador
pv-country-EE = Estonia
pv-country-EG = Egitto
pv-country-EH = Sahara occidentale
pv-country-ER = Eritrea
pv-country-ES = Spagna
pv-country-ET = Etiopia
pv-country-FI = Finlandia
pv-country-FJ = Figi
pv-country-FK = Isole Falkland
pv-country-FM = Micronesia
pv-country-FO = Isole Fær Øer
pv-country-FR = Francia
pv-country-GA = Gabon
pv-country-GB = Regno Unito
pv-country-GD = Grenada
pv-country-GE = Georgia
pv-country-GF = Guyana francese
pv-country-GG = Guernsey
pv-country-GH = Ghana
pv-country-GI = Gibilterra
pv-country-GL = Groenlandia
pv-country-GM = Gambia
pv-country-GN = Guinea
pv-country-GP = Guadalupa
pv-country-GQ = Guinea Equatoriale
pv-country-GR = Grecia
pv-country-GS = Georgia del Sud e Sandwich australi
pv-country-GT = Guatemala
pv-country-GU = Guam
pv-country-GW = Guinea-Bissau
pv-country-GY = Guyana
pv-country-HK = RAS di Hong Kong
pv-country-HM = Isole Heard e McDonald
pv-country-HN = Honduras
pv-country-HR = Croazia
pv-country-HT = Haiti
pv-country-HU = Ungheria
pv-country-ID = Indonesia
pv-country-IE = Irlanda
pv-country-IL = Israele
pv-country-IM = Isola di Man
pv-country-IN = India
pv-country-IO = Territorio britannico dell’Oceano Indiano
pv-country-IQ = Iraq
pv-country-IR = Iran
pv-country-IS = Islanda
pv-country-IT = Italia
pv-country-JE = Jersey
pv-country-JM = Giamaica
pv-country-JO = Giordania
pv-country-JP = Giappone
pv-country-KE = Kenya
pv-country-KG = Kirghizistan
pv-country-KH = Cambogia
pv-country-KI = Kiribati
pv-country-KM = Comore
pv-country-KN = Saint Kitts e Nevis
pv-country-KP = Corea del Nord
pv-country-KR = Corea del Sud
pv-country-KW = Kuwait
pv-country-KY = Isole Cayman
pv-country-KZ = Kazakistan
pv-country-LA = Laos
pv-country-LB = Libano
pv-country-LC = Saint Lucia
pv-country-LI = Liechtenstein
pv-country-LK = Sri Lanka
pv-country-LR = Liberia
pv-country-LS = Lesotho
pv-country-LT = Lituania
pv-country-LU = Lussemburgo
pv-country-LV = Lettonia
pv-country-LY = Libia
pv-country-MA = Marocco
pv-country-MC = Monaco
pv-country-MD = Moldavia
pv-country-ME = Montenegro
pv-country-MF = Saint Martin
pv-country-MG = Madagascar
pv-country-MH = Isole Marshall
pv-country-MK = Macedonia del Nord
pv-country-ML = Mali
pv-country-MM = Myanmar (Birmania)
pv-country-MN = Mongolia
pv-country-MO = RAS di Macao
pv-country-MP = Isole Marianne settentrionali
pv-country-MQ = Martinica
pv-country-MR = Mauritania
pv-country-MS = Montserrat
pv-country-MT = Malta
pv-country-MU = Mauritius
pv-country-MV = Maldive
pv-country-MW = Malawi
pv-country-MX = Messico
pv-country-MY = Malaysia
pv-country-MZ = Mozambico
pv-country-NA = Namibia
pv-country-NC = Nuova Caledonia
pv-country-NE = Niger
pv-country-NF = Isola Norfolk
pv-country-NG = Nigeria
pv-country-NI = Nicaragua
pv-country-NL = Paesi Bassi
pv-country-NO = Norvegia
pv-country-NP = Nepal
pv-country-NR = Nauru
pv-country-NU = Niue
pv-country-NZ = Nuova Zelanda
pv-country-OM = Oman
pv-country-PA = Panamá
pv-country-PE = Perù
pv-country-PF = Polinesia francese
pv-country-PG = Papua Nuova Guinea
pv-country-PH = Filippine
pv-country-PK = Pakistan
pv-country-PL = Polonia
pv-country-PM = Saint-Pierre e Miquelon
pv-country-PN = Isole Pitcairn
pv-country-PR = Portorico
pv-country-PS = Territori palestinesi
pv-country-PT = Portogallo
pv-country-PW = Palau
pv-country-PY = Paraguay
pv-country-QA = Qatar
pv-country-RE = Riunione
pv-country-RO = Romania
pv-country-RS = Serbia
pv-country-RU = Russia
pv-country-RW = Ruanda
pv-country-SA = Arabia Saudita
pv-country-SB = Isole Salomone
pv-country-SC = Seychelles
pv-country-SD = Sudan
pv-country-SE = Svezia
pv-country-SG = Singapore
pv-country-SH = Sant’Elena
pv-country-SI = Slovenia
pv-country-SJ = Svalbard e Jan Mayen
pv-country-SK = Slovacchia
pv-country-SL = Sierra Leone
pv-country-SM = San Marino
pv-country-SN = Senegal
pv-country-SO = Somalia
pv-country-SR = Suriname
pv-country-SS = Sud Sudan
pv-country-ST = São Tomé e Príncipe
pv-country-SV = El Salvador
pv-country-SX = Sint Maarten
pv-country-SY = Siria
pv-country-SZ = Swaziland
pv-country-TC = Isole Turks e Caicos
pv-country-TD = Ciad
pv-country-TF = Terre australi francesi
pv-country-TG = Togo
pv-country-TH = Thailandia
pv-country-TJ = Tagikistan
pv-country-TK = Tokelau
pv-country-TL = Timor Est
pv-country-TM = Turkmenistan
pv-country-TN = Tunisia
pv-country-TO = Tonga
pv-country-TR = Turchia
pv-country-TT = Trinidad e Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taiwan
pv-country-TZ = Tanzania
pv-country-UA = Ucraina
pv-country-UG = Uganda
pv-country-UM = Altre isole americane del Pacifico
pv-country-US = Stati Uniti
pv-country-UY = Uruguay
pv-country-UZ = Uzbekistan
pv-country-VA = Città del Vaticano
pv-country-VC = Saint Vincent e Grenadine
pv-country-VE = Venezuela
pv-country-VG = Isole Vergini Britanniche
pv-country-VI = Isole Vergini Americane
pv-country-VN = Vietnam
pv-country-VU = Vanuatu
pv-country-WF = Wallis e Futuna
pv-country-WS = Samoa
pv-country-YE = Yemen
pv-country-YT = Mayotte
pv-country-ZA = Sudafrica
pv-country-ZM = Zambia
pv-country-ZW = Zimbabwe
pv-country-SU = Unione Sovietica
pv-country-DD = Germania Est
pv-country-YU = Jugoslavia
pv-country-CS = Cecoslovacchia
pv-country-OT = Impero ottomano
lang-aa = Afar
lang-ab = Abcaso
lang-ae = Avestan
lang-af = Afrikaans
lang-ak = Akan
lang-am = Amarico
lang-an = Aragonese
lang-ar = Arabo
lang-as = Assamese
lang-av = Avaro
lang-ay = Aymara
lang-az = Azerbaigiano
lang-ba = Baschiro
lang-be = Bielorusso
lang-bg = Bulgaro
lang-bi = Bislama
lang-bm = Bambara
lang-bn = Bengalese
lang-bo = Tibetano
lang-br = Bretone
lang-bs = Bosniaco
lang-ca = Catalano
lang-ce = Ceceno
lang-ch = Chamorro
lang-co = Corso
lang-cr = Cree
lang-cs = Ceco
lang-cu = Slavo ecclesiastico
lang-cv = Ciuvascio
lang-cy = Gallese
lang-da = Danese
lang-de = Tedesco
lang-dv = Divehi
lang-dz = Dzongkha
lang-ee = Ewe
lang-el = Greco
lang-en = Inglese
lang-eo = Esperanto
lang-es = Spagnolo
lang-et = Estone
lang-eu = Basco
lang-fa = Persiano
lang-ff = Fulah
lang-fi = Finlandese
lang-fj = Figiano
lang-fo = Faroese
lang-fr = Francese
lang-fy = Frisone occidentale
lang-ga = Irlandese
lang-gd = Gaelico scozzese
lang-gl = Galiziano
lang-gn = Guaraní
lang-gu = Gujarati
lang-gv = Mannese
lang-ha = Hausa
lang-he = Ebraico
lang-hi = Hindi
lang-ho = Hiri motu
lang-hr = Croato
lang-ht = Creolo haitiano
lang-hu = Ungherese
lang-hy = Armeno
lang-hz = Herero
lang-ia = Interlingua
lang-id = Indonesiano
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Sichuan yi
lang-ik = Inupiak
lang-io = Ido
lang-is = Islandese
lang-it = Italiano
lang-iu = Inuktitut
lang-ja = Giapponese
lang-jv = Giavanese
lang-ka = Georgiano
lang-kg = Kongo
lang-ki = Kikuyu
lang-kj = Kuanyama
lang-kk = Kazako
lang-kl = Groenlandese
lang-km = Khmer
lang-kn = Kannada
lang-ko = Coreano
lang-kr = Kanuri
lang-ks = Kashmiri
lang-ku = Curdo
lang-kv = Komi
lang-kw = Cornico
lang-ky = Kirghiso
lang-la = Latino
lang-lb = Lussemburghese
lang-lg = Ganda
lang-li = Limburghese
lang-ln = Lingala
lang-lo = Lao
lang-lt = Lituano
lang-lu = Luba-katanga
lang-lv = Lettone
lang-mg = Malgascio
lang-mh = Marshallese
lang-mi = Maori
lang-mk = Macedone
lang-ml = Malayalam
lang-mn = Mongolo
lang-mr = Marathi
lang-ms = Malese
lang-mt = Maltese
lang-my = Birmano
lang-na = Nauru
lang-nb = Norvegese bokmål
lang-nd = Ndebele del nord
lang-ne = Nepalese
lang-ng = Ndonga
lang-nl = Olandese
lang-nn = Norvegese nynorsk
lang-no = Norvegese
lang-nr = Ndebele del sud
lang-nv = Navajo
lang-ny = Nyanja
lang-oc = Occitano
lang-oj = Ojibwa
lang-om = Oromo
lang-or = Odia
lang-os = Ossetico
lang-pa = Punjabi
lang-pi = Pali
lang-pl = Polacco
lang-ps = Pashto
lang-pt = Portoghese
lang-qu = Quechua
lang-rm = Romancio
lang-rn = Rundi
lang-ro = Rumeno
lang-ru = Russo
lang-rw = Kinyarwanda
lang-sa = Sanscrito
lang-sc = Sardo
lang-sd = Sindhi
lang-se = Sami del nord
lang-sg = Sango
lang-sh = Serbo-croato
lang-si = Singalese
lang-sk = Slovacco
lang-sl = Sloveno
lang-sm = Samoano
lang-sn = Shona
lang-so = Somalo
lang-sq = Albanese
lang-sr = Serbo
lang-ss = Swati
lang-st = Sotho del sud
lang-su = Sundanese
lang-sv = Svedese
lang-sw = Swahili
lang-ta = Tamil
lang-te = Telugu
lang-tg = Tagico
lang-th = Thai
lang-ti = Tigrino
lang-tk = Turcomanno
lang-tl = Tagalog
lang-tn = Tswana
lang-to = Tongano
lang-tr = Turco
lang-ts = Tsonga
lang-tt = Tataro
lang-tw = Ci
lang-ty = Taitiano
lang-ug = Uiguro
lang-uk = Ucraino
lang-ur = Urdu
lang-uz = Uzbeco
lang-ve = Venda
lang-vi = Vietnamita
lang-vo = Volapük
lang-wa = Vallone
lang-wo = Wolof
lang-xh = Xhosa
lang-yi = Yiddish
lang-yo = Yoruba
lang-za = Zhuang
lang-zh = Cinese
lang-zu = Zulu
person-tab-profile = Profilo
profile-groups-label = Sezioni del profilo
profile-group-withheld = Una parte di questa sezione non è visibile per te
profile-withheld = Registrato per questa persona e non visibile per te: { $classes }.
profile-empty = In questa sezione non è ancora registrato nulla.
profile-earlier = modulo precedente
profile-earlier-title = Registrato da una versione precedente di questa applicazione, in un campo che AXGF 1.1 non prevede. È conservato come è stato scritto.
profile-other-names = { $n ->
        [one] e un altro nome
       *[other] e altri { $n } nomi
    }
profile-edit-group = Modifica «{ $group }»
profile-summary-link = { $n ->
        [one] Un dato nel profilo
       *[other] { $n } dati nel profilo
    }
profile-from = dal
profile-until = al
profile-yes = Sì
profile-no = No
profile-value = Valore
profile-editor-title = Profilo
profile-problems = Una parte dei dati inseriti non è stata salvata. Ogni problema è indicato accanto al suo campo, e non è stato scritto nulla.
profile-editor-withheld = Questa sezione contiene anche, per questa persona, dati della categoria { $classes } che non puoi leggere. Non sono mostrati qui, e salvare il modulo li lascia invariati.
profile-living-class-note = Questa persona risulta in vita. Quello che inserisci qui in una categoria sensibile è visibile solo agli amministratori.
profile-relationships-elsewhere = Genitori, coniugi, figli, padrini e testimoni non sono registrati su questa persona. Sono famiglie, collegamenti ed eventi che la nominano — per questo ogni modifica qui cambia anche la scheda di tutte le altre persone coinvolte.
profile-documents-first = Un artefatto rimanda a un documento collegato a questa persona. Allega prima il file.
profile-editor-nothing = In questa sezione non c’è nulla che tu possa modificare.
profile-new-entry = Nuova voce
profile-provenance = Data, fonte e attendibilità
profile-from-date = Valido dal
profile-until-date = Valido fino al
profile-remove-entry = Rimuovi questa voce
profile-add-entry = Aggiungi un’altra voce
profile-no-such-group-title = Sezione inesistente
profile-no-such-group-detail = Il profilo non ha una sezione con questo nome.
profile-error-number = Un valore di questo campo deve essere un numero.
profile-error-integer = Un valore di questo campo deve essere un numero intero.
profile-error-range = Un numero è fuori dall’intervallo ammesso per questo attributo.
profile-error-term = Un valore non è tra le scelte proposte.
profile-error-required = A una voce manca un campo necessario.
profile-error-one-of = Una voce richiede almeno uno dei suoi campi principali.
profile-error-confidence = L’attendibilità va da 0 a 1, per esempio 0,8.
profile-error-time = Un orario si scrive in ore e minuti, per esempio 05:40.
profile-error-currency = Una valuta si scrive con il suo codice di tre lettere, per esempio EUR.
profile-error-language = Una lingua si scrive con il suo codice, per esempio it o zh-Hans.
profile-error-coordinates = Le coordinate richiedono una latitudine tra −90 e 90 e una longitudine tra −180 e 180.
profile-error-rank-country = Il grado appartiene a un Paese diverso da quello scelto.
record-unknown-place = [Luogo sconosciuto]
record-missing-document = [Documento mancante]

## Interface

confidence-certain = Certezza { $percent }% — praticamente certo
confidence-high = Certezza { $percent }% — ben documentato
confidence-medium = Certezza { $percent }% — plausibile ma non confermato
confidence-low = Certezza { $percent }% — ipotesi
tree-edge-union-between = { $from } e { $to } — { $confidence }
tree-edge-parentage-of = { $from }, genitore di { $to } — { $confidence }
record-note-biography = Biografia
record-note-birth-date-as-recorded = Data di nascita, così come è registrata
record-note-death-date-as-recorded = Data di morte, così come è registrata
record-note-event-date-as-recorded = Data di «{ $event }», così come è registrata
record-unknown-source = [Fonte sconosciuta]
record-untitled-source = [Fonte senza titolo]
record-unnamed = [Senza nome]
record-untitled = [Senza titolo]
record-period-from = dal { $date }
record-period-until = fino al { $date }
record-dates-unrecorded = date non registrate
record-link-unlabelled = collegato a
record-link-reverse = { $label } (di)
record-place-worked-as = Ha lavorato come { $title }
record-place-married-to = Matrimonio con { $name }
record-place-married = Matrimonio
record-source-use-name = il nome «{ $name }»
record-source-use-working-as = il lavoro di { $title }
record-source-use-union-with = l'unione con { $name }
record-source-use-union = l'unione
record-lifespan-born = n. { $year }
record-lifespan-died = m. { $year }
size-bytes = { $n ->
        [one] { $n } byte
       *[other] { $n } byte
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = gregoriano
calendar-julian = giuliano
calendar-hebrew = ebraico
calendar-hijri = islamico
calendar-persian = persiano
calendar-chinese = cinese
calendar-ethiopian = etiope
calendar-japanese_era = ere giapponesi
calendar-republican_french = repubblicano francese
calendar-roman = romano
diff-summary-none = non ha modificato alcun campo
diff-summary-one = ha modificato { $a }
diff-summary-two = ha modificato { $a } e { $b }
diff-summary-many = ha modificato { $a }, { $b } e { $n ->
        [one] un altro campo
       *[other] altri { $n } campi
    }
diff-saved-none = nessun campo modificato
diff-saved-one = { $a } modificato
diff-saved-two = { $a } e { $b } modificati
diff-saved-many = { $a }, { $b } e { $n ->
        [one] un altro campo modificati
       *[other] altri { $n } campi modificati
    }
history-created = ha creato
history-deleted = ha eliminato
history-attached = ha allegato un file
admin-raw-json-unparsed = Il JSON grezzo non si è potuto leggere ({ $error }). Non è stato salvato nulla.
conflict-someone = Qualcuno
conflict-unrecorded-time = (ora non registrata)
dedup-merged-persons = { $n ->
        [one] una persona unita
       *[other] { $n } persone unite
    }
dedup-merged-families = { $n ->
        [one] una famiglia unita
       *[other] { $n } famiglie unite
    }
dedup-manual-review = { $n ->
        [one] un caso lasciato all'esame di una persona
       *[other] { $n } casi lasciati all'esame di una persona
    }
dedup-nothing = Niente da segnalare.
record-union-duplicate = Una coppia, più di una scheda.
record-union-duplicate-detail = Il pacchetto conserva schede Famiglia distinte per queste due persone. È un difetto dei dati, non una seconda unione.
record-union-duplicate-action = Unisci i duplicati
record-union-duplicate-confirm = Deduplicare l’intero pacchetto? Ogni coppia che la libreria può unire verrà unita; le altre verranno segnalate.
dedup-pair-merged = La coppia in questione è ora una sola scheda.
dedup-pair-refused = La coppia in questione non è stata unita: la libreria l’ha rifiutata e l’ha lasciata all’esame di una persona.

validate-errors = { $n ->
        [one] un errore
       *[other] { $n } errori
    }
validate-warnings = { $n ->
        [one] un avviso
       *[other] { $n } avvisi
    }
validate-notes = { $n ->
        [one] una nota
       *[other] { $n } note
    }
validate-nothing = Niente da segnalare.
list-separator = { ", " }
result-written = L'archivio è stato scritto su disco.
result-refused = La libreria ha rifiutato questa operazione. L'archivio su disco è invariato.
convert-error-no-file = Non è stato caricato alcun file. Scegliete prima un file .ged.
convert-error-file-too-large = Questo file pesa { $size } MB e il limite è { $limit } MB. Non è stato convertito nulla.
convert-error-too-large = Il caricamento supera il limite di { $limit } MB. Non è stato convertito nulla.
convert-error-unreadable = Il caricamento non si è potuto leggere ({ $error }). Non è stato convertito nulla.
convert-error-not-gedcom = Non sembra un file GEDCOM: un file GEDCOM 5.5.1 inizia con una riga «0 HEAD». Non è stato convertito nulla.
convert-error-packaging = Il file è stato convertito ma non si è potuto impacchettare ({ $error }).
completeness-fraction = { $part } su { $whole }
event-category-adoption = Adozione
event-category-migration = Migrazione
event-category-naturalization = Naturalizzazione
event-category-incarceration = Detenzione
event-category-name_change = Cambio di nome
event-category-legal = Vicenda giudiziaria
event-category-religious = Evento religioso
event-category-social = Evento sociale
event-category-historical = Evento storico
precision-quarter_century = al quarto di secolo
source-type-birth_certificate = atto di nascita
source-type-death_certificate = atto di morte
source-type-marriage_certificate = atto di matrimonio
source-type-census = censimento
source-type-baptism_record = atto di battesimo
source-type-burial_record = atto di sepoltura
source-type-will = testamento
source-type-land_record = registro fondiario
source-type-military_record = foglio matricolare
source-type-immigration_record = registro d'immigrazione
source-type-naturalization = atto di naturalizzazione
source-type-passport = passaporto
source-type-photograph = fotografia
source-type-letter = lettera
source-type-diary = diario
source-type-newspaper = giornale
source-type-oral_tradition = tradizione orale
source-type-dna = test del DNA
source-type-family_bible = Bibbia di famiglia
source-type-gravestone = lapide
source-type-published_genealogy = genealogia pubblicata
source-type-other = altra fonte
source-status-verified = verificata sull'originale
source-status-unverified = non ancora verificata
source-status-lost = perduta
source-status-known_missing = notoriamente mancante
document-type-birth_certificate = atto di nascita
document-type-death_certificate = atto di morte
document-type-marriage_certificate = atto di matrimonio
document-type-census_page = pagina di censimento
document-type-baptism_record = atto di battesimo
document-type-military_record = foglio matricolare
document-type-will = testamento
document-type-land_record = registro fondiario
document-type-diary = diario
document-type-newspaper_clipping = ritaglio di giornale
document-type-gravestone_photo = foto di una lapide
document-type-family_tree_drawing = albero genealogico disegnato
document-type-audio = registrazione sonora
document-type-video = registrazione video
document-status-present = conservato qui
document-status-referenced = citato, conservato altrove
document-status-known_missing = notoriamente mancante
document-status-lost = perduto
document-status-unknown = collocazione sconosciuta
diag-unsupported_spec_version = L'archivio dichiara una versione di AXGF che questo programma non sa leggere.
diag-invalid_json = Qualcosa che dovrebbe essere JSON non si riesce a leggere.
diag-invalid_bundle_structure = L'archivio non è organizzato come richiede AXGF.
diag-schema_validation_failed = Un record non corrisponde allo schema AXGF.
diag-dangling_reference = Un record rimanda a un altro record che non è nell'archivio.
diag-duplicate_entity_id = Due record hanno lo stesso identificativo.
diag-duplicate_unique_ref = Due record rivendicano lo stesso riferimento, che dovrebbe essere unico.
diag-cycle_detected = I legami familiari girano in tondo: qualcuno sarebbe antenato di sé stesso.
diag-chronology_conflict = Le date si contraddicono, per esempio un figlio nato prima di un genitore.
diag-out_of_vocabulary = Un valore non è tra i termini che il suo elenco ammette.
diag-claim_inconsistent = Un'affermazione contraddice sé stessa o un'altra affermazione sulla stessa cosa.
diag-spec_version_mismatch = La versione di AXGF dichiarata da un record non corrisponde al suo contenuto.
diag-unknown_attribute = Un record contiene un attributo che AXGF non definisce.
diag-entity_not_found = Il record da modificare non è nell'archivio.
diag-entity_already_exists = Esiste già un record con questo identificativo.
diag-unknown_entity_kind = Questo non è un tipo di record previsto da AXGF.
diag-delete_blocked_by_reference = Il record non si può eliminare finché altri record vi rimandano.
diag-manual_review_required = Deve esaminarlo una persona; non è stato modificato automaticamente.
diag-zip_read_error = Il file dell'archivio non si è potuto leggere.
diag-zip_write_error = Il file dell'archivio non si è potuto scrivere.
diag-payloads_external = I file allegati sono conservati fuori dai dati dell'archivio.
diag-payload_source_failed = Un file allegato non si è potuto leggere.
diag-payload_sink_failed = Un file allegato non si è potuto scrivere.
diag-gedcom_parse_error = Una riga del file GEDCOM non si è potuta interpretare.
diag-gedcom_unrecognized_tag = Il file GEDCOM usa un'etichetta che l'importazione non conosce, quindi quella voce non è stata ripresa.
diag-internal = Qualcosa è andato storto all'interno della libreria.
field-person-display-name = Nome visualizzato
field-person-display-name-hint = Il nome mostrato ovunque nel sito.
field-person-gender = Genere
field-person-living = In vita
field-person-birth-date = Data di nascita
field-date-value-hint = Un anno, anno e mese oppure una data completa: 1923, 1923-04 o 1923-04-12. Lasciate vuoto se nessuno la conosce.
field-person-birth-precision = Precisione della nascita
field-precision-hint = Con quanta precisione la fonte lo stabilisce.
field-person-birth-circa = Nascita approssimativa
field-circa-hint = Mostrata come «circa 1923» anziché come affermazione esatta.
field-person-birth-place = Identificativo del luogo di nascita
field-person-birth-confidence = Certezza della nascita
field-person-confidence-hint = Quanto ne siete sicuri. È ciò che il sito disegna.
field-person-death-date = Data di morte
field-person-death-precision = Precisione della morte
field-person-death-circa = Morte approssimativa
field-person-death-place = Identificativo del luogo di morte
field-person-death-confidence = Certezza della morte
field-person-death-cause = Causa di morte
field-person-bio = Biografia
field-notes = Note
field-family-name = Nome della famiglia
field-description = Descrizione
field-family-union-type = Tipo di unione
field-family-union-status = Stato dell'unione
field-family-union-confidence = Certezza dell'unione
field-family-union-confidence-hint = Stabilisce quanto marcata è la linea tra i partner nell'albero.
field-family-union-start = Inizio dell'unione
field-family-union-end = Fine dell'unione
field-family-notes-hint = Partner e figli sono elenchi: modificateli nel JSON grezzo qui sotto o nella pagina delle relazioni della persona.
field-category = Categoria
field-required-hint = Obbligatorio.
field-event-subcategory = Sottocategoria
field-date = Data
field-event-date-hint = Richiesta dallo schema.
field-precision = Precisione
field-circa = Approssimativa
field-place-id = Identificativo del luogo
field-confidence = Certezza
field-source-id = Identificativo della fonte
field-link-from-type = Tipo di partenza
field-link-from-id = Identificativo di partenza
field-link-to-type = Tipo di arrivo
field-link-to-id = Identificativo di arrivo
field-link-label = Denominazione
field-link-label-hint = Si legge nel verso del legame: «padrino», «datore di lavoro», «testimone». Obbligatorio.
field-link-label-reverse = Denominazione inversa
field-link-label-reverse-hint = Come si legge dall'altro capo: «figlioccio», «dipendente».
field-link-bidirectional = Si legge uguale in entrambi i versi
field-valid-from = Valido dal
field-link-valid-from-hint = Quando è iniziata la relazione.
field-valid-until = Valido fino al
field-link-confidence-hint = «Sicuro all'85%, secondo una lettera di famiglia» — ciò che GEDCOM non sa dire.
field-note = Nota
field-occupation-person-id = Identificativo della persona
field-occupation-title = Attività
field-occupation-title-hint = Obbligatorio, per esempio Maestra.
field-occupation-title-latin = Attività (alfabeto latino)
field-occupation-employer = Datore di lavoro
field-occupation-from = Dal
field-occupation-from-hint = Un'attività è un periodo. Dare entrambi gli estremi permette di disegnarla come una barra.
field-occupation-until = Al
field-source-title = Titolo
field-source-type = Tipo di fonte
field-source-reliability = Affidabilità
field-source-reliability-hint = Obbligatorio. Mostrata come etichetta accanto a ogni fatto che si basa su questa fonte.
field-source-status = Stato della fonte
field-source-repository = Luogo di conservazione
field-source-repository-reference = Segnatura nel luogo di conservazione
field-source-transcription = Trascrizione
field-place-name = Nome principale
field-place-name-lang = Lingua del nome
field-place-name-lang-hint = Un codice di lingua, per esempio en, fr o pl.
field-place-type = Tipo di luogo
field-place-region = Regione
field-place-country-current = Stato attuale
field-place-country-current-hint = La storia dei suoi confini è un elenco: modificatela nel JSON grezzo qui sotto.
field-document-filename = Nome del file
field-document-mime-type = Tipo di contenuto
field-document-mime-type-hint = Obbligatorio, per esempio image/jpeg.
field-document-type = Tipo di documento
field-document-status = Stato del file
field-document-url = Indirizzo web
field-document-caption = Didascalia
lang-zh-Hans = Cinese semplificato
family-lineage = Filiazione
links-relation = Tipo di legame
occupations-position = Posizione ricoperta
field-link-relation = Tipo di legame
field-link-relation-hint = Uno dei legami che AXGF 1.1 nomina. La denominazione sopra conserva le parole dell'atto.
field-occupation-position = Posizione ricoperta
field-occupation-position-hint = La posizione all'interno dell'attività: Direttrice, quando l'attività è Maestra.
error-delete-changed-title = Modificato da quando l'avete guardato
error-delete-changed-detail = Questo record è stato salvato di nuovo dopo che è stata mostrata la pagina da cui lo state eliminando; ora è alla versione { $version }. Non è stato eliminato nulla. Guardatelo com'è ora prima di decidere di nuovo.
error-delete-changed-look = Guardarlo di nuovo
documents-files = File allegati qui
documents-files-help = Modificate i dettagli di un file, oppure eliminate il file stesso. L'eliminazione toglie il documento e i suoi byte dall'archivio, per tutti quelli a cui è allegato; per staccarlo solo da questa persona, svuotate la sua riga qui sopra.
documents-edit-details = Modifica dettagli
documents-delete = Elimina questo file

## Charts

radar-section = Grafici dal record
radar-section-help = Tre letture di ciò che contiene questo record, ogni asse da 0 a 100. Ogni numero è calcolato dai fatti elencati accanto, secondo regole scritte nella documentazione dell'applicazione; nulla viene salvato, nulla viene indovinato, e un asse da cui non c'è nulla da leggere resta vuoto invece di ricevere un punteggio medio.
radar-physique = Fisico
radar-mind = Temperamento e mente
radar-vitality = Salute e vitalità
radar-axis-stature = Statura
radar-axis-build = Corporatura
radar-axis-lean-mass = Massa magra
radar-axis-posture = Postura
radar-axis-gait = Andatura
radar-axis-dentition = Denti
radar-axis-openness = Apertura
radar-axis-conscientiousness = Coscienziosità
radar-axis-extraversion = Estroversione
radar-axis-agreeableness = Gradevolezza
radar-axis-stability = Stabilità emotiva
radar-axis-cognition = Cognizione
radar-axis-circulation = Circolazione
radar-axis-breathing = Respirazione
radar-axis-metabolism = Metabolismo
radar-axis-illness = Assenza di malattie
radar-axis-senses = Sensi
radar-axis-rest = Sonno e umore
radar-folded-open = Mostra questo grafico
radar-folded-why = Questa persona risulta in vita. Il ritratto del temperamento di una persona in vita resta chiuso finché qualcuno che può leggerlo non chiede di vederlo.
radar-empty = Da questo record non si può ancora leggere nulla per questo grafico.
radar-table-caption = { $chart }: ogni asse, il suo punteggio e da cosa è stato letto
radar-col-axis = Asse
radar-col-score = Punteggio
radar-col-from = Letto da
radar-no-score = nessun punteggio
radar-from-none = nulla
record-link-outgoing = da questa persona
record-link-incoming = verso questa persona
# L'avviso operativo nel pannello di amministrazione.
health-attention = Richiede attenzione:
health-standing-token = Un token di amministrazione d'emergenza è ancora impostato. Accede scavalcando ogni account e ogni permesso — è ciò che serve il giorno in cui nessuno riesce ad accedere, non qualcosa da lasciare quando qualcuno ci riesce. Rimuovilo dal file di ambiente del servizio e riavvialo.
health-bundle-invalid = { $errors ->
        [one] I dati familiari non superano più la convalida: un errore, elencato sotto «Convalida».
        [many] I dati familiari non superano più la convalida: { $errors } errori, elencati sotto «Convalida».
       *[other] I dati familiari non superano più la convalida: { $errors } errori, elencati sotto «Convalida».
    }
health-disk-unknown = Non è stato possibile leggere lo spazio libero di questa macchina, quindi nulla qui può garantire che il prossimo salvataggio ci stia.
health-disk-no-room-to-save = Salvare non è possibile: sono liberi { $free } e ricostruire questo pacchetto richiede { $need }. Nulla è andato perduto e nulla potrà essere modificato finché non c'è spazio.
health-disk-critical = Il disco è libero al { $percent } % — restano { $free }. Il salvataggio smetterà di funzionare a breve.
health-disk-low = Il disco è libero al { $percent } % — restano { $free }. Da affrontare prima che diventi urgente.
health-backup-unconfigured = Non si sta salvando nulla. Un solo disco guasto porterebbe via ogni dato.
health-backup-never = È impostata una cartella di backup, ma non vi è mai stato scritto alcun backup.
health-backup-stale = { $days ->
        [one] Il backup più recente è di un giorno. Ne va scritto uno al giorno.
        [many] Il backup più recente è di { $days } giorni. Ne va scritto uno al giorno.
       *[other] Il backup più recente è di { $days } giorni. Ne va scritto uno al giorno.
    }
health-cache-missing = { $missing ->
        [one] Uno dei { $declared } file allegati manca dalla cache e non sarà scaricabile finché il prossimo salvataggio non lo ricostruisce.
        [many] { $missing } dei { $declared } file allegati mancano dalla cache e non saranno scaricabili finché il prossimo salvataggio non li ricostruisce.
       *[other] { $missing } dei { $declared } file allegati mancano dalla cache e non saranno scaricabili finché il prossimo salvataggio non li ricostruisce.
    }
