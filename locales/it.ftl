# axgf-cms — testi dell'interfaccia, italiano.
#
# QUALITÀ AUTOMATICA — non rivisto da una persona di madrelingua italiana. In
# particolare il lessico genealogico («union», «affiliation», «confidence») ha
# equivalenti consolidati che variano secondo la tradizione archivistica. Le
# correzioni sono benvenute — si veda CONTRIBUTING.md.
#
# REGOLA: questo file traduce solo l'interfaccia. Nomi, luoghi, note e
# mestieri provengono dal file .axgf e restano nella loro lingua e scrittura.

app-name = axgf-cms

nav-tree = Albero
nav-admin = Amministrazione
nav-sign-in = Accedi
nav-sign-out = Esci

prefs-title = Lingua e aspetto
prefs-language = Lingua
prefs-language-note = Questo cambia solo l'interfaccia. Nomi, luoghi e note sono sempre mostrati nella loro lingua e scrittura.
prefs-theme = Aspetto
prefs-apply = Applica
prefs-reviewed = rivisto
prefs-machine = automatico, { $coverage } %

theme-light = Chiaro
theme-dark = Scuro
theme-system = Segui il sistema
theme-high-contrast = Contrasto elevato
theme-sepia = Seppia
theme-deuteranopia = Deuteranopia
theme-protanopia = Protanopia
theme-tritanopia = Tritanopia
theme-colour-blind-note = adatto al daltonismo
theme-contrast-note = contrasto massimo

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
    }, { $depth } generazioni per lato. I più anziani in basso. L'opacità dei collegamenti indica la certezza della relazione: una linea tenue è un'affermazione di cui il documento non è sicuro.
tree-lede-whole = Tutte le persone del file. I più anziani in basso, i più giovani in alto. L'opacità dei collegamenti indica la certezza della relazione.
tree-filter-label = Filtra le schede visibili
tree-filter-placeholder = Digita un nome…
tree-centre-on = Centra su
tree-depth = Generazioni per lato
tree-show = Mostra
tree-hidden-notice = { $n ->
        [one] Una persona è mostrata senza i suoi dati
       *[other] { $n } persone sono mostrate senza i loro dati
    }
tree-hidden-because-role = , perché la loro visibilità supera ciò che il tuo account può leggere.
tree-hidden-because-anonymous = , perché non sono pubbliche.
tree-hidden-sign-in = Accedi se hai un account.
tree-restricted-card = La scheda di questa persona non ti è visibile
tree-empty = Questo file non contiene nessuno da disegnare.
tree-unplaced = In nessuna famiglia registrata

record-identity = Identità
record-life-events = Eventi della vita
record-family = Famiglia
record-other-relationships = Altre relazioni
record-occupations = Mestieri
record-places = Luoghi
record-sources-documents = Fonti e documenti
record-notes = Note
record-history = Cronologia
record-raw = Entità grezza
record-raw-summary-note = il JSON con cui è stata costruita questa pagina
record-sources-documents-help = Ogni fonte nomina i fatti di questa pagina che vi si appoggiano, ordinate secondo la solidità della prova.
record-notes-help = Note su questa scheda, compreso il testo che nessun convertitore ha saputo interpretare e che è stato conservato alla lettera anziché scartato.
record-help-toggle = Che cosa mostra questa sezione

record-gender = Genere
record-living = In vita
record-visibility = Visibilità
record-yes = sì
record-no = no
record-name-type = Tipo di nome
record-name-used = Usato
record-name-evidence = Prova
record-transliteration = Traslitterazione latina
record-born = Nato/a
record-died = Morto/a
record-parents = Genitori
record-siblings = Fratelli e sorelle
record-children = Figli
record-unknown-person = [Sconosciuto]
record-restricted-person = Riservato
record-restricted-title = La scheda di questa persona non ti è visibile
record-absent-person-title = Citato da questo file ma non presente in esso
record-confidence = Certezza
record-source = Fonte
record-download = Scarica

access-restricted-title = Non visibile per te
access-restricted-anonymous = Questa scheda non è pubblica. Accedi per vedere se il tuo account può leggerla.
access-role-title = Non per il tuo ruolo
access-role-write = Il tuo account può leggere questo file ma non modificarlo. Un amministratore può elevare il tuo ruolo a collaboratore.
access-scope-title = Fuori dal tuo ramo

error-not-found-title = Non trovato
error-not-found-detail = Questa pagina non esiste in questo file.
error-no-such-person-title = Nessuna persona simile
error-no-such-person-detail = Questo file non contiene alcuna persona con quell'identificativo.
error-no-such-entity-title = Nessuna entità simile
error-no-such-entity-detail = Questo file non contiene alcuna entità con quell'identificativo.
error-deleted-while-editing = Questo file non contiene alcuna entità con quell'identificativo. Potrebbe essere stata eliminata mentre la modificavi.
error-no-such-file-title = Nessun file simile
error-not-an-image-title = Non è un'immagine
error-not-an-image-detail = Non c'è anteprima per questo documento, perché non è un'immagine che questa versione sa decodificare.
error-back = Indietro

login-title = Accesso
login-lede = Gli account sono creati da un amministratore.
login-username = Nome utente
login-password = Password
login-submit = Accedi
login-wrong = Questo nome utente e questa password non corrispondono.
login-token-wrong = Questo token non è corretto.
login-throttled = Troppi tentativi falliti. Attendi qualche minuto e riprova.
login-no-accounts-title = Questa installazione non ha ancora alcun account.
login-emergency-summary = Accesso di emergenza
login-emergency-label = Token di emergenza
login-emergency-submit = Usa il token di emergenza
login-sign-in-prompt = Accedi per raggiungere il pannello di amministrazione.

admin-title = Amministrazione
admin-entities = Entità
admin-create = Crea
admin-new-kind = Nuovo: { $kind }
admin-operations = Operazioni
admin-validate = Convalida
admin-deduplicate = Unisci i duplicati
admin-export = Esporta il file
admin-accounts = Account
admin-dedup-confirm = L'unione dei duplicati fonde entità e riscrive il file. Continuare?
admin-recent-changes = Modifiche recenti
admin-sessions-open = { $n ->
        [one] Una sessione aperta ora.
       *[other] { $n } sessioni aperte ora.
    }
admin-no-changes-yet = Non è ancora stato modificato nulla tramite questa applicazione. Ogni salvataggio da ora in poi è registrato in { $path }.
admin-last-validation = Ultima convalida
admin-fields = Campi
admin-raw-json = JSON grezzo
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
admin-not-deleted = Non eliminato — il file è invariato
admin-what-changed = che cosa è cambiato
admin-field = Campo
admin-from = Da
admin-to = A
admin-version = versione { $version }

accounts-title = Account
accounts-existing = Esistenti
accounts-username = Nome utente
accounts-role = Ruolo
accounts-status = Stato
accounts-branch = Ramo
accounts-last-seen = Ultimo accesso
accounts-change = Modifica
accounts-you = (tu)
accounts-active = attivo
accounts-disabled = disattivato
accounts-never = mai
accounts-whole-tree = albero intero
accounts-roots = { $n ->
        [one] una radice
       *[other] { $n } radici
    }
accounts-add = Aggiungi un account
accounts-password-hint = Lascia vuoto e ne verrà generata una, mostrata una sola volta. Almeno { $min } caratteri se la imposti tu.
accounts-new-password-placeholder = nuova password (vuoto = invariata)
accounts-email = E-mail
accounts-optional = (facoltativo)
accounts-create = Crea l'account
accounts-role-viewer = lettore — legge le schede pubbliche e dei membri
accounts-role-contributor = collaboratore — inoltre crea, modifica e carica
accounts-role-admin = amministratore — inoltre gestisce gli account, elimina ed esporta
accounts-branch-placeholder = un identificativo di persona per riga
accounts-ids-in-bundle = Identificativi di persona in questo file
accounts-created = { $username } creato.
accounts-updated = { $username } aggiornato. Ogni sessione aperta è stata chiusa.
accounts-username-taken = Questo nome utente è già preso.
accounts-pick-role = Scegli un ruolo.
accounts-no-such = Nessun account simile.
accounts-not-saved = Non salvato: { $error }

conflict-title = Qualcun altro l'ha modificato prima di te
conflict-versions = Sei partito dalla versione { $expected }; il file contiene ora la versione { $current }.
conflict-both-changed = L'avete modificato entrambi
conflict-both-changed-detail = Questi campi sono stati modificati da entrambi. Ciò che salvi sostituirà quanto { $who } vi ha messo:
conflict-field-by-field = Campo per campo
conflict-theirs = In che cosa l'ha cambiato { $who }
conflict-yours = In che cosa l'hai cambiato tu
conflict-unchanged-by-you = non modificato da te
conflict-unchanged-by-them = non modificato da loro
conflict-what-now = E adesso
conflict-reapply = Riapplica la tua versione sulla loro
conflict-save-over = Salva questa al posto della loro
conflict-discard = Scarta la mia e ricomincia
conflict-their-version = La versione di { $who }, così come il file la contiene ora
conflict-history-of = Cronologia di questa entità ({ $kind })

## Dates

date-unknown = Data sconosciuta
date-not-recorded = Non registrata
date-circa = circa { $date }
date-between = tra { $from } e { $to }
date-before = prima del { $date }
date-after = dopo il { $date }
date-preserved = registrato come «{ $text }»
date-day-month-year = { $day } { $month } { $year }
date-month-year = { $month } { $year }
date-decade = gli anni { $decade }
date-century = il { $century }° secolo

month-1 = gennaio
month-2 = febbraio
month-3 = marzo
month-4 = aprile
month-5 = maggio
month-6 = giugno
month-7 = luglio
month-8 = agosto
month-9 = settembre
month-10 = ottobre
month-11 = novembre
month-12 = dicembre
