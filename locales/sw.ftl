# axgf-cms — maandishi ya kiolesura, Kiswahili.
#
# Ubora wa mashine — bado haijapitiwa na mzungumzaji wa Kiswahili kama lugha
# ya kwanza. Istilahi za nasaba hasa huenda si sahihi hapa. Marekebisho
# yanakaribishwa — tazama CONTRIBUTING.md.
#
# Istilahi zilizochaguliwa (zinaweza kujadiliwa):
#   union → muungano · link → kiungo · confidence → uhakika
#   reliability → kuaminika · source → chanzo · record → rekodi
#   archive → kumbukumbu · ancestors → mababu · descendants → wazao
#   generation → kizazi · administrator → msimamizi · contributor → mchangiaji
#
# Kiswahili kina aina mbili za wingi katika CLDR: one na other.
# Majina ya nchi na lugha yametoka kwenye data ya CLDR.
#
# KANUNI: faili hii inatafsiri kiolesura tu. Majina, mahali, madokezo na kazi
# hutoka kwenye faili ya .axgf na hubaki katika lugha na maandishi yake asilia.

app-name = ax-genealogy

nav-tree = Mti
nav-convert = Leta
nav-admin = Usimamizi
nav-sign-in = Ingia
nav-sign-out = Toka
account-emergency = Tokeni ya dharura
footer-credit = © 2026 · Imebuniwa na kujengwa na { $author }
footer-format-link = Muundo wa AXGF kwenye GitHub

settings-title = Mipangilio
edit-mode-label = Uhariri — unachohifadhi kinabadilisha rekodi za familia
display-menu-label = Lugha na mwonekano
display-settings = Mandhari na mwonekano…
settings-tab-theme = Mandhari
settings-tab-language = Lugha
settings-tab-appearance = Mwonekano
settings-done = Imekamilika
prefs-language = Lugha
prefs-theme = Mandhari
prefs-background-on = Rangi laini nyuma ya ukurasa
prefs-apply = Tumia
busy-saving = Inahifadhi…
busy-working = Inafanya kazi…
prefs-machine = mashine, { $coverage }%
prefs-machine-complete = kamili, bado haijakaguliwa

theme-canopee = Canopy
theme-canopee-note = giza, kijani cha msitu · chaguo-msingi
theme-fusion = Fusion
theme-fusion-note = giza, neon · mti hai
theme-galaxy = Galaxy
theme-galaxy-note = anga za mbali · mfumo wa jua
theme-summit = Summit
theme-summit-note = giza, alfajiri ya dhahabu · mpando

tree-title-around = Kumzunguka { $name }
tree-title-whole = Mti mzima
tree-lede-focused = { $ancestors ->
        [one] Babu mmoja
       *[other] Mababu { $ancestors }
    }, { $descendants ->
        [one] mzao mmoja
       *[other] wazao { $descendants }
    } na { $spouses ->
        [one] mwenzi mmoja
       *[other] wenzi { $spouses }
    }, vizazi { $depth } kila upande.
tree-filter-label = Chuja kadi zinazoonekana
tree-depth = Vizazi kila upande
tree-show = Onyesha
tree-hidden-notice = { $n ->
        [one] Mtu mmoja anaonyeshwa bila maelezo yake
       *[other] Watu { $n } wanaonyeshwa bila maelezo yao
    }
tree-hidden-because-role = , kwa sababu mwonekano wao uko juu ya kile akaunti yako inaruhusiwa kusoma.
tree-hidden-because-anonymous = , kwa sababu si wa hadharani.
tree-hidden-sign-in = Ingia ikiwa una akaunti.
tree-restricted-card = Rekodi ya mtu huyu haionekani kwako
tree-empty = Bado hakuna mtu wa kuchora.

record-identity = Utambulisho
record-life-events = Matukio ya maisha
record-family = Familia
record-other-relationships = Mahusiano mengine
record-occupations = Kazi
record-places = Mahali
record-sources-documents = Vyanzo na hati
record-notes = Maelezo
record-history = Historia

record-identity-help = Kila jina lililorekodiwa pamoja na aina yake, kipindi lilipotumika na chanzo chake, maandishi asilia na unukuzi wake wa Kilatini kando kwa kando yanapotofautiana, pamoja na jinsia, hali ya kuwa hai na mwonekano.
record-life-events-help = Kuzaliwa, kifo na kila tukio ambalo mtu huyu alishiriki, kwa mpangilio wa tarehe, kila moja na nafasi yake — kwa hiyo harusi aliyoishuhudia tu inaonekana kando ya harusi yake mwenyewe. Jambo lisilo na tarehe huwekwa mwisho badala ya kujifanya ni la kwanza.
record-family-help = Wazazi na ndugu, kisha kila muungano pamoja na aina yake, tarehe, mahali, jinsi ulivyoisha na watoto wake kwa mpangilio wa kuzaliwa.
record-other-relationships-help = Kila kiungo chenye mtu huyu upande wowote, kikisomwa kutoka upande wake, ili rekodi ileile ionekane kama "mzazi wa ubatizo wa" upande mmoja na "mtoto wa ubatizo wa" upande mwingine.
record-places-help = Kila mahali ambapo rekodi hii inagusa, pamoja na kilichotokea katika kila pahali na historia ya mipaka inayofanya mahali kuwa na maana kwa wakati.
record-sources-documents-help = Kila chanzo kinataja mambo ya ukurasa huu yanayotegemea chanzo hicho, kwa mpangilio wa nguvu ya ushahidi.
record-notes-help = Maelezo kuhusu rekodi hii, pamoja na maandishi ambayo kigeuzi hakikuweza kuyachanganua na kikayahifadhi kama yalivyo badala ya kuyaacha.
record-history-help = Kila badiliko lililohifadhiwa kwenye rekodi hii, la karibuni kwanza. Nani alirekebisha nini ni ukweli kuhusu watu wanaotunza mti, si kuhusu familia iliyomo, kwa hiyo hauwekwi kwenye kumbukumbu inayohamishwa na unaonyeshwa tu kwa jamaa walioingia.
record-help-toggle = Sehemu hii inaonyesha nini

record-gender = Jinsia
record-living = Yu hai
record-visibility = Mwonekano
record-yes = ndiyo
record-no = hapana
record-name-type = Aina ya jina
record-name-used = Lilitumika
record-name-evidence = Ushahidi
record-transliteration = Unukuzi wa Kilatini
record-born = Alizaliwa
record-died = Alifariki
record-parents = Wazazi
record-siblings = Ndugu
record-children = Watoto
record-unknown-person = [Hajulikani]
record-restricted-person = Binafsi
record-restricted-title = Rekodi ya mtu huyu haionekani kwako
record-absent-person-title = Ametajwa katika mti huu lakini hajarekodiwa ndani yake
record-confidence = Uhakika
conf-percent = { $percent }%
record-source = Chanzo
record-download = Pakua

access-restricted-title = Haionekani kwako
access-restricted-signed-in = Mwonekano wa rekodi hii uko juu ya kile akaunti yako inaruhusiwa kusoma. Msimamizi anaweza kubadilisha mwonekano wa rekodi au nafasi yako.
access-restricted-anonymous = Rekodi hii si ya hadharani. Ingia ili kuona kama akaunti yako inaweza kuisoma.
access-role-title = Si kwa nafasi yako
access-role-admin = Huu ni ukurasa wa msimamizi. Akaunti yako inaweza kuunda na kuhariri rekodi, lakini haiwezi kusimamia akaunti, kufuta rekodi wala kuhamisha kumbukumbu.
access-role-write = Akaunti yako inaweza kusoma mti huu lakini haiwezi kuubadilisha. Msimamizi anaweza kukupandisha hadi kuwa mchangiaji.
access-scope-title = Nje ya tawi lako
access-scope-named = Akaunti yako imewekewa tawi moja la mti, na rekodi hii inamhusu mtu aliye nje yake. Kila mtu anayetajwa na rekodi lazima awe ndani ya tawi lako — vinginevyo familia yenye mwenzi mmoja kutoka nje ingekuwa njia ya kuandika upya uzazi wa mtu huyo.
access-scope-unnamed = Akaunti yako imewekewa tawi moja la mti, na rekodi hii haimtaji mtu yeyote wa kuipimia. Vyanzo na mahali huhaririwa na akaunti zenye ufikiaji wa mti mzima.

error-not-found-title = Haikupatikana
error-not-found-detail = Ukurasa huo haupo hapa.
error-no-such-person-title = Hakuna mtu kama huyo
error-no-such-person-detail = Hakuna mtu hapa mwenye kitambulisho hicho.
error-merged-title = Rekodi hii haipo tena.
error-merged-detail = Iliunganishwa na { $name } tarehe { $date }. Viungo vya anwani ya zamani sasa vinaelekea kwenye rekodi iliyounganishwa.
error-merged-open = Fungua rekodi iliyounganishwa
error-back-to-river = Rudi kwenye mto
view-tree-back = Rudi kwenye mti
view-galaxy-back = Rudi kwenye galaksi
view-summit-back = Rudi mlimani
error-no-such-entity-title = Hakuna rekodi kama hiyo
error-no-such-entity-detail = Hakuna rekodi hapa yenye kitambulisho hicho.
error-deleted-while-editing = Hakuna rekodi hapa yenye kitambulisho hicho. Huenda ilifutwa ulipokuwa ukiihariri.
error-no-such-file-title = Hakuna faili kama hiyo
error-no-such-file-detail = Hakuna hati hapa yenye kitambulisho hicho, au hati imerekodiwa bila faili — hati inayorejelewa inataja kitu kilichohifadhiwa mahali pengine.
error-not-an-image-title = Si picha
error-not-an-image-detail = Hakuna picha ndogo ya hati hii, kwa sababu si picha ambayo toleo hili linaweza kuisoma.
error-back = Rudi

login-title = Ingia
login-lede = Akaunti huundwa na msimamizi.
login-username = Jina la mtumiaji
login-password = Nenosiri
login-submit = Ingia
login-wrong = Jina la mtumiaji na nenosiri hayalingani.
login-token-wrong = Tokeni hiyo si sahihi.
login-throttled = Majaribio mengi mno yameshindwa. Subiri dakika chache kisha ujaribu tena.
login-no-accounts-title = Usakinishaji huu bado hauna akaunti.
login-no-accounts-detail = Kwa makusudi hakuna ukurasa wa kuanzisha hapa — kipindi kati ya kusambaza na kuingia kwa mara ya kwanza ndicho hasa ambapo usakinishaji hauna ulinzi, kwa hiyo msimamizi wa kwanza huundwa kutoka kwenye mstari wa amri.
login-no-accounts-note = Inachapisha nenosiri lililotengenezwa kwenye stderr mara moja tu. Hadi wakati huo njia pekee ya kuingia ni tokeni ya dharura iliyo hapa chini.
login-emergency-summary = Ufikiaji wa dharura
login-emergency-detail = Tokeni ya pamoja bado inafungua kipindi cha msimamizi, na ipo kwa lengo moja: kurudi ndani faili ya .acl ikipotea au wasimamizi wote wakifungiwa nje. Si akaunti — haina mapendeleo, na daftari la mabadiliko huirekodi kama emergency-token badala ya mtu. Matumizi yake huandikwa kama onyo.
login-emergency-label = Tokeni ya dharura
login-emergency-submit = Tumia tokeni ya dharura
login-sign-in-prompt = Ingia ili kufikia jopo la usimamizi.

admin-title = Usimamizi
setup-title = Anzisha kumbukumbu hii
setup-lede = Usakinishaji huu bado hauna familia. Chagua jinsi itakavyoanza. Hakuna kitu hapa kinachoandikwa juu ya familia: ukurasa huu hufungwa mara moja kunapokuwa na familia.
setup-door-archive-title = Kutoka kumbukumbu ya AXGF
setup-door-archive-detail = Faili ya .axgf kutoka usakinishaji mwingine au kutoka nakala rudufu. Watu, vyanzo na picha zake huja kama zilivyo.
setup-door-archive-file = Kumbukumbu (.axgf)
setup-door-archive-go = Anzisha kutoka kumbukumbu hii
setup-limit-gb = Hadi GB { $n }, inaandikwa moja kwa moja kwenye diski.
setup-door-scratch-title = Kuanzia mwanzo
setup-door-scratch-detail = Kumbukumbu tupu. Ongeza mtu wa kwanza, kisha familia yake, rekodi moja baada ya nyingine.
setup-door-scratch-go = Ongeza mtu wa kwanza
setup-door-gedcom-title = Kutoka faili ya GEDCOM
setup-door-gedcom-detail = Faili ambayo kila programu ya nasaba huhamisha: GEDCOM 5.5, 5.5.1, 5.5.5 au 7.0, kama .ged au .gdz, kwa usimbaji wowote ilioandikwa nao. Unaona jinsi ilivyosomwa na kinachopita kabla chochote hakijahifadhiwa.
setup-door-gedcom-file = Faili ya GEDCOM (.ged, .gdz)
setup-door-gedcom-go = Soma faili hii
setup-never-over = Ili kubadilisha familia iliyopo tayari, rejesha nakala rudufu kutoka kwenye mstari wa amri (axgf-cms restore); hakuna ukurasa unaoandika juu yake.
setup-adopt-title = Fanya hii kuwa familia ya kumbukumbu
setup-adopt-detail = Rekodi ambazo zingekuwa familia ya usakinishaji huu: { $total }. Hakuna kinachohifadhiwa hadi ubonyeze kitufe.
setup-adopt-go = Anzisha kumbukumbu kwa familia hii
setup-adopt-back = Chagua njia nyingine ya kuingia
setup-done-title = Kumbukumbu imeanzishwa
setup-done-detail = Rekodi katika familia ya usakinishaji huu sasa: { $total }.
setup-done-photos = Picha na hati zilizokuja pamoja nayo: { $n }.
setup-done-mismatches = Faili ambazo baiti zake hazikulingana na alama iliyorekodiwa kwao: { $n }. Zimehifadhiwa; zikague.
setup-done-river = Fungua mto
view-tree-open = Fungua mti
view-galaxy-open = Fungua galaksi
view-summit-open = Fungua mlima
setup-done-admin = Usimamizi
setup-refused-title = Hii haikuweza kuwa kumbukumbu
setup-refused-detail = Hakuna kilichoandikwa. Kilichoizuia:
setup-closed-title = Kumbukumbu hii tayari imeanzishwa
setup-closed-detail = Ina familia, na hakuna ukurasa unaoandika juu yake. Ili kuibadilisha, rejesha nakala rudufu kutoka kwenye mstari wa amri (axgf-cms restore).
setup-not-archive = Hii si kumbukumbu ya .axgf.
setup-archive-too-large = Kumbukumbu hii ni kubwa kuliko upakiaji unavyoruhusiwa.
admin-lede = Unahariri { $path } — rekodi { $total }, { $files ->
        [one] faili moja iliyoambatishwa
       *[other] faili { $files } zilizoambatishwa
    }, { $size } kwenye diski. Kila badiliko huandikwa kwa ukamilifu; badiliko lililokataliwa huiacha faili bila kuguswa.
admin-entities = Rekodi
admin-create = Unda
admin-new-kind = { $kind } mpya
admin-operations = Shughuli
admin-validate = Hakiki
admin-deduplicate = Ondoa nakala
admin-export = Hamisha kumbukumbu
admin-roles-note = Kuhakiki, kuondoa nakala, kuhamisha, kufuta na kusimamia akaunti ni kwa wasimamizi tu. Mchangiaji hufikia kila ukurasa mwingine hapa.
admin-dedup-confirm = Kuondoa nakala huunganisha rekodi na kuandika upya kumbukumbu. Uendelee?
admin-recent-changes = Mabadiliko ya karibuni
admin-recent-note = { $shown } za mwisho kati ya { $total ->
        [one] badiliko moja lililorekodiwa
       *[other] mabadiliko { $total } yaliyorekodiwa
    }, kutoka { $path }.
admin-sessions-open = { $n ->
        [one] Kipindi kimoja kiko wazi sasa.
       *[other] Vipindi { $n } viko wazi sasa.
    }
admin-no-changes-yet = Bado hakuna kilichobadilishwa kupitia programu hii. Kila uhifadhi kuanzia sasa unarekodiwa katika { $path }.
admin-last-validation = Uhakiki wa mwisho
admin-bundle-heavy = Kumbukumbu hii ni { $size }. Yote hupakiwa wakati wa kuanza na kushikiliwa kwenye kumbukumbu ya kompyuta, kwa hiyo zaidi ya takriban { $warn } tovuti huanza kutumia kumbukumbu halisi na kuanzisha upya huchukua muda. Hilo linafaa kumbukumbu ya familia, si maktaba ya vyombo vya habari — viambatisho vikikua bila kikomo, viweke kwenye hifadhi ya faili na kumbukumbu ielekeze kwao.

admin-fields = Sehemu
admin-raw-json = JSON ghafi
form-more-fields = { $n ->
        [one] Sehemu moja zaidi
       *[other] Sehemu { $n } zaidi
    }
admin-raw-json-help = Rekodi nzima, ili hakuna kisichoweza kuhaririwa — orodha kama wenzi na watoto wa familia, au historia ya mipaka ya mahali, ziko hapa. Hii ndiyo hati ya kuanzia; sehemu zilizo juu kisha huandikwa juu ya njia zinazozimiliki, kwa hiyo hariri thamani mahali pamoja au pengine, si pote. Lazima iwe JSON sahihi la sivyo hakuna kinachohifadhiwa.
admin-save = Hifadhi
admin-cancel = Ghairi
place-editor-title = Hariri mahali
place-names = Majina
place-name-primary = Kuu
place-name-lang = Lugha
place-name-value = Jina
place-names-hint = Mstari mmoja kwa kila jina lililorekodiwa. Mahali palipotawaliwa na himaya tatu pana majina matatu; kuu ndilo linaloonyeshwa kila mahali pengine.
place-where = Mahali lilipo
place-type = Aina
place-region = Eneo
place-country-current = Nchi ya leo
place-country-hint = ISO 3166-1 alpha-2, kwa mfano PL, FR, DE.
place-country-history = Historia ya mipaka
place-history-country = Dola
place-history-from = Kuanzia
place-history-until = Hadi
place-country-history-hint = Dola gani ilishika mahali hapa katika kipindi gani. Ina maana kwa nasaba: rekodi iliyoandikwa kwa Kirusi mwaka 1880 na nyingine kwa Kipolandi mwaka 1930 zinaweza kutaja kijiji kilekile.
place-coordinates = Viwianishi
place-lat = Latitudo
place-lon = Longitudo
place-precision = Usahihi
place-identifiers = Vitambulisho
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } inatumiwa na { $n ->
        [one] rekodi nyingine moja
       *[other] rekodi nyingine { $n }
    }.
place-error-no-name = Mahali panahitaji angalau jina moja.
place-error-coords-pair = Latitudo na longitudo huenda pamoja: toa zote mbili, au usitoe yoyote.
place-error-coords-number = Latitudo na longitudo lazima ziwe nambari.
place-error-coords-range = Latitudo ni kutoka -90 hadi 90 na longitudo kutoka -180 hadi 180.
place-type-continent = bara
place-type-country = nchi
place-type-region = eneo
place-type-department = wilaya
place-type-city = jiji

place-type-village = kijiji
place-type-district = mtaa
place-type-street = barabara
place-type-building = jengo
place-type-farm = shamba
place-type-island = kisiwa
place-type-historical = cha kihistoria
place-type-unknown = haijulikani
place-precision-exact = halisi
place-precision-building = jengo
place-precision-street = barabara
place-precision-city_center = katikati ya jiji
place-precision-region_center = katikati ya eneo
place-precision-country_center = katikati ya nchi
place-precision-approximate = takriban

place-coordinates-hint = Kuandika kwa mkono ndiyo njia ya kawaida. Mahali mengi yaliyorekodiwa chini ya utawala wa zamani hayapatikani kabisa kwa utafutaji wa kisasa.
place-geocode-search = Tafuta jina hili
place-geocode-hint = Hutuma jina, eneo na nchi kwa huduma ya kutafuta mahali, mahali pamoja kwa wakati mmoja. Hakuna kinachohifadhiwa hadi uhifadhi.
place-geocode-off = Utafutaji wa majina umezimwa. Unahitaji anwani ya mawasiliano ambayo huduma inaweza kutambulia usakinishaji huu; anzisha seva kwa --geocoder-contact ili kuuwasha.
place-geocode-query = Kilichotafutwa: { $q }
place-geocode-error = Huduma ya utafutaji haikuweza kufikiwa. Sehemu za viwianishi zilizo juu bado zinafanya kazi.
place-geocode-none = Hakuna kilichopatikana. Kwa kijiji kilichorekodiwa chini ya utawala wa Kirusi, Kiprussia au Kiaustria hii ndiyo matokeo ya kawaida; weka mahali kwa mkono.
place-geocode-not-a-place = si makazi
place-geocode-use = Tumia hii
place-geocode-attribution = Matokeo kutoka OpenStreetMap kupitia Nominatim, chini ya Open Database Licence.

place-paste = Bandika mahali
place-paste-placeholder = kiungo cha ramani, au 52.0782795, 21.2508068
place-paste-read = Isome
place-paste-hint = Kiungo cha Google Maps au OpenStreetMap, URI ya geo:, jozi ya kawaida, au digrii-dakika-sekunde kama 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Imesomwa kwenye sehemu zilizo juu. Ikague, kisha uhifadhi.
place-paste-unreadable = Hapo si mahali ambapo kinaweza kusomwa. Sehemu zilizo juu bado zinapokea jozi ya kawaida ya nambari.

place-map-hint = Bofya ramani kuweka nukta, au buruta pini. Sehemu zilizo juu ndizo rekodi.
place-map-clear = Ondoa nukta
place-open-in-map = Tafuta mahali hapa kwenye OpenStreetMap, kisha ubandike kiungo hapa

person-tab-record = Rekodi
person-tab-life = Matukio
person-tab-media = Vyombo vya habari
person-tab-tree = Mti
person-tab-history = Historia

admin-delete = Futa
admin-not-set = — haijawekwa —
admin-edit = Hariri
list-select-row = Chagua { $name }
list-kind-label = Aina
list-select-page = Chagua kila mstari kwenye ukurasa huu
list-selected = { $n ->
        [one] { $n } imechaguliwa
       *[other] { $n } zimechaguliwa
    }
list-export = Hamisha
list-escape = Esc kuondoa uchaguzi
bulk-confirm-title = { $n ->
        [one] Futa rekodi { $n }?
       *[other] Futa rekodi { $n }?
    }
bulk-confirm-detail = Rekodi ambayo nyingine bado inairejelea inakataliwa, na hapo hakuna kinachofutwa.
bulk-confirm-go = Zifute
bulk-deleted = { $n ->
        [one] Rekodi { $n } imefutwa.
       *[other] Rekodi { $n } zimefutwa.
    }
bulk-refused = Hakuna kilichofutwa: { $name } bado inarejelewa mahali pengine. Ondoa rejeleo hilo kwanza, kisha ujaribu tena.
bulk-changed = { $n ->
        [one] Rekodi { $n } imebadilika tangu orodha ilipoonyeshwa, kwa hiyo hakuna kilichofutwa. Pakia orodha upya ujaribu tena.
       *[other] Rekodi { $n } zimebadilika tangu orodha ilipoonyeshwa, kwa hiyo hakuna kilichofutwa. Pakia orodha upya ujaribu tena.
    }
admin-range = { $from }–{ $to } kati ya { $total }
list-filiation = Ukoo
admin-previous = Iliyotangulia
admin-next = Inayofuata
admin-saved = Imehifadhiwa kama toleo { $version } — { $summary }
admin-not-saved = Haijahifadhiwa
admin-created = Imeundwa
admin-not-created = Haijaundwa
admin-deleted = Imefutwa
admin-not-deleted = Haijafutwa — hakuna kilichobadilika
admin-what-changed = kilichobadilika
admin-field = Sehemu
admin-from = Kutoka
admin-to = Kwenda
admin-version = toleo { $version }

accounts-title = Akaunti
accounts-lede = Zimehifadhiwa katika { $path }, kwa ruhusa 600, kando ya kumbukumbu na kamwe si ndani yake. Kumbukumbu hunakiliwa, hutumwa na kuchapishwa; heshi za nenosiri zikisafiri ndani yake zingefanya kila nakala ya mti wa familia kuwa nakala ya taarifa za kuingia.
accounts-existing = Zilizopo
accounts-username = Jina la mtumiaji
accounts-role = Nafasi
accounts-status = Hali
accounts-branch = Tawi
accounts-last-seen = Mara ya mwisho kuonekana
accounts-change = Badilisha
accounts-you = (wewe)
accounts-active = hai
accounts-disabled = imezimwa
accounts-never = kamwe
accounts-whole-tree = mti mzima
accounts-roots = { $n ->
        [one] mzizi mmoja
       *[other] mizizi { $n }
    }
accounts-add = Ongeza akaunti
accounts-no-registration = Wageni hawawezi kufungua akaunti wenyewe; usajili umewekwa hapo juu. Kwa familia nyingi msimamizi anayemjua kila mtu anatosha.
accounts-password-hint = Acha wazi na moja itatengenezwa na kuonyeshwa mara moja. Angalau herufi { $min } ukiiweka mwenyewe.
accounts-new-password-placeholder = nenosiri jipya (wazi = acha lilivyo)
accounts-email = Barua pepe
accounts-optional = (si lazima)
accounts-create = Unda akaunti
accounts-role-viewer = msomaji — husoma rekodi za hadharani na za wanafamilia
accounts-role-contributor = mchangiaji — pia huunda, huhariri na hupakia
accounts-role-admin = msimamizi — pia husimamia akaunti, hufuta na huhamisha
accounts-branch-hint = Inaweka kikomo cha kile akaunti hii inaweza kuhariri kwa watu hao, wazao wao na wenzi wao.
accounts-branch-reading = Haiweki kikomo cha wanachoweza kusoma — hilo huamuliwa na mwonekano wa kila rekodi, na hayo mawili yanatenganishwa kwa makusudi.
accounts-branch-placeholder = kitambulisho kimoja cha mtu kwa kila mstari
accounts-ids-in-bundle = Vitambulisho vya watu katika mti huu
accounts-emergency-warning = Umeingia kwa tokeni ya dharura. Inatoa haki za msimamizi kwa kipindi hiki lakini si akaunti: haina mapendeleo, na daftari la mabadiliko litarekodi mabadiliko yako kama emergency-token badala ya mtu. Jiundie akaunti halisi hapa chini na uingie nayo.
accounts-created-with-password = { $username } ameundwa. Nenosiri lake ni { $password } — linaonyeshwa mara moja na kuhifadhiwa tu kama heshi ya Argon2id, kwa hiyo mpe sasa.
accounts-created = { $username } ameundwa.
accounts-updated = { $username } amesasishwa. Kila kipindi alichokuwa nacho wazi kimefungwa.
accounts-username-taken = Jina hilo la mtumiaji limechukuliwa.
accounts-pick-role = Chagua nafasi.
accounts-no-such = Hakuna akaunti kama hiyo.
accounts-last-admin = Huyo ndiye msimamizi pekee aliye hai. Mpandishe mtu mwingine kwanza — usakinishaji usio na msimamizi unaweza kurejeshwa tu kwa kuhariri faili ya .acl au kutumia tokeni ya dharura.
accounts-not-saved = Haijahifadhiwa: { $error }

conflict-title = Mtu mwingine alibadilisha hii kwanza
conflict-lede = { $who } alihifadhi badiliko kwenye { $kind } hii saa { $when }, baada ya wewe kuifungua. Uhariri wako haujahifadhiwa, na hakuna kilichoandikwa juu.
conflict-no-merge = Hakuna kinachounganishwa kiotomatiki hapa. Kuunganisha uhariri wa watu wawili huzaa rekodi ambayo hakuna kati yao aliyeichagua, na katika nasaba wahariri wawili wanaotofautiana kuhusu tarehe kwa kawaida wanasoma vyanzo tofauti — hilo ni swali la mtu, si la programu. Linganisha hizo mbili hapa chini kisha uamue.
conflict-versions = Ulianza kutoka toleo { $expected }; rekodi sasa ina toleo { $current }.
conflict-both-changed = Nyote wawili mlibadilisha hizi
conflict-both-changed-detail = Sehemu hizi zilihaririwa na nyote wawili. Chochote utakachohifadhi kitachukua nafasi ya kile { $who } aliweka hapo:
conflict-different-fields = Mlibadilisha sehemu tofauti, kwa hiyo hakuna kazi ya { $who } inayobishaniwa — lakini kutumia tena bado kunaandika rekodi yako nzima juu ya yake. Kagua safu zote mbili kabla ya kuhifadhi.
conflict-field-by-field = Sehemu kwa sehemu
conflict-theirs = Kile { $who } alibadilisha kuwa
conflict-yours = Kile ulichobadilisha kuwa
conflict-unchanged-by-you = haijabadilishwa na wewe
conflict-unchanged-by-them = haijabadilishwa na wao
conflict-nothing-differs = Hakuna toleo linalotofautiana na lile ulilolianzia katika sehemu yoyote ambayo ukurasa huu unaonyesha. Nambari ya toleo ilisogea, kwa hiyo mtu alihifadhi rekodi bila kubadilisha chochote inachorekodi.
conflict-what-now = Sasa nini
conflict-reapply = Tumia tena toleo lako juu ya lao
conflict-reapply-hint = Huu ni uhariri wako, umeletwa mbele dhidi ya toleo { $version }. Uhariri hapa ili kubakiza kazi yoyote ya { $who } unayotaka, kisha uhifadhi. Toleo lao linaonyeshwa hapa chini ili kunakili kutoka kwake.
conflict-save-over = Hifadhi hii juu ya yao
conflict-discard = Tupa langu na uanze upya
conflict-their-version = Toleo la { $who }, lilivyo sasa
conflict-history-of = Historia ya { $kind } hii

convert-title = Leta faili ya familia
convert-submit = Leta
convert-result-title = Ripoti ya kuleta

date-unknown = Tarehe haijulikani
date-not-recorded = Haijarekodiwa
date-circa = karibu { $date }
date-between = kati ya { $from } na { $to }
date-before = kabla ya { $date }
date-after = baada ya { $date }
date-preserved = imerekodiwa kama “{ $text }”
date-day-month-year = { $day } { $month ->
        [1] Januari
        [2] Februari
        [3] Machi
        [4] Aprili
        [5] Mei
        [6] Juni
        [7] Julai
        [8] Agosti
        [9] Septemba
        [10] Oktoba
        [11] Novemba
        [12] Desemba
        *[other] { $month }
    } { $year }
date-month-year = { $month ->
        [1] Januari
        [2] Februari
        [3] Machi
        [4] Aprili
        [5] Mei
        [6] Juni
        [7] Julai
        [8] Agosti
        [9] Septemba
        [10] Oktoba
        [11] Novemba
        [12] Desemba
        *[other] { $month }
    } { $year }
date-decade = miaka ya { $decade }
date-century = karne ya { $century }
date-quarter-century = robo ya { $quarter ->
        [1] kwanza
        [2] pili
        [3] tatu
       *[other] nne
    } ya karne ya { $century }

error-back-to-start = Rudi mwanzo
error-payload-missing-title = Hakuna faili kama hiyo
error-payload-missing-detail = Maudhui ya hati hiyo hayapo kwenye akiba.
error-payload-unopenable-detail = Maudhui ya hati hiyo hayakuweza kufunguliwa.
error-no-such-document-detail = Hakuna hati hapa yenye kitambulisho hicho.
error-bad-preference-title = Si mojawapo ya machaguo
error-bad-preference-detail = Hiyo si lugha wala mandhari ambayo tovuti hii inatoa. Hakuna kilichobadilishwa.
error-unknown-kind-title = Aina isiyojulikana
error-unknown-kind-detail = “{ $kind }” si aina ya rekodi. Kumbukumbu hii ina: { $kinds }.
error-io-title = Haikuweza kuhifadhi
error-io-detail = { $error }. Hakuna kilichobadilishwa kwenye diski.
error-upload-too-large = Faili hiyo ni kubwa kuliko kikomo cha MB { $mb }. Hakuna kilichohifadhiwa, na kumbukumbu haijabadilika.
error-upload-refused = Hati ilikataliwa: { $reason }. Kumbukumbu haijabadilika.
error-back-to-person = Rudi kwenye rekodi
error-no-such-person-to-attach = Hakuna mtu hapa mwenye kitambulisho hicho, kwa hiyo hakuna wa kuambatisha hati kwake.
error-upload-title = Upakiaji huo haukuhifadhiwa
error-download-expired-title = Upakuaji huo umepitwa na wakati
error-download-expired-detail = Uletaji huhifadhiwa kwa dakika kumi na tano, kisha hutupwa. Leta faili tena.
error-upload-none = Hakuna faili iliyopakiwa. Chagua faili kwanza.
error-upload-unsupported = Faili hiyo si aina ambayo kumbukumbu hii huhifadhi. Picha, PDF, maandishi ya kawaida, sauti na video zinakubaliwa; aina husomwa kutoka kwenye baiti za faili yenyewe, kwa hiyo kubadilisha jina la programu tekelezi hakuiingizi. SVG inakataliwa kabisa, kwa sababu SVG inaweza kubeba hati ya programu.
error-export-unreadable-title = Haikuweza kusoma kumbukumbu iliyohamishwa
error-export-unreadable-detail = { $error }

tree-title-suffix = mti
tree-show-all = Onyesha zote { $n }
tree-width-notice = Mwonekano huu una upana wa pikseli { $width } — kwenye skrini ya pikseli 1500, ni { $screens ->
        [one] skrini moja
       *[other] skrini { $screens }
    } za kusogeza kwa mlalo.
tree-confidence-label = Uhakika:
tree-band-certain = hakika
tree-band-high = juu
tree-band-medium = wastani
tree-band-low = ya kubahatisha
tree-counts = Watu { $drawn } kati ya { $total } · { $generations ->
        [one] kizazi kimoja
       *[other] vizazi { $generations }
    }
tree-unplaced-count = { $n } bila mahali
tree-contradicts-title = Mti huu unajipinga wenyewe.
tree-contradicts-detail = Hakuna mpangilio wa mistari unaoweza kutimiza hilo, kwa hiyo uhusiano ulio hapa chini uliachwa nje ya hesabu ya vizazi na baadhi ya mistari huenda imechorwa mahali pasipo pake. Sahihisha rekodi yoyote kati ya hizo mbili iliyo na kosa.
tree-contradicts-pair = Wamerekodiwa kama wanandoa na pia kama mzazi na mtoto:
tree-contradicts-more = { $n ->
        [one] Mgongano mmoja zaidi haujaorodheshwa.
       *[other] Migongano { $n } zaidi haijaorodheshwa.
    }
tree-no-people = Bado hakuna mtu katika mti huu.
tree-no-people-cta = Leta faili ya familia, au ongeza mtu wa kwanza.
tree-nobody-selected = Hakuna wa kuchora kwa uchaguzi huo.
tree-nobody-selected-cta = Anza kutoka mwonekano wa chaguo-msingi.
tree-edge-union = Muungano uliorekodiwa
tree-edge-parentage = Uzazi uliorekodiwa

home-empty = Bado hakuna kilichorekodiwa.
home-count = { $total ->
        [one] Rekodi moja
       *[other] Rekodi { $total }
    }, zimehifadhiwa katika faili moja ambayo familia inamiliki.
home-unnamed-family = Mti huu wa familia
home-in-this-tree = Kile familia imerekodi hadi sasa
home-showcase-title = Mahali mti huu tayari unasema zaidi ya majina na tarehe

home-showcase-example = Ona mfano →
home-nothing-title = Bado hakuna cha kuonyesha.
home-nothing-admin = Usakinishaji huu bado hauna familia: uanzishe kutoka kumbukumbu, kutoka faili ya GEDCOM, au kuanzia mwanzo.
home-nothing-visitor = Kumbukumbu hii ya familia inaanzishwa.
home-eyebrow = Kumbukumbu ya familia
home-hero-1 = Kila maisha ni chanzo.
home-hero-2 = Kila familia, mto.
view-tree-hero-1 = Kila maisha ni jani.
view-tree-hero-2 = Kila familia, mti.
view-galaxy-hero-1 = Kila maisha ni nyota.
view-galaxy-hero-2 = Kila familia, galaksi.
view-summit-hero-1 = Kila maisha ni hatua.
view-summit-hero-2 = Kila familia, mpando.
home-cta-river = Chunguza mto
view-tree-cta = Chunguza mti
view-galaxy-cta = Chunguza galaksi
view-summit-cta = Panda mlima
home-cta-tree = Tazama mti
home-scroll = Sogeza ili kuchunguza
home-trust-title = Uhakika wa kila uzazi
home-trust-certain = karibu hakika
home-trust-high = umethibitishwa vizuri
home-trust-medium = unaowezekana
home-trust-low = wa kukisia
home-trust-unrated = bado haujapimwa
home-showcase-eyebrow = Zaidi ya majina na tarehe
home-cta-title = Pamoja, tuhifadhi kumbukumbu ya familia.
home-cta-detail = Kila jina linaloongezwa na kila chanzo kinachokaguliwa hufanya mto kuwa wa kweli zaidi kwa wale wanaokuja baadaye.
view-tree-cta-detail = Kila jina linaloongezwa na kila chanzo kinachokaguliwa hufanya mti kuwa wa kweli zaidi kwa wale wanaokuja baadaye.
view-galaxy-cta-detail = Kila jina linaloongezwa na kila chanzo kinachokaguliwa hufanya galaksi kuwa ya kweli zaidi kwa wale wanaokuja baadaye.
view-summit-cta-detail = Kila jina linaloongezwa na kila chanzo kinachokaguliwa hufanya mlima kuwa wa kweli zaidi kwa wale wanaokuja baadaye.
home-cta-add = Ongeza mtu

showcase-links-title = { $n ->
        [one] Uhusiano mmoja nje ya familia
       *[other] Mahusiano { $n } nje ya familia
    }
showcase-links-detail = Wazazi wa ubatizo, waajiri, mashahidi na walezi, kila mmoja na tarehe zake, chanzo chake na jinsi una uhakika nacho.
showcase-occupations-title = { $n ->
        [one] Kazi moja yenye mwanzo na mwisho
       *[other] Kazi { $n } zenye mwanzo na mwisho
    }
showcase-occupations-detail = “Mwalimu, 1948–1978” huhifadhi urefu wake, na huchorwa kama mstari unaovuka miaka badala ya mstari wenye tarehe moja.
showcase-uncertain-title = { $n ->
        [one] Tarehe moja iliyoachwa na shaka kama ilivyotolewa
       *[other] Tarehe { $n } zilizoachwa na shaka kama zilivyotolewa
    }
showcase-uncertain-detail = Karibu, kabla, baada na kati hubaki kauli nne tofauti. Tarehe ambayo chanzo hakikuweza kuithibitisha haionyeshwi kamwe kana kwamba ilithibitishwa.
showcase-preserved-title = { $n ->
        [one] Tarehe moja iliyohifadhiwa kwa maneno ilivyoandikwa
       *[other] Tarehe { $n } zilizohifadhiwa kwa maneno zilivyoandikwa
    }
showcase-preserved-detail = Maneno ambayo hakuna aliyeweza kuyasoma kama tarehe huhifadhiwa kama yalivyoandikwa, badala ya kutupwa kimya kimya.
showcase-sources-title = { $n ->
        [one] Chanzo kimoja chenye uaminifu wake umerekodiwa
       *[other] Vyanzo { $n } vyenye uaminifu wake umerekodiwa
    }
showcase-sources-detail = { $primary ->
        [one] Chanzo kimoja cha msingi.
       *[other] Vya msingi { $primary }.
    } Kila jambo linaonyesha ushahidi unaolitegemeza, na jinsi ushahidi huo ulivyo na nguvu.
showcase-places-title = { $n ->
        [one] Mahali pamoja ambapo mipaka ilihama
       *[other] Mahali { $n } ambapo mipaka ilihama
    }
showcase-places-detail = Mji unaweza kuwa wa nchi tofauti nyakati tofauti, na rekodi inasema ipi ilihusika lini.

record-also-recorded-as = pia imerekodiwa kama
record-borders-moved = Mipaka ilihama:
record-display-name = jina la kuonyesha
record-read-as = soma kama
record-note = Maelezo
record-living-yes = yu hai
record-deceased = amefariki
record-open-full-page = Fungua ukurasa kamili ↗
record-open-full-title = Fungua ukurasa unaojitegemea, unaoweza kushirikiwa
record-edit = Hariri
record-see-in-river = Tazama kwenye mto
view-tree-see = Tazama kwenye mti
view-galaxy-see = Tazama kwenye galaksi
view-summit-see = Tazama mlimani
panel-empty = Chagua kadi kuona rekodi kamili ya mtu huyo hapa.
person-visibility-inline = mwonekano:
person-age-at-death = { $n ->
        [one] alifariki akiwa na mwaka { $n }
       *[other] alifariki akiwa na miaka { $n }
    }
person-age-now = { $n ->
        [one] ana mwaka { $n }
       *[other] ana miaka { $n }
    }
person-born-in = alizaliwa { $place }
person-died-in = alifariki { $place }
person-children-count = { $n ->
        [one] mtoto mmoja
       *[other] watoto { $n }
    }
person-generations-below = { $n ->
        [one] kizazi kimoja chini
       *[other] vizazi { $n } chini
    }
person-portrait-of = Picha ya { $name }
person-no-portrait = Hakuna picha iliyorekodiwa

result-diagnostics = Uchunguzi
result-diagnostics-note = Kila ujumbe wa uchunguzi ambao maktaba ilirudisha, pamoja na maonyo ambayo hayakuzuia shughuli. Hakuna unaochujwa.
result-no-diagnostics = Maktaba haikurudisha ujumbe wowote wa uchunguzi.
result-continue = Endelea
result-dashboard = Dashibodi
person-sections-label = Sehemu kwenye ukurasa huu

name-part-nasab = nasab (ukoo)
name-part-laqab = laqab (lakabu)
name-part-kunya = kunya (jina la mzazi kwa mtoto)
name-part-nisbah = nisbah (asili)
name-part-alias = jina lingine
name-part-religious_name = jina la kidini
name-part-pen_name = jina la kalamu
name-type-pen_name = jina la kalamu
gender-U = Haijarekodiwa

record-notes-title = Maelezo kuhusu rekodi hii:
record-name = Jina
record-type = Aina
record-cause = Sababu:
record-as = kama
record-partner-not-recorded = Mwenzi hajarekodiwa
record-union-from = Kuanzia
record-union-at = huko
record-union-until = hadi
record-occupation-from = kuanzia
record-occupation-until = hadi
record-source-reliability = Uaminifu
record-source-supports = Inaunga mkono
record-photographs = Picha
record-documents = Hati
record-file = Faili
record-status = Hali
record-size = Ukubwa
record-absent-document = Imetajwa na mtu huyu lakini haijahifadhiwa hapa.
record-upload-help-short = Hadi MB { $mb }. SVG inakataliwa.
record-verbatim-note = Imehifadhiwa kama rekodi ilivyosema, kwa sababu hakuna kigeuzi kilichoweza kuitafsiri.
record-file-to-attach = Faili ya kuambatisha
record-document-type = Aina ya hati
record-caption = Maelezo mafupi
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }
kind-person = mtu
kind-family = familia
kind-event = tukio
kind-link = kiungo
kind-occupation = kazi
kind-source = chanzo
kind-place = mahali
kind-document = hati

kind-person-plural = { $n ->
        [one] mtu
       *[other] watu
    }
kind-family-plural = { $n ->
        [one] familia
       *[other] familia
    }
kind-event-plural = { $n ->
        [one] tukio
       *[other] matukio
    }
kind-link-plural = { $n ->
        [one] kiungo
       *[other] viungo
    }
kind-occupation-plural = { $n ->
        [one] kazi
       *[other] kazi
    }
kind-source-plural = { $n ->
        [one] chanzo
       *[other] vyanzo
    }
kind-place-plural = { $n ->
        [one] mahali
       *[other] mahali
    }
kind-document-plural = { $n ->
        [one] hati
       *[other] hati
    }

list-matching = { $total ->
        [one] Mlingano mmoja
       *[other] Milingano { $total }
    }, { $per_page } kwa kila ukurasa.
list-filter-placeholder = Chuja kwa jina au kitambulisho
list-filter = Chuja
list-clear = Futa
list-summary = Muhtasari
list-id = Kitambulisho
list-nothing = Hakuna kitu hapa.
list-nothing-matching = Hakuna kitu hapa kinacholingana na “{ $q }”.
list-delete-confirm = Futa { $kind } hii? Chagua kitakachotokea kwa rekodi zinazoirejelea:
list-policy-reject = Kataa
list-policy-reject-detail = — kataa ikiwa kitu bado kinairejelea. Hakuna kinachopotea.
list-policy-cascade = Kwa mfuatano
list-policy-cascade-detail = — ifute na uondoe kabisa kila rejeleo kwake.
list-policy-orphan = Yatima
list-policy-orphan-detail = — ifute lakini ubakize rekodi zinazoirejelea, kiungo kikiwa tupu.

completeness-dates-title = Tarehe, kwa umbo zilizonalo kweli
completeness-no-dates = Bado hakuna tarehe zilizorekodiwa.
completeness-dates-note = Tarehe ambayo mtu aliibana hadi siku moja na tarehe ambayo mtu aliweza tu kuiweka katika muongo mmoja ni madai tofauti, na zote mbili huhifadhiwa kama zilivyotolewa. Maandishi yasiyoweza kusomwa kama tarehe kabisa huhifadhiwa neno kwa neno badala ya kutupwa.
completeness-shape-exact = halisi
completeness-shape-exact-note = siku kamili ya kalenda
completeness-shape-approximate = takriban
completeness-shape-approximate-note = karibu, au mwaka/muongo tu
completeness-shape-ranged = kipindi
completeness-shape-ranged-note = kabla, baada, au kati
completeness-shape-preserved = imehifadhiwa
completeness-shape-preserved-note = maandishi yasiyochanganulika, yamehifadhiwa neno kwa neno
completeness-shape-unknown = haijulikani
completeness-shape-unknown-note = imerekodiwa kama haijulikani

convert-page-title = Leta faili ya familia
convert-lede = Leta mti uliopo kutoka faili ya GEDCOM — faili ambayo programu nyingi za nasaba huhamisha. Hakuna kinachohifadhiwa hapa, na mti ambao tovuti hii tayari inaonyesha unabaki kama ulivyokuwa.
convert-file-label = Faili ya familia (.ged)
convert-file-hint = Hadi MB { $mb }. Mti wa watu 767 ni takriban KB 320.
convert-confidence-label = Uhakika wa mambo haya, kwa kuanzia
convert-confidence-hint = Faili inayoletwa haisemi mtu alikuwa na uhakika kiasi gani, kwa hiyo kila jambo linahitaji mahali pa kuanzia. Weka chini kwa mti uliokusanywa haraka, juu zaidi kwa uliotengenezwa kutoka kwa hati. Tafsiri ya kweli ya nambari hii ni “imeletwa, na haijakaguliwa na mtu tangu hapo” — unaweza kupandisha au kushusha kila jambo baadaye, moja moja.
convert-lang-label = Lugha ya majina ya mahali
convert-lang-hint = Lebo kama en, fr au pl.

convert-failed = Uletaji haukufanikiwa
convert-try-another = Jaribu faili nyingine
convert-converted = { $filename } imeletwa
convert-result-lede = { $total ->
        [one] Rekodi moja
       *[other] Rekodi { $total }
    }, KB { $size }. Kila kitu kiliingia kwa uhakika wa { $confidence }, na majina ya mahali yakisomwa kama { $lang }. Mti ambao tovuti hii inaonyesha haukuguswa.
convert-produced = Kilichopita
convert-skipped-title = { $n ->
        [one] Rekodi moja isiyoweza kusomwa
       *[other] Rekodi { $n } zisizoweza kusomwa
    }
convert-skipped-note = Rekodi hizi hazikuwa na chochote kinachoweza kuletwa.
convert-other-diagnostics = { $n ->
        [one] Jambo jingine moja linalofaa kujua
       *[other] Mambo mengine { $n } yanayofaa kujua
    }
convert-clean = Hakuna kilichoachwa nyuma — kila rekodi katika faili ilipita.
convert-download-title = Pakua
convert-download-named = Pakua { $name }
convert-download-note = Inahifadhiwa hapa kwa dakika kumi na tano kisha hutupwa, kwa hiyo ipakue sasa.
convert-another = Leta faili nyingine
admin-history-on = kwenye
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Kosa moja
       *[other] Makosa { $errors }
    }, { $warnings ->
        [one] onyo moja
       *[other] maonyo { $warnings }
    }, { $infos ->
        [one] taarifa moja
       *[other] taarifa { $infos }
    }.
admin-warnings-never-block = Maonyo hayazuii kamwe — ni taarifa, si vizuizi.
admin-validator-clean = Mhakiki hakuripoti chochote.
record-occupations-help-undated = Kazi hurekodiwa na mwanzo na mwisho, ili kazi kadhaa zilinganishwe kwenye mstari mmoja wa wakati. Kumbukumbu hii ina majina ya kazi lakini haina tarehe zake — jambo la kawaida baada ya kuleta, kwa kuwa faili nyingi za familia hazina pa kuziweka — kwa hiyo bado hakuna kipimo cha kuchora.
record-occupations-help-axis = Kazi ni hali yenye muda, si tukio la tarehe moja. Vipindi vyote vinashiriki mhimili mmoja, { $from }–{ $to }.
admin-value-not-set = haijawekwa
admin-validation-report = Ripoti ya uhakiki
admin-dedup-complete = Kuondoa nakala kumekamilika

admin-dedup-refused = Kuondoa nakala kumekataliwa
record-birth-order = mpangilio wa kuzaliwa
record-start-not-recorded = mwanzo haujarekodiwa
record-end-not-recorded = mwisho haujarekodiwa
panel-selected-person = Mtu aliyechaguliwa

tree-band-generation = Kizazi { $g }
tree-level-self = Kizazi hicho hicho
tree-level-up = { $n ->
        [1] Watoto
        [2] Wajukuu
        [3] Vitukuu
       *[other] Vizazi, kizazi cha { $n }
    }
tree-level-down = { $n ->
        [1] Wazazi
        [2] Babu na bibi
        [3] Wazazi wa babu na bibi
       *[other] Mababu, kizazi cha { $n }
    }
tree-card-centre = Weka mti katikati hapa
tree-card-same = Mtu yule yule, amechorwa mara ya pili
tree-zoom-out = Punguza
tree-zoom-in = Kuza
tree-zoom-fit = Tosheleza
tree-band-people = { $n ->
        [one] mtu mmoja
       *[other] watu { $n }
    }
tree-band-unplaced = Bila mahali
tree-band-unplaced-note = { $n ->
        [one] mtu mmoja asiye katika familia yoyote — anaonyeshwa badala ya kuachwa
       *[other] watu { $n } wasio katika familia yoyote — wanaonyeshwa badala ya kuachwa
    }

gender-M = Mwanamume
gender-F = Mwanamke
gender-NB = Si wa jinsia mbili
gender-unrecorded = Haijarekodiwa

name-part-given_name = jina la kwanza
name-part-family_name = jina la ukoo
name-part-patronymic = jina la baba
name-part-matronymic = jina la mama
name-part-middle_name = jina la kati
name-part-nickname = jina la utani
name-part-prefix = kiambishi awali
name-part-suffix = kiambishi tamati
name-part-particle = kiunganishi
name-part-part = sehemu

name-type-primary = kuu
name-type-other = jingine
name-type-alias = jina lingine
name-type-birth = la kuzaliwa
name-type-married = la ndoa
name-type-religious = la kidini
name-type-transliteration = unukuzi
name-type-nickname = jina la utani

note-links = { $n ->
        [one] uhusiano mmoja nje ya familia, wenye tarehe na vyanzo vyake
       *[other] mahusiano { $n } nje ya familia, yenye tarehe na vyanzo vyake
    }
note-occupations = { $n ->
        [one] kazi moja iliyorekodiwa na mwanzo na mwisho
       *[other] kazi { $n } zilizorekodiwa na mwanzo na mwisho
    }
note-birth-imprecise = tarehe ya kuzaliwa ambayo chanzo hakikuweza kuithibitisha, inaonyeshwa kama ilivyorekodiwa
note-death-imprecise = tarehe ya kifo ambayo chanzo hakikuweza kuithibitisha, inaonyeshwa kama ilivyorekodiwa
note-names = { $n ->
        [one] jina moja lililorekodiwa
       *[other] majina { $n } yaliyorekodiwa
    }
note-transliteration = jina katika maandishi yake asilia kando ya unukuzi wake wa Kilatini
note-witnessed = { $n ->
        [one] tukio moja alilolishuhudia badala ya kulimiliki
       *[other] matukio { $n } aliyoyashuhudia badala ya kuyamiliki
    }

visibility-public = hadharani
visibility-members = wanafamilia
visibility-contributors = wachangiaji
visibility-private = binafsi

family-label-couple = { $children ->
        [0] { $a } na { $b }
        [one] { $a } na { $b } — mtoto mmoja
       *[other] { $a } na { $b } — watoto { $children }
    }
family-label-half = { $children ->
        [0] { $a } na { $unknown }
        [one] { $a } na { $unknown } — mtoto mmoja
       *[other] { $a } na { $unknown } — watoto { $children }
    }
family-label-children = { $others ->
        [0] { $first } — wazazi hawajarekodiwa
        [one] { $first } na ndugu mmoja — wazazi hawajarekodiwa
       *[other] { $first } na ndugu { $others } — wazazi hawajarekodiwa
    }
family-label-empty = Familia isiyo na mtu aliyerekodiwa

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } na { $b }
event-more-people = { $a }, { $b } na { $others ->
        [one] mwingine mmoja
       *[other] wengine { $others }
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } isiyo na jina

event-category-birth = Kuzaliwa
event-category-death = Kifo
event-category-marriage = Ndoa
event-category-divorce = Talaka
event-category-baptism = Ubatizo
event-category-burial = Mazishi
event-category-immigration = Kuhamia
event-category-emigration = Kuhama
event-category-census = Sensa
event-category-residence = Makazi
event-category-military = Utumishi wa jeshi
event-category-education = Elimu
event-category-other = Tukio

reliability-primary = chanzo cha msingi
reliability-secondary = chanzo cha pili
reliability-tertiary = chanzo cha tatu
reliability-recollection = kumbukumbu ya mtu
reliability-derivative = kazi iliyotokana
reliability-authored = kazi iliyotungwa
reliability-oral = mapokeo ya mdomo
reliability-unknown = uaminifu haujulikani

document-type-photo = picha
document-type-certificate = cheti
document-type-letter = barua
document-type-record = rekodi
document-type-newspaper = kipande cha gazeti
document-type-other = hati

completeness-title = Mahali mti huu ungeweza kusema zaidi
completeness-intro = Kilichorekodiwa na kilicho bado tupu.
completeness-import-title = Kile uletaji ulileta
completeness-import-intro = Imehesabiwa kutoka faili uliyopakia sasa hivi. Mstari tupu ni kitu ambacho faili asilia haikurekodi — si kitu ambacho uletaji ulipoteza.

completeness-headline-full = Kila aina ya maelezo hapa chini imerekodiwa mahali fulani katika mti huu.
completeness-headline-empty = { $total ->
        [one] Aina moja ya maelezo hapa chini bado haijarekodiwa popote.
       *[other] Hakuna kati ya aina { $total } za maelezo hapa chini iliyorekodiwa bado.
    } Kila moja ni mahali ambapo rekodi ingeweza kusema zaidi.
completeness-headline-partial = { $carried ->
        [one] Aina moja ya maelezo hapa chini imerekodiwa
       *[other] Aina { $carried } za maelezo hapa chini zimerekodiwa
    }; { $empty ->
        [one] moja bado iko tupu
       *[other] { $empty } bado ziko tupu
    }.

completeness-metric-confidence = Uhakika wa kila jambo
completeness-metric-confidence-none = Hakuna kati ya mambo { $slots } hapa linalosema lina uhakika kiasi gani. Tarehe ambayo mtu aliisoma kwenye cheti na ile ambayo mtu aliikisia zinaonekana sawa hadi siku zinapotofautiana.
completeness-metric-confidence-uniform = Mambo { $with } kati ya { $slots } yana alama na yote yana nambari ileile ({ $modal }). Hicho ndicho uletaji wa jumla huacha nyuma: kishika nafasi ambacho hakuna aliyerudi kukitazama. Hakuna lililopimwa peke yake bado.
completeness-metric-confidence-some = Mambo { $with } kati ya { $slots } yana alama. { $modal_count } yanashiriki thamani moja ({ $modal }); { $assessed } yanatofautiana nayo na kwa hiyo yametazamwa moja moja.
completeness-metric-confidence-many = Mambo { $with } kati ya { $slots } yana alama, { $assessed } kati yake yakitofautiana na thamani ya kawaida zaidi ({ $modal }) katika viwango { $distinct } tofauti. Mti huu unarekodi shaka halisi inayobadilika.

completeness-metric-parentage = Uhakika wa kila kiungo cha mzazi na mtoto
completeness-metric-parentage-none = Hakuna uzazi hapa unaosema una uhakika kiasi gani. Kuasili, ukoo unaobishaniwa na kujenga upya kutokana na kutajwa mara moja ndipo hasa familia inahitaji kurekodi shaka — na mti huchora kiungo chenye uhakika mdogo kama mstari hafifu zaidi.
completeness-metric-parentage-some = { $n ->
        [one] Uzazi mmoja una alama yake
       *[other] Uzazi { $n } una alama zake
    }, kwa hiyo mstari wa kukisia unaonekana dhaifu kuliko uliothibitishwa kwa hati.

completeness-metric-links = Mahusiano zaidi ya damu na ndoa
completeness-metric-links-none = Wazazi wa ubatizo, waajiri, mashahidi, walezi, walinzi. Hakuna hata mmoja aliyerekodiwa bado. Kila mmoja anaweza kubeba tarehe zake, chanzo chake na uhakika wako.
completeness-metric-links-some = { $n ->
        [one] Mmoja amerekodiwa, na tarehe zake, chanzo na uhakika wako.
       *[other] { $n } wamerekodiwa, kila mmoja na tarehe zake, chanzo na uhakika wako.
    }

completeness-metric-occupations = Kazi iliyorekodiwa na mwanzo na mwisho
completeness-metric-occupations-none = Hakuna kazi zilizorekodiwa. Ufundi ulioshikiliwa kwa miaka thelathini unasema zaidi kuhusu maisha kuliko kumbukumbu moja yenye tarehe.
completeness-metric-occupations-undated = { $total ->
        [one] Kazi moja imerekodiwa, bila tarehe
       *[other] Kazi { $total } zimerekodiwa, bila tarehe
    }. Ongeza mwanzo na mwisho na zitaweza kulinganishwa kando kwa kando kwenye mstari mmoja wa wakati.
completeness-metric-occupations-some = { $span } kati ya { $total } zina mwanzo au mwisho, kwa hiyo zinaweza kulinganishwa kando kwa kando kwenye mstari mmoja wa wakati.

completeness-metric-sources = Vyanzo vilivyopimwa kwa uaminifu wake
completeness-metric-sources-none = Hakuna vyanzo vilivyorekodiwa. Kutaja jambo lilitoka wapi ndiko kunakomwezesha jamaa kulikagua baadaye — au kutokubaliana nalo na kusema kwa nini.
completeness-metric-sources-some = { $graded } kati ya { $total } vinasema vina nguvu kiasi gani, kwa hiyo dai linalotegemea cheti cha kuzaliwa linaonekana tofauti na lile linalotegemea kumbukumbu ya mtu.

completeness-what-is-recorded = Kile rekodi inaweza kusema
completeness-in-this-tree = Katika mti huu
completeness-not-yet = bado haijarekodiwa

role-spouse = mwenzi
role-spouse_1 = mwenzi wa kwanza
role-spouse_2 = mwenzi wa pili
role-subject = mhusika
role-participant = mshiriki
role-witness = shahidi
role-officiant = msimamizi wa ibada
role-informant = mtoa taarifa
role-godparent = mzazi wa ubatizo

phys-no-source = hakuna chanzo
phys-col-date = Lini
phys-col-source = Chanzo
phys-col-confidence = Uhakika
phys-col-note = Maelezo
phys-field-height-cm = Urefu
phys-field-weight-kg = Uzito
phys-field-eye-colour = Rangi ya macho
phys-field-hair-colour = Rangi ya nywele
phys-field-build = Umbo
phys-field-handedness = Mkono anaoutumia zaidi
phys-field-features = Alama za kutambulisha
phys-field-military = Utumishi wa jeshi
phys-field-languages = Lugha anazozungumza
phys-field-blood-group = Kundi la damu
phys-field-conditions = Hali za kiafya zinazojulikana
phys-field-operations = Upasuaji na majeraha
phys-field-cause-of-death = Sababu ya kifo
phys-field-religion = Dini au uanachama
phys-field-health-notes = Maelezo
admin-export-health-note = Uhamishaji wa kawaida huacha kila aina nyeti — afya na imani, data ya kibayometriki, data ya kijenetiki na rekodi za uhalifu — na wasifu wa tabia wa yeyote aliye hai, kwa hiyo faili inayotumwa kwa jamaa haibebi chochote kati ya hivyo. Weka alama kwa kile ambacho faili fulani inapaswa kubeba; kumbukumbu yenyewe hurekodi aina zipi ziliachwa.
avatar-picker-title = Chagua picha
avatar-choose = Picha ipi inamwakilisha mtu huyu
avatar-mode-auto = Acha programu ichague
avatar-mode-auto-note = Picha ya kwanza ya uso, au isipokuwepo picha ya kwanza iliyounganishwa na rekodi hii.
avatar-mode-none = Onyesha herufi za kwanza badala yake
avatar-mode-none-note = Kwa rekodi ambayo picha zake pekee ni hati badala ya nyuso.
avatar-focal-hint = Bofya picha kuichagua, na ubofye tena sehemu inayopaswa kubaki ndani ya fremu — avatar ni mraba na nakala nyingi za skana sizo.
avatar-no-images = Bado hakuna picha zilizounganishwa na rekodi hii.
avatar-upload-title = Pakia picha na uitumie
avatar-upload-button = Pakia na utumie kama picha
avatar-not-available-title = Picha hiyo haipatikani
avatar-not-available-detail = Faili iliyochaguliwa haijaunganishwa na mtu huyu, au si unayoruhusiwa kuisoma.
ai-badge = AI
ai-portraits-title = Picha zilizoboreshwa na AI
ai-portraits-band = Zimeboreshwa
ai-portraits-upload = Pakia picha zilizoboreshwa na AI
ai-portraits-manage = Simamia picha za AI
ai-portraits-intro = Picha ambazo AI ilitengeneza kutoka kwa picha za mtu huyu: jinsi ambavyo huenda alionekana. Zinaonekana kando ya uso kwenye rekodi, zimewekwa alama “AI”, na wanafamilia walioingia pekee ndio wanaoziona. Kamwe si picha ya mtu, kamwe si miongoni mwa faili zake, kamwe si ushahidi.
ai-portraits-current = Kwenye rekodi
ai-portraits-add = Ongeza picha — nafasi ya { $n }
ai-portraits-files = Picha (PNG, JPEG, WebP au GIF), kadhaa kwa mara moja
ai-portraits-generator = Imetengenezwa kwa
ai-portraits-from = Imetengenezwa kutoka
ai-portraits-from-none = Haikusemwa
ai-portraits-remove = Ondoa
ai-portraits-alt = Picha ya { $name }, iliyoboreshwa na AI
ai-portraits-not-evidence = Zimetengenezwa na AI kutoka kwa picha: si picha za wakati huo.
ai-portraits-full = Rekodi ina picha zake tatu za AI. Ondoa moja ili kuongeza nyingine.
ai-portraits-too-many = Rekodi inashika picha tatu za AI zaidi. Ondoa moja kabla ya kuongeza nyingine.
ai-portraits-images-only = Picha ya AI lazima iwe picha: PNG, JPEG, WebP au GIF.
ai-portraits-stale = Rekodi ilibadilika ukurasa huu ulipokuwa wazi. Hakuna kilichohifadhiwa: ifungue tena urudie.
ai-portraits-back = Rudi kwenye picha za AI

record-history-withheld = imezuiliwa kwako

record-life-status = Hali
record-presumed-deceased = anadhaniwa amefariki
record-presumed-short = inadhaniwa
record-presumed-why = Hakuna kifo kilichorekodiwa na kuzaliwa ni zaidi ya miaka { $years } iliyopita, kwa hiyo rekodi hii haiwezi kuwa sahihi. Kifurushi hakijabadilika: hiki ndicho ukurasa unachokisia, si kile chanzo kinachosema.

identity-editor-title = Majina na utambulisho
identity-primary-name = Jina linaloonyeshwa kila mahali
identity-primary-help = Kile ambacho kadi ya mti, kichwa na kila orodha hutumia. Majina mengine hapa chini ni yale ambayo chanzo kilimwita mtu huyu wakati mwingine.
identity-display = Jina
identity-display-latin = Kwa maandishi ya Kilatini
identity-culture = Lugha
identity-direction = Mwelekeo
identity-direction-ltr = kushoto kwenda kulia
identity-direction-rtl = kulia kwenda kushoto
identity-direction-auto = kutoka kwenye maandishi
identity-components = Sehemu za jina
identity-components-help = Sehemu ipi ni jina la kwanza na ipi ni jina la ukoo, kwa mpangilio zinavyoandikwa. Rekodi isiyo na sehemu bado inaonyeshwa: sehemu ndizo utafutaji unaweza kulinganisha nazo.
identity-part = Sehemu
identity-value = Maandishi
identity-other-names = Majina mengine
identity-other-help = Jina la ndoa, jina la kidini, jina ambalo rekodi ya baadaye ilitumia. Kila moja linabeba wakati lilipotumika na chanzo kinachosema hivyo.
identity-name-type = Aina ya jina
identity-valid-from = Lilitumika kuanzia
identity-valid-until = Lilitumika hadi
identity-about = Kuhusu mtu
identity-living-help = Hii ni alama ambayo chanzo kiliweka. Ukurasa kando yake hudhani kifo kuzaliwa kunapokuwa zamani mno, na dhana hiyo kamwe haibadilishi kisanduku hiki wala kifurushi.
identity-error-no-display = Rekodi inahitaji jina la kuonyeshwa kwalo. Hakuna kilichohifadhiwa.
editor-blank-to-remove = Futa jina ili kuondoa kipengele hiki.
identity-edit-link = Hariri majina na utambulisho

union-type-marriage = ndoa
union-type-civil_union = muungano wa kiraia
union-type-cohabitation = waliishi pamoja
union-type-religious_only = muungano wa kidini
union-type-polygamous = ndoa ya wake wengi
union-type-unknown = haijarekodiwa
union-role-spouse = mwenzi
union-role-partner = mwenzi wa maisha
union-role-husband = mume
union-role-wife = mke
union-status-active = unaendelea
union-status-ended_by_death = uliisha kwa kifo
union-status-ended_by_divorce = uliisha kwa talaka
union-status-ended_by_separation = uliisha kwa kutengana
union-status-annulled = ulibatilishwa
union-status-unknown = haijarekodiwa
union-status-ended = uliisha
union-status-ended-by = uliisha kwa { $reason }
union-reason-death_of_spouse = kifo cha mwenzi
precision-exact = hadi siku
precision-year = hadi mwaka
precision-month = hadi mwezi
precision-decade = hadi muongo
precision-century = hadi karne
precision-unknown = haijulikani
record-precision = Usahihi
record-place = Mahali

family-editor-title = Familia na mahusiano
family-unions = Miungano
family-no-unions = Hakuna muungano uliorekodiwa kwa mtu huyu.
family-union-legend = Muungano { $n }
family-writes-family = Kuhifadhi hii kunabadilisha rekodi ya familia #{ $id }, ambayo watu wote wawili wanashiriki. Ukurasa wa mtu mwingine unabadilika pamoja nayo.
family-partners = Wenzi
family-partner = Mwenzi
family-role = Nafasi
family-children = Watoto
family-children-help = Mpangilio wa kuzaliwa ni dai la rekodi yenyewe kuhusu nani alitangulia. Ukiachwa wazi, hakuna kinachodaiwa: nambari iliyochukuliwa kutoka nafasi ya mstari ingekuwa jambo ambalo hakuna aliyeliandika.
family-child = Mtoto
family-birth-order = Mpangilio wa kuzaliwa
family-the-union = Muungano wenyewe
family-type = Aina ya muungano
family-status = Hali yake
family-started = Ulianza
family-ended = Uliisha
family-leave = Mtoe mtu huyu katika muungano huu
family-open-entity = Fungua rekodi ya familia
family-new-union = Muungano mpya
family-new-union-help = Hii inaunda rekodi mpya ya familia yenye mtu huyu ndani yake. Mwenzi si lazima: mzazi ambaye rekodi inamtaja bila mwenzi kabisa ni muungano wa mtu mmoja.
family-create-union = Unda muungano
family-parents = Wazazi
family-no-parents = Mtu huyu hajarekodiwa kama mtoto wa familia yoyote.
family-child-of = Mtoto wa familia hii
family-detach-child = Mtoe mtu huyu katika familia hii
family-attach-parents = Unganisha na wazazi
family-attach-help = Chagua familia ambayo mtu huyu ni mtoto wake. Hii inamwongeza kwenye rekodi ya familia hiyo, kwa hiyo anaonekana pia kwenye kurasa za wazazi.
family-the-family = Familia
family-attach = Unganisha
family-error-last-partner = Muungano unahitaji angalau mtu mmoja ndani yake. Badala yake futa rekodi ya familia, ambayo itauliza cha kufanya na kila kinachoirejelea.
family-error-no-family = Hakuna familia iliyochaguliwa. Hakuna kilichohifadhiwa.
family-error-already-child = Mtu huyu tayari ni mtoto wa familia hiyo.
pick-error-empty = Hakuna mtu aliyetajwa. Hakuna kilichohifadhiwa.
pick-error-not-found = Hakuna mtu wa jina hilo katika kumbukumbu hii. Hakuna kilichohifadhiwa.
pick-error-ambiguous = Zaidi ya mtu mmoja analingana na hilo. Chagua mmoja kutoka orodha ili rekodi iseme ni yupi. Hakuna kilichohifadhiwa.

links-editor-title = Viungo
links-editor-help = Mahusiano yasiyo ya kifamilia: mzazi wa ubatizo, mwajiri, shahidi, kikosi. Kila moja ni rekodi yake inayotaja watu wawili, kwa hiyo kuhariri hapa kunabadilisha kile rekodi nyingine inachoonyesha.
links-none = Hakuna kiungo kilichorekodiwa kwa mtu huyu.
links-new = Kiungo kipya
links-create = Unda kiungo
links-remove = Ondoa kiungo hiki
links-other-end = Upande mwingine
links-label = Ni nini
links-label-reverse = Kikisomwa kinyume
links-category = Kundi
links-bidirectional = Husomeka sawa pande zote mbili
links-from = Kuanzia
links-until = Hadi
links-reversed = Kiungo hiki kilichorwa kutoka rekodi nyingine. Kukihariri hapa kunabadilisha rekodi ileile.
link-error-no-label = Kiungo kinahitaji kusema ni nini. Hakuna kilichohifadhiwa.
occupations-editor-title = Kazi
occupations-editor-help = Kazi ni kipindi chenye mwanzo na mwisho, si cheo cha kazi tu. Kila moja ina tarehe na chanzo chake.
occupations-none = Hakuna kazi iliyorekodiwa kwa mtu huyu.
occupations-new = Kazi mpya
occupations-create = Unda kazi
occupations-remove = Ondoa kazi hii
occupations-title = Alichofanya
occupations-employer = Kwa ajili ya nani
occupations-employer-place = Alikokuwa
occupations-from = Kuanzia
occupations-until = Hadi
occupation-error-no-title = Kazi inahitaji kusema mtu alifanya nini. Hakuna kilichohifadhiwa.
link-category-spiritual = kiroho
link-category-professional = kikazi
link-category-social = kijamii
link-category-legal = kisheria
link-category-medical = kitabibu
link-category-educational = kielimu
link-category-conflict = mgogoro
link-category-other = jingine
links-edit-link = Hariri viungo
occupations-edit-link = Hariri kazi
family-edit-link = Hariri familia na mahusiano

events-editor-title = Matukio
events-editor-help = Tukio linataja watu kadhaa kwa mara moja — ndoa, ubatizo, sensa — kwa hiyo kila moja ni rekodi yake na linaonekana kwenye kila ukurasa linalotaja.
events-none = Hakuna tukio linalomtaja mtu huyu.
events-new = Tukio jipya
events-new-help = Mtu huyu huongezwa kama mhusika usipomtaja mwingine yeyote. Tukio lisilo na mtu ni tarehe tu.
events-create = Unda tukio
events-remove = Ondoa tukio hili
events-category = Kilichotokea
events-subcategory = Kwa usahihi zaidi
events-description = Maelezo
events-participants = Nani alikuwepo
events-participants-help = Kuhifadhi hii kunabadilisha rekodi ya tukio, ambayo kila mtu mwingine aliyetajwa ndani yake huonyesha pia.
events-who = Nani
event-error-no-category = Tukio linapaswa kusema kilichotokea. Hakuna kilichohifadhiwa.
documents-editor-title = Hati
documents-editor-help = Faili zipi rekodi hii inazielekeza, na kila moja ni nini kwake. Kufuta mstari kunatenganisha faili: hati na baiti zake zinabaki kwenye kumbukumbu.
documents-attached = Zimeambatishwa kwenye rekodi hii
documents-upload = Pakia faili
documents-upload-help = Hadi MB { $mb }. Faili huhifadhiwa kwenye kumbukumbu na kuambatishwa kwenye rekodi hii.
documents-caption = Maelezo mafupi
events-edit-link = Hariri matukio

prefs-style = Msongamano
style-comfortable = Starehe
style-comfortable-note = chaguo-msingi, na nafasi ya kusoma
style-compact = Kubanana
style-compact-note = sehemu kubwa ya rekodi kwa kila skrini, kwa kufanya kazi kwenye kadhaa
style-paper = Karatasi
style-paper-note = herufi zenye vikanyagio na mistari badala ya kadi, kwa kusoma mara moja au kuchapisha

admin-export-choose = Jumuisha katika uhamishaji huu
scope-health = Afya na imani
scope-biometrics = Bayometriki
scope-genomics = Data ya kijenetiki
scope-legal = Rekodi za uhalifu
scope-behaviour = Wasifu wa tabia wa watu walio hai
admin-export-with-chosen = Hamisha pamoja na vilivyowekwa alama

pg-identity = Utambulisho na hali ya kiraia
pg-identity-intro = Rekodi inasema mtu huyo alikuwa nani, na kile ambacho sajili za kiraia ziliandika.
pg-morphology = Umbo la mwili
pg-morphology-intro = Mwili kama ulivyopimwa na kuelezwa.
pg-biometrics = Bayometriki
pg-biometrics-intro = Sauti, mikono na hisia, na violezo vinavyomtambulisha mtu.
pg-health = Afya
pg-health-intro = Hali za kiafya, matibabu, vipimo na matokeo.
pg-genomics = Jenetiki
pg-genomics-intro = Vipimo vya DNA, haplogrupu, tofauti za jeni na matokeo mengine ya kimolekuli.
pg-death = Kifo
pg-death-intro = Jinsi, lini na wapi maisha yalikoma, na kilichoupata mwili.
pg-residence = Makazi na uraia
pg-residence-intro = Mtu aliishi wapi, nchi zipi zilimdai, na lugha alizozungumza.
pg-education = Elimu na kazi
pg-education-intro = Masomo, sifa, mapato na mali.
pg-military = Jeshi na heshima
pg-military-intro = Utumishi, vyeo, vikosi na tuzo.
pg-legal = Kisheria
pg-legal-intro = Mashauri ya jinai na matokeo yake.
pg-belief = Imani na uanachama
pg-belief-intro = Dini, ibada, imani na uanachama.
pg-personality = Haiba na tabia
pg-personality-intro = Hulka, mazoea na burudani, kama vyanzo vinavyozieleza.
pg-relationships = Mahusiano
pg-relationships-intro = Wazazi, wenzi, watoto na watu wengine katika maisha.
pg-digital-legacy = Urithi wa kidijitali
pg-digital-legacy-intro = Skani, mifano, rekodi za sauti na kumbukumbu zinazomwakilisha mtu.
pa-identity-titles = Vyeo
pa-identity-sex-at-birth = Jinsia wakati wa kuzaliwa
pa-identity-gender-identity = Utambulisho wa kijinsia
pa-birth-time = Saa ya kuzaliwa
pa-birth-coordinates = Mahali pa kuzaliwa, kama viwianishi
pa-civil-status-birth-certificate-number = Nambari ya cheti cha kuzaliwa
pa-civil-status-register-entries = Maandiko katika sajili ya kiraia
pa-civil-status-marginal-annotations = Maelezo ya pembeni
pa-morphology-height = Urefu
pa-morphology-weight = Uzito
pa-morphology-bmi = Kipimo cha uzito wa mwili
pa-morphology-body-composition = Muundo wa mwili
pa-morphology-build = Umbo
pa-morphology-eye-colour = Rangi ya macho
pa-morphology-eye-shape = Umbo la macho
pa-morphology-eye-spacing = Nafasi kati ya macho
pa-morphology-hair-colour = Rangi asilia ya nywele
pa-morphology-hair-texture = Aina ya nywele
pa-morphology-hairline = Mstari wa nywele
pa-morphology-facial-hair = Ndevu na masharubu
pa-morphology-body-hair = Nywele za mwili
pa-morphology-skin-tone = Rangi ya ngozi (Fitzpatrick)
pa-morphology-skin-undertone = Rangi ya ndani ya ngozi
pa-morphology-freckles = Madoadoa
pa-morphology-pigmentation = Alama za rangi kwenye ngozi
pa-morphology-scars = Makovu
pa-morphology-tattoos = Michoro ya ngozi
pa-morphology-moles = Fuko
pa-morphology-facial-asymmetries = Kutolingana kwa uso
pa-morphology-face-shape = Umbo la uso
pa-morphology-nose-shape = Umbo la pua
pa-morphology-ear-shape = Umbo la masikio
pa-morphology-lip-shape = Umbo la midomo
pa-morphology-dentition = Meno
pa-morphology-malocclusion = Kutokulingana kwa meno (daraja la Angle)
pa-morphology-posture = Mkao
pa-morphology-gait = Mwendo
pa-morphology-distinguishing-features = Alama za kutambulisha
pa-biometrics-fingerprints = Alama za vidole
pa-biometrics-retinal-print = Alama ya retina
pa-biometrics-voice-signature = Alama ya sauti
pa-biometrics-voice-frequency = Masafa ya msingi ya sauti
pa-biometrics-vocal-timbre = Rangi ya sauti
pa-biometrics-spoken-accent = Lafudhi
pa-biometrics-speech-rate = Kasi ya kuzungumza
pa-biometrics-verbal-tics = Mazoea ya maneno
pa-biometrics-frequent-vocabulary = Msamiati wa mara kwa mara
pa-biometrics-speech-register = Mtindo wa lugha
pa-biometrics-motor-tics = Mazoea ya mwili
pa-biometrics-handedness = Mkono anaoutumia zaidi
pa-biometrics-hearing = Kusikia
pa-biometrics-visual-acuity = Uwezo wa kuona
pa-biometrics-optical-correction = Marekebisho ya macho
pa-health-blood-group = Kundi la damu (ABO)
pa-health-rhesus = Rhesus (RhD)
pa-health-blood-pressure = Shinikizo la damu
pa-health-resting-heart-rate = Mapigo ya moyo wakati wa kupumzika
pa-health-respiratory-capacity = Uwezo wa kupumua
pa-health-conditions = Hali za kiafya
pa-health-surgeries = Historia ya upasuaji
pa-health-injuries = Majeraha
pa-health-deformities = Ulemavu wa umbo
pa-health-amputations = Kukatwa viungo
pa-health-prostheses = Viungo bandia
pa-health-implants = Vipandikizi
pa-health-devices = Vifaa vilivyopandikizwa
pa-health-medications = Dawa
pa-health-allergies = Mzio
pa-health-vaccinations = Chanjo
pa-health-serology = Seroloji
pa-health-lab-results = Matokeo ya maabara
pa-health-deficiencies = Upungufu
pa-health-sleep-disorders = Matatizo ya usingizi
pa-health-mental-health-assessments = Tathmini za afya ya akili
pa-genomics-autosomal-mapping = Kipimo cha DNA ya autosomu
pa-genomics-y-haplogroup = Haplogrupu ya Y-DNA
pa-genomics-mt-haplogroup = Haplogrupu ya mitokondria
pa-genomics-whole-genome-sequencing = Upangaji wa jenomu nzima
pa-genomics-risk-variants = Tofauti za jeni za hatari
pa-genomics-hereditary-conditions = Hali za kurithi

pa-genomics-predispositions = Mielekeo ya kurithi
pa-genomics-epigenetic-markers = Alama za epijenetiki
pa-genomics-epigenetic-age = Umri wa kiepijenetiki
pa-genomics-gut-microbiome = Vijidudu vya utumbo
pa-genomics-skin-microbiome = Vijidudu vya ngozi
pa-genomics-toxicological-sensitivities = Unyeti kwa dawa na sumu
pa-death-time = Saa ya kifo
pa-death-coordinates = Mahali pa kifo, kama viwianishi
pa-death-causes = Visababishi vya kifo
pa-death-contributing-factors = Mambo yaliyochangia
pa-death-autopsy = Uchunguzi wa maiti
pa-death-disposition = Kilichofanywa na mwili
pa-death-grave = Kaburi
pa-residence-addresses = Anwani
pa-residence-nationality-of-origin = Uraia wa asili
pa-residence-acquired-nationalities = Uraia uliopatikana
pa-residence-mother-tongue = Lugha ya mama
pa-residence-spoken-languages = Lugha anazozungumza
pa-education-level = Kiwango cha elimu
pa-education-diplomas = Stashahada na shahada
pa-education-institutions = Shule na taasisi
pa-education-income = Mapato
pa-education-real-estate = Mali isiyohamishika
pa-military-distinctions = Nishani
pa-military-citations = Pongezi rasmi
pa-military-ranks = Vyeo
pa-military-units = Vikosi
pa-military-service-numbers = Nambari za utumishi
pa-legal-criminal-record = Rekodi ya uhalifu
pa-belief-religions = Dini
pa-belief-sacraments = Sakramenti na ibada
pa-belief-beliefs = Imani
pa-belief-political-leanings = Mwelekeo wa kisiasa
pa-belief-memberships = Uanachama
pa-personality-big-five = Alama za Big Five
pa-personality-mbti = Aina ya MBTI
pa-personality-introversion-extraversion = Ukimya na uchangamfu
pa-personality-stress-tolerance = Ustahimilivu wa msongo
pa-personality-decision-style = Mtindo wa kufanya maamuzi
pa-personality-interests = Mambo anayoyapenda
pa-personality-hobbies = Burudani
pa-personality-sports = Michezo
pa-personality-dietary-habits = Lishe
pa-personality-dependencies = Uraibu
pa-digital-legacy-body-models = Mifano ya mwili
pa-digital-legacy-skin-textures = Michoro ya ngozi
pa-digital-legacy-rigs = Mifupa ya kidijitali
pa-digital-legacy-voice-corpora = Rekodi za sauti kwa usanisi
pa-digital-legacy-text-corpora = Maandishi kwa mfumo wa lugha
pa-digital-legacy-digital-traces = Nyayo za kidijitali
pa-digital-legacy-carbon-footprint = Kiwango cha kaboni
pa-digital-legacy-behaviour-models = Mifano ya tabia
pf-identity-titles-text = Cheo kama kilivyoandikwa
pf-identity-titles-kind = Aina ya cheo
pf-civil-status-marginal-annotations-text = Maelezo
pf-morphology-pigmentation-kind = Aina ya alama
pf-biometrics-spoken-accent-description = Inavyoelezwa
pf-biometrics-optical-correction-kind = Marekebisho
pf-health-amputations-level = Kiwango cha kukatwa
pf-health-prostheses-kind = Kiungo bandia
pf-health-implants-kind = Kipandikizi
pf-health-devices-kind = Kifaa
pf-health-allergies-type = Aina ya mzio
pf-health-vaccinations-status = Hali ya chanjo
pf-health-sleep-disorders-category = Kundi la tatizo
pf-death-autopsy-kind = Uchunguzi wa maiti
pf-education-institutions-name = Jina la taasisi
pf-military-distinctions-name = Jina la nishani
pf-military-distinctions-kind = Aina ya nishani
pf-military-citations-text = Pongezi
pf-military-ranks-category = Kundi la cheo
pf-belief-political-leanings-position = Nafasi kwenye mstari wa kushoto–kulia
pf-belief-memberships-kind = Aina ya shirika
pf-digital-legacy-carbon-footprint-method = Jinsi ilivyokadiriwa
pf-age-years = Umri kwa miaka
pf-agreeableness = Upole
pf-allergen = Kisababishi mzio
pf-amount = Kiasi
pf-analyte = Kinachopimwa
pf-artefact-type = Aina ya kitu
pf-autoimmune = Kinga dhidi ya mwili
pf-body-region = Sehemu ya mwili
pf-bone-percent = Mfupa
pf-carrier-status = Hali ya ubebaji
pf-cause = Sababu
pf-chronic = Sugu
pf-clock = Saa
pf-condition = Hali
pf-conferred-by = Ilitolewa na
pf-congenital = Ya kuzaliwa nayo
pf-conscientiousness = Uangalifu
pf-consent = Ridhaa
pf-coordinates = Viwianishi
pf-corrected = Kwa marekebisho
pf-country = Nchi
pf-court = Mahakama
pf-coverage = Upana wa usomaji
pf-currency = Sarafu
pf-decimal = Uwezo wa kuona (desimali)
pf-denomination = Dhehebu
pf-derived-from-id = Imetokana na
pf-description = Maelezo
pf-details = Undani
pf-diagnosis = Utambuzi
pf-diameter-mm = Kipenyo
pf-diastolic = Diastoli
pf-diet = Lishe
pf-document-id = Hati
pf-dose = Kipimo
pf-ear = Sikio
pf-entry-number = Nambari ya maandiko
pf-extraversion = Uchangamfu
pf-eye = Jicho
pf-fat-percent = Mafuta
pf-fev1-fvc-ratio = Uwiano wa FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = Muundo wa faili
pf-findings = Matokeo
pf-flag = Alama
pf-format = Muundo
pf-fracture = Kuvunjika
pf-fvc-litres = FVC
pf-gene = Jeni
pf-generator = Kimetengenezwa kwa
pf-grade = Daraja
pf-iccs-section = Kifungu cha kosa (ICCS)
pf-icd10-chapter = Sura ya ICD-10
pf-indication = Sababu ya matumizi
pf-inheritance = Urithi
pf-inscription = Maandishi
pf-institution = Taasisi
pf-instrument = Kifaa
pf-isced-level = Kiwango cha ISCED
pf-jurisdiction = Mamlaka
pf-language = Lugha
pf-lat = Latitudo
pf-level = Kiwango
pf-lines = Anwani
pf-location = Mahali
pf-lon = Longitudo
pf-major = Haplogrupu kuu
pf-marker = Alama
pf-metaboliser-status = Hali ya umetaboli
pf-method = Mbinu
pf-mode = Jinsi ilivyopatikana
pf-muscle-percent = Misuli
pf-neuroticism = Wasiwasi
pf-number = Nambari
pf-nutrient = Kirutubisho
pf-offence = Kosa
pf-office = Wadhifa
pf-openness = Uwazi
pf-organisation = Shirika
pf-outcome = Matokeo
pf-pace = Kasi ya kuzeeka
pf-page = Ukurasa
pf-panel = Jopo la vipimo
pf-party = Chama
pf-pathogen = Kimelea
pf-pattern = Mtindo wa matumizi
pf-percentile = Asilimia ya mpangilio
pf-period = Kipindi cha malipo
pf-place-id = Mahali
pf-plot = Kiwanja
pf-polygenic-score = Alama ya jeni nyingi
pf-postal-code = Msimbo wa posta
pf-precision = Usahihi
pf-prescription = Agizo la daktari
pf-proficiency = Ujuzi
pf-provider = Mtoa huduma
pf-quintile = Kundi la mapato
pf-rank = Cheo
pf-rank-text = Cheo kama kilivyoandikwa
pf-reaction = Mwitikio
pf-reference-build = Jenomu ya marejeo
pf-reference-high = Kiwango cha marejeo, juu
pf-reference-low = Kiwango cha marejeo, chini
pf-register-type = Aina ya maandiko
pf-result = Matokeo
pf-role = Nafasi
pf-sacrament = Sakramenti au ibada
pf-score = Alama
pf-sentence = Hukumu
pf-sequence = Nafasi katika mfuatano
pf-service = Huduma
pf-severity = Ukali
pf-shannon-diversity = Utofauti wa Shannon
pf-shape = Umbo
pf-significance = Umuhimu wa kitabibu
pf-snp-count = SNP zilizopimwa
pf-sport = Mchezo
pf-subclade = Tawi dogo
pf-substance = Dutu
pf-summary = Muhtasari
pf-systolic = Sistoli
pf-tenure = Umiliki
pf-test = Kipimo
pf-threshold-db = Kiwango cha kusikia
pf-title = Kichwa
pf-tonnes-co2e-per-year = Uzalishaji wa hewa
pf-tradition = Mapokeo
pf-tree-version = Toleo la mti
pf-unit = Kipimo
pf-use = Matumizi
pf-variant = Tofauti
pf-volume = Juzuu
pf-zygosity = Zaigositi
pu-cm = sm { $n }
pu-kg = kg { $n }
pu-kg-m2 = { $n } kg/m²
pu-percent = asilimia { $n }
pu-mm = mm { $n }
pu-hz = { $n } Hz
pu-words-min = maneno { $n } kwa dakika
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = mapigo { $n } kwa dakika
pu-litres = lita { $n }
pu-coverage = { $n }×
pu-years = miaka { $n }
pu-t-co2e-yr = tani { $n } za CO₂e kwa mwaka
pv-sensitive-class-health = Afya na imani
pv-sensitive-class-biometrics = Bayometriki
pv-sensitive-class-genomics = Data ya kijenetiki
pv-sensitive-class-legal = Rekodi za uhalifu
pv-laterality-left = Kushoto
pv-laterality-right = Kulia
pv-laterality-both = Pande zote mbili
pv-body-region-head = Kichwa
pv-body-region-face = Uso
pv-body-region-neck = Shingo
pv-body-region-left-shoulder = Bega la kushoto
pv-body-region-right-shoulder = Bega la kulia
pv-body-region-left-arm = Mkono wa kushoto
pv-body-region-right-arm = Mkono wa kulia
pv-body-region-left-hand = Kiganja cha kushoto
pv-body-region-right-hand = Kiganja cha kulia
pv-body-region-chest = Kifua
pv-body-region-abdomen = Tumbo
pv-body-region-upper-back = Mgongo wa juu
pv-body-region-lower-back = Mgongo wa chini
pv-body-region-pelvis = Nyonga
pv-body-region-left-leg = Mguu wa kushoto
pv-body-region-right-leg = Mguu wa kulia
pv-body-region-left-foot = Wayo wa kushoto
pv-body-region-right-foot = Wayo wa kulia
pv-body-region-internal = Ndani ya mwili
pv-body-region-whole-body = Mwili mzima
pv-body-region-other = Sehemu nyingine
pv-artefact-type-mesh = Matundu ya 3D
pv-artefact-type-point-cloud = Wingu la nukta
pv-artefact-type-skin-texture-map = Ramani ya ngozi
pv-artefact-type-skeletal-rig = Mifupa ya kidijitali
pv-artefact-type-voice-corpus = Mkusanyiko wa sauti
pv-artefact-type-text-corpus = Mkusanyiko wa maandishi
pv-artefact-type-trace-archive = Kumbukumbu ya shughuli mtandaoni
pv-artefact-type-behaviour-model = Mfano wa tabia
pv-artefact-type-fingerprint-card = Kadi ya alama za vidole
pv-artefact-type-fingerprint-template = Kiolezo cha alama ya kidole
pv-artefact-type-retinal-image = Picha ya retina
pv-artefact-type-voiceprint = Alama ya sauti
pv-consent-given = Imetolewa
pv-consent-given-by-estate = Imetolewa na wasimamizi wa mirathi
pv-consent-refused = Imekataliwa
pv-consent-withdrawn = Imeondolewa
pv-consent-not-asked = Haikuombwa
pv-consent-unknown = Haijulikani
pv-sex-at-birth-female = Ke
pv-sex-at-birth-male = Me
pv-sex-at-birth-intersex = Jinsia mbili
pv-sex-at-birth-undetermined = Haikubainishwa
pv-sex-at-birth-unknown = Haijulikani
pv-gender-identity-woman = Mwanamke
pv-gender-identity-man = Mwanamume
pv-gender-identity-non-binary = Si ya pande mbili
pv-gender-identity-other = Nyingine
pv-gender-identity-undisclosed = Haikuelezwa
pv-gender-identity-unknown = Haijulikani
pv-title-kind-nobility = Ukoo wa kifalme
pv-title-kind-academic = Kitaaluma
pv-title-kind-professional = Kikazi
pv-title-kind-religious = Kidini
pv-title-kind-military = Kijeshi
pv-title-kind-civic = Kiraia au cha heshima
pv-title-kind-courtesy = Cha adabu
pv-title-kind-other = Nyingine
pv-register-type-birth = Kuzaliwa
pv-register-type-baptism = Ubatizo
pv-register-type-marriage = Ndoa
pv-register-type-death = Kifo
pv-register-type-burial = Mazishi
pv-register-type-divorce = Talaka
pv-register-type-recognition = Kumtambua mtoto
pv-register-type-legitimation = Kuhalalisha mtoto
pv-register-type-adoption = Kuasili
pv-register-type-name-change = Kubadilisha jina
pv-register-type-other = Nyingine
pv-build-slight = Mwembamba sana
pv-build-slim = Mwembamba
pv-build-average = Wastani

pv-build-sturdy = Imara
pv-build-stout = Mnene kiasi
pv-build-heavy = Mnene
pv-eye-colour-light-blue = Samawati hafifu
pv-eye-colour-blue = Samawati
pv-eye-colour-dark-blue = Samawati iliyokolea
pv-eye-colour-grey = Kijivu
pv-eye-colour-blue-grey = Samawati-kijivu
pv-eye-colour-green = Kijani
pv-eye-colour-grey-green = Kijivu-kijani
pv-eye-colour-hazel = Hudhurungi-kijani
pv-eye-colour-amber = Kaharabu
pv-eye-colour-light-brown = Kahawia hafifu
pv-eye-colour-brown = Kahawia
pv-eye-colour-dark-brown = Kahawia iliyokolea
pv-eye-colour-black = Nyeusi
pv-eye-colour-mixed = Mchanganyiko
pv-eye-colour-other = Nyingine
pv-eye-shape-almond = Kama lozi
pv-eye-shape-round = Duara
pv-eye-shape-hooded = Kope zilizofunika
pv-eye-shape-monolid = Kope moja
pv-eye-shape-deep-set = Yaliyozama
pv-eye-shape-protruding = Yaliyotokeza
pv-eye-shape-upturned = Yaliyoinuka pembeni
pv-eye-shape-downturned = Yaliyoshuka pembeni
pv-eye-shape-other = Nyingine
pv-eye-spacing-close-set = Karibu karibu
pv-eye-spacing-average = Wastani
pv-eye-spacing-wide-set = Mbali mbali
pv-hair-colour-black = Nyeusi
pv-hair-colour-dark-brown = Kahawia iliyokolea
pv-hair-colour-brown = Kahawia
pv-hair-colour-light-brown = Kahawia hafifu
pv-hair-colour-auburn = Kahawia-nyekundu
pv-hair-colour-red = Nyekundu
pv-hair-colour-strawberry-blond = Manjano-nyekundu
pv-hair-colour-dark-blond = Manjano iliyokolea
pv-hair-colour-blond = Manjano
pv-hair-colour-light-blond = Manjano hafifu
pv-hair-colour-grey = Mvi
pv-hair-colour-white = Nyeupe
pv-hair-colour-none = Bila nywele
pv-hair-colour-other = Nyingine
pv-hair-texture-straight = Singa
pv-hair-texture-wavy = Za mawimbi
pv-hair-texture-curly = Za mapindo
pv-hair-texture-coily = Za kipilipili
pv-hair-texture-other = Nyingine
pv-hairline-straight = Mnyoofu
pv-hairline-rounded = Wa mviringo
pv-hairline-widows-peak = Ncha katikati ya paji
pv-hairline-m-shaped = Umbo la M
pv-hairline-bell-shaped = Umbo la kengele
pv-hairline-uneven = Usiolingana
pv-hairline-receding = Unaorudi nyuma
pv-hairline-bald = Upara
pv-facial-hair-none = Hakuna
pv-facial-hair-stubble = Ndevu fupi
pv-facial-hair-moustache = Masharubu
pv-facial-hair-goatee = Kidevu cha mbuzi
pv-facial-hair-full-beard = Ndevu kamili
pv-facial-hair-sideburns = Ndevu za mashavu
pv-facial-hair-other = Nyingine
pv-body-hair-none = Hakuna
pv-body-hair-sparse = Chache
pv-body-hair-moderate = Wastani
pv-body-hair-dense = Nyingi
pv-skin-tone-type-i = Aina I — huungua kila mara, haibadiliki rangi
pv-skin-tone-type-ii = Aina II — kwa kawaida huungua, hubadilika kidogo
pv-skin-tone-type-iii = Aina III — wakati mwingine huungua, hubadilika sawasawa
pv-skin-tone-type-iv = Aina IV — mara chache huungua, hubadilika vizuri
pv-skin-tone-type-v = Aina V — mara chache sana huungua
pv-skin-tone-type-vi = Aina VI — haiungui kamwe
pv-skin-undertone-cool = Baridi
pv-skin-undertone-neutral = Wastani
pv-skin-undertone-warm = Joto
pv-skin-undertone-olive = Zeituni
pv-freckles-none = Hakuna
pv-freckles-few = Machache
pv-freckles-moderate = Wastani
pv-freckles-many = Mengi
pv-pigmentation-mark-birthmark = Alama ya kuzaliwa
pv-pigmentation-mark-port-wine-stain = Doa la rangi ya divai
pv-pigmentation-mark-cafe-au-lait-spot = Doa la rangi ya kahawa
pv-pigmentation-mark-depigmented-patch = Doa lisilo na rangi
pv-pigmentation-mark-hyperpigmented-patch = Doa lenye rangi nyingi
pv-pigmentation-mark-other = Nyingine
pv-mole-shape-round = Duara
pv-mole-shape-oval = Yai
pv-mole-shape-irregular = Isiyo na umbo maalum
pv-mole-shape-other = Nyingine
pv-face-shape-oval = Umbo la yai
pv-face-shape-round = Duara
pv-face-shape-square = Mraba
pv-face-shape-oblong = Mrefu
pv-face-shape-heart = Umbo la moyo
pv-face-shape-diamond = Umbo la almasi
pv-face-shape-triangular = Pembetatu
pv-nose-shape-straight = Nyoofu
pv-nose-shape-aquiline = Ya kinda
pv-nose-shape-snub = Fupi
pv-nose-shape-upturned = Iliyoinuka
pv-nose-shape-flat = Bapa
pv-nose-shape-broad = Pana
pv-nose-shape-bulbous = Ya duara
pv-nose-shape-crooked = Iliyopinda
pv-nose-shape-other = Nyingine
pv-ear-shape-free-lobe = Ndewe huru
pv-ear-shape-attached-lobe = Ndewe zilizoshikamana
pv-ear-shape-protruding = Yaliyotokeza
pv-ear-shape-close-set = Karibu na kichwa
pv-ear-shape-pointed = Yenye ncha
pv-ear-shape-other = Nyingine
pv-lip-shape-thin = Myembamba
pv-lip-shape-medium = Ya wastani
pv-lip-shape-full = Minene
pv-lip-shape-bow-shaped = Umbo la upinde
pv-lip-shape-wide = Mipana
pv-lip-shape-downturned = Iliyoshuka pembeni
pv-lip-shape-other = Nyingine
pv-dentition-primary = Meno ya utotoni
pv-dentition-mixed = Mchanganyiko
pv-dentition-permanent-complete = Ya kudumu, kamili
pv-dentition-permanent-partial-loss = Ya kudumu, mengine yametoka
pv-dentition-edentulous = Hakuna meno asilia
pv-dentition-partial-denture = Meno bandia ya sehemu
pv-dentition-full-denture = Meno bandia kamili
pv-dentition-implants = Vipandikizi vya meno
pv-malocclusion-normal = Mlingano wa kawaida
pv-malocclusion-class-i = Daraja I
pv-malocclusion-class-ii-division-1 = Daraja II, kundi 1
pv-malocclusion-class-ii-division-2 = Daraja II, kundi 2
pv-malocclusion-class-iii = Daraja III
pv-posture-ideal = Bora
pv-posture-kyphotic-lordotic = Kifosisi-lordosisi
pv-posture-flat-back = Mgongo bapa
pv-posture-sway-back = Mgongo uliobonyea
pv-posture-stooped = Ameinama
pv-posture-scoliotic = Mgongo uliopinda pembeni
pv-posture-other = Nyingine
pv-gait-brisk = Wa haraka
pv-gait-average = Wastani
pv-gait-slow = Wa polepole
pv-gait-shuffling = Wa kukokota miguu
pv-gait-limping = Wa kuchechemea
pv-gait-waddling = Wa kuyumbayumba
pv-gait-unsteady = Usio thabiti
pv-gait-stiff = Mgumu
pv-gait-other = Nyingine
pv-vocal-timbre-bright = Angavu
pv-vocal-timbre-dark = Nzito
pv-vocal-timbre-warm = Ya joto
pv-vocal-timbre-breathy = Ya pumzi
pv-vocal-timbre-nasal = Ya puani
pv-vocal-timbre-hoarse = Iliyokwaruza
pv-vocal-timbre-resonant = Inayovuma
pv-vocal-timbre-thin = Nyembamba
pv-vocal-timbre-other = Nyingine
pv-speech-register-frozen = Ya kimapokeo
pv-speech-register-formal = Rasmi
pv-speech-register-consultative = Ya kushauriana
pv-speech-register-casual = Ya kawaida
pv-speech-register-intimate = Ya karibu
pv-handedness-left = Mkono wa kushoto
pv-handedness-right = Mkono wa kulia
pv-handedness-ambidextrous = Mikono yote miwili
pv-handedness-mixed = Mchanganyiko
pv-handedness-unknown = Haijulikani
pv-hearing-grade-normal = Kawaida
pv-hearing-grade-mild = Kidogo
pv-hearing-grade-moderate = Wastani
pv-hearing-grade-moderately-severe = Kali kiasi
pv-hearing-grade-severe = Kali
pv-hearing-grade-profound = Kali sana
pv-hearing-grade-complete = Kamili
pv-optical-correction-none = Hakuna
pv-optical-correction-glasses = Miwani
pv-optical-correction-contact-lenses = Lenzi za macho
pv-optical-correction-glasses-and-contact-lenses = Miwani na lenzi za macho
pv-optical-correction-refractive-surgery = Upasuaji wa kurekebisha uoni
pv-optical-correction-intraocular-lens = Lenzi ndani ya jicho
pv-optical-correction-other = Nyingine
pv-rhesus-positive = RhD chanya
pv-rhesus-negative = RhD hasi
pv-rhesus-weak-d = D dhaifu
pv-rhesus-unknown = Haijulikani
pv-icd10-chapter-infectious-parasitic = I Magonjwa ya kuambukiza na ya vimelea
pv-icd10-chapter-neoplasms = II Uvimbe
pv-icd10-chapter-blood-immune = III Magonjwa ya damu na kinga
pv-icd10-chapter-endocrine-metabolic = IV Homoni, lishe na umetaboli
pv-icd10-chapter-mental-behavioural = V Matatizo ya akili na tabia
pv-icd10-chapter-nervous-system = VI Mfumo wa neva
pv-icd10-chapter-eye-adnexa = VII Jicho na viungo vyake
pv-icd10-chapter-ear-mastoid = VIII Sikio na mfupa wa mastoidi
pv-icd10-chapter-circulatory = IX Mfumo wa mzunguko wa damu
pv-icd10-chapter-respiratory = X Mfumo wa kupumua
pv-icd10-chapter-digestive = XI Mfumo wa kumeng'enya chakula
pv-icd10-chapter-skin = XII Ngozi na tishu za chini ya ngozi
pv-icd10-chapter-musculoskeletal = XIII Mfumo wa misuli na mifupa
pv-icd10-chapter-genitourinary = XIV Mfumo wa mkojo na uzazi
pv-icd10-chapter-pregnancy-childbirth = XV Ujauzito na kujifungua
pv-icd10-chapter-perinatal = XVI Hali za wakati wa kuzaliwa
pv-icd10-chapter-congenital = XVII Ulemavu wa kuzaliwa nao
pv-icd10-chapter-ill-defined = XVIII Dalili na sababu zisizobainishwa
pv-icd10-chapter-injury-poisoning = XIX Majeraha na sumu
pv-icd10-chapter-external-causes = XX Sababu za nje
pv-icd10-chapter-health-factors = XXI Mambo yanayoathiri afya
pv-icd10-chapter-special-purposes = XXII Misimbo kwa madhumuni maalum
pv-diagnosis-status-diagnosed = Imethibitishwa
pv-diagnosis-status-suspected = Inashukiwa
pv-diagnosis-status-self-reported = Ameripoti mwenyewe
pv-diagnosis-status-unknown = Haijulikani
pv-prosthesis-kind-limb = Kiungo
pv-prosthesis-kind-joint = Kiungio bandia
pv-prosthesis-kind-ocular = Jicho
pv-prosthesis-kind-dental = Meno
pv-prosthesis-kind-auditory = Kusikia
pv-prosthesis-kind-breast = Titi
pv-prosthesis-kind-other = Nyingine
pv-implant-kind-orthopaedic = Mifupa
pv-implant-kind-dental = Meno
pv-implant-kind-cochlear = Kipandikizi cha kusikia
pv-implant-kind-breast = Titi
pv-implant-kind-intraocular-lens = Lenzi ndani ya jicho
pv-implant-kind-contraceptive = Uzazi wa mpango
pv-implant-kind-cosmetic = Urembo
pv-implant-kind-other = Nyingine
pv-device-kind-pacemaker = Kidhibiti mapigo ya moyo
pv-device-kind-implantable-defibrillator = Kishtua moyo kilichopandikizwa
pv-device-kind-cardiac-resynchronisation = Kifaa cha kuoanisha moyo
pv-device-kind-ventricular-assist = Kifaa cha kusaidia moyo
pv-device-kind-neurostimulator = Kichocheo cha neva
pv-device-kind-insulin-pump = Pampu ya insulini
pv-device-kind-drug-port = Mlango wa kutolea dawa
pv-device-kind-shunt = Mrija wa kupitisha maji
pv-device-kind-stent = Stenti
pv-device-kind-other = Nyingine
pv-allergy-type-drug = Dawa
pv-allergy-type-food = Chakula
pv-allergy-type-environmental = Mazingira
pv-allergy-type-insect-venom = Sumu ya wadudu
pv-allergy-type-latex = Mpira
pv-allergy-type-other = Nyingine
pv-allergy-severity-mild = Kidogo
pv-allergy-severity-moderate = Wastani
pv-allergy-severity-severe = Kali
pv-allergy-severity-anaphylactic = Mshtuko wa mzio
pv-allergy-severity-unknown = Haijulikani
pv-pathogen-diphtheria = Dondakoo
pv-pathogen-tetanus = Pepopunda
pv-pathogen-pertussis = Kifaduro
pv-pathogen-poliomyelitis = Polio
pv-pathogen-measles = Surua
pv-pathogen-mumps = Matubwitubwi
pv-pathogen-rubella = Rubela
pv-pathogen-varicella = Tetekuwanga
pv-pathogen-smallpox = Ndui
pv-pathogen-tuberculosis = Kifua kikuu
pv-pathogen-hepatitis-a = Homa ya ini A
pv-pathogen-hepatitis-b = Homa ya ini B
pv-pathogen-hepatitis-c = Homa ya ini C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae aina b
pv-pathogen-pneumococcal = Ugonjwa wa nimokoka
pv-pathogen-meningococcal = Ugonjwa wa meningokoka
pv-pathogen-human-papillomavirus = Virusi vya papiloma
pv-pathogen-influenza = Mafua makali
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotavirusi
pv-pathogen-yellow-fever = Homa ya manjano
pv-pathogen-typhoid = Homa ya matumbo
pv-pathogen-cholera = Kipindupindu
pv-pathogen-rabies = Kichaa cha mbwa
pv-pathogen-japanese-encephalitis = Homa ya ubongo ya Kijapani
pv-pathogen-tick-borne-encephalitis = Homa ya ubongo ya kupe
pv-pathogen-hiv = VVU
pv-pathogen-syphilis = Kaswende
pv-pathogen-toxoplasmosis = Toksoplasmosisi
pv-pathogen-cytomegalovirus = Saitomegalovirusi
pv-pathogen-epstein-barr = Virusi vya Epstein–Barr
pv-pathogen-other = Nyingine
pv-vaccination-status-vaccinated = Amechanjwa
pv-vaccination-status-partially-vaccinated = Amechanjwa kwa sehemu
pv-vaccination-status-unvaccinated = Hajachanjwa
pv-vaccination-status-contraindicated = Hapaswi kuchanjwa
pv-vaccination-status-unknown = Haijulikani
pv-serology-result-positive = Chanya
pv-serology-result-negative = Hasi
pv-serology-result-equivocal = Haieleweki
pv-serology-result-unknown = Haijulikani
pv-lab-panel-basic-metabolic = Vipimo vya msingi vya umetaboli
pv-lab-panel-lipid = Vipimo vya mafuta ya damu
pv-lab-panel-liver = Vipimo vya ini
pv-lab-panel-renal = Vipimo vya figo
pv-lab-panel-glycated-haemoglobin = Himoglobini ya sukari
pv-lab-panel-iron = Vipimo vya chuma
pv-lab-analyte-sodium = Sodiamu
pv-lab-analyte-potassium = Potasiamu
pv-lab-analyte-chloride = Kloridi
pv-lab-analyte-bicarbonate = Bikabonati

pv-lab-analyte-urea = Urea
pv-lab-analyte-creatinine = Kreatinini
pv-lab-analyte-glucose = Glukosi
pv-lab-analyte-calcium = Kalsiamu
pv-lab-analyte-total-cholesterol = Kolesteroli yote
pv-lab-analyte-ldl-cholesterol = Kolesteroli LDL
pv-lab-analyte-hdl-cholesterol = Kolesteroli HDL
pv-lab-analyte-triglycerides = Trigliseridi
pv-lab-analyte-non-hdl-cholesterol = Kolesteroli isiyo HDL
pv-lab-analyte-alt = Alanini aminotransferasi (ALT)
pv-lab-analyte-ast = Aspartati aminotransferasi (AST)
pv-lab-analyte-alp = Fosfatasi ya alkali (ALP)
pv-lab-analyte-ggt = Gama-glutamili transferasi (GGT)
pv-lab-analyte-total-bilirubin = Bilirubini yote
pv-lab-analyte-direct-bilirubin = Bilirubini ya moja kwa moja
pv-lab-analyte-albumin = Albumini
pv-lab-analyte-total-protein = Protini yote
pv-lab-analyte-egfr = GFR inayokadiriwa
pv-lab-analyte-uric-acid = Asidi ya mkojo
pv-lab-analyte-phosphate = Fosfati
pv-lab-analyte-urine-albumin-creatinine-ratio = Uwiano wa albumini na kreatinini kwenye mkojo
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Chuma kwenye seramu
pv-lab-analyte-ferritin = Feritini
pv-lab-analyte-transferrin = Transferini
pv-lab-analyte-transferrin-saturation = Ujazo wa transferini
pv-lab-analyte-tibc = Uwezo wote wa kubeba chuma
pv-lab-flag-low = Chini
pv-lab-flag-normal = Kawaida
pv-lab-flag-high = Juu
pv-lab-flag-critical-low = Chini kwa hatari
pv-lab-flag-critical-high = Juu kwa hatari
pv-nutrient-vitamin-a = Vitamini A
pv-nutrient-thiamine = Thiamini (B1)
pv-nutrient-riboflavin = Riboflavini (B2)
pv-nutrient-niacin = Niasini (B3)
pv-nutrient-vitamin-b6 = Vitamini B6
pv-nutrient-folate = Folati (B9)
pv-nutrient-vitamin-b12 = Vitamini B12
pv-nutrient-vitamin-c = Vitamini C
pv-nutrient-vitamin-d = Vitamini D
pv-nutrient-vitamin-e = Vitamini E
pv-nutrient-vitamin-k = Vitamini K
pv-nutrient-iron = Chuma
pv-nutrient-zinc = Zinki
pv-nutrient-magnesium = Magnesiamu
pv-nutrient-calcium = Kalsiamu
pv-nutrient-iodine = Ayodini
pv-nutrient-selenium = Seleniamu
pv-nutrient-copper = Shaba
pv-nutrient-potassium = Potasiamu
pv-nutrient-phosphorus = Fosforasi
pv-nutrient-other = Nyingine
pv-sleep-disorder-insomnia = Kukosa usingizi
pv-sleep-disorder-sleep-related-breathing = Tatizo la kupumua wakati wa usingizi
pv-sleep-disorder-central-hypersomnolence = Usingizi mwingi kupita kiasi
pv-sleep-disorder-circadian-rhythm = Tatizo la mzunguko wa mchana na usiku
pv-sleep-disorder-parasomnia = Parasomnia
pv-sleep-disorder-sleep-related-movement = Tatizo la mwendo wakati wa usingizi
pv-sleep-disorder-other = Nyingine
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Mahojiano ya kitabibu
pv-assessment-instrument-other = Nyingine
pv-assessment-severity-none-minimal = Hakuna au kidogo sana
pv-assessment-severity-mild = Kidogo
pv-assessment-severity-moderate = Wastani
pv-assessment-severity-moderately-severe = Kali kiasi
pv-assessment-severity-severe = Kali
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Data ghafi ya mikroarei
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Nyingine
pv-zygosity-heterozygous = Heterozaigoti
pv-zygosity-homozygous = Homozaigoti
pv-zygosity-hemizygous = Hemizaigoti
pv-zygosity-compound-heterozygous = Heterozaigoti changamano
pv-clinical-significance-pathogenic = Husababisha ugonjwa
pv-clinical-significance-likely-pathogenic = Huenda husababisha ugonjwa
pv-clinical-significance-uncertain-significance = Umuhimu usio na uhakika
pv-clinical-significance-likely-benign = Huenda haina madhara
pv-clinical-significance-benign = Haina madhara
pv-inheritance-pattern-autosomal-dominant = Autosomu inayotawala
pv-inheritance-pattern-autosomal-recessive = Autosomu iliyojificha
pv-inheritance-pattern-x-linked-dominant = Inayotawala, kupitia X
pv-inheritance-pattern-x-linked-recessive = Iliyojificha, kupitia X
pv-inheritance-pattern-y-linked = Kupitia Y
pv-inheritance-pattern-mitochondrial = Mitokondria
pv-inheritance-pattern-multifactorial = Visababishi vingi
pv-inheritance-pattern-unknown = Haijulikani
pv-carrier-status-affected = Ameathirika
pv-carrier-status-carrier = Mbebaji
pv-carrier-status-not-carrier = Si mbebaji
pv-carrier-status-unknown = Haijulikani
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Nyingine
pv-metaboliser-status-poor = Huvunja dawa polepole sana
pv-metaboliser-status-intermediate = Huvunja dawa kwa kasi ya kati
pv-metaboliser-status-normal = Huvunja dawa kwa kawaida
pv-metaboliser-status-rapid = Huvunja dawa haraka
pv-metaboliser-status-ultrarapid = Huvunja dawa haraka sana
pv-autopsy-not-performed = Haukufanyika
pv-autopsy-clinical = Wa kitabibu
pv-autopsy-forensic = Wa kijinai
pv-autopsy-external-examination = Uchunguzi wa nje tu
pv-autopsy-unknown = Haijulikani
pv-disposition-burial = Mazishi
pv-disposition-cremation = Kuchomwa
pv-disposition-entombment = Kuwekwa kaburini la jengo
pv-disposition-burial-at-sea = Mazishi baharini
pv-disposition-natural-burial = Mazishi ya asili
pv-disposition-body-donation = Ulitolewa kwa sayansi
pv-disposition-other = Nyingine
pv-disposition-unknown = Haijulikani
pv-address-use-principal = Makazi makuu
pv-address-use-secondary = Makazi ya pili
pv-address-use-temporary = Makazi ya muda
pv-address-use-postal = Anwani ya posta
pv-address-use-other = Nyingine
pv-nationality-mode-descent = Kwa nasaba
pv-nationality-mode-birth-in-territory = Kwa kuzaliwa nchini
pv-nationality-mode-naturalisation = Kwa kuandikishwa uraia
pv-nationality-mode-marriage = Kwa ndoa
pv-nationality-mode-registration = Kwa usajili
pv-nationality-mode-restoration = Kwa kurejeshewa
pv-nationality-mode-state-succession = Kwa mabadiliko ya mamlaka ya nchi
pv-nationality-mode-other = Nyingine
pv-language-proficiency-a1 = A1 Mwanzo
pv-language-proficiency-a2 = A2 Msingi
pv-language-proficiency-b1 = B1 Kati
pv-language-proficiency-b2 = B2 Kati ya juu
pv-language-proficiency-c1 = C1 Juu
pv-language-proficiency-c2 = C2 Umahiri
pv-language-proficiency-native = Lugha ya kwanza
pv-isced-level-isced-0 = 0 Elimu ya awali
pv-isced-level-isced-1 = 1 Msingi
pv-isced-level-isced-2 = 2 Sekondari ya chini
pv-isced-level-isced-3 = 3 Sekondari ya juu
pv-isced-level-isced-4 = 4 Baada ya sekondari, si chuo kikuu
pv-isced-level-isced-5 = 5 Chuo cha muda mfupi
pv-isced-level-isced-6 = 6 Shahada ya kwanza au sawa nayo
pv-isced-level-isced-7 = 7 Shahada ya uzamili au sawa nayo
pv-isced-level-isced-8 = 8 Shahada ya uzamivu au sawa nayo
pv-income-quintile-q1 = Sehemu ya tano ya chini
pv-income-quintile-q2 = Sehemu ya tano ya pili
pv-income-quintile-q3 = Sehemu ya tano ya kati
pv-income-quintile-q4 = Sehemu ya tano ya nne
pv-income-quintile-q5 = Sehemu ya tano ya juu
pv-pay-period-hourly = Kwa saa
pv-pay-period-daily = Kwa siku
pv-pay-period-weekly = Kwa wiki
pv-pay-period-monthly = Kwa mwezi
pv-pay-period-annual = Kwa mwaka
pv-tenure-owned = Anamiliki
pv-tenure-co-owned = Wanamiliki pamoja
pv-tenure-leasehold = Kwa pango la muda mrefu
pv-tenure-rented = Kwa kupanga
pv-tenure-usufruct = Haki ya kutumia
pv-tenure-other = Nyingine
pv-distinction-kind-order = Agizo la heshima
pv-distinction-kind-decoration = Pambo la heshima
pv-distinction-kind-medal = Medali
pv-distinction-kind-title = Cheo cha heshima
pv-distinction-kind-other = Nyingine
pv-military-service-army = Jeshi la nchi kavu
pv-military-service-navy = Jeshi la majini
pv-military-service-air-force = Jeshi la anga
pv-military-service-marines = Wanamaji wa vita
pv-military-service-gendarmerie = Polisi wa kijeshi
pv-military-service-border-guard = Walinzi wa mpaka
pv-military-service-national-guard = Walinzi wa taifa
pv-military-service-other = Nyingine
pv-rank-category-enlisted = Askari
pv-rank-category-non-commissioned = Afisa asiye na kamisheni
pv-rank-category-warrant = Afisa wa waranti
pv-rank-category-officer-cadet = Kadeti
pv-rank-category-junior-officer = Afisa wa ngazi ya chini
pv-rank-category-senior-officer = Afisa wa ngazi ya juu
pv-rank-category-general-officer = Jenerali
pv-iccs-section-acts-leading-to-death = 01 Vitendo vilivyosababisha kifo
pv-iccs-section-acts-causing-harm = 02 Vitendo vilivyosababisha madhara
pv-iccs-section-sexual-acts = 03 Vitendo vya kingono vyenye madhara
pv-iccs-section-property-with-violence = 04 Dhidi ya mali, kwa nguvu
pv-iccs-section-property-only = 05 Dhidi ya mali tu
pv-iccs-section-controlled-substances = 06 Dawa zinazodhibitiwa
pv-iccs-section-fraud-deception-corruption = 07 Ulaghai, udanganyifu au rushwa
pv-iccs-section-public-order-and-state = 08 Dhidi ya utulivu wa umma na Dola
pv-iccs-section-public-safety-and-security = 09 Dhidi ya usalama wa umma
pv-iccs-section-natural-environment = 10 Dhidi ya mazingira asilia
pv-iccs-section-other-criminal-acts = 11 Vitendo vingine vya jinai
pv-case-outcome-convicted = Alitiwa hatiani
pv-case-outcome-acquitted = Aliachiliwa huru
pv-case-outcome-dismissed = Kesi ilifutwa
pv-case-outcome-conviction-quashed = Hukumu ilibatilishwa
pv-case-outcome-pardoned = Alisamehewa
pv-case-outcome-amnestied = Alipata msamaha wa jumla
pv-case-outcome-expunged = Iliondolewa kwenye rekodi
pv-case-outcome-pending = Inasubiri
pv-case-outcome-unknown = Haijulikani
pv-religion-buddhism = Ubudha
pv-religion-christianity-catholic = Ukristo: Katoliki
pv-religion-christianity-orthodox = Ukristo: Orthodoksi
pv-religion-christianity-protestant = Ukristo: Kiprotestanti
pv-religion-christianity-other = Ukristo: mwingine
pv-religion-hinduism = Uhindu
pv-religion-islam-sunni = Uislamu: Suni
pv-religion-islam-shia = Uislamu: Shia
pv-religion-islam-other = Uislamu: mwingine
pv-religion-jainism = Ujaini
pv-religion-judaism = Uyahudi
pv-religion-sikhism = Usikh
pv-religion-bahai = Imani ya Kibahai
pv-religion-shinto = Shinto
pv-religion-taoism = Utao
pv-religion-zoroastrianism = Uzoroasta
pv-religion-traditional = Dini ya jadi
pv-religion-other = Nyingine
pv-religion-none = Hana dini
pv-religion-unknown = Haijulikani
pv-sacrament-baptism = Ubatizo
pv-sacrament-confirmation = Kipaimara
pv-sacrament-first-communion = Komunyo ya kwanza
pv-sacrament-reconciliation = Kitubio
pv-sacrament-anointing-of-the-sick = Mpako wa wagonjwa
pv-sacrament-holy-orders = Daraja takatifu
pv-sacrament-matrimony = Ndoa
pv-sacrament-other-rite = Ibada nyingine
pv-political-position-far-left = Mrengo wa kushoto kabisa
pv-political-position-left = Kushoto
pv-political-position-centre-left = Kati-kushoto
pv-political-position-centre = Kati
pv-political-position-centre-right = Kati-kulia
pv-political-position-right = Kulia
pv-political-position-far-right = Mrengo wa kulia kabisa
pv-political-position-apolitical = Hajihusishi na siasa
pv-political-position-other = Nje ya mstari huu
pv-political-position-unknown = Haijulikani
pv-membership-kind-trade-union = Chama cha wafanyakazi
pv-membership-kind-political-party = Chama cha siasa
pv-membership-kind-professional-body = Chama cha kitaaluma
pv-membership-kind-religious-order = Shirika la kitawa
pv-membership-kind-religious-association = Chama cha kidini
pv-membership-kind-fraternal-order = Udugu
pv-membership-kind-veterans-association = Chama cha wanajeshi wastaafu
pv-membership-kind-sports-club = Klabu ya michezo
pv-membership-kind-cultural-association = Chama cha kitamaduni
pv-membership-kind-charitable-association = Chama cha hisani
pv-membership-kind-other = Nyingine
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Imekadiriwa na mtu aliyemfahamu
pv-personality-instrument-inferred = Imekisiwa kutoka kwa rekodi
pv-personality-instrument-other = Nyingine
pv-introversion-extraversion-strongly-introverted = Mkimya sana
pv-introversion-extraversion-introverted = Mkimya
pv-introversion-extraversion-ambiverted = Wa kati
pv-introversion-extraversion-extraverted = Mchangamfu
pv-introversion-extraversion-strongly-extraverted = Mchangamfu sana
pv-stress-tolerance-very-low = Chini sana
pv-stress-tolerance-low = Chini
pv-stress-tolerance-moderate = Wastani
pv-stress-tolerance-high = Juu
pv-stress-tolerance-very-high = Juu sana
pv-decision-style-rational = Wa kimantiki
pv-decision-style-intuitive = Wa hisia
pv-decision-style-dependent = Wa kutegemea wengine
pv-decision-style-avoidant = Wa kukwepa
pv-decision-style-spontaneous = Wa ghafla
pv-sport-level-recreational = Burudani
pv-sport-level-amateur-competitive = Amateur wa mashindano
pv-sport-level-semi-professional = Nusu-kitaaluma
pv-sport-level-professional = Kitaaluma
pv-diet-omnivore = Hula kila kitu
pv-diet-flexitarian = Hula nyama mara chache
pv-diet-pescatarian = Hula samaki, si nyama
pv-diet-vegetarian = Hali nyama
pv-diet-vegan = Hali chochote cha wanyama
pv-diet-other = Nyingine

pv-substance-tobacco = Tumbaku na nikotini
pv-substance-alcohol = Pombe
pv-substance-cannabis = Bangi
pv-substance-opioids = Dawa za kulevya za afyuni
pv-substance-stimulants = Vichangamshi
pv-substance-sedatives-hypnotics = Dawa za kutuliza na usingizi
pv-substance-hallucinogens = Vileta njozi
pv-substance-inhalants = Vitu vya kuvuta puani
pv-substance-gambling = Kamari
pv-substance-gaming = Michezo ya video
pv-substance-other = Nyingine
pv-use-pattern-occasional-use = Matumizi ya mara moja moja
pv-use-pattern-regular-use = Matumizi ya kawaida
pv-use-pattern-harmful-use = Matumizi yenye madhara
pv-use-pattern-dependence = Uraibu
pv-use-pattern-in-remission = Amepata nafuu
pv-lineage-biological = Wa kuzaa
pv-lineage-adoptive = Wa kuasili
pv-lineage-foster = Wa kulea
pv-lineage-step = Wa kambo
pv-lineage-guardianship = Ulezi wa kisheria
pv-lineage-unknown = Haijulikani
pv-link-relation-godparent = Mzazi wa ubatizo
pv-link-relation-godchild = Mtoto wa ubatizo
pv-link-relation-witness = Shahidi
pv-link-relation-officiant = Msimamizi wa ibada
pv-link-relation-business-partner = Mbia wa biashara
pv-link-relation-employer = Mwajiri
pv-link-relation-employee = Mwajiriwa
pv-link-relation-mentor = Mshauri
pv-link-relation-apprentice = Mwanafunzi wa kazi
pv-link-relation-close-friend = Rafiki wa karibu
pv-link-relation-neighbour = Jirani
pv-link-relation-guardian = Mlezi
pv-link-relation-ward = Anayelelewa
pv-link-relation-other = Nyingine
pv-country-AD = Andorra
pv-country-AE = Falme za Kiarabu
pv-country-AF = Afghanistan
pv-country-AG = Antigua na Barbuda
pv-country-AI = Anguilla
pv-country-AL = Albania
pv-country-AM = Armenia
pv-country-AO = Angola
pv-country-AQ = Antaktiki
pv-country-AR = Ajentina
pv-country-AS = Samoa ya Marekani
pv-country-AT = Austria
pv-country-AU = Australia
pv-country-AW = Aruba
pv-country-AX = Visiwa vya Aland
pv-country-AZ = Azerbaijani
pv-country-BA = Bosnia na Hezegovina
pv-country-BB = Babadosi
pv-country-BD = Bangladeshi
pv-country-BE = Ubelgiji
pv-country-BF = Bukinafaso
pv-country-BG = Bulgaria
pv-country-BH = Bahareni
pv-country-BI = Burundi
pv-country-BJ = Benin
pv-country-BL = St. Barthelemy
pv-country-BM = Bermuda
pv-country-BN = Brunei
pv-country-BO = Bolivia
pv-country-BQ = Uholanzi ya Karibiani
pv-country-BR = Brazil
pv-country-BS = Bahama
pv-country-BT = Bhutan
pv-country-BV = Kisiwa cha Bouvet
pv-country-BW = Botswana
pv-country-BY = Belarus
pv-country-BZ = Belize
pv-country-CA = Kanada
pv-country-CC = Visiwa vya Cocos (Keeling)
pv-country-CD = Jamhuri ya Kidemokrasia ya Kongo
pv-country-CF = Jamhuri ya Afrika ya Kati
pv-country-CG = Kongo - Brazzaville
pv-country-CH = Uswisi
pv-country-CI = Cote d’Ivoire
pv-country-CK = Visiwa vya Cook
pv-country-CL = Chile
pv-country-CM = Kameruni
pv-country-CN = Uchina
pv-country-CO = Kolombia
pv-country-CR = Kostarika
pv-country-CU = Kuba
pv-country-CV = Cape Verde
pv-country-CW = Curacao
pv-country-CX = Kisiwa cha Krismasi
pv-country-CY = Saiprasi
pv-country-CZ = Chechia
pv-country-DE = Ujerumani
pv-country-DJ = Jibuti
pv-country-DK = Denmaki
pv-country-DM = Dominika
pv-country-DO = Jamhuri ya Dominika
pv-country-DZ = Aljeria
pv-country-EC = Ecuador
pv-country-EE = Estonia
pv-country-EG = Misri
pv-country-EH = Sahara Magharibi
pv-country-ER = Eritrea
pv-country-ES = Uhispania
pv-country-ET = Ethiopia
pv-country-FI = Ufini
pv-country-FJ = Fiji
pv-country-FK = Visiwa vya Falkland
pv-country-FM = Mikronesia
pv-country-FO = Visiwa vya Faroe
pv-country-FR = Ufaransa
pv-country-GA = Gabon
pv-country-GB = Ufalme wa Muungano
pv-country-GD = Grenada
pv-country-GE = Jojia
pv-country-GF = Guiana ya Ufaransa
pv-country-GG = Guernsey
pv-country-GH = Ghana
pv-country-GI = Gibraltar
pv-country-GL = Greenland
pv-country-GM = Gambia
pv-country-GN = Gine
pv-country-GP = Guadeloupe
pv-country-GQ = Guinea ya Ikweta
pv-country-GR = Ugiriki
pv-country-GS = Visiwa vya Georgia Kusini na Sandwich Kusini
pv-country-GT = Guatemala
pv-country-GU = Guam
pv-country-GW = Ginebisau
pv-country-GY = Guyana
pv-country-HK = Hong Kong SAR China
pv-country-HM = Kisiwa cha Heard na Visiwa vya McDonald
pv-country-HN = Honduras
pv-country-HR = Croatia
pv-country-HT = Haiti
pv-country-HU = Hungaria
pv-country-ID = Indonesia
pv-country-IE = Ayalandi
pv-country-IL = Israeli
pv-country-IM = Kisiwa cha Man
pv-country-IN = India
pv-country-IO = Eneo la Uingereza katika Bahari Hindi
pv-country-IQ = Iraki
pv-country-IR = Iran
pv-country-IS = Aisilandi
pv-country-IT = Italia
pv-country-JE = Jersey
pv-country-JM = Jamaika
pv-country-JO = Jordan
pv-country-JP = Japani
pv-country-KE = Kenya
pv-country-KG = Kirigizistani
pv-country-KH = Kambodia
pv-country-KI = Kiribati
pv-country-KM = Komoro
pv-country-KN = St. Kitts na Nevis
pv-country-KP = Korea Kaskazini
pv-country-KR = Korea Kusini
pv-country-KW = Kuwait
pv-country-KY = Visiwa vya Cayman
pv-country-KZ = Kazakistani
pv-country-LA = Laos
pv-country-LB = Lebanon
pv-country-LC = St. Lucia
pv-country-LI = Liechtenstein
pv-country-LK = Sri Lanka
pv-country-LR = Liberia
pv-country-LS = Lesoto
pv-country-LT = Lithuania
pv-country-LU = Luxembourg
pv-country-LV = Latvia
pv-country-LY = Libya
pv-country-MA = Morocco
pv-country-MC = Monaco
pv-country-MD = Moldova
pv-country-ME = Montenegro
pv-country-MF = St. Martin
pv-country-MG = Madagaska
pv-country-MH = Visiwa vya Marshall
pv-country-MK = Masedonia ya Kaskazini
pv-country-ML = Mali
pv-country-MM = Myanmar (Burma)
pv-country-MN = Mongolia
pv-country-MO = Makau SAR China
pv-country-MP = Visiwa vya Mariana vya Kaskazini
pv-country-MQ = Martinique
pv-country-MR = Moritania
pv-country-MS = Montserrat
pv-country-MT = Malta
pv-country-MU = Morisi
pv-country-MV = Maldivi
pv-country-MW = Malawi
pv-country-MX = Meksiko
pv-country-MY = Malesia
pv-country-MZ = Msumbiji
pv-country-NA = Namibia
pv-country-NC = New Caledonia
pv-country-NE = Niger
pv-country-NF = Kisiwa cha Norfolk
pv-country-NG = Nigeria
pv-country-NI = Nikaragwa
pv-country-NL = Uholanzi
pv-country-NO = Norway
pv-country-NP = Nepal
pv-country-NR = Nauru
pv-country-NU = Niue
pv-country-NZ = Nyuzilandi
pv-country-OM = Oman
pv-country-PA = Panama
pv-country-PE = Peru
pv-country-PF = Polynesia ya Ufaransa
pv-country-PG = Papua New Guinea
pv-country-PH = Ufilipino
pv-country-PK = Pakistani
pv-country-PL = Poland
pv-country-PM = Santapierre na Miquelon
pv-country-PN = Visiwa vya Pitcairn
pv-country-PR = Puerto Rico
pv-country-PS = Maeneo ya Palestina
pv-country-PT = Ureno
pv-country-PW = Palau
pv-country-PY = Paraguay
pv-country-QA = Qatar
pv-country-RE = Reunion
pv-country-RO = Romania
pv-country-RS = Serbia
pv-country-RU = Urusi
pv-country-RW = Rwanda
pv-country-SA = Saudia
pv-country-SB = Visiwa vya Solomon
pv-country-SC = Ushelisheli
pv-country-SD = Sudan
pv-country-SE = Uswidi
pv-country-SG = Singapore
pv-country-SH = St. Helena
pv-country-SI = Slovenia
pv-country-SJ = Svalbard na Jan Mayen
pv-country-SK = Slovakia
pv-country-SL = Siera Leoni
pv-country-SM = San Marino
pv-country-SN = Senegali
pv-country-SO = Somalia
pv-country-SR = Suriname
pv-country-SS = Sudan Kusini
pv-country-ST = Sao Tome na Principe
pv-country-SV = El Salvador
pv-country-SX = Sint Maarten
pv-country-SY = Syria
pv-country-SZ = Eswatini
pv-country-TC = Visiwa vya Turks na Caicos
pv-country-TD = Chad
pv-country-TF = Himaya za Kusini za Kifaranza
pv-country-TG = Togo
pv-country-TH = Tailandi
pv-country-TJ = Tajikistani
pv-country-TK = Tokelau
pv-country-TL = Timor-Leste
pv-country-TM = Turkmenistan
pv-country-TN = Tunisia
pv-country-TO = Tonga
pv-country-TR = Uturuki
pv-country-TT = Trinidad na Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taiwan
pv-country-TZ = Tanzania
pv-country-UA = Ukraine
pv-country-UG = Uganda
pv-country-UM = Visiwa Vidogo vya Nje vya Marekani
pv-country-US = Marekani
pv-country-UY = Uruguay
pv-country-UZ = Uzibekistani
pv-country-VA = Mji wa Vatican
pv-country-VC = St. Vincent na Grenadines
pv-country-VE = Venezuela
pv-country-VG = Visiwa vya Virgin, Uingereza
pv-country-VI = Visiwa vya Virgin, Marekani
pv-country-VN = Vietnamu
pv-country-VU = Vanuatu
pv-country-WF = Wallis na Futuna
pv-country-WS = Samoa
pv-country-YE = Yemeni
pv-country-YT = Mayotte
pv-country-ZA = Afrika Kusini
pv-country-ZM = Zambia
pv-country-ZW = Zimbabwe
pv-country-SU = Muungano wa Kisovyeti
pv-country-DD = Ujerumani Mashariki
pv-country-YU = Yugoslavia
pv-country-CS = Chekoslovakia
pv-country-OT = Milki ya Osmani
lang-aa = Kiafar
lang-ab = Kiabkhazi
lang-ae = Kiavesta
lang-af = Kiafrikana
lang-ak = Kiakani
lang-am = Kiamhari
lang-an = Kiaragoni
lang-ar = Kiarabu
lang-as = Kiassam
lang-av = Kiavari

lang-ay = Kiaymara
lang-az = Kiazerbaijani
lang-ba = Kibashkiri
lang-be = Kibelarusi
lang-bg = Kibulgaria
lang-bi = Kibislama
lang-bm = Kibambara
lang-bn = Kibengali
lang-bo = Kitibeti
lang-br = Kibretoni
lang-bs = Kibosnia
lang-ca = Kikatalani
lang-ce = Kichechenia
lang-ch = Kichamorro
lang-co = Kikosikani
lang-cr = Kikree
lang-cs = Kicheki
lang-cu = Kislovakia cha Kanisa
lang-cv = Kichuvash
lang-cy = Kiwelisi
lang-da = Kidenmaki
lang-de = Kijerumani
lang-dv = Kidivehi
lang-dz = Kizongkha
lang-ee = Kiewe
lang-el = Kigiriki
lang-en = Kiingereza
lang-eo = Kiesperanto
lang-es = Kihispania
lang-et = Kiestonia
lang-eu = Kibaski
lang-fa = Kiajemi
lang-ff = Kifulani
lang-fi = Kifini
lang-fj = Kifiji
lang-fo = Kifaroe
lang-fr = Kifaransa
lang-fy = Kifrisia cha Magharibi
lang-ga = Kiayalandi
lang-gd = Kigaeli cha Uskoti
lang-gl = Kigalisi
lang-gn = Kiguarani
lang-gu = Kigujarati
lang-gv = Kimanx
lang-ha = Kihausa
lang-he = Kiebrania
lang-hi = Kihindi
lang-ho = Kihiri Motu
lang-hr = Kikorasia
lang-ht = Kihaiti
lang-hu = Kihungaria
lang-hy = Kiarmenia
lang-hz = Kiherero
lang-ia = Kiintalingua
lang-id = Kiindonesia
lang-ie = Lugha ya kisayansi
lang-ig = Kiigbo
lang-ii = Kiyi cha Sichuan
lang-ik = Kiinupiaq
lang-io = Kiido
lang-is = Kiisilandi
lang-it = Kiitaliano
lang-iu = Kiinuktituti
lang-ja = Kijapani
lang-jv = Kijava
lang-ka = Kijojia
lang-kg = Kikongo
lang-ki = Kikikuyu
lang-kj = Kikwanyama
lang-kk = Kikazakh
lang-kl = Kikalaallisut
lang-km = Kikambodia
lang-kn = Kikannada
lang-ko = Kikorea
lang-kr = Kikanuri
lang-ks = Kikashmiri
lang-ku = Kikurdi
lang-kv = Kikomi
lang-kw = Kikorni
lang-ky = Kikyrgyz
lang-la = Kilatini
lang-lb = Kilasembagi
lang-lg = Kiganda
lang-li = Limburgish
lang-ln = Kilingala
lang-lo = Kilaosi
lang-lt = Kilithuania
lang-lu = Kiluba-Katanga
lang-lv = Kilatvia
lang-mg = Kimalagasi
lang-mh = Kimashale
lang-mi = Kimaori
lang-mk = Kimacedonia
lang-ml = Kimalayalamu
lang-mn = Kimongolia
lang-mr = Kimarathi
lang-ms = Kimalei
lang-mt = Kimalta
lang-my = Kiburma
lang-na = Kinauru
lang-nb = Kinorwe cha Bokmal
lang-nd = Kindebele cha Kaskazini
lang-ne = Kinepali
lang-ng = Kindonga
lang-nl = Kiholanzi
lang-nn = Kinorwe cha Nynorsk
lang-no = Kinorwe
lang-nr = Kindebele
lang-nv = Kinavajo
lang-ny = Kinyanja
lang-oc = Kiokitani
lang-oj = Kiojibwa
lang-om = Kioromo
lang-or = Kioriya
lang-os = Kiosetia
lang-pa = Kipunjabi
lang-pi = Kipali
lang-pl = Kipolandi
lang-ps = Kipashto
lang-pt = Kireno
lang-qu = Kikechua
lang-rm = Kiromanshi
lang-rn = Kirundi
lang-ro = Kiromania
lang-ru = Kirusi
lang-rw = Kinyarwanda
lang-sa = Kisanskriti
lang-sc = Kisardinia
lang-sd = Kisindhi
lang-se = Kisami cha Kaskazini
lang-sg = Kisango
lang-sh = Kiserbia-kroeshia
lang-si = Kisinhala
lang-sk = Kislovakia
lang-sl = Kislovenia
lang-sm = Kisamoa
lang-sn = Kishona
lang-so = Kisomali
lang-sq = Kialbania
lang-sr = Kiserbia
lang-ss = Kiswati
lang-st = Kisotho
lang-su = Kisunda
lang-sv = Kiswidi
lang-sw = Kiswahili
lang-ta = Kitamili
lang-te = Kitelugu
lang-tg = Kitajiki
lang-th = Kithai
lang-ti = Kitigrinya
lang-tk = Kiturukimeni
lang-tl = Kitagalogi
lang-tn = Kitswana
lang-to = Kitonga
lang-tr = Kituruki
lang-ts = Kitsonga
lang-tt = Kitatari
lang-tw = Twi
lang-ty = Kitahiti
lang-ug = Kiuyghur
lang-uk = Kiukraini
lang-ur = Kiurdu
lang-uz = Kiuzbeki
lang-ve = Kivenda
lang-vi = Kivietinamu
lang-vo = Kivolapuk
lang-wa = Kiwaloon
lang-wo = Kiwolofu
lang-xh = Kixhosa
lang-yi = Kiyiddi
lang-yo = Kiyoruba
lang-za = Kizhuang
lang-zh = Kichina
lang-zu = Kizulu
person-tab-profile = Wasifu
profile-groups-label = Makundi ya wasifu
profile-group-withheld = Sehemu ya kundi hili imefichwa kwako
profile-withheld = Imerekodiwa kwa mtu huyu na imefichwa kwako: { $classes }.
profile-empty = Bado hakuna kilichorekodiwa katika kundi hili.
profile-earlier = muundo wa awali
profile-earlier-title = Imerekodiwa na toleo la awali la programu hii, katika sehemu ambayo AXGF 1.1 haina nafasi yake. Imehifadhiwa kama ilivyoandikwa.
profile-other-names = { $n ->
        [one] na jina lingine moja
       *[other] na majina mengine { $n }
    }
profile-edit-group = Hariri { $group }
profile-summary-link = { $n ->
        [one] Jambo moja katika wasifu
       *[other] Mambo { $n } katika wasifu
    }
profile-from = kuanzia
profile-until = hadi
profile-yes = Ndiyo
profile-no = Hapana
profile-value = Thamani
profile-editor-title = Wasifu
profile-group-filled = { $n } zimejazwa
profile-problems = Baadhi ya ulichoingiza hakikuweza kuhifadhiwa. Kila tatizo limetajwa kando ya sehemu yake, na hakuna kilichoandikwa.
profile-editor-withheld = Kundi hili pia lina { $classes } kwa mtu huyu, ambayo huruhusiwi kusoma. Haionyeshwi hapa, na kuhifadhi fomu hii kunaiacha kama ilivyo kabisa.
profile-living-class-note = Mtu huyu amerekodiwa kuwa hai. Unachoingiza hapa katika kundi nyeti huonyeshwa kwa wasimamizi tu.
profile-relationships-elsewhere = Wazazi, wenzi, watoto, wazazi wa ubatizo na mashahidi hawahifadhiwi kwenye mtu huyu. Ni familia, viungo na matukio yanayowataja — kwa hiyo kila badiliko hapa linabadilisha pia rekodi ya kila mtu mwingine aliye ndani yake.
profile-documents-first = Kitu cha kidijitali kinaelekeza kwenye hati iliyoambatishwa kwa mtu huyu. Ambatisha faili kwanza.
profile-editor-nothing = Hakuna chochote katika kundi hili unachoruhusiwa kuhariri.
profile-new-entry = Kipengele kipya
profile-provenance = Tarehe, chanzo na uhakika
profile-from-date = Kweli kuanzia
profile-until-date = Kweli hadi
profile-remove-entry = Ondoa kipengele hiki
profile-add-entry = Ongeza kipengele kingine
profile-no-such-group-title = Hakuna kundi kama hilo
profile-no-such-group-detail = Wasifu hauna kundi la jina hilo.
profile-error-number = Thamani hapa ni nambari.
profile-error-integer = Thamani hapa ni nambari kamili.
profile-error-range = Nambari iko nje ya kiwango kinachoruhusiwa na sifa hii.
profile-error-term = Thamani si mojawapo ya machaguo yaliyoorodheshwa.
profile-error-required = Kipengele kinakosa sehemu kinayohitaji.
profile-error-one-of = Kipengele kinahitaji angalau sehemu moja kuu kujazwa.
profile-error-confidence = Uhakika ni kati ya 0 na 1, kwa mfano 0.8.
profile-error-time = Saa huandikwa kama saa na dakika, kwa mfano 05:40.
profile-error-currency = Sarafu ni msimbo wake wa herufi tatu, kwa mfano TZS.
profile-error-language = Lugha ni msimbo wake, kwa mfano sw au zh-Hans.
profile-error-coordinates = Viwianishi vinahitaji latitudo kati ya −90 na 90 na longitudo kati ya −180 na 180.
profile-error-rank-country = Cheo ni cha nchi tofauti na ile iliyochaguliwa.
record-unknown-place = [Mahali pasipojulikana]
record-missing-document = [Hati inayokosekana]
confidence-certain = Uhakika { $percent }% — karibu hakika
confidence-high = Uhakika { $percent }% — una ushahidi mzuri
confidence-medium = Uhakika { $percent }% — inawezekana lakini haijathibitishwa
confidence-low = Uhakika { $percent }% — dhana tu
tree-edge-union-between = { $from } na { $to } — { $confidence }
tree-edge-parentage-of = { $from }, mzazi wa { $to } — { $confidence }
record-note-biography = Wasifu wa maisha
record-note-birth-date-as-recorded = Tarehe ya kuzaliwa, kama ilivyorekodiwa
record-note-death-date-as-recorded = Tarehe ya kifo, kama ilivyorekodiwa
record-note-event-date-as-recorded = Tarehe ya { $event }, kama ilivyorekodiwa
record-unknown-source = [Chanzo kisichojulikana]
record-untitled-source = [Chanzo kisicho na kichwa]
record-unnamed = [Bila jina]
record-untitled = [Bila kichwa]
record-period-from = kuanzia { $date }
record-period-until = hadi { $date }
record-dates-unrecorded = tarehe hazijarekodiwa
record-link-unlabelled = ameunganishwa na
record-link-reverse = { $label } (wa)
record-place-worked-as = Alifanya kazi kama { $title }
record-place-married-to = Alifunga ndoa na { $name }
record-place-married = Walifunga ndoa
record-source-use-name = jina “{ $name }”
record-source-use-working-as = akifanya kazi kama { $title }
record-source-use-union-with = muungano na { $name }
record-source-use-union = muungano
record-lifespan-born = alizaliwa { $year }
record-lifespan-died = alifariki { $year }
size-bytes = { $n ->
        [one] baiti { $n }
       *[other] baiti { $n }
    }
size-kb = KB { $n }
size-mb = MB { $n }
size-gb = GB { $n }
calendar-gregorian = Gregori
calendar-julian = Juliasi
calendar-hebrew = Kiebrania
calendar-hijri = Hijria
calendar-persian = Kiajemi
calendar-chinese = Kichina
calendar-ethiopian = Kiethiopia
calendar-japanese_era = Enzi za Kijapani
calendar-republican_french = Jamhuri ya Ufaransa
calendar-roman = Kirumi
diff-summary-none = hakuna sehemu iliyobadilika
diff-summary-one = imebadilisha { $a }
diff-summary-two = imebadilisha { $a } na { $b }
diff-summary-many = imebadilisha { $a }, { $b } na { $n ->
        [one] nyingine moja
       *[other] nyingine { $n }
    }
diff-saved-none = hakuna sehemu iliyobadilika
diff-saved-one = imebadilisha { $a }
diff-saved-two = imebadilisha { $a } na { $b }
diff-saved-many = imebadilisha { $a }, { $b } na { $n ->
        [one] nyingine moja
       *[other] nyingine { $n }
    }
diff-section-other = sehemu nyingine
diff-section-avatar = Picha ya uso
diff-section-partners = Wenzi
diff-section-ai = Msaada wa AI
diff-section-birth = Kuzaliwa
diff-section-civil-status = Hali ya kiraia
diff-section-documents = Hati zilizoambatishwa
diff-section-extensions = Data ya programu
diff-section-children = Watoto
diff-section-union = Muungano
diff-section-date = Tarehe
diff-section-participants = Washiriki
diff-section-from = Imeunganishwa kutoka

diff-section-to = Imeunganishwa na
diff-section-valid-from = Halali kuanzia
diff-section-valid-until = Halali hadi
diff-section-visibility = Mwonekano
diff-section-employer = Mwajiri
diff-section-title-normalized = Cheo sanifu
diff-section-conflicts = Vyanzo vinavyopingana
diff-section-dna = DNA
diff-section-document = Hati
diff-section-language = Lugha
diff-section-place = Mahali
diff-section-repository = Hifadhi ya nyaraka
diff-section-script = Maandishi
diff-section-coordinates = Viwianishi
diff-section-country-history = Historia ya nchi
diff-section-identifiers = Vitambulisho
diff-section-names = Majina
diff-section-file = Faili
diff-section-linked-to = Imeambatishwa kwa
diff-section-ocr = Maandishi yaliyotambuliwa
diff-section-format-version = Toleo la muundo
history-created = ameunda
history-deleted = amefuta
history-merged = ameunganisha na rekodi nyingine
history-setup = ameanzisha kumbukumbu
history-attached = ameambatisha faili
admin-raw-json-unparsed = JSON ghafi haikuweza kuchanganuliwa ({ $error }). Hakuna kilichohifadhiwa.
admin-raw-json-dangling = JSON ghafi inaelekeza { $field } kwa { $target }, ambayo haipo katika kumbukumbu hii. Ichague kwa sehemu iliyo juu, au rekebisha kitambulisho. Hakuna kilichohifadhiwa.
conflict-someone = Mtu fulani
conflict-unrecorded-time = wakati ambao hakuna aliyeurekodi
dedup-merged-persons = { $n ->
        [one] mtu mmoja ameunganishwa
       *[other] watu { $n } wameunganishwa
    }
dedup-merged-families = { $n ->
        [one] familia moja imeunganishwa
       *[other] familia { $n } zimeunganishwa
    }
dedup-manual-review = { $n ->
        [one] kesi moja imeachwa ili mtu aikague
       *[other] kesi { $n } zimeachwa ili mtu azikague
    }
dedup-nothing = Hakuna cha kuripoti.
record-union-duplicate = Wenzi mmoja, zaidi ya rekodi moja.
record-union-duplicate-detail = Kifurushi kina rekodi tofauti za Familia kwa watu hawa wawili. Hiyo ni kasoro katika data, si muungano wa pili.
dedup-pair-merged = Jozi uliyouliza sasa ni rekodi moja.
dedup-pair-refused = Jozi uliyouliza haikuunganishwa: maktaba ilikataa na kuiacha ili mtu aikague.
validate-errors = { $n ->
        [one] kosa moja
       *[other] makosa { $n }
    }
validate-warnings = { $n ->
        [one] onyo moja
       *[other] maonyo { $n }
    }
validate-notes = { $n ->
        [one] dokezo moja
       *[other] madokezo { $n }
    }
validate-nothing = Hakuna cha kuripoti.
list-separator = { ", " }
result-written = Kumbukumbu iliandikwa kwenye diski.
result-refused = Maktaba ilikataa operesheni hii. Kumbukumbu iliyo kwenye diski haijabadilika.
convert-error-no-file = Hakuna faili iliyopakiwa. Chagua faili ya .ged kwanza.
convert-error-file-too-large = Faili hiyo ni MB { $size } na kikomo ni MB { $limit }. Hakuna kilichogeuzwa.
convert-error-too-large = Upakiaji huo ni mkubwa kuliko kikomo cha MB { $limit }. Hakuna kilichogeuzwa.
convert-error-unreadable = Upakiaji haukuweza kusomwa ({ $error }). Hakuna kilichogeuzwa.
convert-error-not-gedcom = Hiyo haionekani kuwa faili ya GEDCOM: faili ya GEDCOM 5.5.1 huanza na mstari “0 HEAD”. Hakuna kilichogeuzwa.
gedcom-read-title = Jinsi faili ilivyosomwa
gedcom-read-as = Imesomwa kama GEDCOM { $version }, kwa { $encoding }.
gedcom-read-as-unversioned = Imesomwa kama GEDCOM (kichwa chake hakitaji toleo), kwa { $encoding }.
gedcom-read-declared = Kichwa chake kinasema { $declared }; baiti zake zinasema vinginevyo, na zilisomwa kama zilivyo.
gedcom-read-gedzip = Kumbukumbu ya GEDCOM 7 (.gdz): seti yake ya data ilisomwa. Faili nyingine ndani yake (picha, skani) hazikuletwa: { $n }.
gedcom-read-crs = Mistari inayoishia kwa njia ya zamani ya Macintosh, imerekebishwa: { $n }.
gedcom-read-voids = Viashiria kwa @VOID@ — “hakuna rekodi” ya GEDCOM 7 — viliwekwa kando badala ya kusomwa kama watu: { $n }.
gedcom-read-notes = Madokezo ya pamoja (SNOTE) yalisomwa kama madokezo: { $n }.
gedcom-read-stray = Baiti ambazo hazikuwa UTF-8 katika faili hii ya UTF-8, kila moja imesomwa kama Windows-1252: { $n }.
gedcom-read-tidied = Mistari iliyopangwa kwa njia ambayo kigeuzi hakisomi (imesogezwa ndani, vichupo au nafasi za ziada kati ya sehemu, lebo kwa herufi ndogo, nafasi kabla ya kiungo cha CONC, herufi ya kudhibiti, alama ya kalenda ya Gregori), imerekebishwa: { $n }.
gedcom-read-too-deep = Mistari iliyopachikwa ndani zaidi kuliko muundo wowote wa GEDCOM, imewekwa kando pamoja na ilichobeba: { $n }.
convert-error-no-dataset = Zip hii haina gedcom.ged, wala faili moja ya .ged ya kusoma badala yake.
convert-error-refused = Kigeuzi kilikataa faili hii. Uchunguzi ulio hapa chini unaeleza kwa nini. Hakuna kilichohifadhiwa.
convert-error-packaging = Faili iligeuzwa lakini haikuweza kufungashwa ({ $error }).
completeness-fraction = { $part } kati ya { $whole }
event-category-adoption = Kuasili
event-category-migration = Uhamiaji
event-category-naturalization = Kupata uraia
event-category-incarceration = Kifungo
event-category-name_change = Kubadilisha jina
event-category-legal = Jambo la kisheria
event-category-religious = Tukio la kidini
event-category-social = Tukio la kijamii
event-category-historical = Tukio la kihistoria
precision-quarter_century = hadi robo karne
source-type-birth_certificate = cheti cha kuzaliwa
source-type-death_certificate = cheti cha kifo
source-type-marriage_certificate = cheti cha ndoa
source-type-census = sensa
source-type-baptism_record = rekodi ya ubatizo
source-type-burial_record = rekodi ya mazishi
source-type-will = wosia
source-type-land_record = rekodi ya ardhi
source-type-military_record = rekodi ya kijeshi
source-type-immigration_record = rekodi ya uhamiaji
source-type-naturalization = rekodi ya kupata uraia
source-type-passport = pasipoti
source-type-photograph = picha
source-type-letter = barua
source-type-diary = shajara
source-type-newspaper = gazeti
source-type-oral_tradition = mapokeo ya mdomo
source-type-dna = kipimo cha DNA
source-type-family_bible = Biblia ya familia
source-type-gravestone = jiwe la kaburi
source-type-published_genealogy = nasaba iliyochapishwa
source-type-other = chanzo kingine
source-status-verified = imelinganishwa na asili
source-status-unverified = bado haijakaguliwa
source-status-lost = imepotea
source-status-known_missing = inajulikana kuwa haipo
document-type-birth_certificate = cheti cha kuzaliwa
document-type-death_certificate = cheti cha kifo
document-type-marriage_certificate = cheti cha ndoa
document-type-census_page = ukurasa wa sensa
document-type-baptism_record = rekodi ya ubatizo
document-type-military_record = rekodi ya kijeshi
document-type-will = wosia
document-type-land_record = rekodi ya ardhi
document-type-diary = shajara
document-type-newspaper_clipping = kipande cha gazeti
document-type-gravestone_photo = picha ya jiwe la kaburi
document-type-family_tree_drawing = mchoro wa mti wa familia
document-type-audio = rekodi ya sauti
document-type-video = rekodi ya video
document-status-present = imehifadhiwa hapa
document-status-referenced = imetajwa, imehifadhiwa mahali pengine
document-status-known_missing = inajulikana kuwa haipo
document-status-lost = imepotea
document-status-unknown = haijulikani ilipo
diag-unsupported_spec_version = Kumbukumbu inatangaza toleo la AXGF ambalo toleo hili la programu haliwezi kusoma.
diag-invalid_json = Kitu kinachopaswa kuwa JSON hakichanganuliki.
diag-invalid_bundle_structure = Kumbukumbu haijapangwa jinsi AXGF inavyotaka.
diag-schema_validation_failed = Rekodi hailingani na skima ya AXGF.
diag-dangling_reference = Rekodi inaelekeza kwa rekodi nyingine ambayo haipo katika kumbukumbu.
diag-duplicate_entity_id = Rekodi mbili zina kitambulisho kimoja.
diag-duplicate_unique_ref = Rekodi mbili zinadai rejeo lilelile linalopaswa kuwa la kipekee.
diag-cycle_detected = Viungo vya familia vinazunguka katika duara: mtu angekuwa babu yake mwenyewe.
diag-chronology_conflict = Tarehe zinapingana, kama mtoto aliyezaliwa kabla ya mzazi.
diag-out_of_vocabulary = Thamani si mojawapo ya istilahi zinazoruhusiwa na orodha yake.
diag-claim_inconsistent = Dai linajipinga lenyewe au linapinga dai jingine kuhusu jambo hilohilo.
diag-spec_version_mismatch = Toleo la AXGF lililotangazwa na rekodi halilingani na kilichomo ndani yake.
diag-unknown_attribute = Rekodi ina sifa ambayo AXGF haiifafanui.
diag-entity_not_found = Rekodi ya kubadilisha haipo katika kumbukumbu.
diag-entity_already_exists = Rekodi yenye kitambulisho hiki tayari ipo.
diag-unknown_entity_kind = Hii si aina ya rekodi iliyopo katika AXGF.
diag-delete_blocked_by_reference = Rekodi haiwezi kufutwa wakati rekodi nyingine bado zinaielekeza.
diag-manual_review_required = Mtu anahitaji kuangalia hili; halikubadilishwa kiotomatiki.
diag-zip_read_error = Faili ya kumbukumbu haikuweza kusomwa.
diag-zip_write_error = Faili ya kumbukumbu haikuweza kuandikwa.
diag-payloads_external = Faili zilizoambatishwa zinahifadhiwa nje ya data ya kumbukumbu yenyewe.
diag-payload_source_failed = Faili iliyoambatishwa haikuweza kusomwa.
diag-payload_sink_failed = Faili iliyoambatishwa haikuweza kuandikwa.
diag-gedcom_parse_error = Mstari mmoja wa faili ya GEDCOM haukueleweka.
diag-gedcom_unrecognized_tag = Faili ya GEDCOM inatumia lebo ambayo kiingizaji hakiijui, kwa hiyo kipengele hicho hakikuletwa.
diag-internal = Kuna hitilafu ndani ya maktaba.
diag-merge_refused = Rekodi hizo mbili haziwezi kuunganishwa kama zilivyo.
diag-merge_unresolved = Tofauti kati ya rekodi hizo mbili haijaamuliwa.
diag-merge_unsupported = Rekodi za aina hii haziwezi kuunganishwa.
diag-stale_plan = Moja ya rekodi imebadilika tangu mpango ulipofanywa: panga upya.
field-person-display-name = Jina la kuonyesha
field-person-display-name-hint = Jina linaloonyeshwa kila mahali kwenye tovuti.
field-person-gender = Jinsia
field-person-living = Yu hai
field-person-birth-date = Tarehe ya kuzaliwa
field-date-value-hint = Mwaka, mwaka na mwezi, au tarehe kamili: 1923, 1923-04 au 1923-04-12. Acha wazi ikiwa hakuna anayejua.
field-person-birth-precision = Usahihi wa kuzaliwa
field-precision-hint = Jinsi chanzo kinavyolieleza kwa usahihi.
field-person-birth-circa = Kuzaliwa ni takriban
field-circa-hint = Huonyeshwa kama “takriban 1923” badala ya dai kamili.
field-person-birth-place = Kitambulisho cha mahali pa kuzaliwa
field-person-birth-confidence = Uhakika wa kuzaliwa
field-person-confidence-hint = Una uhakika kiasi gani. Hiki ndicho tovuti inachochora.
field-person-death-date = Tarehe ya kifo
field-person-death-precision = Usahihi wa kifo
field-person-death-circa = Kifo ni takriban
field-person-death-place = Kitambulisho cha mahali pa kifo
field-person-death-confidence = Uhakika wa kifo
field-person-death-cause = Kisababishi cha kifo
field-person-bio = Wasifu wa maisha
field-notes = Madokezo
field-family-name = Jina la familia
field-description = Maelezo
field-family-union-type = Aina ya muungano
field-family-union-status = Hali ya muungano
field-family-union-confidence = Uhakika wa muungano
field-family-union-confidence-hint = Huamua jinsi mstari kati ya wenzi unavyochorwa kwa nguvu kwenye mti.
field-family-union-start = Mwanzo wa muungano
field-family-union-end = Mwisho wa muungano
field-family-notes-hint = Wenzi na watoto ni orodha — wahariri kwenye JSON ghafi hapa chini, au kwenye ukurasa wa mahusiano wa mtu.
field-category = Kundi
field-required-hint = Lazima.
field-event-subcategory = Kundi dogo
field-date = Tarehe
field-event-date-hint = Inahitajika na skima.
field-precision = Usahihi
field-circa = Takriban
field-place-id = Kitambulisho cha mahali
field-confidence = Uhakika
field-source-id = Kitambulisho cha chanzo
field-link-from-type = Aina ya mwanzo
field-link-from-id = Kitambulisho cha mwanzo
field-link-to-type = Aina ya mwisho
field-link-to-id = Kitambulisho cha mwisho
field-link-label = Lebo
field-link-label-hint = Husomwa mbele: “baba wa ubatizo”, “mwajiri”, “shahidi”. Lazima.
field-link-label-reverse = Lebo ya kinyume
field-link-label-reverse-hint = Jinsi kinavyosomwa kutoka upande mwingine: “mtoto wa ubatizo”, “mwajiriwa”.
field-link-bidirectional = Husomeka sawa pande zote mbili
field-valid-from = Halali kuanzia
field-link-valid-from-hint = Uhusiano ulipoanza.
field-valid-until = Halali hadi
field-link-confidence-hint = “Uhakika 85%, kulingana na barua ya familia” — jambo ambalo GEDCOM haiwezi kusema.
field-note = Dokezo
field-occupation-person-id = Kitambulisho cha mtu
field-occupation-title = Kazi
field-occupation-title-hint = Lazima, kwa mfano Mwalimu wa shule.
field-occupation-title-latin = Kazi (maandishi ya Kilatini)
field-occupation-employer = Mwajiri
field-occupation-from = Kuanzia
field-occupation-from-hint = Kazi ni kipindi. Kutoa ncha zote mbili ndiko kunakoichora kama upau.
field-occupation-until = Hadi
field-source-title = Kichwa
field-source-type = Aina ya chanzo
field-source-reliability = Kuaminika
field-source-reliability-hint = Lazima. Huonyeshwa kama beji kando ya kila jambo linalotegemea chanzo hiki.
field-source-status = Hali ya chanzo
field-source-repository = Hifadhi ya nyaraka
field-source-repository-reference = Rejeo katika hifadhi
field-source-transcription = Unukuzi
field-place-name = Jina kuu
field-place-name-lang = Lugha ya jina
field-place-name-lang-hint = Lebo ya lugha, kwa mfano sw, en au fr.
field-place-type = Aina ya mahali
field-place-region = Eneo
field-place-country-current = Nchi ya leo
field-place-country-current-hint = Historia ya mipaka yake ni orodha — ihariri kwenye JSON ghafi hapa chini.
field-document-filename = Jina la faili
field-document-mime-type = Aina ya midia
field-document-mime-type-hint = Lazima, kwa mfano image/jpeg.
field-document-type = Aina ya hati
field-document-status = Hali ya faili
field-document-url = Anwani ya wavuti
field-document-caption = Maelezo mafupi
lang-zh-Hans = Kichina (Kilichorahisishwa)
family-lineage = Uzazi
links-relation = Aina ya uhusiano
occupations-position = Wadhifa
field-link-relation = Aina ya uhusiano
field-link-relation-hint = Mojawapo ya mahusiano ambayo AXGF 1.1 inayataja. Lebo iliyo juu inahifadhi maneno ya rekodi yenyewe.
field-occupation-position = Wadhifa
field-occupation-position-hint = Wadhifa ndani ya kazi: Mwalimu Mkuu, pale kazi ni Ualimu.
error-delete-changed-title = Imebadilika tangu ulipoiangalia
error-delete-changed-detail = Rekodi hii ilihifadhiwa tena baada ya ukurasa uliotumia kuifuta kuchorwa, na sasa ni toleo { $version }. Hakuna kilichofutwa. Iangalie ilivyo sasa kabla ya kuamua tena.
error-delete-changed-look = Iangalie tena
documents-files = Faili zilizoambatishwa hapa
documents-files-help = Hariri maelezo ya faili, au futa faili lenyewe. Kufuta kunaondoa hati na baiti zake kwenye kumbukumbu, kwa kila mtu aliyeambatishiwa; ili kuiondoa kwa mtu huyu tu, futa mstari wake hapo juu.
documents-edit-details = Hariri maelezo
documents-delete = Futa faili hili
radar-section = Chati kutoka kwenye rekodi
radar-section-help = Usomaji tatu wa kile rekodi hii inachobeba, kila mhimili kuanzia 0 hadi 100. Kila nambari inakokotolewa kutoka kwa mambo yaliyoorodheshwa kando yake, kwa kanuni zilizoandikwa katika nyaraka za programu; hakuna kinachohifadhiwa, hakuna kinachokisiwa, na mhimili usio na cha kusoma huachwa wazi badala ya kupewa alama ya wastani.
radar-physique = Umbo la mwili
radar-mind = Hulka na akili
radar-vitality = Afya na nguvu
radar-axis-stature = Kimo
radar-axis-build = Umbo
radar-axis-lean-mass = Uzito usio wa mafuta
radar-axis-posture = Mkao
radar-axis-gait = Mwendo
radar-axis-dentition = Meno
radar-axis-openness = Uwazi
radar-axis-conscientiousness = Uangalifu
radar-axis-extraversion = Uchangamfu
radar-axis-agreeableness = Upole
radar-axis-stability = Utulivu wa kihisia
radar-axis-cognition = Utambuzi
radar-axis-circulation = Mzunguko wa damu
radar-axis-breathing = Kupumua
radar-axis-metabolism = Umetaboli
radar-axis-illness = Kutokuwa na magonjwa
radar-axis-senses = Hisia
radar-axis-rest = Usingizi na hali ya moyo
radar-folded-open = Onyesha chati hii
radar-folded-why = Mtu huyu amerekodiwa kuwa hai. Picha ya hulka ya mtu aliye hai hubaki imekunjwa hadi mtu anayeruhusiwa kuisoma aombe kuiona.
radar-empty = Bado hakuna chochote katika rekodi hii kinachoweza kusomwa kwenye chati hii.
radar-table-caption = { $chart }: kila mhimili, alama yake na kilichosomwa
radar-col-axis = Mhimili
radar-col-score = Alama
radar-col-from = Imesomwa kutoka
radar-no-score = hakuna alama
radar-from-none = hakuna
record-link-outgoing = kutoka kwa mtu huyu
record-link-incoming = kwa mtu huyu

health-attention = Inahitaji kushughulikiwa:
health-standing-token = Tokeni ya dharura ya msimamizi bado imewekwa. Inaingia kupita kila akaunti na kila ruhusa — ndiyo kazi yake siku ambayo hakuna anayeweza kuingia, na si kitu cha kuacha mahali pake sasa kwa kuwa kuna anayeweza. Iondoe kwenye faili ya mazingira ya huduma na uanzishe upya.
health-bundle-invalid = { $errors ->
        [one] Data ya familia haipiti tena uthibitishaji: kosa moja, limeorodheshwa chini ya Uthibitishaji hapa chini.
       *[other] Data ya familia haipiti tena uthibitishaji: makosa { $errors }, yameorodheshwa chini ya Uthibitishaji hapa chini.
    }
health-disk-unknown = Nafasi iliyo wazi kwenye mashine hii haikuweza kusomwa, kwa hiyo hakuna kitu hapa kinachoweza kuahidi kwamba uhifadhi unaofuata utatosha.
health-disk-no-room-to-save = Kuhifadhi hakuwezekani: { $free } iko wazi, na kujenga upya kifurushi hiki kunahitaji { $need }. Hakuna kilichopotea, na hakuna kinachoweza kubadilishwa hadi kuwe na nafasi.
health-disk-critical = Diski iko wazi kwa asilimia { $percent } — { $free } zimebaki. Kuhifadhi kutaacha kufanya kazi hivi karibuni.
health-disk-low = Diski iko wazi kwa asilimia { $percent } — { $free } zimebaki. Inafaa kushughulikiwa kabla haijawa ya dharura.
health-backup-unconfigured = Hakuna kinachohifadhiwa nakala. Diski moja ikiharibika ingechukua kila rekodi.
health-backup-never = Saraka ya nakala imewekwa, lakini hakuna nakala iliyowahi kuandikwa humo.
health-backup-stale = { $days ->
        [one] Nakala mpya zaidi ina siku moja. Nakala inapaswa kuandikwa kila siku.
       *[other] Nakala mpya zaidi ina siku { $days }. Nakala inapaswa kuandikwa kila siku.
    }
health-cache-missing = { $missing ->
        [one] Faili moja kati ya { $declared } zilizoambatishwa haipo kwenye akiba na haiwezi kupakuliwa hadi uhifadhi unaofuata uijenge upya.
       *[other] Faili { $missing } kati ya { $declared } zilizoambatishwa hazipo kwenye akiba na haziwezi kupakuliwa hadi uhifadhi unaofuata uzijenge upya.
    }

nav-river = Mto
nav-home = Familia
river-aria = Mto unaomzunguka { $name }: mababu chini, wazao juu.
river-band-living = WALIO HAI
river-circa = takr.
river-trail = Njia
river-back = ← Rudi
river-range = Upana
river-motion = Mwendo
river-travel = safiri
river-cut = kata
river-legend-attested = imethibitishwa
river-legend-inferred = imekisiwa
river-legend-speculative = dhana
river-legend-lost = rekodi inaishia
river-legend-repeat = ×n amefikiwa kwa mistari n, amechorwa chini ya kila mmoja
river-legend-again = ↔ amechorwa tena kando ya mwenzi wa baadaye
river-legend-discharge = wazao
river-legend-direction = ↓ kuelekea chanzo · ↑ kuelekea walio hai
view-tree-direction = ↓ mizizi: mababu · ↑ taji: wazao
view-galaxy-direction = ↓ mababu chini ya jua · ↑ wazao juu
view-summit-direction = ↓ mababu bondeni · ↑ wazao wakipanda kuelekea kilele
river-legend-generation = kizazi kimoja ≈ miaka 29
river-zoom = Kuza
river-still = Simamisha mandhari
view-tree-still = Simamisha mti
view-galaxy-still = Simamisha anga
view-summit-still = Simamisha mlima
river-whole = Familia nzima
river-names-all = Majina yote
river-zoom-hint = Kuza: Ctrl + gurudumu, bana, au + / − · ukiwa umekuza, buruta ili kusogeza
river-tab-sources = Vyanzo
river-legend-title = Jinsi ya kusoma mto
view-tree-name = Mti hai
view-tree-legend = Jinsi ya kusoma mti
view-galaxy-name = Galaksi
view-galaxy-legend = Jinsi ya kusoma galaksi
view-summit-name = Kupanda
view-summit-legend = Jinsi ya kusoma mlima
hud-ancestors = Mababu
hud-descendants = Wazao
hud-generations = Vizazi
hud-living = Walio hai
hud-attested = Viungo vilivyothibitishwa

search-label = Tafuta mtu
search-short = Mtu
date-qualifier = Kibainishi
date-qualifier-circa = takriban
search-title = Tafuta
search-heading = Watu wanaolingana na “{ $q }”
search-none = Hakuna unayeweza kumwona anayeitwa hivyo.
search-open-record = Fungua rekodi
search-hint = Enter inasafiri mtoni hadi kwao; Shift+Enter inafungua rekodi yao.
view-tree-search-hint = Enter inaukuza mti kuwazunguka; Shift+Enter inafungua rekodi yao.
view-galaxy-search-hint = Enter inawafanya kuwa jua; Shift+Enter inafungua rekodi yao.
view-summit-search-hint = Enter inakupeleka kwao; Shift+Enter inafungua rekodi yao.
search-summary = { $n ->
        [one] mtu { $n }
       *[other] watu { $n }
    } · ↵ safiri · ⇧↵ fungua rekodi
search-empty-query = Andika jina ili kutafuta.

nav-quality = Ubora wa data
quality-title = Mambo yanayofaa kuangaliwa
quality-none = Hakuna kinachofaa kuangaliwa: kila ukaguzi umepita.
quality-intro = Hakuna kinachobadilishwa kwa niaba yako hapa. Kila mstari ni mtindo unaofaa kukaguliwa — mara nyingi kuna sababu nzuri. Fungua rekodi na uamue, au weka alama kuwa imekaguliwa.
quality-groups-label = Aina za matokeo
quality-all = Zote
quality-show-label = Onyesha
quality-show-open-option = Za kukagua
quality-download = Pakua
quality-group-names = Majina
quality-group-relations = Mahusiano
quality-group-dates = Tarehe
quality-group-places = Mahali
quality-group-evidence = Ushahidi
quality-export-csv = Pakua kama CSV
quality-export-json = Pakua kama JSON
quality-open = Fungua
quality-none-file = Hakuna kinachofaa kuangaliwa katika faili hii.
quality-show-list = Onyesha orodha
quality-after-import = Hakuna kilichobadilishwa katika faili. Ikishakuwa kumbukumbu ya tovuti, ukurasa wa ubora wa data unaorodhesha mambo hayo hayo pamoja na kiungo cha kila rekodi.
quality-file-list-cut = Ni { $shown } za kwanza tu zinazoorodheshwa hapa. Faili ikishakuwa kumbukumbu ya tovuti, ukurasa wa ubora wa data unaziorodhesha zote.
quality-rule-name-sentence = Sentensi katika sehemu ya jina
quality-name-sentence = “{ $value }” inaonekana zaidi kama sentensi kuliko jina: ikiwa ni maelezo, acha jina katika sehemu hiyo na uhamishe mengine kwenye dokezo.
quality-rule-union-parent-child = Wenzi ambao pia ni mzazi na mtoto
quality-union-parent-child = { $parent } na { $child } wanaonekana kama wenzi na pia kama mzazi na mtoto: inafaa kukagua ni uhusiano upi vyanzo vinaunga mkono.
quality-rule-child-listed-twice = Mtoto aliyeorodheshwa mara mbili
quality-child-listed-twice = { $child } anaonekana mara mbili kati ya watoto wa familia hii: ikiwa ni mtoto yuleyule, kipengele kimoja kinaweza kuondolewa.
quality-rule-duplicate-union = Wenzi walewale wamerekodiwa mara mbili
quality-duplicate-union = { $a } na { $b } wana miungano { $count } iliyorekodiwa: ikiwa walioana mara moja tu, huenda huu ni muungano mmoja ulioingizwa mara mbili.
quality-rule-partner-twice = Mtu mmoja pande zote mbili za muungano
quality-partner-twice = { $person } anaonekana pande zote mbili za muungano huu: inafaa kukagua mwenzi mwingine alikuwa nani, au kuacha upande huo wazi ikiwa hakuna anayejua.
quality-rule-foreign-surnames = Jina la ukoo la mwana lisilolingana na la mzazi yeyote
quality-foreign-surnames = Jina la ukoo lisilolingana na la mzazi yeyote: { $who }. Hilo linaweza kuwa la kawaida — kubadilisha jina, mtoto wa kambo, tahajia tofauti — lakini linafaa kuangaliwa. (Mabinti hawajajumuishwa, kwa kuwa jina la ndoa ndiyo sababu ya kawaida.)
quality-rule-death-before-birth = Kifo kabla ya kuzaliwa
quality-death-before-birth = { $person } ana kifo mwaka { $death }, kabla ya kuzaliwa kwake mwaka { $birth }: moja ya tarehe hizo mbili huenda imeandikwa vibaya; inafaa kukaguliwa dhidi ya vyanzo.
quality-rule-child-before-parent = Mtoto aliyezaliwa kabla ya mzazi
quality-child-before-parent = { $child } (alizaliwa { $childborn }) anaonekana kuzaliwa kabla ya mzazi wake { $parent } (alizaliwa { $parentborn }): inafaa kukagua tarehe au uzazi.
quality-rule-parent-too-young = Mzazi mwenye umri chini ya miaka kumi na mbili
quality-parent-too-young = { $parent } angekuwa na miaka { $age } { $child } alipozaliwa, jambo lisilo la kawaida: inafaa kukagua tarehe au uzazi.
quality-rule-parent-too-old = Mzazi mwenye umri zaidi ya miaka sitini na mitano
quality-parent-too-old = { $parent } angekuwa na miaka { $age } { $child } alipozaliwa, jambo lisilo la kawaida: inafaa kukagua tarehe au uzazi.
quality-rule-marriage-before-birth = Ndoa kabla ya kuzaliwa kwa mwenzi
quality-marriage-before-birth = Ndoa hii ina tarehe { $married }, kabla { $person } hajazaliwa mwaka { $born }: inafaa kukagua tarehe ya ndoa.
quality-rule-isolated-record = Rekodi isiyo na tarehe wala mahusiano
quality-isolated-record = { $person } hana tarehe wala kiungo na yeyote katika familia: inafaa kuongeza jinsi anavyohusika, au kuondoa rekodi ikiwa iliingizwa kwa makosa.
quality-rule-place-no-region = Mahali pasipo na eneo
quality-place-no-region = { $place } hapana eneo: kuongeza moja kunasaidia kutofautisha mahali wenye jina moja.
quality-rule-region-free-text = Eneo linalosomeka kama dokezo
quality-region-free-text = Eneo la { $place } linasema “{ $value }”, ambalo linaonekana zaidi kama dokezo kuliko eneo: inafaa kuhamisha dokezo kwenye rekodi linaloielezea.
quality-rule-place-near-duplicate = Mahali ambapo huenda ni pamoja
quality-place-near-duplicate = { $names } huenda ni mahali pamoja palipoandikwa tofauti: ikiwa ndivyo, panaweza kuunganishwa kuwa pamoja.
quality-rule-not-a-place = Kipengele ambacho huenda si mahali
quality-not-a-place = “{ $place }” kiko kwenye orodha ya mahali lakini huenda si mahali — kinaanza kwa herufi ndogo, jambo ambalo majina ya mahali kwa kawaida hayafanyi: inafaa kukagua ikiwa ni dokezo kuhusu mahali.
quality-rule-certain-without-source = Hakika, bila chanzo
quality-certain-without-source = { $record } ina dai lililowekwa alama ya hakika kamili ({ $claims }) bila chanzo kilichoambatishwa: inafaa kuambatisha chanzo, au kupunguza uhakika.
quality-dismiss = Imekaguliwa — ni kawaida kwetu
quality-restore = Ionyeshe tena
quality-note-label = Kwa nini (si lazima)
quality-show-dismissed = Zilizokaguliwa na kubakizwa
quality-show-open = Rudi kwenye orodha iliyo wazi
quality-dismissed-by = Imekaguliwa na { $by } tarehe { $at }
quality-gone-title = Tokeo hilo halipo tena
quality-gone-detail = Rekodi imebadilika tangu orodha ilipofunguliwa, kwa hiyo tokeo halisomeki tena vilevile. Pakia orodha upya na uangalie tena.
quality-summary = { $n ->
        [one] Jambo moja linalofaa kuangaliwa
       *[other] Mambo { $n } yanayofaa kuangaliwa
    }
quality-summary-file = { $n ->
        [one] Jambo moja linalofaa kuangaliwa katika faili hii
       *[other] Mambo { $n } yanayofaa kuangaliwa katika faili hii
    }
quality-dismissed-summary = { $n ->
        [one] Tokeo moja limekaguliwa na kubakizwa
       *[other] Matokeo { $n } yamekaguliwa na kubakizwa
    }
quality-dismiss-all = Weka alama zote kuwa zimekaguliwa ({ $n })
quality-restore-all = Onyesha zote tena ({ $n })
quality-bulk-conflict = Rekodi zilizobadilika tangu orodha ilipofunguliwa: { $n }. Hakuna kilichowekwa alama; pakia orodha upya na ujaribu tena.

edit-tab-changed = imebadilishwa, bado haijahifadhiwa
edit-remove = Ondoa
edit-add-partner = Ongeza mwenzi
edit-add-child = Ongeza mtoto
edit-union-with = Muungano na { $names }
edit-union-alone = Muungano usio na mwenzi aliyerekodiwa
edit-child-of = Mtoto wa { $names }
edit-new-family = Anzisha familia
edit-cite-title = Kinachotegemea chanzo
edit-attach-to = Kitaje kwa ajili ya
edit-add-source = Ongeza chanzo
edit-error-source-title = Chanzo kipya kinahitaji kichwa. Hakuna kilichohifadhiwa.
edit-detach-file = Tenganisha na rekodi hii
edit-no-sources = Hakuna jambo lolote la mtu huyu linalotaja chanzo bado.
edit-no-files = Bado hakuna faili iliyoambatishwa kwenye rekodi hii.
edit-saved = { $n ->
        [one] Imehifadhiwa: rekodi moja imeandikwa.
       *[other] Imehifadhiwa: rekodi { $n } zimeandikwa kwa uhifadhi mmoja.
    }
edit-saved-none = Hakuna kilichobadilika, kwa hiyo hakuna kilichohifadhiwa.
edit-back-to-record = Rudi kwenye rekodi
edit-avatar = Picha ya wasifu
edit-avatar-frame = Weka uso katikati
edit-avatar-upload = Tumia kama picha ya wasifu
edit-stale = Mtu mwingine amebadilisha { $what } tangu ukurasa huu ufunguliwe. Hakuna kilichohifadhiwa; kila ulichoingiza bado kipo hapa.
edit-refused = Hakuna kilichohifadhiwa: { $reason }

error-cross-site-title = Imekataliwa: imetumwa kutoka tovuti nyingine
error-cross-site-detail = Fomu hii ilitoka kwenye ukurasa wa tovuti nyingine, kwa hiyo haikutekelezwa. Ikiwa uliituma kutoka tovuti hii, pakia ukurasa upya na uitume tena.
admin-nav-label = Sehemu za usimamizi
admin-nav-overview = Muhtasari
admin-nav-bundle = Kifurushi cha AXGF
admin-nav-infrastructure = Miundombinu
admin-nav-accounts = Akaunti
admin-nav-connectivity = Muunganisho

bundle-title = Kifurushi cha AXGF
bundle-lede = Kumbukumbu ya familia kama tovuti hii inavyoishikilia: { $path }. Inalingana na nini, kinachoonekana si sawa, na njia mbili za kuchukua nakala.
bundle-unset-title = Bado hakuna familia.
bundle-unset-detail = Usakinishaji huu una kifurushi tupu, kwa hiyo bado hakuna cha kukagua. Kianzishe kutoka kumbukumbu, kutoka faili ya GEDCOM au kutoka mwanzo.
bundle-kpi-conformity = Ulinganifu
bundle-verdict-ok = Inalingana
bundle-verdict-warn = Ya kufuatilia
bundle-verdict-fail = Hailingani
findings-tally = Kijani: { $ok } · Maonyo: { $warn } · Hatari: { $fail }
bundle-kpi-entities = Vipengele
bundle-kpi-entities-note = { $persons ->
        [one] mtu 1
       *[other] watu { $persons }
    } · { $files ->
        [one] faili 1 iliyoambatishwa
       *[other] faili { $files } zilizoambatishwa
    }
bundle-kpi-size = Ukubwa kwenye diski
bundle-kpi-members = { $n ->
        [one] faili 1 katika kumbukumbu
       *[other] faili { $n } katika kumbukumbu
    }
bundle-kpi-format = Muundo
bundle-kpi-format-note = Toleo hili linasoma na kuandika AXGF { $current }.
bundle-conformity = Ulinganifu
bundle-conformity-intro = Ukaguzi tisa, unaosomwa kutoka kwenye kifurushi kama kilivyo sasa; hakuna hata mmoja unaokibadilisha. Fungua mstari ili kuona cha kufanya.
bundle-area-format = Muundo
bundle-area-integrity = Uadilifu
bundle-area-privacy = Faragha
bundle-area-content = Maudhui
level-ok = Kijani
level-warn = Onyo
level-fail = Hatari
findings-what-to-do = Cha kufanya
findings-go = Nenda huko →
bundle-anomalies = Hitilafu
bundle-anomalies-intro = Kile ukurasa wa ubora wa data unachopata, kanuni kwa kanuni: bado wazi, na tayari zimewekwa alama ya kukaguliwa.
bundle-anomalies-rule = Kanuni
bundle-anomalies-open = Wazi
bundle-anomalies-checked = Zimekaguliwa
bundle-anomalies-none = Ukurasa wa ubora wa data haupati chochote cha kuangalia.
bundle-diagnostics = Uchunguzi wa kithibitishaji
bundle-facts = Taarifa
bundle-fact-path = Faili
bundle-fact-size = Ukubwa
bundle-fact-spec = Muundo
bundle-fact-schema = Skima iliyobebwa
bundle-fact-none = hakuna
bundle-fact-created = Imeundwa
bundle-fact-updated = Iliandikwa mwisho
bundle-fact-generator = Imeandikwa na
bundle-fact-gedcom = Imegeuzwa kutoka
bundle-export = Hamisha
bundle-export-raw = Kifurushi kama kilivyo (.axgf)
bundle-export-raw-go = Pakua kifurushi
bundle-export-sealed = Kimesimbwa kwa mtu mmoja (.axgf.gpg)
bundle-export-sealed-intro = Kifurushi kilekile, kama ujumbe wa OpenPGP ambao ni mwenye ufunguo wa siri tu anayeweza kuufungua. Bandika ufunguo wake wa umma: maandishi yanayoanza na “-----BEGIN PGP PUBLIC KEY BLOCK-----”.
bundle-export-sealed-go = Pakua kilichosimbwa
seal-recipient = Kwa
seal-fingerprint = Alama ya kidole
seal-algorithm = Algoriti
seal-fips-yes = ndani ya seti iliyoidhinishwa na FIPS
seal-fips-no = nje ya seti iliyoidhinishwa na FIPS
seal-expires = Ufunguo unaisha
seal-never = kamwe
seal-key-label = Ufunguo wa umma wa mpokeaji
seal-key-hint = Ufunguo wa RSA wa angalau biti 2048, au ufunguo wa mkunjo duaradufu. Hakiki alama ya kidole na mmiliki wake kabla ya kutuma chochote.
seal-remember = Hifadhi ufunguo huu kwa wakati ujao
seal-check = Soma ufunguo tu
seal-fips-note = Kifurushi kinasimbwa kwa AES-256 kwa ufunguo wa mpokeaji. Algoriti zilizoidhinishwa na FIPS ni RSA ya biti 3072 au zaidi na mikunjo ya NIST P-256, P-384 na P-521: omba P-384. Maktaba inayotumika hapa yenyewe si moduli iliyothibitishwa ya FIPS 140-3.
seal-decrypt-note = Mpokeaji anakifungua kwa GnuPG:
seal-forget = Sahau ufunguo uliohifadhiwa
seal-key-kept = Ufunguo { $fingerprint } umehifadhiwa kwa uhamishaji unaofuata uliosimbwa.
seal-error-no-key = Bandika kwanza ufunguo wa umma wa mpokeaji.
seal-error-not-a-key = Huo si ufunguo wa umma wa OpenPGP. Bandika kipande kizima, kuanzia mstari wa BEGIN hadi mstari wa END, na kamwe usibandike ufunguo wa siri.
seal-error-bad-signatures = Sahihi za ufunguo huo wenyewe hazithibitiki: huenda umebadilishwa. Omba nakala mpya kwa mmiliki wake.
seal-error-expired = Ufunguo huo, au kila ufunguo ndani yake unaoweza kusimba, umekwisha muda. Mwombe mmiliki wake auongeze muda au atume mpya.
seal-error-revoked = Mmiliki wake ameubatilisha ufunguo huo. Mwombe ufunguo wake mpya.
seal-error-no-encryption-key = Ufunguo huo unaweza kutia sahihi lakini si kusimba: hauna ufunguo mdogo wa usimbaji. Mwombe mmiliki wake mmoja (gpg --quick-add-key … encr).
seal-error-too-weak = Ufunguo huo wa RSA una biti { $bits }; uhamishaji unahitaji angalau { $min }.
seal-error-unsupported = Algoriti ya ufunguo huo si ile ambayo uhamishaji hutumia: funguo za RSA au za mkunjo duaradufu tu.
seal-error-io = Nakala iliyosimbwa haikuweza kutengenezwa: { $error }
date-time-utc = { $date }, { $time } UTC

bundle-check-format-title = Toleo la muundo
bundle-check-format-ok = AXGF { $version }, toleo ambalo programu hii inasoma na kuandika.
bundle-check-format-missing = Faili ya maelezo haitaji toleo lolote la AXGF.
bundle-check-format-missing-fix = Kifurushi hakikuandikwa na zana ya AXGF, au kiliharibika. Rejesha nakala ya mwisho iliyothibitishwa (axgf-cms restore).

bundle-check-format-older = Kimeandikwa kama AXGF { $version }; uhifadhi unaofuata unakiandika kama AXGF { $current }.
bundle-check-format-older-fix = Hakuna cha kufanya: uhariri unaofuata unakiandika upya. Hifadhi nakala ya faili ya zamani ikiwa programu nyingine bado inasoma { $version } tu.
bundle-check-format-newer = Kimeandikwa kama AXGF { $version }, mpya kuliko { $current } ya toleo hili: kuhifadhi kunaweza kupoteza kile toleo hili lisichoelewa.
bundle-check-format-newer-fix = Pandisha axgf-cms hadi toleo linalosoma AXGF { $version } kabla mtu yeyote hajahariri (Miundombinu, Toleo).
bundle-check-manifest-title = Hesabu za faili ya maelezo
bundle-check-manifest-ok = Hesabu za faili ya maelezo zinalingana na kilichomo kwenye kifurushi.
bundle-check-manifest-drift = Faili ya maelezo inatangaza { $declared } ilhali kifurushi kina { $actual } ({ $collection }).
bundle-check-manifest-fix = Hakuna kilichopotea: hesabu zinaandikwa upya katika uhifadhi unaofuata. Ikiwa kumbukumbu hii ilitoka mahali pengine, hakikisha ilinakiliwa kamili.
bundle-check-validation-title = Uthibitishaji
bundle-check-validation-ok = Kithibitishaji cha maktaba hakipati kosa lolote.
bundle-check-validation-errors = { $errors ->
        [one] kosa 1
       *[other] makosa { $errors }
    }: kifurushi hakithibitiki.
bundle-check-validation-errors-fix = Kila uchunguzi ulio hapa chini unataja rekodi na sehemu. Rekebisha rekodi, au rejesha nakala ya mwisho iliyothibitishwa.
bundle-check-validation-warnings = { $warnings ->
        [one] onyo 1
       *[other] maonyo { $warnings }
    }. Kamwe hayazuii kuhifadhi; kila moja linasema kuna kitu huenda si sawa.
bundle-check-validation-warnings-fix = Yasome hapa chini. Mengi ni marejeo kwa rekodi ambazo hazipo katika kifurushi hiki, jambo ambalo uingizaji wa GEDCOM huacha mara nyingi.
bundle-check-payloads-title = Faili zilizoambatishwa
bundle-check-payloads-none = Hakuna faili zilizoambatishwa.
bundle-check-payloads-ok = { $declared ->
        [one] Faili moja ipo.
       *[other] Faili zote { $declared } zipo.
    }
bundle-check-payloads-missing = Faili { $missing } kati ya { $declared } zilizoambatishwa hazipo kwenye akiba.
bundle-check-payloads-fix = Uhifadhi unaofuata unajenga upya akiba kutoka kwenye kifurushi. Ikiwa hazipo kwenye kifurushi chenyewe, zirejeshe kutoka kwa nakala.
bundle-check-binding-title = Akaunti na kifurushi
bundle-check-binding-ok = Faili ya akaunti iliundwa kwa ajili ya familia hii.
bundle-check-binding-mismatch = Faili ya akaunti iliundwa kwa ajili ya { $expected }; kifurushi hiki ni { $found }.
bundle-check-binding-fix = Ikiwa kifurushi kilirejeshwa au kubadilishwa jina kwa makusudi, hakuna cha kufanya. Vinginevyo akaunti za familia moja zinatumika kwenye mti wa nyingine: rudisha faili sahihi ya .acl kando yake.
bundle-check-living-title = Walio hai, hadharani
bundle-check-living-ok = Hakuna aliyerekodiwa kuwa hai anayeonekana kwa wageni.
bundle-check-living-public = { $n ->
        [one] Mtu 1 aliyerekodiwa kuwa hai amewekwa
       *[other] Watu { $n } waliorekodiwa kuwa hai wamewekwa
    } kuwa wa umma: mgeni yeyote anasoma rekodi yao.
bundle-check-living-fix = Isipokuwa walikubali, weka mwonekano wao kuwa wa wanachama (Hariri, Utambulisho).
bundle-check-special-title = Makundi maalum
bundle-check-special-none = Hakuna rekodi yenye data ya afya, bayometriki, jenetiki, uhalifu au tabia.
bundle-check-special-some = { $n ->
        [one] Rekodi 1 ina
       *[other] Rekodi { $n } zina
    } data ya afya, bayometriki, jenetiki, uhalifu au tabia. Kwa walio hai inabaki kwa wasimamizi, bila kujali rekodi inasema nini.
bundle-check-quality-title = Ubora wa data
bundle-check-quality-ok = Hakuna cha kuangalia.
bundle-check-quality-open = { $n ->
        [one] Jambo 1 la kuangalia
       *[other] Mambo { $n } ya kuangalia
    }, katika { $groups ->
        [one] kundi 1
       *[other] makundi { $groups }
    }.
bundle-check-quality-fix = Fungua ukurasa wa ubora wa data: kila mstari unataja rekodi na kinachoonekana si sawa. Rekebisha, au weka alama kuwa imekaguliwa.
bundle-check-size-title = Ukubwa
bundle-check-size-ok = { $size }, chini ya { $warn } ambapo ukurasa huu huanza kuonya.
bundle-check-size-heavy = { $size }, zaidi ya { $warn }.
bundle-check-size-fix = Kumbukumbu ni ya familia kuikuza. Fuatilia diski na nakala (Miundombinu, Hifadhi), na ongeza --size-warn-mb ikiwa ukubwa huu unatarajiwa.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

infra-area-security = Usalama
infra-area-access = Ufikiaji
infra-area-data = Data
infra-area-capacity = Uwezo
infra-area-release = Toleo
infra-area-platform = Jukwaa
infra-verdict-ok = Iko salama
infra-verdict-warn = Inahitaji kushughulikiwa
infra-verdict-fail = Hatari
duration-days = { $n ->
        [one] siku { $n }
       *[other] siku { $n }
    }
duration-hours = { $n ->
        [one] saa { $n }
       *[other] saa { $n }
    }
duration-minutes = { $n ->
        [one] dakika { $n }
       *[other] dakika { $n }
    }
infra-transport-title = Jinsi tovuti inavyofikiwa
infra-transport-ok = Ukurasa huu ulifika kupitia HTTPS.
infra-transport-exposed = Ukurasa huu ulifika bila usimbaji, na seva inasikiliza kwenye { $bind }, nje ya mashine hii: nywila na vidakuzi vya kikao vinapita mtandaoni wazi.
infra-transport-exposed-fix = Weka proksi ya kinyume ya HTTPS (Caddy, nginx, HAProxy) mbele yake, na uifunge huduma tena kwa 127.0.0.1: endesha bootstrap.sh upya ukitumia --bind 127.0.0.1:8080, au ondoa faili ya drop-in iliyoipanua.
infra-transport-plain = Ukurasa huu ulifika bila usimbaji, kutoka kwa seva inayosikiliza kwenye { $bind }. Hilo ni sawa kwenye mashine hii peke yake, na huko tu.
infra-transport-plain-fix = Fikia tovuti kupitia anwani yake ya HTTPS. Ikiwa proksi inakamilisha TLS, iwekee kutuma X-Forwarded-Proto: https ili seva hii ijue.
infra-forwarded-title = Anwani zinazopitishwa
infra-forwarded-ok = Seva inasikiliza kwenye mashine hii tu, kwa hiyo anwani ya mteja ambayo proksi inapitisha inaweza kuaminiwa ili kupunguza kasi ya majaribio ya kuingia yaliyoshindwa mara kwa mara.
infra-forwarded-exposed = Seva inaweza kufikiwa moja kwa moja, na inaamini kichwa cha X-Forwarded-For: mteja anayeweka kichwa hicho mwenyewe anaweza kuepuka upunguzaji wa kasi baada ya kuingia kushindwa.
infra-forwarded-fix = Funga huduma kwa 127.0.0.1 nyuma ya proksi, au zuia mlango kwa ngome ili proksi pekee iufikie.
infra-cross-site-title = Fomu kutoka tovuti nyingine
infra-cross-site-ok = Hakuna fomu iliyotumwa hapa kutoka tovuti nyingine katika siku iliyopita.
infra-cross-site-refused = { $n ->
        [one] Fomu moja iliyotumwa kutoka tovuti nyingine ilikataliwa katika siku iliyopita.
       *[other] Fomu { $n } zilizotumwa kutoka tovuti nyingine zilikataliwa katika siku iliyopita.
    }
infra-cross-site-fix = Hakuna kilichopita. Ikiwa makataa hayo ni yako — tovuti ileile iliyofikiwa kwa anwani ya pili — weka anwani ya umma chini ya Muunganisho, au tumia anwani hiyo moja.
infra-sso-title = Huduma za kuingia
infra-sso-none = Hakuna huduma ya kuingia iliyowashwa; akaunti zinaingia kwa nywila.
infra-sso-ok = { $n ->
        [one] Huduma moja ya kuingia imewashwa, na anwani ya umma ni HTTPS.
       *[other] Huduma { $n } za kuingia zimewashwa, na anwani ya umma ni HTTPS.
    }
infra-sso-no-https = Huduma ya kuingia imewashwa, lakini anwani ya umma si HTTPS: watoa huduma hukataa kutuma jibu lao kwa anwani isiyosimbwa, na kuingia kunashindwa.
infra-sso-fix = Chini ya Muunganisho, weka anwani ya umma kuwa anwani ya https:// ambayo wasomaji hutumia.
infra-admins-title = Wasimamizi
infra-admins-none = Hakuna akaunti hai inayoweza kusimamia tovuti hii. Ni tokeni ya dharura tu inayoweza, na wakati imewekwa tu.
infra-admins-none-fix = Ingia kwa tokeni ya dharura, unda msimamizi chini ya Akaunti, kisha ondoa tokeni.
infra-admins-one = Msimamizi mmoja. Mtu huyo akiwa hayupo au akipoteza nywila, hakuna anayeweza kusimamia tovuti.
infra-admins-one-fix = Mpe mtu wa pili unayemwamini nafasi ya msimamizi chini ya Akaunti.
infra-admins-ok = { $n ->
        [one] Msimamizi { $n } anaweza kuingia.
       *[other] Wasimamizi { $n } wanaweza kuingia.
    }
infra-token-title = Tokeni ya dharura
infra-token-generated = Hakuna tokeni ya dharura iliyowekwa.
infra-token-only-way = Tokeni ya dharura imewekwa, na bila akaunti ya msimamizi ndiyo njia pekee ya kuingia. Ibakize hadi msimamizi awepo.
infra-token-standing = Tokeni ya dharura bado imewekwa. Inaingia kupita kila akaunti na kila ruhusa.
infra-token-fix = Ondoa AXGF_CMS_ADMIN_TOKEN kwenye faili ya mazingira ya huduma, kisha anzisha huduma upya.
infra-registration-title = Kujisajili
infra-registration-closed = Wageni hawawezi kuunda akaunti; msimamizi pekee ndiye anayeweza.
infra-registration-secret = Wageni wanaweza kuunda akaunti ya mwanachama ikiwa wanajua siri ya usajili.
infra-registration-open = Mgeni yeyote anaweza kuunda akaunti ya mwanachama.
infra-registration-open-fix = Hiyo inafaa tovuti iliyokusudiwa kuwa wazi. Vinginevyo hitaji siri ya usajili, au funga usajili, chini ya Akaunti.
infra-failures-title = Majaribio ya kuingia yaliyoshindwa
infra-failures-ok = { $n ->
        [one] Jaribio { $n } la kuingia lililoshindwa katika siku iliyopita.
       *[other] Majaribio { $n } ya kuingia yaliyoshindwa katika siku iliyopita.
    }
infra-failures-many = { $n ->
        [one] Jaribio { $n } la kuingia lililoshindwa katika siku iliyopita: huenda mtu anabahatisha nywila.
       *[other] Majaribio { $n } ya kuingia yaliyoshindwa katika siku iliyopita: huenda mtu anabahatisha nywila.
    }
infra-failures-fix = Kila anwani tayari inapunguzwa kasi baada ya kushindwa mara kwa mara. Hesabu ikiendelea kupanda, tafuta majaribio yanakotoka kwenye kumbukumbu za proksi na uyazuie huko, na hakikisha kila msimamizi ana nywila ndefu au anaingia kupitia huduma ya kuingia.
infra-health-bundle-title = Data ya familia
infra-health-bundle-ok = Kifurushi kinafunguka na kinapita uthibitishaji.
infra-health-bundle-fix = Fungua ukurasa wa kifurushi cha AXGF ili kuona makosa na kuyarekebisha, au rejesha nakala ya mwisho iliyothibitishwa (axgf-cms restore).
infra-health-cache-title = Akiba ya viambatisho
infra-health-cache-ok = Kila faili iliyoambatishwa iko kwenye akiba.
infra-health-cache-fix = Hifadhi rekodi yoyote mara moja: uhifadhi unajenga upya akiba. Faili ambazo bado hazipo baada ya hapo hazipo kwenye kifurushi chenyewe.
infra-health-backup-title = Nakala
infra-health-backup-ok = { $count ->
        [one] Nakala ya hivi karibuni ipo; kumbukumbu { $count } imehifadhiwa.
       *[other] Nakala ya hivi karibuni ipo; kumbukumbu { $count } zimehifadhiwa.
    }
infra-health-backup-fix = Bila saraka ya nakala, endesha bootstrap.sh upya ukitumia --backup-dir. Ikiwa ipo, kagua kipima muda (systemctl status axgf-cms-backup.timer) na kumbukumbu yake (journalctl -u axgf-cms-backup).
infra-disk-title = Nafasi ya diski
infra-disk-ok = { $free } wazi kati ya { $total } (asilimia { $percent } wazi).
infra-disk-fix = Toa nafasi kwenye kizigeu chenye saraka ya data — nakala za zamani zinazohifadhiwa pia mahali pengine, kumbukumbu, faili za programu nyingine — au hamishia data kwenye diski kubwa zaidi.
infra-growth-title = Ukuaji
infra-growth-ok = { $days ->
        [one] Inakua takriban { $rate } kwa mwezi; nafasi ya siku { $days } zaidi kwa kasi hiyo.
       *[other] Inakua takriban { $rate } kwa mwezi; nafasi ya siku { $days } zaidi kwa kasi hiyo.
    }
infra-growth-soon = { $days ->
        [one] Inakua takriban { $rate } kwa mwezi: kwa kasi hiyo diski itajaa baada ya siku { $days }.
       *[other] Inakua takriban { $rate } kwa mwezi: kwa kasi hiyo diski itajaa baada ya siku { $days }.
    }
infra-growth-fix = Panga nafasi sasa: diski kubwa zaidi, nakala chache zinazohifadhiwa kwenye diski ileile, au nakala kuhamishiwa kwenye nyingine.
infra-growth-flat = Nakala hazijakua; hakuna cha kupanga.
infra-growth-unknown = Bado hakuna nakala za kutosha kupima ukuaji: inahitaji angalau siku moja kati ya ya zamani zaidi na mpya zaidi.
infra-memory-title = Kumbukumbu ya mashine
infra-memory-ok = { $used } zimetumika kati ya kikomo cha { $max } cha huduma (asilimia { $percent }).
infra-memory-high = { $used } zimetumika kati ya kikomo cha { $max } cha huduma (asilimia { $percent }): karibu na pale mfumo husimamisha huduma.
infra-memory-fix = Ongeza MemoryMax= katika faili ya drop-in ya axgf-cms.service, au tafuta kinachoshikilia kumbukumbu — uingizaji mkubwa sana, kiambatisho kikubwa — na uanzishe huduma upya.
infra-memory-unlimited = { $used } zinatumika, bila kikomo kilichowekwa kwa huduma.
infra-version-title = Toleo
infra-version-ok = v{ $current } ndilo toleo jipya zaidi lililochapishwa.
infra-version-behind = v{ $current } inaendeshwa; { $latest } imechapishwa.
infra-version-behind-fix = Soma maelezo ya toleo, kisha pandisha toleo kutoka sehemu ya Toleo hapa chini. Nakala inachukuliwa kwanza, na toleo la awali linarudi lenyewe ikiwa jipya halianzi.
infra-version-unknown = Haijulikani kama kuna toleo jipya zaidi: orodha ya matoleo haijaletwa.
infra-version-unknown-fix = Tumia “Angalia matoleo mapya” hapa chini. Mazingira yaliyotengwa ya uzalishaji hayawezi kufikia GitHub yenyewe; msaidizi wa kupandisha toleo akishasakinishwa, huleta orodha kila siku.
infra-version-stale = { $days ->
        [one] Orodha ya matoleo ililetwa mara ya mwisho siku { $days } iliyopita.
       *[other] Orodha ya matoleo ililetwa mara ya mwisho siku { $days } zilizopita.
    }
infra-upgrades-title = Kupandisha toleo
infra-upgrades-ok = Msaidizi wa kupandisha toleo amesakinishwa: kitufe kilicho hapa chini kinaweza kubadilisha toleo.
infra-upgrades-manual = Msaidizi wa kupandisha toleo hajasakinishwa, kwa hiyo toleo hubadilishwa kwa mkono kwenye seva.
infra-upgrades-fix = Endesha bootstrap.sh ya sasa mara moja kama root: inasakinisha msaidizi pamoja na huduma.
infra-sandbox-title = Mazingira yaliyotengwa
infra-sandbox-ok = Mchakato unaendeshwa kwa no-new-privileges na kichujio cha miito ya mfumo.
infra-sandbox-unknown = Mfumo huu hauripoti mazingira yaliyotengwa ya mchakato.
infra-sandbox-off = Mchakato unaendeshwa bila sehemu ya mazingira yake yaliyotengwa: no-new-privileges au kichujio cha miito ya mfumo kimezimwa.
infra-sandbox-fix = Endesha huduma kutoka kwenye unit ambayo bootstrap.sh huandika, bila faili za drop-in zinazoilegeza. Kwenye mashine ya maendeleo hili linatarajiwa.
infra-clock-title = Saa
infra-clock-ok = Saa ya mfumo inaonyesha { $year }, jambo linalowezekana.
infra-clock-wrong = Saa ya mfumo inaonyesha { $year }: tarehe, umri wa nakala na vikao vyote si sahihi.
infra-clock-fix = Washa usawazishaji wa muda: timedatectl set-ntp true.

infra-title = Miundombinu
infra-lede = Mashine ambayo tovuti hii inaendeshwa juu yake, ikikaguliwa kila ukurasa huu unapofunguliwa: jinsi inavyofikiwa, nani anashikilia funguo, kama data iko salama na bado inatosha, toleo gani linaendeshwa, na mazingira yaliyotengwa yanayoizunguka.
infra-kpi-health = Afya
infra-kpi-version = Toleo
infra-kpi-version-newer = { $version } inapatikana
infra-kpi-version-current = Imesasishwa
infra-kpi-version-unknown = Bado haijakaguliwa
infra-kpi-disk = Nafasi iliyo wazi
infra-kpi-disk-note = Asilimia { $percent } ya { $total } imetumika
infra-kpi-compute = Mashine
infra-kpi-cpus = { $n ->
        [one] kiini { $n }
       *[other] viini { $n }
    }
infra-kpi-load = Mzigo { $load }
infra-kpi-rss = { $size } zinatumika
infra-unknown = Haijulikani
infra-health = Afya
infra-health-intro = Kijani ni sawa, onyo linafaa kuangaliwa, hatari inahitaji kushughulikiwa sasa. Kila tokeo linafunguka kwenye cha kufanya kuhusu jambo hilo.
infra-json = Ukaguzi uleule kama JSON, kwa mfumo wa ufuatiliaji
infra-version = Toleo
infra-version-running = Linaloendeshwa
infra-version-latest = Lililochapishwa karibuni
infra-version-up-to-date = imesasishwa
infra-version-checked = Orodha imeletwa
infra-version-never = kamwe
infra-source-direct = na seva hii
infra-source-helper = na msaidizi wa kupandisha toleo
infra-version-token = Hazina ya faragha
infra-token-env-set = Tokeni ya GitHub imewekwa kwa ajili ya kuorodhesha na kupakua matoleo.
infra-token-env-unset = Hakuna tokeni. Ikiwa hazina ni ya faragha, weka moja katika { $name }, inayosomeka na root tu: msaidizi wa kupandisha toleo anaisoma hapo.
infra-releases-error = Jaribio la mwisho la kuleta orodha lilishindwa:
infra-releases-refresh = Angalia matoleo mapya
infra-releases-asked = Msaidizi wa kupandisha toleo ameombwa orodha mpya. Pakia ukurasa huu upya baada ya dakika moja.
infra-releases-not-asked = Ombi halikuweza kuandikwa: { $error }
infra-upgrade-status-running = Inabadilisha kutoka { $from } hadi { $to }…
infra-upgrade-status-succeeded = Imebadilishwa kutoka { $from } hadi { $to }.
infra-upgrade-status-failed = Mabadiliko kutoka { $from } hadi { $to } yameshindwa; { $from } bado ndilo toleo linaloendeshwa.
infra-upgrade-status-refused = Msaidizi wa kupandisha toleo alikataa mabadiliko hadi { $to }.
infra-upgrade-log = Matokeo ya msaidizi
infra-upgrade-pending = { $by } aliomba { $version } tarehe { $at }; msaidizi wa kupandisha toleo bado hajaichukua.
infra-releases = Matoleo yaliyochapishwa
infra-releases-tag = Toleo
infra-releases-published = Limechapishwa
infra-releases-relation = Ukilinganisha na hili
infra-releases-notes = Maelezo ya toleo
infra-releases-open = Soma
infra-relation-newer = jipya zaidi
infra-relation-current = hili
infra-relation-older = la zamani zaidi
infra-prerelease = toleo la majaribio
infra-releases-none = Bado hakuna orodha ya matoleo.
infra-upgrade = Badilisha toleo
infra-upgrade-intro = Kitufe kinamwomba msaidizi wa kupandisha toleo, anayeendeshwa kama root nje ya mazingira yaliyotengwa ya tovuti hii. Anachukua nakala, anasakinisha toleo baada ya checksum yake kuthibitishwa, anaanzisha huduma upya na kuikagua; toleo jipya lisipojibu, la awali linarudishwa.
infra-upgrade-version = Toleo la kusakinisha
infra-upgrade-confirm = Ili kurudi kwenye toleo la zamani, andika lebo yake tena
infra-upgrade-confirm-hint = Haihitajiki kwa toleo jipya zaidi. Toleo la zamani huenda lisisome data iliyoandikwa na jipya: soma maelezo yake kwanza.
infra-upgrade-go = Sakinisha toleo hili
infra-upgrade-safety = Tovuti haipatikani kwa sekunde chache huduma inapoanzishwa upya.
infra-upgrade-manual = Msaidizi wa kupandisha toleo hajasakinishwa kwenye seva hii, kwa hiyo ukurasa huu hauwezi kubadilisha toleo. Kama root, kwenye seva:
infra-upgrade-manual-helper = Kuendesha bootstrap.sh ya sasa mara moja kunasakinisha msaidizi, na kitufe hiki kinafanya kazi tangu hapo.
infra-upgrade-asked = { $version } imeombwa. Msaidizi wa kupandisha toleo anaichukua ndani ya sekunde chache; ukurasa huu unaonyesha jinsi ilivyokwenda.
infra-upgrade-no-helper = Msaidizi wa kupandisha toleo hajasakinishwa, kwa hiyo { $version } haiwezi kusakinishwa kutoka hapa.
infra-upgrade-unknown-version = { $version } haipo kwenye orodha ya matoleo yaliyochapishwa.
infra-upgrade-same-version = { $version } ndilo toleo linaloendeshwa tayari.
infra-upgrade-confirm-older = { $version } ni la zamani kuliko toleo linaloendeshwa. Andika lebo yake katika sehemu ya uthibitisho ili kurudi kwake.
infra-volumes = Hifadhi
infra-volumes-intro = Kizigeu chenye saraka ya data ({ $path }), na kile tovuti hii inachohifadhi humo.
infra-volumes-partition = Kizigeu
infra-of = { $used } kati ya { $total }
infra-volumes-ours = Tovuti hii
infra-volumes-breakdown = Kile tovuti hii inachohifadhi
infra-part-bundle = Kifurushi cha familia
infra-part-cache = Akiba ya viambatisho
infra-part-backups = Nakala
infra-part-binary = Programu
infra-part-journal = Kumbukumbu ya mabadiliko
infra-part-accounts = Akaunti na mipangilio
infra-files = { $n ->
        [one] faili { $n }
       *[other] faili { $n }
    }
infra-growth = Ukuaji
infra-growth-month = Kwa mwezi
infra-growth-not-enough = nakala hazitoshi kujua
infra-growth-left = Nafasi iliyobaki kwa kasi hii
infra-backups = Nakala
infra-backups-dir = Saraka
infra-backups-count = Kumbukumbu
infra-backups-newest = Mpya zaidi
infra-backups-oldest = Ya zamani zaidi
infra-backups-room = Nafasi zinapohifadhiwa
infra-free-of = { $free } wazi kati ya { $total }
infra-paths = Njia
infra-path-data = Saraka ya data
infra-path-binary = Programu
infra-compute = Kompyuta
infra-compute-hidden = Mazingira yaliyotengwa ya huduma yanaficha takwimu za mashine yenyewe; kinachoonyeshwa hapa ni kile mchakato unaweza kuona.
infra-compute-memory = Kumbukumbu ya mashine
infra-available-of = { $available } wazi kati ya { $total }
infra-compute-ceiling = Kumbukumbu ya huduma
infra-compute-no-ceiling = { $used }, bila kikomo
infra-compute-os = Mfumo
infra-compute-kernel = Kerneli
infra-compute-arch = Usanifu

infra-compute-cpu = Prosesa
infra-compute-cpus = Viini
infra-compute-load = Mzigo (dakika 1, 5, 15)
infra-compute-rss = Mchakato huu
infra-compute-threads = Nyuzi
infra-compute-files = Faili zilizo wazi
infra-compute-uptime = Huduma imekuwa ikiendeshwa kwa
infra-compute-sys-uptime = Mashine imekuwa ikiendeshwa kwa
infra-compute-sandbox = Mazingira yaliyotengwa
infra-compute-nnp-on = hakuna ruhusa mpya
infra-compute-nnp-off = ruhusa zinaweza kupatikana
infra-compute-seccomp-on = miito ya mfumo imechujwa
infra-compute-seccomp-off = miito ya mfumo haijachujwa
infra-activity = Shughuli
infra-activity-intro = Imehesabiwa katika { $span }: huduma hubakiza siku moja, na huanza tena kutoka sifuri inapoanzishwa upya.
infra-activity-sessions = Vikao vilivyo wazi
infra-activity-signed-in = Kuingia
infra-activity-failed = Majaribio ya kuingia yaliyoshindwa
infra-activity-registered = Akaunti zilizoundwa
infra-activity-cross-site = Fomu kutoka tovuti nyingine zilizokataliwa

sso-error-not-configured = Huduma hii ya kuingia haijawekwa kwenye tovuti hii.
sso-error-no-public-url = Kuingia kupitia huduma kunahitaji anwani ya umma ya tovuti hii, ambayo bado haijawekwa. Msimamizi huiweka chini ya Muunganisho.
sso-error-unreachable = Huduma ya kuingia haikuweza kufikiwa. Jaribu tena baada ya muda mfupi, au ingia kwa njia nyingine.
sso-error-refused = Huduma ya kuingia haikukuruhusu kuingia, au kuingia kulighairiwa huko.
sso-error-stale = Kuingia huko kulikuwa kumekwisha muda, au kulianzishwa kwenye kivinjari kingine. Anza tena.
sso-error-invalid = Jibu la huduma ya kuingia halikupita ukaguzi wa tovuti hii, kwa hiyo halikutumika. Anza tena; likiendelea kutokea, mwambie msimamizi.
infra-upgrade-already-pending = { $version } tayari inasubiri msaidizi wa kupandisha toleo. Subiri imalizike kabla ya kuomba toleo jingine.

connectivity-title = Muunganisho
connectivity-lede = Jinsi wasomaji wanavyofikia tovuti hii na jinsi wanavyoingia: anwani ya umma, huduma za kuingia na kanuni ambazo kila kuingia hufuata, zikiwemo za wasimamizi.
connectivity-kpi-address = Anwani ya umma
connectivity-address-unset = Haijawekwa
connectivity-address-https = HTTPS
connectivity-address-plain = Bila usimbaji
connectivity-address-unset-note = Kila huduma ya kuingia inaihitaji
connectivity-kpi-services = Huduma za kuingia
connectivity-kpi-services-note = { $n ->
        [one] iko tayari kutolewa
       *[other] ziko tayari kutolewa
    }
connectivity-kpi-passwords = Nywila
connectivity-policy-short-everyone = Kila mtu
connectivity-policy-short-non-admins = Si wasimamizi
connectivity-policy-short-nobody = Hakuna
connectivity-policy-note-everyone = Kila akaunti inaweza kuingia kwa nywila yake, na pia kwa huduma iliyounganishwa.
connectivity-policy-note-non-admins = Wasimamizi huingia kupitia huduma tu; wengine wote bado wanaweza kutumia nywila. Nywila ya msimamizi iliyoibwa basi haifungui chochote.
connectivity-policy-note-nobody = Kila mtu huingia kupitia huduma. Akaunti isiyo na huduma iliyounganishwa wala anwani ya kuunganisha nayo haiwezi kuingia.
connectivity-kpi-admins-linked = { $n ->
        [one] Msimamizi { $n } mwenye huduma iliyounganishwa
       *[other] Wasimamizi { $n } wenye huduma iliyounganishwa
    }
connectivity-kpi-registration = Usajili
connectivity-kpi-registration-go = Weka kwenye ukurasa wa Akaunti →
registration-short-closed = Wasimamizi tu
registration-short-secret = Kwa siri ya familia
registration-short-open = Wazi kwa wageni
connectivity-address = Anwani ya umma
connectivity-address-intro = Anwani ambayo wasomaji huandika, kama wanavyoiandika: https://family.example. Huduma za kuingia huwarudisha wasomaji kwake, kwa hiyo lazima iwe anwani ya umma, si ile ambayo seva hii inasikiliza.
connectivity-address-label = Anwani ya umma
connectivity-address-hint = Iache wazi wakati hakuna anayeingia kupitia huduma.
connectivity-address-invalid = Hiyo si anwani ya http au https yenye jina la mwenyeji.
connectivity-address-saved = Anwani ya umma imehifadhiwa.
connectivity-policy = Kanuni za kuingia
connectivity-policy-intro = Zinatumika kwa kila akaunti, zikiwemo za wasimamizi. Kanuni ambayo ingeacha bila msimamizi yeyote anayeweza kuingia inakataliwa.
connectivity-policy-passwords = Nani anaweza kuingia kwa nywila
connectivity-policy-without-way = { $n ->
        [one] Akaunti { $n } hai haina huduma iliyounganishwa wala anwani: chini ya “Hakuna” haingeweza kuingia.
       *[other] Akaunti { $n } hai hazina huduma iliyounganishwa wala anwani: chini ya “Hakuna” hazingeweza kuingia.
    }
connectivity-link-by-email = Unganisha huduma na akaunti yenye anwani ileile iliyothibitishwa
connectivity-link-by-email-note = Hivi ndivyo ndugu anavyopata kuingia kwa kubofya mara moja bila usajili wowote: unda akaunti yake kwa anwani yake, na kuingia kwake kwa kwanza kupitia huduma iliyothibitisha anwani hiyo kunaiunganisha. Ni anwani iliyoandikwa na msimamizi au iliyothibitishwa na huduma tu ndiyo hulinganishwa.
connectivity-domains = Vikoa vinavyoruhusiwa kufungua akaunti
connectivity-domains-hint = Kimoja kwa kila mstari, kama family.example. Tupu: chochote. Vikiwekwa, ni anwani tu iliyothibitishwa na huduma katika mojawapo ya vikoa hivi inayoweza kufungua akaunti, na fomu ya usajili haitolewi.
connectivity-domain-invalid = “{ $domain }” si jina la kikoa.
connectivity-policy-unknown = Chagua nani anaweza kuingia kwa nywila.
connectivity-policy-lockout = Imekataliwa: kwa badiliko hili hakuna msimamizi hai ambaye angeweza kuingia. Kwanza unganisha huduma ya kuingia na akaunti ya msimamizi, kwa kuingia kupitia huduma hiyo mara moja, au bakiza nywila kwa wasimamizi.
connectivity-policy-saved = Kanuni za kuingia zimehifadhiwa.
connectivity-not-saved = Haijahifadhiwa: { $error }
connectivity-services = Huduma za kuingia
connectivity-services-intro = Sajili tovuti hii kwenye dashibodi ya huduma yenyewe ukitumia anwani ya kurudisha iliyoonyeshwa, kisha bandika kitambulisho cha mteja na siri hapa. Wageni wataona “Endelea na …” kwenye kurasa za kuingia na usajili.
connectivity-outbound-note = Kuingia kupitia huduma kunahitaji seva hii kufikia huduma. Unit ambayo bootstrap.sh huandika inakataa miunganisho ya kutoka nje (IPAddressDeny=any): iruhusu kwa sudo systemctl edit axgf-cms, ukiongeza IPAddressAllow=any chini ya [Service], kisha uanzishe upya. Kitufe cha jaribio kinasema kama inaweza.
connectivity-service-ready = Tayari
connectivity-service-incomplete = Haijakamilika
connectivity-service-off = Imezimwa
connectivity-service-enable = Toa huduma hii kwenye kurasa za kuingia na usajili
connectivity-service-issuer = Mtoaji
connectivity-service-issuer-hint = Anwani ya mtoa huduma, kama https://auth.family.example/realms/family; mipangilio yake husomwa kutoka /.well-known/openid-configuration chini yake.
connectivity-service-label = Lebo ya kitufe
connectivity-service-tenant = Saraka (tenant)
connectivity-service-tenant-hint = common kwa akaunti yoyote ya Microsoft, organizations kwa akaunti za kazi na shule, consumers kwa za binafsi, au kitambulisho cha saraka yako.
connectivity-service-client-id = Kitambulisho cha mteja
connectivity-service-secret = Siri ya mteja
connectivity-service-secret-set = Siri imewekwa. Iache wazi ili kuibakiza.
connectivity-service-secret-unset = Bado hakuna siri iliyowekwa.
connectivity-service-secret-clear = Ondoa siri
connectivity-service-redirect = Anwani ya kurudisha ya kusajili
connectivity-service-redirect-needs-address = Weka anwani ya umma kwanza
connectivity-service-console = Mahali pa kusajili
connectivity-service-console-open = Dashibodi ya { $name }
connectivity-service-test = Jaribu muunganisho
connectivity-issuer-invalid = Mtoaji lazima awe anwani ya https://.
connectivity-tenant-invalid = Saraka ni jina au kitambulisho: herufi, tarakimu, nukta na vistari.
connectivity-provider-incomplete = { $name } haijakamilika: inahitaji kitambulisho cha mteja na siri ya mteja, na mtoaji kwa mtoa huduma wa jumla.
connectivity-provider-saved = { $name } imehifadhiwa.
connectivity-test-ok = { $name } imejibu: { $answer }.
connectivity-test-failed = { $name } haikuweza kufikiwa kutoka seva hii: { $detail }
registration-title = Usajili
registration-intro = Kama mgeni anaweza kufungua akaunti hapa mwenyewe. Akaunti kama hiyo ni ya mwanachama: inasoma kile wanachama wanachosoma na haibadilishi chochote. Kuipa nafasi nyingine ni uamuzi wa msimamizi, hapa chini.
registration-who = Nani anaweza kufungua akaunti
registration-note-closed = Msimamizi pekee ndiye huunda akaunti, kwenye ukurasa huu.
registration-note-secret = Mgeni anayejua siri ya familia anaweza kufungua akaunti ya mwanachama. Shiriki siri kama ambavyo ungeshiriki msimbo wa mlango.
registration-note-open = Mgeni yeyote anaweza kufungua akaunti ya mwanachama. Wanachama husoma rekodi zinazohifadhiwa kwa wanachama.
registration-rotate = Tengeneza siri mpya
registration-secret = Siri ya familia
registration-secret-note = Yeyote aliye nayo anaweza kufungua akaunti wakati hali hii imewashwa. Kutengeneza mpya kunaifungia nje ile ya zamani; akaunti zilizokwisha kufunguliwa zinabaki.
registration-link = Ukurasa wa usajili
registration-quick = Njia za haraka za kuingia
registration-quick-none = Hakuna huduma ya kuingia iliyo tayari: weka moja chini ya Muunganisho
registration-changed = Ilibadilishwa mwisho na { $by }, { $at }.
registration-unknown = Chagua nani anaweza kufungua akaunti.
accounts-kpi-total = Akaunti hai
accounts-kpi-total-note = { $disabled ->
        [one] { $disabled } imezimwa
       *[other] { $disabled } zimezimwa
    }
accounts-kpi-admins = Wasimamizi
accounts-kpi-admins-few = Ongeza wa pili
accounts-kpi-admins-ok = Zaidi ya mmoja, kama inavyopaswa
accounts-kpi-linked = Zimeunganishwa na huduma
accounts-kpi-linked-note = { $n ->
        [one] Huduma { $n } iko tayari
       *[other] Huduma { $n } ziko tayari
    }
accounts-kpi-registered = { $n ->
        [one] { $n } imefunguliwa na mgeni
       *[other] { $n } zimefunguliwa na wageni
    }
accounts-roles = Nafasi
accounts-sign-in = Huingia kwa
accounts-sign-in-password = nywila
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = kuingia mara moja
accounts-sign-in-none = bado hakuna njia ya kuingia
accounts-created-via-register = imefunguliwa na mgeni
accounts-created-via-sso = imefunguliwa kupitia huduma
accounts-add-also-register = Wageni pia wanaweza kufungua akaunti za wanachama wenyewe, kama ilivyowekwa hapo juu. Akaunti iliyoundwa hapa inaweza kuwa na nafasi yoyote.
login-password-not-allowed = Kanuni za kuingia za tovuti hii haziruhusu akaunti hii kutumia nywila: ingia kupitia huduma.
login-register = Bado huna akaunti? Unda moja
register-title = Unda akaunti
register-lede = Akaunti ya mwanachama inasoma rekodi zinazohifadhiwa kwa wanachama wa familia. Njia ya haraka zaidi ni akaunti ambayo tayari unayo.
register-closed = Akaunti hapa huundwa na msimamizi. Mwombe mmoja wao akaunti yako.
register-have-account = Tayari una akaunti? Ingia
register-or = au
register-secret = Siri ya familia
register-secret-hint = Msimamizi alikupa, au mtu wa familia alikupa.
register-secret-hint-services = Iandike kabla ya kuchagua huduma, la sivyo hatua inayofuata itaiomba.
register-domains = Ni anwani za { $domains } tu zinazoweza kufungua akaunti.
register-username-hint = Herufi, tarakimu, nukta, vistari au mistari ya chini 2 hadi 64.
register-submit = Unda akaunti
register-member-note = Hakuna anwani inayoombwa: hakuna kinachotumwa kwako, na hakuna anayeweza kudai yako.
register-form-closed = Tovuti hii haifungui akaunti kwa fomu hii.
register-throttled = Majaribio mengi mno kutoka hapa. Subiri robo saa na ujaribu tena.
register-secret-wrong = Hiyo si siri ya familia.
register-username-invalid = Jina la mtumiaji lina herufi, tarakimu, nukta, vistari au mistari ya chini 2 hadi 64.
register-password-short = Nywila inahitaji angalau herufi { $min }.
register-username-taken = Jina hilo la mtumiaji limechukuliwa. Chagua jingine.
register-not-saved = Akaunti haikuweza kuhifadhiwa. Jaribu tena baada ya muda mfupi.
register-finish-title = Hatua moja zaidi
register-finish-lede = Huduma imethibitisha wewe ni nani. Ili kufungua akaunti hapa, andika siri ya familia.
register-finish-lede-email = Huduma imekuthibitisha kama { $email }. Ili kufungua akaunti hapa, andika siri ya familia.
sso-continue = Endelea
sso-continue-with = Endelea na { $name }
sso-error-title = Kuingia hakukukamilika
sso-account-disabled = Akaunti iliyounganishwa na kuingia huku imezimwa. Muulize msimamizi.
sso-no-account = Hakuna akaunti hapa inayolingana na kuingia huku, na tovuti hii haifungui akaunti yenyewe. Mwombe msimamizi aunde yako kwa anwani hii: kuingia kwako kunakofuata kutaiunganisha.
sso-domain-refused = Akaunti hapa zinaweza kufunguliwa tu kwa anwani katika vikoa vya familia, iliyothibitishwa na huduma.

overview-accounts-note = { $admins ->
        [one] Msimamizi { $admins }
       *[other] Wasimamizi { $admins }
    }
register-password-hint = Angalau herufi { $min }. Kifungu cha maneno kadhaa ni rahisi kukumbuka kuliko mchanganyiko.

theme-clarity = Clarity
theme-clarity-note = angavu, nyeupe na machungwa · faneli
view-funnel-back = Rudi kwenye faneli
view-funnel-open = Fungua faneli
view-funnel-hero-1 = Kila kizazi ni pete.
view-funnel-hero-2 = Kila familia, faneli.
view-funnel-cta = Chunguza faneli
view-funnel-cta-detail = Kila jina linaloongezwa na kila chanzo kinachokaguliwa hufanya faneli kuwa ya kweli zaidi kwa wale watakaokuja baadaye.
view-funnel-see = Tazama kwenye faneli
view-funnel-direction = ↓ wazee zaidi kwenye ncha nyembamba · ↑ wapya zaidi mdomoni
view-funnel-still = Simamisha faneli
view-funnel-name = Faneli
view-funnel-legend = Jinsi ya kusoma faneli
view-funnel-search-hint = Enter inawaleta mbele; Shift+Enter inafungua rekodi yao.
theme-genealogy = Genealogy
theme-genealogy-note = kijani kilichokolea na krimu · mti wa kupanda
view-scroll-back = Rudi kwenye mti wa familia
view-scroll-open = Fungua mti wa familia
view-scroll-hero-1 = Kila maisha ni picha.
view-scroll-hero-2 = Kila familia, mti wa kupanda.
view-scroll-cta = Chunguza mti wa familia
view-scroll-cta-detail = Kila jina linaloongezwa na kila chanzo kinachokaguliwa hufanya mti wa familia kuwa wa kweli zaidi kwa wale wanaokuja baadaye.
view-scroll-see = Tazama kwenye mti wa familia
view-scroll-direction = ↓ kuelekea mizizi: mababu · ↑ kuelekea taji: wazao
view-scroll-still = Simamisha majani
view-scroll-name = Mti wa familia
view-scroll-legend = Jinsi ya kusoma mti wa familia
view-scroll-search-hint = Enter inawaweka kwenye shina; Shift+Enter inafungua rekodi yao.
bundle-binding-unnamed = kumbukumbu isiyo na jina
bundle-binding-created = kumbukumbu iliyoundwa tarehe { $date }
bundle-binding-named = “{ $family }”
bundle-binding-named-created = “{ $family }”, iliyoundwa tarehe { $date }

notes-section-title = Madokezo na michango
notes-section-intro = Kile ambacho wanafamilia wameandika: kumbukumbu, utafiti, barua, hati. Yatafute, yapitie kwa herufi, au fungua moja ili kulisoma na kulihifadhi kama PDF.
notes-search-label = Tafuta katika madokezo
notes-search-go = Tafuta
notes-all = Yote
notes-letters-label = Madokezo kwa herufi ya kwanza ya kichwa chake
notes-shown = Yanayoonyeshwa: { $shown } kati ya { $total }
notes-none = Bado hakuna dokezo lililoandikwa.
notes-none-found = Hakuna dokezo linalolingana na utafutaji huu.
notes-write = Andika dokezo
notes-family-wide = Familia nzima
notes-by = na { $author }
notes-about = Kuhusu: { $names }
notes-about-prefix = Kuhusu
notes-about-family = Kuhusu familia nzima
notes-pdf = Pakua kama PDF
notes-edit = Hariri
notes-close = Funga
notes-back = Rudi kwenye madokezo
notes-add-about = Andika dokezo kuhusu mtu huyu
notes-sign-in = Madokezo na michango ya familia ni kwa ajili ya wanachama wake. Ingia ili kuyasoma.
notes-admin-title = Madokezo na michango
notes-admin-intro = Andika kile ambacho rekodi haziwezi kubeba: kumbukumbu, utafiti, barua, pamoja na picha. Dokezo lililounganishwa na watu linaonekana kwenye rekodi zao; dokezo lisilounganishwa na mtu yeyote ni la familia nzima. Kila mwanachama husoma yote kwenye ukurasa wa Familia.
notes-deleted = Dokezo limefutwa.
notes-col-title = Kichwa
notes-col-category = Kundi
notes-col-date = Tarehe
notes-col-about = Kuhusu
notes-col-author = Mwandishi
notes-col-actions = Vitendo
notes-new-title = Dokezo jipya
notes-edit-title = Hariri dokezo
notes-section-about = Dokezo
notes-field-title = Kichwa
notes-field-category = Kundi
notes-field-category-hint = Maandishi huru; makundi yaliyokwisha kutumika yanapendekezwa.
notes-field-date = Tarehe
notes-field-date-hint = Mwaka, mwezi au siku: 1952, 1952-06 au 1952-06-14. Acha wazi ikiwa dokezo halina tarehe.
notes-field-people = Watu wanaohusika
notes-field-people-hint = Andika jina na ulichague kutoka kwenye orodha. Bila mtu yeyote kutajwa, dokezo ni la familia nzima.
notes-person-add = Ongeza
notes-person-remove = Ondoa mtu huyu
notes-field-text = Maandishi
notes-text-hint = Herufi nzito, za mlalo, vichwa, orodha na viungo huandikwa kwa Markdown; vitufe vilivyo juu vinakuandikia. Bandika au dondosha picha ndani ya maandishi ili kuiongeza, hadi MB { $mb } kila moja.
notes-tool-bold = Herufi nzito
notes-tool-italic = Herufi za mlalo
notes-tool-heading = Kichwa
notes-tool-list = Orodha
notes-tool-quote = Nukuu
notes-tool-link = Kiungo
notes-tool-image = Ongeza picha
notes-tool-preview = Hakiki
notes-tool-write = Rudi kuandika
notes-uploading = Inaongeza picha…
notes-save = Hifadhi dokezo
notes-cancel = Ghairi
notes-delete = Futa dokezo hili
notes-delete-confirm = Dokezo na picha zake zitaondolewa kwenye kumbukumbu.
notes-category-suggestions = Kumbukumbu, Utafiti, Barua, Hadithi, Mapishi, Hati
notes-error-title = Dokezo linahitaji kichwa. Hakuna kilichohifadhiwa.
notes-error-title-long = Kichwa ni kirefu mno: herufi 200 zaidi. Hakuna kilichohifadhiwa.
notes-error-text-long = Maandishi ni marefu mno. Hakuna kilichohifadhiwa.
notes-error-date = Tarehe hii haieleweki. Andika mwaka, mwezi au siku: 1952, 1952-06 au 1952-06-14. Hakuna kilichohifadhiwa.
notes-error-person = Watu wanaohusika: “{ $name }”.
notes-error-image-gone = Picha iliyobandikwa haikuwa tena ikisubiri kuhifadhiwa; picha husubiri saa sita. Iongeze tena. Hakuna kilichohifadhiwa.
notes-error-too-many-images = Dokezo linabeba picha 40 zaidi. Hakuna kilichohifadhiwa.
notes-error-stale = Mtu fulani alibadilisha dokezo hili ulipokuwa ukilifanyia kazi. Lifungue tena ili kuona toleo lake, kisha fanya badiliko lako. Hakuna kilichohifadhiwa.
notes-error-refused = Kumbukumbu ilikataa badiliko hili. Hakuna kilichohifadhiwa.
notes-error-image = Faili hii si picha ambayo tovuti hii inaweza kuonyesha: PNG, JPEG, GIF au WebP.
notes-error-image-large = Picha hii ni kubwa mno.
notes-stale-title = Dokezo halikubadilishwa
notes-missing-title = Dokezo halikupatikana
notes-missing = Hakuna dokezo hapa, au huruhusiwi kulisoma.
notes-not-yours = Mchangiaji hubadilisha madokezo aliyoyaandika tu. Msimamizi anaweza kubadilisha hili.
notes-pdf-failed-title = PDF haikuweza kutengenezwa
notes-pdf-failed = Kuna hitilafu wakati wa kutengeneza PDF ya dokezo hili. Lisome kwenye ukurasa wake kwa sasa, na mwambie msimamizi.
notes-dashboard-count = Madokezo yaliyoandikwa: { $n }
notes-dashboard-open = Andika na usimamie madokezo

kind-bundle = kumbukumbu
manifest-title = Familia
manifest-intro = Kile ambacho kumbukumbu inasema kuhusu familia inayoibeba. Jina lake ndilo linaloonyeshwa kwenye ukurasa wa Familia, vichwa vya kurasa na PDF ya kila dokezo; mengine husafiri pamoja na faili. Takwimu na tarehe za faili yenyewe huhesabiwa inapoandikwa.
manifest-name = Jina la familia
manifest-name-hint = Huonyeshwa kwenye ukurasa wa Familia na kwenye vichwa. Akaunti hufuata jina jipya.
manifest-description = Maelezo
manifest-culture = Utamaduni mkuu
manifest-culture-hint = Lebo ya lugha: sw, fr, pt-BR…
manifest-place = Mahali pakuu
manifest-place-hint = Bara ambalo familia iliishi zaidi.

manifest-span = Kipindi kinachohusika
manifest-span-hint = Kuanzia kuzaliwa kwa mapema zaidi hadi kwa karibuni zaidi kunakorekodiwa na kumbukumbu; kunahesabiwa, na kuandikwa kwenye faili maelezo yanapohifadhiwa.
manifest-license = Leseni
manifest-license-type = Leseni
manifest-save = Hifadhi maelezo ya familia
manifest-saved = Maelezo ya familia yamehifadhiwa. Akaunti hufuata jina lake.
manifest-error-long = Sehemu moja ni ndefu mno: herufi 120 kwa jina, 2,000 kwa maelezo. Hakuna kilichohifadhiwa.
manifest-error-culture = Utamaduni ni lebo ya lugha, kama sw, fr au pt-BR. Hakuna kilichohifadhiwa.
manifest-error-stale = Mtu fulani alibadilisha maelezo ya familia ulipokuwa ukiyahariri. Toleo lako liko hapa chini: kulihifadhi tena kunachukua nafasi ya lake. Hakuna kilichohifadhiwa.
manifest-error-io = Kumbukumbu haikuweza kuandikwa; kumbukumbu za seva zinaeleza kwa nini. Hakuna kilichohifadhiwa.

manifest-none = Haijasemwa
manifest-span-value = { $earliest } – { $latest }
manifest-span-none = Bado hakuna tarehe ya kuzaliwa iliyorekodiwa.
manifest-license-pick = Chagua leseni ili kuona inawaruhusu wengine kufanya nini na kumbukumbu.
manifest-license-author = Mwandishi
manifest-license-author-value = { $who }, { $at }
manifest-license-author-none = Hurekodiwa leseni inapochaguliwa.
manifest-error-place = Chagua mahali pakuu kutoka kwenye orodha. Hakuna kilichohifadhiwa.
manifest-error-licence = Chagua leseni kutoka kwenye orodha. Hakuna kilichohifadhiwa.
continent-africa = Afrika
continent-antarctica = Antaktika
continent-asia = Asia
continent-europe = Ulaya
continent-north-america = Amerika Kaskazini
continent-south-america = Amerika Kusini
continent-oceania = Oceania
continent-several = Mabara kadhaa
licence-all-rights-reserved-name = Haki zote zimehifadhiwa
licence-all-rights-reserved-about = Hakuna anayeweza kunakili, kushiriki au kutumia tena kumbukumbu bila ruhusa ya familia. Ndivyo inavyokuwa pale hakuna kilichosemwa.
licence-family-private-name = Faragha, kwa familia
licence-family-private-about = Kwa ajili ya wanafamilia tu: si ya kuchapishwa wala kupitishwa nje ya familia.
licence-cc0-name = CC0 1.0 — mali ya umma
licence-cc0-about = Yeyote anaweza kunakili, kubadilisha na kutumia kumbukumbu kwa kusudi lolote, hata la kibiashara, bila kuomba wala kumtaja yeyote.
licence-cc-by-name = CC BY 4.0 — Kutaja mwandishi
licence-cc-by-about = Yeyote anaweza kushiriki na kurekebisha kumbukumbu, hata kibiashara, mradi awataje waandishi wake na aseme alichobadilisha.
licence-cc-by-sa-name = CC BY-SA 4.0 — Kutaja mwandishi, Kushiriki kwa masharti yaleyale
licence-cc-by-sa-about = Yeyote anaweza kuishiriki na kuirekebisha, hata kibiashara, akiwataja waandishi wake; anachotengeneza kutoka kwake lazima kishirikiwe kwa leseni ileile.
licence-cc-by-nc-name = CC BY-NC 4.0 — Kutaja mwandishi, Si kwa biashara
licence-cc-by-nc-about = Yeyote anaweza kuishiriki na kuirekebisha, akiwataja waandishi wake, lakini si kwa makusudi ya kibiashara.
licence-cc-by-nc-sa-name = CC BY-NC-SA 4.0 — Kutaja mwandishi, Si kwa biashara, Kushiriki kwa masharti yaleyale
licence-cc-by-nc-sa-about = Yeyote anaweza kuishiriki na kuirekebisha, akiwataja waandishi wake, si kwa makusudi ya kibiashara, na kwa leseni ileile tu.
licence-cc-by-nd-name = CC BY-ND 4.0 — Kutaja mwandishi, Bila mabadiliko
licence-cc-by-nd-about = Yeyote anaweza kuishiriki kama ilivyo, hata kibiashara, akiwataja waandishi wake, lakini hawezi kushiriki toleo lililobadilishwa.
licence-cc-by-nc-nd-name = CC BY-NC-ND 4.0 — Kutaja mwandishi, Si kwa biashara, Bila mabadiliko
licence-cc-by-nc-nd-about = Yeyote anaweza kuishiriki kama ilivyo, akiwataja waandishi wake, lakini si kwa makusudi ya kibiashara wala katika toleo lililobadilishwa: leseni kali zaidi ya Creative Commons.
licence-odbl-name = ODbL 1.0 — Leseni ya Hifadhidata Huria
licence-odbl-about = Imetengenezwa kwa hifadhidata: yeyote anaweza kutumia, kushiriki na kurekebisha data, akitaja chanzo chake, na hifadhidata iliyotengenezwa kutoka kwake lazima ibaki huria kwa masharti yaleyale.

nav-account = Akaunti yangu
account-title = Akaunti yangu
account-who = Akaunti
account-email = Anwani ya barua pepe
account-since = Tangu
account-person-title = Rekodi yangu kwenye mti
account-person-intro = Sema wewe ni mtu yupi kwenye mti. Hakuna anayekagua: kila akaunti huchagua rekodi yake, kwa jukumu lake, na inaweza kuibadilisha au kuiondoa wakati wowote.
account-person-is = Akaunti hii inasema ni
account-person-unlink = Ondoa kiungo
account-person-pick = Mtu kwenye mti
account-person-link = Unganisha akaunti yangu na rekodi hii
account-person-change = Unganisha na rekodi hii badala yake
account-person-linked = Akaunti yako imeunganishwa na rekodi yake.
account-person-unlinked = Akaunti yako haijaunganishwa tena na rekodi.
account-person-error = Rekodi hii haikuweza kuunganishwa:
account-person-io = Akaunti haikuweza kuhifadhiwa; kumbukumbu za seva zinaeleza kwa nini. Hakuna kilichobadilishwa.
account-emergency-page = Tokeni ya dharura si akaunti: haina ukurasa wake wala rekodi kwenye mti.
accounts-person = Rekodi:

## The merge screen (src/routes/merge.rs): two records a person says are one.
merge-action = Unganisha
merge-needs-two = Chagua rekodi mbili hasa ili kuziunganisha.
merge-title = Unganisha rekodi mbili
merge-lede = Hakuna kinachobadilika hadi uthibitishe. Rekodi ya kwanza inabaki; ya pili inaunganishwa ndani yake na kuhifadhiwa kama ilivyokuwa katika kumbukumbu ya mabadiliko.
merge-keep = Inabaki
merge-absorb = Inaunganishwa ndani yake
merge-swap = Baki na nyingine
merge-refused = Muunganisho huu umekataliwa:
merge-refusal-ancestor = moja ni babu au bibi wa nyingine
merge-refusal-spouse-and-child = katika familia moja, mmoja ni mwenzi na mwingine ni mtoto
merge-refusal-spouses = ni wenzi wawili wa muungano mmoja
merge-refusal-siblings = wameandikwa kama watoto wa wazazi walewale
merge-refusal-different-couples = familia hizo mbili ni wanandoa tofauti
merge-refusal-child-is-spouse = mtoto wa familia moja ni mwenzi katika nyingine
merge-refusal-same-record = ni rekodi ileile mara mbili
merge-conflicts = Zinapotofautiana
merge-nothing = Hakuna cha kuchagua: hazipingani.
merge-choice-keep = Rekodi inayobaki
merge-choice-absorb = Rekodi nyingine
merge-choice-both = Zote mbili
merge-because-higher-confidence = Pendekezo: thamani yenye uhakika zaidi.
merge-because-sourced = Pendekezo: thamani inayotaja chanzo.
merge-because-kept-record = Pendekezo: thamani ya rekodi inayobaki.
merge-because-more-restrictive = Pendekezo: mwonekano wenye vikwazo zaidi.
merge-because-protective = Pendekezo: yu hai — usomaji unaolinda.
merge-because-both-kept = Pendekezo: hifadhi maandishi yote mawili.
merge-becomes-alias = Jina lisilochaguliwa linahifadhiwa kama jina jingine.
merge-becomes-event = Thamani isiyochaguliwa inahifadhiwa kama tukio.
merge-agreements = Zinapokubaliana
merge-from-keep = kutoka rekodi inayobaki
merge-from-absorb = kutoka rekodi nyingine
merge-from-both = sawa pande zote mbili
merge-from-joined = orodha zimeunganishwa
merge-from-combined = zimeunganishwa
merge-effects = Kingine kinachobadilika
merge-effect-references = Marejeo yaliyoandikwa upya
merge-effect-self-links = Viungo vya kujirejelea vilivyoondolewa
merge-effect-listed-twice = Orodha ambazo zingeitaja mara mbili
merge-effect-same-couple = Familia za wanandoa walewale
merge-confirm-label = Huenda bado ni mtu mmoja. Ili kuunganisha hata hivyo, andika jina hasa: { $name }
merge-go = Unganisha
merge-missing = Moja ya rekodi hizo mbili haipo tena.
merge-done-title = Zimeunganishwa
merge-done-lede = { $absorbed } sasa ni sehemu ya { $kept }.
merge-report-references = Marejeo yaliyoandikwa upya
merge-report-events = Matukio yaliyotokana na thamani isiyochaguliwa
merge-report-source-conflicts = Tofauti zilizoandikwa kwenye chanzo
merge-report-links = Viungo vilivyoondolewa
merge-report-tidied = Maingizo yaliyopangwa
merge-absorbed-record = Rekodi iliyounganishwa, kama ilivyokuwa
merge-open-kept = Fungua rekodi iliyobaki
