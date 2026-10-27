# axgf-cms — makomi ya interface, Lingala.
#
# Bobongoli ya masini — moto oyo lokota na ye ya mbotama ezali Lingala atali
# yango naino te. Maloba ya lisolo ya libota mingimingi ekoki kozala mabe awa.
# Bobongisi endimami na esengo — tala CONTRIBUTING.md.
#
# Maloba eponami (ekoki kosololama):
#   union → boyokani · link → boyokani · confidence → bondimi
#   source → liziba · record → mokanda · archive → ebombelo
#   ancestors → bankoko · descendants → bakitani · generation → nkola
#   administrator → mokambi · contributor → mosungi · reader → motangi
#
# Makomi: e mpe o na esika ya ɛ mpe ɔ, mpe bilembo ya tonalité te, ndenge
# bato mingi bakomaka na Kinshasa mpe na Brazzaville. Maloba ya informatique
# mingi etikali na Lifalanse (fisyé, compte, mot de passe).
# Bankombo ya mikili mpe ya minoko euti na CLDR; minoko oyo CLDR ezali na
# yango te na Lingala ekomami na Lifalanse.
#
# Lingala ezali na lolenge mibale ya motango na CLDR: one mpe other.
#
# MOBEKO: fisyé oyo ebongolaka kaka interface. Bankombo, bisika, makomi mpe
# misala euti na fisyé .axgf mpe etikalaka na lokota mpe makomi na yango.

app-name = ax-genealogy

nav-tree = Nzete
nav-convert = Kokotisa
nav-admin = Bokambi
nav-sign-in = Kota
nav-sign-out = Bima
account-emergency = Elembo ya likama
footer-credit = © 2026 · Ebongisami mpe etongami na { $author }
footer-format-link = Lolenge ya AXGF na GitHub

settings-title = Bobongisi
edit-mode-label = Kobongola — oyo okobomba ekobongola mikanda ya libota
display-menu-label = Lokota mpe komonana
display-settings = Lolenge mpe komonana…
settings-tab-theme = Lolenge
settings-tab-language = Lokota
settings-tab-appearance = Komonana
settings-done = Esili
prefs-language = Lokota
prefs-theme = Lolenge
prefs-background-on = Langi ya pepele na nsima ya lokasa
prefs-apply = Salela
busy-saving = Ezali kobomba…
busy-working = Ezali kosala…
prefs-machine = masini, { $coverage }%
prefs-machine-complete = mobimba, natango etalami te

theme-canopee = Canopy
theme-canopee-note = molili, langi ya zamba · oyo ya ebandeli
theme-fusion = Fusion
theme-fusion-note = molili, neon · nzete ya bomoi
theme-galaxy = Galaxy
theme-galaxy-note = likolo ya mosika · lisanga ya moi
theme-summit = Summit
theme-summit-note = molili, ntongo ya wolo · komata

tree-title-around = Zingazinga ya { $name }
tree-title-whole = Nzete mobimba
tree-lede-focused = { $ancestors ->
        [one] Nkoko moko
       *[other] Bankoko { $ancestors }
    }, { $descendants ->
        [one] mokitani moko
       *[other] bakitani { $descendants }
    } mpe { $spouses ->
        [one] molongani moko
       *[other] balongani { $spouses }
    }, nkola { $depth } na ngambo moko na moko.
tree-filter-label = Pona bakarte oyo ezali komonana
tree-depth = Nkola na ngambo moko na moko
tree-show = Lakisa
tree-hidden-notice = { $n ->
        [one] Moto moko alakisami kozanga makambo na ye
       *[other] Bato { $n } balakisami kozanga makambo na bango
    }
tree-hidden-because-role = , mpo komonana na bango eleki oyo konti na yo ekoki kotanga.
tree-hidden-because-anonymous = , mpo bazali ya bato nyonso te.
tree-hidden-sign-in = Kota soki ozali na konti.
tree-restricted-card = Mokanda ya moto oyo ezali komonana epai na yo te
tree-empty = Natango moto ya kobenda azali te.

record-identity = Bomoto
record-life-events = Makambo ya bomoi
record-family = Libota
record-other-relationships = Boyokani mosusu
record-occupations = Misala
record-places = Bisika
record-sources-documents = Maziba mpe mikanda
record-notes = Makomi
record-history = Lisolo

record-identity-help = Nkombo nyonso ekomami elongo na lolenge na yango, ntango esalelamaki mpe liziba na yango, makomi ya ebandeli mpe kobongola na yango na makomi ya Latin pembeni na pembeni soki ekeseni, elongo na mobali to mwasi, soki azali na bomoi mpe komonana.
record-life-events-help = Kobotama, kokufa mpe likambo nyonso oyo moto oyo asangani na yango, na molongo ya mikolo, moko na moko na mosala na ye — boye libala oyo atalaki kaka emonani pembeni ya libala na ye moko. Likambo ezanga mokolo etiamaka na suka na esika ya komimonisa lokola ya liboso.
record-family-help = Baboti mpe bandeko, sima boyokani moko na moko elongo na lolenge na yango, mikolo, esika, ndenge esilaki mpe bana na yango na molongo ya kobotama.
record-other-relationships-help = Boyokani nyonso oyo moto oyo azali na suka moko na yango, etangami na ngambo na ye, boye mokanda moko emonanaka lokola "tata ya batisimo ya" na ngambo moko mpe "mwana ya batisimo ya" na ngambo mosusu.
record-places-help = Esika nyonso oyo mokanda oyo esimbi, elongo na oyo esalemaki na esika moko na moko mpe lisolo ya ndelo oyo epesaka esika ntina na boumeli ya ntango.
record-sources-documents-help = Liziba moko na moko elobi makambo ya lokasa oyo oyo etelemi likolo na yango, na molongo ya makasi ya litatoli.
record-notes-help = Makomi mpo na mokanda oyo, elongo na makomi oyo mobongoli akokaki kotanga te mpe abombaki ndenge ezalaki na esika ya kobwaka yango.
record-history-help = Mbongwana nyonso oyo ebombami na mokanda oyo, oyo ya sika liboso. Nani abongisaki nini ezali likambo ya bato oyo babatelaka nzete, kasi ya libota oyo ezali kati na yango te, yango wana ezali te na ebombelo oyo ekotisami libanda mpe emonanaka kaka na bandeko oyo bakoti.
record-help-toggle = Eteni oyo emonisaka nini

record-gender = Mobali to mwasi
record-living = Na bomoi
record-visibility = Komonana
record-yes = iyo
record-no = te
record-name-type = Lolenge ya nkombo
record-name-used = Esalelamaki
record-name-evidence = Litatoli
record-transliteration = Kobongola na makomi ya Latin
record-born = Abotamaki
record-died = Akufaki
record-parents = Baboti
record-siblings = Bandeko
record-children = Bana
record-unknown-person = [Eyebani te]
record-restricted-person = Ya sekele
record-restricted-title = Mokanda ya moto oyo ezali komonana epai na yo te
record-absent-person-title = Atangami na nzete oyo kasi akomami kati na yango te
record-confidence = Bondimi
conf-percent = { $percent }%
record-source = Liziba
record-download = Kokitisa

access-restricted-title = Ezali komonana epai na yo te
access-restricted-signed-in = Komonana ya mokanda oyo eleki oyo konti na yo ekoki kotanga. Mokambi akoki kobongola komonana ya mokanda to mosala na yo.
access-restricted-anonymous = Mokanda oyo ezali ya bato nyonso te. Kota mpo na koyeba soki konti na yo ekoki kotanga yango.
access-role-title = Ezali mpo na mosala na yo te
access-role-admin = Oyo ezali lokasa ya mokambi. Konti na yo ekoki kosala mpe kobongola mikanda, kasi ekoki te kokamba bakonti, kolongola mikanda to kobimisa ebombelo.
access-role-write = Konti na yo ekoki kotanga nzete oyo kasi ekoki kobongola yango te. Mokambi akoki kotombola yo mosungi.
access-scope-title = Libanda ya etape na yo
access-scope-named = Konti na yo ekangami na etape moko ya nzete, mpe mokanda oyo etali moto moko oyo azali libanda na yango. Moto nyonso oyo mokanda etangi asengeli kozala kati na etape na yo — soki te libota oyo molongani moko auti libanda ekokaki kozala nzela ya kokoma lisusu baboti ya moto yango.
access-scope-unnamed = Konti na yo ekangami na etape moko ya nzete, mpe mokanda oyo etangi moto moko te oyo bakoki komeka yango na ye. Maziba mpe bisika ebongolamaka na bakonti oyo bazali na nzela ya nzete mobimba.

error-not-found-title = Emonani te
error-not-found-detail = Lokasa yango ezali awa te.
error-no-such-person-title = Moto ya ndenge wana azali te
error-no-such-person-detail = Moto azali awa te na elembo yango.
error-merged-title = Mokanda oyo ezali lisusu te.
error-merged-detail = Esangisamaki na { $name } na mokolo { $date }. Makangisi ya adresi ya kala ememaka sikoyo na mokanda oyo esangisami.
error-merged-open = Fungola mokanda oyo esangisami
error-back-to-river = Zonga na ebale
view-tree-back = Zonga na nzete
view-galaxy-back = Zonga na galaksi
view-summit-back = Zonga na ngomba
error-no-such-entity-title = Mokanda ya ndenge wana ezali te
error-no-such-entity-detail = Mokanda ezali awa te na elembo yango.
error-deleted-while-editing = Mokanda ezali awa te na elembo yango. Ekoki kozala ete elongolamaki ntango ozalaki kobongola yango.
error-no-such-file-title = Fisyé ya ndenge wana ezali te
error-no-such-file-detail = Mokanda ezali awa te na elembo yango, to mokanda ekomami kozanga fisyé — mokanda oyo etangami elakisi eloko oyo ebombami esika mosusu.
error-not-an-image-title = Ezali elilingi te
error-not-an-image-detail = Elilingi moke ezali te mpo na mokanda oyo, mpo ezali elilingi te oyo version oyo ekoki kotanga.
error-back = Zonga

login-title = Kota
login-lede = Bakonti esalemaka na mokambi.
login-username = Nkombo ya mosaleli
login-password = Liloba ya sekele
login-submit = Kota
login-wrong = Nkombo ya mosaleli mpe liloba ya sekele eyokani te.
login-token-wrong = Elembo yango ezali malamu te.
login-throttled = Komeka mingi elongi te. Zela miniti mwa moke mpe meka lisusu.
login-no-accounts-title = Bosimbi oyo ezali naino na konti te.
login-no-accounts-detail = Na mokano lokasa ya kobongisa ezali awa te — ntango kati na kotia site mpe kokota ya liboso ezali mpenza ntango oyo bosimbi ezali na libateli te, yango wana mokambi ya liboso asalemaka na molongo ya mitindo.
login-no-accounts-note = Ebimisaka liloba ya sekele oyo esalemi na stderr mbala moko mpe lisusu te. Kino wana nzela kaka ya kokota ezali elembo ya likama oyo ezali awa na nse.
login-emergency-summary = Nzela ya likama
login-emergency-detail = Elembo oyo bato nyonso basangani efungolaka naino ntango ya mokambi, mpe ezali mpo na ntina moko: kozonga kati soki fisyé .acl ebungi to bakambi nyonso bakangami libanda. Ezali konti te — ezali na baposa te, mpe buku ya mbongwana ekomaka yango lokola emergency-token kasi lokola moto te. Bosaleli na yango ekomamaka lokola likebisi.
login-emergency-label = Elembo ya likama
login-emergency-submit = Salela elembo ya likama
login-sign-in-prompt = Kota mpo na kokoma na etando ya bokambi.

admin-title = Bokambi
setup-title = Bongisa ebombelo oyo
setup-lede = Bosimbi oyo ezali naino na libota te. Pona ndenge ekobanda. Eloko moko te awa ekomamaka likolo ya libota: lokasa oyo ekangamaka soki libota ezali.
setup-door-archive-title = Uta na ebombelo ya AXGF
setup-door-archive-detail = Fisyé .axgf oyo euti na bosimbi mosusu to na kopi ya libateli. Bato, maziba mpe bafoto na yango eyaka ndenge bazali.
setup-door-archive-file = Ebombelo (.axgf)
setup-door-archive-go = Bongisa uta na ebombelo oyo
setup-limit-gb = Kino GB { $n }, ekomami mbala moko na disiki.
setup-door-scratch-title = Uta na eloko te
setup-door-scratch-detail = Ebombelo ya mpamba. Bakisa moto ya liboso, sima libota na ye, mokanda moko sima ya mosusu.
setup-door-scratch-go = Bakisa moto ya liboso
setup-door-gedcom-title = Uta na fisyé GEDCOM
setup-door-gedcom-detail = Fisyé oyo programe nyonso ya bankoko ebimisaka: GEDCOM 5.5, 5.5.1, 5.5.5 to 7.0, lokola .ged to .gdz, na lolenge nyonso ya kokoma. Okomona ndenge etangami mpe oyo ekoleka liboso eloko moko ebombama.
setup-door-gedcom-file = Fisyé GEDCOM (.ged, .gdz)
setup-door-gedcom-go = Tanga fisyé oyo
setup-never-over = Mpo na kobongola libota oyo ezali awa, zongisa kopi ya libateli na molongo ya mitindo (axgf-cms restore); lokasa moko te ekomaka likolo na yango.
setup-adopt-title = Kómisa oyo libota ya ebombelo
setup-adopt-detail = Mikanda oyo ekokoma libota ya bosimbi oyo: { $total }. Eloko moko te ebombami kino okofina buto.
setup-adopt-go = Bongisa ebombelo na libota oyo
setup-adopt-back = Pona nzela mosusu ya kokota
setup-done-title = Ebombelo ebongisami
setup-done-detail = Mikanda na libota ya bosimbi oyo sikoyo: { $total }.
setup-done-photos = Bafoto mpe mikanda oyo eyaki elongo na yango: { $n }.
setup-done-mismatches = Bafisyé oyo babaiti na yango eyokani te na elembo oyo ekomamaki mpo na yango: { $n }. Ebombami; tala yango.
setup-done-river = Fungola ebale
view-tree-open = Fungola nzete
view-galaxy-open = Fungola galaksi
view-summit-open = Fungola ngomba
setup-done-admin = Bokambi
setup-refused-title = Oyo ekokaki kokoma ebombelo te
setup-refused-detail = Eloko moko te ekomami. Oyo epekisaki yango:
setup-closed-title = Ebombelo oyo ebongisami kala
setup-closed-detail = Ezali na libota, mpe lokasa moko te ekomaka likolo na yango. Mpo na kobongola yango, zongisa kopi ya libateli na molongo ya mitindo (axgf-cms restore).
setup-not-archive = Oyo ezali ebombelo .axgf te.
setup-archive-too-large = Ebombelo oyo eleki monene oyo bakoki kotinda.
admin-lede = Kobongola { $path } — mikanda { $total }, { $files ->
        [one] fisyé moko ekangisami
       *[other] bafisyé { $files } ekangisami
    }, { $size } na disiki. Mbongwana nyonso ekomamaka mobimba; mbongwana oyo eboyami etikaka fisyé ndenge ezalaki.
admin-entities = Mikanda
admin-create = Kosala
admin-new-kind = { $kind } ya sika
admin-operations = Misala
admin-validate = Kondimisa
admin-deduplicate = Kolongola mibale
admin-export = Kobimisa ebombelo
admin-roles-note = Kondimisa, kolongola mibale, kobimisa, kolongola mpe kokamba bakonti ezali kaka mpo na bakambi. Mosungi akomaka na lokasa nyonso mosusu awa.
admin-dedup-confirm = Kolongola mibale esangisaka mikanda mpe ekomaka lisusu ebombelo. Kokoba?
admin-recent-changes = Mbongwana ya sika
admin-recent-note = { $shown } ya suka kati na { $total ->
        [one] mbongwana moko ekomami
       *[other] mbongwana { $total } ekomami
    }, uta na { $path }.
admin-sessions-open = { $n ->
        [one] Ntango moko ya kokota efungwami sikoyo.
       *[other] Ntango { $n } ya kokota efungwami sikoyo.
    }
admin-no-changes-yet = Natango eloko moko te ebongolami na nzela ya programe oyo. Kobomba nyonso uta sikoyo ekomamaka na { $path }.
admin-last-validation = Kondimisa ya suka
admin-bundle-heavy = Ebombelo oyo ezali { $size }. Mobimba na yango ekotaka na ebandeli mpe ebombamaka na mayele ya masini, yango wana soki eleki pene na { $warn } site ebandaka kosalela mayele mingi mpe kobanda lisusu ezwaka ntango. Yango ebongi mpo na ebombelo ya libota, kasi mpo na ebombelo ya bafoto mingi te — soki biloko ekangisami ezali kokola kozanga suka, bomba yango na ebombelo ya bafisyé mpe ebombelo elakisa yango.

admin-fields = Bisika
admin-raw-json = JSON ya ebandeli
form-more-fields = { $n ->
        [one] Esika moko lisusu
       *[other] Bisika { $n } lisusu
    }
admin-raw-json-help = Mokanda mobimba, boye eloko moko te ezangi kobongolama — milongo lokola balongani mpe bana ya libota, to lisolo ya ndelo ya esika, ezali awa. Oyo ezali mokanda ya ebandeli; bisika oyo ezali likolo ekomamaka sima likolo ya banzela oyo ezali na yango, yango wana bongola motuya esika moko to esika mosusu, kasi na bisika mibale te. Esengeli kozala JSON ya malamu soki te eloko moko te ebombami.
admin-save = Bomba
admin-cancel = Tika
place-editor-title = Bongola esika
place-names = Bankombo
place-name-primary = Ya liboso
place-name-lang = Lokota
place-name-value = Nkombo
place-names-hint = Molongo moko mpo na nkombo moko na moko ekomami. Esika oyo bampeló misato bakambaki ezali na bankombo misato; ya liboso nde emonanaka bisika nyonso mosusu.
place-where = Esika ezali
place-type = Lolenge
place-region = Etuka
place-country-current = Mboka lelo
place-country-hint = ISO 3166-1 alpha-2, ndakisa PL, FR, DE.
place-country-history = Lisolo ya ndelo
place-history-country = Leta
place-history-from = Uta
place-history-until = Kino
place-country-history-hint = Leta nini esimbaki esika oyo na ntango nini. Ezali na ntina mpo na bankoko: mokanda oyo ekomamaki na Lirusi na 1880 mpe mosusu na Lipolone na 1930 ekoki kotanga mboka moko.
place-coordinates = Bisika na mapa
place-lat = Latitude
place-lon = Longitude
place-precision = Bosikisiki
place-identifiers = Bilembo
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } esalelami na { $n ->
        [one] mokanda mosusu moko
       *[other] mikanda mosusu { $n }
    }.
place-error-no-name = Esika esengeli kozala na nkombo moko na moke.
place-error-coords-pair = Latitude mpe longitude ekendaka elongo: pesa nyonso mibale, to moko te.
place-error-coords-number = Latitude mpe longitude esengeli kozala mituya.
place-error-coords-range = Latitude ezali uta -90 kino 90 mpe longitude uta -180 kino 180.
place-type-continent = mokili monene
place-type-country = mboka
place-type-region = etuka
place-type-department = etuka ya moke
place-type-city = engumba

place-type-village = mboka
place-type-district = quartier
place-type-street = balabala
place-type-building = ndako
place-type-farm = elanga
place-type-island = esanga
place-type-historical = ya kala
place-type-unknown = eyebani te
place-precision-exact = sikisiki
place-precision-building = ndako
place-precision-street = balabala
place-precision-city_center = katikati ya engumba
place-precision-region_center = katikati ya etuka
place-precision-country_center = katikati ya mboka
place-precision-approximate = pene na pene

place-coordinates-hint = Kokoma na loboko ezali nzela ya momesano. Bisika mingi oyo ekomamaki na ntango ya bokambi ya kala emonanaka ata moke te na boluki ya lelo.
place-geocode-search = Luka nkombo oyo
place-geocode-hint = Etindaka nkombo, etuka mpe mboka na mosala ya koluka bisika, esika moko na mbala moko. Eloko moko te ebombami kino okobomba.
place-geocode-off = Boluki ya bankombo ekangami. Esengeli adresi ya kosolola oyo mosala yango ekoki koyeba bosimbi oyo na yango; banda serveur na --geocoder-contact mpo na kofungola yango.
place-geocode-query = Oyo elukamaki: { $q }
place-geocode-error = Mosala ya boluki ekokaki kokoma te. Bisika ya mapa oyo ezali likolo ezali kaka kosala.
place-geocode-none = Eloko moko te emonani. Mpo na mboka oyo ekomamaki na ntango ya bokambi ya Rusi, Prusi to Otrish oyo ezali eloko ya momesano; koma esika na loboko.
place-geocode-not-a-place = ezali esika ya kofanda te
place-geocode-use = Salela oyo
place-geocode-attribution = Biloko oyo euti na OpenStreetMap na nzela ya Nominatim, na nse ya Open Database Licence.

place-paste = Kangisa esika
place-paste-placeholder = lien ya mapa, to 52.0782795, 21.2508068
place-paste-read = Tanga yango
place-paste-hint = Lien ya Google Maps to OpenStreetMap, URI ya geo:, mituya mibale ya pete, to badegre-baminiti-basekondi lokola 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Etangami na bisika oyo ezali likolo. Tala yango, sima bomba.
place-paste-unreadable = Oyo ezali esika te oyo ekoki kotangama. Bisika oyo ezali likolo ezali kaka kozwa mituya mibale ya pete.

place-map-hint = Fina mapa mpo na kotia litono, to benda epeŋgi. Bisika oyo ezali likolo nde mokanda.
place-map-clear = Longola litono
place-open-in-map = Luka esika oyo na OpenStreetMap, sima kangisa lien awa

person-tab-record = Mokanda
person-tab-life = Makambo
person-tab-media = Bafoto mpe mikanda
person-tab-tree = Nzete
person-tab-history = Lisolo

admin-delete = Longola
admin-not-set = — etiami te —
admin-edit = Bongola
list-select-row = Pona { $name }
list-kind-label = Lolenge
list-select-page = Pona milongo nyonso ya lokasa oyo
list-selected = { $n ->
        [one] { $n } eponami
       *[other] { $n } eponami
    }
list-export = Bimisa
list-escape = Esc mpo na kolongola bopono
bulk-confirm-title = { $n ->
        [one] Kolongola mokanda { $n }?
       *[other] Kolongola mikanda { $n }?
    }
bulk-confirm-detail = Mokanda oyo mosusu ezali naino kotanga eboyamaka, mpe na ntango wana eloko moko te elongolamaka.
bulk-confirm-go = Longola yango
bulk-deleted = { $n ->
        [one] Mokanda { $n } elongolami.
       *[other] Mikanda { $n } elongolami.
    }
bulk-refused = Eloko moko te elongolami: { $name } ezali naino kotangama esika mosusu. Longola boyokani yango liboso, sima meka lisusu.
bulk-changed = { $n ->
        [one] Mokanda { $n } ebongwanaki uta molongo emonisamaki, yango wana eloko moko te elongolami. Zongisa molongo mpe meka lisusu.
       *[other] Mikanda { $n } ebongwanaki uta molongo emonisamaki, yango wana eloko moko te elongolami. Zongisa molongo mpe meka lisusu.
    }
admin-range = { $from }–{ $to } kati na { $total }
list-filiation = Libota ya baboti
admin-previous = Oyo eleki
admin-next = Oyo elandi
admin-saved = Ebombami lokola version { $version } — { $summary }
admin-not-saved = Ebombami te
admin-created = Esalemi
admin-not-created = Esalemi te
admin-deleted = Elongolami
admin-not-deleted = Elongolami te — eloko moko te ebongwani
admin-what-changed = oyo ebongwani
admin-field = Esika
admin-from = Uta
admin-to = Kino
admin-version = version { $version }

accounts-title = Bakonti
accounts-lede = Ebombami na { $path }, na ndingisa 600, pembeni ya ebombelo mpe soko moke te kati na yango. Ebombelo ezwamaka kopi, etindamaka mpe ebimisamaka; soki bilembo ya maloba ya sekele ezali kotambola kati na yango, kopi nyonso ya nzete ya libota ekokoma kopi ya makambo ya kokota.
accounts-existing = Oyo ezali
accounts-username = Nkombo ya mosaleli
accounts-role = Mosala
accounts-status = Lolenge
accounts-branch = Etape
accounts-last-seen = Emonanaki mbala ya suka
accounts-change = Bongola
accounts-you = (yo)
accounts-active = ezali kosala
accounts-disabled = ekangami
accounts-never = soko moke te
accounts-whole-tree = nzete mobimba
accounts-roots = { $n ->
        [one] mosisa moko
       *[other] misisa { $n }
    }
accounts-add = Bakisa konti
accounts-no-registration = Bapaya bakoki kofungola bakonti bango moko te; komikomisa ebongisami likolo. Mpo na mabota mingi mokambi oyo ayebi bato nyonso akoki.
accounts-password-hint = Tika mpamba mpe moko ekosalema mpe ekomonisama mbala moko. Ata bilembo { $min } soki otie yango yo moko.
accounts-new-password-placeholder = liloba ya sekele ya sika (mpamba = tika ndenge ezali)
accounts-email = Imeli
accounts-optional = (ezali na makasi te)
accounts-create = Sala konti
accounts-role-viewer = motangi — atangaka mikanda ya bato nyonso mpe ya bato ya libota
accounts-role-contributor = mosungi — asalaka lisusu, abongolaka mpe atindaka bafisyé
accounts-role-admin = mokambi — akambaka lisusu bakonti, alongolaka mpe abimisaka
accounts-branch-hint = Ekangaka oyo konti oyo ekoki kobongola kaka na bato wana, bakitani na bango mpe balongani na bango.
accounts-branch-reading = Ekangaka te oyo bakoki kotanga — yango ekatamaka na komonana ya mokanda moko na moko, mpe makambo mibale wana ekabolami na mokano.
accounts-branch-placeholder = elembo moko ya moto na molongo moko na moko
accounts-ids-in-bundle = Bilembo ya bato na nzete oyo
accounts-emergency-warning = Okoti na elembo ya likama. Epesaka makoki ya mokambi mpo na ntango oyo kasi ezali konti te: ezali na baposa te, mpe buku ya mbongwana ekokoma mbongwana na yo lokola emergency-token kasi lokola moto te. Misalela konti ya solo awa na nse mpe kota na yango.
accounts-created-with-password = { $username } asalemi. Liloba na ye ya sekele ezali { $password } — emonisami mbala moko mpe ebombami kaka lokola elembo ya Argon2id, yango wana pesa ye yango sikoyo.
accounts-created = { $username } asalemi.
accounts-updated = { $username } abongolami. Ntango nyonso ya kokota oyo ezalaki ya kofungwama ekangami.
accounts-username-taken = Nkombo wana ya mosaleli ezwami kala.
accounts-pick-role = Pona mosala.
accounts-no-such = Konti ya ndenge wana ezali te.
accounts-last-admin = Oyo nde mokambi kaka oyo azali kosala. Tombola moto mosusu liboso — bosimbi oyo ezangi mokambi ekoki kozongisama kaka na kobongola fisyé .acl to na kosalela elembo ya likama.
accounts-not-saved = Ebombami te: { $error }

conflict-title = Moto mosusu abongolaki oyo liboso
conflict-lede = { $who } abombaki mbongwana na { $kind } oyo na { $when }, sima ya yo kofungola yango. Mbongwana na yo ebombami te, mpe eloko moko te ekomami likolo.
conflict-no-merge = Eloko moko te esangisamaka yango moko awa. Kosangisa mbongwana ya bato mibale ebimisaka mokanda oyo moko te kati na bango aponaki, mpe na makambo ya bankoko babongoli mibale oyo bakeseni na mokolo mbala mingi bazali kotanga maziba ekeseni — wana ezali motuna mpo na moto, kasi mpo na programe te. Kokanisa nyonso mibale oyo ezali awa na nse mpe kata likambo.
conflict-versions = Obandaki uta na version { $expected }; mokanda ezali sikoyo na version { $current }.
conflict-both-changed = Bino mibale bobongolaki oyo
conflict-both-changed-detail = Bisika oyo ebongolamaki na bino mibale. Oyo okobomba ekozwa esika ya oyo { $who } atiaki wana:
conflict-different-fields = Bobongolaki bisika ekeseni, yango wana mosala moko te ya { $who } ezali na ntembe — kasi kosalela lisusu ekomaka kaka mokanda na yo mobimba likolo ya oyo ya ye. Tala milongo nyonso mibale liboso ya kobomba.
conflict-field-by-field = Esika na esika
conflict-theirs = Oyo { $who } abongolaki yango
conflict-yours = Oyo yo obongolaki yango
conflict-unchanged-by-you = ebongolami na yo te
conflict-unchanged-by-them = ebongolami na bango te
conflict-nothing-differs = Version moko te ekeseni na oyo obandaki na yango na esika moko oyo lokasa oyo emonisaka. Motuya ya version ebongwanaki, yango wana moto abombaki mokanda kozanga kobongola eloko oyo ekomi.
conflict-what-now = Sikoyo nini
conflict-reapply = Salela lisusu version na yo likolo ya oyo ya bango
conflict-reapply-hint = Oyo ezali mbongwana na yo, ememami liboso na version { $version }. Bongola yango awa mpo na kobatela mosala nyonso ya { $who } oyo olingi, sima bomba. Version na bango emonisami awa na nse mpo na kozwa kopi.
conflict-save-over = Bomba oyo likolo ya oyo ya bango
conflict-discard = Bwaka oyo ya ngai mpe banda lisusu
conflict-their-version = Version ya { $who }, ndenge ezali sikoyo
conflict-history-of = Lisolo ya { $kind } oyo

convert-title = Kotisa fisyé ya libota
convert-submit = Kotisa
convert-result-title = Lapolo ya kokotisa

date-unknown = Mokolo eyebani te
date-not-recorded = Ekomami te
date-circa = pene na { $date }
date-between = kati na { $from } mpe { $to }
date-before = liboso ya { $date }
date-after = sima ya { $date }
date-preserved = ekomami lokola “{ $text }”
date-day-month-year = { $day } { $month ->
        [1] sanza ya yambo
        [2] sanza ya mibale
        [3] sanza ya misato
        [4] sanza ya minei
        [5] sanza ya mitano
        [6] sanza ya motoba
        [7] sanza ya nsambo
        [8] sanza ya mwambe
        [9] sanza ya libwa
        [10] sanza ya zomi
        [11] sanza ya zomi na moko
        [12] sanza ya zomi na mibale
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] sanza ya yambo
        [2] sanza ya mibale
        [3] sanza ya misato
        [4] sanza ya minei
        [5] sanza ya mitano
        [6] sanza ya motoba
        [7] sanza ya nsambo
        [8] sanza ya mwambe
        [9] sanza ya libwa
        [10] sanza ya zomi
        [11] sanza ya zomi na moko
        [12] sanza ya zomi na mibale
        *[other] { $month }
    } { $year }
date-decade = mibu ya { $decade }
date-century = ekeke ya { $century }
date-quarter-century = eteni ya { $quarter ->
        [1] liboso
        [2] mibale
        [3] misato
       *[other] minei
    } ya ekeke ya { $century }

error-back-to-start = Zonga na ebandeli
error-payload-missing-title = Fisyé ya ndenge wana ezali te
error-payload-missing-detail = Makambo ya mokanda wana ezali na ebombelo ya ntango moke te.
error-payload-unopenable-detail = Makambo ya mokanda wana ekokaki kofungwama te.
error-no-such-document-detail = Mokanda ezali awa te na elembo yango.
error-bad-preference-title = Ezali moko te kati na maponi
error-bad-preference-detail = Oyo ezali lokota to lolenge te oyo site oyo epesaka. Eloko moko te ebongwani.
error-unknown-kind-title = Lolenge eyebani te
error-unknown-kind-detail = “{ $kind }” ezali lolenge ya mokanda te. Ebombelo oyo ezali na: { $kinds }.
error-io-title = Ekokaki kobomba te
error-io-detail = { $error }. Eloko moko te ebongwani na disiki.
error-upload-too-large = Fisyé wana eleki ndelo ya MB { $mb }. Eloko moko te ebombami, mpe ebombelo ebongwani te.
error-upload-refused = Mokanda eboyamaki: { $reason }. Ebombelo ebongwani te.
error-back-to-person = Zonga na mokanda
error-no-such-person-to-attach = Moto azali awa te na elembo yango, yango wana eloko te ya kokangisa mokanda na yango.
error-upload-title = Botindi wana ebombami te
error-download-expired-title = Bokitisi wana eleki ntango
error-download-expired-detail = Bokotisi ebatelamaka miniti zomi na mitano, sima ebwakamaka. Kotisa fisyé lisusu.
error-upload-none = Fisyé moko te etindami. Pona fisyé liboso.
error-upload-unsupported = Fisyé wana ezali lolenge te oyo ebombelo oyo ebombaka. Bilili, PDF, makomi ya pete, mongongo mpe video endimami; lolenge etangamaka na babaiti ya fisyé yango moko, yango wana kobongola nkombo ya programe ekotisaka yango te. SVG eboyamaka nyonso, mpo SVG ekoki komema script.
error-export-unreadable-title = Ekokaki kotanga ebombelo oyo ebimisami te
error-export-unreadable-detail = { $error }

tree-title-suffix = nzete
tree-show-all = Lakisa nyonso { $n }
tree-width-notice = Komonana oyo ezali na bonene ya pikseli { $width } — na ekrani ya pikseli 1500, ezali { $screens ->
        [one] ekrani moko
       *[other] bekrani { $screens }
    } ya kotambola na ngambo.
tree-confidence-label = Bondimi:
tree-band-certain = sikisiki
tree-band-high = likolo
tree-band-medium = katikati
tree-band-low = likanisi
tree-counts = Bato { $drawn } kati na { $total } · { $generations ->
        [one] nkola moko
       *[other] nkola { $generations }
    }
tree-unplaced-count = { $n } bazangi esika
tree-contradicts-title = Nzete oyo ezali komiwangana.
tree-contradicts-detail = Ndenge moko te ya kotia milongo ekoki kokokisa yango, yango wana boyokani oyo ezali awa na nse etikamaki libanda ya kotanga nkola mpe milongo mosusu ekoki kozala ebendami esika mabe. Bongisa mokanda oyo ezali na libunga kati na mibale.
tree-contradicts-pair = Bakomami lokola mobali na mwasi mpe lokola moboti na mwana:
tree-contradicts-more = { $n ->
        [one] Bowangani moko lisusu etangami te.
       *[other] Bowangani { $n } lisusu etangami te.
    }
tree-no-people = Natango moto azali na nzete oyo te.
tree-no-people-cta = Kotisa fisyé ya libota, to bakisa moto ya liboso.
tree-nobody-selected = Moto te ya kobenda mpo na bopono wana.
tree-nobody-selected-cta = Banda na komonana ya ebandeli.
tree-edge-union = Boyokani ekomami
tree-edge-parentage = Bomoboti ekomami

home-empty = Natango eloko moko te ekomami.
home-count = { $total ->
        [one] Mokanda moko
       *[other] Mikanda { $total }
    }, ebombami na fisyé moko oyo libota ezali nkolo na yango.
home-unnamed-family = Nzete oyo ya libota
home-in-this-tree = Oyo libota ekomi kino sikoyo
home-showcase-title = Esika nzete oyo esili koloba koleka bankombo mpe mikolo

home-showcase-example = Tala ndakisa →
home-nothing-title = Natango eloko te ya kolakisa.
home-nothing-admin = Bosimbi oyo ezali naino na libota te: bongisa yango uta na ebombelo, uta na fisyé GEDCOM, to uta na eloko te.
home-nothing-visitor = Ebombelo oyo ya libota ezali kobongisama.
home-eyebrow = Ebombelo ya libota
home-hero-1 = Bomoi nyonso ezali liziba.
home-hero-2 = Libota nyonso, ebale.
view-tree-hero-1 = Bomoi nyonso ezali lokasa.
view-tree-hero-2 = Libota nyonso, nzete.
view-galaxy-hero-1 = Bomoi nyonso ezali monzoto.
view-galaxy-hero-2 = Libota nyonso, galaksi.
view-summit-hero-1 = Bomoi nyonso ezali litambe.
view-summit-hero-2 = Libota nyonso, komata.
home-cta-river = Yeba ebale
view-tree-cta = Yeba nzete
view-galaxy-cta = Yeba galaksi
view-summit-cta = Mata ngomba
home-cta-tree = Tala nzete
home-scroll = Tambola mpo na koyeba
home-trust-title = Bondimi na bomoboti moko na moko
home-trust-certain = pene na sikisiki
home-trust-high = endimisami malamu
home-trust-medium = ekoki kozala
home-trust-low = likanisi
home-trust-unrated = natango emekami te
home-showcase-eyebrow = Koleka bankombo mpe mikolo
home-cta-title = Elongo, tobatela bokundoli ya libota.
home-cta-detail = Nkombo nyonso oyo ebakisami mpe liziba nyonso oyo etalami ekomisaka ebale ya solo koleka mpo na baoyo bakoya sima.
view-tree-cta-detail = Nkombo nyonso oyo ebakisami mpe liziba nyonso oyo etalami ekomisaka nzete ya solo koleka mpo na baoyo bakoya sima.
view-galaxy-cta-detail = Nkombo nyonso oyo ebakisami mpe liziba nyonso oyo etalami ekomisaka galaksi ya solo koleka mpo na baoyo bakoya sima.
view-summit-cta-detail = Nkombo nyonso oyo ebakisami mpe liziba nyonso oyo etalami ekomisaka ngomba ya solo koleka mpo na baoyo bakoya sima.
home-cta-add = Bakisa moto

showcase-links-title = { $n ->
        [one] Boyokani moko libanda ya libota
       *[other] Boyokani { $n } libanda ya libota
    }
showcase-links-detail = Batata mpe bamama ya batisimo, bankolo mosala, batatoli mpe balakisi, moko na moko na mikolo na ye, liziba na ye mpe ndenge ozali na bondimi.
showcase-occupations-title = { $n ->
        [one] Mosala moko na ebandeli mpe suka
       *[other] Misala { $n } na ebandeli mpe suka
    }
showcase-occupations-detail = “Molakisi, 1948–1978” ebatelaka bolai na yango, mpe ebendamaka lokola molongo oyo ekatisi mibu na esika ya molongo ya mokolo moko.
showcase-uncertain-title = { $n ->
        [one] Mokolo moko oyo etikami na ntembe ndenge epesamaki
       *[other] Mikolo { $n } oyo etikami na ntembe ndenge epesamaki
    }
showcase-uncertain-detail = Pene na, liboso, sima mpe kati na etikalaka maloba minei ekeseni. Mokolo oyo liziba ekokaki kotia sikisiki te emonisamaka soko moke te lokola nde etiamaki.
showcase-preserved-title = { $n ->
        [one] Mokolo moko oyo ebatelami na maloba oyo ekomamaki
       *[other] Mikolo { $n } oyo ebatelami na maloba oyo ekomamaki
    }
showcase-preserved-detail = Maloba oyo moto moko te akokaki kotanga lokola mokolo ebatelamaka ndenge ekomamaki, na esika ya kobwakama na nkimi.
showcase-sources-title = { $n ->
        [one] Liziba moko oyo bondimi na yango ekomami
       *[other] Maziba { $n } oyo bondimi na yango ekomami
    }
showcase-sources-detail = { $primary ->
        [one] Liziba moko ya liboso.
       *[other] { $primary } ya liboso.
    } Likambo moko na moko emonisaka litatoli oyo etelemi likolo na yango, mpe ndenge litatoli yango ezali makasi.
showcase-places-title = { $n ->
        [one] Esika moko oyo ndelo na yango etambolaki
       *[other] Bisika { $n } oyo ndelo na yango etambolaki
    }
showcase-places-detail = Engumba ekoki kozala ya mikili ekeseni na ntango ekeseni, mpe mokanda elobi oyo ezalaki na ntango nini.

record-also-recorded-as = ekomami mpe lokola
record-borders-moved = Ndelo etambolaki:
record-display-name = nkombo ya komonisa
record-read-as = tanga lokola
record-note = Makomi
record-living-yes = na bomoi
record-deceased = akufa
record-open-full-page = Fungola lokasa mobimba ↗
record-open-full-title = Fungola lokasa oyo ezali yango moko, oyo ekoki kokabolama
record-edit = Bongola
record-see-in-river = Tala na ebale
view-tree-see = Tala na nzete
view-galaxy-see = Tala na galaksi
view-summit-see = Tala na ngomba
panel-empty = Pona karte mpo na komona mokanda mobimba ya moto yango awa.
person-visibility-inline = komonana:
person-age-at-death = { $n ->
        [one] akufaki na mobu { $n }
       *[other] akufaki na mibu { $n }
    }
person-age-now = { $n ->
        [one] azali na mobu { $n }
       *[other] azali na mibu { $n }
    }
person-born-in = abotamaki na { $place }
person-died-in = akufaki na { $place }
person-children-count = { $n ->
        [one] mwana moko
       *[other] bana { $n }
    }
person-generations-below = { $n ->
        [one] nkola moko na nse
       *[other] nkola { $n } na nse
    }
person-portrait-of = Foto ya { $name }
person-no-portrait = Foto moko te ekomami

result-diagnostics = Botali
result-diagnostics-note = Botali nyonso oyo bibliothèque ezongisaki, elongo na makebisi oyo epekisaki mosala te. Moko te elongolami.
result-no-diagnostics = Bibliothèque ezongisaki botali moko te.
result-continue = Kokoba
result-dashboard = Etando ya bokambi
person-sections-label = Biteni na lokasa oyo

name-part-nasab = nasab (libota ya baboti)
name-part-laqab = laqab (nkombo ya kopesa)
name-part-kunya = kunya (nkombo na nzela ya mwana)
name-part-nisbah = nisbah (esika auti)
name-part-alias = nkombo mosusu
name-part-religious_name = nkombo ya losambo
name-part-pen_name = nkombo ya mokomi
name-type-pen_name = nkombo ya mokomi
gender-U = Ekomami te

record-notes-title = Makomi mpo na mokanda oyo:
record-name = Nkombo
record-type = Lolenge
record-cause = Ntina:
record-as = lokola
record-partner-not-recorded = Molongani akomami te
record-union-from = Uta
record-union-at = na
record-union-until = kino
record-occupation-from = uta
record-occupation-until = kino
record-source-reliability = Bondimi
record-source-supports = Esungaka
record-photographs = Bafoto
record-documents = Mikanda
record-file = Fisyé
record-status = Lolenge
record-size = Bonene
record-absent-document = Etangami na moto oyo kasi ebombami awa te.
record-upload-help-short = Kino MB { $mb }. SVG eboyamaka.
record-verbatim-note = Ebatelami ndenge mokanda elobaki, mpo mobongoli moko te akokaki kososola yango.
record-file-to-attach = Fisyé ya kokangisa
record-document-type = Lolenge ya mokanda
record-caption = Maloba ya foto
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }
kind-person = moto
kind-family = libota
kind-event = likambo
kind-link = boyokani
kind-occupation = mosala
kind-source = liziba
kind-place = esika
kind-document = mokanda

kind-person-plural = { $n ->
        [one] moto
       *[other] bato
    }
kind-family-plural = { $n ->
        [one] libota
       *[other] mabota
    }
kind-event-plural = { $n ->
        [one] likambo
       *[other] makambo
    }
kind-link-plural = { $n ->
        [one] boyokani
       *[other] boyokani
    }
kind-occupation-plural = { $n ->
        [one] mosala
       *[other] misala
    }
kind-source-plural = { $n ->
        [one] liziba
       *[other] maziba
    }
kind-place-plural = { $n ->
        [one] esika
       *[other] bisika
    }
kind-document-plural = { $n ->
        [one] mokanda
       *[other] mikanda
    }

list-matching = { $total ->
        [one] Moko eyokani
       *[other] { $total } eyokani
    }, { $per_page } na lokasa moko.
list-filter-placeholder = Pona na nkombo to elembo
list-filter = Pona
list-clear = Longola
list-summary = Mokuse
list-id = Elembo
list-nothing = Eloko ezali awa te.
list-nothing-matching = Eloko ezali awa te oyo eyokani na “{ $q }”.
list-delete-confirm = Kolongola { $kind } oyo? Pona oyo ekosalema na mikanda oyo etangi yango:
list-policy-reject = Boya
list-policy-reject-detail = — boya soki eloko ezali naino kotanga yango. Eloko moko te ebungi.
list-policy-cascade = Na molongo
list-policy-cascade-detail = — longola yango mpe longola mpenza boyokani nyonso na yango.
list-policy-orphan = Etike
list-policy-orphan-detail = — longola yango kasi batela mikanda oyo etangaki yango, boyokani etikali mpamba.

completeness-dates-title = Mikolo, na lolenge oyo ezali na yango mpenza
completeness-no-dates = Natango mikolo moko te ekomami.
completeness-dates-note = Mokolo oyo moto atiaki kino na mokolo moko mpe mokolo oyo moto akokaki kotia kaka na mibu zomi ezali maloba ekeseni, mpe nyonso mibale ebatelamaka ndenge epesamaki. Makomi oyo ekokaki kotangama lokola mokolo te ebatelamaka liloba na liloba na esika ya kobwakama.
completeness-shape-exact = sikisiki
completeness-shape-exact-note = mokolo mobimba ya manaka
completeness-shape-approximate = pene na pene
completeness-shape-approximate-note = pene na, to mobu/mibu zomi kaka
completeness-shape-ranged = kati na ndelo
completeness-shape-ranged-note = liboso, sima, to kati na
completeness-shape-preserved = ebatelami
completeness-shape-preserved-note = makomi oyo etangamaki te, ebatelami ndenge ezali
completeness-shape-unknown = eyebani te
completeness-shape-unknown-note = ekomami lokola eyebani te

convert-page-title = Kotisa fisyé ya libota
convert-lede = Memá nzete oyo ezali uta na fisyé GEDCOM — fisyé oyo baprograme mingi ya bankoko ebimisaka. Eloko moko te ebombamaka awa, mpe nzete oyo site oyo emonisaka kala etikalaka ndenge ezalaki.
convert-file-label = Fisyé ya libota (.ged)
convert-file-hint = Kino MB { $mb }. Nzete ya bato 767 ezali pene na KB 320.
convert-confidence-label = Bondimi na makambo oyo, mpo na kobanda
convert-confidence-hint = Fisyé oyo ekotisami elobi te ndenge moto azalaki na bondimi, yango wana likambo moko na moko esengeli esika ya kobanda. Tia yango na nse mpo na nzete oyo esangisamaki noki, likolo koleka mpo na oyo esalemaki na mikanda. Ndimbola ya solo ya motuya oyo ezali “ekotisami, mpe moto moko te atali yango uta wana” — okoki kotombola to kokitisa likambo nyonso sima, moko moko.
convert-lang-label = Lokota ya bankombo ya bisika
convert-lang-hint = Elembo lokola en, fr to pl.

convert-failed = Bokotisi elongi te
convert-try-another = Meka fisyé mosusu
convert-converted = { $filename } ekotisami
convert-result-lede = { $total ->
        [one] Mokanda moko
       *[other] Mikanda { $total }
    }, KB { $size }. Nyonso ekotaki na bondimi ya { $confidence }, bankombo ya bisika etangami lokola { $lang }. Nzete oyo site oyo emonisaka esimbami te.
convert-produced = Oyo eleki
convert-skipped-title = { $n ->
        [one] Mokanda moko oyo ekokaki kotangama te
       *[other] Mikanda { $n } oyo ekokaki kotangama te
    }
convert-skipped-note = Mikanda oyo ezalaki na eloko moko te oyo ekokaki komemama.
convert-other-diagnostics = { $n ->
        [one] Likambo mosusu moko oyo ebongi koyeba
       *[other] Makambo mosusu { $n } oyo ebongi koyeba
    }
convert-clean = Eloko moko te etikalaki — mokanda nyonso na fisyé eleki.
convert-download-title = Kokitisa
convert-download-named = Kokitisa { $name }
convert-download-note = Ebombami awa miniti zomi na mitano mpe sima ebwakamaka, yango wana kitisa yango sikoyo.
convert-another = Kotisa fisyé mosusu
admin-history-on = na
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Libunga moko
       *[other] Mabunga { $errors }
    }, { $warnings ->
        [one] likebisi moko
       *[other] makebisi { $warnings }
    }, { $infos ->
        [one] liyebisi moko
       *[other] mayebisi { $infos }
    }.
admin-warnings-never-block = Makebisi epekisaka soko moke te — ezali mayebisi, kasi bikuke te.
admin-validator-clean = Mondimisi alobaki eloko moko te.
record-occupations-help-undated = Mosala ekomamaka na ebandeli mpe suka, mpo misala mingi ekoka kokokanisama na molongo moko ya ntango. Ebombelo oyo ezali na bankombo ya misala kasi mikolo na yango te — likambo ya momesano sima ya bokotisi, mpo bafisyé mingi ya mabota ezali na esika ya kobomba yango te — yango wana natango emekeli ezali te ya kobenda.
record-occupations-help-axis = Mosala ezali lolenge oyo ezali na boumeli, kasi likambo ya mokolo moko te. Ntango nyonso esangani molongo moko, { $from }–{ $to }.
admin-value-not-set = etiami te
admin-validation-report = Lapolo ya kondimisa
admin-dedup-complete = Kolongola mibale esili

admin-dedup-refused = Kolongola mibale eboyami
record-birth-order = molongo ya kobotama
record-start-not-recorded = ebandeli ekomami te
record-end-not-recorded = suka ekomami te
panel-selected-person = Moto oyo aponami

tree-band-generation = Nkola { $g }
tree-level-self = Nkola moko
tree-level-up = { $n ->
        [1] Bana
        [2] Bankoko
        [3] Bana ya bankoko
       *[other] Bakitani, nkola { $n }
    }
tree-level-down = { $n ->
        [1] Baboti
        [2] Bankoko
        [3] Baboti ya bankoko
       *[other] Bankoko ya kala, nkola { $n }
    }
tree-card-centre = Tia nzete awa na kati
tree-card-same = Moto moko, abendami mbala ya mibale
tree-zoom-out = Kokitisa
tree-zoom-in = Kobakisa
tree-zoom-fit = Kobongisa
tree-band-people = { $n ->
        [one] moto moko
       *[other] bato { $n }
    }
tree-band-unplaced = Bazangi esika
tree-band-unplaced-note = { $n ->
        [one] moto moko oyo azali na libota moko te — alakisami na esika ya kotikama
       *[other] bato { $n } oyo bazali na libota moko te — balakisami na esika ya kotikama
    }

gender-M = Mobali
gender-F = Mwasi
gender-NB = Mobali te mpe mwasi te
gender-unrecorded = Ekomami te

name-part-given_name = nkombo ya mbotama
name-part-family_name = nkombo ya libota
name-part-patronymic = nkombo ya tata
name-part-matronymic = nkombo ya mama
name-part-middle_name = nkombo ya katikati
name-part-nickname = nkombo ya kosakana
name-part-prefix = liloba ya liboso
name-part-suffix = liloba ya suka
name-part-particle = liloba ya kokangisa
name-part-part = eteni

name-type-primary = ya liboso
name-type-other = mosusu
name-type-alias = nkombo mosusu
name-type-birth = ya kobotama
name-type-married = ya libala
name-type-religious = ya losambo
name-type-transliteration = kobongola makomi
name-type-nickname = nkombo ya kosakana

note-links = { $n ->
        [one] boyokani moko libanda ya libota, na mikolo mpe maziba na yango
       *[other] boyokani { $n } libanda ya libota, na mikolo mpe maziba na yango
    }
note-occupations = { $n ->
        [one] mosala moko ekomami na ebandeli mpe suka
       *[other] misala { $n } ekomami na ebandeli mpe suka
    }
note-birth-imprecise = mokolo ya kobotama oyo liziba ekokaki kotia sikisiki te, emonisami ndenge ekomamaki
note-death-imprecise = mokolo ya kokufa oyo liziba ekokaki kotia sikisiki te, emonisami ndenge ekomamaki
note-names = { $n ->
        [one] nkombo moko ekomami
       *[other] bankombo { $n } ekomami
    }
note-transliteration = nkombo na makomi na yango moko pembeni ya kobongola na yango na makomi ya Latin
note-witnessed = { $n ->
        [one] likambo moko oyo atatolaki kasi ezalaki ya ye te
       *[other] makambo { $n } oyo atatolaki kasi ezalaki ya ye te
    }

visibility-public = ya bato nyonso
visibility-members = bato ya libota
visibility-contributors = basungi
visibility-private = ya sekele

family-label-couple = { $children ->
        [0] { $a } mpe { $b }
        [one] { $a } mpe { $b } — mwana moko
       *[other] { $a } mpe { $b } — bana { $children }
    }
family-label-half = { $children ->
        [0] { $a } mpe { $unknown }
        [one] { $a } mpe { $unknown } — mwana moko
       *[other] { $a } mpe { $unknown } — bana { $children }
    }
family-label-children = { $others ->
        [0] { $first } — baboti bakomami te
        [one] { $first } mpe ndeko moko — baboti bakomami te
       *[other] { $first } mpe bandeko { $others } — baboti bakomami te
    }
family-label-empty = Libota oyo moto moko te akomami

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } mpe { $b }
event-more-people = { $a }, { $b } mpe { $others ->
        [one] moto mosusu moko
       *[other] bato mosusu { $others }
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } ezangi nkombo

event-category-birth = Kobotama
event-category-death = Kokufa
event-category-marriage = Libala
event-category-divorce = Koboma libala
event-category-baptism = Batisimo
event-category-burial = Kokunda
event-category-immigration = Koya kofanda
event-category-emigration = Kokende kofanda libanda
event-category-census = Botangi bato
event-category-residence = Esika ya kofanda
event-category-military = Mosala ya basoda
event-category-education = Boyekoli
event-category-other = Likambo

reliability-primary = liziba ya liboso
reliability-secondary = liziba ya mibale
reliability-tertiary = liziba ya misato
reliability-recollection = bokundoli
reliability-derivative = mosala oyo euti na mosusu
reliability-authored = mosala ya mokomi
reliability-oral = makambo ya monoko
reliability-unknown = bondimi eyebani te

document-type-photo = foto
document-type-certificate = mokanda ya ndingisa
document-type-letter = mokanda ya kotinda
document-type-record = mokanda ya kokoma
document-type-newspaper = eteni ya zulunalo
document-type-other = mokanda

completeness-title = Esika nzete oyo ekokaki koloba koleka
completeness-intro = Oyo ekomami mpe oyo ezali naino mpamba.
completeness-import-title = Oyo bokotisi ememaki
completeness-import-intro = Etangami uta na fisyé oyo osili kotinda. Molongo ya mpamba ezali eloko oyo fisyé ya ebandeli ekomaki te — kasi eloko oyo bokotisi ebungisaki te.

completeness-headline-full = Lolenge nyonso ya makambo awa na nse ekomami esika moko na nzete oyo.
completeness-headline-empty = { $total ->
        [one] Lolenge moko ya makambo awa na nse ekomami naino esika moko te.
       *[other] Moko te kati na mitindo { $total } ya makambo awa na nse ekomami naino.
    } Moko na moko ezali esika oyo mokanda ekokaki koloba koleka.
completeness-headline-partial = { $carried ->
        [one] Lolenge moko ya makambo awa na nse ekomami
       *[other] Mitindo { $carried } ya makambo awa na nse ekomami
    }; { $empty ->
        [one] moko ezali naino mpamba
       *[other] { $empty } ezali naino mpamba
    }.

completeness-metric-confidence = Bondimi na likambo moko na moko
completeness-metric-confidence-none = Moko te kati na makambo { $slots } awa elobi ndenge ezali sikisiki. Mokolo oyo moto atangaki na mokanda ya ndingisa mpe oyo moto akanisaki emonanaka ndenge moko kino mokolo ekeseni.
completeness-metric-confidence-uniform = Makambo { $with } kati na { $slots } ezali na motuya mpe nyonso ezali na motuya moko ({ $modal }). Yango nde bokotisi ya mbala moko etikaka sima: motuya ya kotia esika oyo moto moko te azongeli. Moko te esambisami naino yango moko.
completeness-metric-confidence-some = Makambo { $with } kati na { $slots } ezali na motuya. { $modal_count } ekabolaka motuya moko ({ $modal }); { $assessed } ekeseni na yango mpe yango wana etalamaki moko moko.
completeness-metric-confidence-many = Makambo { $with } kati na { $slots } ezali na motuya, { $assessed } kati na yango ekeseni na motuya oyo ezali mingi ({ $modal }) na mitindo { $distinct } ekeseni. Nzete oyo ekomaka ntembe ya solo oyo ebongwanaka.

completeness-metric-parentage = Bondimi na boyokani moko na moko ya moboti na mwana
completeness-metric-parentage-none = Bomoboti moko te awa elobi ndenge ezali sikisiki. Kozwa mwana ya kobokola, mabota oyo ezali na ntembe mpe kotonga lisusu uta na litangi moko nde mpenza esika libota esengeli kokoma ntembe — mpe nzete ebendaka boyokani oyo ezali sikisiki te lokola molongo ya pepele.
completeness-metric-parentage-some = { $n ->
        [one] Bomoboti moko ezali na motuya na yango
       *[other] Bomoboti { $n } ezali na motuya na yango
    }, boye molongo ya likanisi emonanaka na bolembu koleka oyo endimisami na mikanda.

completeness-metric-links = Boyokani koleka makila mpe libala
completeness-metric-links-none = Batata mpe bamama ya batisimo, bankolo mosala, batatoli, balakisi, babateli. Moko te ekomami naino. Moko na moko ekoki komema mikolo na yango, liziba na yango mpe bondimi na yo.
completeness-metric-links-some = { $n ->
        [one] Moko ekomami, na mikolo, liziba mpe bondimi na yango.
       *[other] { $n } ekomami, moko na moko na mikolo, liziba mpe bondimi na yango.
    }

completeness-metric-occupations = Mosala ekomami na ebandeli mpe suka
completeness-metric-occupations-none = Misala moko te ekomami. Mosala oyo esalemaki mibu ntuku misato elobaka mingi mpo na bomoi koleka mokanda moko na mokolo moko.
completeness-metric-occupations-undated = { $total ->
        [one] Mosala moko ekomami, kozanga mikolo
       *[other] Misala { $total } ekomami, kozanga mikolo
    }. Bakisa ebandeli mpe suka mpe ekoki kokokanisama pembeni na pembeni na molongo moko ya ntango.
completeness-metric-occupations-some = { $span } kati na { $total } ezali na ebandeli to suka, boye ekoki kokokanisama pembeni na pembeni na molongo moko ya ntango.

completeness-metric-sources = Maziba oyo emekami na bondimi na yango
completeness-metric-sources-none = Maziba moko te ekomami. Kotanga esika likambo eutaki nde epesaka ndeko nzela ya kotala yango sima — to koyokana na yango te mpe koloba ntina.
completeness-metric-sources-some = { $graded } kati na { $total } elobi ndenge ezali makasi, boye likambo oyo etelemi na mokanda ya kobotama emonanaka ekeseni na oyo etelemi na bokundoli.

completeness-what-is-recorded = Oyo mokanda ekoki koloba
completeness-in-this-tree = Na nzete oyo
completeness-not-yet = natango ekomami te

role-spouse = molongani
role-spouse_1 = molongani ya liboso
role-spouse_2 = molongani ya mibale
role-subject = moto oyo etali
role-participant = mosangani
role-witness = motatoli
role-officiant = mokambi ya molulu
role-informant = mopesi sango
role-godparent = tata to mama ya batisimo

phys-no-source = liziba ezali te
phys-col-date = Ntango nini
phys-col-source = Liziba
phys-col-confidence = Bondimi
phys-col-note = Makomi
phys-field-height-cm = Bolai
phys-field-weight-kg = Kilo
phys-field-eye-colour = Langi ya miso
phys-field-hair-colour = Langi ya nsuki
phys-field-build = Lolenge ya nzoto
phys-field-handedness = Loboko oyo asalelaka mingi
phys-field-features = Bilembo ya koyeba ye
phys-field-military = Mosala ya basoda
phys-field-languages = Minoko oyo alobaka
phys-field-blood-group = Lolenge ya makila
phys-field-conditions = Bokono oyo eyebani
phys-field-operations = Kokata nzoto mpe bampota
phys-field-cause-of-death = Ntina ya kokufa
phys-field-religion = Losambo to masanga
phys-field-health-notes = Makomi
admin-export-health-note = Bobimisi ya pete etikaka mitindo nyonso ya makambo ya bokebi — bokolongono mpe kondima, makambo ya nzoto, makambo ya bankoko ya makila mpe mikanda ya bosambisi — mpe lolenge ya bizaleli ya moto nyonso oyo azali na bomoi, boye fisyé oyo etindami na ndeko ememaka moko te kati na yango. Pona oyo fisyé moko esengeli komema; ebombelo yango moko ekomaka mitindo nini etikamaki.
avatar-picker-title = Pona elilingi
avatar-choose = Elilingi nini emonisaka moto oyo
avatar-mode-auto = Tika programe epona
avatar-mode-auto-note = Foto ya elongi ya liboso, to soki ezali te elilingi ya liboso oyo ekangisami na mokanda oyo.
avatar-mode-none = Lakisa bilembo ya liboso ya nkombo na esika na yango
avatar-mode-none-note = Mpo na mokanda oyo bilili na yango kaka ezali mikanda kasi bilongi te.
avatar-focal-hint = Fina elilingi mpo na kopona yango, mpe fina lisusu eteni oyo esengeli kotikala na kati — avatar ezali na bampembeni minei ekokani mpe bafoto mingi ya scan ezali bongo te.
avatar-no-images = Natango elilingi moko te ekangisami na mokanda oyo.
avatar-upload-title = Tinda elilingi mpe salela yango
avatar-upload-button = Tinda mpe salela lokola elilingi
avatar-not-available-title = Elilingi wana ezali te
avatar-not-available-detail = Fisyé oyo eponami ekangisami na moto oyo te, to ezali te oyo okoki kotanga.
ai-badge = AI
ai-portraits-title = Bafoto oyo AI ebongisi
ai-portraits-band = Ebongisami
ai-portraits-upload = Tinda bafoto oyo AI ebongisi
ai-portraits-manage = Kamba bafoto ya AI
ai-portraits-intro = Bafoto oyo AI esalaki uta na bafoto ya moto oyo: ndenge ekoki kozala azalaki komonana. Emonanaka pembeni ya elongi na mokanda, na elembo “AI”, mpe kaka bato ya libota oyo bakoti nde bamonaka yango. Ezali soko moke te foto ya moto, soko moke te kati na bafisyé na ye, soko moke te litatoli.
ai-portraits-current = Na mokanda
ai-portraits-add = Bakisa bafoto — esika mpo na { $n }
ai-portraits-files = Bafoto (PNG, JPEG, WebP to GIF), mingi mbala moko
ai-portraits-generator = Esalemi na
ai-portraits-from = Esalemi uta na
ai-portraits-from-none = Elobami te
ai-portraits-remove = Longola
ai-portraits-alt = Foto ya { $name }, oyo AI ebongisi
ai-portraits-not-evidence = Esalemi na AI uta na bafoto: ezali bafoto ya ntango wana te.
ai-portraits-full = Mokanda ezali na bafoto na yango misato ya AI. Longola moko mpo na kobakisa mosusu.
ai-portraits-too-many = Mokanda ezwaka bafoto misato ya AI kaka. Longola moko liboso ya kobakisa mosusu.
ai-portraits-images-only = Foto ya AI esengeli kozala elilingi: PNG, JPEG, WebP to GIF.
ai-portraits-stale = Mokanda ebongwanaki ntango lokasa oyo ezalaki ya kofungwama. Eloko moko te ebombami: fungola yango lisusu mpe sala lisusu.
ai-portraits-back = Zonga na bafoto ya AI

record-history-withheld = ebombami mpo na yo

record-life-status = Lolenge
record-presumed-deceased = akanisami ete akufa
record-presumed-short = ekanisami
record-presumed-why = Kokufa moko te ekomami mpe kobotama eleki mibu { $years }, yango wana mokanda oyo ekoki kozala malamu te. Ebombelo ebongwani te: oyo nde lokasa ekanisi, kasi oyo liziba elobi te.

identity-editor-title = Bankombo mpe bomoto
identity-primary-name = Nkombo oyo emonanaka bisika nyonso
identity-primary-help = Oyo karte ya nzete, motó ya lokasa mpe molongo nyonso esalelaka. Bankombo mosusu awa na nse ezali oyo liziba moko ebengaki moto oyo na ntango mosusu.
identity-display = Nkombo
identity-display-latin = Na makomi ya Latin
identity-culture = Lokota
identity-direction = Nzela
identity-direction-ltr = uta na loboko ya mwasi kino na ya mobali
identity-direction-rtl = uta na loboko ya mobali kino na ya mwasi
identity-direction-auto = uta na makomi
identity-components = Biteni ya nkombo
identity-components-help = Eteni nini ezali nkombo ya mbotama mpe nini nkombo ya libota, na molongo oyo ekomamaka. Mokanda oyo ezangi biteni emonisamaka kaka: biteni nde boluki ekoki koyokanisa.
identity-part = Eteni
identity-value = Makomi
identity-other-names = Bankombo mosusu
identity-other-help = Nkombo ya libala, nkombo ya losambo, nkombo oyo mokanda ya sima esalelaki. Moko na moko ememaka ntango esalelamaki mpe liziba oyo elobi yango.
identity-name-type = Lolenge ya nkombo
identity-valid-from = Esalelamaki uta
identity-valid-until = Esalelamaki kino
identity-about = Mpo na moto
identity-living-help = Oyo ezali elembo oyo liziba etiaki. Lokasa ekanisaka kokufa na ndenge mosusu soki kobotama eleki kala mingi, mpe makanisi yango ebongolaka soko moke te kesi oyo to ebombelo.
identity-error-no-display = Mokanda esengeli kozala na nkombo mpo na komonana. Eloko moko te ebombami.
editor-blank-to-remove = Longola nkombo mpo na kolongola eteni oyo.
identity-edit-link = Bongola bankombo mpe bomoto

union-type-marriage = libala
union-type-civil_union = boyokani ya leta
union-type-cohabitation = bafandaki elongo
union-type-religious_only = boyokani ya losambo
union-type-polygamous = libala ya basi mingi
union-type-unknown = ekomami te
union-role-spouse = molongani
union-role-partner = moninga ya bomoi
union-role-husband = mobali
union-role-wife = mwasi
union-status-active = ezali kokoba
union-status-ended_by_death = esilaki na kokufa
union-status-ended_by_divorce = esilaki na koboma libala
union-status-ended_by_separation = esilaki na kokabwana
union-status-annulled = elongolamaki
union-status-unknown = ekomami te
union-status-ended = esilaki
union-status-ended-by = esilaki na { $reason }
union-reason-death_of_spouse = kokufa ya molongani
precision-exact = kino na mokolo
precision-year = kino na mobu
precision-month = kino na sanza
precision-decade = kino na mibu zomi
precision-century = kino na ekeke
precision-unknown = eyebani te
record-precision = Bosikisiki
record-place = Esika

family-editor-title = Libota mpe boyokani
family-unions = Boyokani ya balongani
family-no-unions = Boyokani moko te ekomami mpo na moto oyo.
family-union-legend = Boyokani { $n }
family-writes-family = Kobomba oyo ekobongola mokanda ya libota #{ $id }, oyo bato mibale bakabolaka. Lokasa ya moto mosusu ekobongwana elongo na yango.
family-partners = Balongani
family-partner = Molongani
family-role = Mosala
family-children = Bana
family-children-help = Molongo ya kobotama ezali liloba ya mokanda yango moko mpo na nani ayaki liboso. Soki etikami mpamba, eloko moko te elobami: motuya oyo ezwami na esika ya molongo ekozala likambo oyo moto moko te akomaki.
family-child = Mwana
family-birth-order = Molongo ya kobotama
family-the-union = Boyokani yango moko
family-type = Lolenge ya boyokani
family-status = Ndenge ezali
family-started = Ebandaki
family-ended = Esilaki
family-leave = Longola moto oyo na boyokani oyo
family-open-entity = Fungola mokanda ya libota
family-new-union = Boyokani ya sika
family-new-union-help = Oyo esalaka mokanda ya sika ya libota oyo moto oyo azali kati na yango. Molongani ezali na makasi te: moboti oyo mokanda etangi kozanga molongani ezali boyokani ya moto moko.
family-create-union = Sala boyokani
family-parents = Baboti
family-no-parents = Moto oyo akomami te lokola mwana ya libota moko.
family-child-of = Mwana ya libota oyo
family-detach-child = Longola moto oyo na libota oyo
family-attach-parents = Kangisa na baboti
family-attach-help = Pona libota oyo moto oyo azali mwana na yango. Oyo ebakisaka ye na mokanda ya libota yango, boye amonanaka mpe na nkasa ya baboti.
family-the-family = Libota
family-attach = Kangisa
family-error-last-partner = Boyokani esengeli kozala na moto moko na moke. Na esika na yango longola mokanda ya libota, oyo ekotuna nini ya kosala na nyonso oyo etangi yango.
family-error-no-family = Libota moko te eponami. Eloko moko te ebombami.
family-error-already-child = Moto oyo azali kala mwana ya libota wana.
pick-error-empty = Moto moko te atangami. Eloko moko te ebombami.
pick-error-not-found = Moto ya nkombo wana azali na ebombelo oyo te. Eloko moko te ebombami.
pick-error-ambiguous = Bato koleka moko bayokani na yango. Pona moko na molongo mpo mokanda eloba nani. Eloko moko te ebombami.

links-editor-title = Boyokani
links-editor-help = Boyokani oyo ezali ya libota te: tata ya batisimo, nkolo mosala, motatoli, etuluku ya basoda. Moko na moko ezali mokanda na yango moko oyo etangi bato mibale, boye kobongola awa ebongolaka oyo mokanda mosusu emonisaka.
links-none = Boyokani moko te ekomami mpo na moto oyo.
links-new = Boyokani ya sika
links-create = Sala boyokani
links-remove = Longola boyokani oyo
links-other-end = Suka mosusu
links-label = Ezali nini
links-label-reverse = Etangami na nsima
links-category = Lolenge
links-bidirectional = Etangamaka ndenge moko na ngambo nyonso mibale
links-from = Uta
links-until = Kino
links-reversed = Boyokani oyo ebendamaki uta na mokanda mosusu. Kobongola yango awa ebongolaka mokanda yango moko.
link-error-no-label = Boyokani esengeli koloba ezali nini. Eloko moko te ebombami.
occupations-editor-title = Misala
occupations-editor-help = Mosala ezali ntango oyo ezali na ebandeli mpe suka, kasi nkombo ya mosala kaka te. Moko na moko ezali na mikolo mpe liziba na yango.
occupations-none = Mosala moko te ekomami mpo na moto oyo.
occupations-new = Mosala ya sika
occupations-create = Sala mosala
occupations-remove = Longola mosala oyo
occupations-title = Oyo asalaki
occupations-employer = Mpo na nani
occupations-employer-place = Esika azalaki
occupations-from = Uta
occupations-until = Kino
occupation-error-no-title = Mosala esengeli koloba oyo moto asalaki. Eloko moko te ebombami.
link-category-spiritual = ya molimo
link-category-professional = ya mosala
link-category-social = ya bato
link-category-legal = ya mibeko
link-category-medical = ya monganga
link-category-educational = ya boyekoli
link-category-conflict = matata
link-category-other = mosusu
links-edit-link = Bongola boyokani
occupations-edit-link = Bongola misala
family-edit-link = Bongola libota mpe boyokani

events-editor-title = Makambo
events-editor-help = Likambo etangaka bato mingi mbala moko — libala, batisimo, botangi bato — yango wana moko na moko ezali mokanda na yango moko mpe emonanaka na lokasa nyonso oyo etangi.
events-none = Likambo moko te etangi moto oyo.
events-new = Likambo ya sika
events-new-help = Moto oyo abakisamaka lokola moto oyo likambo etali soki otangi moto mosusu te. Likambo ezangi moto ezali kaka mokolo.
events-create = Sala likambo
events-remove = Longola likambo oyo
events-category = Oyo esalemaki
events-subcategory = Na bosikisiki koleka
events-description = Ndimbola
events-participants = Nani azalaki wana
events-participants-help = Kobomba oyo ekobongola mokanda ya likambo, oyo moto nyonso mosusu oyo atangami kati na yango amonisaka mpe.
events-who = Nani
event-error-no-category = Likambo esengeli koloba oyo esalemaki. Eloko moko te ebombami.
documents-editor-title = Mikanda
documents-editor-help = Bafisyé nini mokanda oyo elakisaka, mpe moko na moko ezali nini mpo na yango. Kolongola molongo ekabolaka fisyé: mokanda mpe babaiti na yango etikalaka na ebombelo.
documents-attached = Ekangisami na mokanda oyo
documents-upload = Tinda fisyé
documents-upload-help = Kino MB { $mb }. Fisyé ebombamaka na ebombelo mpe ekangisamaka na mokanda oyo.
documents-caption = Maloba ya foto
events-edit-link = Bongola makambo

prefs-style = Bopikami
style-comfortable = Na esika
style-comfortable-note = oyo ya ebandeli, na esika ya kotanga
style-compact = Ekangami
style-compact-note = eteni monene ya mokanda na ekrani moko, mpo na kosala na mingi
style-paper = Papye
style-paper-note = makomi ya kala mpe milongo na esika ya bakarte, mpo na kotanga mbala moko to kobimisa na papye

admin-export-choose = Kotisa na bobimisi oyo
scope-health = Bokolongono mpe kondima
scope-biometrics = Makambo ya nzoto
scope-genomics = Makambo ya makila (ADN)
scope-legal = Mikanda ya bosambisi
scope-behaviour = Bizaleli ya bato oyo bazali na bomoi
admin-export-with-chosen = Bimisa na oyo eponami

pg-identity = Bomoto mpe lolenge ya leta
pg-identity-intro = Moto yango azalaki nani na ndenge mokanda elobi, mpe oyo mikanda ya leta ekomaki.
pg-morphology = Lolenge ya nzoto
pg-morphology-intro = Nzoto ndenge emekamaki mpe elimbolamaki.
pg-biometrics = Makambo ya nzoto
pg-biometrics-intro = Mongongo, maboko mpe mayoki, mpe bilembo oyo eyebisaka moto.
pg-health = Bokolongono
pg-health-intro = Bokono, bosalisi, bomeki mpe biloko oyo ebimaki.
pg-genomics = Makambo ya makila
pg-genomics-intro = Bomeki ya ADN, haplogroupes, mbongwana ya bageni mpe biloko mosusu ya molekile.
pg-death = Kokufa
pg-death-intro = Ndenge nini, ntango nini mpe esika nini bomoi esilaki, mpe nini esalemaki na nzoto.
pg-residence = Esika ya kofanda mpe bomboka
pg-residence-intro = Esika moto afandaki, mikili nini mizalaki koloba ete azali na yango, mpe minoko alobaki.
pg-education = Boyekoli mpe mosala
pg-education-intro = Kelasi, mikanda ya boyekoli, lifuti mpe biloko.
pg-military = Basoda mpe lokumu
pg-military-intro = Mosala, mabonza, bituluku mpe bilembo ya lokumu.
pg-legal = Mibeko
pg-legal-intro = Makambo ya bosambisi mpe oyo ebimaki.
pg-belief = Kondima mpe masanga
pg-belief-intro = Losambo, milulu, kondima mpe masanga.
pg-personality = Bizaleli mpe etamboli
pg-personality-intro = Motema, mimeseno mpe masano, ndenge maziba elobi.
pg-relationships = Boyokani
pg-relationships-intro = Baboti, balongani, bana mpe bato mosusu na bomoi.
pg-digital-legacy = Libula ya numérique
pg-digital-legacy-intro = Ba-scan, bamodele, bamongongo ekomami mpe biloko ya kobomba oyo ezali komonisa moto.
pa-identity-titles = Bankombo ya lokumu
pa-identity-sex-at-birth = Mobali to mwasi na kobotama
pa-identity-gender-identity = Ndenge amimonaka
pa-birth-time = Ngonga ya kobotama
pa-birth-coordinates = Esika ya kobotama, na bisika ya mapa
pa-civil-status-birth-certificate-number = Motuya ya mokanda ya kobotama
pa-civil-status-register-entries = Makomi na buku ya leta
pa-civil-status-marginal-annotations = Makomi na pembeni
pa-morphology-height = Bolai
pa-morphology-weight = Kilo
pa-morphology-bmi = Motuya ya kilo ya nzoto (IMC)
pa-morphology-body-composition = Lolenge ya biteni ya nzoto
pa-morphology-build = Lolenge ya nzoto
pa-morphology-eye-colour = Langi ya miso
pa-morphology-eye-shape = Lolenge ya miso
pa-morphology-eye-spacing = Esika kati na miso
pa-morphology-hair-colour = Langi ya nsuki ya mbotama
pa-morphology-hair-texture = Lolenge ya nsuki
pa-morphology-hairline = Ndelo ya nsuki
pa-morphology-facial-hair = Mandefu
pa-morphology-body-hair = Nsuki ya nzoto
pa-morphology-skin-tone = Langi ya loposo (Fitzpatrick)
pa-morphology-skin-undertone = Langi ya nse ya loposo
pa-morphology-freckles = Matono ya loposo
pa-morphology-pigmentation = Bilembo ya langi na loposo
pa-morphology-scars = Bilembo ya bampota
pa-morphology-tattoos = Bakatakata ya loposo
pa-morphology-moles = Matono
pa-morphology-facial-asymmetries = Elongi oyo ekokani te
pa-morphology-face-shape = Lolenge ya elongi
pa-morphology-nose-shape = Lolenge ya zolo
pa-morphology-ear-shape = Lolenge ya matoyi
pa-morphology-lip-shape = Lolenge ya mbebu
pa-morphology-dentition = Mino
pa-morphology-malocclusion = Mino oyo ekutani malamu te (lolenge ya Angle)
pa-morphology-posture = Ndenge atelemaka
pa-morphology-gait = Ndenge atambolaka
pa-morphology-distinguishing-features = Bilembo ya koyeba ye
pa-biometrics-fingerprints = Bilembo ya misapi
pa-biometrics-retinal-print = Elembo ya retina
pa-biometrics-voice-signature = Elembo ya mongongo
pa-biometrics-voice-frequency = Motuya ya mongongo
pa-biometrics-vocal-timbre = Lolenge ya mongongo
pa-biometrics-spoken-accent = Ndenge alobaka
pa-biometrics-speech-rate = Lombangu ya koloba
pa-biometrics-verbal-tics = Maloba oyo azongelaka
pa-biometrics-frequent-vocabulary = Maloba oyo asalelaka mingi
pa-biometrics-speech-register = Lolenge ya lokota
pa-biometrics-motor-tics = Bisalelo ya nzoto oyo azongelaka
pa-biometrics-handedness = Loboko oyo asalelaka mingi
pa-biometrics-hearing = Koyoka
pa-biometrics-visual-acuity = Komona
pa-biometrics-optical-correction = Bobongisi ya miso
pa-health-blood-group = Lolenge ya makila (ABO)
pa-health-rhesus = Rhésus (RhD)
pa-health-blood-pressure = Tansio
pa-health-resting-heart-rate = Mbeti ya motema na bopemi
pa-health-respiratory-capacity = Makoki ya kopema
pa-health-conditions = Bokono
pa-health-surgeries = Lisolo ya kokatama
pa-health-injuries = Bampota
pa-health-deformities = Nzoto oyo ebebi lolenge
pa-health-amputations = Biteni ya nzoto ekatami
pa-health-prostheses = Biteni ya nzoto ya bosali
pa-health-implants = Biloko etiami na nzoto
pa-health-devices = Masini etiami na nzoto
pa-health-medications = Nkisi
pa-health-allergies = Allergies
pa-health-vaccinations = Mangwele
pa-health-serology = Bomeki ya makila
pa-health-lab-results = Biloko ya laboratwari
pa-health-deficiencies = Bozangi
pa-health-sleep-disorders = Mikakatano ya mpongi
pa-health-mental-health-assessments = Bomeki ya bokolongono ya makanisi
pa-genomics-autosomal-mapping = Bomeki ya ADN autosomale
pa-genomics-y-haplogroup = Haplogroupe ya Y-ADN
pa-genomics-mt-haplogroup = Haplogroupe ya mitochondrie
pa-genomics-whole-genome-sequencing = Botangi génome mobimba
pa-genomics-risk-variants = Mbongwana ya bageni ya likama
pa-genomics-hereditary-conditions = Bokono ya libula

pa-genomics-predispositions = Makoki ya kozwa bokono
pa-genomics-epigenetic-markers = Bilembo ya épigénétique
pa-genomics-epigenetic-age = Mbula ya épigénétique
pa-genomics-gut-microbiome = Mikrobe ya misopo
pa-genomics-skin-microbiome = Mikrobe ya loposo
pa-genomics-toxicological-sensitivities = Bolembu liboso ya nkisi mpe ngenge
pa-death-time = Ngonga ya kokufa
pa-death-coordinates = Esika ya kokufa, na bisika ya mapa
pa-death-causes = Bantina ya kokufa
pa-death-contributing-factors = Makambo esalisaki
pa-death-autopsy = Bofungoli ya ebembe
pa-death-disposition = Nini esalemaki na nzoto
pa-death-grave = Lilita
pa-residence-addresses = Bisika ya kofanda
pa-residence-nationality-of-origin = Bomboka ya mbotama
pa-residence-acquired-nationalities = Bomboka oyo azwaki
pa-residence-mother-tongue = Lokota ya mama
pa-residence-spoken-languages = Minoko alobaka
pa-education-level = Litomba ya boyekoli
pa-education-diplomas = Mikanda ya boyekoli
pa-education-institutions = Bakelasi mpe bisalelo
pa-education-income = Lifuti
pa-education-real-estate = Bandako mpe mabelé
pa-military-distinctions = Bilembo ya lokumu
pa-military-citations = Maloba ya lokumu
pa-military-ranks = Mabonza
pa-military-units = Bituluku
pa-military-service-numbers = Motuya ya mosala
pa-legal-criminal-record = Mokanda ya bosambisi
pa-belief-religions = Losambo
pa-belief-sacraments = Masakrama mpe milulu
pa-belief-beliefs = Kondima
pa-belief-political-leanings = Makanisi ya politiki
pa-belief-memberships = Masanga
pa-personality-big-five = Bapwe ya Big Five
pa-personality-mbti = Lolenge ya MBTI
pa-personality-introversion-extraversion = Kimya mpe esengo ya bato
pa-personality-stress-tolerance = Bokasi liboso ya mitungisi
pa-personality-decision-style = Ndenge ya kozwa mikano
pa-personality-interests = Makambo alingaka
pa-personality-hobbies = Masano
pa-personality-sports = Masano ya nzoto
pa-personality-dietary-habits = Bilei
pa-personality-dependencies = Bozali moumbu ya eloko
pa-digital-legacy-body-models = Bamodele ya nzoto
pa-digital-legacy-skin-textures = Lolenge ya loposo
pa-digital-legacy-rigs = Mikuwa ya numérique
pa-digital-legacy-voice-corpora = Mongongo ekomami mpo na kosala mongongo
pa-digital-legacy-text-corpora = Makomi mpo na modele ya lokota
pa-digital-legacy-digital-traces = Bilembo ya numérique
pa-digital-legacy-carbon-footprint = Elembo ya carbone
pa-digital-legacy-behaviour-models = Bamodele ya bizaleli
pf-identity-titles-text = Nkombo ya lokumu ndenge ekomamaki
pf-identity-titles-kind = Lolenge ya nkombo ya lokumu
pf-civil-status-marginal-annotations-text = Makomi
pf-morphology-pigmentation-kind = Lolenge ya elembo
pf-biometrics-spoken-accent-description = Ndenge elimbolamaka
pf-biometrics-optical-correction-kind = Bobongisi
pf-health-amputations-level = Esika ekatamaki
pf-health-prostheses-kind = Eteni ya bosali
pf-health-implants-kind = Eloko etiami
pf-health-devices-kind = Masini
pf-health-allergies-type = Lolenge ya allergie
pf-health-vaccinations-status = Ndenge ya mangwele
pf-health-sleep-disorders-category = Lolenge ya mokakatano
pf-death-autopsy-kind = Bofungoli ya ebembe
pf-education-institutions-name = Nkombo ya esalelo
pf-military-distinctions-name = Nkombo ya elembo ya lokumu
pf-military-distinctions-kind = Lolenge ya elembo ya lokumu
pf-military-citations-text = Maloba ya lokumu
pf-military-ranks-category = Lolenge ya libonza
pf-belief-political-leanings-position = Esika na molongo gauche–droite
pf-belief-memberships-kind = Lolenge ya lisanga
pf-digital-legacy-carbon-footprint-method = Ndenge etangamaki
pf-age-years = Mibu
pf-agreeableness = Boboto
pf-allergen = Eloko ya allergie
pf-amount = Motuya
pf-analyte = Eloko emekami
pf-artefact-type = Lolenge ya eloko
pf-autoimmune = Nzoto ebundisaka yango moko
pf-body-region = Eteni ya nzoto
pf-bone-percent = Mokuwa
pf-carrier-status = Ndenge ya komema
pf-cause = Ntina
pf-chronic = Ya kala
pf-clock = Ngonga
pf-condition = Bokono
pf-conferred-by = Epesamaki na
pf-congenital = Ya mbotama
pf-conscientiousness = Bokebi
pf-consent = Ndingisa
pf-coordinates = Bisika ya mapa
pf-corrected = Na bobongisi
pf-country = Mboka
pf-court = Esambiselo
pf-coverage = Bozindo ya botangi
pf-currency = Mbongo
pf-decimal = Komona (décimale)
pf-denomination = Lingomba
pf-derived-from-id = Euti na
pf-description = Ndimbola
pf-details = Makambo ya mike
pf-diagnosis = Bomeki ya bokono
pf-diameter-mm = Bonene
pf-diastolic = Diastolique
pf-diet = Bilei
pf-document-id = Mokanda
pf-dose = Ndenge ya nkisi
pf-ear = Litoyi
pf-entry-number = Motuya ya makomi
pf-extraversion = Esengo ya bato
pf-eye = Liso
pf-fat-percent = Mafuta
pf-fev1-fvc-ratio = Bokokani FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = Lolenge ya fisyé
pf-findings = Oyo emonanaki
pf-flag = Elembo
pf-format = Lolenge
pf-fracture = Kobukana
pf-fvc-litres = FVC
pf-gene = Geni
pf-generator = Esalemaki na
pf-grade = Litomba
pf-iccs-section = Eteni ya mbeko (ICCS)
pf-icd10-chapter = Mokapo ya ICD-10
pf-indication = Ntina ya nkisi
pf-inheritance = Libula
pf-inscription = Makomi
pf-institution = Esalelo
pf-instrument = Esaleli
pf-isced-level = Litomba ya ISCED
pf-jurisdiction = Bokonzi ya bosambisi
pf-language = Lokota
pf-lat = Latitude
pf-level = Litomba
pf-lines = Esika ya kofanda
pf-location = Esika
pf-lon = Longitude
pf-major = Haplogroupe monene
pf-marker = Elembo
pf-metaboliser-status = Ndenge nzoto esalelaka nkisi
pf-method = Ndenge
pf-mode = Ndenge azwaki yango
pf-muscle-percent = Misuni
pf-neuroticism = Bobangi
pf-number = Motuya
pf-nutrient = Eloko ya bilei
pf-offence = Mbeko ebukami
pf-office = Mokumba
pf-openness = Bofungoli ya makanisi
pf-organisation = Lisanga
pf-outcome = Oyo ebimaki
pf-pace = Lombangu ya kokola
pf-page = Lokasa
pf-panel = Etuluku ya bomeki
pf-party = Lisanga ya politiki
pf-pathogen = Mikrobe ya bokono
pf-pattern = Ndenge ya kosalela
pf-percentile = Percentile
pf-period = Ntango ya lifuti
pf-place-id = Esika
pf-plot = Eteni ya mabelé
pf-polygenic-score = Pwe ya bageni mingi
pf-postal-code = Code postal
pf-precision = Bosikisiki
pf-prescription = Mokanda ya monganga
pf-proficiency = Boyebi
pf-provider = Mopesi
pf-quintile = Etuluku ya lifuti
pf-rank = Libonza
pf-rank-text = Libonza ndenge ekomamaki
pf-reaction = Eyano ya nzoto
pf-reference-build = Génome ya ndakisa
pf-reference-high = Ndelo ya ndakisa, likolo
pf-reference-low = Ndelo ya ndakisa, nse
pf-register-type = Lolenge ya makomi
pf-result = Oyo ebimaki
pf-role = Mosala
pf-sacrament = Sakrama to molulu
pf-score = Pwe
pf-sentence = Etumbu
pf-sequence = Esika na molongo
pf-service = Mosala
pf-severity = Bozito
pf-shannon-diversity = Bokeseni ya Shannon
pf-shape = Lolenge
pf-significance = Ntina mpo na monganga
pf-snp-count = Ba-SNP emekami
pf-sport = Lisano
pf-subclade = Etape moke
pf-substance = Eloko
pf-summary = Na mokuse
pf-systolic = Systolique
pf-tenure = Bozwi
pf-test = Bomeki
pf-threshold-db = Ndelo ya koyoka
pf-title = Motó ya likambo
pf-tonnes-co2e-per-year = Milinga ebimaka
pf-tradition = Bonkoko
pf-tree-version = Version ya nzete
pf-unit = Emekeli
pf-use = Bosaleli
pf-variant = Mbongwana
pf-volume = Buku
pf-zygosity = Zygosité
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n } %
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = maloba { $n } na miniti
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = mbeti { $n } na miniti
pu-litres = { $n } L
pu-coverage = { $n }×
pu-years = mibu { $n }
pu-t-co2e-yr = { $n } t CO₂e na mobu
pv-sensitive-class-health = Bokolongono mpe kondima
pv-sensitive-class-biometrics = Makambo ya nzoto
pv-sensitive-class-genomics = Makambo ya makila (ADN)
pv-sensitive-class-legal = Mikanda ya bosambisi
pv-laterality-left = Ngambo ya mwasi
pv-laterality-right = Ngambo ya mobali
pv-laterality-both = Nyonso mibale
pv-body-region-head = Motó
pv-body-region-face = Elongi
pv-body-region-neck = Nkingo
pv-body-region-left-shoulder = Lipeka ya mwasi
pv-body-region-right-shoulder = Lipeka ya mobali
pv-body-region-left-arm = Loboko ya mwasi
pv-body-region-right-arm = Loboko ya mobali
pv-body-region-left-hand = Liboko ya mwasi
pv-body-region-right-hand = Liboko ya mobali
pv-body-region-chest = Ntolo
pv-body-region-abdomen = Libumu
pv-body-region-upper-back = Mokongo ya likolo
pv-body-region-lower-back = Mokongo ya nse
pv-body-region-pelvis = Loketo
pv-body-region-left-leg = Lokolo ya mwasi
pv-body-region-right-leg = Lokolo ya mobali
pv-body-region-left-foot = Litambe ya mwasi
pv-body-region-right-foot = Litambe ya mobali
pv-body-region-internal = Kati ya nzoto
pv-body-region-whole-body = Nzoto mobimba
pv-body-region-other = Eteni mosusu
pv-artefact-type-mesh = Monyama ya 3D
pv-artefact-type-point-cloud = Lipata ya matono
pv-artefact-type-skin-texture-map = Mapa ya loposo
pv-artefact-type-skeletal-rig = Mikuwa ya numérique
pv-artefact-type-voice-corpus = Lisanga ya mingongo
pv-artefact-type-text-corpus = Lisanga ya makomi
pv-artefact-type-trace-archive = Ebombelo ya misala na internet
pv-artefact-type-behaviour-model = Modele ya bizaleli
pv-artefact-type-fingerprint-card = Karte ya bilembo ya misapi
pv-artefact-type-fingerprint-template = Ndakisa ya elembo ya mosapi
pv-artefact-type-retinal-image = Foto ya retina
pv-artefact-type-voiceprint = Elembo ya mongongo
pv-consent-given = Epesami
pv-consent-given-by-estate = Epesami na bato ya libula
pv-consent-refused = Eboyami
pv-consent-withdrawn = Ezongisami
pv-consent-not-asked = Esengami te
pv-consent-unknown = Eyebani te
pv-sex-at-birth-female = Mwasi
pv-sex-at-birth-male = Mobali
pv-sex-at-birth-intersex = Intersexe
pv-sex-at-birth-undetermined = Eyebani malamu te
pv-sex-at-birth-unknown = Eyebani te
pv-gender-identity-woman = Mwasi
pv-gender-identity-man = Mobali
pv-gender-identity-non-binary = Non-binaire
pv-gender-identity-other = Mosusu
pv-gender-identity-undisclosed = Alobi te
pv-gender-identity-unknown = Eyebani te
pv-title-kind-nobility = Bokonzi ya bankoko
pv-title-kind-academic = Ya boyekoli
pv-title-kind-professional = Ya mosala
pv-title-kind-religious = Ya losambo
pv-title-kind-military = Ya basoda
pv-title-kind-civic = Ya leta to ya lokumu
pv-title-kind-courtesy = Ya limemya
pv-title-kind-other = Mosusu
pv-register-type-birth = Kobotama
pv-register-type-baptism = Batisimo
pv-register-type-marriage = Libala
pv-register-type-death = Kokufa
pv-register-type-burial = Kokunda
pv-register-type-divorce = Koboma libala
pv-register-type-recognition = Kondima mwana
pv-register-type-legitimation = Kokotisa mwana na mibeko
pv-register-type-adoption = Kozwa mwana ya kobokola
pv-register-type-name-change = Kobongola nkombo
pv-register-type-other = Mosusu
pv-build-slight = Moke mpenza
pv-build-slim = Mokonde
pv-build-average = Ya katikati

pv-build-sturdy = Makasi
pv-build-stout = Monene moke
pv-build-heavy = Monene
pv-eye-colour-light-blue = Bule pole
pv-eye-colour-blue = Bule
pv-eye-colour-dark-blue = Bule makasi
pv-eye-colour-grey = Gri
pv-eye-colour-blue-grey = Bule-gri
pv-eye-colour-green = Langi ya mayi ya nzete
pv-eye-colour-grey-green = Gri-mayi ya nzete
pv-eye-colour-hazel = Langi ya noisette
pv-eye-colour-amber = Ambre
pv-eye-colour-light-brown = Marron pole
pv-eye-colour-brown = Marron
pv-eye-colour-dark-brown = Marron makasi
pv-eye-colour-black = Moindo
pv-eye-colour-mixed = Esangani
pv-eye-colour-other = Mosusu
pv-eye-shape-almond = Lokola amande
pv-eye-shape-round = Lingenda
pv-eye-shape-hooded = Lipapu ezipi
pv-eye-shape-monolid = Lipapu moko
pv-eye-shape-deep-set = Ekoti mozindo
pv-eye-shape-protruding = Ebimi libanda
pv-eye-shape-upturned = Etomboki na nsuka
pv-eye-shape-downturned = Ekiti na nsuka
pv-eye-shape-other = Mosusu
pv-eye-spacing-close-set = Pene pene
pv-eye-spacing-average = Ya katikati
pv-eye-spacing-wide-set = Mosika mosika
pv-hair-colour-black = Moindo
pv-hair-colour-dark-brown = Marron makasi
pv-hair-colour-brown = Marron
pv-hair-colour-light-brown = Marron pole
pv-hair-colour-auburn = Marron-motane
pv-hair-colour-red = Motane
pv-hair-colour-strawberry-blond = Blond-motane
pv-hair-colour-dark-blond = Blond makasi
pv-hair-colour-blond = Blond
pv-hair-colour-light-blond = Blond pole
pv-hair-colour-grey = Mpembe-moindo
pv-hair-colour-white = Mpembe
pv-hair-colour-none = Nsuki te
pv-hair-colour-other = Mosusu
pv-hair-texture-straight = Etelemi
pv-hair-texture-wavy = Lokola mbonge
pv-hair-texture-curly = Ezingami
pv-hair-texture-coily = Ezingami makasi
pv-hair-texture-other = Mosusu
pv-hairline-straight = Etelemi
pv-hairline-rounded = Lingenda
pv-hairline-widows-peak = Nsonge na katikati ya mbunzu
pv-hairline-m-shaped = Lolenge ya M
pv-hairline-bell-shaped = Lolenge ya ngonga
pv-hairline-uneven = Ekokani te
pv-hairline-receding = Ezali kozonga sima
pv-hairline-bald = Libandi
pv-facial-hair-none = Eloko te
pv-facial-hair-stubble = Mandefu mike
pv-facial-hair-moustache = Mandefu ya likolo
pv-facial-hair-goatee = Mandefu ya ntaba
pv-facial-hair-full-beard = Mandefu mobimba
pv-facial-hair-sideburns = Mandefu ya matama
pv-facial-hair-other = Mosusu
pv-body-hair-none = Eloko te
pv-body-hair-sparse = Moke
pv-body-hair-moderate = Ya katikati
pv-body-hair-dense = Mingi
pv-skin-tone-type-i = Lolenge I — ezikaka ntango nyonso, ebongwanaka te
pv-skin-tone-type-ii = Lolenge II — ezikaka mingi, ebongwanaka moke
pv-skin-tone-type-iii = Lolenge III — ezikaka mbala mosusu, ebongwanaka malamu
pv-skin-tone-type-iv = Lolenge IV — ezikaka mbala moke, ebongwanaka malamu mpenza
pv-skin-tone-type-v = Lolenge V — ezikaka mbala moke mpenza
pv-skin-tone-type-vi = Lolenge VI — ezikaka soko moke te
pv-skin-undertone-cool = Malili
pv-skin-undertone-neutral = Ya katikati
pv-skin-undertone-warm = Moto
pv-skin-undertone-olive = Olive
pv-freckles-none = Eloko te
pv-freckles-few = Moke
pv-freckles-moderate = Ya katikati
pv-freckles-many = Mingi
pv-pigmentation-mark-birthmark = Elembo ya mbotama
pv-pigmentation-mark-port-wine-stain = Elembo ya langi ya vino
pv-pigmentation-mark-cafe-au-lait-spot = Elembo ya langi ya kafe na miliki
pv-pigmentation-mark-depigmented-patch = Elembo ezangi langi
pv-pigmentation-mark-hyperpigmented-patch = Elembo ya langi makasi
pv-pigmentation-mark-other = Mosusu
pv-mole-shape-round = Lingenda
pv-mole-shape-oval = Lokola likei
pv-mole-shape-irregular = Ezangi lolenge
pv-mole-shape-other = Mosusu
pv-face-shape-oval = Lokola likei
pv-face-shape-round = Lingenda
pv-face-shape-square = Carré
pv-face-shape-oblong = Molai
pv-face-shape-heart = Lolenge ya motema
pv-face-shape-diamond = Lolenge ya diama
pv-face-shape-triangular = Triangle
pv-nose-shape-straight = Etelemi
pv-nose-shape-aquiline = Lokola ya mpongo
pv-nose-shape-snub = Mokuse
pv-nose-shape-upturned = Etomboki
pv-nose-shape-flat = Petee
pv-nose-shape-broad = Monene
pv-nose-shape-bulbous = Lingenda na nsuka
pv-nose-shape-crooked = Egumbami
pv-nose-shape-other = Mosusu
pv-ear-shape-free-lobe = Nsuka ya litoyi ekangami te
pv-ear-shape-attached-lobe = Nsuka ya litoyi ekangami
pv-ear-shape-protruding = Ebimi libanda
pv-ear-shape-close-set = Pene na motó
pv-ear-shape-pointed = Na nsonge
pv-ear-shape-other = Mosusu
pv-lip-shape-thin = Mike
pv-lip-shape-medium = Ya katikati
pv-lip-shape-full = Minene
pv-lip-shape-bow-shaped = Lolenge ya likonga
pv-lip-shape-wide = Milai
pv-lip-shape-downturned = Ekiti na nsuka
pv-lip-shape-other = Mosusu
pv-dentition-primary = Mino ya bomwana
pv-dentition-mixed = Esangani
pv-dentition-permanent-complete = Mino ya libela, mobimba
pv-dentition-permanent-partial-loss = Mino ya libela, mosusu ekweyi
pv-dentition-edentulous = Mino ya mbotama te
pv-dentition-partial-denture = Mino ya bosali ndambo
pv-dentition-full-denture = Mino ya bosali mobimba
pv-dentition-implants = Mino etiami
pv-malocclusion-normal = Mino ekutani malamu
pv-malocclusion-class-i = Lolenge I
pv-malocclusion-class-ii-division-1 = Lolenge II, eteni 1
pv-malocclusion-class-ii-division-2 = Lolenge II, eteni 2
pv-malocclusion-class-iii = Lolenge III
pv-posture-ideal = Malamu mpenza
pv-posture-kyphotic-lordotic = Cyphose-lordose
pv-posture-flat-back = Mokongo petee
pv-posture-sway-back = Mokongo ekoti
pv-posture-stooped = Agumbami
pv-posture-scoliotic = Mokongo egumbami na ngambo
pv-posture-other = Mosusu
pv-gait-brisk = Na lombangu
pv-gait-average = Ya katikati
pv-gait-slow = Malembe
pv-gait-shuffling = Abendaka makolo
pv-gait-limping = Atambolaka na bolema
pv-gait-waddling = Aningananaka
pv-gait-unsteady = Atelemaka malamu te
pv-gait-stiff = Makasi
pv-gait-other = Mosusu
pv-vocal-timbre-bright = Engengi
pv-vocal-timbre-dark = Ezito
pv-vocal-timbre-warm = Ya moto
pv-vocal-timbre-breathy = Na mpema
pv-vocal-timbre-nasal = Na zolo
pv-vocal-timbre-hoarse = Ekauki
pv-vocal-timbre-resonant = Eyokanaka makasi
pv-vocal-timbre-thin = Moke
pv-vocal-timbre-other = Mosusu
pv-speech-register-frozen = Ya milulu
pv-speech-register-formal = Ya lokumu
pv-speech-register-consultative = Ya mosala
pv-speech-register-casual = Ya mokolo na mokolo
pv-speech-register-intimate = Ya bondeko
pv-handedness-left = Loboko ya mwasi
pv-handedness-right = Loboko ya mobali
pv-handedness-ambidextrous = Maboko nyonso mibale
pv-handedness-mixed = Esangani
pv-handedness-unknown = Eyebani te
pv-hearing-grade-normal = Malamu
pv-hearing-grade-mild = Moke
pv-hearing-grade-moderate = Ya katikati
pv-hearing-grade-moderately-severe = Makasi moke
pv-hearing-grade-severe = Makasi
pv-hearing-grade-profound = Makasi mpenza
pv-hearing-grade-complete = Mobimba
pv-optical-correction-none = Eloko te
pv-optical-correction-glasses = Talatala ya miso
pv-optical-correction-contact-lenses = Lentilles ya miso
pv-optical-correction-glasses-and-contact-lenses = Talatala mpe lentilles
pv-optical-correction-refractive-surgery = Kokata miso mpo na komona malamu
pv-optical-correction-intraocular-lens = Lentille kati na liso
pv-optical-correction-other = Mosusu
pv-rhesus-positive = RhD positif
pv-rhesus-negative = RhD négatif
pv-rhesus-weak-d = D ya bolembu
pv-rhesus-unknown = Eyebani te
pv-icd10-chapter-infectious-parasitic = I Bokono ya kopanzana mpe ya banyama
pv-icd10-chapter-neoplasms = II Bampota ya kovimba
pv-icd10-chapter-blood-immune = III Bokono ya makila mpe ya bobateli ya nzoto
pv-icd10-chapter-endocrine-metabolic = IV Bahormone, bilei mpe métabolisme
pv-icd10-chapter-mental-behavioural = V Mikakatano ya makanisi mpe ya bizaleli
pv-icd10-chapter-nervous-system = VI Misisa ya nzoto
pv-icd10-chapter-eye-adnexa = VII Liso mpe biteni na yango
pv-icd10-chapter-ear-mastoid = VIII Litoyi mpe mokuwa ya mastoïde
pv-icd10-chapter-circulatory = IX Botamboli ya makila
pv-icd10-chapter-respiratory = X Kopema
pv-icd10-chapter-digestive = XI Kolamba bilei
pv-icd10-chapter-skin = XII Loposo mpe oyo ezali na nse na yango
pv-icd10-chapter-musculoskeletal = XIII Misuni mpe mikuwa
pv-icd10-chapter-genitourinary = XIV Masuba mpe kobota
pv-icd10-chapter-pregnancy-childbirth = XV Zemi mpe kobota
pv-icd10-chapter-perinatal = XVI Makambo ya ntango ya kobotama
pv-icd10-chapter-congenital = XVII Bolema ya mbotama
pv-icd10-chapter-ill-defined = XVIII Bilembo mpe bantina eyebani malamu te
pv-icd10-chapter-injury-poisoning = XIX Bampota mpe ngenge
pv-icd10-chapter-external-causes = XX Bantina ya libanda
pv-icd10-chapter-health-factors = XXI Makambo etali bokolongono
pv-icd10-chapter-special-purposes = XXII Ba-code mpo na makambo ya sipesiale
pv-diagnosis-status-diagnosed = Eyebani na monganga
pv-diagnosis-status-suspected = Ekanisami
pv-diagnosis-status-self-reported = Ye moko alobi
pv-diagnosis-status-unknown = Eyebani te
pv-prosthesis-kind-limb = Loboko to lokolo
pv-prosthesis-kind-joint = Litonga ya bosali
pv-prosthesis-kind-ocular = Liso
pv-prosthesis-kind-dental = Mino
pv-prosthesis-kind-auditory = Koyoka
pv-prosthesis-kind-breast = Libele
pv-prosthesis-kind-other = Mosusu
pv-implant-kind-orthopaedic = Ya mikuwa
pv-implant-kind-dental = Ya mino
pv-implant-kind-cochlear = Ya koyoka
pv-implant-kind-breast = Libele
pv-implant-kind-intraocular-lens = Lentille kati na liso
pv-implant-kind-contraceptive = Mpo na kopekisa zemi
pv-implant-kind-cosmetic = Ya kitoko
pv-implant-kind-other = Mosusu
pv-device-kind-pacemaker = Masini ya motema
pv-device-kind-implantable-defibrillator = Défibrillateur etiami
pv-device-kind-cardiac-resynchronisation = Masini ya koyokanisa motema
pv-device-kind-ventricular-assist = Masini ya kosalisa motema
pv-device-kind-neurostimulator = Masini ya misisa
pv-device-kind-insulin-pump = Pompe ya insuline
pv-device-kind-drug-port = Ekuke ya kopesa nkisi
pv-device-kind-shunt = Mopepe ya mai
pv-device-kind-stent = Stent
pv-device-kind-other = Mosusu
pv-allergy-type-drug = Nkisi
pv-allergy-type-food = Bilei
pv-allergy-type-environmental = Zingazinga
pv-allergy-type-insect-venom = Ngenge ya banyama ya mike
pv-allergy-type-latex = Latex
pv-allergy-type-other = Mosusu
pv-allergy-severity-mild = Moke
pv-allergy-severity-moderate = Ya katikati
pv-allergy-severity-severe = Makasi
pv-allergy-severity-anaphylactic = Anaphylactique
pv-allergy-severity-unknown = Eyebani te
pv-pathogen-diphtheria = Diphtérie
pv-pathogen-tetanus = Tétanos
pv-pathogen-pertussis = Kokosola ya makasi
pv-pathogen-poliomyelitis = Polio
pv-pathogen-measles = Mbala
pv-pathogen-mumps = Oreillons
pv-pathogen-rubella = Rubéole
pv-pathogen-varicella = Varicelle
pv-pathogen-smallpox = Variole
pv-pathogen-tuberculosis = Tuberculose
pv-pathogen-hepatitis-a = Hépatite A
pv-pathogen-hepatitis-b = Hépatite B
pv-pathogen-hepatitis-c = Hépatite C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae lolenge b
pv-pathogen-pneumococcal = Bokono ya pneumocoque
pv-pathogen-meningococcal = Bokono ya méningocoque
pv-pathogen-human-papillomavirus = Papillomavirus ya bato
pv-pathogen-influenza = Grippe
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotavirus
pv-pathogen-yellow-fever = Fièvre jaune
pv-pathogen-typhoid = Typhoïde
pv-pathogen-cholera = Kolera
pv-pathogen-rabies = Rage
pv-pathogen-japanese-encephalitis = Encéphalite ya Japon
pv-pathogen-tick-borne-encephalitis = Encéphalite ya tiques
pv-pathogen-hiv = VIH
pv-pathogen-syphilis = Syphilis
pv-pathogen-toxoplasmosis = Toxoplasmose
pv-pathogen-cytomegalovirus = Cytomégalovirus
pv-pathogen-epstein-barr = Virus ya Epstein–Barr
pv-pathogen-other = Mosusu
pv-vaccination-status-vaccinated = Azwi mangwele
pv-vaccination-status-partially-vaccinated = Azwi mangwele ndambo
pv-vaccination-status-unvaccinated = Azwi mangwele te
pv-vaccination-status-contraindicated = Apekisami mangwele
pv-vaccination-status-unknown = Eyebani te
pv-serology-result-positive = Positif
pv-serology-result-negative = Négatif
pv-serology-result-equivocal = Ezali polele te
pv-serology-result-unknown = Eyebani te
pv-lab-panel-basic-metabolic = Bomeki ya ebandeli ya métabolisme
pv-lab-panel-lipid = Bomeki ya mafuta ya makila
pv-lab-panel-liver = Bomeki ya libale
pv-lab-panel-renal = Bomeki ya reins
pv-lab-panel-glycated-haemoglobin = Hémoglobine glyquée
pv-lab-panel-iron = Bomeki ya fer
pv-lab-analyte-sodium = Sodium
pv-lab-analyte-potassium = Potassium
pv-lab-analyte-chloride = Chlorure
pv-lab-analyte-bicarbonate = Bicarbonate

pv-lab-analyte-urea = Urée
pv-lab-analyte-creatinine = Créatinine
pv-lab-analyte-glucose = Sukali ya makila
pv-lab-analyte-calcium = Calcium
pv-lab-analyte-total-cholesterol = Cholestérol mobimba
pv-lab-analyte-ldl-cholesterol = Cholestérol LDL
pv-lab-analyte-hdl-cholesterol = Cholestérol HDL
pv-lab-analyte-triglycerides = Triglycérides
pv-lab-analyte-non-hdl-cholesterol = Cholestérol oyo ezali HDL te
pv-lab-analyte-alt = Alanine aminotransférase (ALT)
pv-lab-analyte-ast = Aspartate aminotransférase (AST)
pv-lab-analyte-alp = Phosphatase alcaline (ALP)
pv-lab-analyte-ggt = Gamma-glutamyl transférase (GGT)
pv-lab-analyte-total-bilirubin = Bilirubine mobimba
pv-lab-analyte-direct-bilirubin = Bilirubine directe
pv-lab-analyte-albumin = Albumine
pv-lab-analyte-total-protein = Protéines mobimba
pv-lab-analyte-egfr = GFR ekanisami
pv-lab-analyte-uric-acid = Acide urique
pv-lab-analyte-phosphate = Phosphate
pv-lab-analyte-urine-albumin-creatinine-ratio = Bokokani albumine na créatinine na masuba
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Fer na sérum
pv-lab-analyte-ferritin = Ferritine
pv-lab-analyte-transferrin = Transferrine
pv-lab-analyte-transferrin-saturation = Botondi ya transferrine
pv-lab-analyte-tibc = Makoki mobimba ya kokanga fer
pv-lab-flag-low = Na nse
pv-lab-flag-normal = Malamu
pv-lab-flag-high = Likolo
pv-lab-flag-critical-low = Na nse mpenza, likama
pv-lab-flag-critical-high = Likolo mpenza, likama
pv-nutrient-vitamin-a = Vitamine A
pv-nutrient-thiamine = Thiamine (B1)
pv-nutrient-riboflavin = Riboflavine (B2)
pv-nutrient-niacin = Niacine (B3)
pv-nutrient-vitamin-b6 = Vitamine B6
pv-nutrient-folate = Folate (B9)
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
pv-nutrient-copper = Mbengi
pv-nutrient-potassium = Potassium
pv-nutrient-phosphorus = Phosphore
pv-nutrient-other = Mosusu
pv-sleep-disorder-insomnia = Kozanga mpongi
pv-sleep-disorder-sleep-related-breathing = Mokakatano ya kopema na mpongi
pv-sleep-disorder-central-hypersomnolence = Mpongi koleka ndelo
pv-sleep-disorder-circadian-rhythm = Mokakatano ya ntango ya mpongi
pv-sleep-disorder-parasomnia = Parasomnie
pv-sleep-disorder-sleep-related-movement = Kooningana na mpongi
pv-sleep-disorder-other = Mosusu
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Masolo na monganga
pv-assessment-instrument-other = Mosusu
pv-assessment-severity-none-minimal = Eloko te to moke mpenza
pv-assessment-severity-mild = Moke
pv-assessment-severity-moderate = Ya katikati
pv-assessment-severity-moderately-severe = Makasi moke
pv-assessment-severity-severe = Makasi
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Ba-données ya microarray ya ebandeli
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Mosusu
pv-zygosity-heterozygous = Hétérozygote
pv-zygosity-homozygous = Homozygote
pv-zygosity-hemizygous = Hémizygote
pv-zygosity-compound-heterozygous = Hétérozygote composé
pv-clinical-significance-pathogenic = Ebimisaka bokono
pv-clinical-significance-likely-pathogenic = Ekoki kobimisa bokono
pv-clinical-significance-uncertain-significance = Ntina eyebani malamu te
pv-clinical-significance-likely-benign = Ekoki kozala na likama te
pv-clinical-significance-benign = Ezali na likama te
pv-inheritance-pattern-autosomal-dominant = Autosomique dominant
pv-inheritance-pattern-autosomal-recessive = Autosomique récessif
pv-inheritance-pattern-x-linked-dominant = Dominant na X
pv-inheritance-pattern-x-linked-recessive = Récessif na X
pv-inheritance-pattern-y-linked = Na Y
pv-inheritance-pattern-mitochondrial = Mitochondrial
pv-inheritance-pattern-multifactorial = Bantina mingi
pv-inheritance-pattern-unknown = Eyebani te
pv-carrier-status-affected = Azali na bokono
pv-carrier-status-carrier = Amemi yango
pv-carrier-status-not-carrier = Amemi yango te
pv-carrier-status-unknown = Eyebani te
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Mosusu
pv-metaboliser-status-poor = Asalelaka nkisi malembe mpenza
pv-metaboliser-status-intermediate = Asalelaka nkisi malembe moke
pv-metaboliser-status-normal = Asalelaka nkisi malamu
pv-metaboliser-status-rapid = Asalelaka nkisi na lombangu
pv-metaboliser-status-ultrarapid = Asalelaka nkisi na lombangu mpenza
pv-autopsy-not-performed = Esalemaki te
pv-autopsy-clinical = Ya monganga
pv-autopsy-forensic = Ya bosambisi
pv-autopsy-external-examination = Bomeki ya libanda kaka
pv-autopsy-unknown = Eyebani te
pv-disposition-burial = Kokunda
pv-disposition-cremation = Kotumba nzoto
pv-disposition-entombment = Kotia na ndako ya lilita
pv-disposition-burial-at-sea = Kokunda na mbu
pv-disposition-natural-burial = Kokunda na ndenge ya bozalisi
pv-disposition-body-donation = Epesamaki na siansi
pv-disposition-other = Mosusu
pv-disposition-unknown = Eyebani te
pv-address-use-principal = Esika monene ya kofanda
pv-address-use-secondary = Esika ya mibale ya kofanda
pv-address-use-temporary = Esika ya kofanda mwa ntango
pv-address-use-postal = Adresi ya posita
pv-address-use-other = Mosusu
pv-nationality-mode-descent = Na libota
pv-nationality-mode-birth-in-territory = Na kobotama na mboka
pv-nationality-mode-naturalisation = Na bozwi ya bomboka
pv-nationality-mode-marriage = Na libala
pv-nationality-mode-registration = Na bokomi
pv-nationality-mode-restoration = Na bozongisi
pv-nationality-mode-state-succession = Na mbongwana ya bokonzi ya mboka
pv-nationality-mode-other = Mosusu
pv-language-proficiency-a1 = A1 Ebandeli
pv-language-proficiency-a2 = A2 Moboko
pv-language-proficiency-b1 = B1 Katikati
pv-language-proficiency-b2 = B2 Katikati ya likolo
pv-language-proficiency-c1 = C1 Likolo
pv-language-proficiency-c2 = C2 Boyebi mobimba
pv-language-proficiency-native = Lokota ya liboso
pv-isced-level-isced-0 = 0 Bana mike
pv-isced-level-isced-1 = 1 Kelasi ya ebandeli
pv-isced-level-isced-2 = 2 Kelasi ya katikati ya nse
pv-isced-level-isced-3 = 3 Kelasi ya katikati ya likolo
pv-isced-level-isced-4 = 4 Sima ya kelasi ya katikati, liboso ya iniversite
pv-isced-level-isced-5 = 5 Iniversite ya ntango mokuse
pv-isced-level-isced-6 = 6 Licence to oyo ekokani
pv-isced-level-isced-7 = 7 Master to oyo ekokani
pv-isced-level-isced-8 = 8 Doctorat to oyo ekokani
pv-income-quintile-q1 = Eteni ya mitano ya nse
pv-income-quintile-q2 = Eteni ya mitano ya mibale
pv-income-quintile-q3 = Eteni ya mitano ya katikati
pv-income-quintile-q4 = Eteni ya mitano ya minei
pv-income-quintile-q5 = Eteni ya mitano ya likolo
pv-pay-period-hourly = Na ngonga
pv-pay-period-daily = Na mokolo
pv-pay-period-weekly = Na poso
pv-pay-period-monthly = Na sanza
pv-pay-period-annual = Na mobu
pv-tenure-owned = Ya ye moko
pv-tenure-co-owned = Ya bango elongo
pv-tenure-leasehold = Na kontra ya ntango molai
pv-tenure-rented = Na lifuti ya ndako
pv-tenure-usufruct = Lotomo ya kosalela
pv-tenure-other = Mosusu
pv-distinction-kind-order = Ordre ya lokumu
pv-distinction-kind-decoration = Elembo ya lokumu
pv-distinction-kind-medal = Medaye
pv-distinction-kind-title = Nkombo ya lokumu
pv-distinction-kind-other = Mosusu
pv-military-service-army = Basoda ya mabelé
pv-military-service-navy = Basoda ya mai
pv-military-service-air-force = Basoda ya mpepo
pv-military-service-marines = Basoda ya mbu
pv-military-service-gendarmerie = Gendarmerie
pv-military-service-border-guard = Bakengeli ya ndelo
pv-military-service-national-guard = Garde nationale
pv-military-service-other = Mosusu
pv-rank-category-enlisted = Soda
pv-rank-category-non-commissioned = Sous-officier
pv-rank-category-warrant = Adjudant
pv-rank-category-officer-cadet = Élève-officier
pv-rank-category-junior-officer = Officier ya nse
pv-rank-category-senior-officer = Officier ya likolo
pv-rank-category-general-officer = Jenerale
pv-iccs-section-acts-leading-to-death = 01 Misala oyo ememaki na kokufa
pv-iccs-section-acts-causing-harm = 02 Misala oyo ezokisaki
pv-iccs-section-sexual-acts = 03 Misala ya kozokisa na ndenge ya nzoto
pv-iccs-section-property-with-violence = 04 Na biloko ya bato, na makasi
pv-iccs-section-property-only = 05 Na biloko ya bato kaka
pv-iccs-section-controlled-substances = 06 Biloko epekisami
pv-iccs-section-fraud-deception-corruption = 07 Lokuta, bokosi to kanyaka
pv-iccs-section-public-order-and-state = 08 Na kimya ya bato mpe Leta
pv-iccs-section-public-safety-and-security = 09 Na bokengi ya bato
pv-iccs-section-natural-environment = 10 Na bozalisi
pv-iccs-section-other-criminal-acts = 11 Misala mosusu ya mabe
pv-case-outcome-convicted = Akweaki na likambo
pv-case-outcome-acquitted = Alongi likambo
pv-case-outcome-dismissed = Likambo elongolamaki
pv-case-outcome-conviction-quashed = Etumbu elongolamaki
pv-case-outcome-pardoned = Alimbisamaki
pv-case-outcome-amnestied = Alimbisamaki na amnistie
pv-case-outcome-expunged = Elongolamaki na mikanda
pv-case-outcome-pending = Ezali kozela
pv-case-outcome-unknown = Eyebani te
pv-religion-buddhism = Bouddhisme
pv-religion-christianity-catholic = Boklisto: Katolike
pv-religion-christianity-orthodox = Boklisto: Ortodokse
pv-religion-christianity-protestant = Boklisto: Protestante
pv-religion-christianity-other = Boklisto: mosusu
pv-religion-hinduism = Hindouisme
pv-religion-islam-sunni = Islam: Sunni
pv-religion-islam-shia = Islam: Shia
pv-religion-islam-other = Islam: mosusu
pv-religion-jainism = Jaïnisme
pv-religion-judaism = Boyuda
pv-religion-sikhism = Sikhisme
pv-religion-bahai = Kondima ya Bahá’í
pv-religion-shinto = Shinto
pv-religion-taoism = Taoïsme
pv-religion-zoroastrianism = Zoroastrisme
pv-religion-traditional = Losambo ya bankoko
pv-religion-other = Mosusu
pv-religion-none = Losambo te
pv-religion-unknown = Eyebani te
pv-sacrament-baptism = Batisimo
pv-sacrament-confirmation = Konfirmasio
pv-sacrament-first-communion = Komunio ya liboso
pv-sacrament-reconciliation = Boyokani na Nzambe
pv-sacrament-anointing-of-the-sick = Mafuta ya babeli
pv-sacrament-holy-orders = Bonganga-Nzambe
pv-sacrament-matrimony = Libala
pv-sacrament-other-rite = Molulu mosusu
pv-political-position-far-left = Gauche mpenza
pv-political-position-left = Gauche
pv-political-position-centre-left = Centre-gauche
pv-political-position-centre = Centre
pv-political-position-centre-right = Centre-droite
pv-political-position-right = Droite
pv-political-position-far-right = Droite mpenza
pv-political-position-apolitical = Akotaka na politiki te
pv-political-position-other = Libanda ya molongo oyo
pv-political-position-unknown = Eyebani te
pv-membership-kind-trade-union = Sindika
pv-membership-kind-political-party = Lisanga ya politiki
pv-membership-kind-professional-body = Lisanga ya mosala
pv-membership-kind-religious-order = Lisanga ya bansantu
pv-membership-kind-religious-association = Lisanga ya losambo
pv-membership-kind-fraternal-order = Lisanga ya bondeko
pv-membership-kind-veterans-association = Lisanga ya basoda ya kala
pv-membership-kind-sports-club = Ekipi ya masano
pv-membership-kind-cultural-association = Lisanga ya bonkoko
pv-membership-kind-charitable-association = Lisanga ya bolingo
pv-membership-kind-other = Mosusu
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Epesami na moto oyo ayebaki ye
pv-personality-instrument-inferred = Ekanisami uta na mikanda
pv-personality-instrument-other = Mosusu
pv-introversion-extraversion-strongly-introverted = Kimya mpenza
pv-introversion-extraversion-introverted = Kimya
pv-introversion-extraversion-ambiverted = Katikati
pv-introversion-extraversion-extraverted = Alingaka bato
pv-introversion-extraversion-strongly-extraverted = Alingaka bato mpenza
pv-stress-tolerance-very-low = Na nse mpenza
pv-stress-tolerance-low = Na nse
pv-stress-tolerance-moderate = Ya katikati
pv-stress-tolerance-high = Likolo
pv-stress-tolerance-very-high = Likolo mpenza
pv-decision-style-rational = Na mayele
pv-decision-style-intuitive = Na motema
pv-decision-style-dependent = Na kotalela basusu
pv-decision-style-avoidant = Na kokima
pv-decision-style-spontaneous = Na mbalakaka
pv-sport-level-recreational = Mpo na kosepela
pv-sport-level-amateur-competitive = Amateur ya mabundisi
pv-sport-level-semi-professional = Ndambo ya mosala
pv-sport-level-professional = Mosala
pv-diet-omnivore = Alyaka nyonso
pv-diet-flexitarian = Alyaka mosuni mbala moke
pv-diet-pescatarian = Alyaka mbisi, mosuni te
pv-diet-vegetarian = Alyaka mosuni te
pv-diet-vegan = Alyaka eloko ya nyama te
pv-diet-other = Mosusu

pv-substance-tobacco = Likaya mpe nicotine
pv-substance-alcohol = Masanga
pv-substance-cannabis = Bangi
pv-substance-opioids = Opioïdes
pv-substance-stimulants = Biloko ya kolamusa nzoto
pv-substance-sedatives-hypnotics = Nkisi ya kokitisa motema mpe ya mpongi
pv-substance-hallucinogens = Biloko ya komonisa bilili ya lokuta
pv-substance-inhalants = Biloko ya kopema
pv-substance-gambling = Masano ya mbongo
pv-substance-gaming = Masano ya video
pv-substance-other = Mosusu
pv-use-pattern-occasional-use = Mbala na mbala
pv-use-pattern-regular-use = Ntango nyonso
pv-use-pattern-harmful-use = Bosaleli ya mabe
pv-use-pattern-dependence = Bozali moumbu
pv-use-pattern-in-remission = Azali kobika
pv-lineage-biological = Ya makila
pv-lineage-adoptive = Ya kobokola
pv-lineage-foster = Ya kobatela
pv-lineage-step = Ya libala ya sika
pv-lineage-guardianship = Bokengeli na mibeko
pv-lineage-unknown = Eyebani te
pv-link-relation-godparent = Tata to mama ya batisimo
pv-link-relation-godchild = Mwana ya batisimo
pv-link-relation-witness = Motatoli
pv-link-relation-officiant = Mokambi ya molulu
pv-link-relation-business-partner = Moninga ya mombongo
pv-link-relation-employer = Nkolo mosala
pv-link-relation-employee = Mosali
pv-link-relation-mentor = Molakisi
pv-link-relation-apprentice = Moyekoli ya mosala
pv-link-relation-close-friend = Moninga ya motema
pv-link-relation-neighbour = Mofandi pembeni
pv-link-relation-guardian = Mokengeli
pv-link-relation-ward = Mwana oyo abatelami
pv-link-relation-other = Mosusu
pv-country-AD = Andore
pv-country-AE = Lemila alabo
pv-country-AF = Afiganisita
pv-country-AG = Antiga mpe Barbuda
pv-country-AI = Angiye
pv-country-AL = Alibani
pv-country-AM = Ameni
pv-country-AO = Angola
pv-country-AQ = Antarctique
pv-country-AR = Arizantine
pv-country-AS = Samoa ya Ameriki
pv-country-AT = Otilisi
pv-country-AU = Ositali
pv-country-AW = Aruba
pv-country-AX = Bisanga Eland
pv-country-AZ = Azelebaiza
pv-country-BA = Bosini mpe Hezegovine
pv-country-BB = Barebade
pv-country-BD = Bengalidesi
pv-country-BE = Beleziki
pv-country-BF = Bukina Faso
pv-country-BG = Biligari
pv-country-BH = Bahrene
pv-country-BI = Burundi
pv-country-BJ = Bene
pv-country-BL = Santu Barthelemy
pv-country-BM = Bermuda
pv-country-BN = Brineyi
pv-country-BO = Bolivi
pv-country-BQ = Pays-Bas caribéens
pv-country-BR = Brezile
pv-country-BS = Bahamase
pv-country-BT = Butani
pv-country-BV = Esanga Buve
pv-country-BW = Botswana
pv-country-BY = Byelorisi
pv-country-BZ = Belize
pv-country-CA = Kanada
pv-country-CC = Bisanga Kokos
pv-country-CD = Republiki ya Kongo Demokratiki
pv-country-CF = Repibiki ya Afrika ya Kati
pv-country-CG = Kongo
pv-country-CH = Swise
pv-country-CI = Kotidivuale
pv-country-CK = Bisanga bya Kooke
pv-country-CL = Sili
pv-country-CM = Kamerune
pv-country-CN = Sine
pv-country-CO = Kolombi
pv-country-CR = Kositarika
pv-country-CU = Kiba
pv-country-CV = Bisanga bya Kapevere
pv-country-CW = Curaçao
pv-country-CX = Esanga ya Mbotama
pv-country-CY = Sipele
pv-country-CZ = Shekia
pv-country-DE = Alemani
pv-country-DJ = Dzibuti
pv-country-DK = Danemarike
pv-country-DM = Dominike
pv-country-DO = Repibiki ya Dominike
pv-country-DZ = Alizeri
pv-country-EC = Ekwatele
pv-country-EE = Esitoni
pv-country-EG = Ezipite
pv-country-EH = Sahara ya Limbe
pv-country-ER = Elitele
pv-country-ES = Esipanye
pv-country-ET = Etsiopi
pv-country-FI = Filande
pv-country-FJ = Fidzi
pv-country-FK = Bisanga bya Maluni
pv-country-FM = Mikronezi
pv-country-FO = Bisanga bya Faloe
pv-country-FR = Falanse
pv-country-GA = Gabo
pv-country-GB = Angeletele
pv-country-GD = Gelenade
pv-country-GE = Zorzi
pv-country-GF = Giyane ya Falanse
pv-country-GG = Guernesey
pv-country-GH = Gana
pv-country-GI = Zibatale
pv-country-GL = Gowelande
pv-country-GM = Gambi
pv-country-GN = Gine
pv-country-GP = Gwadelupe
pv-country-GQ = Ginekwatele
pv-country-GR = Geleki
pv-country-GS = Iles de Georgie du Sud et Sandwich du Sud
pv-country-GT = Gwatemala
pv-country-GU = Gwame
pv-country-GW = Ginebisau
pv-country-GY = Giyane
pv-country-HK = Hong Kong (Shina)
pv-country-HM = Ile Heard et Iles McDonald
pv-country-HN = Ondurase
pv-country-HR = Krowasi
pv-country-HT = Ayiti
pv-country-HU = Ongili
pv-country-ID = Indonezi
pv-country-IE = Irelande
pv-country-IL = Isirayele
pv-country-IM = Esanga ya Man
pv-country-IN = Inde
pv-country-IO = Mabele ya Angeletele na mbu ya Indiya
pv-country-IQ = Iraki
pv-country-IR = Ira
pv-country-IS = Isilande
pv-country-IT = Itali
pv-country-JE = Jelezy
pv-country-JM = Zamaiki
pv-country-JO = Zodani
pv-country-JP = Zapo
pv-country-KE = Kenya
pv-country-KG = Kigizisita
pv-country-KH = Kambodza
pv-country-KI = Kiribati
pv-country-KM = Komore
pv-country-KN = Santu kristofe mpe Neves
pv-country-KP = Kore ya nordi
pv-country-KR = Kore ya sudi
pv-country-KW = Koweti
pv-country-KY = Bisanga bya Kayima
pv-country-KZ = Kazakisita
pv-country-LA = Lawosi
pv-country-LB = Liba
pv-country-LC = Santu lisi
pv-country-LI = Lisheteni
pv-country-LK = Sirilanka
pv-country-LR = Liberiya
pv-country-LS = Lesoto
pv-country-LT = Litwani
pv-country-LU = Likisambulu
pv-country-LV = Letoni
pv-country-LY = Libi
pv-country-MA = Maroke
pv-country-MC = Monako
pv-country-MD = Molidavi
pv-country-ME = Montenegro
pv-country-MF = Santu Martin
pv-country-MG = Madagasikari
pv-country-MH = Bisanga bya Marishale
pv-country-MK = Macédoine du Nord
pv-country-ML = Mali
pv-country-MM = Birmanie
pv-country-MN = Mongoli
pv-country-MO = Makau (Shina)
pv-country-MP = Bisanga bya Mariane ya nordi
pv-country-MQ = Martiniki
pv-country-MR = Moritani
pv-country-MS = Mosera
pv-country-MT = Malite
pv-country-MU = Morise
pv-country-MV = Madive
pv-country-MW = Malawi
pv-country-MX = Meksike
pv-country-MY = Malezi
pv-country-MZ = Mozambiki
pv-country-NA = Namibi
pv-country-NC = Kaledoni ya sika
pv-country-NE = Nizere
pv-country-NF = Esanga Norfoke
pv-country-NG = Nizerya
pv-country-NI = Nikaragwa
pv-country-NL = Olande
pv-country-NO = Noriveze
pv-country-NP = Nepale
pv-country-NR = Nauru
pv-country-NU = Nyue
pv-country-NZ = Zelande ya sika
pv-country-OM = Omane
pv-country-PA = Panama
pv-country-PE = Peru
pv-country-PF = Polinezi ya Falanse
pv-country-PG = Papwazi Gine ya sika
pv-country-PH = Filipine
pv-country-PK = Pakisita
pv-country-PL = Poloni
pv-country-PM = Santu peteto mpe Mikelo
pv-country-PN = Pikairni
pv-country-PR = Potoriko
pv-country-PS = Palesine
pv-country-PT = Putulugesi
pv-country-PW = Palau
pv-country-PY = Palagwei
pv-country-QA = Katari
pv-country-RE = Lenyo
pv-country-RO = Romani
pv-country-RS = Serbie
pv-country-RU = Risi
pv-country-RW = Rwanda
pv-country-SA = Alabi Sawudite
pv-country-SB = Bisanga Solomo
pv-country-SC = Seshele
pv-country-SD = Suda
pv-country-SE = Swede
pv-country-SG = Singapure
pv-country-SH = Santu eleni
pv-country-SI = Siloveni
pv-country-SJ = Svalbard mpe Jan Mayen
pv-country-SK = Silovaki
pv-country-SL = Siera Leone
pv-country-SM = Santu Marine
pv-country-SN = Senegale
pv-country-SO = Somali
pv-country-SR = Suriname
pv-country-SS = Sudani ya Sidi
pv-country-ST = Sao Tome mpe Presipe
pv-country-SV = Savadore
pv-country-SX = Sint Maarten
pv-country-SY = Siri
pv-country-SZ = Swazilandi
pv-country-TC = Bisanga bya Turki mpe Kaiko
pv-country-TD = Tsadi
pv-country-TF = Terres australes et antarctiques francaises
pv-country-TG = Togo
pv-country-TH = Tailande
pv-country-TJ = Tazikisita
pv-country-TK = Tokelau
pv-country-TL = Timore ya Moniele
pv-country-TM = Tikemenisita
pv-country-TN = Tinizi
pv-country-TO = Tonga
pv-country-TR = Tiliki
pv-country-TT = Tinidade mpe Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taiwanin
pv-country-TZ = Tanzani
pv-country-UA = Ikreni
pv-country-UG = Uganda
pv-country-UM = Bisanga Mokena Mosika bya Lisanga lya Amerika
pv-country-US = Ameriki
pv-country-UY = Irigwei
pv-country-UZ = Uzibekisita
pv-country-VA = Vatika
pv-country-VC = Santu vesa mpe Gelenadine
pv-country-VE = Venezuela
pv-country-VG = Bisanga bya Vierzi ya Angeletele
pv-country-VI = Bisanga bya Vierzi ya Ameriki
pv-country-VN = Viyetiname
pv-country-VU = Vanuatu
pv-country-WF = Walise mpe Futuna
pv-country-WS = Samoa
pv-country-YE = Yemene
pv-country-YT = Mayote
pv-country-ZA = Afrika ya Sudi
pv-country-ZM = Zambi
pv-country-ZW = Zimbabwe
pv-country-SU = Union soviétique
pv-country-DD = Alemani ya Est
pv-country-YU = Yougoslavie
pv-country-CS = Tchécoslovaquie
pv-country-OT = Bokonzi ya Ottoman
lang-aa = Afar
lang-ab = Abkhaze
lang-ae = Avestique
lang-af = Afrikansi
lang-ak = Akan
lang-am = Liamariki
lang-an = Aragonais
lang-ar = Lialabo
lang-as = Assamais
lang-av = Avar

lang-ay = Aymara
lang-az = Azerbaïdjanais
lang-ba = Bachkir
lang-be = Libyelorisi
lang-bg = Libiligali
lang-bi = Bichelamar
lang-bm = Bambara
lang-bn = Libengali
lang-bo = Tibétain
lang-br = Breton
lang-bs = Bosniaque
lang-ca = Catalan
lang-ce = Tchétchène
lang-ch = Chamorro
lang-co = Corse
lang-cr = Cree
lang-cs = Litsheke
lang-cu = Slavon d’église
lang-cv = Tchouvache
lang-cy = Gallois
lang-da = Danois
lang-de = Lialema
lang-dv = Maldivien
lang-dz = Dzongkha
lang-ee = Éwé
lang-el = Ligeleki
lang-en = Lingelesa
lang-eo = Espéranto
lang-es = Lisipanye
lang-et = Estonien
lang-eu = Basque
lang-fa = Lipelesane
lang-ff = Peul
lang-fi = Finnois
lang-fj = Fidjien
lang-fo = Féroïen
lang-fr = Lifalanse
lang-fy = Frison occidental
lang-ga = Irlandais
lang-gd = Gaélique écossais
lang-gl = Galicien
lang-gn = Guarani
lang-gu = Goudjarati
lang-gv = Mannois
lang-ha = Hausa
lang-he = Liebeleo
lang-hi = Lihindi
lang-ho = Hiri motu
lang-hr = Croate
lang-ht = Créole haïtien
lang-hu = Liongili
lang-hy = Arménien
lang-hz = Héréro
lang-ia = Interlingua
lang-id = Lindonezi
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Yi du Sichuan
lang-ik = Inupiaq
lang-io = Ido
lang-is = Islandais
lang-it = Litaliano
lang-iu = Inuktitut
lang-ja = Lizapo
lang-jv = Lizava
lang-ka = Géorgien
lang-kg = Kikongo
lang-ki = Kikuyu
lang-kj = Kuanyama
lang-kk = Kazakh
lang-kl = Groenlandais
lang-km = Likambodza
lang-kn = Kannada
lang-ko = Likoreya
lang-kr = Kanouri
lang-ks = Cachemiri
lang-ku = Kurde
lang-kv = Komi
lang-kw = Cornique
lang-ky = Kirghize
lang-la = Latina
lang-lb = Luxembourgeois
lang-lg = Ganda
lang-li = Limbourgeois
lang-ln = Lingala
lang-lo = Lao
lang-lt = Lituanien
lang-lu = Kiluba
lang-lv = Letton
lang-mg = Malgache
lang-mh = Marshallais
lang-mi = Maori
lang-mk = Macédonien
lang-ml = Malayalam
lang-mn = Mongol
lang-mr = Marathi
lang-ms = Limalezi
lang-mt = Maltais
lang-my = Libilima
lang-na = Nauruan
lang-nb = Norvégien bokmål
lang-nd = Ndébélé du Nord
lang-ne = Linepale
lang-ng = Ndonga
lang-nl = Lifalama
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
lang-pa = Lipendzabi
lang-pi = Pali
lang-pl = Lipolone
lang-ps = Pachto
lang-pt = Lipulutugesi
lang-qu = Quechua
lang-rm = Liromansh
lang-rn = Roundi
lang-ro = Liromani
lang-ru = Lirisi
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
lang-so = Lisomali
lang-sq = Albanais
lang-sr = Serbe
lang-ss = Swati
lang-st = Sotho du Sud
lang-su = Soundanais
lang-sv = Lisuwede
lang-sw = Kiswahili
lang-ta = Litamuli
lang-te = Télougou
lang-tg = Tadjik
lang-th = Litaye
lang-ti = Tigrigna
lang-tk = Turkmène
lang-tl = Tagalog
lang-tn = Tswana
lang-to = Tongien
lang-tr = Litiliki
lang-ts = Tsonga
lang-tt = Tatar
lang-tw = Twi
lang-ty = Tahitien
lang-ug = Ouïghour
lang-uk = Likreni
lang-ur = Liurdu
lang-uz = Ouzbek
lang-ve = Venda
lang-vi = Liviyetinami
lang-vo = Volapük
lang-wa = Wallon
lang-wo = Wolof
lang-xh = Xhosa
lang-yi = Yiddish
lang-yo = Yoruba
lang-za = Zhuang
lang-zh = Lisinwa
lang-zu = Zulu
person-tab-profile = Bomoto mobimba
profile-groups-label = Bituluku ya bomoto
profile-group-withheld = Ndambo ya etuluku oyo ebombami mpo na yo
profile-withheld = Ekomami mpo na moto oyo mpe ebombami mpo na yo: { $classes }.
profile-empty = Eloko moko te ekomami naino na etuluku oyo.
profile-earlier = lolenge ya kala
profile-earlier-title = Ekomamaki na version ya kala ya programu oyo, na esika oyo AXGF 1.1 ezali na yango te. Ebatelami ndenge ekomamaki.
profile-other-names = { $n ->
        [one] mpe nkombo mosusu moko
       *[other] mpe bankombo mosusu { $n }
    }
profile-edit-group = Bongola { $group }
profile-summary-link = { $n ->
        [one] Likambo moko na bomoto
       *[other] Makambo { $n } na bomoto
    }
profile-from = uta
profile-until = kino
profile-yes = Iyo
profile-no = Te
profile-value = Motuya
profile-editor-title = Bomoto mobimba
profile-group-filled = { $n } etondisami
profile-problems = Ndambo ya oyo okotisaki ekokaki kobombama te. Mokakatano moko na moko etangami pembeni ya esika na yango, mpe eloko moko te ekomami.
profile-editor-withheld = Etuluku oyo ezali mpe na { $classes } mpo na moto oyo, oyo opesami ndingisa ya kotanga te. Emonisami awa te, mpe kobomba formulaire oyo etikaka yango ndenge ezali.
profile-living-class-note = Moto oyo akomami lokola azali na bomoi. Oyo okokotisa awa na etuluku ya sekele emonisamaka kaka epai ya bakambi.
profile-relationships-elsewhere = Baboti, balongani, bana, baboti ya batisimo mpe batatoli babombamaka na moto oyo te. Bazali mabota, boyokani mpe makambo oyo etangi bango — boye mbongwana nyonso awa ebongolaka mpe mokanda ya moto nyonso mosusu oyo azali kati na yango.
profile-documents-first = Eloko ya numérique elakisaka mokanda oyo ekangisami na moto oyo. Kangisa fisyé liboso.
profile-editor-nothing = Eloko moko te na etuluku oyo opesami ndingisa ya kobongola.
profile-new-entry = Eteni ya sika
profile-provenance = Mokolo, liziba mpe bondimi
profile-from-date = Ya solo uta
profile-until-date = Ya solo kino
profile-remove-entry = Longola eteni oyo
profile-add-entry = Bakisa eteni mosusu
profile-no-such-group-title = Etuluku yango ezali te
profile-no-such-group-detail = Bomoto ezali na etuluku ya nkombo wana te.
profile-error-number = Motuya awa ezali motango.
profile-error-integer = Motuya awa ezali motango mobimba.
profile-error-range = Motango moko ezali libanda ya ndelo oyo eteni oyo endimaka.
profile-error-term = Motuya moko ezali moko te kati na maponi oyo etangami.
profile-error-required = Eteni moko ezangi esika oyo esengeli na yango.
profile-error-one-of = Eteni esengeli kozala na esika moko ya ntina na moke etondisami.
profile-error-confidence = Bondimi euti na 0 kino na 1, ndakisa 0.8.
profile-error-time = Ngonga ekomamaka na ngonga mpe miniti, ndakisa 05:40.
profile-error-currency = Mbongo ezali code na yango ya mikanda misato, ndakisa CDF.
profile-error-language = Lokota ezali code na yango, ndakisa ln to zh-Hans.
profile-error-coordinates = Bisika ya mapa esengeli latitude kati na −90 mpe 90 mpe longitude kati na −180 mpe 180.
profile-error-rank-country = Libonza ezali ya mboka mosusu, kasi ya oyo eponami te.
record-unknown-place = [Esika eyebani te]
record-missing-document = [Mokanda ezangi]
confidence-certain = Bondimi { $percent }% — pene na solo
confidence-high = Bondimi { $percent }% — ezali na bilembeteli malamu
confidence-medium = Bondimi { $percent }% — ekoki kozala kasi endimisami te
confidence-low = Bondimi { $percent }% — makanisi kaka
tree-edge-union-between = { $from } mpe { $to } — { $confidence }
tree-edge-parentage-of = { $from }, moboti ya { $to } — { $confidence }
record-note-biography = Lisolo ya bomoi
record-note-birth-date-as-recorded = Mokolo ya kobotama, ndenge ekomamaki
record-note-death-date-as-recorded = Mokolo ya kokufa, ndenge ekomamaki
record-note-event-date-as-recorded = Mokolo ya { $event }, ndenge ekomamaki
record-unknown-source = [Liziba eyebani te]
record-untitled-source = [Liziba ezangi motó]
record-unnamed = [Ezangi nkombo]
record-untitled = [Ezangi motó]
record-period-from = uta { $date }
record-period-until = kino { $date }
record-dates-unrecorded = mikolo ekomami te
record-link-unlabelled = ekangami na
record-link-reverse = { $label } (ya)
record-place-worked-as = Asalaki lokola { $title }
record-place-married-to = Abalaki { $name }
record-place-married = Babalanaki
record-source-use-name = nkombo “{ $name }”
record-source-use-working-as = kosala lokola { $title }
record-source-use-union-with = boyokani na { $name }
record-source-use-union = boyokani
record-lifespan-born = abot. { $year }
record-lifespan-died = akuf. { $year }
size-bytes = { $n ->
        [one] baiti { $n }
       *[other] babaiti { $n }
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = Grégorien
calendar-julian = Julien
calendar-hebrew = Ya Ebrei
calendar-hijri = Hégire
calendar-persian = Ya Perse
calendar-chinese = Ya Shina
calendar-ethiopian = Ya Etiopi
calendar-japanese_era = Ntango ya Japon
calendar-republican_french = Républicain ya Falanse
calendar-roman = Ya Roma
diff-summary-none = esika moko te ebongwani
diff-summary-one = ebongoli { $a }
diff-summary-two = ebongoli { $a } mpe { $b }
diff-summary-many = ebongoli { $a }, { $b } mpe { $n ->
        [one] mosusu moko
       *[other] mosusu { $n }
    }
diff-saved-none = esika moko te ebongwani
diff-saved-one = ebongoli { $a }
diff-saved-two = ebongoli { $a } mpe { $b }
diff-saved-many = ebongoli { $a }, { $b } mpe { $n ->
        [one] mosusu moko
       *[other] mosusu { $n }
    }
diff-section-other = esika mosusu
diff-section-avatar = Foto ya elongi
diff-section-partners = Balongani
diff-section-ai = Lisalisi ya AI
diff-section-birth = Kobotama
diff-section-civil-status = Lolenge ya leta
diff-section-documents = Mikanda ekangisami
diff-section-extensions = Ba-données ya programu
diff-section-children = Bana
diff-section-union = Boyokani
diff-section-date = Mokolo
diff-section-participants = Bato bazalaki
diff-section-from = Ekangami uta

diff-section-to = Ekangami na
diff-section-valid-from = Ezali na motuya uta
diff-section-valid-until = Ezali na motuya kino
diff-section-visibility = Komonana
diff-section-employer = Nkolo mosala
diff-section-title-normalized = Nkombo ya mosala ebongisami
diff-section-conflicts = Maziba oyo ewelanaka
diff-section-dna = ADN
diff-section-document = Mokanda
diff-section-language = Lokota
diff-section-place = Esika
diff-section-repository = Ebombelo ya mikanda
diff-section-script = Makomi
diff-section-coordinates = Bisika ya mapa
diff-section-country-history = Lisolo ya mboka
diff-section-identifiers = Bilembo ya koyeba
diff-section-names = Bankombo
diff-section-file = Fisyé
diff-section-linked-to = Ekangisami na
diff-section-ocr = Makomi oyo eyebanaki
diff-section-format-version = Version ya lolenge
history-created = asali
history-deleted = alongoli
history-merged = asangisi na mokanda mosusu
history-setup = abandisi ebombelo
history-attached = akangisi fisyé
admin-raw-json-unparsed = JSON ya ebandeli etangamaki te ({ $error }). Eloko moko te ebombami.
admin-raw-json-dangling = JSON ya ebandeli elakisi { $field } epai ya { $target }, oyo ezali na ebombelo oyo te. Pona yango na esika oyo ezali likolo, to bongisa elembo. Eloko moko te ebombami.
conflict-someone = Moto moko
conflict-unrecorded-time = na ntango oyo moto moko te akomaki
dedup-merged-persons = { $n ->
        [one] moto moko asangisami
       *[other] bato { $n } basangisami
    }
dedup-merged-families = { $n ->
        [one] libota moko esangisami
       *[other] mabota { $n } esangisami
    }
dedup-manual-review = { $n ->
        [one] likambo moko etikali mpo moto atala yango
       *[other] makambo { $n } etikali mpo moto atala yango
    }
dedup-nothing = Eloko ya koyebisa ezali te.
record-union-duplicate = Balongani mibale, mikanda koleka moko.
record-union-duplicate-detail = Ebombelo ezali na mikanda ya Libota ekeseni mpo na bato oyo mibale. Yango ezali libunga na ba-données, kasi boyokani ya mibale te.
dedup-pair-merged = Mibale oyo otunaki ezali sikoyo mokanda moko.
dedup-pair-refused = Mibale oyo otunaki esangisamaki te: biblioteke eboyaki mpe etikaki yango mpo moto atala.
validate-errors = { $n ->
        [one] libunga moko
       *[other] mabunga { $n }
    }
validate-warnings = { $n ->
        [one] likebisi moko
       *[other] makebisi { $n }
    }
validate-notes = { $n ->
        [one] likomi moko
       *[other] makomi { $n }
    }
validate-nothing = Eloko ya koyebisa ezali te.
list-separator = { ", " }
result-written = Ebombelo ekomamaki na disque.
result-refused = Biblioteke eboyaki mosala oyo. Ebombelo oyo ezali na disque ebongwani te.
convert-error-no-file = Fisyé moko te etindami. Pona liboso fisyé ya .ged.
convert-error-file-too-large = Fisyé wana ezali { $size } MB mpe ndelo ezali { $limit } MB. Eloko moko te ebongolami.
convert-error-too-large = Fisyé oyo etindami eleki ndelo ya { $limit } MB. Eloko moko te ebongolami.
convert-error-unreadable = Fisyé etindami ekokaki kotangama te ({ $error }). Eloko moko te ebongolami.
convert-error-not-gedcom = Wana ezali lokola fisyé ya GEDCOM te: fisyé ya GEDCOM 5.5.1 ebandaka na molongo “0 HEAD”. Eloko moko te ebongolami.
gedcom-read-title = Ndenge fisyé etangamaki
gedcom-read-as = Etangami lokola GEDCOM { $version }, na { $encoding }.
gedcom-read-as-unversioned = Etangami lokola GEDCOM (motó na yango elobi version te), na { $encoding }.
gedcom-read-declared = Motó na yango elobi { $declared }; babaiti na yango elobi ndenge mosusu, mpe etangamaki ndenge ezali.
gedcom-read-gedzip = Ebombelo ya GEDCOM 7 (.gdz): ba-données na yango etangamaki. Bafisyé mosusu kati na yango (bafoto, ba-scan) ememami te: { $n }.
gedcom-read-crs = Milongo oyo esilaka na ndenge ya Macintosh ya kala, ebongisami: { $n }.
gedcom-read-voids = Bilembo epai ya @VOID@ — “mokanda te” ya GEDCOM 7 — etiami pembeni na esika ya kotangama lokola bato: { $n }.
gedcom-read-notes = Makomi oyo ekabolami (SNOTE) etangami lokola makomi: { $n }.
gedcom-read-stray = Babaiti oyo ezalaki UTF-8 te na fisyé oyo ya UTF-8, moko na moko etangami lokola Windows-1252: { $n }.
gedcom-read-tidied = Milongo etiami na ndenge oyo mobongoli atangaka te (ekotisami kati, ba-tabulation to bisika mingi kati na biteni, elembo na mikanda mike, esika liboso ya bokangisi CONC, elembo ya bokambi, elembo ya kalandrie grégorien), ebongisami: { $n }.
gedcom-read-too-deep = Milongo ekotisami mozindo koleka lolenge nyonso ya GEDCOM, etiami pembeni elongo na oyo ezalaki na yango: { $n }.
convert-error-no-dataset = Zip oyo ezali na gedcom.ged te, mpe ezali na fisyé moko ya .ged te mpo na kotanga na esika na yango.
convert-error-refused = Mobongoli eboyaki fisyé oyo. Bomeki oyo ezali na nse elimboli ntina. Eloko moko te ebombami.
convert-error-packaging = Fisyé ebongolamaki kasi ekokaki kokangama te ({ $error }).
completeness-fraction = { $part } na { $whole }
event-category-adoption = Kozwa mwana ya kobokola
event-category-migration = Kokende mboka mosusu
event-category-naturalization = Kozwa bomboka
event-category-incarceration = Boloko
event-category-name_change = Kobongola nkombo
event-category-legal = Likambo ya mibeko
event-category-religious = Likambo ya losambo
event-category-social = Likambo ya bato
event-category-historical = Likambo ya lisolo
precision-quarter_century = kino na eteni ya minei ya ekeke
source-type-birth_certificate = mokanda ya kobotama
source-type-death_certificate = mokanda ya kokufa
source-type-marriage_certificate = mokanda ya libala
source-type-census = botangi bato
source-type-baptism_record = mokanda ya batisimo
source-type-burial_record = mokanda ya kokunda
source-type-will = mokanda ya libula
source-type-land_record = mokanda ya mabelé
source-type-military_record = mokanda ya basoda
source-type-immigration_record = mokanda ya koya na mboka
source-type-naturalization = mokanda ya kozwa bomboka
source-type-passport = passeport
source-type-photograph = foto
source-type-letter = mokanda ya kotinda
source-type-diary = buku ya mokolo na mokolo
source-type-newspaper = zulunalo
source-type-oral_tradition = masolo ya bankoko
source-type-dna = bomeki ya ADN
source-type-family_bible = Biblia ya libota
source-type-gravestone = libanga ya lilita
source-type-published_genealogy = lisolo ya libota ebimisami
source-type-other = liziba mosusu
source-status-verified = etalami na oyo ya ebandeli
source-status-unverified = etalami naino te
source-status-lost = ebungi
source-status-known_missing = eyebani ete ezangi
document-type-birth_certificate = mokanda ya kobotama
document-type-death_certificate = mokanda ya kokufa
document-type-marriage_certificate = mokanda ya libala
document-type-census_page = lokasa ya botangi bato
document-type-baptism_record = mokanda ya batisimo
document-type-military_record = mokanda ya basoda
document-type-will = mokanda ya libula
document-type-land_record = mokanda ya mabelé
document-type-diary = buku ya mokolo na mokolo
document-type-newspaper_clipping = eteni ya zulunalo
document-type-gravestone_photo = foto ya libanga ya lilita
document-type-family_tree_drawing = elilingi ya nzete ya libota
document-type-audio = mongongo ekomami
document-type-video = video ekomami
document-status-present = ebombami awa
document-status-referenced = etangami, ebombami esika mosusu
document-status-known_missing = eyebani ete ezangi
document-status-lost = ebungi
document-status-unknown = esika ezali eyebani te
diag-unsupported_spec_version = Ebombelo elobi version ya AXGF oyo programu oyo ekoki kotanga te.
diag-invalid_json = Eloko oyo esengeli kozala JSON etangami te.
diag-invalid_bundle_structure = Ebombelo ebongisami te ndenge AXGF esengaka.
diag-schema_validation_failed = Mokanda moko eyokani te na schéma ya AXGF.
diag-dangling_reference = Mokanda moko elakisi mokanda mosusu oyo ezali na ebombelo te.
diag-duplicate_entity_id = Mikanda mibale ezali na elembo moko.
diag-duplicate_unique_ref = Mikanda mibale ezali koloba ete ezali na elembo moko oyo esengeli kozala ya moko kaka.
diag-cycle_detected = Boyokani ya libota ezali kobaluka na lingenda: moto akokoma nkoko na ye moko.
diag-chronology_conflict = Mikolo ewelanaka, ndakisa mwana abotami liboso ya moboti.
diag-out_of_vocabulary = Motuya moko ezali moko te kati na maloba oyo molongo na yango endimaka.
diag-claim_inconsistent = Liloba moko ewelanaka na yango moko to na liloba mosusu mpo na likambo yango moko.
diag-spec_version_mismatch = Version ya AXGF oyo mokanda elobi eyokani te na oyo ezali kati na yango.
diag-unknown_attribute = Mokanda ememi eteni oyo AXGF elimboli te.
diag-entity_not_found = Mokanda ya kobongola ezali na ebombelo te.
diag-entity_already_exists = Mokanda na elembo oyo ezali kala.
diag-unknown_entity_kind = Oyo ezali lolenge ya mokanda oyo AXGF ezali na yango te.
diag-delete_blocked_by_reference = Mokanda ekoki kolongolama te soki mikanda mosusu ezali naino kolakisa yango.
diag-manual_review_required = Moto asengeli kotala likambo oyo; ebongolamaki na yango moko te.
diag-zip_read_error = Fisyé ya ebombelo ekokaki kotangama te.
diag-zip_write_error = Fisyé ya ebombelo ekokaki kokomama te.
diag-payloads_external = Bafisyé ekangisami ebombamaka libanda ya ba-données ya ebombelo yango moko.
diag-payload_source_failed = Fisyé moko ekangisami ekokaki kotangama te.
diag-payload_sink_failed = Fisyé moko ekangisami ekokaki kokomama te.
diag-gedcom_parse_error = Molongo moko ya fisyé ya GEDCOM esosolamaki te.
diag-gedcom_unrecognized_tag = Fisyé ya GEDCOM esalelaka elembo oyo mokotisi ayebi te, yango wana eteni yango ememamaki te.
diag-internal = Likambo moko ebebaki kati na biblioteke.
diag-merge_refused = Mikanda mibale ekoki kosangisama te ndenge ezali.
diag-merge_unresolved = Bokeseni kati na mikanda mibale ekatami naino te.
diag-merge_unsupported = Mikanda ya lolenge oyo ekoki kosangisama te.
diag-stale_plan = Moko ya mikanda ebongwani uta mwango esalemaki: sala mwango lisusu.
field-person-display-name = Nkombo ya komonisa
field-person-display-name-hint = Nkombo oyo emonanaka bisika nyonso na site.
field-person-gender = Mobali to mwasi
field-person-living = Azali na bomoi
field-person-birth-date = Mokolo ya kobotama
field-date-value-hint = Mobu, mobu mpe sanza, to mokolo mobimba: 1923, 1923-04 to 1923-04-12. Tika mpamba soki moto moko te ayebi.
field-person-birth-precision = Bosikisiki ya kobotama
field-precision-hint = Ndenge nini liziba elobi yango na bosikisiki.
field-person-birth-circa = Kobotama ezali pene na pene
field-circa-hint = Emonisami lokola “pene na 1923” na esika ya liloba ya solo mpenza.
field-person-birth-place = Elembo ya esika ya kobotama
field-person-birth-confidence = Bondimi ya kobotama
field-person-confidence-hint = Ozali na ntembe te boni. Oyo nde site ebendaka.
field-person-death-date = Mokolo ya kokufa
field-person-death-precision = Bosikisiki ya kokufa
field-person-death-circa = Kokufa ezali pene na pene
field-person-death-place = Elembo ya esika ya kokufa
field-person-death-confidence = Bondimi ya kokufa
field-person-death-cause = Ntina ya kokufa
field-person-bio = Lisolo ya bomoi
field-notes = Makomi
field-family-name = Nkombo ya libota
field-description = Ndimbola
field-family-union-type = Lolenge ya boyokani
field-family-union-status = Ndenge boyokani ezali
field-family-union-confidence = Bondimi ya boyokani
field-family-union-confidence-hint = Ekatisaka ndenge molongo kati na balongani ebendami makasi na nzete.
field-family-union-start = Ebandeli ya boyokani
field-family-union-end = Suka ya boyokani
field-family-notes-hint = Balongani mpe bana bazali milongo — bongola bango na JSON ya ebandeli na nse, to na lokasa ya boyokani ya moto.
field-category = Lolenge
field-required-hint = Esengeli.
field-event-subcategory = Lolenge ya mike
field-date = Mokolo
field-event-date-hint = Schéma esengi yango.
field-precision = Bosikisiki
field-circa = Pene na pene
field-place-id = Elembo ya esika
field-confidence = Bondimi
field-source-id = Elembo ya liziba
field-link-from-type = Lolenge ya ebandeli
field-link-from-id = Elembo ya ebandeli
field-link-to-type = Lolenge ya suka
field-link-to-id = Elembo ya suka
field-link-label = Nkombo
field-link-label-hint = Etangamaka liboso: “tata ya batisimo”, “nkolo mosala”, “motatoli”. Esengeli.
field-link-label-reverse = Nkombo ya nsima
field-link-label-reverse-hint = Ndenge etangamaka uta na suka mosusu: “mwana ya batisimo”, “mosali”.
field-link-bidirectional = Etangamaka ndenge moko na ngambo nyonso mibale
field-valid-from = Ezali na motuya uta
field-link-valid-from-hint = Ntango boyokani ebandaki.
field-valid-until = Ezali na motuya kino
field-link-confidence-hint = “85% ya solo, na ndenge mokanda ya libota elobi” — likambo oyo GEDCOM ekoki koloba te.
field-note = Likomi
field-occupation-person-id = Elembo ya moto
field-occupation-title = Mosala
field-occupation-title-hint = Esengeli, ndakisa Molakisi ya kelasi.
field-occupation-title-latin = Mosala (na makomi ya Latin)
field-occupation-employer = Nkolo mosala
field-occupation-from = Uta
field-occupation-from-hint = Mosala ezali ntango. Kopesa nsuka nyonso mibale nde ebendaka yango lokola molongo.
field-occupation-until = Kino
field-source-title = Motó
field-source-type = Lolenge ya liziba
field-source-reliability = Bondimi ya liziba
field-source-reliability-hint = Esengeli. Emonisami lokola elembo pembeni ya likambo nyonso oyo etelemi na liziba oyo.
field-source-status = Ndenge liziba ezali
field-source-repository = Ebombelo ya mikanda
field-source-repository-reference = Elembo na ebombelo
field-source-transcription = Makomi ya mokanda
field-place-name = Nkombo ya liboso
field-place-name-lang = Lokota ya nkombo
field-place-name-lang-hint = Elembo ya lokota, ndakisa ln, fr to en.
field-place-type = Lolenge ya esika
field-place-region = Etuka
field-place-country-current = Mboka lelo
field-place-country-current-hint = Lisolo ya bandelo na yango ezali molongo — bongola yango na JSON ya ebandeli na nse.
field-document-filename = Nkombo ya fisyé
field-document-mime-type = Lolenge ya média
field-document-mime-type-hint = Esengeli, ndakisa image/jpeg.
field-document-type = Lolenge ya mokanda
field-document-status = Ndenge fisyé ezali
field-document-url = Adresi ya internet
field-document-caption = Maloba ya foto
lang-zh-Hans = Chinois simplifié
family-lineage = Bomoboti
links-relation = Lolenge ya boyokani
occupations-position = Mokumba
field-link-relation = Lolenge ya boyokani
field-link-relation-hint = Moko kati na boyokani oyo AXGF 1.1 etangi. Nkombo oyo ezali likolo ebatelaka maloba ya mokanda yango moko.
field-occupation-position = Mokumba
field-occupation-position-hint = Mokumba kati na mosala: Mokambi ya kelasi, soki mosala ezali Molakisi.
error-delete-changed-title = Ebongwani uta otalaki
error-delete-changed-detail = Mokanda oyo ebombamaki lisusu sima ya kobenda lokasa oyo olongolaki yango, mpe sikoyo ezali version { $version }. Eloko moko te elongolami. Tala yango ndenge ezali liboso ya kozwa mokano lisusu.
error-delete-changed-look = Tala yango lisusu
documents-files = Bafisyé ekangisami awa
documents-files-help = Bongola makambo ya fisyé, to longola fisyé yango moko. Kolongola elongolaka mokanda mpe babaiti na yango na ebombelo, mpo na bato nyonso oyo ekangisami na bango; mpo na kolongola yango kaka na moto oyo, longola molongo na yango likolo.
documents-edit-details = Bongola makambo
documents-delete = Longola fisyé oyo
radar-section = Bilili uta na mokanda
radar-section-help = Ndenge misato ya kotanga oyo mokanda oyo ememi, molongo moko na moko uta na 0 kino na 100. Motango nyonso etangamaka uta na makambo oyo etangami pembeni na yango, na mibeko oyo ekomami na mikanda ya programu; eloko moko te ebombamaka, eloko moko te ekanisamaka, mpe molongo oyo ezangi eloko ya kotanga etikalaka mpamba na esika ya kozwa pwe ya katikati.
radar-physique = Lolenge ya nzoto
radar-mind = Bizaleli mpe makanisi
radar-vitality = Bokolongono mpe makasi
radar-axis-stature = Bolai
radar-axis-build = Lolenge
radar-axis-lean-mass = Kilo ezangi mafuta
radar-axis-posture = Ndenge atelemaka
radar-axis-gait = Ndenge atambolaka
radar-axis-dentition = Mino
radar-axis-openness = Bofungoli ya makanisi
radar-axis-conscientiousness = Bokebi
radar-axis-extraversion = Esengo ya bato
radar-axis-agreeableness = Boboto
radar-axis-stability = Kimya ya motema
radar-axis-cognition = Mayele
radar-axis-circulation = Botamboli ya makila
radar-axis-breathing = Kopema
radar-axis-metabolism = Métabolisme
radar-axis-illness = Kozanga bokono
radar-axis-senses = Mayoki
radar-axis-rest = Mpongi mpe esengo
radar-folded-open = Monisa elilingi oyo
radar-folded-why = Moto oyo akomami lokola azali na bomoi. Elilingi ya bizaleli ya moto ya bomoi etikalaka ekangami kino moto oyo apesami ndingisa ya kotanga yango asenga kotala.
radar-empty = Eloko moko te na mokanda oyo ekoki naino kotangama na elilingi oyo.
radar-table-caption = { $chart }: molongo moko na moko, pwe na yango mpe oyo etangamaki
radar-col-axis = Molongo
radar-col-score = Pwe
radar-col-from = Etangami uta
radar-no-score = pwe te
radar-from-none = eloko te
record-link-outgoing = uta na moto oyo
record-link-incoming = epai ya moto oyo

health-attention = Esengeli kotala:
health-standing-token = Jeton ya mokambi ya likama ezali naino. Ekotaka koleka compte nyonso mpe ndingisa nyonso — yango nde mosala na yango na mokolo oyo moto moko te akoki kokota, kasi eloko ya kotika te sikoyo ete moto akoki kokota. Longola yango na fisyé ya zingazinga ya service mpe banda lisusu.
health-bundle-invalid = { $errors ->
        [one] Ba-données ya libota elongi lisusu bomeki te: libunga moko, etangami na nse ya Bomeki.
       *[other] Ba-données ya libota elongi lisusu bomeki te: mabunga { $errors }, etangami na nse ya Bomeki.
    }
health-disk-unknown = Esika ya mpamba na masini oyo ekokaki kotangama te, yango wana eloko moko awa ekoki kolaka te ete kobomba oyo ekoya ekokoka.
health-disk-no-room-to-save = Kobomba ekoki te: { $free } ezali mpamba, mpe kotonga lisusu ebombelo oyo esengi { $need }. Eloko moko te ebungi, mpe eloko moko te ekoki kobongwana kino esika ezala.
health-disk-critical = Disque ezali mpamba na { $percent }% — { $free } etikali. Kobomba ekotika kosala noki.
health-disk-low = Disque ezali mpamba na { $percent }% — { $free } etikali. Ebongi kotala yango liboso ekoma likambo ya lombangu.
health-backup-unconfigured = Eloko moko te ezali kobombama na kopi. Disque moko soki ebebi ekomema mokanda nyonso.
health-backup-never = Esika ya kobomba bakopi etiami, kasi kopi moko te ekomami kuna naino.
health-backup-stale = { $days ->
        [one] Kopi ya sika koleka ezali na mokolo moko. Esengeli kokoma kopi mokolo na mokolo.
       *[other] Kopi ya sika koleka ezali na mikolo { $days }. Esengeli kokoma kopi mokolo na mokolo.
    }
health-cache-missing = { $missing ->
        [one] Fisyé moko kati na { $declared } ekangisami ezangi na cache mpe ekoki kokitisama te kino kobomba oyo ekoya etonga yango lisusu.
       *[other] Bafisyé { $missing } kati na { $declared } ekangisami ezangi na cache mpe ekoki kokitisama te kino kobomba oyo ekoya etonga yango lisusu.
    }

nav-river = Ebale
nav-home = Libota
river-aria = Ebale zingazinga ya { $name }: bankoko na nse, bakitani likolo.
river-band-living = BAZALI NA BOMOI
river-circa = pene
river-trail = Nzela
river-back = ← Zonga
river-range = Bonene
river-motion = Koningana
river-travel = kotambola
river-cut = kokata
river-legend-attested = endimisami
river-legend-inferred = ekanisami
river-legend-speculative = makanisi
river-legend-lost = mokanda esili
river-legend-repeat = ×n akomami na milongo n, abendami na nse ya moko na moko
river-legend-again = ↔ abendami lisusu pembeni ya molongani ya sima
river-legend-discharge = bakitani
river-legend-direction = ↓ epai ya liziba · ↑ epai ya bato ya bomoi
view-tree-direction = ↓ misisa: bankoko · ↑ motó ya nzete: bakitani
view-galaxy-direction = ↓ bankoko na nse ya moi · ↑ bakitani likolo
view-summit-direction = ↓ bankoko na lobwaku · ↑ bakitani bazali komata epai ya nsonge
river-legend-generation = nkola moko ≈ mibu 29
river-zoom = Zoom
river-still = Tikala esika moko
view-tree-still = Tika nzete kimya
view-galaxy-still = Tika likolo kimya
view-summit-still = Tika ngomba kimya
river-whole = Libota mobimba
river-names-all = Bankombo nyonso
# A tap on a person in a view, on a touch screen: the drawing moves onto them.
river-pick-centre = Tia awa na katikati
river-zoom-hint = Zoom: Ctrl + roulette, kokanga na misapi, to + / − · soki ozali na zoom, benda mpo na koningisa
river-tab-sources = Maziba
river-legend-title = Ndenge ya kotanga ebale
view-tree-name = Nzete ya bomoi
view-tree-legend = Ndenge ya kotanga nzete
view-galaxy-name = Galaxie
view-galaxy-legend = Ndenge ya kotanga galaxie
view-summit-name = Komata
view-summit-legend = Ndenge ya kotanga ngomba
hud-ancestors = Bankoko
hud-descendants = Bakitani
hud-generations = Bankola
hud-living = Bazali na bomoi
hud-attested = Boyokani endimisami

search-label = Luka moto
search-short = Moto
date-qualifier = Elembo
date-qualifier-circa = pene na
search-title = Luka
search-heading = Bato oyo bayokani na “{ $q }”
search-none = Moto moko te oyo okoki komona abengami bongo.
search-open-record = Fungola mokanda
search-hint = Enter etambolaka na ebale kino epai na bango; Shift+Enter efungolaka mokanda na bango.
view-tree-search-hint = Enter ekolisaka nzete zingazinga na bango; Shift+Enter efungolaka mokanda na bango.
view-galaxy-search-hint = Enter ekomisaka bango moi; Shift+Enter efungolaka mokanda na bango.
view-summit-search-hint = Enter ememaka yo epai na bango; Shift+Enter efungolaka mokanda na bango.
search-summary = { $n ->
        [one] moto { $n }
       *[other] bato { $n }
    } · ↵ tambola · ⇧↵ fungola mokanda
search-empty-query = Koma nkombo mpo na koluka.

nav-quality = Bolamu ya ba-données
quality-title = Makambo ya kotala
quality-none = Eloko ya kotala ezali te: bomeki nyonso elongi.
quality-intro = Eloko moko awa ebongwanaka te na esika na yo. Molongo moko na moko ezali ndenge ya kotala — mbala mingi ezali na ntina malamu. Fungola mokanda mpe zwa mokano, to tia elembo ete etalami.
quality-groups-label = Mitindo ya makambo emonani
quality-all = Nyonso
quality-show-label = Monisa
quality-show-open-option = Ya kotala
quality-download = Kitisa
quality-group-names = Bankombo
quality-group-relations = Boyokani
quality-group-dates = Mikolo
quality-group-places = Bisika
quality-group-evidence = Bilembeteli
quality-export-csv = Kitisa lokola CSV
quality-export-json = Kitisa lokola JSON
quality-open = Fungola
quality-none-file = Eloko ya kotala ezali te na fisyé oyo.
quality-show-list = Monisa molongo
quality-after-import = Eloko moko te ebongolamaki na fisyé. Soki ekomi ebombelo ya site, lokasa ya bolamu ya ba-données ekotanga makambo yango moko na lien ya mokanda moko na moko.
quality-file-list-cut = Kaka { $shown } ya liboso etangami awa. Soki fisyé ekomi ebombelo ya site, lokasa ya bolamu ya ba-données ekotanga yango nyonso.
quality-rule-name-sentence = Fraze na esika ya nkombo
quality-name-sentence = “{ $value }” ezali lokola fraze koleka nkombo: soki ezali ndimbola, tika nkombo na esika na yango mpe tia oyo etikali na likomi.
quality-rule-union-parent-child = Balongani oyo bazali mpe moboti na mwana
quality-union-parent-child = { $parent } mpe { $child } bamonani lokola balongani mpe lokola moboti na mwana: ebongi kotala boyokani nini maziba endimisaka.
quality-rule-child-listed-twice = Mwana atangami mbala mibale
quality-child-listed-twice = { $child } amonani mbala mibale kati na bana ya libota oyo: soki ezali mwana yango moko, eteni moko ekoki kolongolama.
quality-rule-duplicate-union = Balongani yango moko bakomami mbala mibale
quality-duplicate-union = { $a } mpe { $b } bazali na boyokani { $count } ekomami: soki babalanaki mbala moko kaka, ekoki kozala boyokani moko ekotisami mbala mibale.
quality-rule-partner-twice = Moto moko na ngambo nyonso mibale ya boyokani
quality-partner-twice = { $person } amonani na ngambo nyonso mibale ya boyokani oyo: ebongi kotala nani azalaki molongani mosusu, to kotika ngambo wana mpamba soki moto moko te ayebi.
quality-rule-foreign-surnames = Nkombo ya libota ya mwana mobali oyo eyokani na ya moboti moko te
quality-foreign-surnames = Nkombo ya libota oyo eyokani na ya moboti moko te: { $who }. Yango ekoki kozala malamu — kobongola nkombo, mwana ya libala ya sika, makomi ekeseni — kasi ebongi kotala. (Bana basi batangami te, mpo nkombo ya libala ezali ntina oyo emonanaka mingi.)
quality-rule-death-before-birth = Kokufa liboso ya kobotama
quality-death-before-birth = { $person } azali na kokufa na { $death }, liboso ya kobotama na ye na { $birth }: moko kati na mikolo mibale ekoki kozala ekomami mabe; ebongi kotala yango na maziba.
quality-rule-child-before-parent = Mwana abotami liboso ya moboti
quality-child-before-parent = { $child } (abotama { $childborn }) amonani lokola abotamaki liboso ya moboti na ye { $parent } (abotama { $parentborn }): ebongi kotala mikolo to bomoboti.
quality-rule-parent-too-young = Moboti oyo azali na mibu na nse ya zomi na mibale
quality-parent-too-young = { $parent } alingaki kozala na mibu { $age } ntango { $child } abotamaki, likambo oyo emonanaka mingi te: ebongi kotala mikolo to bomoboti.
quality-rule-parent-too-old = Moboti oyo aleki mibu ntuku motoba na mitano
quality-parent-too-old = { $parent } alingaki kozala na mibu { $age } ntango { $child } abotamaki, likambo oyo emonanaka mingi te: ebongi kotala mikolo to bomoboti.
quality-rule-marriage-before-birth = Libala liboso ya kobotama ya molongani
quality-marriage-before-birth = Libala oyo ezali na mokolo { $married }, liboso { $person } abotama na { $born }: ebongi kotala mokolo ya libala.
quality-rule-isolated-record = Mokanda ezangi mokolo mpe boyokani
quality-isolated-record = { $person } azali na mokolo te mpe na boyokani te na moto moko na libota: ebongi kobakisa ndenge azali na libota, to kolongola mokanda soki ekotisamaki na libunga.
quality-rule-place-no-region = Esika ezangi etuka
quality-place-no-region = { $place } ezali na etuka te: kobakisa moko esalisaka kokesenisa bisika oyo ezali na nkombo moko.
quality-rule-region-free-text = Etuka oyo etangamaka lokola likomi
quality-region-free-text = Etuka ya { $place } elobi “{ $value }”, oyo ezali lokola likomi koleka etuka: ebongi kotia likomi yango na mokanda oyo ezali kolimbola.
quality-rule-place-near-duplicate = Bisika oyo ekoki kozala esika moko
quality-place-near-duplicate = { $names } ekoki kozala esika moko ekomami na ndenge ekeseni: soki ezali bongo, ekoki kosangisama na moko.
quality-rule-not-a-place = Eteni oyo ekoki kozala esika te
quality-not-a-place = “{ $place }” ezali na molongo ya bisika kasi ekoki kozala esika te — ebandi na lilembo moke, oyo bankombo ya bisika esalaka mingi te: ebongi kotala soki ezali likomi mpo na esika moko.
quality-rule-certain-without-source = Ya solo, kozanga liziba
quality-certain-without-source = { $record } ezali na liloba oyo etiami elembo ya solo mobimba ({ $claims }) kozanga liziba ekangisami: ebongi kokangisa liziba, to kokitisa bondimi.
quality-dismiss = Etalami — ezali malamu epai na biso
quality-restore = Monisa yango lisusu
quality-note-label = Mpo na nini (ezali na makasi te)
quality-show-dismissed = Oyo etalami mpe ebatelami
quality-show-open = Zonga na molongo ya kotala
quality-dismissed-by = Etalami na { $by } na { $at }
quality-gone-title = Likambo yango ezali lisusu te
quality-gone-detail = Mokanda ebongwanaki uta molongo efungwamaki, yango wana likambo etangamaka lisusu ndenge moko te. Tia molongo lisusu mpe tala lisusu.
quality-summary = { $n ->
        [one] Likambo moko ya kotala
       *[other] Makambo { $n } ya kotala
    }
quality-summary-file = { $n ->
        [one] Likambo moko ya kotala na fisyé oyo
       *[other] Makambo { $n } ya kotala na fisyé oyo
    }
quality-dismissed-summary = { $n ->
        [one] Likambo moko etalami mpe ebatelami
       *[other] Makambo { $n } etalami mpe ebatelami
    }
quality-dismiss-all = Tia elembo na nyonso ete etalami ({ $n })
quality-restore-all = Monisa nyonso lisusu ({ $n })
quality-bulk-conflict = Mikanda oyo ebongwanaki uta molongo efungwamaki: { $n }. Eloko moko te etiami elembo; tia molongo lisusu mpe meka lisusu.

edit-tab-changed = ebongolami, ebombami naino te
edit-remove = Longola
edit-add-partner = Bakisa molongani
edit-add-child = Bakisa mwana
edit-union-with = Boyokani na { $names }
edit-union-alone = Boyokani kozanga molongani akomami
edit-child-of = Mwana ya { $names }
edit-new-family = Banda libota
edit-cite-title = Oyo etelemi na liziba
edit-attach-to = Tanga yango mpo na
edit-add-source = Bakisa liziba
edit-error-source-title = Liziba ya sika esengeli kozala na motó. Eloko moko te ebombami.
edit-detach-file = Kabola na mokanda oyo
edit-no-sources = Likambo moko te ya moto oyo etangi naino liziba.
edit-no-files = Fisyé moko te ekangisami naino na mokanda oyo.
edit-saved = { $n ->
        [one] Ebombami: mokanda moko ekomami.
       *[other] Ebombami: mikanda { $n } ekomami na kobomba moko.
    }
edit-saved-none = Eloko moko te ebongwanaki, yango wana eloko moko te ebombami.
edit-back-to-record = Zonga na mokanda
edit-avatar = Foto ya profil
edit-avatar-frame = Tia elongi na katikati
edit-avatar-upload = Salela lokola foto ya profil
edit-stale = Moto mosusu abongoli { $what } uta lokasa oyo efungwamaki. Eloko moko te ebombami; nyonso oyo okotisaki ezali naino awa.
edit-refused = Eloko moko te ebombami: { $reason }

error-cross-site-title = Eboyami: etindami uta site mosusu
error-cross-site-detail = Formulaire oyo euti na lokasa ya site mosusu, yango wana esalemaki te. Soki otindaki yango uta site oyo, tia lokasa lisusu mpe tinda yango lisusu.
admin-nav-label = Biteni ya bokambi
admin-nav-overview = Botali ya mobimba
admin-nav-bundle = Ebombelo AXGF
admin-nav-infrastructure = Bisaleli
admin-nav-accounts = Ba-compte
admin-nav-connectivity = Bokangisi

bundle-title = Ebombelo AXGF
bundle-lede = Ebombelo ya libota ndenge site oyo esimbi yango: { $path }. Na nini eyokani, nini emonani mabe, mpe nzela mibale ya kozwa kopi.
bundle-unset-title = Libota ezali naino te.
bundle-unset-detail = Installation oyo ezali na ebombelo mpamba, yango wana eloko ya kotala ezali naino te. Bandisa yango uta na ebombelo, uta na fisyé ya GEDCOM to uta na eloko te.
bundle-kpi-conformity = Boyokani na mibeko
bundle-verdict-ok = Eyokani
bundle-verdict-warn = Ya kotala
bundle-verdict-fail = Eyokani te
findings-tally = Langi ya mayi ya nzete: { $ok } · Makebisi: { $warn } · Likama: { $fail }
bundle-kpi-entities = Biteni
bundle-kpi-entities-note = { $persons ->
        [one] moto 1
       *[other] bato { $persons }
    } · { $files ->
        [one] fisyé 1 ekangisami
       *[other] bafisyé { $files } ekangisami
    }
bundle-kpi-size = Bonene na disque
bundle-kpi-members = { $n ->
        [one] fisyé 1 na ebombelo
       *[other] bafisyé { $n } na ebombelo
    }
bundle-kpi-format = Lolenge
bundle-kpi-format-note = Programu oyo etangaka mpe ekomaka AXGF { $current }.
bundle-conformity = Boyokani na mibeko
bundle-conformity-intro = Bomeki libwa, etangami uta na ebombelo ndenge ezali sikoyo; moko te ebongolaka yango. Fungola molongo mpo na komona nini ya kosala.
bundle-area-format = Lolenge
bundle-area-integrity = Bosembo ya ba-données
bundle-area-privacy = Sekele
bundle-area-content = Biloko ezali kati
level-ok = Malamu
level-warn = Likebisi
level-fail = Likama
findings-what-to-do = Nini ya kosala
findings-go = Kende kuna →
bundle-anomalies = Makambo ya mabe
bundle-anomalies-intro = Oyo lokasa ya bolamu ya ba-données emonaka, mobeko na mobeko: ezali naino ya kotala, mpe etiami kala elembo ete etalami.
bundle-anomalies-rule = Mobeko
bundle-anomalies-open = Ya kotala
bundle-anomalies-checked = Etalami
bundle-anomalies-none = Lokasa ya bolamu ya ba-données emoni eloko ya kotala te.
bundle-diagnostics = Bomeki ya mosali-bomeki
bundle-facts = Makambo ya mike
bundle-fact-path = Fisyé
bundle-fact-size = Bonene
bundle-fact-spec = Lolenge
bundle-fact-schema = Schéma ememami
bundle-fact-none = eloko te
bundle-fact-created = Esalemaki
bundle-fact-updated = Ekomamaki mbala ya suka
bundle-fact-generator = Ekomamaki na
bundle-fact-gedcom = Ebongolamaki uta
bundle-export = Bimisa
bundle-export-raw = Ebombelo ndenge ezali (.axgf)
bundle-export-raw-go = Kitisa ebombelo
bundle-export-sealed = Ebombami na sekele mpo na moto moko (.axgf.gpg)
bundle-export-sealed-intro = Ebombelo yango moko, lokola nsango ya OpenPGP oyo kaka moto oyo azali na fungola ya sekele akoki kofungola. Tia fungola na ye ya polele: makomi oyo ebandaka na “-----BEGIN PGP PUBLIC KEY BLOCK-----”.
bundle-export-sealed-go = Kitisa na sekele
seal-recipient = Mpo na
seal-fingerprint = Elembo ya mosapi
seal-algorithm = Ba-algorithme
seal-fips-yes = kati na etuluku endimami na FIPS
seal-fips-no = libanda ya etuluku endimami na FIPS
seal-expires = Fungola esilaka
seal-never = soko moke te
seal-key-label = Fungola ya polele ya moto oyo akozwa
seal-key-hint = Fungola RSA ya babit 2048 na moke, to fungola ya courbe elliptique. Tala elembo ya mosapi elongo na nkolo na yango liboso ya kotinda eloko.
seal-remember = Batela fungola oyo mpo na mbala ekoya
seal-check = Tanga kaka fungola
seal-fips-note = Ebombelo ebombamaka na sekele na AES-256 mpo na fungola ya moto oyo akozwa. Ba-algorithme endimami na FIPS ezali RSA ya babit 3072 to koleka mpe ba-courbe ya NIST P-256, P-384 mpe P-521: senga P-384. Biblioteke oyo esalelami awa ezali yango moko module endimisami na FIPS 140-3 te.
seal-decrypt-note = Moto oyo akozwa afungolaka yango na GnuPG:
seal-forget = Bosana fungola ebatelami
seal-key-kept = Fungola { $fingerprint } ebatelami mpo na bobimisi ya sekele oyo ekoya.
seal-error-no-key = Tia liboso fungola ya polele ya moto oyo akozwa.
seal-error-not-a-key = Wana ezali fungola ya polele ya OpenPGP te. Tia eteni mobimba, uta na molongo BEGIN kino na molongo END, mpe soko moke te fungola ya sekele.
seal-error-bad-signatures = Ba-signature ya fungola wana yango moko endimisami te: ekoki kozala ebongolami. Senga nkolo na yango kopi ya sika.
seal-error-expired = Fungola wana, to fungola nyonso kati na yango oyo ekoki kobomba na sekele, esili ntango. Senga nkolo na yango abakisa ntango to atinda ya sika.
seal-error-revoked = Nkolo na yango alongoli fungola wana. Senga ye fungola na ye ya sika.
seal-error-no-encryption-key = Fungola wana ekoki kotia signature kasi kobomba na sekele te: ezali na fungola-moke ya sekele te. Senga nkolo na yango moko (gpg --quick-add-key … encr).
seal-error-too-weak = Fungola RSA wana ezali na babit { $bits }; bobimisi esengi { $min } na moke.
seal-error-unsupported = Algorithme ya fungola wana ezali te oyo bobimisi esalelaka: kaka bafungola RSA to ya courbe elliptique.
seal-error-io = Kopi ya sekele ekokaki kosalema te: { $error }
date-time-utc = { $date }, { $time } UTC

bundle-check-format-title = Version ya lolenge
bundle-check-format-ok = AXGF { $version }, version oyo programu oyo etangaka mpe ekomaka.
bundle-check-format-missing = Manifeste etangi version moko te ya AXGF.
bundle-check-format-missing-fix = Ebombelo ekomamaki na esaleli ya AXGF te, to ebebaki. Zongisa kopi ya suka oyo elongaki bomeki (axgf-cms restore).

bundle-check-format-older = Ekomami lokola AXGF { $version }; kobomba oyo ekoya ekokoma yango lokola AXGF { $current }.
bundle-check-format-older-fix = Eloko ya kosala te: bobongoli oyo ekoya ekokoma yango lisusu. Batela kopi ya fisyé ya kala soki programu mosusu etangaka naino kaka { $version }.
bundle-check-format-newer = Ekomami lokola AXGF { $version }, ya sika koleka { $current } ya programu oyo: kobomba ekoki kobungisa oyo programu oyo esosolaka te.
bundle-check-format-newer-fix = Mata axgf-cms na version oyo etangaka AXGF { $version } liboso moto moko abongola (Bisaleli, Version).
bundle-check-manifest-title = Mitango ya manifeste
bundle-check-manifest-ok = Mitango ya manifeste eyokani na oyo ebombelo ezali na yango.
bundle-check-manifest-drift = Manifeste elobi { $declared } kasi ebombelo ezali na { $actual } ({ $collection }).
bundle-check-manifest-fix = Eloko moko te ebungi: mitango ekokomama lisusu na kobomba oyo ekoya. Soki ebombelo oyo euti esika mosusu, tala soki ekopiamaki mobimba.
bundle-check-validation-title = Bomeki
bundle-check-validation-ok = Mosali-bomeki ya biblioteke amoni libunga moko te.
bundle-check-validation-errors = { $errors ->
        [one] libunga 1
       *[other] mabunga { $errors }
    }: ebombelo elongi bomeki te.
bundle-check-validation-errors-fix = Bomeki moko na moko na nse etangi mokanda mpe esika. Bongisa mokanda, to zongisa kopi ya suka oyo elongaki bomeki.
bundle-check-validation-warnings = { $warnings ->
        [one] likebisi 1
       *[other] makebisi { $warnings }
    }. Epekisaka kobomba soko moke te; moko na moko elobi ete eloko moko ekoki kozala malamu te.
bundle-check-validation-warnings-fix = Tanga yango na nse. Mingi ezali kolakisa mikanda oyo ezali na ebombelo oyo te, likambo oyo bokotisi ya GEDCOM etikaka mbala mingi.
bundle-check-payloads-title = Bafisyé ekangisami
bundle-check-payloads-none = Fisyé moko te ekangisami.
bundle-check-payloads-ok = { $declared ->
        [one] Fisyé moko yango ezali.
       *[other] Bafisyé nyonso { $declared } ezali.
    }
bundle-check-payloads-missing = Bafisyé { $missing } kati na { $declared } ekangisami ezangi na cache.
bundle-check-payloads-fix = Kobomba oyo ekoya etongaka lisusu cache uta na ebombelo. Soki ezangi na ebombelo yango moko, zongisa yango uta na kopi.
bundle-check-binding-title = Ba-compte mpe ebombelo
bundle-check-binding-ok = Fisyé ya ba-compte esalemaki mpo na libota oyo.
bundle-check-binding-mismatch = Fisyé ya ba-compte esalemaki mpo na { $expected }; ebombelo oyo ezali { $found }.
bundle-check-binding-fix = Soki ebombelo ezongisamaki to ebongolamaki nkombo na mokano, eloko ya kosala te. Soki te, ba-compte ya libota moko ezali kosala na nzete ya libota mosusu: zongisa fisyé .acl ya malamu pembeni na yango.
bundle-check-living-title = Bato ya bomoi, na polele
bundle-check-living-ok = Moto moko te oyo akomami lokola azali na bomoi amonanaka na bapaya.
bundle-check-living-public = { $n ->
        [one] Moto 1 akomami lokola azali na bomoi atiami
       *[other] Bato { $n } bakomami lokola bazali na bomoi batiami
    } na polele: mopaya nyonso atangaka mokanda na bango.
bundle-check-living-fix = Soki bandimaki yango te, tia komonana na bango mpo na bandeko ya lisanga (Bongola, Bomoto).
bundle-check-special-title = Mitindo ya sipesiale
bundle-check-special-none = Mokanda moko te ezali na ba-données ya bokolongono, ya nzoto, ya makila (ADN), ya bosambisi to ya bizaleli.
bundle-check-special-some = { $n ->
        [one] Mokanda 1 ezali na
       *[other] Mikanda { $n } ezali na
    } ba-données ya bokolongono, ya nzoto, ya makila (ADN), ya bosambisi to ya bizaleli. Mpo na bato ya bomoi etikalaka epai ya bakambi, ata mokanda elobi nini.
bundle-check-quality-title = Bolamu ya ba-données
bundle-check-quality-ok = Eloko ya kotala te.
bundle-check-quality-open = { $n ->
        [one] Likambo 1 ya kotala
       *[other] Makambo { $n } ya kotala
    }, na { $groups ->
        [one] etuluku 1
       *[other] bituluku { $groups }
    }.
bundle-check-quality-fix = Fungola lokasa ya bolamu ya ba-données: molongo moko na moko etangi mokanda mpe nini emonani mabe. Bongisa yango, to tia elembo ete etalami.
bundle-check-size-title = Bonene
bundle-check-size-ok = { $size }, na nse ya { $warn } oyo sima na yango lokasa oyo ekebisaka.
bundle-check-size-heavy = { $size }, eleki { $warn }.
bundle-check-size-fix = Ebombelo ezali ya libota mpo na kokolisa yango. Tala disque mpe bakopi (Bisaleli, Ebombelo), mpe tombola --size-warn-mb soki bonene oyo ezali oyo ezelamaki.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

infra-area-security = Bokengi
infra-area-access = Bokoti
infra-area-data = Ba-données
infra-area-capacity = Makoki
infra-area-release = Version
infra-area-platform = Esika ya mosala
infra-verdict-ok = Malamu
infra-verdict-warn = Esengeli kotala
infra-verdict-fail = Likama
duration-days = { $n ->
        [one] mokolo { $n }
       *[other] mikolo { $n }
    }
duration-hours = { $n ->
        [one] ngonga { $n }
       *[other] ngonga { $n }
    }
duration-minutes = { $n ->
        [one] miniti { $n }
       *[other] miniti { $n }
    }
infra-transport-title = Ndenge bato bakomaka na site
infra-transport-ok = Lokasa oyo eyaki na nzela ya HTTPS.
infra-transport-exposed = Lokasa oyo eyaki kozanga sekele, mpe serveur ezali koyoka na { $bind }, libanda ya masini oyo: ba-mot de passe mpe ba-cookie ya session ezali koleka na réseau na polele.
infra-transport-exposed-fix = Tia proxy ya HTTPS (Caddy, nginx, HAProxy) liboso na yango, mpe kangisa lisusu service na 127.0.0.1: banda lisusu bootstrap.sh na --bind 127.0.0.1:8080, to longola fisyé ya drop-in oyo efungolaki yango.
infra-transport-plain = Lokasa oyo eyaki kozanga sekele, uta na serveur oyo ezali koyoka na { $bind }. Yango ezali malamu kaka na masini oyo, mpe kaka kuna.
infra-transport-plain-fix = Kende na site na nzela ya adresi na yango ya HTTPS. Soki proxy esukisaka TLS, sala ete etinda X-Forwarded-Proto: https mpo serveur oyo eyeba.
infra-forwarded-title = Ba-adresi oyo etindami
infra-forwarded-ok = Serveur ezali koyoka kaka na masini oyo, yango wana adresi ya client oyo proxy etindaka ekoki kondimama mpo na kokitisa lombangu ya bomeki ya kokota oyo elongi te mbala na mbala.
infra-forwarded-exposed = Bato bakoki kokoma na serveur mbala moko, mpe endimaka en-tête X-Forwarded-For: client oyo atiaka en-tête yango ye moko akoki kokima bokitisi ya lombangu sima ya bomeki ya kokota elongi te.
infra-forwarded-fix = Kangisa service na 127.0.0.1 nsima ya proxy, to kanga port na pare-feu mpo kaka proxy ekoma na yango.
infra-cross-site-title = Ba-formulaire uta na ba-site mosusu
infra-cross-site-ok = Formulaire moko te etindamaki awa uta na site mosusu na mokolo moko oyo eleki.
infra-cross-site-refused = { $n ->
        [one] Formulaire moko etindamaki uta na site mosusu eboyamaki na mokolo moko oyo eleki.
       *[other] Ba-formulaire { $n } etindamaki uta na ba-site mosusu eboyamaki na mokolo moko oyo eleki.
    }
infra-cross-site-fix = Eloko moko te elekaki. Soki boboyi yango ezali ya yo — site yango moko ekomami na adresi ya mibale — tia adresi ya polele na nse ya Bokangisi, to salela kaka adresi wana moko.
infra-sso-title = Ba-service ya kokota
infra-sso-none = Service moko te ya kokota epelisami; ba-compte ekotaka na mot de passe.
infra-sso-ok = { $n ->
        [one] Service moko ya kokota epelisami, mpe adresi ya polele ezali HTTPS.
       *[other] Ba-service { $n } ya kokota epelisami, mpe adresi ya polele ezali HTTPS.
    }
infra-sso-no-https = Service ya kokota epelisami, kasi adresi ya polele ezali HTTPS te: bapesi ya service baboyaka kotinda eyano na bango na adresi ezangi sekele, mpe kokota elongaka te.
infra-sso-fix = Na nse ya Bokangisi, tia adresi ya polele lokola adresi ya https:// oyo batangi basalelaka.
infra-admins-title = Bakambi
infra-admins-none = Compte moko te ya bomoi ekoki kokamba site oyo. Kaka jeton ya likama ekoki, mpe kaka ntango etiami.
infra-admins-none-fix = Kota na jeton ya likama, sala mokambi na nse ya Ba-compte, sima longola jeton.
infra-admins-one = Mokambi moko. Soki moto yango azali te to abungisi mot de passe, moto moko te akoki kokamba site.
infra-admins-one-fix = Pesa moto ya mibale oyo ondimaka mokumba ya mokambi na nse ya Ba-compte.
infra-admins-ok = { $n ->
        [one] Mokambi { $n } akoki kokota.
       *[other] Bakambi { $n } bakoki kokota.
    }
infra-token-title = Jeton ya likama
infra-token-generated = Jeton ya likama etiami te.
infra-token-only-way = Jeton ya likama etiami, mpe soki compte ya mokambi ezali te ezali nzela kaka moko ya kokota. Batela yango kino mokambi azala.
infra-token-standing = Jeton ya likama ezali naino. Ekotaka koleka compte nyonso mpe ndingisa nyonso.
infra-token-fix = Longola AXGF_CMS_ADMIN_TOKEN na fisyé ya zingazinga ya service, sima banda service lisusu.
infra-registration-title = Komikomisa
infra-registration-closed = Bapaya bakoki kosala ba-compte te; kaka mokambi akoki.
infra-registration-secret = Bapaya bakoki kosala compte ya ndeko ya lisanga soki bayebi sekele ya bokomisi.
infra-registration-open = Mopaya nyonso akoki kosala compte ya ndeko ya lisanga.
infra-registration-open-fix = Yango ebongi mpo na site oyo esengeli kozala polele. Soki te senga sekele ya bokomisi, to kanga bokomisi, na nse ya Ba-compte.
infra-failures-title = Bomeki ya kokota elongi te
infra-failures-ok = { $n ->
        [one] Bomeki { $n } ya kokota elongi te na mokolo moko oyo eleki.
       *[other] Bomeki { $n } ya kokota elongi te na mokolo moko oyo eleki.
    }
infra-failures-many = { $n ->
        [one] Bomeki { $n } ya kokota elongi te na mokolo moko oyo eleki: moto moko akoki kozala kosimba ba-mot de passe.
       *[other] Bomeki { $n } ya kokota elongi te na mokolo moko oyo eleki: moto moko akoki kozala kosimba ba-mot de passe.
    }
infra-failures-fix = Adresi moko na moko ekitisami kala lombangu sima ya bolongi te mbala na mbala. Soki motango ezali kaka komata, luka bomeki euti wapi na zulunalo ya proxy mpe kanga yango kuna, mpe tala ete mokambi nyonso azali na mot de passe molai to akotaka na nzela ya service ya kokota.
infra-health-bundle-title = Ba-données ya libota
infra-health-bundle-ok = Ebombelo efungwamaka mpe elongaka bomeki.
infra-health-bundle-fix = Fungola lokasa ya ebombelo AXGF mpo na komona mabunga mpe kobongisa yango, to zongisa kopi ya suka oyo elongaki bomeki (axgf-cms restore).
infra-health-cache-title = Cache ya bafisyé ekangisami
infra-health-cache-ok = Fisyé nyonso ekangisami ezali na cache.
infra-health-cache-fix = Bomba mokanda moko mbala moko: kobomba etongaka lisusu cache. Bafisyé oyo ezangi naino sima na yango ezangi na ebombelo yango moko.
infra-health-backup-title = Bakopi
infra-health-backup-ok = { $count ->
        [one] Kopi ya sika ezali; ebombelo { $count } ebatelami.
       *[other] Kopi ya sika ezali; bibombelo { $count } ebatelami.
    }
infra-health-backup-fix = Soki esika ya bakopi ezali te, banda lisusu bootstrap.sh na --backup-dir. Soki ezali, tala minuterie (systemctl status axgf-cms-backup.timer) mpe zulunalo na yango (journalctl -u axgf-cms-backup).
infra-disk-title = Esika na disque
infra-disk-ok = { $free } mpamba kati na { $total } ({ $percent }% mpamba).
infra-disk-fix = Longola biloko na partition oyo ezali na esika ya ba-données — bakopi ya kala oyo ebatelami mpe esika mosusu, bazulunalo, bafisyé ya baprogramu mosusu — to tinda ba-données na disque monene koleka.
infra-growth-title = Bokoli
infra-growth-ok = { $days ->
        [one] Ezali kokola pene na { $rate } na sanza; esika mpo na mokolo { $days } lisusu na lombangu wana.
       *[other] Ezali kokola pene na { $rate } na sanza; esika mpo na mikolo { $days } lisusu na lombangu wana.
    }
infra-growth-soon = { $days ->
        [one] Ezali kokola pene na { $rate } na sanza: na lombangu wana disque ekotonda na mokolo { $days }.
       *[other] Ezali kokola pene na { $rate } na sanza: na lombangu wana disque ekotonda na mikolo { $days }.
    }
infra-growth-fix = Bongisa esika sikoyo: disque monene koleka, bakopi moke ebatelami na disque yango moko, to bakopi etindami na disque mosusu.
infra-growth-flat = Bakopi ekoli te; eloko ya kobongisa ezali te.
infra-growth-unknown = Bakopi ekoki naino te mpo na komeka bokoli: esengeli mokolo moko na moke kati na oyo ya kala koleka mpe oyo ya sika koleka.
infra-memory-title = Mémoire
infra-memory-ok = { $used } esalelami kati na ndelo ya { $max } ya service ({ $percent }%).
infra-memory-high = { $used } esalelami kati na ndelo ya { $max } ya service ({ $percent }%): pene na esika oyo système etelemisaka service.
infra-memory-fix = Tombola MemoryMax= na fisyé ya drop-in mpo na axgf-cms.service, to luka nini ezali kosimba mémoire — bokotisi monene mpenza, fisyé monene ekangisami — mpe banda service lisusu.
infra-memory-unlimited = { $used } ezali kosalelama, ndelo moko te etiami mpo na service.
infra-version-title = Version
infra-version-ok = v{ $current } ezali version ya sika koleka oyo ebimisami.
infra-version-behind = v{ $current } ezali kosala; { $latest } ebimisami.
infra-version-behind-fix = Tanga makomi ya version, sima mata version uta na eteni Version na nse. Kopi ezwamaka liboso, mpe version ya liboso ezongaka yango moko soki ya sika ebandi te.
infra-version-unknown = Eyebani te soki version ya sika ezali: molongo ya ba-version ezwami te.
infra-version-unknown-fix = Salela “Luka ba-version ya sika” na nse. Esika ekangami ya production ekoki kokoma na GitHub yango moko te; soki mosalisi ya komata version atiami, azwaka molongo mokolo na mokolo.
infra-version-stale = { $days ->
        [one] Molongo ya ba-version ezwamaki mbala ya suka mokolo { $days } eleki.
       *[other] Molongo ya ba-version ezwamaki mbala ya suka mikolo { $days } eleki.
    }
infra-upgrades-title = Komata version
infra-upgrades-ok = Mosalisi ya komata version atiami: bouton na nse ekoki kobongola version.
infra-upgrades-manual = Mosalisi ya komata version atiami te, yango wana version ebongolamaka na loboko na serveur.
infra-upgrades-fix = Banda bootstrap.sh ya sikoyo mbala moko lokola root: etiaka mosalisi elongo na service.
infra-sandbox-title = Esika ekangami
infra-sandbox-ok = Processus esalaka na no-new-privileges mpe na filtre ya mabengi ya système.
infra-sandbox-unknown = Système oyo eyebisaka te esika ekangami ya processus.
infra-sandbox-off = Processus esalaka kozanga eteni ya esika na yango ekangami: no-new-privileges to filtre ya mabengi ya système ekufi.
infra-sandbox-fix = Banda service uta na unit oyo bootstrap.sh ekomaka, kozanga bafisyé ya drop-in oyo efungolaka yango. Na masini ya bokeli yango ezelamaka.
infra-clock-title = Ngonga
infra-clock-ok = Ngonga ya système elakisi { $year }, oyo ekoki kozala.
infra-clock-wrong = Ngonga ya système elakisi { $year }: mikolo, mibu ya bakopi mpe ba-session nyonso ezali mabe.
infra-clock-fix = Pelisa boyokanisi ya ngonga: timedatectl set-ntp true.

infra-title = Bisaleli
infra-lede = Masini oyo site oyo esalaka likolo na yango, etalami mbala nyonso oyo lokasa oyo efungwami: ndenge bato bakomaka na yango, nani asimbi bafungola, soki ba-données ezali na bokengi mpe ekoki naino, version nini ezali kosala, mpe esika ekangami oyo ezingi yango.
infra-kpi-health = Bokolongono
infra-kpi-version = Version
infra-kpi-version-newer = { $version } ezali
infra-kpi-version-current = Ezali ya sika
infra-kpi-version-unknown = Etalami naino te
infra-kpi-disk = Esika mpamba
infra-kpi-disk-note = { $percent }% ya { $total } esalelami
infra-kpi-compute = Masini
infra-kpi-cpus = { $n ->
        [one] cœur { $n }
       *[other] ba-cœur { $n }
    }
infra-kpi-load = Kilo ya mosala { $load }
infra-kpi-rss = { $size } ezali kosalelama
infra-unknown = Eyebani te
infra-health = Bokolongono
infra-health-intro = Malamu ezali malamu, likebisi ebongi kotala, likama esengeli kotalama sikoyo. Likambo moko na moko efungwamaka na oyo ya kosala.
infra-json = Bomeki yango moko lokola JSON, mpo na système ya bokengi
infra-version = Version
infra-version-running = Ezali kosala
infra-version-latest = Ya suka ebimisami
infra-version-up-to-date = ezali ya sika
infra-version-checked = Molongo ezwami
infra-version-never = soko moke te
infra-source-direct = na serveur oyo
infra-source-helper = na mosalisi ya komata version
infra-version-token = Dépôt ya sekele
infra-token-env-set = Jeton ya GitHub etiami mpo na kotanga mpe kokitisa ba-version.
infra-token-env-unset = Jeton ezali te. Soki dépôt ezali ya sekele, tia moko na { $name }, oyo kaka root akoki kotanga: mosalisi ya komata version atangaka yango kuna.
infra-releases-error = Bomeki ya suka ya kozwa molongo elongaki te:
infra-releases-refresh = Luka ba-version ya sika
infra-releases-asked = Mosalisi ya komata version asengami molongo ya sika. Tia lokasa oyo lisusu sima ya miniti moko.
infra-releases-not-asked = Bosengi ekokaki kokomama te: { $error }
infra-upgrade-status-running = Ezali kobongola uta { $from } kino { $to }…
infra-upgrade-status-succeeded = Ebongwani uta { $from } kino { $to }.
infra-upgrade-status-failed = Bobongoli uta { $from } kino { $to } elongaki te; { $from } ezali naino version oyo ezali kosala.
infra-upgrade-status-refused = Mosalisi ya komata version aboyaki bobongoli kino { $to }.
infra-upgrade-log = Oyo mosalisi abimisaki
infra-upgrade-pending = { $by } asengaki { $version } na { $at }; mosalisi ya komata version azwi yango naino te.
infra-releases = Ba-version ebimisami
infra-releases-tag = Version
infra-releases-published = Ebimisami
infra-releases-relation = Soki tokokanisi na oyo
infra-releases-notes = Makomi ya version
infra-releases-open = Tanga
infra-relation-newer = ya sika koleka
infra-relation-current = oyo
infra-relation-older = ya kala koleka
infra-prerelease = version ya komeka
infra-releases-none = Molongo ya ba-version ezali naino te.
infra-upgrade = Bongola version
infra-upgrade-intro = Bouton esengaka mosalisi ya komata version, oyo asalaka lokola root libanda ya esika ekangami ya site oyo. Azwaka kopi, atiaka version sima ya kondimisa checksum na yango, abandaka service lisusu mpe atalaka yango; soki version ya sika eyanoli te, oyo ya liboso ezongisamaka.
infra-upgrade-version = Version ya kotia
infra-upgrade-confirm = Mpo na kozonga na version ya kala, koma lisusu elembo na yango
infra-upgrade-confirm-hint = Esengeli te mpo na version ya sika koleka. Version ya kala ekoki kotanga te ba-données oyo version ya sika ekomaki: tanga liboso makomi na yango.
infra-upgrade-go = Tia version oyo
infra-upgrade-safety = Site ezali te mpo na basekonde moke ntango service ezali kobanda lisusu.
infra-upgrade-manual = Mosalisi ya komata version atiami te na serveur oyo, yango wana lokasa oyo ekoki kobongola version te. Lokola root, na serveur:
infra-upgrade-manual-helper = Kobanda bootstrap.sh ya sikoyo mbala moko etiaka mosalisi, mpe bouton awa esalaka uta wana.
infra-upgrade-asked = { $version } esengami. Mosalisi ya komata version azwaka yango na basekonde moke; lokasa oyo elakisaka ndenge esalemaki.
infra-upgrade-no-helper = Mosalisi ya komata version atiami te, yango wana { $version } ekoki kotiama awa te.
infra-upgrade-unknown-version = { $version } ezali na molongo ya ba-version ebimisami te.
infra-upgrade-same-version = { $version } ezali version oyo ezali kala kosala.
infra-upgrade-confirm-older = { $version } ezali ya kala koleka version oyo ezali kosala. Koma elembo na yango na esika ya bondimisi mpo na kozonga na yango.
infra-volumes = Ebombelo
infra-volumes-intro = Partition oyo ezali na esika ya ba-données ({ $path }), mpe oyo site oyo ebatelaka kuna.
infra-volumes-partition = Partition
infra-of = { $used } kati na { $total }
infra-volumes-ours = Site oyo
infra-volumes-breakdown = Oyo site oyo ebatelaka
infra-part-bundle = Ebombelo ya libota
infra-part-cache = Cache ya bafisyé ekangisami
infra-part-backups = Bakopi
infra-part-binary = Programu
infra-part-journal = Zulunalo ya mbongwana
infra-part-accounts = Ba-compte mpe makambo ya bobongisi
infra-files = { $n ->
        [one] fisyé { $n }
       *[other] bafisyé { $n }
    }
infra-growth = Bokoli
infra-growth-month = Na sanza
infra-growth-not-enough = bakopi ekoki te mpo na koyeba
infra-growth-left = Esika etikali na lombangu oyo
infra-backups = Bakopi
infra-backups-dir = Esika
infra-backups-count = Bibombelo
infra-backups-newest = Ya sika koleka
infra-backups-oldest = Ya kala koleka
infra-backups-room = Esika oyo ebatelami
infra-free-of = { $free } mpamba kati na { $total }
infra-paths = Banzela
infra-path-data = Esika ya ba-données
infra-path-binary = Programu
infra-compute = Ordinatere
infra-compute-hidden = Esika ekangami ya service ebombaka mitango ya masini yango moko; oyo emonani awa ezali oyo processus ekoki komona.
infra-compute-memory = Mémoire ya masini
infra-available-of = { $available } mpamba kati na { $total }
infra-compute-ceiling = Mémoire ya service
infra-compute-no-ceiling = { $used }, ndelo te
infra-compute-os = Système
infra-compute-kernel = Kernel
infra-compute-arch = Architecture

infra-compute-cpu = Processeur
infra-compute-cpus = Ba-cœur
infra-compute-load = Kilo ya mosala (miniti 1, 5, 15)
infra-compute-rss = Processus oyo
infra-compute-threads = Ba-thread
infra-compute-files = Bafisyé efungwami
infra-compute-uptime = Service ezali kosala uta
infra-compute-sys-uptime = Masini ezali kosala uta
infra-compute-sandbox = Esika ekangami
infra-compute-nnp-on = ndingisa ya sika te
infra-compute-nnp-off = ndingisa ekoki kozwama
infra-compute-seccomp-on = mabengi ya système ekangami
infra-compute-seccomp-off = mabengi ya système ekangami te
infra-activity = Misala
infra-activity-intro = Etangami na { $span }: service ebatelaka mokolo moko, mpe ebandaka lisusu na zero ntango ebandi lisusu.
infra-activity-sessions = Ba-session efungwami
infra-activity-signed-in = Bokoti
infra-activity-failed = Bomeki ya kokota elongi te
infra-activity-registered = Ba-compte esalemi
infra-activity-cross-site = Ba-formulaire uta site mosusu eboyami

sso-error-not-configured = Service oyo ya kokota ebongisami na site oyo te.
sso-error-no-public-url = Kokota na nzela ya service esengi adresi ya polele ya site oyo, oyo etiami naino te. Mokambi atiaka yango na nse ya Bokangisi.
sso-error-unreachable = Service ya kokota ekokaki kokomama te. Meka lisusu sima ya mwa ntango, to kota na nzela mosusu.
sso-error-refused = Service ya kokota endimaki yo te, to kokota etikamaki kuna.
sso-error-stale = Kokota yango esilaki ntango, to ebandamaki na navigateur mosusu. Banda lisusu.
sso-error-invalid = Eyano ya service ya kokota elongaki bomeki ya site oyo te, yango wana esalelamaki te. Banda lisusu; soki ezali kaka kosalema, yebisa mokambi.
infra-upgrade-already-pending = { $version } ezali kala kozela mosalisi ya komata version. Zela esila liboso ya kosenga version mosusu.

connectivity-title = Bokangisi
connectivity-lede = Ndenge batangi bakomaka na site oyo mpe ndenge bakotaka: adresi ya polele, ba-service ya kokota mpe mibeko oyo kokota nyonso elandaka, ata ya bakambi.
connectivity-kpi-address = Adresi ya polele
connectivity-address-unset = Etiami te
connectivity-address-https = HTTPS
connectivity-address-plain = Ezangi sekele
connectivity-address-unset-note = Service nyonso ya kokota esengaka yango
connectivity-kpi-services = Ba-service ya kokota
connectivity-kpi-services-note = { $n ->
        [one] ebongami mpo na kopesama
       *[other] ebongami mpo na kopesama
    }
connectivity-kpi-passwords = Ba-mot de passe
connectivity-policy-short-everyone = Bato nyonso
connectivity-policy-short-non-admins = Bakambi te
connectivity-policy-short-nobody = Moto moko te
connectivity-policy-note-everyone = Compte nyonso ekoki kokota na mot de passe na yango, mpe na service ekangisami lisusu.
connectivity-policy-note-non-admins = Bakambi bakotaka kaka na nzela ya service; basusu nyonso bakoki kaka kosalela mot de passe. Mot de passe ya mokambi oyo eyibami efungolaka eloko te.
connectivity-policy-note-nobody = Moto nyonso akotaka na nzela ya service. Compte ezangi service ekangisami mpe adresi ya kokangisa ekoki kokota te.
connectivity-kpi-admins-linked = { $n ->
        [one] Mokambi { $n } na service ekangisami
       *[other] Bakambi { $n } na service ekangisami
    }
connectivity-kpi-registration = Bokomisi
connectivity-kpi-registration-go = Tia yango na lokasa ya Ba-compte →
registration-short-closed = Kaka bakambi
registration-short-secret = Na sekele ya libota
registration-short-open = Polele mpo na bapaya
connectivity-address = Adresi ya polele
connectivity-address-intro = Adresi oyo batangi bakomaka, ndenge bakomaka yango: https://family.example. Ba-service ya kokota bazongisaka batangi kuna, yango wana esengeli kozala adresi ya polele, kasi oyo serveur oyo eyokaka te.
connectivity-address-label = Adresi ya polele
connectivity-address-hint = Tika yango mpamba soki moto moko te akotaka na nzela ya service.
connectivity-address-invalid = Wana ezali adresi ya http to https na nkombo ya hôte te.
connectivity-address-saved = Adresi ya polele ebombami.
connectivity-policy = Mibeko ya kokota
connectivity-policy-intro = Esalaka mpo na compte nyonso, ata ya bakambi. Mobeko oyo ekotika mokambi moko te oyo akoki kokota eboyamaka.
connectivity-policy-passwords = Nani akoki kokota na mot de passe
connectivity-policy-without-way = { $n ->
        [one] Compte { $n } ya bomoi ezali na service ekangisami te mpe na adresi te: na nse ya “Moto moko te” ekokaki kokota te.
       *[other] Ba-compte { $n } ya bomoi ezali na service ekangisami te mpe na adresi te: na nse ya “Moto moko te” ekokaki kokota te.
    }
connectivity-link-by-email = Kangisa service na compte oyo ezali na adresi yango moko endimisami
connectivity-link-by-email-note = Ndenge ndeko azwaka kokota na kofina mbala moko kozanga bokomisi: sala compte na ye na adresi na ye, mpe kokota na ye ya liboso na nzela ya service oyo endimisaki adresi yango ekangisaka yango. Kaka adresi oyo mokambi akomaki to service endimisaki nde eyokanisamaka.
connectivity-domains = Ba-domaine oyo ekoki kofungola compte
connectivity-domains-hint = Moko na molongo moko, lokola family.example. Mpamba: nyonso. Soki etiami, kaka adresi oyo service endimisaki na moko ya ba-domaine oyo ekoki kofungola compte, mpe formulaire ya bokomisi epesamaka te.
connectivity-domain-invalid = “{ $domain }” ezali nkombo ya domaine te.
connectivity-policy-unknown = Pona nani akoki kokota na mot de passe.
connectivity-policy-lockout = Eboyami: na mbongwana oyo mokambi moko te ya bomoi akokaki kokota. Kangisa liboso service ya kokota na compte ya mokambi, na kokota na nzela na yango mbala moko, to batela ba-mot de passe mpo na bakambi.
connectivity-policy-saved = Mibeko ya kokota ebombami.
connectivity-not-saved = Ebombami te: { $error }
connectivity-services = Ba-service ya kokota
connectivity-services-intro = Komisa site oyo na console ya service yango moko na adresi ya bozongisi oyo emonisami, sima tia awa client ID mpe sekele. Bapaya bakomona “Kokoba na …” na nkasa ya kokota mpe ya bokomisi.
connectivity-outbound-note = Kokota na nzela ya service esengi ete serveur oyo ekoma na service. Unit oyo bootstrap.sh ekomaka eboyaka bokangisi ya kobima (IPAddressDeny=any): pesa ndingisa na sudo systemctl edit axgf-cms, na kobakisa IPAddressAllow=any na nse ya [Service], sima banda lisusu. Bouton ya komeka elobaka soki ekoki.
connectivity-service-ready = Ebongami
connectivity-service-incomplete = Esili te
connectivity-service-off = Ekufi
connectivity-service-enable = Pesa service oyo na nkasa ya kokota mpe ya bokomisi
connectivity-service-issuer = Mopesi
connectivity-service-issuer-hint = Adresi ya mopesi, lokola https://auth.family.example/realms/family; makambo na ye ya bobongisi etangamaka uta na /.well-known/openid-configuration na nse na yango.
connectivity-service-label = Nkombo ya bouton
connectivity-service-tenant = Annuaire (tenant)
connectivity-service-tenant-hint = common mpo na compte nyonso ya Microsoft, organizations mpo na ba-compte ya mosala mpe ya kelasi, consumers mpo na oyo ya moto moko, to elembo ya annuaire na yo.
connectivity-service-client-id = Client ID
connectivity-service-secret = Sekele ya client
connectivity-service-secret-set = Sekele etiami. Tika yango mpamba mpo na kobatela yango.
connectivity-service-secret-unset = Sekele etiami naino te.
connectivity-service-secret-clear = Longola sekele
connectivity-service-redirect = Adresi ya bozongisi ya kokomisa
connectivity-service-redirect-needs-address = Tia liboso adresi ya polele
connectivity-service-console = Esika ya kokomisa
connectivity-service-console-open = Console ya { $name }
connectivity-service-test = Meka bokangisi
connectivity-issuer-invalid = Mopesi asengeli kozala adresi ya https://.
connectivity-tenant-invalid = Annuaire ezali nkombo to elembo: mikanda, bitalele, matono mpe milongo mike.
connectivity-provider-incomplete = { $name } esili te: esengi client ID mpe sekele ya client, mpe mopesi mpo na mopesi ya mobimba.
connectivity-provider-saved = { $name } ebombami.
connectivity-test-ok = { $name } eyanoli: { $answer }.
connectivity-test-failed = { $name } ekokaki kokomama te uta na serveur oyo: { $detail }
registration-title = Bokomisi
registration-intro = Soki mopaya akoki kofungola compte awa ye moko. Compte ya ndenge wana ezali ya ndeko ya lisanga: etangaka oyo bandeko ya lisanga batangaka mpe ebongolaka eloko te. Kopesa yango mokumba mosusu ezali mokano ya mokambi, na nse.
registration-who = Nani akoki kofungola compte
registration-note-closed = Kaka mokambi asalaka ba-compte, na lokasa oyo.
registration-note-secret = Mopaya oyo ayebi sekele ya libota akoki kofungola compte ya ndeko ya lisanga. Kabola sekele ndenge okokabola code ya ekuke.
registration-note-open = Mopaya nyonso akoki kofungola compte ya ndeko ya lisanga. Bandeko ya lisanga batangaka mikanda oyo ebatelami mpo na bango.
registration-rotate = Sala sekele ya sika
registration-secret = Sekele ya libota
registration-secret-note = Moto nyonso oyo azali na yango akoki kofungola compte ntango lolenge oyo epelisami. Kosala ya sika ekangaka oyo ya kala; ba-compte oyo efungwamaki kala etikalaka.
registration-link = Lokasa ya bokomisi
registration-quick = Banzela ya lombangu ya kokota
registration-quick-none = Service moko te ya kokota ebongami: bongisa moko na nse ya Bokangisi
registration-changed = Ebongolamaki mbala ya suka na { $by }, { $at }.
registration-unknown = Pona nani akoki kofungola compte.
accounts-kpi-total = Ba-compte ya bomoi
accounts-kpi-total-note = { $disabled ->
        [one] { $disabled } ekufi
       *[other] { $disabled } ekufi
    }
accounts-kpi-admins = Bakambi
accounts-kpi-admins-few = Bakisa ya mibale
accounts-kpi-admins-ok = Koleka moko, ndenge esengeli
accounts-kpi-linked = Ekangisami na service
accounts-kpi-linked-note = { $n ->
        [one] Service { $n } ebongami
       *[other] Ba-service { $n } ebongami
    }
accounts-kpi-registered = { $n ->
        [one] { $n } efungwamaki na mopaya
       *[other] { $n } efungwamaki na bapaya
    }
accounts-roles = Mikumba
accounts-sign-in = Akotaka na
accounts-sign-in-password = mot de passe
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = kokota mbala moko
accounts-sign-in-none = nzela ya kokota ezali naino te
accounts-created-via-register = efungwamaki na mopaya
accounts-created-via-sso = efungwamaki na nzela ya service
accounts-add-also-register = Bapaya bakoki mpe kofungola ba-compte ya bandeko ya lisanga bango moko, ndenge etiami likolo. Compte oyo esalemi awa ekoki kozala na mokumba nyonso.
login-password-not-allowed = Mibeko ya kokota ya site oyo endimaka te ete compte oyo esalela mot de passe: kota na nzela ya service.
login-register = Ozali naino na compte te? Sala moko
register-title = Sala compte
register-lede = Compte ya ndeko ya lisanga etangaka mikanda oyo ebatelami mpo na bandeko ya libota. Nzela ya lombangu koleka ezali compte oyo ozali na yango kala.
register-closed = Ba-compte awa esalemaka na mokambi. Senga moko kati na bango compte na yo.
register-have-account = Ozali kala na compte? Kota
register-or = to
register-secret = Sekele ya libota
register-secret-hint = Mokambi apesaki yo yango, to moto moko ya libota.
register-secret-hint-services = Koma yango liboso ya kopona service, soki te etape oyo ekoya ekosenga yango.
register-domains = Kaka ba-adresi ya { $domains } ekoki kofungola compte.
register-username-hint = Mikanda, bitalele, matono, milongo mike to milongo ya nse 2 kino 64.
register-submit = Sala compte
register-member-note = Adresi esengami te: eloko moko te etindamaka epai na yo, mpe moto moko te akoki koloba ete adresi na yo ezali ya ye.
register-form-closed = Site oyo efungolaka ba-compte na formulaire oyo te.
register-throttled = Bomeki mingi mpenza uta awa. Zela eteni ya minei ya ngonga mpe meka lisusu.
register-secret-wrong = Wana ezali sekele ya libota te.
register-username-invalid = Nkombo ya mosaleli ezali na mikanda, bitalele, matono, milongo mike to milongo ya nse 2 kino 64.
register-password-short = Mot de passe esengi bilembo { $min } na moke.
register-username-taken = Nkombo wana ya mosaleli ezwami kala. Pona mosusu.
register-not-saved = Compte ekokaki kobombama te. Meka lisusu sima ya mwa ntango.
register-finish-title = Etape moko lisusu
register-finish-lede = Service endimisi yo ozali nani. Mpo na kofungola compte awa, koma sekele ya libota.
register-finish-lede-email = Service endimisi yo lokola { $email }. Mpo na kofungola compte awa, koma sekele ya libota.
sso-continue = Kokoba
sso-continue-with = Kokoba na { $name }
sso-error-title = Kokota esilaki te
sso-account-disabled = Compte oyo ekangisami na kokota oyo ekufi. Tuna mokambi.
sso-no-account = Compte moko te awa eyokani na kokota oyo, mpe site oyo efungolaka ba-compte yango moko te. Senga mokambi asala compte na yo na adresi oyo: kokota na yo oyo ekoya ekokangisama na yango.
sso-domain-refused = Ba-compte awa ekoki kofungwama kaka na adresi ya ba-domaine ya libota, endimisami na service.

overview-accounts-note = { $admins ->
        [one] Mokambi { $admins }
       *[other] Bakambi { $admins }
    }
register-password-hint = Bilembo { $min } na moke. Fraze ya maloba mingi ezali pete mpo na kokanisa koleka bilembo esangani.

theme-clarity = Clarity
theme-clarity-note = pole, mpembe mpe orange · entonnoir
view-funnel-back = Zonga na entonnoir
view-funnel-open = Fungola entonnoir
view-funnel-hero-1 = Nkola nyonso ezali lopete.
view-funnel-hero-2 = Libota nyonso, entonnoir.
view-funnel-cta = Luka kati na entonnoir
view-funnel-cta-detail = Nkombo nyonso ebakisami mpe liziba nyonso etalami ekomisaka entonnoir ya solo koleka mpo na baoyo bakoya sima.
view-funnel-see = Tala na entonnoir
view-funnel-direction = ↓ bankoko koleka na nsuka ya moke · ↑ ba-sika koleka na monoko
view-funnel-still = Tika entonnoir kimya
view-funnel-name = Entonnoir
view-funnel-legend = Ndenge ya kotanga entonnoir
view-funnel-search-hint = Enter ememaka bango liboso; Shift+Enter efungolaka mokanda na bango.
theme-genealogy = Genealogy
theme-genealogy-note = langi ya mayi ya nzete ya molili mpe crème · nzete ya komata
view-scroll-back = Zonga na nzete ya libota
view-scroll-open = Fungola nzete ya libota
view-scroll-hero-1 = Bomoi nyonso ezali elilingi.
view-scroll-hero-2 = Libota nyonso, nzete ya komata.
view-scroll-cta = Yeba nzete ya libota
view-scroll-cta-detail = Nkombo nyonso oyo ebakisami mpe liziba nyonso oyo etalami ekomisaka nzete ya libota ya solo koleka mpo na baoyo bakoya sima.
view-scroll-see = Tala na nzete ya libota
view-scroll-direction = ↓ epai ya misisa: bankoko · ↑ epai ya motó ya nzete: bakitani
view-scroll-still = Tika makasa kimya
view-scroll-name = Nzete ya libota
view-scroll-legend = Ndenge ya kotanga nzete ya libota
view-scroll-search-hint = Enter etiaka bango na katikati ya nzete; Shift+Enter efungolaka mokanda na bango.
bundle-binding-unnamed = ebombelo ezangi nkombo
bundle-binding-created = ebombelo esalemaki na { $date }
bundle-binding-named = “{ $family }”
bundle-binding-named-created = “{ $family }”, esalemaki na { $date }

notes-section-title = Makomi mpe makabo
notes-section-intro = Oyo bandeko ya libota bakomi: makanisi ya kala, bolukiluki, mikanda, mikanda ya leta. Luka kati na yango, tala yango na lilembo, to fungola moko mpo na kotanga yango mpe kobomba yango lokola PDF.
notes-search-label = Luka kati na makomi
notes-search-go = Luka
notes-all = Nyonso
notes-letters-label = Makomi na lilembo ya liboso ya motó na yango
notes-shown = Emonisami: { $shown } kati na { $total }
notes-none = Likomi moko te ekomami naino.
notes-none-found = Likomi moko te eyokani na boluki oyo.
notes-write = Koma likomi
notes-family-wide = Libota mobimba
notes-by = na { $author }
notes-about = Mpo na: { $names }
notes-about-prefix = Mpo na
notes-about-family = Mpo na libota mobimba
notes-pdf = Kitisa lokola PDF
notes-edit = Bongola
notes-close = Kanga
notes-back = Zonga na makomi
notes-add-about = Koma likomi mpo na moto oyo
notes-sign-in = Makomi mpe makabo ya libota ezali mpo na bandeko na yango. Kota mpo na kotanga yango.
notes-admin-title = Makomi mpe makabo
notes-admin-intro = Koma oyo mikanda ekoki kosimba te: likanisi ya kala, bolukiluki, mokanda, na bafoto. Likomi ekangisami na bato emonanaka na mikanda na bango; likomi ekangisami na moto moko te ezali ya libota mobimba. Ndeko nyonso ya lisanga atangaka nyonso na lokasa ya Libota.
notes-deleted = Likomi elongolami.
notes-col-title = Motó
notes-col-category = Lolenge
notes-col-date = Mokolo
notes-col-about = Mpo na
notes-col-author = Mokomi
notes-col-actions = Misala
notes-new-title = Likomi ya sika
notes-edit-title = Bongola likomi
notes-section-about = Likomi
notes-field-title = Motó
notes-field-category = Lolenge
notes-field-category-hint = Makomi ya bonsomi; mitindo oyo esalelami kala epesami lokola makanisi.
notes-field-date = Mokolo
notes-field-date-hint = Mobu, sanza to mokolo: 1952, 1952-06 to 1952-06-14. Tika mpamba soki likomi ezali na mokolo te.
notes-field-people = Bato oyo etali
notes-field-people-hint = Koma nkombo mpe pona yango na molongo. Soki moto moko te atangami, likomi ezali ya libota mobimba.
notes-person-add = Bakisa
notes-person-remove = Longola moto oyo
notes-field-text = Makomi
notes-text-hint = Makomi minene, ya kogumbama, mitó, milongo mpe ba-lien ekomamaka na Markdown; ba-bouton oyo ezali likolo ekomaka yango mpo na yo. Tia to bwaka foto kati na makomi mpo na kobakisa yango, kino { $mb } MB moko na moko.
notes-tool-bold = Makomi minene
notes-tool-italic = Makomi ya kogumbama
notes-tool-heading = Motó
notes-tool-list = Molongo
notes-tool-quote = Maloba ya moto mosusu
notes-tool-link = Lien
notes-tool-image = Bakisa foto
notes-tool-preview = Tala liboso
notes-tool-write = Zonga na kokoma
notes-uploading = Ezali kobakisa foto…
notes-save = Bomba likomi
notes-cancel = Tika
notes-delete = Longola likomi oyo
notes-delete-confirm = Likomi mpe bafoto na yango ekolongolama na ebombelo.
notes-category-suggestions = Likanisi ya kala, Bolukiluki, Mokanda, Lisolo, Ndenge ya kolamba, Mokanda ya leta
notes-error-title = Likomi esengeli kozala na motó. Eloko moko te ebombami.
notes-error-title-long = Motó eleki molai: bilembo 200 kaka. Eloko moko te ebombami.
notes-error-text-long = Makomi eleki molai. Eloko moko te ebombami.
notes-error-date = Mokolo oyo esosolami te. Koma mobu, sanza to mokolo: 1952, 1952-06 to 1952-06-14. Eloko moko te ebombami.
notes-error-person = Bato oyo etali: “{ $name }”.
notes-error-image-gone = Foto oyo etiamaki ezalaki lisusu kozela kobombama te; bafoto ezelaka ngonga motoba. Bakisa yango lisusu. Eloko moko te ebombami.
notes-error-too-many-images = Likomi esimbaka bafoto 40 kaka. Eloko moko te ebombami.
notes-error-stale = Moto moko abongolaki likomi oyo ntango ozalaki kosala na yango. Fungola yango lisusu mpo na komona version na ye, sima sala mbongwana na yo. Eloko moko te ebombami.
notes-error-refused = Ebombelo eboyaki mbongwana oyo. Eloko moko te ebombami.
notes-error-image = Fisyé oyo ezali foto te oyo site oyo ekoki komonisa: PNG, JPEG, GIF to WebP.
notes-error-image-large = Foto oyo eleki monene.
notes-stale-title = Likomi ebongolamaki te
notes-missing-title = Likomi emonani te
notes-missing = Likomi ezali awa te, to opesami ndingisa ya kotanga yango te.
notes-not-yours = Mosungi abongolaka kaka makomi oyo ye moko akomaki. Mokambi akoki kobongola oyo.
notes-pdf-failed-title = PDF ekokaki kosalema te
notes-pdf-failed = Likambo moko ebebaki ntango ya kosala PDF ya likomi oyo. Na ntango oyo tanga yango na lokasa na yango, mpe yebisa mokambi.
notes-dashboard-count = Makomi ekomami: { $n }
notes-dashboard-open = Koma mpe kamba makomi

kind-bundle = ebombelo
manifest-title = Libota
manifest-intro = Oyo ebombelo elobi mpo na libota oyo esimbi. Nkombo na yango nde emonanaka na lokasa ya Libota, na mitó ya nkasa mpe na PDF ya likomi nyonso; oyo etikali etambolaka elongo na fisyé. Mitango mpe mikolo ya fisyé yango moko etangamaka ntango ekomami.
manifest-name = Nkombo ya libota
manifest-name-hint = Emonanaka na lokasa ya Libota mpe na mitó. Ba-compte elandaka nkombo ya sika.
manifest-description = Ndimbola
manifest-culture = Bonkoko monene
manifest-culture-hint = Elembo ya lokota: ln, fr, pt-BR…
manifest-place = Esika monene
manifest-place-hint = Mokili monene oyo libota efandaki mingi.

manifest-span = Ntango oyo etalami
manifest-span-hint = Uta na kobotama ya kala koleka kino na oyo ya sika koleka oyo ebombelo ekomi; etangami, mpe ekomami na fisyé ntango makambo ebombami.
manifest-license = Licence
manifest-license-type = Licence
manifest-save = Bomba makambo ya libota
manifest-saved = Makambo ya libota ebombami. Ba-compte elandaka nkombo na yango.
manifest-error-long = Esika moko eleki molai: bilembo 120 mpo na nkombo, 2,000 mpo na ndimbola. Eloko moko te ebombami.
manifest-error-culture = Bonkoko ezali elembo ya lokota, lokola ln, fr to pt-BR. Eloko moko te ebombami.
manifest-error-stale = Moto moko abongolaki makambo ya libota ntango ozalaki kobongola yango. Version na yo ezali na nse: kobomba yango lisusu ekozwa esika ya oyo ya ye. Eloko moko te ebombami.
manifest-error-io = Ebombelo ekokaki kokomama te; zulunalo ya serveur elimboli ntina. Eloko moko te ebombami.

manifest-none = Elobami te
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = Mokolo ya kobotama moko te ekomami naino.
manifest-license-pick = Pona licence mpo na komona nini endimaka basusu kosala na ebombelo.
manifest-license-author = Mokomi
manifest-license-author-value = { $who }, { $at }
manifest-license-author-none = Ekomamaka ntango licence eponami.
manifest-error-place = Pona esika monene na molongo. Eloko moko te ebombami.
manifest-error-licence = Pona licence na molongo. Eloko moko te ebombami.
continent-africa = Afrika
continent-antarctica = Antarctique
continent-asia = Azia
continent-europe = Erop
continent-north-america = Amerika ya Nord
continent-south-america = Amerika ya Sudi
continent-oceania = Océanie
continent-several = Mikili minene mingi
licence-all-rights-reserved-name = Makoki nyonso ebatelami
licence-all-rights-reserved-about = Moto moko te akoki kokopia, kokabola to kosalela lisusu ebombelo kozanga ndingisa ya libota. Yango nde esalaka soki eloko moko te elobami.
licence-family-private-name = Ya sekele, mpo na libota
licence-family-private-about = Kaka mpo na bandeko ya libota: te mpo na kobimisa to kopesa yango libanda ya libota.
licence-cc0-name = CC0 1.0 — ya bato nyonso
licence-cc0-about = Moto nyonso akoki kokopia, kobongola mpe kosalela ebombelo mpo na ntina nyonso, ata ya mombongo, kozanga kosenga mpe kozanga kotanga moto.
licence-cc-by-name = CC BY 4.0 — Kotanga bakomi
licence-cc-by-about = Moto nyonso akoki kokabola mpe kobongola ebombelo, ata mpo na mombongo, soki atangi bakomi na yango mpe alobi nini abongolaki.
licence-cc-by-sa-name = CC BY-SA 4.0 — Kotanga bakomi, Kokabola na ndenge moko
licence-cc-by-sa-about = Moto nyonso akoki kokabola mpe kobongola yango, ata mpo na mombongo, na kotanga bakomi na yango; oyo akosala uta na yango esengeli kokabolama na licence yango moko.
licence-cc-by-nc-name = CC BY-NC 4.0 — Kotanga bakomi, Mombongo te
licence-cc-by-nc-about = Moto nyonso akoki kokabola mpe kobongola yango, na kotanga bakomi na yango, kasi mpo na mombongo te.
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — Kotanga bakomi, Mombongo te, Kokabola na ndenge moko
licence-cc-by-nc-sa-about = Moto nyonso akoki kokabola mpe kobongola yango, na kotanga bakomi na yango, mpo na mombongo te, mpe kaka na licence yango moko.
licence-cc-by-nd-name = CC BY-ND 4.0 — Kotanga bakomi, Kobongola te
licence-cc-by-nd-about = Moto nyonso akoki kokabola yango ndenge ezali, ata mpo na mombongo, na kotanga bakomi na yango, kasi akoki kokabola version ebongolami te.
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — Kotanga bakomi, Mombongo te, Kobongola te
licence-cc-by-nc-nd-about = Moto nyonso akoki kokabola yango ndenge ezali, na kotanga bakomi na yango, kasi mpo na mombongo te mpe na version ebongolami te: licence ya Creative Commons ya makasi koleka.
licence-odbl-name = ODbL 1.0 — Licence ya ebombelo ya ba-données ya polele
licence-odbl-about = Esalemaki mpo na bibombelo ya ba-données: moto nyonso akoki kosalela, kokabola mpe kobongola ba-données, na kotanga liziba na yango, mpe ebombelo oyo esalemi uta na yango esengeli kotikala polele na mibeko yango moko.

nav-account = Compte na ngai
account-title = Compte na ngai
account-who = Compte
account-email = Adresi ya email
account-since = Uta
account-person-title = Mokanda na ngai na nzete
account-person-intro = Loba moto nini na nzete ozali. Moto moko te atalaka yango: compte moko na moko eponaka mokanda na yango, na mokumba na yango moko, mpe ekoki kobongola to kolongola yango ntango nyonso.
account-person-is = Compte oyo elobi ete ezali
account-person-unlink = Longola bokangisi
account-person-pick = Moto na nzete
account-person-link = Kangisa compte na ngai na mokanda oyo
account-person-change = Kangisa na mokanda oyo na esika na yango
account-person-linked = Compte na yo ekangisami na mokanda na yango.
account-person-unlinked = Compte na yo ekangisami lisusu na mokanda te.
account-person-error = Mokanda oyo ekokaki kokangisama te:
account-person-io = Compte ekokaki kobombama te; zulunalo ya serveur elimboli ntina. Eloko moko te ebongolami.
account-emergency-page = Jeton ya likama ezali compte te: ezali na lokasa na yango moko te mpe na mokanda na nzete te.
accounts-person = Mokanda:

## The merge screen (src/routes/merge.rs): two records a person says are one.
merge-action = Kosangisa
merge-needs-two = Pona mikanda mibale kaka mpo na kosangisa yango.
merge-title = Kosangisa mikanda mibale
merge-lede = Eloko moko te ebongwanaka liboso ondima. Mokanda ya liboso etikalaka; ya mibale esangisami na kati na yango mpe ebombami ndenge ezalaki na zulunalo ya mbongwana.
merge-keep = Etikali
merge-absorb = Esangisami na kati
merge-swap = Kotika mosusu
merge-refused = Bosangisi oyo eboyami:
merge-refusal-ancestor = moko azali nkoko ya mosusu
merge-refusal-spouse-and-child = na libota moko, moko azali molongani mpe mosusu mwana
merge-refusal-spouses = bazali balongani mibale ya libala moko
merge-refusal-siblings = bakomami lokola bana ya baboti bamoko
merge-refusal-different-couples = mabota mibale ezali mibali na basi bakeseni
merge-refusal-child-is-spouse = mwana ya libota moko azali molongani na libota mosusu
merge-refusal-same-record = ezali mokanda moko mbala mibale
merge-conflicts = Esika bakeseni
merge-nothing = Eloko ya kopona ezali te: bawelani te.
merge-choice-keep = Mokanda oyo etikali
merge-choice-absorb = Mokanda mosusu
merge-choice-both = Mibale nyonso
merge-because-higher-confidence = Likanisi: motuya oyo ezali na bondimi koleka.
merge-because-sourced = Likanisi: motuya oyo etangi liziba.
merge-because-kept-record = Likanisi: motuya ya mokanda oyo etikali.
merge-because-more-restrictive = Likanisi: komonana oyo ezali makasi koleka.
merge-because-protective = Likanisi: na bomoi — botangi oyo ebatelaka.
merge-because-both-kept = Likanisi: kobomba makomi mibale nyonso.
merge-becomes-alias = Nkombo oyo eponami te ebombami lokola nkombo mosusu.
merge-becomes-event = Motuya oyo eponami te ebombami lokola likambo.
merge-agreements = Esika bayokani
merge-from-keep = uta na mokanda oyo etikali
merge-from-absorb = uta na mokanda mosusu
merge-from-both = ndenge moko na mibale
merge-from-joined = milongo esangisami
merge-from-combined = esangisami
merge-effects = Nini mosusu ebongwani
merge-effect-references = Batangi oyo bakomami lisusu
merge-effect-self-links = Makangisi na yango moko oyo elongolami
merge-effect-listed-twice = Milongo oyo ekotanga yango mbala mibale
merge-effect-same-couple = Mabota ya mibali na basi bamoko
merge-confirm-label = Bakoki kozala kaka moto moko. Mpo na kosangisa ata bongo, koma nkombo malamu: { $name }
merge-go = Kosangisa
merge-missing = Moko ya mikanda mibale ezali lisusu te.
merge-done-title = Esangisami
merge-done-lede = { $absorbed } ezali sikoyo eteni ya { $kept }.
merge-report-references = Batangi oyo bakomami lisusu
merge-report-events = Makambo oyo esalemi uta na motuya oyo eponami te
merge-report-source-conflicts = Bokeseni oyo ekomami na liziba
merge-report-links = Makangisi oyo elongolami
merge-report-tidied = Makomi oyo ebongisami
merge-absorbed-record = Mokanda oyo esangisami, ndenge ezalaki
merge-open-kept = Kofungola mokanda oyo etikali
