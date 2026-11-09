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
account-emergency = Token di emergenza
footer-credit = © 2026 · Progettato e sviluppato da { $author }
footer-format-link = Il formato AXGF su GitHub

## Preferenze

settings-title = Impostazioni
edit-mode-label = Modifica: ciò che salvi cambia le schede della famiglia
display-menu-label = Lingua e aspetto
display-settings = Tema e aspetto…
settings-tab-theme = Tema
settings-tab-language = Lingua
settings-tab-appearance = Aspetto
settings-done = Fatto
prefs-language = Lingua
prefs-theme = Tema
prefs-background-on = Una velatura di colore dietro la pagina
prefs-apply = Applica
busy-saving = Salvataggio…
busy-working = In corso…
prefs-machine = automatica, { $coverage }%
prefs-machine-complete = completa, non ancora riletta

theme-canopee = Chioma
theme-canopee-note = scuro, verde bosco · predefinito
theme-fusion = Fusione
theme-fusion-note = scuro, neon · un albero vivo
theme-galaxy = Galassia
theme-galaxy-note = spazio profondo · un sistema solare
theme-summit = Vetta
theme-summit-note = scuro, alba dorata · un'ascesa

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

record-identity-help = Ogni nome registrato con il suo tipo, il periodo in cui fu usato e la fonte che lo sostiene, con la scrittura originale accanto alla traslitterazione latina dove differiscono, più genere, stato in vita e visibilità.
record-life-events-help = Nascita, morte e ogni evento a cui questa persona ha preso parte, in ordine di data, ciascuno con il suo ruolo — così un matrimonio a cui fece solo da testimone sta accanto al proprio. Un fatto senza data va in fondo, invece di fingere di venire per primo.
record-family-help = Genitori e fratelli, poi ogni unione con il suo tipo, le date, il luogo, il modo in cui finì e i figli in ordine di nascita.
record-other-relationships-help = Ogni legame che ha questa persona a uno dei due capi, letto dal suo lato: la stessa scheda appare come «padrino di» da un capo e «figlioccio di» dall'altro.
record-places-help = Ogni luogo che questa scheda tocca, con quel che vi accadde e con la storia dei confini che rende un luogo comprensibile nel tempo.
record-sources-documents-help = Ogni fonte elenca i fatti di questa pagina che vi si appoggiano, in ordine di forza della prova.
record-notes-help = Note su questa scheda, compreso il testo che nessun convertitore ha saputo interpretare e che è stato conservato alla lettera invece di essere scartato.
record-history-help = Ogni modifica salvata di questa scheda, dalla più recente. Chi ha corretto cosa è un fatto sulle persone che tengono l'albero, non sulla famiglia che vi si trova: resta perciò fuori dall'archivio esportato ed è mostrato solo ai parenti che hanno effettuato l'accesso.
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
conf-percent = { $percent }%
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
error-merged-title = Questa scheda non esiste più.
error-merged-detail = È stata unita a { $name } il { $date }. I link al vecchio indirizzo portano ora alla scheda unita.
error-merged-open = Apri la scheda unita
error-back-to-river = Torna al fiume
view-tree-back = Torna all’albero
view-galaxy-back = Torna alla galassia
view-summit-back = Torna alla montagna
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
setup-title = Avviare questo archivio
setup-lede = Questa installazione non contiene ancora nessuna famiglia. Scegliete come comincia.
setup-door-archive-title = Da un archivio AXGF
setup-door-archive-detail = Un file .axgf proveniente da un’altra installazione o da un backup. Persone, fonti e fotografie arrivano così come sono.
setup-door-archive-file = Archivio (.axgf)
setup-door-archive-go = Avvia con questo archivio
setup-limit-gb = Fino a { $n } GB, scritti direttamente su disco.
setup-door-scratch-title = Da zero
setup-door-scratch-detail = Un archivio vuoto. Aggiungete la prima persona, poi la sua famiglia, scheda dopo scheda.
setup-door-scratch-go = Aggiungi la prima persona
setup-door-gedcom-title = Da un file GEDCOM
setup-door-gedcom-detail = Il file che ogni programma di genealogia esporta: GEDCOM 5.5, 5.5.1, 5.5.5 o 7.0, in .ged o .gdz, in qualunque codifica. Vedete come è stato letto e cosa passa prima che si conservi qualcosa.
setup-door-gedcom-file = File GEDCOM (.ged, .gdz)
setup-door-gedcom-go = Leggi questo file
setup-never-over = Quando una famiglia c’è già, questa pagina la sostituisce solo dopo un backup verificato e le parole che lo confermano.
setup-adopt-title = Farne la famiglia dell’archivio
setup-adopt-detail = Schede che diventerebbero la famiglia di questa installazione: { $total }. Nulla viene conservato prima che premiate il pulsante.
setup-adopt-go = Avvia l’archivio con questa famiglia
setup-adopt-back = Scegli un’altra via
setup-done-title = L’archivio è avviato
setup-done-detail = Schede nella famiglia di questa installazione ora: { $total }.
setup-done-photos = Fotografie e documenti arrivati con essa: { $n }.
setup-done-mismatches = File il cui contenuto non corrisponde all’impronta registrata: { $n }. Sono conservati; controllateli.
setup-done-river = Apri il fiume
view-tree-open = Apri l’albero
view-galaxy-open = Apri la galassia
view-summit-open = Apri la montagna
setup-done-admin = Amministrazione
setup-refused-title = Questo non è potuto diventare l’archivio
setup-refused-detail = Non è stato scritto nulla. Ciò che lo ha impedito:
setup-closed-title = Questo archivio è già avviato
setup-closed-detail = Nel frattempo è arrivata una famiglia, e non è stato scritto nulla. Riaprite la pagina d’importazione: su una famiglia chiede una conferma e prima ne fa il backup.
setup-not-archive = Questo non è un archivio .axgf.
setup-archive-too-large = Questo archivio supera la dimensione consentita per un invio.
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
admin-roles-note = Verificare, unire i doppioni, esportare, eliminare e gli account sono riservati agli amministratori.
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
admin-no-changes-yet = Ancora nessuna modifica. Ogni salvataggio è registrato in { $path }.
admin-last-validation = Ultima verifica
admin-bundle-heavy = Quest'archivio pesa { $size }. Viene caricato per intero all'avvio e tenuto in memoria, quindi oltre circa { $warn } il sito comincia a costare memoria vera e i riavvii diventano lenti. Va bene per un archivio di famiglia, non per una mediateca: se gli allegati crescono senza limite, teneteli in un deposito di file e fate che l'archivio vi punti.

admin-fields = Campi
admin-raw-json = JSON grezzo
form-more-fields = { $n ->
        [one] Un altro campo
        [many] Altri { $n } campi
       *[other] Altri { $n } campi
    }
admin-raw-json-help = L'elemento intero, così nulla è immodificabile: elenchi come i partner e i figli di una famiglia, o la storia dei confini di un luogo, stanno proprio qui. Questo è il documento di partenza; i campi qui sopra vengono poi scritti sui percorsi che possiedono, quindi modificate un valore o in un posto o nell'altro, non in entrambi. Deve leggersi come JSON, altrimenti non si salva nulla.
admin-save = Salva
admin-cancel = Annulla
place-editor-title = Modifica un luogo
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
person-tab-life = Eventi
person-tab-media = Media
person-tab-tree = Albero
person-tab-history = Modifiche
person-tab-afg = AFG
person-tab-status = Stato
status-check = Da verificare
status-know = Da sapere
status-fix = Correggi
status-open = Apri
status-dismissed = Segnati «normale per noi»: { $n }
status-panel = Da verificare: { $n }
person-afg-title = Chiedi all'AFG di { $name }
person-afg-title-short = Conversazioni con l'AFG
person-afg-lede = Chiedi della vita di { $name }, della sua famiglia, della sua epoca. Le tue conversazioni qui sono solo tue.
person-afg-reads = Letto a ogni messaggio: la scheda, le foto ({ $photos }), i PDF ({ $pdfs }), le note ({ $notes }), i parenti stretti ({ $relatives }), la famiglia ({ $persons }) · circa { $tokens } token.
person-afg-ask = La tua domanda su { $name }
person-afg-example = Per esempio: «In quale reggimento era { $name } nella battaglia di cui parlano le note?»
person-afg-conversations = Le tue conversazioni
person-afg-none = Ancora nessuna conversazione: fai una prima domanda.
person-afg-all = Tutte le tue conversazioni
person-afg-back = Torna alle tue conversazioni
person-afg-gone = Questa conversazione non esiste, o non è tua.
person-afg-delete-all = Elimina tutte le tue conversazioni su { $name }
person-afg-delete-all-confirm = Voglio eliminarle tutte.
person-afg-deleted-all = Le tue conversazioni su questa persona sono state eliminate.

admin-delete = Elimina
admin-not-set = — non impostato —
admin-edit = Modifica
list-select-row = Seleziona { $name }
list-kind-label = Tipo
list-select-page = Seleziona tutte le righe di questa pagina
list-selected = { $n ->
        [one] { $n } selezionata
        [many] { $n } selezionate
       *[other] { $n } selezionate
    }
list-export = Esporta
list-escape = Esc per deselezionare
bulk-confirm-title = { $n ->
        [one] Eliminare { $n } scheda?
        [many] Eliminare { $n } schede?
       *[other] Eliminare { $n } schede?
    }
bulk-confirm-detail = Una scheda a cui un’altra rimanda ancora viene rifiutata, e allora non si elimina nulla.
bulk-confirm-go = Eliminale
bulk-deleted = { $n ->
        [one] { $n } scheda eliminata.
        [many] { $n } schede eliminate.
       *[other] { $n } schede eliminate.
    }
bulk-refused = Non è stato eliminato nulla: { $name } è ancora citato altrove. Rimuovi prima quel rimando, poi riprova.
bulk-changed = { $n ->
        [one] { $n } scheda è cambiata da quando è stato mostrato l’elenco, quindi non è stato eliminato nulla. Ricarica l’elenco e riprova.
        [many] { $n } schede sono cambiate da quando è stato mostrato l’elenco, quindi non è stato eliminato nulla. Ricarica l’elenco e riprova.
       *[other] { $n } schede sono cambiate da quando è stato mostrato l’elenco, quindi non è stato eliminato nulla. Ricarica l’elenco e riprova.
    }
admin-range = { $from }–{ $to } di { $total }
list-filiation = Filiazione
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
accounts-lede = Salvato in { $path } (modo 600), accanto all'archivio e mai dentro.
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
accounts-no-registration = I visitatori non possono aprire account da soli; la registrazione si imposta più in alto.
accounts-password-hint = Vuoto: ne viene generata una, mostrata una volta. Altrimenti almeno { $min } caratteri.
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
accounts-emergency-warning = Sei entrato con il token di emergenza, non con un account: crea qui sotto un account vero e usalo.
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
upload-file-too-large = Questo file supera il limite di { $mb } MB e non verrà inviato. Scegline uno più piccolo.
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
tree-contradicts-detail = Questi legami restano fuori dalla numerazione e alcune righe possono essere fuori posto. Correggi la scheda sbagliata.
tree-contradicts-pair = Coppia e genitore-figlio insieme:
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

home-empty = Ancora nulla di registrato.
home-count = { $total ->
        [one] Una scheda
       *[other] { $total } schede
    }, in un solo file che appartiene alla famiglia.
home-unnamed-family = Quest'albero di famiglia
home-in-this-tree = Che cosa ha registrato finora la famiglia
home-showcase-title = Dove quest'albero dice già più di nomi e date
home-showcase-example = Vedi un esempio →
home-nothing-title = Ancora nulla da mostrare.
home-nothing-admin = Questa installazione non contiene ancora nessuna famiglia: avviatela da un archivio, da un file GEDCOM o da zero.
home-nothing-visitor = Questo archivio di famiglia è in preparazione.
home-eyebrow = Archivio di famiglia
home-hero-1 = Ogni vita è una sorgente.
home-hero-2 = Ogni famiglia, un fiume.
view-tree-hero-1 = Ogni vita è una foglia.
view-tree-hero-2 = Ogni famiglia, un albero.
view-galaxy-hero-1 = Ogni vita è una stella.
view-galaxy-hero-2 = Ogni famiglia, una galassia.
view-summit-hero-1 = Ogni vita è un passo.
view-summit-hero-2 = Ogni famiglia, un'ascesa.
home-cta-river = Esplora il fiume
view-tree-cta = Esplora l’albero
view-galaxy-cta = Esplora la galassia
view-summit-cta = Sali sulla montagna
home-cta-tree = Vedi l’albero
home-scroll = Scorra per esplorare
home-trust-title = Affidabilità delle filiazioni
home-trust-certain = praticamente certe
home-trust-high = ben documentate
home-trust-medium = plausibili
home-trust-low = ipotetiche
home-trust-unrated = non ancora valutate
home-showcase-eyebrow = Oltre i nomi e le date
home-cta-title = Insieme, custodiamo la memoria della famiglia.
home-cta-detail = Ogni nome aggiunto e ogni fonte verificata rendono il fiume più fedele per chi verrà.
view-tree-cta-detail = Ogni nome aggiunto e ogni fonte verificata rendono l’albero più fedele per chi verrà.
view-galaxy-cta-detail = Ogni nome aggiunto e ogni fonte verificata rendono la galassia più fedele per chi verrà.
view-summit-cta-detail = Ogni nome aggiunto e ogni fonte verificata rendono la montagna più vera per chi verrà dopo.
home-cta-add = Aggiungi una persona

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
record-deceased = { $gender ->
        [F] defunta
        [M] defunto
       *[other] defunto/a
    }
record-open-full-page = Apri la pagina intera ↗
record-open-full-title = Apri la pagina autonoma e condivisibile
record-edit = Modifica
record-see-in-river = Vedi nel fiume
view-tree-see = Vedi nell’albero
view-galaxy-see = Vedi nella galassia
view-summit-see = Vedi sulla montagna
panel-empty = Scegliete una scheda per vedere qui il documento completo di quella persona.
person-visibility-inline = visibilità:
person-age-at-death = { $gender ->
        [F] { $n ->
            [one] morta a { $n } anno
           *[other] morta a { $n } anni
        }
       *[other] { $n ->
            [one] morto a { $n } anno
           *[other] morto a { $n } anni
        }
    }
person-age-now = { $n ->
        [one] { $n } anno
       *[other] { $n } anni
    }
person-born-in = { $gender ->
        [F] nata a { $place }
       *[other] nato a { $place }
    }
person-died-in = { $gender ->
        [F] morta a { $place }
       *[other] morto a { $place }
    }
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
record-clips = Filmati e audio
record-documents = Documenti
record-file = File
record-status = Stato
record-size = Dimensione
record-absent-document = Nominato da questa persona ma non conservato qui.
record-upload-help-short = Fino a { $mb } MB. L'SVG è rifiutato.
record-verbatim-note = Conservato esattamente come lo dava la scheda, perché nessun convertitore ha saputo interpretarlo.
record-file-to-attach = File da allegare
record-document-type = Tipo di documento
record-caption = Didascalia
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
completeness-dates-note = Le date restano come sono state date, al giorno o al decennio; un testo che non è una data resta parola per parola.
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
convert-lede = Importa un albero da un file GEDCOM. Non si salva nulla, e l'albero del sito non cambia.
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
panel-selected-person = Persona scelta

## Fasce delle generazioni

tree-band-generation = Generazione { $g }
tree-level-self = Stessa generazione
tree-level-up = { $n ->
        [1] Figli
        [2] Nipoti
        [3] Pronipoti
       *[other] Discendenti, { $n }ª generazione
    }
tree-level-down = { $n ->
        [1] Genitori
        [2] Nonni
        [3] Bisnonni
       *[other] Antenati, { $n }ª generazione
    }
tree-card-centre = Centra l’albero qui
tree-card-same = La stessa persona, disegnata una seconda volta
# The tree page's legend: what a line to a child says (2026-10-28).
tree-key-lines = Linee:
tree-key-son = Figlio
tree-key-daughter = Figlia
tree-key-adoptive = Adozione
tree-key-foster = Affido o tutela
tree-key-step = Figliastro
tree-key-union = Unione
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

note-links = Relazioni fuori dalla famiglia: { $n }
note-occupations = Mestieri datati: { $n }
note-birth-imprecise = Data di nascita imprecisa nella fonte
note-death-imprecise = Data di morte imprecisa nella fonte
note-names = Nomi registrati: { $n }
note-transliteration = Un nome nella sua scrittura originale, con la traslitterazione
note-witnessed = Eventi come testimone: { $n }

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
completeness-metric-confidence-many = { $with } fatti su { $slots } hanno un grado: { $assessed } si scostano da { $modal }, su { $distinct } livelli.

completeness-metric-parentage = Quanto è certo ogni legame genitore-figlio
completeness-metric-parentage-none = Nessuna discendenza qui dice quanto sia certa. Adozioni, linee contestate e ricostruzioni da una sola menzione sono proprio i punti in cui una famiglia ha bisogno di registrare il dubbio — e l'albero disegna un legame meno certo con una linea più pallida.
completeness-metric-parentage-some = Filiazioni con un proprio grado: { $n }

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
completeness-metric-occupations-some = { $span } su { $total } hanno un inizio o una fine.

completeness-metric-sources = Fonti con il grado di affidabilità registrato
completeness-metric-sources-none = Nessuna fonte registrata. Dire da dove viene un fatto è ciò che permette a un parente di verificarlo più tardi — o di non essere d'accordo e dire perché.
completeness-metric-sources-some = { $graded } su { $total } dicono quanto sono solide.

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
admin-export-health-note = L'esportazione semplice esclude ogni categoria sensibile e il comportamento dei viventi. Spunta ciò che questo file deve contenere.
avatar-picker-title = Scegli un'immagine
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
ai-badge = IA
ai-portraits-title = Ritratti rielaborati dall'IA
ai-portraits-band = Ritocchi
ai-portraits-upload = Carica immagini rielaborate con l'IA
ai-portraits-manage = Gestisci le immagini IA
ai-portraits-intro = Immagini che un'IA ha ricavato dalle fotografie di questa persona: come poteva apparire. Compaiono accanto al volto nella scheda, segnate «IA», e le vedono solo i membri che hanno effettuato l'accesso. Non sono mai l'immagine della persona, mai tra i suoi file, mai una prova.
ai-portraits-current = Nella scheda
ai-portraits-add = Aggiungi immagini — spazio per { $n }
ai-portraits-files = Immagini (PNG, JPEG, WebP o GIF), anche più d'una alla volta
ai-portraits-generator = Realizzate con
ai-portraits-from = A partire da
ai-portraits-from-none = Non indicato
ai-portraits-remove = Rimuovi
ai-portraits-alt = Ritratto di { $name } rielaborato da un'IA
ai-portraits-not-evidence = Realizzate da un'IA a partire dalle fotografie: non sono immagini dell'epoca.
ai-portraits-full = La scheda ha già le sue tre immagini IA. Rimuovine una per aggiungerne un'altra.
ai-portraits-too-many = Una scheda contiene al massimo tre immagini IA. Rimuovine una prima di aggiungerne un'altra.
ai-portraits-images-only = Un ritratto IA deve essere un'immagine: PNG, JPEG, WebP o GIF.
ai-portraits-stale = La scheda è cambiata mentre questa pagina era aperta. Non è stato salvato nulla: riaprila e ripeti.
ai-portraits-back = Torna alle immagini IA

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
events-edit-link = Modifica gli eventi

## Presentation styles: density, never colour

prefs-style = Densità
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
profile-group-filled = { $n ->
        [one] { $n } compilato
        [many] { $n } compilati
       *[other] { $n } compilati
    }
profile-problems = Una parte dei dati inseriti non è stata salvata. Ogni problema è indicato accanto al suo campo, e non è stato scritto nulla.
profile-editor-withheld = Questa sezione contiene anche, per questa persona, dati della categoria { $classes } che non puoi leggere. Non sono mostrati qui, e salvare il modulo li lascia invariati.
profile-living-class-note = Questa persona risulta in vita. Quello che inserisci qui in una categoria sensibile è visibile solo agli amministratori.
profile-relationships-elsewhere = Parenti, padrini e testimoni stanno in famiglie, legami ed eventi: cambiarli qui cambia anche quelle schede.
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
diff-section-other = un altro campo
diff-section-avatar = Foto ritratto
diff-section-partners = Partner
diff-section-ai = Assistenza IA
diff-section-birth = Nascita
diff-section-civil-status = Stato civile
diff-section-documents = Documenti allegati
diff-section-extensions = Dati dell’applicazione
diff-section-children = Figli
diff-section-union = Unione
diff-section-date = Data
diff-section-participants = Partecipanti
diff-section-from = Collegato da
diff-section-to = Collegato a
diff-section-valid-from = Valido dal
diff-section-valid-until = Valido fino al
diff-section-visibility = Visibilità
diff-section-employer = Datore di lavoro
diff-section-title-normalized = Titolo normalizzato
diff-section-conflicts = Fonti in contrasto
diff-section-dna = DNA
diff-section-document = Documento
diff-section-language = Lingua
diff-section-place = Luogo
diff-section-repository = Archivio
diff-section-script = Scrittura
diff-section-coordinates = Coordinate
diff-section-country-history = Storia dei paesi
diff-section-identifiers = Identificativi
diff-section-names = Nomi
diff-section-file = File
diff-section-linked-to = Allegato a
diff-section-ocr = Testo riconosciuto
diff-section-format-version = Versione del formato
history-created = ha creato
history-deleted = ha eliminato
history-merged = ha unito a un’altra scheda
history-setup = ha avviato l’archivio
history-attached = ha allegato un file
admin-raw-json-unparsed = Il JSON grezzo non si è potuto leggere ({ $error }). Non è stato salvato nulla.
admin-raw-json-dangling = Il JSON grezzo fa puntare { $field } a { $target }, che non è in questo archivio. Sceglilo con un campo qui sopra o correggi l’identificativo. Non è stato salvato nulla.
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
gedcom-read-title = Come è stato letto il file
gedcom-read-as = Letto come GEDCOM { $version }, in { $encoding }.
gedcom-read-as-unversioned = Letto come GEDCOM (l’intestazione non indica una versione), in { $encoding }.
gedcom-read-declared = L’intestazione dichiara { $declared }; i byte dicono altro, e sono stati letti per quello che sono.
gedcom-read-gedzip = Un archivio GEDCOM 7 (.gdz): ne è stato letto l’insieme di dati. Altri file contenuti (fotografie, scansioni), non ripresi: { $n }.
gedcom-read-crs = Righe terminate alla vecchia maniera Macintosh, riparate: { $n }.
gedcom-read-voids = Riferimenti a @VOID@ — il «nessun record» di GEDCOM 7 — messi da parte invece di essere letti come persone: { $n }.
gedcom-read-notes = Note condivise (SNOTE) lette come note: { $n }.
gedcom-read-stray = Byte che non erano UTF-8 in questo file UTF-8, letti uno a uno come Windows-1252: { $n }.
gedcom-read-tidied = Righe disposte in un modo che il convertitore non legge (rientrate, tabulazioni o spazi in più tra i campi, tag in minuscolo, spazio prima di una giunzione CONC, carattere di controllo, segno di calendario gregoriano), riparate: { $n }.
gedcom-read-too-deep = Righe annidate più a fondo di qualsiasi struttura GEDCOM, messe da parte con ciò che contenevano: { $n }.
convert-error-no-dataset = Questo zip non contiene né gedcom.ged né un unico file .ged da leggere al suo posto.
convert-error-refused = Il convertitore ha rifiutato questo file. Le diagnosi qui sotto dicono perché. Non è stato conservato nulla.
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
diag-merge_refused = Le due schede non possono essere unite così come sono.
diag-merge_unresolved = Un disaccordo tra le due schede non è stato deciso.
diag-merge_unsupported = Le schede di questo tipo non possono essere unite.
diag-stale_plan = Una delle schede è cambiata dopo il piano: rifai il piano.
field-person-display-name = Nome visualizzato
field-person-display-name-hint = Il nome mostrato ovunque nel sito.
field-person-gender = Genere
field-person-living = In vita
field-person-birth-date = Data di nascita
field-date-value-hint = 1923, 1923-04 o 1923-04-12; vuoto se sconosciuta.
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
health-standing-token = Un token di emergenza è ancora impostato e scavalca ogni account. Toglilo dal file d'ambiente del servizio e riavvia.
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

## River

nav-river = Fiume
nav-home = Famiglia
river-aria = Il fiume attorno a { $name }: antenati in basso, discendenti in alto.
river-band-living = VIVENTI
river-circa = c.{" "}
river-trail = Percorso
river-back = ← Indietro
river-range = Portata
river-motion = Movimento
river-travel = viaggio
river-cut = stacco
river-legend-attested = attestato
river-legend-inferred = dedotto
river-legend-speculative = supposto
river-legend-lost = fine della documentazione
river-legend-repeat = ×n raggiunto da n linee, disegnato sotto ciascuna
river-legend-again = ↔ ridisegnato accanto a un partner successivo
river-legend-discharge = discendenti
river-legend-direction = ↓ verso la sorgente · ↑ verso i vivi
view-tree-direction = ↓ le radici: gli antenati · ↑ la chioma: i discendenti
view-galaxy-direction = ↓ gli antenati sotto il sole · ↑ i discendenti sopra
view-summit-direction = ↓ gli antenati nella valle · ↑ i discendenti verso la vetta
river-legend-generation = una generazione ≈ 29 anni
river-zoom = Zoom
river-still = Ferma il paesaggio
river-world-label = La valle della famiglia, in tre dimensioni
river-world-controls = Muoversi nella valle
river-world-forward = Avanti
river-world-back = Indietro
river-world-left = A sinistra
river-world-right = A destra
river-world-turn-left = Girare a sinistra
river-world-turn-right = Girare a destra
river-world-higher = Vedere più dall’alto
river-world-lower = Vedere più dal basso
river-world-closer = Avvicinarsi
river-world-farther = Allontanarsi
river-world-home = Tornare alla persona
river-world-hint = Trascinare per muoversi, rotella per avvicinarsi, clic destro o due dita per girare, doppio clic per andare in un punto.
view-tree-still = Ferma l’albero
view-galaxy-still = Ferma il cielo
view-summit-still = Ferma la montagna
river-whole = Tutta la famiglia
river-names-all = Tutti i nomi
# A tap on a person in a view, on a touch screen: the drawing moves onto them.
river-pick-centre = Centra qui
river-zoom-hint = Zoom: Ctrl + rotellina, pizzico o + / − · ingrandito, trascina per spostarti
river-tab-sources = Fonti
river-legend-title = Come leggere il fiume
view-tree-name = Albero vivo
view-tree-legend = Come leggere l’albero
view-galaxy-name = Galassia
view-galaxy-legend = Come leggere la galassia
view-summit-name = Ascesa
view-summit-legend = Come leggere la montagna
hud-ancestors = Antenati
hud-descendants = Discendenti
hud-generations = Generazioni
hud-living = Viventi
hud-attested = Legami attestati

# Masthead search
search-label = Cerca una persona
search-short = Persona
date-qualifier = Qualificatore
date-qualifier-circa = circa
search-title = Ricerca
search-heading = Persone che corrispondono a «{ $q }»
search-none = Nessuna persona visibile ha questo nome.
search-open-record = Apri la scheda
search-hint = Invio porta il fiume fino a lei; Maiusc+Invio apre la sua scheda.
view-tree-search-hint = Invio fa crescere l’albero intorno a lei; Maiusc+Invio apre la sua scheda.
view-galaxy-search-hint = Invio ne fa il sole; Maiusc+Invio apre la sua scheda.
view-summit-search-hint = Invio porta fino a quella persona; Maiusc+Invio apre la sua scheda.
search-summary = { $n ->
        [one] { $n } persona
        [many] { $n } persone
       *[other] { $n } persone
    } · ↵ viaggiare · ⇧↵ aprire la scheda
search-empty-query = Scrivi un nome per cercare.

# Data quality (/quality)
nav-quality = Qualità dei dati
quality-title = Cose da guardare
quality-none = Niente da guardare: tutti i controlli sono superati.
quality-intro = Punti da verificare; spesso c'è una buona ragione. Apri la scheda o segna il punto come verificato.
quality-groups-label = Tipi di rilievo
quality-all = Tutto
quality-show-label = Mostra
quality-show-open-option = Da verificare
quality-download = Scarica
quality-group-names = Nomi
quality-group-relations = Parentele
quality-group-dates = Date
quality-group-places = Luoghi
quality-group-evidence = Prove
quality-export-csv = Scarica come CSV
quality-export-json = Scarica come JSON
quality-open = Apri
quality-none-file = Niente da guardare in questo file.
quality-show-list = Mostra l’elenco
quality-after-import = Nel file non è stato cambiato nulla. Quando sarà l’archivio del sito, la pagina Qualità dei dati elencherà le stesse cose con un collegamento a ogni scheda.
quality-file-list-cut = Qui sono elencate solo le prime { $shown }. Quando il file sarà l’archivio del sito, la pagina Qualità dei dati le elencherà tutte.
quality-rule-name-sentence = Una frase in un campo del nome
quality-name-sentence = «{ $value }» sembra una frase, non un nome.
quality-rule-union-parent-child = Una coppia che è anche genitore e figlio
quality-union-parent-child = { $parent } e { $child }: coppia e genitore-figlio insieme.
quality-rule-child-listed-twice = Un figlio registrato due volte
quality-child-listed-twice = { $child } compare due volte tra i figli.
quality-rule-duplicate-union = La stessa coppia registrata due volte
quality-duplicate-union = { $a } e { $b }: { $count } unioni registrate.
quality-rule-partner-twice = Una persona su entrambi i lati di un’unione
quality-partner-twice = { $person } è su entrambi i lati dell'unione.
quality-rule-foreign-surnames = Un figlio con un cognome diverso da entrambi i genitori
quality-foreign-surnames = Cognome diverso da quello di entrambi i genitori: { $who }.
quality-rule-death-before-birth = Morte prima della nascita
quality-death-before-birth = { $person }: morte ({ $death }) prima della nascita ({ $birth }).
quality-rule-child-before-parent = Un figlio nato prima di un genitore
quality-child-before-parent = Nascita di { $child } ({ $childborn }) prima di quella del genitore { $parent } ({ $parentborn }).
quality-rule-parent-too-young = Un genitore con meno di dodici anni
quality-parent-too-young = { $parent } aveva { $age } anni alla nascita di { $child }.
quality-rule-parent-too-old = Un genitore con più di sessantacinque anni
quality-parent-too-old = { $parent } aveva { $age } anni alla nascita di { $child }.
quality-rule-marriage-before-birth = Un matrimonio prima della nascita di un coniuge
quality-marriage-before-birth = Matrimonio ({ $married }) prima della nascita di { $person } ({ $born }).
quality-rule-isolated-record = Una scheda senza date né legami
quality-isolated-record = { $person }: né data né legame.
quality-rule-place-no-region = Un luogo senza regione
quality-place-no-region = { $place }: nessuna regione.
quality-rule-region-free-text = Una regione che sembra una nota
quality-region-free-text = Regione di { $place }: «{ $value }» sembra una nota.
quality-rule-place-near-duplicate = Luoghi che potrebbero essere lo stesso
quality-place-near-duplicate = Forse un solo luogo: { $names }.
quality-rule-not-a-place = Una voce che forse non è un luogo
quality-not-a-place = «{ $place }» forse non è un luogo.
quality-rule-certain-without-source = Certo, senza fonte
quality-certain-without-source = { $record }: certo ({ $claims }) senza fonte.
quality-dismiss = Verificato — normale per noi
quality-restore = Mostralo di nuovo
quality-note-label = Perché (facoltativo)
quality-show-dismissed = Verificati e mantenuti
quality-show-open = Torna all’elenco aperto
quality-dismissed-by = Verificato da { $by } il { $at }
quality-gone-title = Quel rilievo non c’è più
quality-gone-detail = La scheda è cambiata da quando l’elenco è stato aperto e il rilievo non è più lo stesso. Ricarica l’elenco e guarda di nuovo.
quality-summary = { $n ->
    [one] { $n } cosa da guardare
   *[other] { $n } cose da guardare
}
quality-summary-file = { $n ->
    [one] { $n } cosa da guardare in questo file
   *[other] { $n } cose da guardare in questo file
}
quality-dismissed-summary = { $n ->
    [one] { $n } rilievo verificato e mantenuto
   *[other] { $n } rilievi verificati e mantenuti
}
quality-dismiss-all = Segna tutto come verificato ({ $n })
quality-restore-all = Mostra di nuovo tutto ({ $n })
quality-bulk-conflict = Schede cambiate dall’apertura dell’elenco: { $n }. Non è stato segnato nulla; ricarica l’elenco e riprova.

## The record's editor (v0.4.2)

edit-tab-changed = modificato, non ancora salvato
edit-remove = Togli
edit-add-partner = Aggiungi un coniuge
edit-add-child = Aggiungi un figlio
edit-union-with = Unione con { $names }
edit-union-alone = Unione senza coniuge registrato
edit-child-of = Figlio di { $names }
edit-new-family = Crea una famiglia
edit-cite-title = Ciò che poggia su una fonte
edit-attach-to = Citarla per
edit-add-source = Aggiungi una fonte
edit-error-source-title = Una nuova fonte ha bisogno di un titolo. Non è stato salvato nulla.
edit-detach-file = Togli da questa scheda
edit-no-sources = Nessun dato proprio di questa persona cita ancora una fonte.
edit-no-files = A questa scheda non è ancora allegato alcun file.
edit-saved = { $n ->
        [one] Salvato: una scheda scritta.
        [many] Salvato: { $n } schede scritte in un solo salvataggio.
       *[other] Salvato: { $n } schede scritte in un solo salvataggio.
    }
edit-saved-none = Non era cambiato nulla, quindi non è stato salvato nulla.
edit-back-to-record = Torna alla scheda
edit-avatar = Foto del profilo
edit-avatar-frame = Inquadra il volto
edit-avatar-upload = Usa come foto del profilo
edit-stale = Qualcun altro ha modificato { $what } da quando questa pagina è stata aperta. Non è stato salvato nulla; tutto ciò che ha inserito è ancora qui.
edit-refused = Non è stato salvato nulla: { $reason }

# Administration console (v0.5.0, docs/adr/0011)
error-cross-site-title = Rifiutato: inviato da un altro sito
error-cross-site-detail = Questo modulo proveniva da una pagina di un altro sito, quindi non è stato eseguito. Se l’ha inviato da questo sito, ricarichi la pagina e lo invii di nuovo.
admin-nav-label = Sezioni dell’amministrazione
admin-nav-overview = Panoramica
admin-nav-bundle = Archivio AXGF
admin-nav-infrastructure = Infrastruttura
admin-nav-accounts = Account
admin-nav-connectivity = Connettività

# Administration console: the AXGF bundle (v0.5.0, docs/adr/0011)
bundle-title = Archivio AXGF
bundle-lede = L'archivio della famiglia: { $path }. La sua conformità, ciò che sembra sbagliato e come prenderne una copia.
bundle-unset-title = Ancora nessuna famiglia.
bundle-unset-detail = Questa installazione contiene un archivio vuoto, quindi non c'è ancora nulla da verificare. Avviala da un archivio, da un file GEDCOM o da zero.
bundle-kpi-conformity = Conformità
bundle-verdict-ok = Conforme
bundle-verdict-warn = Da tenere d'occhio
bundle-verdict-fail = Non conforme
findings-tally = In verde: { $ok } · Avvisi: { $warn } · Critici: { $fail }
bundle-kpi-entities = Entità
bundle-kpi-entities-note = { $persons ->
    [one] 1 persona
    *[other] { $persons } persone
    } · { $files ->
    [one] 1 file allegato
    *[other] { $files } file allegati
    }
bundle-kpi-size = Dimensione su disco
bundle-kpi-members = { $n ->
    [one] 1 file nell'archivio
    *[other] { $n } file nell'archivio
    }
bundle-kpi-format = Formato
bundle-kpi-format-note = Questa versione legge e scrive AXGF { $current }.
bundle-conformity = Conformità
bundle-conformity-intro = Nove verifiche sull'archivio così com'è ora; nessuna lo modifica. Apri una riga per vedere cosa fare.
bundle-area-format = Formato
bundle-area-integrity = Integrità
bundle-area-privacy = Riservatezza
bundle-area-content = Contenuto
level-ok = In verde
level-warn = Avviso
level-fail = Critico
findings-what-to-do = Cosa fare
findings-go = Vai →
bundle-anomalies = Anomalie
bundle-anomalies-intro = Ciò che trova la pagina sulla qualità dei dati, regola per regola: ancora aperto e già segnato come verificato.
bundle-anomalies-rule = Regola
bundle-anomalies-open = Aperte
bundle-anomalies-checked = Verificate
bundle-anomalies-none = La pagina sulla qualità dei dati non trova nulla da controllare.
bundle-diagnostics = Diagnostica del validatore
bundle-facts = Caratteristiche
bundle-fact-path = File
bundle-fact-size = Dimensione
bundle-fact-spec = Formato
bundle-fact-schema = Schema incluso
bundle-fact-none = nessuno
bundle-fact-created = Creato
bundle-fact-updated = Ultima scrittura
bundle-fact-generator = Scritto da
bundle-fact-gedcom = Convertito da
bundle-export = Esporta
bundle-export-raw = L'archivio così com'è (.axgf)
bundle-export-raw-go = Scarica l'archivio
bundle-export-sealed = Cifrato per una persona (.axgf.gpg)
bundle-export-sealed-intro = Lo stesso archivio, come messaggio OpenPGP che solo chi possiede la chiave privata può aprire. Incolla la sua chiave pubblica: il testo che inizia con «-----BEGIN PGP PUBLIC KEY BLOCK-----».
bundle-export-sealed-go = Scarica cifrato
seal-recipient = Per
seal-fingerprint = Impronta
seal-algorithm = Algoritmi
seal-fips-yes = nell'insieme approvato FIPS
seal-fips-no = fuori dall'insieme approvato FIPS
seal-expires = Scadenza della chiave
seal-never = mai
seal-key-label = La chiave pubblica del destinatario
seal-key-hint = Una chiave RSA di almeno 2048 bit, o una chiave a curva ellittica. Verifica l'impronta con il titolare prima di inviare qualsiasi cosa.
seal-remember = Conserva questa chiave per la prossima volta
seal-check = Leggi solo la chiave
seal-fips-note = L'archivio è cifrato con AES-256 verso la chiave del destinatario. Gli algoritmi approvati FIPS sono RSA da 3072 bit in su e le curve NIST P-256, P-384 e P-521: chiedi P-384. La libreria usata qui non è di per sé un modulo validato FIPS 140-3.
seal-decrypt-note = Il destinatario lo apre con GnuPG:
seal-forget = Dimentica la chiave conservata
seal-key-kept = La chiave { $fingerprint } è conservata per la prossima esportazione cifrata.
seal-error-no-key = Incolla prima la chiave pubblica del destinatario.
seal-error-not-a-key = Questa non è una chiave pubblica OpenPGP. Incolla l'intero blocco, dalla riga BEGIN alla riga END, e mai una chiave privata.
seal-error-bad-signatures = Le firme proprie di questa chiave non si verificano: potrebbe essere stata alterata. Chiedine una copia nuova al titolare.
seal-error-expired = Questa chiave, o ogni sua chiave in grado di cifrare, è scaduta. Chiedi al titolare di prolungarla o di inviarne una nuova.
seal-error-revoked = Il titolare ha revocato questa chiave. Chiedigli quella nuova.
seal-error-no-encryption-key = Questa chiave può firmare ma non cifrare: non ha una sottochiave di cifratura. Chiedine una al titolare (gpg --quick-add-key … encr).
seal-error-too-weak = Questa chiave RSA ha { $bits } bit; un'esportazione ne richiede almeno { $min }.
seal-error-unsupported = L'algoritmo di questa chiave non è usato dall'esportazione: solo chiavi RSA o a curva ellittica.
seal-error-io = Non è stato possibile creare la copia cifrata: { $error }
date-time-utc = { $date }, { $time } UTC

# Administration console: the bundle's conformity (v0.5.0, docs/adr/0011)
bundle-check-format-title = Versione del formato
bundle-check-format-ok = AXGF { $version }, la versione che questa build legge e scrive.
bundle-check-format-missing = Il manifesto non indica alcuna versione AXGF.
bundle-check-format-missing-fix = L'archivio non è stato scritto da uno strumento AXGF, oppure è danneggiato. Ripristina l'ultimo backup valido (axgf-cms restore).
bundle-check-format-older = Scritto in AXGF { $version }; il prossimo salvataggio lo scriverà in AXGF { $current }.
bundle-check-format-older-fix = Niente da fare: la prossima modifica lo riscrive. Conserva una copia del vecchio file se un altro programma legge solo la { $version }.
bundle-check-format-newer = Scritto in AXGF { $version }, più recente della { $current } di questa build: salvare potrebbe perdere ciò che non capisce.
bundle-check-format-newer-fix = Aggiorna axgf-cms a una versione che legga AXGF { $version } prima che qualcuno modifichi (Infrastruttura, Versione).
bundle-check-manifest-title = I conteggi del manifesto
bundle-check-manifest-ok = I conteggi del manifesto corrispondono al contenuto dell'archivio.
bundle-check-manifest-drift = Il manifesto dichiara { $declared } dove l'archivio ne contiene { $actual } ({ $collection }).
bundle-check-manifest-fix = Non si perde nulla: i conteggi vengono riscritti al prossimo salvataggio. Se l'archivio viene da altrove, verifica che sia stato copiato per intero.
bundle-check-validation-title = Validazione
bundle-check-validation-ok = Il validatore della libreria non trova nulla di errato.
bundle-check-validation-errors = { $errors ->
    [one] 1 errore
    *[other] { $errors } errori
    }: l'archivio non è valido.
bundle-check-validation-errors-fix = Ogni diagnostica qui sotto indica la scheda e il campo. Correggi la scheda o ripristina l'ultimo backup valido.
bundle-check-validation-warnings = { $warnings ->
    [one] 1 avviso
    *[other] { $warnings } avvisi
    }. Non bloccano mai un salvataggio; ognuno segnala che qualcosa potrebbe non andare.
bundle-check-validation-warnings-fix = Leggili qui sotto. Per lo più sono rimandi a schede assenti dall'archivio, che un'importazione GEDCOM lascia spesso.
bundle-check-payloads-title = File allegati
bundle-check-payloads-none = Nessun file allegato.
bundle-check-payloads-ok = { $declared ->
    [one] L'unico file è presente.
    *[other] Tutti i { $declared } file sono presenti.
    }
bundle-check-payloads-missing = Mancano { $missing } file allegati su { $declared } nella cache.
bundle-check-payloads-fix = Il prossimo salvataggio ricostruisce la cache dall'archivio. Se mancano nell'archivio stesso, ripristinali da un backup.
bundle-check-binding-title = Account e archivio
bundle-check-binding-ok = Il file degli account è stato creato per questa famiglia.
bundle-check-binding-mismatch = Il file degli account è stato creato per { $expected }; questo archivio è { $found }.
bundle-check-binding-fix = Se l'archivio è stato ripristinato o rinominato di proposito, niente da fare. Altrimenti gli account di una famiglia si applicano all'albero di un'altra: rimetti accanto il .acl giusto.
bundle-check-living-title = I viventi, in pubblico
bundle-check-living-ok = Nessuna persona registrata come vivente è visibile ai visitatori.
bundle-check-living-public = { $n ->
    [one] 1 persona registrata come vivente è contrassegnata
    *[other] { $n } persone registrate come viventi sono contrassegnate
    } come pubbliche: qualsiasi visitatore ne legge la scheda.
bundle-check-living-fix = A meno che non abbiano acconsentito, imposta la loro visibilità su membri (Modifica, Identità).
bundle-check-special-title = Categorie particolari
bundle-check-special-none = Nessuna scheda contiene dati sanitari, biometrici, genomici, penali o comportamentali.
bundle-check-special-some = { $n ->
    [one] 1 scheda contiene
    *[other] { $n } schede contengono
    } dati sanitari, biometrici, genomici, penali o comportamentali. Per i viventi restano agli amministratori, qualunque cosa dica la scheda.
bundle-check-quality-title = Qualità dei dati
bundle-check-quality-ok = Niente da controllare.
bundle-check-quality-open = { $n ->
    [one] 1 cosa da controllare
    *[other] { $n } cose da controllare
    }, in { $groups ->
    [one] 1 gruppo
    *[other] { $groups } gruppi
    }.
bundle-check-quality-fix = Apri la pagina sulla qualità dei dati: ogni riga indica la scheda e cosa sembra sbagliato. Correggila o segnala come verificata.
bundle-check-size-title = Dimensione
bundle-check-size-ok = { $size }, sotto i { $warn } oltre i quali questa pagina avvisa.
bundle-check-size-heavy = { $size }, oltre { $warn }.
bundle-check-size-fix = L'archivio è della famiglia ed è libero di crescere. Tieni d'occhio disco e backup (Infrastruttura, Volumi) e alza --size-warn-mb se questa dimensione è prevista.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

# Administration console: the infrastructure's checks (v0.5.0, docs/adr/0011 §3)
infra-area-security = Sicurezza
infra-area-access = Accesso
infra-area-data = Dati
infra-area-capacity = Capacità
infra-area-release = Versione
infra-area-platform = Piattaforma
infra-verdict-ok = In salute
infra-verdict-warn = Richiede attenzione
infra-verdict-fail = Critico
duration-days = { $n ->
    [one] { $n } giorno
    [many] { $n } giorni
    *[other] { $n } giorni
    }
duration-hours = { $n ->
    [one] { $n } ora
    [many] { $n } ore
    *[other] { $n } ore
    }
duration-minutes = { $n ->
    [one] { $n } minuto
    [many] { $n } minuti
    *[other] { $n } minuti
    }
infra-transport-title = Come si raggiunge il sito
infra-transport-ok = Questa pagina è arrivata via HTTPS.
infra-transport-exposed = Questa pagina è arrivata in chiaro e il server ascolta su { $bind }, oltre questa macchina: password e cookie di sessione attraversano la rete in chiaro.
infra-transport-exposed-fix = Metti davanti un reverse proxy HTTPS (Caddy, nginx, HAProxy) e lega di nuovo il servizio a 127.0.0.1: riesegui bootstrap.sh con --bind 127.0.0.1:8080, oppure rimuovi il drop-in che lo ha allargato.
infra-transport-plain = In chiaro, da { $bind }: va bene solo su questa macchina.
infra-transport-plain-fix = Raggiungi il sito dal suo indirizzo HTTPS. Se un proxy termina il TLS, fagli inviare X-Forwarded-Proto: https perché questo server lo sappia.
infra-forwarded-title = Indirizzi inoltrati
infra-forwarded-ok = Il server ascolta solo su questa macchina, quindi l'indirizzo del client inoltrato da un proxy è affidabile per rallentare gli accessi falliti ripetuti.
infra-forwarded-exposed = Il server è raggiungibile direttamente e si fida dell'intestazione X-Forwarded-For: un client che la imposta da sé sfugge al rallentamento dopo gli accessi falliti.
infra-forwarded-fix = Lega il servizio a 127.0.0.1 dietro il proxy, oppure filtra la porta perché solo il proxy la raggiunga.
infra-cross-site-title = Invii da altri siti
infra-cross-site-ok = Nessun modulo è stato inviato qui da un altro sito nell'ultimo giorno.
infra-cross-site-refused = { $n ->
    [one] Nell'ultimo giorno è stato rifiutato un modulo inviato da un altro sito.
    [many] Nell'ultimo giorno sono stati rifiutati { $n } moduli inviati da altri siti.
    *[other] Nell'ultimo giorno sono stati rifiutati { $n } moduli inviati da altri siti.
    }
infra-cross-site-fix = Non è passato nulla. Se i rifiuti sono tuoi — lo stesso sito raggiunto da un secondo indirizzo — imposta l'indirizzo pubblico in Connettività, oppure usa solo quello.
infra-sso-title = Servizi di accesso
infra-sso-none = Nessun servizio di accesso è attivo; gli account entrano con una password.
infra-sso-ok = { $n ->
    [one] È attivo un servizio di accesso, e l'indirizzo pubblico è HTTPS.
    [many] Sono attivi { $n } servizi di accesso, e l'indirizzo pubblico è HTTPS.
    *[other] Sono attivi { $n } servizi di accesso, e l'indirizzo pubblico è HTTPS.
    }
infra-sso-no-https = È attivo un servizio di accesso, ma l'indirizzo pubblico non è HTTPS: i fornitori rifiutano di inviare la risposta a un indirizzo in chiaro, e l'accesso fallisce.
infra-sso-fix = In Connettività, imposta come indirizzo pubblico l'indirizzo https:// che usano i lettori.
infra-admins-title = Amministratori
infra-admins-none = Nessun account attivo può amministrare questo sito; solo il token di emergenza.
infra-admins-none-fix = Entra con il token di emergenza, crea un amministratore, poi togli il token.
infra-admins-one = Un solo amministratore. Se quella persona è assente o perde la password, nessuno può gestire il sito.
infra-admins-one-fix = Assegna il ruolo di amministratore a una seconda persona fidata, in Account.
infra-admins-ok = { $n ->
    [one] { $n } amministratore può accedere.
    [many] { $n } amministratori possono accedere.
    *[other] { $n } amministratori possono accedere.
    }
infra-token-title = Token di emergenza
infra-token-generated = Nessun token di emergenza è impostato.
infra-token-only-way = Il token di emergenza è l'unico accesso finché non esiste un account amministratore.
infra-token-standing = È ancora impostato un token di emergenza. Fa accedere scavalcando ogni account e ogni permesso.
infra-token-fix = Rimuovi AXGF_CMS_ADMIN_TOKEN dal file d'ambiente del servizio, poi riavvia il servizio.
infra-registration-title = Registrazione dei visitatori
infra-registration-closed = I visitatori non possono creare account; può farlo solo un amministratore.
infra-registration-secret = I visitatori possono creare un account membro se conoscono il segreto di registrazione.
infra-registration-open = Qualsiasi visitatore può creare un account membro.
infra-registration-open-fix = Va bene per un sito pensato per essere aperto. Altrimenti richiedi il segreto di registrazione, o chiudi la registrazione, in Account.
infra-failures-title = Accessi falliti
infra-failures-ok = { $n ->
    [one] { $n } accesso fallito nell'ultimo giorno.
    [many] { $n } accessi falliti nell'ultimo giorno.
    *[other] { $n } accessi falliti nell'ultimo giorno.
    }
infra-failures-many = { $n ->
    [one] { $n } accesso fallito nell'ultimo giorno: forse qualcuno sta tirando a indovinare le password.
    [many] { $n } accessi falliti nell'ultimo giorno: forse qualcuno sta tirando a indovinare le password.
    *[other] { $n } accessi falliti nell'ultimo giorno: forse qualcuno sta tirando a indovinare le password.
    }
infra-failures-fix = Ogni indirizzo viene già rallentato dopo fallimenti ripetuti. Se il numero continua a salire, cerca nel log del proxy da dove arrivano i tentativi e bloccali lì, e assicurati che ogni amministratore abbia una password lunga o acceda tramite un servizio di accesso.
infra-health-bundle-title = Dati della famiglia
infra-health-bundle-ok = L'archivio si apre e supera la validazione.
infra-health-bundle-fix = Apri la pagina Archivio AXGF per vedere gli errori e correggerli, oppure ripristina l'ultimo backup valido (axgf-cms restore).
infra-health-cache-title = Cache degli allegati
infra-health-cache-ok = Ogni file allegato è nella cache.
infra-health-cache-fix = Salva una scheda qualsiasi una volta: il salvataggio ricostruisce la cache. I file che mancano ancora dopo mancano nell'archivio stesso.
infra-health-backup-title = Backup
infra-health-backup-ok = { $count ->
    [one] Esiste un backup recente; è conservato { $count } archivio.
    [many] Esiste un backup recente; sono conservati { $count } archivi.
    *[other] Esiste un backup recente; sono conservati { $count } archivi.
    }
infra-health-backup-fix = Senza directory di backup, riesegui bootstrap.sh con --backup-dir. Se c'è, controlla il timer (systemctl status axgf-cms-backup.timer) e il suo log (journalctl -u axgf-cms-backup).
infra-disk-title = Spazio su disco
infra-disk-ok = { $free } liberi su { $total } ({ $percent }% libero).
infra-disk-fix = Libera spazio sulla partizione che contiene la directory dei dati — vecchi backup conservati anche altrove, log, file di altri programmi — oppure sposta i dati su un disco più grande.
infra-growth-title = Crescita
infra-growth-ok = { $days ->
    [one] Cresce di circa { $rate } al mese; c'è spazio per altro { $days } giorno a questo ritmo.
    [many] Cresce di circa { $rate } al mese; c'è spazio per altri { $days } giorni a questo ritmo.
    *[other] Cresce di circa { $rate } al mese; c'è spazio per altri { $days } giorni a questo ritmo.
    }
infra-growth-soon = { $days ->
    [one] Cresce di circa { $rate } al mese: a questo ritmo il disco è pieno fra { $days } giorno.
    [many] Cresce di circa { $rate } al mese: a questo ritmo il disco è pieno fra { $days } giorni.
    *[other] Cresce di circa { $rate } al mese: a questo ritmo il disco è pieno fra { $days } giorni.
    }
infra-growth-fix = Pianifica lo spazio ora: un disco più grande, meno backup sullo stesso disco, oppure i backup spostati su un altro.
infra-growth-flat = I backup non sono cresciuti; non c'è nulla da pianificare.
infra-growth-unknown = Non ci sono ancora abbastanza backup per misurare la crescita: serve almeno un giorno tra il più vecchio e il più recente.
infra-memory-title = Memoria
infra-memory-ok = { $used } usati sul limite di { $max } del servizio ({ $percent }%).
infra-memory-high = { $used } usati sul limite di { $max } del servizio ({ $percent }%): vicino al punto in cui il sistema ferma il servizio.
infra-memory-fix = Alza MemoryMax= in un drop-in per axgf-cms.service, oppure trova cosa trattiene la memoria — un'importazione molto grande, un allegato grande — e riavvia il servizio.
infra-memory-unlimited = { $used } in uso, senza un limite impostato per il servizio.
infra-version-title = Versione
infra-version-ok = v{ $current } è la versione pubblicata più recente.
infra-version-behind = È in esecuzione v{ $current }; è pubblicata { $latest }.
infra-version-behind-fix = Leggi le note di rilascio, poi aggiorna dalla sezione Versione qui sotto. Prima si fa un backup, e la versione precedente torna da sola se la nuova non si avvia.
infra-version-unknown = Non si sa se esiste una versione più recente: l'elenco delle versioni non è stato recuperato.
infra-version-unknown-fix = Usa «Cerca nuove versioni» qui sotto. La sandbox di produzione non raggiunge GitHub da sé; una volta installato l'assistente di aggiornamento, è lui a recuperare l'elenco ogni giorno.
infra-version-stale = { $days ->
    [one] L'elenco delle versioni è stato recuperato l'ultima volta { $days } giorno fa.
    [many] L'elenco delle versioni è stato recuperato l'ultima volta { $days } giorni fa.
    *[other] L'elenco delle versioni è stato recuperato l'ultima volta { $days } giorni fa.
    }
infra-upgrades-title = Aggiornamenti
infra-upgrades-ok = L'assistente di aggiornamento è installato: il pulsante qui sotto può cambiare versione.
infra-upgrades-manual = L'assistente di aggiornamento non è installato, quindi la versione si cambia a mano sul server.
infra-upgrades-fix = Esegui una volta il bootstrap.sh attuale come root: installa l'assistente insieme al servizio.
infra-sandbox-title = Sandbox
infra-sandbox-ok = Il processo gira con no-new-privileges e un filtro delle chiamate di sistema.
infra-sandbox-unknown = Questo sistema non riporta la sandbox del processo.
infra-sandbox-off = Il processo gira senza una parte della sua sandbox: no-new-privileges o il filtro delle chiamate di sistema è disattivato.
infra-sandbox-fix = Esegui il servizio dall'unità che scrive bootstrap.sh, senza drop-in che la allentino. Su una macchina di sviluppo è normale.
infra-clock-title = Orologio
infra-clock-ok = L'orologio di sistema segna il { $year }, il che è plausibile.
infra-clock-wrong = L'orologio di sistema segna il { $year }: date, età dei backup e sessioni sono tutte sbagliate.
infra-clock-fix = Attiva la sincronizzazione dell'ora: timedatectl set-ntp true.

# Administration console: the Infrastructure page (v0.5.0, docs/adr/0011)
infra-title = Infrastruttura
infra-lede = La macchina di questo sito: accesso, chiavi, dati, versione e sandbox, verificati a ogni apertura della pagina.
infra-kpi-health = Salute
infra-kpi-version = Versione
infra-kpi-version-newer = { $version } disponibile
infra-kpi-version-current = Aggiornata
infra-kpi-version-unknown = Non ancora verificata
infra-kpi-disk = Spazio libero
infra-kpi-disk-note = { $percent }% di { $total } usato
infra-kpi-compute = Macchina
infra-kpi-cpus = { $n ->
    [one] { $n } core
    [many] { $n } core
    *[other] { $n } core
    }
infra-kpi-load = Carico { $load }
infra-kpi-rss = { $size } in uso
infra-unknown = Sconosciuto
infra-health = Salute
infra-health-intro = Verde va bene, un avviso merita un'occhiata, critico chiede di agire subito.
infra-json = Gli stessi controlli in JSON, per un monitoraggio
infra-version = Versione
infra-version-running = In esecuzione
infra-version-latest = Ultima pubblicata
infra-version-up-to-date = aggiornata
infra-version-checked = Elenco recuperato
infra-version-never = mai
infra-source-direct = da questo server
infra-source-helper = dall'assistente di aggiornamento
infra-version-token = Repository privato
infra-token-env-set = È impostato un token GitHub per elencare e scaricare le versioni.
infra-token-env-unset = Nessun token. Se il repository è privato, mettine uno in { $name }, leggibile solo da root: l'assistente di aggiornamento lo legge lì.
infra-releases-error = L'ultimo tentativo di recuperare l'elenco non è riuscito:
infra-releases-refresh = Cerca nuove versioni
infra-releases-asked = All'assistente di aggiornamento è stato chiesto un elenco aggiornato. Ricarica questa pagina fra un minuto.
infra-releases-not-asked = Non è stato possibile scrivere la richiesta: { $error }
infra-upgrade-status-running = Passaggio da { $from } a { $to } in corso…
infra-upgrade-status-succeeded = Passata da { $from } a { $to }.
infra-upgrade-status-failed = Il passaggio da { $from } a { $to } non è riuscito; è ancora in esecuzione { $from }.
infra-upgrade-status-refused = L'assistente di aggiornamento ha rifiutato il passaggio a { $to }.
infra-upgrade-log = Output dell'assistente
infra-upgrade-pending = { $by } ha chiesto { $version } il { $at }; l'assistente di aggiornamento non l'ha ancora preso in carico.
infra-releases = Versioni pubblicate
infra-releases-tag = Versione
infra-releases-published = Pubblicata
infra-releases-relation = Rispetto a questa
infra-releases-notes = Note di rilascio
infra-releases-open = Leggi
infra-relation-newer = più recente
infra-relation-current = questa
infra-relation-older = più vecchia
infra-prerelease = release candidate
infra-releases-none = Ancora nessun elenco delle versioni.
infra-upgrade = Cambia versione
infra-upgrade-intro = L'assistente di aggiornamento fa un backup, installa la versione verificata, riavvia e controlla; se fallisce, torna la versione precedente.
infra-upgrade-version = Versione da installare
infra-upgrade-confirm = Per tornare a una versione più vecchia, digita di nuovo il suo tag
infra-upgrade-confirm-hint = Non serve per una versione più recente. Una versione più vecchia potrebbe non leggere dati scritti da una più recente: leggi prima le sue note.
infra-upgrade-go = Installa questa versione
infra-upgrade-safety = Il sito non è disponibile per qualche secondo mentre il servizio si riavvia.
infra-upgrade-manual = L'assistente di aggiornamento non è installato su questo server, quindi questa pagina non può cambiare versione. Come root, sul server:
infra-upgrade-manual-helper = Esegui una volta il bootstrap.sh attuale per installare l'assistente.
infra-upgrade-asked = È stata richiesta { $version }. L'assistente di aggiornamento la prende in carico in pochi secondi; questa pagina mostra com'è andata.
infra-upgrade-no-helper = L'assistente di aggiornamento non è installato, quindi { $version } non può essere installata da qui.
infra-upgrade-unknown-version = { $version } non è nell'elenco delle versioni pubblicate.
infra-upgrade-same-version = { $version } è la versione già in esecuzione.
infra-upgrade-confirm-older = { $version } è più vecchia della versione in esecuzione. Digita il suo tag nel campo di conferma per tornarci.
infra-volumes = Archiviazione
infra-volumes-intro = La partizione che contiene la directory dei dati ({ $path }) e ciò che questo sito vi conserva.
infra-volumes-partition = Partizione
infra-of = { $used } su { $total }
infra-volumes-ours = Questo sito
infra-volumes-breakdown = Ciò che conserva questo sito
infra-part-bundle = Archivio della famiglia
infra-part-cache = Cache degli allegati
infra-part-backups = Backup
infra-part-binary = Programma
infra-part-journal = Registro delle modifiche
infra-part-accounts = Account e impostazioni
infra-files = { $n ->
    [one] { $n } file
    [many] { $n } file
    *[other] { $n } file
    }
infra-growth = Crescita
infra-growth-month = Al mese
infra-growth-not-enough = backup insufficienti per dirlo
infra-growth-left = Spazio residuo a questo ritmo
infra-backups = Backup
infra-backups-dir = Directory
infra-backups-count = Archivi
infra-backups-newest = Il più recente
infra-backups-oldest = Il più vecchio
infra-backups-room = Spazio dove sono conservati
infra-free-of = { $free } liberi su { $total }
infra-paths = Percorsi
infra-path-data = Directory dei dati
infra-path-binary = Programma
infra-compute = Calcolo
infra-compute-hidden = La sandbox del servizio gli nasconde i dati della macchina stessa; ciò che appare qui è ciò che il processo può vedere.
infra-compute-memory = Memoria della macchina
infra-available-of = { $available } liberi su { $total }
infra-compute-ceiling = Memoria del servizio
infra-compute-no-ceiling = { $used }, senza limite
infra-compute-os = Sistema
infra-compute-kernel = Kernel
infra-compute-arch = Architettura
infra-compute-cpu = Processore
infra-compute-cpus = Core
infra-compute-load = Carico (1, 5, 15 min)
infra-compute-rss = Questo processo
infra-compute-threads = Thread
infra-compute-files = File aperti
infra-compute-uptime = Servizio attivo da
infra-compute-sys-uptime = Macchina attiva da
infra-compute-sandbox = Sandbox
infra-compute-nnp-on = nessun nuovo privilegio
infra-compute-nnp-off = i privilegi possono essere acquisiti
infra-compute-seccomp-on = chiamate di sistema filtrate
infra-compute-seccomp-off = chiamate di sistema non filtrate
infra-activity = Attività
infra-activity-intro = Su { $span }; riparte da zero al riavvio del servizio.
infra-activity-sessions = Sessioni aperte
infra-activity-signed-in = Accessi
infra-activity-failed = Accessi falliti
infra-activity-registered = Account creati
infra-activity-cross-site = Invii da altri siti rifiutati

# Sign-in services: what the reader is told when one fails (v0.5.0, docs/adr/0012)
sso-error-not-configured = Questo servizio di accesso non è configurato su questo sito.
sso-error-no-public-url = Accedere tramite un servizio richiede l'indirizzo pubblico di questo sito, che non è ancora impostato. Lo imposta un amministratore in Connettività.
sso-error-unreachable = Non è stato possibile raggiungere il servizio di accesso. Riprova tra poco o accedi in un altro modo.
sso-error-refused = Il servizio di accesso non ti ha fatto entrare, oppure l'accesso è stato annullato lì.
sso-error-stale = Quell'accesso era scaduto o è stato avviato in un altro browser. Ricomincia.
sso-error-invalid = La risposta del servizio di accesso non ha superato i controlli di questo sito, quindi non è stata usata. Ricomincia; se succede ancora, avvisa un amministratore.
infra-upgrade-already-pending = { $version } è già in attesa dell'assistente di aggiornamento. Aspetta che finisca prima di chiedere un'altra versione.

# Administration console: Connectivity, sign-in services and registration (v0.5.0, docs/adr/0012)
connectivity-title = Connettività
connectivity-lede = Come i lettori raggiungono questo sito e accedono: indirizzo pubblico, servizi di accesso e regole.
connectivity-kpi-address = Indirizzo pubblico
connectivity-address-unset = Non impostato
connectivity-address-https = HTTPS
connectivity-address-plain = Non cifrato
connectivity-address-unset-note = Ogni servizio di accesso ne ha bisogno
connectivity-kpi-services = Servizi di accesso
connectivity-kpi-services-note = { $n ->
    [one] pronto da offrire
    [many] pronti da offrire
    *[other] pronti da offrire
    }
connectivity-kpi-passwords = Password
connectivity-policy-short-everyone = Tutti
connectivity-policy-short-non-admins = Non gli amministratori
connectivity-policy-short-nobody = Nessuno
connectivity-policy-note-everyone = Ogni account può accedere con la sua password, e anche con un servizio collegato.
connectivity-policy-note-non-admins = Gli amministratori accedono solo tramite un servizio; gli altri possono ancora usare una password. Una password di amministratore rubata non apre più nulla.
connectivity-policy-note-nobody = Tutti accedono tramite un servizio; un account senza servizio collegato non può accedere.
connectivity-kpi-admins-linked = { $n ->
    [one] { $n } amministratore con un servizio collegato
    [many] { $n } amministratori con un servizio collegato
    *[other] { $n } amministratori con un servizio collegato
    }
connectivity-kpi-registration = Registrazione
connectivity-kpi-registration-go = Si imposta in Account →
registration-short-closed = Solo amministratori
registration-short-secret = Con il segreto di famiglia
registration-short-open = Aperta ai visitatori
connectivity-address = Indirizzo pubblico
connectivity-address-intro = L'indirizzo pubblico che i lettori digitano, come https://family.example; i servizi di accesso li rimandano lì.
connectivity-address-label = Indirizzo pubblico
connectivity-address-hint = Lascialo vuoto finché nessuno accede tramite un servizio.
connectivity-address-invalid = Non è un indirizzo http o https con un nome host.
connectivity-address-saved = L'indirizzo pubblico è salvato.
connectivity-policy = Regole di accesso
connectivity-policy-intro = Valgono per ogni account; una regola che non lascerebbe entrare nessun amministratore viene rifiutata.
connectivity-policy-passwords = Chi può accedere con una password
connectivity-policy-without-way = { $n ->
    [one] { $n } account attivo non ha né servizio collegato né indirizzo: con «Nessuno» non potrebbe accedere.
    [many] { $n } account attivi non hanno né servizio collegato né indirizzo: con «Nessuno» non potrebbero accedere.
    *[other] { $n } account attivi non hanno né servizio collegato né indirizzo: con «Nessuno» non potrebbero accedere.
    }
connectivity-link-by-email = Collegare un servizio all'account con lo stesso indirizzo verificato
connectivity-link-by-email-note = Crea l'account di un parente con il suo indirizzo: il primo accesso tramite un servizio che lo verifica lo collega.
connectivity-domains = Domini autorizzati ad aprire un account
connectivity-domains-hint = Uno per riga (family.example). Vuoto: tutti. Altrimenti solo gli indirizzi verificati di questi domini possono aprire un account.
connectivity-domain-invalid = «{ $domain }» non è un nome di dominio.
connectivity-policy-unknown = Scegli chi può accedere con una password.
connectivity-policy-lockout = Rifiutato: con questa modifica nessun amministratore attivo potrebbe accedere. Collega prima un servizio di accesso all'account di un amministratore, accedendo una volta tramite esso, oppure mantieni le password per gli amministratori.
connectivity-policy-saved = Le regole di accesso sono salvate.
connectivity-not-saved = Non salvato: { $error }
connectivity-services = Servizi di accesso
connectivity-services-intro = Registra questo sito nella console del servizio con l'indirizzo di reindirizzamento indicato, poi incolla qui ID client e segreto.
connectivity-outbound-note = Accedere tramite un servizio richiede connessioni in uscita: sudo systemctl edit axgf-cms, aggiungi IPAddressAllow=any sotto [Service], poi riavvia.
connectivity-service-ready = Pronto
connectivity-service-incomplete = Incompleto
connectivity-service-off = Disattivato
connectivity-service-enable = Offrire questo servizio nelle pagine di accesso e registrazione
connectivity-service-issuer = Emittente
connectivity-service-issuer-hint = L'indirizzo del fornitore, ad esempio https://auth.family.example/realms/family; le sue impostazioni si leggono da /.well-known/openid-configuration sotto di esso.
connectivity-service-label = Etichetta del pulsante
connectivity-service-tenant = Directory (tenant)
connectivity-service-tenant-hint = common per qualsiasi account Microsoft, organizations per account aziendali e scolastici, consumers per quelli personali, o l'ID della tua directory.
connectivity-service-client-id = ID client
connectivity-service-secret = Segreto client
connectivity-service-secret-set = Un segreto è impostato. Lascia vuoto per mantenerlo.
connectivity-service-secret-unset = Nessun segreto ancora impostato.
connectivity-service-secret-clear = Rimuovi il segreto
connectivity-service-redirect = Indirizzo di reindirizzamento da registrare
connectivity-service-redirect-needs-address = Imposta prima l'indirizzo pubblico
connectivity-service-console = Dove registrarlo
connectivity-service-console-open = Console di { $name }
connectivity-service-test = Prova la connessione
connectivity-issuer-invalid = L'emittente deve essere un indirizzo https://.
connectivity-tenant-invalid = Una directory è un nome o un ID: lettere, cifre, punti e trattini.
connectivity-provider-incomplete = { $name } non è completo: servono un ID client e un segreto client, e un emittente per un fornitore generico.
connectivity-provider-saved = { $name } è salvato.
connectivity-test-ok = { $name } ha risposto: { $answer }.
connectivity-test-failed = { $name } non è raggiungibile da questo server: { $detail }
registration-title = Registrazione
registration-intro = Se i visitatori possono aprire da soli un account membro; un amministratore può dargli un altro ruolo qui sotto.
registration-who = Chi può aprire un account
registration-note-closed = Solo un amministratore crea account, in questa pagina.
registration-note-secret = Chi conosce il segreto della famiglia può aprire un account membro. Condividilo come un codice del portone.
registration-note-open = Qualsiasi visitatore può aprire un account da membro. I membri leggono le schede riservate ai membri.
registration-rotate = Genera un nuovo segreto
registration-secret = Il segreto di famiglia
registration-secret-note = Chiunque lo abbia può aprire un account finché questa modalità è attiva. Generarne uno nuovo esclude il vecchio; gli account già aperti restano.
registration-link = Pagina di registrazione
registration-quick = Accessi rapidi
registration-quick-none = Nessun servizio di accesso è pronto: configurane uno in Connettività
registration-changed = Modificata l'ultima volta da { $by }, { $at }.
registration-unknown = Scegli chi può aprire un account.
accounts-kpi-total = Account attivi
accounts-kpi-total-note = { $disabled ->
    [one] { $disabled } disattivato
    [many] { $disabled } disattivati
    *[other] { $disabled } disattivati
    }
accounts-kpi-admins = Amministratori
accounts-kpi-admins-few = Aggiungine un secondo
accounts-kpi-admins-ok = Più di uno, come deve essere
accounts-kpi-linked = Collegati a un servizio
accounts-kpi-linked-note = { $n ->
    [one] { $n } servizio pronto
    [many] { $n } servizi pronti
    *[other] { $n } servizi pronti
    }
accounts-kpi-registered = { $n ->
    [one] { $n } aperto da un visitatore
    [many] { $n } aperti da visitatori
    *[other] { $n } aperti da visitatori
    }
accounts-roles = Ruoli
accounts-sign-in = Accede con
accounts-sign-in-password = password
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = accesso unico
accounts-sign-in-none = ancora nessun modo di entrare
accounts-created-via-register = aperto dal visitatore
accounts-created-via-sso = aperto tramite un servizio
accounts-add-also-register = I visitatori possono anche aprire da sé account da membro, come impostato sopra. Un account creato qui può avere qualsiasi ruolo.
login-password-not-allowed = Le regole di accesso di questo sito non consentono a questo account di usare una password: accedi tramite un servizio.
login-register = Non hai ancora un account? Creane uno
register-title = Crea un account
register-lede = Un account da membro legge le schede riservate ai membri della famiglia. Il modo più rapido è un account che hai già.
register-closed = Qui gli account li crea un amministratore. Chiedigli il tuo.
register-have-account = Hai già un account? Accedi
register-or = oppure
register-secret = Il segreto di famiglia
register-secret-hint = Te l'ha dato un amministratore, o qualcuno della famiglia.
register-secret-hint-services = Digitalo prima di scegliere un servizio, altrimenti te lo chiederà il passo successivo.
register-domains = Solo gli indirizzi di { $domains } possono aprire un account.
register-username-hint = Da 2 a 64 lettere, cifre, punti, trattini o trattini bassi.
register-submit = Crea l'account
register-member-note = Non si chiede alcun indirizzo: non ti viene inviato nulla, e nessuno può appropriarsi del tuo.
register-form-closed = Questo sito non apre account con questo modulo.
register-throttled = Troppi tentativi da qui. Attendi un quarto d'ora e riprova.
register-secret-wrong = Non è il segreto di famiglia.
register-username-invalid = Un nome utente ha da 2 a 64 lettere, cifre, punti, trattini o trattini bassi.
register-password-short = La password deve avere almeno { $min } caratteri.
register-username-taken = Quel nome utente è già preso. Scegline un altro.
register-not-saved = Non è stato possibile salvare l'account. Riprova tra poco.
register-finish-title = Ancora un passo
register-finish-lede = Il servizio ha confermato chi sei. Per aprire un account qui, digita il segreto di famiglia.
register-finish-lede-email = Il servizio ti ha confermato come { $email }. Per aprire un account qui, digita il segreto di famiglia.
sso-continue = Continua
sso-continue-with = Continua con { $name }
sso-error-title = L'accesso non è stato completato
sso-account-disabled = L'account collegato a questo accesso è disattivato. Rivolgiti a un amministratore.
sso-no-account = Nessun account qui corrisponde a questo accesso, e questo sito non apre account da sé. Chiedi a un amministratore di creare il tuo con questo indirizzo: il tuo prossimo accesso vi sarà collegato.
sso-domain-refused = Qui si può aprire un account solo con un indirizzo dei domini della famiglia, confermato dal servizio.

# Administration console: the overview's tiles (v0.5.0, docs/adr/0011)
overview-accounts-note = { $admins ->
    [one] { $admins } amministratore
    [many] { $admins } amministratori
    *[other] { $admins } amministratori
    }
register-password-hint = Almeno { $min } caratteri. Una frase di più parole si ricorda meglio di un guazzabuglio di segni.

# Clarity: the light theme and its funnel (v0.5.0, docs/adr/0013)
theme-clarity = Chiarezza
theme-clarity-note = chiaro, bianco e arancio · un imbuto
view-funnel-back = Torna all'imbuto
view-funnel-open = Apri l'imbuto
view-funnel-hero-1 = Ogni generazione è un anello.
view-funnel-hero-2 = Ogni famiglia, un imbuto.
view-funnel-cta = Esplora l'imbuto
view-funnel-cta-detail = Ogni nome aggiunto e ogni fonte verificata rendono l'imbuto più vero per chi verrà dopo.
view-funnel-see = Vedi nell'imbuto
view-funnel-direction = ↓ i più antichi all'estremità stretta · ↑ i più recenti all'imboccatura
view-funnel-still = Ferma l'imbuto
view-funnel-name = Imbuto
view-funnel-legend = Come leggere l'imbuto
view-funnel-search-hint = Invio li porta in primo piano; Maiusc+Invio apre la loro scheda.
theme-genealogy = Genealogia
theme-genealogy-note = verde scuro e crema · un albero da scalare
view-scroll-back = Torna all’albero genealogico
view-scroll-open = Apri l’albero genealogico
view-scroll-hero-1 = Ogni vita è un ritratto.
view-scroll-hero-2 = Ogni famiglia, un albero da scalare.
view-scroll-cta = Esplora l’albero genealogico
view-scroll-cta-detail = Ogni nome aggiunto e ogni fonte verificata rendono l’albero genealogico più fedele per chi verrà.
view-scroll-see = Vedi nell’albero genealogico
view-scroll-direction = ↓ verso le radici: gli antenati · ↑ verso la chioma: i discendenti
view-scroll-still = Ferma le foglie
view-scroll-name = Albero genealogico
view-scroll-legend = Come leggere l’albero genealogico
view-scroll-search-hint = Invio la mette sul tronco; Maiusc+Invio apre la sua scheda.
bundle-binding-unnamed = un archivio senza nome
bundle-binding-created = un archivio creato il { $date }
bundle-binding-named = «{ $family }»
bundle-binding-named-created = «{ $family }», creato il { $date }

# Notes and contributions (docs/adr/0014)
notes-section-title = Note e contributi
notes-section-intro = Ciò che i membri della famiglia hanno messo per iscritto: ricordi, ricerche, lettere, documenti. Cercateli, sfogliateli per lettera o apritene uno per leggerlo e salvarlo in PDF.
notes-search-label = Cerca nelle note
notes-search-go = Cerca
notes-all = Tutte
notes-letters-label = Le note per prima lettera del titolo
notes-shown = Mostrate: { $shown } su { $total }
notes-none = Non è ancora stata scritta alcuna nota.
notes-none-found = Nessuna nota corrisponde a questa ricerca.
notes-write = Scrivi una nota
notes-family-wide = Tutta la famiglia
notes-by = di { $author }
notes-about = Riguardo a: { $names }
notes-about-prefix = Riguardo a
notes-about-family = Riguardo a tutta la famiglia
notes-pdf = Scarica in PDF
notes-edit = Modifica
notes-close = Chiudi
notes-back = Torna alle note
notes-add-about = Scrivi una nota su questa persona
notes-sign-in = Le note e i contributi della famiglia sono riservati ai suoi membri. Accedete per leggerli.
notes-admin-title = Note e contributi
notes-admin-intro = Scrivete ciò che le schede non possono contenere: un ricordo, una ricerca, una lettera, con immagini. Una nota collegata a delle persone compare nelle loro schede; una nota non collegata a nessuno appartiene a tutta la famiglia. Ogni membro le legge tutte nella pagina Famiglia.
notes-deleted = La nota è stata eliminata.
notes-col-title = Titolo
notes-col-category = Categoria
notes-col-date = Data
notes-col-about = Riguardo a
notes-col-author = Autore
notes-col-actions = Azioni
notes-new-title = Nuova nota
notes-edit-title = Modifica la nota
notes-section-about = La nota
notes-field-title = Titolo
notes-field-category = Categoria
notes-field-category-hint = Testo libero; vengono proposte le categorie già usate.
notes-field-date = Data
notes-field-date-hint = 1952, 1952-06 o 1952-06-14; vuoto se non ce n'è.
notes-field-people = Persone interessate
notes-field-people-hint = Scrivi un nome e sceglilo. Senza nessuno nominato, la nota è di tutta la famiglia.
notes-person-add = Aggiungi
notes-person-remove = Togli questa persona
notes-field-text = Testo
notes-text-hint = Markdown, o i pulsanti qui sopra. Incolla o trascina immagini, fino a { $mb } MB l'una.
notes-tool-bold = Grassetto
notes-tool-italic = Corsivo
notes-tool-heading = Titoletto
notes-tool-list = Elenco
notes-tool-quote = Citazione
notes-tool-link = Link
notes-tool-image = Aggiungi un'immagine
notes-tool-preview = Anteprima
notes-tool-write = Torna alla scrittura
notes-uploading = Aggiunta dell'immagine…
notes-save = Salva la nota
notes-cancel = Annulla
notes-delete = Elimina questa nota
notes-delete-confirm = La nota e le sue immagini saranno tolte dall'archivio.
notes-category-suggestions = Ricordo, Ricerca, Lettera, Racconto, Ricetta, Documento
notes-error-title = La nota ha bisogno di un titolo. Non è stato salvato nulla.
notes-error-title-long = Il titolo è troppo lungo: al massimo 200 caratteri. Non è stato salvato nulla.
notes-error-text-long = Il testo è troppo lungo. Non è stato salvato nulla.
notes-error-date = Questa data non è compresa. Scrivete un anno, un mese o un giorno: 1952, 1952-06 o 1952-06-14. Non è stato salvato nulla.
notes-error-person = Persone interessate: «{ $name }».
notes-error-image-gone = Un'immagine incollata non era più in attesa di essere salvata; le immagini attendono sei ore. Aggiungetela di nuovo. Non è stato salvato nulla.
notes-error-too-many-images = Una nota contiene al massimo 40 immagini. Non è stato salvato nulla.
notes-error-stale = Qualcuno ha modificato questa nota mentre ci lavoravate. Riapritela per vedere la sua versione, poi rifate la vostra modifica. Non è stato salvato nulla.
notes-error-refused = L'archivio ha rifiutato questa modifica. Non è stato salvato nulla.
notes-error-image = Questo file non è un'immagine che questo sito sa mostrare: PNG, JPEG, GIF o WebP.
notes-error-image-large = Questa immagine è troppo pesante.
notes-stale-title = La nota non è stata modificata
notes-missing-title = Nota non trovata
notes-missing = Qui non c'è nessuna nota, oppure non è aperta per voi.
notes-not-yours = Chi contribuisce modifica solo le note che ha scritto. Un amministratore può modificare questa.
notes-pdf-failed-title = Non è stato possibile creare il PDF
notes-pdf-failed = Qualcosa è andato storto nel creare il PDF di questa nota. Nel frattempo leggetela nella sua pagina e avvisate un amministratore.
notes-dashboard-count = Note scritte: { $n }
notes-dashboard-open = Scrivi e gestisci le note

# The family the archive describes: the bundle room's form (2026-10-23)
kind-bundle = archivio
manifest-title = La famiglia
manifest-intro = Ciò che l'archivio dice della sua famiglia. Il suo nome compare nella pagina Famiglia, nei titoli e nei PDF delle note.
manifest-name = Nome della famiglia
manifest-name-hint = Mostrato nella pagina Famiglia e nei titoli. Gli account seguono il nuovo nome.
manifest-description = Descrizione
manifest-culture = Cultura principale
manifest-culture-hint = Un'etichetta di lingua: fr, pl, pt-BR…
manifest-place = Luogo principale
manifest-place-hint = Il continente in cui la famiglia ha vissuto soprattutto.
manifest-span = Periodo coperto
manifest-span-hint = Dalla nascita più antica alla più recente; calcolato al salvataggio.
manifest-license = Licenza
manifest-license-type = Licenza
manifest-save = Salva i dati della famiglia
manifest-saved = I dati della famiglia sono salvati. Gli account seguono il suo nome.
manifest-error-long = Un campo è troppo lungo: 120 caratteri per il nome, 2000 per la descrizione. Non è stato salvato nulla.
manifest-error-culture = La cultura è un'etichetta di lingua, come fr, pl o pt-BR. Non è stato salvato nulla.
manifest-error-stale = Qualcuno ha modificato i dati della famiglia mentre li modificavate. La vostra versione è qui sotto: salvarla di nuovo sostituisce l'altra. Non è stato salvato nulla.
manifest-error-io = Non è stato possibile scrivere l'archivio; il registro del server dice perché. Non è stato salvato nulla.

# The family form: places, licences and the period (2026-10-23)
manifest-none = Non indicato
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = Nessuna data di nascita registrata per ora.
manifest-license-pick = Scegliete una licenza per vedere che cosa permette agli altri di fare con l'archivio.
manifest-license-author = Autore
manifest-license-author-value = { $who }, { $at }
manifest-license-author-none = Registrato quando si sceglie una licenza.
manifest-error-place = Scegliete il luogo principale dall'elenco. Non è stato salvato nulla.
manifest-error-licence = Scegliete la licenza dall'elenco. Non è stato salvato nulla.
continent-africa = Africa
continent-antarctica = Antartide
continent-asia = Asia
continent-europe = Europa
continent-north-america = America settentrionale
continent-south-america = America meridionale
continent-oceania = Oceania
continent-several = Più continenti
licence-all-rights-reserved-name = Tutti i diritti riservati
licence-all-rights-reserved-about = Nessuno può copiare, condividere o riutilizzare l'archivio senza il permesso della famiglia. È ciò che vale quando non si dice nulla.
licence-family-private-name = Privata, per la famiglia
licence-family-private-about = Riservata ai membri della famiglia: da non pubblicare né trasmettere fuori dalla famiglia.
licence-cc0-name = CC0 1.0 — pubblico dominio
licence-cc0-about = Chiunque può copiare, modificare e usare l'archivio per qualsiasi scopo, anche commerciale, senza chiedere né citare nessuno.
licence-cc-by-name = CC BY 4.0 — Attribuzione
licence-cc-by-about = Chiunque può condividere e adattare l'archivio, anche commercialmente, purché ne citi gli autori e indichi che cosa ha cambiato.
licence-cc-by-sa-name = CC BY-SA 4.0 — Attribuzione, Condividi allo stesso modo
licence-cc-by-sa-about = Chiunque può condividerlo e adattarlo, anche commercialmente, citandone gli autori; ciò che ne deriva va condiviso con la stessa licenza.
licence-cc-by-nc-name = CC BY-NC 4.0 — Attribuzione, Non commerciale
licence-cc-by-nc-about = Chiunque può condividerlo e adattarlo citandone gli autori, ma non a fini commerciali.
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — Attribuzione, Non commerciale, Condividi allo stesso modo
licence-cc-by-nc-sa-about = Chiunque può condividerlo e adattarlo citandone gli autori, senza fini commerciali e solo con la stessa licenza.
licence-cc-by-nd-name = CC BY-ND 4.0 — Attribuzione, Non opere derivate
licence-cc-by-nd-about = Chiunque può condividerlo così com'è, anche commercialmente, citandone gli autori, ma non diffonderne una versione modificata.
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — Attribuzione, Non commerciale, Non opere derivate
licence-cc-by-nc-nd-about = Chiunque può condividerlo così com'è citandone gli autori, ma né a fini commerciali né modificato: la licenza Creative Commons più restrittiva.
licence-odbl-name = ODbL 1.0 — Open Database License
licence-odbl-about = Pensata per le basi di dati: chiunque può usare, condividere e adattare i dati citandone la fonte, e una base che ne derivi deve restare aperta alle stesse condizioni.

# An account's own page and its record in the tree (2026-10-23)
nav-account = Il mio account
account-title = Il mio account
account-who = L'account
account-email = Indirizzo e-mail
account-since = Dal
account-person-title = La mia scheda nell'albero
account-person-intro = Quale persona dell'albero sei? Scegli tu, senza verifiche; puoi cambiarla quando vuoi.
account-person-is = Questo account dice di essere
account-person-unlink = Togli il collegamento
account-person-pick = Una persona dell'albero
account-person-link = Collega il mio account a questa scheda
account-person-change = Collegami invece a questa scheda
account-person-linked = Il vostro account è collegato alla sua scheda.
account-person-unlinked = Il vostro account non è più collegato a una scheda.
account-person-error = Non è stato possibile collegare questa scheda:
account-person-io = Non è stato possibile salvare l'account; il registro del server dice perché. Non è cambiato nulla.
account-emergency-page = Il token di emergenza non è un account: non ha una pagina propria né una scheda nell'albero.
accounts-person = Scheda:

## The merge screen (src/routes/merge.rs): two records a person says are one.
merge-action = Unisci
merge-needs-two = Seleziona esattamente due schede per unirle.
merge-title = Unisci due schede
merge-lede = Nulla cambia finché non confermi. La prima scheda viene conservata; la seconda vi viene unita e conservata così com’era nel registro delle modifiche.
merge-keep = Conservata
merge-absorb = Unita alla prima
merge-swap = Conserva l’altra
merge-refused = Questa unione è rifiutata:
merge-refusal-ancestor = una è antenata dell’altra
merge-refusal-spouse-and-child = nella stessa famiglia, una è coniuge e l’altra figlia
merge-refusal-spouses = sono i due coniugi della stessa unione
merge-refusal-siblings = risultano figli degli stessi genitori
merge-refusal-different-couples = le due famiglie sono coppie diverse
merge-refusal-child-is-spouse = un figlio di una famiglia è coniuge nell’altra
merge-refusal-same-record = è due volte la stessa scheda
merge-conflicts = In che cosa differiscono
merge-nothing = Nulla da scegliere: non si contraddicono.
merge-choice-keep = Scheda conservata
merge-choice-absorb = Altra scheda
merge-choice-both = Entrambe
merge-because-higher-confidence = Suggerito: il valore con il grado di certezza più alto.
merge-because-sourced = Suggerito: il valore che cita una fonte.
merge-because-kept-record = Suggerito: il valore della scheda conservata.
merge-because-more-restrictive = Suggerito: la visibilità più restrittiva.
merge-because-protective = Suggerito: in vita, la lettura che protegge.
merge-because-both-kept = Suggerito: conservare entrambi i testi.
merge-becomes-alias = Il nome non scelto è conservato come altro nome.
merge-becomes-event = Il valore non scelto è conservato come evento.
merge-agreements = Su che cosa concordano
merge-from-keep = dalla scheda conservata
merge-from-absorb = dall’altra scheda
merge-from-both = uguale in entrambe
merge-from-joined = elenchi riuniti
merge-from-combined = combinati
merge-effects = Che cos’altro cambia
merge-effect-references = Riferimenti riscritti
merge-effect-self-links = Collegamenti a sé stessa rimossi
merge-effect-listed-twice = Elenchi che la nominerebbero due volte
merge-effect-same-couple = Famiglie della stessa coppia
merge-confirm-label = Può comunque trattarsi di una sola persona. Per unire lo stesso, scrivi il nome esattamente: { $name }
merge-go = Unisci
merge-missing = Una delle due schede non esiste più.
merge-done-title = Unite
merge-done-lede = { $absorbed } fa ora parte di { $kept }.
merge-report-references = Riferimenti riscritti
merge-report-events = Eventi creati da un valore non scelto
merge-report-source-conflicts = Disaccordi annotati su una fonte
merge-report-links = Collegamenti rimossi
merge-report-tidied = Voci riordinate
merge-absorbed-record = La scheda unita, com’era
merge-open-kept = Apri la scheda conservata

## Parentela (docs/adr/0020)

kin-tab = Parentela
kin-parent = { $adoptive ->
        [yes] { $sex ->
            [M] padre adottivo
            [F] madre adottiva
           *[other] genitore adottivo
        }
       *[no] { $sex ->
            [M] padre
            [F] madre
           *[other] genitore
        }
    }
kin-ancestor = { $n ->
        [2] { $sex ->
            [M] nonno
            [F] nonna
           *[other] nonno o nonna
        }
        [3] { $sex ->
            [M] bisnonno
            [F] bisnonna
           *[other] bisnonno o bisnonna
        }
        [4] { $sex ->
            [M] trisnonno
            [F] trisnonna
           *[other] trisnonno o trisnonna
        }
       *[other] { $sex ->
            [F] antenata alla { $n }ª generazione
           *[other] antenato alla { $n }ª generazione
        }
    }
kin-child = { $adoptive ->
        [yes] { $sex ->
            [M] figlio adottivo
            [F] figlia adottiva
           *[other] figlio adottivo
        }
       *[no] { $sex ->
            [M] figlio
            [F] figlia
           *[other] figlio o figlia
        }
    }
kin-descendant = { $n ->
        [2] { $sex ->
            [M] nipote (figlio di un figlio)
            [F] nipote (figlia di un figlio)
           *[other] nipote
        }
        [3] pronipote
        [4] trisnipote
       *[other] discendente alla { $n }ª generazione
    }
kin-sibling = { $half ->
        [yes] { $sex ->
            [M] fratellastro
            [F] sorellastra
           *[other] fratellastro o sorellastra
        }
       *[no] { $adoptive ->
            [yes] { $sex ->
                [M] fratello adottivo
                [F] sorella adottiva
               *[other] fratello o sorella adottivi
            }
           *[no] { $sex ->
                [M] fratello
                [F] sorella
               *[other] fratello o sorella
            }
        }
    }
kin-uncle = { $n ->
        [1] { $sex ->
            [M] zio
            [F] zia
           *[other] zio o zia
        }
        [2] { $sex ->
            [M] prozio
            [F] prozia
           *[other] prozio o prozia
        }
        [3] { $sex ->
            [M] fratello di un bisnonno
            [F] sorella di un bisnonno
           *[other] fratello o sorella di un bisnonno
        }
       *[other] { $sex ->
            [M] fratello di un antenato
            [F] sorella di un antenato
           *[other] fratello o sorella di un antenato
        }
    }
kin-nephew = { $n ->
        [1] { $sex ->
            [M] figlio
            [F] figlia
           *[other] figlio o figlia
        } { $via ->
            [M] del fratello
            [F] della sorella
           *[other] di un fratello o di una sorella
        }
        [2] { $sex ->
            [M] nipote
            [F] nipote
           *[other] nipote
        } { $via ->
            [M] del fratello
            [F] della sorella
           *[other] di un fratello o di una sorella
        }
       *[other] discendente di un fratello o di una sorella
    }
kin-cousin = { $sex ->
        [F] { $k ->
            [1] cugina di primo grado
            [2] cugina di secondo grado
            [3] cugina di terzo grado
           *[other] cugina di { $k }° grado
        }
       *[other] { $k ->
            [1] cugino di primo grado
            [2] cugino di secondo grado
            [3] cugino di terzo grado
           *[other] cugino di { $k }° grado
        }
    }
kin-cousin-removed = { $older ->
        [yes] { $cousin } { $forebear }
       *[no] { $descendant } { $ofcousin }
    }
kin-of-forebear = { $n ->
        [1] { $sex ->
            [M] del padre
            [F] della madre
           *[other] di un genitore
        }
        [2] { $sex ->
            [M] del nonno
            [F] della nonna
           *[other] di un nonno
        }
        [3] { $sex ->
            [M] del bisnonno
            [F] della bisnonna
           *[other] di un bisnonno
        }
       *[other] di un antenato
    }
kin-descendant-word = { $n ->
        [1] { $sex ->
            [M] figlio
            [F] figlia
           *[other] figlio
        }
        [2] nipote
        [3] pronipote
       *[other] discendente
    }
kin-of-cousin = { $sex ->
        [F] di una { $k ->
            [1] cugina di primo grado
            [2] cugina di secondo grado
            [3] cugina di terzo grado
           *[other] cugina di { $k }° grado
        }
       *[other] di un { $k ->
            [1] cugino di primo grado
            [2] cugino di secondo grado
            [3] cugino di terzo grado
           *[other] cugino di { $k }° grado
        }
    }
kin-partner = { $ended ->
        [yes] { $married ->
            [yes] { $sex ->
                [M] ex marito
                [F] ex moglie
               *[other] ex coniuge
            }
           *[no] { $sex ->
                [M] ex compagno
                [F] ex compagna
               *[other] ex partner
            }
        }
       *[no] { $married ->
            [yes] { $sex ->
                [M] marito
                [F] moglie
               *[other] coniuge
            }
           *[no] { $sex ->
                [M] compagno
                [F] compagna
               *[other] partner
            }
        }
    }
kin-parent-in-law = { $sex ->
        [M] suocero
        [F] suocera
       *[other] suocero o suocera
    }
kin-child-in-law = { $sex ->
        [M] genero
        [F] nuora
       *[other] genero o nuora
    }
kin-sibling-in-law = { $sex ->
        [M] cognato
        [F] cognata
       *[other] cognato o cognata
    }
kin-step-parent = { $sex ->
        [M] patrigno
        [F] matrigna
       *[other] patrigno o matrigna
    }
kin-step-child = { $sex ->
        [M] figliastro
        [F] figliastra
       *[other] figliastro o figliastra
    }
kin-note-step-parent = { $sex ->
        [F] compagna
       *[other] compagno
    } { $next ->
        [M] del padre
        [F] della madre
       *[other] di un genitore
    }
kin-note-step-child = figlio { $via ->
        [M] del marito
        [F] della moglie
       *[other] del coniuge
    }
kin-foster-parent = { $sex ->
        [M] padre affidatario
        [F] madre affidataria
       *[other] genitore affidatario
    }
kin-foster-child = { $sex ->
        [M] figlio in affido
        [F] figlia in affido
       *[other] minore in affido
    }
kin-guardian = { $sex ->
        [F] tutrice
       *[other] tutore
    }
kin-ward = { $sex ->
        [F] pupilla
       *[other] pupillo
    }
kin-relative-of-partner = { $relative } { $via ->
        [M] del marito
        [F] della moglie
       *[other] del coniuge
    }
kin-partner-of-relative = { $sex ->
        [M] marito
        [F] moglie
       *[other] coniuge
    } { $rsex ->
        [F] di una
       *[other] di un
    } { $relative }
kin-affine = { $sex ->
        [F] parente acquisita
       *[other] parente acquisito
    }
kin-related = { $sex ->
        [F] collegata dai registri della famiglia
       *[other] collegato dai registri della famiglia
    }
kin-note-adoptive = per adozione
kin-of = di { $name }
kin-hop-parent = { $line ->
        [adoption] { $sex ->
            [M] padre adottivo
            [F] madre adottiva
           *[other] genitore adottivo
        }
        [foster] { $sex ->
            [M] padre affidatario
            [F] madre affidataria
           *[other] genitore affidatario
        }
        [guardianship] { $sex ->
            [F] tutrice
           *[other] tutore
        }
        [step] { $sex ->
            [M] patrigno
            [F] matrigna
           *[other] patrigno o matrigna
        }
       *[other] { $sex ->
            [M] padre
            [F] madre
           *[other] genitore
        }
    }
kin-hop-child = { $line ->
        [adoption] { $sex ->
            [M] figlio adottivo
            [F] figlia adottiva
           *[other] figlio adottivo
        }
        [foster] { $sex ->
            [M] figlio in affido
            [F] figlia in affido
           *[other] minore in affido
        }
        [guardianship] { $sex ->
            [F] pupilla
           *[other] pupillo
        }
        [step] { $sex ->
            [M] figliastro
            [F] figliastra
           *[other] figliastro o figliastra
        }
       *[other] { $sex ->
            [M] figlio
            [F] figlia
           *[other] figlio o figlia
        }
    }
kin-group-partners = Coniugi e compagni
kin-group-ancestors = Ascendenti
kin-group-descendants = Discendenti
kin-group-siblings = Fratelli e sorelle
kin-group-uncles = Zii e zie
kin-group-nephews = Figli di fratelli e sorelle
kin-group-cousins = Cugini e cugine
kin-group-in-laws = Parenti acquisiti
kin-group-step = Famiglia ricomposta
kin-group-care = Affido e tutela
kin-group-rites = Riti e usanze
kin-group-others = Altri legami
kin-find-title = Che parentela c’è?
kin-find-lede = Scegli una persona: il suo legame con { $name }, passo dopo passo.
kin-find-label = Persona
kin-find-choose = Scegli una persona…
kin-find-go = Mostra il legame
kin-find-none = I registri della famiglia non collegano { $a } e { $b }.
kin-path = Il legame, passo per passo
kin-degree = parentela di { $n }° grado
kin-all-title = Tutta la parentela di { $name }
kin-all-count = { $n ->
        [one] una persona
       *[other] { $n } persone
    }
kin-common-list = Antenati comuni: { $names }
kin-also-list = anche { $list }
kin-rites-help = Padrini e altri legami creati da una tradizione; l'albero li disegna tratteggiati.
kin-rite-recorded = Il registro dice «{ $label }».
kin-hidden = una persona che non puoi vedere
kin-native = Nelle parole della tradizione
kin-legend-rite = Padrino, rito o usanza
kin-tree-root = { $term } { $of }
kin-editor-rite = Rito o usanza
kin-editor-rite-none = Nessuno: un semplice legame
kin-editor-rite-hint = Che cosa questa persona è per l’altra. La scelta compila la relazione e le parole, e il registro conserva il rito.
kin-editor-rite-option = { $rite }: { $word }
kin-tradition-christian = Cristiana
kin-tradition-orthodox = Cristiana ortodossa
kin-tradition-hispanic = Ispanica e filippina
kin-tradition-jewish = Ebraica
kin-tradition-islamic = Islamica
kin-tradition-south-asian = Dell’Asia meridionale
kin-tradition-east-asian = Dell’Asia orientale
kin-tradition-african = Africana
kin-tradition-indigenous = Indigena e del Pacifico
kin-tradition-civil = Civile e laica
kin-rite-godparent = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio o figlioccia
        }
       *[other] { $sex ->
            [M] padrino
            [F] madrina
           *[other] padrino o madrina
        }
    }
kin-rite-godparent-name = Padrinato
kin-rite-godparent-note = Padrino o madrina, come dice il registro.
kin-rite-baptism = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio o figlioccia
        }
       *[other] { $sex ->
            [M] padrino
            [F] madrina
           *[other] padrino o madrina
        }
    }
kin-rite-baptism-name = Battesimo
kin-rite-baptism-note = Padrino o madrina di battesimo.
kin-rite-confirmation = { $role ->
        [sponsored] { $sex ->
            [M] cresimando
            [F] cresimanda
           *[other] cresimando
        }
       *[other] { $sex ->
            [M] padrino di cresima
            [F] madrina di cresima
           *[other] padrino o madrina di cresima
        }
    }
kin-rite-confirmation-name = Cresima
kin-rite-confirmation-note = Padrino o madrina di cresima o di crismazione.
kin-rite-wedding-sponsor = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio di nozze
            [F] figlioccia di nozze
           *[other] figlioccio di nozze
        }
       *[other] { $sex ->
            [M] padrino di nozze
            [F] madrina di nozze
           *[other] padrino o madrina di nozze
        }
    }
kin-rite-wedding-sponsor-name = Padrinato di nozze
kin-rite-wedding-sponsor-note = Padrino o madrina di un matrimonio: padrino de boda, ninong, koumbaros.
kin-rite-kum = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio o figlioccia
        }
       *[other] { $sex ->
            [M] padrino
            [F] madrina
           *[other] padrino o madrina
        }
    }
kin-rite-kum-name = Kumstvo
kin-rite-kum-note = Padrino o testimone di nozze nella tradizione ortodossa slava; il kumstvo lega due famiglie.
kin-rite-koumbaros = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio o figlioccia
        }
       *[other] { $sex ->
            [M] padrino
            [F] madrina
           *[other] padrino o madrina
        }
    }
kin-rite-koumbaros-name = Koumbaros
kin-rite-koumbaros-note = Testimone di nozze o padrino nella tradizione ortodossa greca.
kin-rite-padrino = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio o figlioccia
        }
       *[other] { $sex ->
            [M] padrino
            [F] madrina
           *[other] padrino o madrina
        }
    }
kin-rite-padrino-name = Padrino
kin-rite-padrino-note = Padrino o madrina: al battesimo, alla prima comunione, alla quinceañera.
kin-rite-compadre = { $sex ->
        [M] compare
        [F] comare
       *[other] compare o comare
    }
kin-rite-compadre-name = Compadrazgo
kin-rite-compadre-note = Il legame tra i genitori di un bambino e i suoi padrini.
kin-rite-ninong = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio o figlioccia
        }
       *[other] { $sex ->
            [M] padrino
            [F] madrina
           *[other] padrino o madrina
        }
    }
kin-rite-ninong-name = Ninong
kin-rite-ninong-note = Ninong o ninang: padrino di battesimo, cresima o nozze nelle Filippine.
kin-rite-sandek = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio
        }
       *[other] sandek
    }
kin-rite-sandek-name = Brit milà: sandek
kin-rite-sandek-note = Tiene il bambino durante il brit milà.
kin-rite-kvatter = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio
        }
       *[other] { $sex ->
            [F] kvatterin
           *[other] kvatter
        }
    }
kin-rite-kvatter-name = Brit milà: kvatter
kin-rite-kvatter-note = Porta il bambino al brit milà.
kin-rite-milk-parent = { $role ->
        [sponsored] { $sex ->
            [M] figlio di latte
            [F] figlia di latte
           *[other] figlio di latte
        }
       *[other] { $sex ->
            [M] padre di latte
            [F] madre di latte
           *[other] genitore di latte
        }
    }
kin-rite-milk-parent-name = Parentela di latte
kin-rite-milk-parent-note = Ha allattato il bambino: nel diritto islamico la parentela di latte crea una famiglia e vieta il matrimonio, come la nascita.
kin-rite-milk-sibling = { $sex ->
        [M] fratello di latte
        [F] sorella di latte
       *[other] fratello o sorella di latte
    }
kin-rite-milk-sibling-name = Fratelli di latte
kin-rite-milk-sibling-note = Allattati dalla stessa donna.
kin-rite-kafala = { $role ->
        [sponsored] { $sex ->
            [F] pupilla (kafala)
           *[other] pupillo (kafala)
        }
       *[other] { $sex ->
            [F] tutrice (kafala)
           *[other] tutore (kafala)
        }
    }
kin-rite-kafala-name = Kafala
kin-rite-kafala-note = Accoglie e cresce un bambino senza la filiazione di un’adozione, secondo il diritto islamico.
kin-rite-rakhi = { $sex ->
        [M] fratello di rakhi
        [F] sorella di rakhi
       *[other] fratello o sorella di rakhi
    }
kin-rite-rakhi-name = Rakhi
kin-rite-rakhi-note = A Raksha Bandhan una sorella lega un rakhi al polso di un fratello, che promette di proteggerla.
kin-rite-guru = { $role ->
        [sponsored] { $sex ->
            [M] discepolo
            [F] discepola
           *[other] discepolo
        }
       *[other] guru
    }
kin-rite-guru-name = Guru
kin-rite-guru-note = Maestro spirituale e discepolo.
kin-rite-gan-parent = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio
            [F] figlioccia
           *[other] figlioccio o figlioccia
        }
       *[other] { $sex ->
            [M] padrino
            [F] madrina
           *[other] padrino o madrina
        }
    }
kin-rite-gan-parent-name = Gan qin
kin-rite-gan-parent-note = Padrini scelti in Cina e nelle famiglie cinesi, spesso per proteggere un bambino.
kin-rite-sworn-sibling = { $sex ->
        [M] fratello giurato
        [F] sorella giurata
       *[other] fratello o sorella giurati
    }
kin-rite-sworn-sibling-name = Fratellanza giurata
kin-rite-sworn-sibling-note = Fratelli o sorelle per giuramento (jiebai).
kin-rite-name-giver = { $role ->
        [sponsored] { $sex ->
            [M] bambino nominato
            [F] bambina nominata
           *[other] bambino nominato
        }
       *[other] { $sex ->
            [F] madrina del nome
           *[other] padrino del nome
        }
    }
kin-rite-name-giver-name = Imposizione del nome
kin-rite-name-giver-note = Ha dato il nome al bambino (nazuke-oya in Giappone).
kin-rite-nakodo = { $role ->
        [sponsored] { $sex ->
            [M] sposo combinato
            [F] sposa combinata
           *[other] coniuge combinato
        }
       *[other] { $sex ->
            [F] mediatrice di nozze
           *[other] mediatore di nozze
        }
    }
kin-rite-nakodo-name = Nakōdo
kin-rite-nakodo-note = Ha combinato il matrimonio e ne è stato testimone (nakōdo in Giappone).
kin-rite-namesake = { $sex ->
        [F] omonima
       *[other] omonimo
    }
kin-rite-namesake-name = Omonimo
kin-rite-namesake-note = Porta lo stesso nome: in Africa centrale e orientale, un legame di doni e di cure (ndoyi, somo).
kin-rite-initiation = { $role ->
        [sponsored] { $sex ->
            [F] iniziata
           *[other] iniziato
        }
       *[other] { $sex ->
            [F] madrina d’iniziazione
           *[other] padrino d’iniziazione
        }
    }
kin-rite-initiation-name = Iniziazione
kin-rite-initiation-note = Ha guidato la persona in un’iniziazione.
kin-rite-blood-pact = { $sex ->
        [M] fratello di sangue
        [F] sorella di sangue
       *[other] fratello o sorella di sangue
    }
kin-rite-blood-pact-name = Patto di sangue
kin-rite-blood-pact-note = Uniti da un patto di sangue.
kin-rite-hanai = { $role ->
        [sponsored] { $sex ->
            [M] figlio hānai
            [F] figlia hānai
           *[other] figlio hānai
        }
       *[other] { $sex ->
            [M] padre hānai
            [F] madre hānai
           *[other] genitore hānai
        }
    }
kin-rite-hanai-name = Hānai
kin-rite-hanai-note = Cresciuto come figlio proprio secondo l’usanza hawaiana, senza adozione legale.
kin-rite-hunka = { $role ->
        [sponsored] { $sex ->
            [M] figlio hunka
            [F] figlia hunka
           *[other] figlio hunka
        }
       *[other] { $sex ->
            [M] padre hunka
            [F] madre hunka
           *[other] genitore hunka
        }
    }
kin-rite-hunka-name = Hunka
kin-rite-hunka-note = Reso parente nella cerimonia lakota dello hunka.
kin-rite-civil-sponsor = { $role ->
        [sponsored] { $sex ->
            [M] figlioccio civile
            [F] figlioccia civile
           *[other] figlioccio civile
        }
       *[other] { $sex ->
            [M] padrino civile
            [F] madrina civile
           *[other] padrino civile
        }
    }
kin-rite-civil-sponsor-name = Padrinato civile
kin-rite-civil-sponsor-note = Padrino o madrina in una cerimonia civile del nome.
kin-rite-guardian = { $role ->
        [sponsored] { $sex ->
            [F] pupilla
           *[other] pupillo
        }
       *[other] { $sex ->
            [F] tutrice
           *[other] tutore
        }
    }
kin-rite-guardian-name = Tutela
kin-rite-guardian-note = Tutore o tutrice, come dice il registro.

## Polygamous unions (docs/adr/0021): one marriage per spouse, the
## co-spouses, the editor's two fields.
kin-co-spouse = { $sex ->
        [M] co-marito
        [F] co-moglie
       *[other] co-coniuge
    }
union-co-spouses = { $sex ->
        [F] { $n ->
            [one] Co-moglie
           *[other] Co-mogli
        }
        [M] { $n ->
            [one] Co-marito
           *[other] Co-mariti
        }
       *[other] { $n ->
            [one] Co-coniuge
           *[other] Co-coniugi
        }
    }
tree-key-polygamous = Unione poligama
family-primary = Al centro dell’unione
family-primary-hint = In un’unione poligama, la persona sposata con ciascuno degli altri. Gli altri non sono sposati tra loro.
family-child-parent = Altro genitore
family-child-parent-hint = In un’unione poligama, il coniuge che è l’altro genitore di questo figlio. Se non è scelto, la scheda non lo dice.

## Emblems (docs/adr/0022): the picture that stands for a family, and for
## the archive's.
emblem-family-edit = Emblema o stemma
emblem-family-title = L’emblema della famiglia
emblem-archive-title = L’emblema della famiglia dell’archivio
emblem-archive-edit = Emblema o stemma della famiglia
emblem-saved = L’emblema è salvato.
emblem-choose = L’immagine che rappresenta la famiglia
emblem-help = Uno stemma, un segno, una fotografia: ciò che rappresenta questa famiglia. È mostrato per intero, accanto all’unione in ogni scheda e sul nodo della coppia nell’albero.
emblem-archive-help = Uno stemma, un segno, una fotografia: ciò che rappresenta la famiglia di cui parla questo archivio. È mostrato per intero in cima a ogni pagina e nella panoramica, a tutti i visitatori.
emblem-mode-auto-note = L’immagine che questa famiglia collega come suo emblema, se c’è.
emblem-mode-none = Nessun emblema
emblem-no-images = Ancora nessuna immagine: aggiungetene una qui sotto.
emblem-upload-title = Aggiungi un’immagine
emblem-upload-button = Aggiungila e usala
emblem-not-an-image = Un emblema è un’immagine: PNG, JPEG, GIF, WebP o BMP.
emblem-back = Torna all’emblema
emblem-no-family-title = Famiglia non trovata
emblem-no-family-detail = Questa famiglia non è nell’archivio, o non potete vederla.

## The Artificial family genealogist, AFG (docs/adr/0023): its room, where
## its computing comes from, and a test.
admin-nav-afg = AFG
afg-title = Genealogista familiare artificiale
afg-lede = Da dove viene il calcolo dell'AFG, e una prova.
afg-kpi-source = Elaborazione
afg-ready = pronto
afg-off-note = Scegliete da dove viene l’elaborazione.
afg-not-ready = incompleto
afg-kpi-sent = File inviati
afg-kpi-sent-note = { $kb } KB, circa { $tokens } token
afg-kpi-last = Ultima prova
afg-never = nessuna ancora
afg-run-open = Aprirla
afg-source = Da dove viene l’elaborazione
afg-source-intro = Un indirizzo; un indirizzo con chiave e modello; o un agente autenticato su questa macchina.
afg-kind = Modo di elaborazione
afg-kind-hint = Un servizio si raggiunge via rete; un agente gira su questa macchina e deve esservi già autenticato.
afg-kind-off = Nessuno
afg-kind-anthropic = Claude (API di Anthropic)
afg-kind-ollama = Ollama
afg-kind-openai = OpenAI
afg-kind-mistral = Mistral
afg-kind-vllm = vLLM
afg-kind-gemini = Gemini
afg-kind-claude-code = Claude Code
afg-kind-codex = Codex
afg-kind-opencode = OpenCode
afg-group-address = Un indirizzo
afg-group-service = Un indirizzo, una chiave API e un modello
afg-group-agent = Un agente di codice su questa macchina
afg-url = Indirizzo
afg-url-hint = Vuoto: l’indirizzo abituale del servizio. Chiamata:
afg-key = Chiave API
afg-key-set = È conservata una chiave che termina con { $end }.
afg-key-unset = Nessuna chiave conservata.
afg-key-hint = Vuoto mantiene la chiave salvata, che non viene più mostrata.
afg-key-clear = Dimentica la chiave
afg-model = Modello
afg-model-hint = Il nome del modello come lo scrive il servizio. Ogni nome è inviato così; vuoto, quello predefinito.
afg-model-unlisted = Non è tra i suggerimenti: viene inviato così come è scritto.
afg-effort = Impegno
afg-effort-hint = Quanto ragiona il modello. Ogni parola è inviata così; vuoto, quella predefinita.
afg-effort-unlisted = Non è tra gli impegni di questo modello: viene inviato così come è scritto.
afg-effort-default = predefinito
afg-program = Programma
afg-program-found = Trovato:
afg-program-missing = Non trovato su questa macchina.
afg-program-hint = Vuoto: cercato dove lo mette il suo installatore. Altrimenti, il percorso completo del programma.
afg-sensitive = Invia anche le classi sensibili
afg-sensitive-hint = Non spuntata, salute, biometria, genetica, casellario e comportamento restano fuori, come in un'esportazione condivisa.
afg-timeout = Tempo concesso (secondi)
afg-timeout-hint = Quanto può durare una risposta prima di rinunciare.
afg-test = Prova
afg-test-intro = Invia l'archivio (JSON, senza foto) e la tua memoria con una domanda; la risposta si apre in una finestra.
afg-sent-files = File
afg-sent-files-value = { $n ->
        [one] { $n } file JSON
       *[other] { $n } file JSON
    }, { $kb } KB
afg-sent-tokens = Token, circa
afg-sent-records = Schede
afg-sent-records-value = { $persons ->
        [one] { $persons } persona
       *[other] { $persons } persone
    }, { $families ->
        [one] { $families } famiglia
       *[other] { $families } famiglie
    }
afg-sent-classes = Classi sensibili
afg-sent-classes-in = inviate
afg-sent-classes-out = escluse
afg-sent-question = Domanda
afg-privacy = Il test invia le schede della famiglia, viventi compresi, al calcolo scelto qui sopra.
afg-test-button = Invia la prova
afg-test-needs = Salvate prima un modo di elaborazione completo.
afg-test-question = Dammi la top 3 delle famiglie con il maggior numero di individui dichiarati.
afg-test-empty = Bundle vuoto
afg-saved = Salvato.
afg-error-not-saved = Non salvato: { $error }
afg-error-off = Nessun modo di elaborazione scelto.
afg-error-archive = Impossibile leggere l’archivio: { $error }
afg-error-url = L’indirizzo deve iniziare con http:// o https://.
afg-error-key = Questo servizio richiede una chiave API.
afg-error-model = Qui serve un modello, scritto con lettere, cifre e . _ : / @ [ ] + -
afg-error-effort = Un impegno si scrive con lettere, cifre e . _ : / @ [ ] + -
afg-error-program = Il programma deve essere quello dell’agente (claude, codex o opencode), con il percorso completo.
afg-run-state-running = In attesa della risposta
afg-run-state-done = Risposta ricevuta
afg-run-state-failed = Non riuscita
afg-run-seconds = { $s } s
afg-run-usage = Token letti
afg-run-usage-out = { $n } scritti
afg-run-cost = { $usd } USD
afg-run-waiting = L’archivio è inviato; la risposta apparirà qui. Questa pagina richiede ogni tre secondi.
afg-run-model = Modello:
afg-run-failed-hint = Controllate il modo di elaborazione, la rete che questo sito può raggiungere e, per un agente, che sia installato e collegato con l’account di questo sito.
afg-run-gone-detail = Le prove sono conservate per la vita del processo, le ultime dieci. Inviatene una nuova.

## The AFG's brains, its window and who may use it (docs/adr/0024).
afg-kpi-brain = La sua memoria
afg-brain-exchanges = { $n ->
        [one] { $n } scambio
       *[other] { $n } scambi
    }
afg-brain-note = { $facts ->
        [one] { $facts } fatto ricordato
       *[other] { $facts } fatti ricordati
    } · { $brains ->
        [one] { $brains } account con memoria
       *[other] { $brains } account con memoria
    }
afg-brain-no-account = Questa sessione non ha un account, quindi nessuna memoria.
afg-access-count = Account che possono parlare con l'AFG: { $n }
afg-access-set = Si imposta nella pagina Account
afg-result-ok = Riuscito
afg-result-error = Errore
afg-result-close = Chiudi
afg-result-remembered = Conservato nella memoria di { $name }:
afg-result-nothing-kept = Niente di nuovo conservato nella memoria di { $name }.
accounts-afg = AFG
accounts-afg-allowed = Può parlare con l’AFG
accounts-afg-yes = sì
accounts-afg-no = no
accounts-afg-hint = Spento se non spuntato. La sua memoria è tenuta a parte, accanto all'archivio.

## An account's AFG brain on its own page: seen and forgotten (docs/adr/0024 §6).
account-brain-title = La memoria del vostro AFG
account-brain-intro = Ciò che l'AFG ricorda di te, riproposto a ogni domanda. Cancella ciò che vuoi; nessun altro lo vede.
account-brain-no-access = L’AFG non è aperto al vostro account: lo decide un amministratore nella pagina Account.
account-brain-none = L’AFG non ricorda ancora nulla di voi.
account-brain-facts = Ciò che ricorda
account-brain-facts-none = Non ha conservato alcun fatto su di voi.
account-brain-exchanges = I vostri scambi con lui
account-brain-exchanges-none = Nessuno scambio è conservato.
account-brain-older = { $n ->
        [one] { $n } scambio più vecchio è conservato e non è elencato qui.
       *[other] { $n } scambi più vecchi sono conservati e non sono elencati qui.
    }
account-brain-context-test = Prova nella sala dell’AFG
account-brain-answer = La sua risposta
account-brain-forget = Dimentica
account-brain-forget-all = Dimentica tutto…
account-brain-forget-all-confirm = Sì, dimentica tutto ciò che l’AFG ricorda di me
account-brain-forget-all-button = Dimentica tutto
account-brain-forgotten = Dimenticato: l’AFG non lo saprà più.
account-brain-forgotten-all = L’AFG non ricorda più nulla di voi.
account-brain-already = Era già dimenticato.
account-brain-confirm-needed = Spuntate la casella per dimenticare tutto.
account-brain-error = Non è stato dimenticato nulla: { $error }
afg-brain-see = Vedi e dimentica

## My personal assistant: projects, conversations, code of conduct, second brain (docs/adr/0026).
assistant-title = Il mio assistente personale
assistant-entry = I tuoi progetti e le tue conversazioni con l'AFG.
assistant-lede = I tuoi progetti con l'AFG, ognuno con il suo codice di condotta, la sua memoria e le sue conversazioni.
assistant-shut-no-account = Questa sessione non ha un account, quindi nessun assistente personale. Accedete con il vostro account.
assistant-shut-not-open = L’AFG non è aperto al vostro account: lo decide un amministratore nella pagina Account.
assistant-shut-broken = Il vostro spazio non si è potuto aprire. Riprovate tra un momento; se continua, avvisate un amministratore.
assistant-shut-gone = Qui non c’è nulla: questo progetto o questa conversazione non esiste, o non è vostro.
assistant-back-account = Torna al mio account
assistant-new-project = Nuovo progetto
assistant-not-ready = Il calcolo dell'AFG non è ancora configurato: per ora solo lettura.
assistant-conversations-count = { $n ->
        [one] { $n } conversazione
       *[other] { $n } conversazioni
    }
assistant-conduct-set = codice di condotta impostato
assistant-empty = Ancora nessun progetto. Un progetto riunisce le conversazioni su uno stesso tema.
assistant-project-name = Nome del progetto
assistant-create = Crea il progetto
assistant-facts-count = { $n ->
        [one] { $n } fatto ricordato
       *[other] { $n } fatti ricordati
    }
assistant-archive-on = con l’archivio della famiglia
assistant-start = Inizia una conversazione
assistant-conversations = Conversazioni
assistant-no-conversation = Ancora nessuna conversazione: la vostra prima domanda ne apre una.
assistant-conduct = Codice di condotta
assistant-conduct-label = Come deve comportarsi l’AFG in questo progetto
assistant-conduct-hint = Le vostre istruzioni per ogni conversazione di questo progetto: il tono, la lingua, a cosa guardare, cosa lasciare da parte. L’AFG le segue, a meno che non gli chiedano di ingannarvi o di nuocere.
assistant-save = Salva
assistant-memory = Memoria del progetto
assistant-memory-intro = Ciò che l’AFG ha scelto di conservare dalle conversazioni di questo progetto: il suo secondo cervello. Gli viene ridata a ogni messaggio del progetto. Fategli dimenticare ciò che non volete più che sappia.
assistant-forget = Dimentica
assistant-forget-all = Dimentica tutto…
assistant-forget-all-confirm = Sì, dimentica tutto ciò che l’AFG ricorda di questo progetto
assistant-forget-all-button = Dimentica tutto
assistant-memory-none = L’AFG non ricorda ancora nulla di questo progetto.
assistant-settings = Impostazioni
assistant-archive = Dare all’AFG l’archivio della famiglia in questo progetto
assistant-archive-hint = Ogni scheda dell’archivio, viventi compresi, va allora con ogni messaggio là dove l’AFG elabora. Le classi sensibili restano fuori, a meno che un amministratore non abbia deciso altrimenti.
assistant-archive-size = File JSON: { $files }; token per messaggio: circa { $tokens }.
assistant-avatar-upload = Un’immagine vostra come avatar
assistant-avatar-send = Usa questa immagine
assistant-delete-project = Elimina il progetto
assistant-delete-project-confirm = Sì, elimina questo progetto, le sue conversazioni, i loro file e la sua memoria
assistant-delete = Elimina
assistant-conversation-settings = Impostazioni della conversazione
assistant-conversation-title = Titolo della conversazione
assistant-delete-conversation = Elimina la conversazione
assistant-delete-conversation-confirm = Sì, elimina questa conversazione e i suoi messaggi
assistant-thread = La conversazione
assistant-reply = Il vostro messaggio
assistant-sending = Invio…
assistant-attach = Allega
assistant-send = Invia
assistant-composer-hint = Invio manda, Maiusc+Invio va a capo. Documenti, foto, suono o video: trascinateli, incollateli o allegateli, al massimo { $n } file, { $mb } MB ciascuno.
assistant-avatar-choose = Scegli il suo avatar
assistant-avatar = Avatar
assistant-avatar-auto = Scelto per lui
assistant-avatar-own = La vostra immagine
assistant-kept = { $n ->
        [one] Conservato nella memoria del progetto: { $n } fatto
       *[other] Conservati nella memoria del progetto: { $n } fatti
    }
assistant-error-answer = L’AFG non ha potuto rispondere.
assistant-retry = Chiedi di nuovo
assistant-unanswered = Questo messaggio è rimasto senza risposta.
assistant-writing = L’AFG sta scrivendo
assistant-saved = Salvato.
assistant-project-deleted = Il progetto è eliminato.
assistant-conversation-deleted = La conversazione è eliminata.
assistant-memory-forgotten = Dimenticato: l’AFG non lo riceverà più.
assistant-memory-forgotten-all = L’AFG non ricorda più nulla di questo progetto.
assistant-avatar-saved = L’avatar è salvato.
assistant-error-name = Serve un nome.
assistant-error-too-many = Ce ne sono già quanti un account può conservarne.
assistant-error-not-saved = Non salvato: il server non ha potuto scriverlo. Riprovate tra un momento.
assistant-error-conduct-long = Il codice di condotta è troppo lungo.
assistant-error-confirm = Spuntate la casella per confermare.
assistant-error-too-big = Il messaggio è troppo pesante.
assistant-error-file-big = Un file è troppo pesante.
assistant-error-too-many-files = Troppi file in un solo messaggio.
assistant-error-text-long = Il messaggio è troppo lungo.
assistant-error-empty = Scrivete un messaggio o allegate un file.
assistant-error-busy = L’AFG risponde già a tutte le conversazioni che può. Riprovate tra un momento.
assistant-error-wait = L’AFG sta ancora rispondendo in questa conversazione.
assistant-error-picture = Questo file non è un’immagine che il sito sappia leggere.
assistant-domain-genealogy = Genealogia
assistant-domain-medicine = Salute e medicina
assistant-domain-work = Lavoro e carriera
assistant-domain-money = Denaro
assistant-domain-law = Diritto e documenti
assistant-domain-learning = Imparare
assistant-domain-home = Casa
assistant-domain-cooking = Cucina
assistant-domain-travel = Viaggi
assistant-domain-nature = Natura e giardino
assistant-domain-wellbeing = Benessere
assistant-domain-family = Vita di famiglia
assistant-domain-arts = Arte e cultura
assistant-domain-sports = Sport
assistant-domain-tech = Tecnologia
assistant-domain-science = Scienza
assistant-domain-history = Storia e memoria
assistant-domain-admin = Pratiche
assistant-domain-pets = Animali
assistant-domain-rites = Feste e riti

## The AFG on a person's record: a face or a photograph restored, a document read, the summary (docs/adr/0029).

summary-title = Sintesi
summary-write = Scrivi la sintesi
summary-edit = Modifica la sintesi
summary-language = Lingua
summary-by-ai = Scritta dall'IA ({ $generator }): da rileggere.
summary-note-title = Sintesi — { $name }
summary-category = Sintesi
summary-edit-title = La sintesi della persona
afg-summary = Scrivi con l'IA
afg-summary-busy = Scrittura…
afg-retouch = Ritocco IA
afg-retouch-busy = Ritocco…
afg-retouched = Restaurata da un'IA: non è una fotografia dell'epoca.
afg-read = Leggi il documento
afg-read-busy = Lettura…
afg-read-note-title = Trascrizione — { $name }
afg-read-category = Trascrizione
afg-rework = Rielabora l'immagine con l'IA
afg-rework-busy = Rielaborazione…
afg-images-off-admin = Nessun servizio di immagini: sceglilo in Amministrazione › AFG › Immagini.
notes-manage-in-media = Modificarle o eliminarle nella scheda Media
notes-delete-short = Elimina
notes-none-yet = Ancora nessuna nota su questa persona.
afg-record-title = L'IA nella scheda
afg-record-back = Torna alla scheda
afg-record-no-access = L'AFG non è aperto al tuo account. Un amministratore può aprirlo nella pagina Account.
afg-record-images-off = Nessun servizio di immagini è configurato. Un amministratore lo sceglie in Amministrazione › AFG › Immagini.
afg-record-text-off = Il calcolo dell'AFG non è configurato. Un amministratore lo sceglie in Amministrazione › AFG.
afg-record-no-face = Questa persona non ha una fotografia da rielaborare.
afg-record-no-document = Questo documento non è tra i file di questa persona, il suo file non è conservato qui, oppure è già una copia fatta da un'IA.
afg-record-not-a-picture = Si può ritoccare solo un'immagine.
afg-record-not-readable = Si possono leggere solo un'immagine o un PDF.
afg-record-pdf-unread = Il calcolo attuale dell'AFG non sa leggere un PDF. OpenAI, Claude, Gemini, Claude Code e OpenCode sì.
afg-record-failed = L'IA non ci è riuscita: { $reason }
afg-record-bad-picture = Il servizio non ha restituito un'immagine che questo sito possa conservare.
afg-record-no-text = L'IA non ha trovato alcun testo in questo documento.
afg-record-stale = Nel frattempo la scheda è cambiata. Ricarica la pagina e riprova.
afg-record-refused = L'archivio l'ha rifiutato: { $reason }
afg-record-summary-not-yours = Un altro membro ha scritto questa sintesi: solo il suo autore o un amministratore possono sostituirla.
afg-images = Immagini
afg-images-intro = Da dove vengono le immagini dell'AFG: un servizio a pagamento (OpenAI, Google, Canva) o una macchina di famiglia con scheda grafica.
afg-image-kind = Servizio di immagini
afg-image-kind-hint = I modelli di OpenAI e di Google, l'IA di Canva con il tuo account, o un server della famiglia: Ollama, qualsiasi server compatibile OpenAI, ComfyUI.
afg-image-kind-off = Nessuno
afg-image-kind-openai = OpenAI (GPT Image)
afg-image-kind-gemini = Google Gemini (Nano Banana)
afg-image-url-hint = Vuoto: l'indirizzo abituale del servizio. Per un tuo server: il suo indirizzo (es. http://192.168.1.20:8000/v1).
afg-image-key-hint = Cambiare servizio fa dimenticare la chiave: inserisci insieme quella del nuovo.
afg-image-model-hint = Vuoto: il primo proposto; un altro modello viene inviato così come è scritto.
afg-images-privacy = La fotografia viene inviata a questo servizio ogni volta che un membro lo chiede. Scegli un servizio a cui la tua famiglia affida le sue immagini.
afg-images-saved = Il servizio di immagini è salvato.
afg-images-ready = Le immagini sono pronte: i pulsanti della scheda possono richiederle.
afg-images-not-ready = Nessun servizio di immagini: i pulsanti immagine della scheda sono disattivati.
afg-images-choose = Scegli il servizio di immagini

## Canva and the family's own servers as the AFG's picture service (docs/adr/0030).

afg-image-kind-canva = Canva (la sua IA, con il tuo account)
afg-image-kind-ollama = Ollama (sperimentale)
afg-image-kind-openai-compatible = Un tuo server (compatibile OpenAI)
afg-image-kind-comfyui = ComfyUI (il tuo workflow)
afg-image-workflow = Workflow ComfyUI (formato API)
afg-image-workflow-hint = Esportato da ComfyUI in formato API («Save (API Format)»). La fotografia va in ogni nodo LoadImage, e questo segno, in un testo, diventa l'istruzione di restauro:
afg-image-key-optional-hint = Facoltativa: solo se il tuo server ne chiede una.
afg-images-at-home = La fotografia resta nella tua rete: questo servizio gira su una macchina della famiglia.
afg-images-saved-canva = Canva è scelto: ora collega un account.
afg-canva-connect = Collega Canva
afg-canva-connected = Collegato all'account Canva { $account }.
afg-canva-connected-unnamed = Un account Canva è collegato.
afg-canva-not-connected = Nessun account Canva è collegato.
afg-canva-disconnect = Scollega Canva
afg-canva-disconnect-confirm = Scollegare questo account Canva? Le immagini già fatte restano nell'archivio.
afg-canva-step-open = 1. Apri Canva e accetta
afg-canva-step-paste = 2. Incolla qui l'indirizzo della pagina a cui Canva ti ha rimandato (http://localhost…)
afg-canva-paste-hint = Quella pagina non si carica: è normale. Basta il suo indirizzo, che vale una sola volta.
afg-canva-finish = Completa il collegamento
afg-canva-finish-wait = Il collegamento richiede fino a un minuto: prepara anche, in Canva, la pagina su cui viene ritoccata ogni foto.
afg-canva-done-ready = Canva è collegato e la sua pagina di lavoro è pronta nella cartella «ax-genealogy». Dopo averla creata, Canva può sospendere i crediti IA per qualche minuto: se il primo ritocco viene rifiutato, riprova un po' più tardi.
afg-canva-cooldown = Canva ha sospeso i crediti IA di questo account per qualche minuto. Riprova tra poco.
afg-canva-done = Canva è collegato.
afg-canva-gone = Canva è scollegato.
afg-canva-save-first = Scegli prima Canva come servizio di immagini, e salva.
afg-canva-policy = Un solo collegamento serve tutta la famiglia: i ritocchi passano da questo account Canva, dai suoi crediti e dalla sua cartella «ax-genealogy». La politica d'uso di Canva prevede che ognuno colleghi il proprio account; collegando il tuo per l'AFG, te ne assumi la responsabilità.
afg-error-workflow = Il workflow ComfyUI manca o non è in formato API (gli serve almeno un nodo LoadImage).
afg-error-canva = Canva non è collegato: usa «Collega Canva».

## Impact: the Universal Synoptic Timeline (docs/adr/0034)

nav-impact = Impatto
impact-title = Impatto
impact-lede = Le vite della famiglia lungo la storia del mondo.
impact-span-all = Tutta la storia
impact-span-family = I secoli della famiglia
impact-placed = Persone collocate: { $n }
impact-undated = Senza data: { $n }
impact-hidden = Non mostrate: { $n }
impact-empty = Nessuna data ancora nota: nessuno può essere collocato.
impact-col-world = Mondo
impact-col-continents = Continenti
impact-col-family = La famiglia
impact-col-life = La sua vita
impact-col-country = Il suo paese
impact-continent-europe = Europa
impact-continent-africa = Africa
impact-continent-asia = Asia
impact-continent-americas = Americhe
impact-continent-oceania = Oceania
impact-kind-era = Civiltà e periodi
impact-kind-war = Guerre
impact-kind-epidemic = Epidemie
impact-kind-migration = Migrazioni
impact-kind-event = Punti di riferimento
impact-kind-economy = Economia e società
impact-kind-disaster = Carestie e catastrofi
impact-kind-invention = Grandi invenzioni
impact-kind-discovery = Grandi scoperte
impact-person-title = { $name } nel suo tempo
impact-back = Tutta la famiglia
impact-record = Apri la scheda
impact-states = Stati a cui appartennero i suoi luoghi
impact-countries-none = Nessun paese trovato nei suoi luoghi.
impact-unplaced = Nessuna data nota: questa persona non può essere collocata.
impact-see = Vedi l'impatto
impact-year-bc = { $year } a.C.
impact-now = oggi
impact-years = { $n ->
        [one] { $n } anno
       *[other] { $n } anni
    }
impact-months = { $n ->
        [one] { $n } mese
       *[other] { $n } mesi
    }
impact-days = { $n ->
        [one] { $n } giorno
       *[other] { $n } giorni
    }
impact-age = { $n ->
        [one] a { $n } anno
       *[other] a { $n } anni
    }
impact-under-way = già in corso alla sua nascita
impact-milestone-birth = Nascita
impact-milestone-death = Morte
impact-milestone-marriage = Matrimonio
impact-milestone-child = Nascita di { $name }
impact-context = Contesto
impact-stakes = La posta in gioco
impact-victims = Vittime
impact-close = Chiudi
impact-hint = Passa sopra un evento per vederne gli anni; fai clic per il contesto.

# A person created with what surrounds them (docs/adr/0039), and the
# pickers' refusals by what they pick.
person-new-title = Nuova persona
person-new-lede = Un nome che l'archivio non ha ancora diventa una nuova scheda, salvata con questa.
person-new-section-name = Nome e cognome
person-new-given = Nome
person-new-family = Cognome
person-new-nicknames = Soprannomi
person-new-nickname-add = Aggiungi un soprannome
person-new-nickname-quoted = «{ $nick }»
person-new-remove = Rimuovi
person-new-preview = Mostrato come
person-new-name-hint = Almeno un nome o un cognome.
person-new-section-life = Vita
person-new-section-texts = Biografia e nota
person-new-birth-place = Luogo di nascita
person-new-death-place = Luogo di morte
person-new-death-hint = Vuoto finché la persona è in vita.
person-new-place-hint = Un luogo che l'archivio non ha viene creato.
person-new-section-portrait = Ritratto
person-new-portrait-hint = PNG, JPEG, GIF o WebP, al massimo { $mb } MB.
person-new-bio-hint = Formattazione libera, come una nota.
person-new-section-note = Nota
person-new-note-hint = Una nota a parte su questa persona, immagini comprese. Senza titolo porta il nome della persona.
person-new-section-parents = Genitori, fratelli e sorelle
person-new-father = Padre
person-new-mother = Madre
person-new-siblings = Fratelli e sorelle
person-new-sibling-add = Aggiungi un fratello o una sorella
person-new-who-hint = Scegli dall'elenco o scrivi un nome: un nome che nessuno porta diventa una nuova scheda.
person-new-section-unions = Unioni e figli
person-new-union-n = Unione { $n }
person-new-partner = Coniuge o partner
person-new-union-type = Tipo di unione
person-new-children = Figli
person-new-child-add = Aggiungi un figlio
person-new-union-add = Aggiungi un'unione
person-new-save = Crea la scheda
person-new-hint-existing = Nell'archivio
person-new-hint-new = Nuova scheda
person-new-hint-ambiguous = Più schede portano questo nome: scegline una dall'elenco.
person-new-hint-place-existing = Luogo dell'archivio
person-new-hint-place-new = Nuovo luogo
person-new-field-error = «{ $typed }»: { $message }
person-new-error-name = Serve un nome o un cognome. Non è stato salvato nulla.
person-new-error-date = Una data si scrive 1890, 1890-06 o 1890-06-14. Non è stato salvato nulla.
person-new-error-same-parents = Il padre e la madre sono la stessa persona. Non è stato salvato nulla.
person-new-error-siblings-need-parent = Fratelli e sorelle formano una famiglia solo tramite un genitore: indica un genitore, o un fratello o una sorella già in una famiglia. Non è stato salvato nulla.
person-new-error-siblings-two-families = Questi fratelli e sorelle sono in famiglie diverse: indica i genitori. Non è stato salvato nulla.
person-new-error-portrait = Il ritratto non è un'immagine che questo sito sa mostrare: PNG, JPEG, GIF o WebP. Non è stato salvato nulla.
person-new-error-conflict = L'archivio è cambiato mentre scrivevi. Non è stato salvato nulla; salva di nuovo.
person-new-error-refused = L'archivio ha rifiutato il salvataggio. Non è stato salvato nulla.
person-new-created = { $others ->
        [0] Scheda creata.
        [one] Scheda creata, con un'altra scheda.
       *[other] Scheda creata, con altre { $others } schede.
    }
pick-error-place-not-found = In questo archivio non c'è nessun luogo con quel nome. Non è stato salvato nulla.
pick-error-place-ambiguous = Più di un luogo corrisponde. Scegline uno dall'elenco. Non è stato salvato nulla.
pick-error-source-not-found = In questo archivio non c'è nessuna fonte con quel nome. Non è stato salvato nulla.
pick-error-source-ambiguous = Più di una fonte corrisponde. Scegline una dall'elenco. Non è stato salvato nulla.
pick-error-thing-empty = Non è stato indicato nulla. Non è stato salvato nulla.
pick-error-thing-not-found = In questo archivio non c'è nulla con quel nome. Non è stato salvato nulla.
pick-error-thing-ambiguous = Più di una scheda corrisponde. Scegline una dall'elenco. Non è stato salvato nulla.
search-create = Crea «{ $name }»

# What a file sent to the site may weigh, and the kinds refused
# (docs/adr/0041).
uploads-title = Invio di file
uploads-lede = Quanto può pesare un file inviato al sito, e quali tipi di file l'archivio accetta.
uploads-sizes = Dimensioni massime
uploads-limit-document = File allegato a una scheda (MB)
uploads-limit-note-image = Immagine incollata in una nota (MB)
uploads-limit-assistant-file = File inviato all'assistente (MB)
uploads-limit-assistant-message = Tutti i file di un messaggio (MB)
uploads-limit-gedcom = File GEDCOM (MB)
uploads-limit-archive = Archivio importato per intero (MB)
uploads-limit-range = Da { $min } a { $max } MB; { $default } per impostazione predefinita.
uploads-kinds = Tipi di file
uploads-kinds-summary = { $accepted } tipi accettati su { $total }
uploads-family-image = Immagini
uploads-family-document = Documenti
uploads-family-audio = Audio
uploads-family-video = Filmati
uploads-kinds-hint = Un tipo non spuntato viene rifiutato all'invio; i file già nell'archivio restano.
uploads-save = Salva
uploads-saved = Impostazioni degli invii salvate.
uploads-error-range = { $field }: un numero intero da { $min } a { $max }. Non è stato salvato nulla.
uploads-not-saved = Non è stato possibile salvare le impostazioni: { $error }
uploads-summary = File allegati alle schede: al massimo { $document } MB; { $accepted } tipi accettati su { $total }.
uploads-open = Regola gli invii
error-upload-kind-refused = Questo tipo di file ({ $kind }) è rifiutato da questo sito: l'amministrazione non lo accetta. Non è stato salvato nulla.

# An import in place of the family, behind a word and a verified backup
# (docs/adr/0042).
setup-replace-title = Sostituire la famiglia
setup-replace-lede = Questo archivio contiene già una famiglia. Importare un file la sostituisce per intero.
setup-replace-warning = Tutto ciò che non è nel file importato andrà perso: schede, note, foto, legami.
setup-replace-backup = Subito prima, un backup verificato della famiglia attuale viene scritto in { $dir }; si ripristina con axgf-cms restore.
setup-replace-keyword = sostituire l'archivio
setup-replace-type = Per confermare, scrivete «{ $keyword }»
setup-replace-go = Sostituire la famiglia
setup-replace-keyword-wrong = Le parole di conferma non corrispondono: scrivete «{ $keyword }». Non è stato sostituito nulla.
setup-replace-backup-failed = Il backup della famiglia attuale non è riuscito, quindi non è stato sostituito nulla: { $error }
setup-replace-changed = La famiglia è cambiata durante il backup, quindi non è stato sostituito nulla. Riprovate.
setup-replace-done = La famiglia sostituita è nel backup, e il backup è verificato: { $file }
setup-replace-room-title = Sostituire la famiglia con un'importazione
setup-replace-room-detail = Un archivio o un GEDCOM al posto della famiglia attuale, dopo un suo backup verificato.
setup-replace-room-go = Aprire l'importazione
