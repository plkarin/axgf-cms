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

prefs-title = Lingua e aspetto
prefs-language = Lingua
prefs-language-note = Questo cambia solo l'interfaccia. Nomi, luoghi e note appaiono sempre nella loro lingua e scrittura.
prefs-theme = Aspetto
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
    }, { $depth } generazioni per parte. I più antichi in basso. L'opacità di una linea è la certezza del legame: una linea pallida è un'affermazione di cui la scheda non è sicura.
tree-lede-whole = Tutte le persone dell'albero. I più antichi in basso, i più recenti in alto. L'opacità di una linea è la certezza del legame.
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
    }, da { $path }. Il registro sta accanto all'archivio e non dentro: un archivio si copia, si spedisce e si pubblica, e chi ha corretto cosa è un fatto sulle persone che tengono l'albero, non sulla famiglia che vi si trova.
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
accounts-branch-hint = Limita ciò che questo account può modificare a quelle persone, ai loro discendenti e ai loro coniugi. Non limita ciò che può leggere: quello lo governa la visibilità di ogni scheda, e le due cose restano separate di proposito.
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
tree-width-notice = Questa vista è larga { $width } pixel. Ogni generazione è una riga, e la più larga fissa quella misura: su uno schermo da 1500 pixel fa { $screens ->
        [one] uno schermo
       *[other] { $screens } schermi
    } di scorrimento orizzontale. La vista attorno a una persona mostra invece qualche decina di persone intorno a lei, e ogni scheda la ricentra.
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
tree-click-hint = Fate clic su una scheda qualsiasi per aprire nella colonna laterale la scheda di quella persona; «Centra l'albero qui» nella colonna sposta la radice della vista.
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
home-what-title = Che cosa dà questo a una famiglia
home-what-archive-title = Un solo posto per tutto l'archivio
home-what-archive-body = L'albero, i documenti e le fotografie stanno insieme. La scansione di un atto di matrimonio pende dal matrimonio stesso, non dalla posta di qualcuno, e una fotografia nomina le persone che vi compaiono.
home-what-together-title = Più parenti, ruoli diversi
home-what-together-body = Una zia con trent'anni di appunti e un cugino che vuole soltanto correggere una grafia non hanno bisogno degli stessi poteri. Ogni parente è invitato con un ruolo proprio, e ogni modifica registra chi l'ha fatta e quando.
home-what-privacy-title = Riservatezza decisa persona per persona
home-what-privacy-body = Un parente vivo può essere visibile alla famiglia e invisibile ai visitatori, mentre la sua bisnonna è aperta a chiunque. La scelta si fa per ogni persona, non una volta sola per tutto l'albero.
home-what-languages-title = Undici lingue
home-what-languages-body = I parenti leggono il sito nella propria lingua — russo compreso, la lingua in cui furono tenuti i registri di stato civile di mezza Europa centrale e orientale. Un nome resta nella propria scrittura accanto alla traslitterazione; non serve appiattire nulla su un solo alfabeto perché il sito funzioni.
home-what-export-title = L'archivio resta vostro
home-what-export-body = Esportate tutto quanto in un solo file quando volete: persone, relazioni, documenti e fotografie insieme. Se un giorno decidete di andarvene, ve ne andate con l'archivio intero.
home-in-this-tree = Che cosa ha registrato finora la famiglia
home-showcase-title = Dove quest'albero dice già più di nomi e date
home-showcase-note = Ognuno di questi punti viene da ciò che qui è davvero registrato, non da un elenco di ciò che il sito saprebbe fare.
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

## Esiti delle operazioni

result-diagnostics = Segnalazioni
result-diagnostics-note = Ogni segnalazione restituita dalla libreria, comprese le avvertenze che non hanno bloccato l'operazione. Non se ne filtra nessuna.
result-no-diagnostics = La libreria non ha restituito segnalazioni.
result-continue = Avanti
result-dashboard = Quadro generale
person-sections-label = Sezioni di questa pagina

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
record-doc-photo = foto
record-doc-certificate = atto
record-doc-letter = lettera
record-doc-record = registrazione
record-doc-newspaper = giornale
record-doc-other = altro
record-upload = Carica
record-upload-help = Fino a { $mb } MB per file. Gli allegati stanno accanto all'albero e vengono riscritti nell'archivio all'esportazione, così una fotografia viaggia insieme alla famiglia a cui appartiene. Il tipo di file si legge dal suo contenuto e non dal nome: si accettano immagini, PDF, testo semplice, audio e video. L'SVG è rifiutato, perché un SVG può contenere uno script.
record-upload-help-short = Fino a { $mb } MB. L'SVG è rifiutato.
record-verbatim-note = Conservato esattamente come lo dava la scheda, perché nessun convertitore ha saputo interpretarlo. L'alternativa sarebbe stata scartarlo.
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
convert-lede = Portate qui un albero già esistente da un file GEDCOM, l'esportazione che produce la maggior parte dei programmi di genealogia. Ne riavete indietro un archivio da conservare. Qui non si conserva nulla, e l'albero che questo sito già mostra resta esattamente com'era.
convert-file-label = File di famiglia (.ged)
convert-file-hint = Fino a { $mb } MB. Un albero di 767 persone pesa circa 320 KB.
convert-confidence-label = Quanto sono certi questi fatti, per cominciare
convert-confidence-hint = Il file che state importando non dice quanto qualcuno fosse sicuro, perciò ogni fatto ha bisogno di un punto di partenza. Mettetelo basso per un albero messo insieme in fretta, più alto per uno lavorato sui documenti. La lettura onesta di questo numero è «importato, e da allora non controllato da nessuno»: potrete poi alzare o abbassare ogni fatto, uno alla volta.
convert-lang-label = Lingua dei nomi di luogo
convert-lang-hint = Una sigla come en, fr o it. Un luogo può portare il proprio nome in più lingue; questo dice in quale lingua sono scritti i nomi nel vostro file.
convert-what-you-get = Che cosa aggiunge l'importazione
convert-what-you-get-1 = Ogni fatto riceve un grado di certezza che potrete correggere in seguito, così un dubbio si scrive invece di buttarlo via. Le date conservano la loro forma: circa 1500, prima del 1430 e fra il 1920 e il 1925 restano tre affermazioni diverse, e una formulazione che nessuno ha saputo leggere come data resta parola per parola. Un mestiere diventa un tratto di tempo con un inizio e una fine. Ogni luogo diventa una voce a sé, così una città che ha cambiato stato conserva quella storia.
convert-no-way-back = Riscrivere un file .ged non è previsto. Quel formato non ha dove mettere quanto un fatto sia certo, una relazione fuori dalla famiglia, la durata di un mestiere o una data che nessuno ha saputo fissare: il viaggio di ritorno le perderebbe in silenzio. Il vostro archivio si esporta invece intero, come un solo file.

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
convert-skipped-note = Queste voci non contenevano nulla che si potesse portare qui. Sono elencate invece che inghiottite: sapere esattamente che cosa è rimasto indietro è la differenza fra un'importazione di cui fidarsi e una di cui non fidarsi.
convert-other-diagnostics = { $n ->
        [one] Un'altra cosa da sapere
       *[other] Altre { $n } cose da sapere
    }
convert-clean = Non è rimasto indietro nulla: ogni voce del file è passata.
convert-download-title = Scaricamento
convert-download-named = Scarica { $name }
convert-download-note = Conservato qui quindici minuti e poi scartato, quindi scaricatelo adesso. Quell'unico file è l'albero intero; tenetelo al sicuro.
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
completeness-intro = Che cosa è registrato e che cosa è ancora vuoto. Niente qui è un errore: una riga vuota è un punto in cui la scheda può crescere, non qualcosa che è andato storto.
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
