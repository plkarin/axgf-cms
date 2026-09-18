# axgf-cms — textos de la interfaz, español.
#
# CALIDAD AUTOMÁTICA — no revisado por una persona de lengua materna española.
# El vocabulario genealógico tiene equivalencias asentadas que varían según la
# tradición archivística, y esta traducción puede estar equivocada. Se
# agradecen las correcciones — véase CONTRIBUTING.md.
#
# Elecciones adoptadas (discutibles):
#   union → unión · link → vínculo · confidence → certeza
#   reliability → fiabilidad · source → fuente
#   primary source → fuente primaria · occupation → ocupación
#   record → ficha · archive → archivo · godparent → padrino/madrina
#   witness → testigo · speculative → hipotético
#
# Plural: reglas CLDR one / other. Nunca sustituirlas por lógica propia.
#
# Fechas: «12 de abril de 1923» — con las dos preposiciones y el mes en
# minúscula. La tabla de meses vive dentro del propio patrón de fecha.
#
# REGLA: este archivo traduce solo la interfaz. Nombres, lugares, notas y
# oficios vienen del archivo familiar y se quedan en su lengua y su escritura.

app-name = ax-genealogy

## Cabecera y pie

nav-tree = Árbol
nav-convert = Importar
nav-admin = Administración
nav-sign-in = Iniciar sesión
nav-sign-out = Cerrar sesión
footer-open-format = El archivo de su familia es un solo fichero que se queda con usted, escrito en un formato abierto: seguirá abriéndose mucho después de que este sitio haya desaparecido.
footer-open-format-link = Sobre el formato

## Preferencias

prefs-title = Idioma y apariencia
prefs-language = Idioma
prefs-theme = Apariencia
prefs-background = Fondo
prefs-background-on = Un velo suave de color detrás de la página
prefs-apply = Aplicar
prefs-reviewed = revisada
prefs-machine = automática, { $coverage } %
prefs-machine-complete = completa, aún sin revisar
prefs-machine-title = Traducida sin revisión por una persona de lengua materna. El vocabulario genealógico en particular puede estar equivocado: las palabras para una unión, un padrino o una fuente primaria cambian según la tradición archivística de cada país. Se agradecen las correcciones, y CONTRIBUTING.md dice por dónde empezar.

theme-light = Claro
theme-dark = Oscuro
theme-system = Como el sistema
theme-high-contrast = Contraste alto
theme-sepia = Sepia
theme-deuteranopia = Deuteranopía
theme-protanopia = Protanopía
theme-tritanopia = Tritanopía
theme-colour-blind-note = apto para daltonismo
theme-contrast-note = contraste máximo

## Árbol

tree-title-around = Alrededor de { $name }
tree-title-whole = El árbol entero
tree-lede-focused = { $ancestors ->
        [one] Un antepasado
       *[other] { $ancestors } antepasados
    }, { $descendants ->
        [one] un descendiente
       *[other] { $descendants } descendientes
    } y { $spouses ->
        [one] una pareja
       *[other] { $spouses } parejas
    }, { $depth } generaciones a cada lado.
tree-filter-label = Filtrar las fichas visibles
tree-filter-placeholder = Escriba un nombre…
tree-centre-on = Centrar en
tree-depth = Generaciones a cada lado
tree-show = Mostrar
tree-hidden-notice = { $n ->
        [one] Una persona se muestra sin sus datos
       *[other] { $n } personas se muestran sin sus datos
    }
tree-hidden-because-role = , porque su visibilidad está por encima de lo que su cuenta puede leer.
tree-hidden-because-anonymous = , porque no son públicas.
tree-hidden-sign-in = Inicie sesión si tiene una cuenta.
tree-restricted-card = Esta ficha no es visible para usted
tree-empty = Todavía no hay a quién dibujar.
tree-unplaced = En ninguna familia registrada

## La ficha

record-identity = Identidad
record-life-events = Hechos de la vida
record-family = Familia
record-other-relationships = Otras relaciones
record-occupations = Ocupaciones
record-places = Lugares
record-sources-documents = Fuentes y documentos
record-notes = Notas
record-history = Historial de cambios
record-raw = Datos en bruto
record-raw-summary-note = el JSON con el que se construyó esta página

record-identity-help = Cada nombre registrado con su tipo, el periodo en que se usó y la fuente que lo respalda, con la escritura propia junto a la transliteración latina allí donde difieren, además del género, si vive y su visibilidad.
record-life-events-help = Nacimiento, defunción y cada hecho en el que participó esta persona, por fecha, cada uno con su papel — así una boda de la que solo fue testigo aparece junto a la suya propia. Un dato sin fecha va al final, en lugar de fingir que viene primero.
record-family-help = Padres y hermanos, luego cada unión con su tipo, sus fechas, su lugar, cómo terminó y sus hijos por orden de nacimiento.
record-other-relationships-help = Cada vínculo con esta persona en uno de los extremos, leído desde su lado: la misma ficha aparece como «padrino de» desde un extremo y «ahijado de» desde el otro.
record-occupations-help = Las ocupaciones como periodos sobre un mismo eje, de modo que dos empleos se comparan a ojo; donde falta un extremo la barra queda abierta.
record-places-help = Cada lugar que toca esta ficha, con lo que allí ocurrió y con la historia de fronteras que hace que un lugar tenga sentido a lo largo del tiempo.
record-sources-documents-help = Cada fuente nombra los hechos de esta página que se apoyan en ella, ordenados por la fuerza de la prueba.
record-notes-help = Notas sobre esta ficha, incluido el texto que ningún convertidor supo interpretar y que se conservó literalmente en lugar de descartarlo.
record-history-help = Cada cambio guardado de esta ficha, el más reciente primero. Quién corrigió qué es un hecho sobre las personas que llevan el árbol, no sobre la familia que está en él: por eso queda fuera del archivo exportado y solo se muestra a los parientes que han iniciado sesión.
record-raw-help = Aquí no hay nada generado para mostrar: esta es la ficha exactamente como está guardada, hasta los nombres de los campos. Si alguna vez tuviera que leer el archivo sin este sitio, vería justamente esto.
record-help-toggle = Qué muestra esta sección

record-gender = Género
record-living = Vive
record-visibility = Visibilidad
record-yes = sí
record-no = no
record-name-type = Tipo de nombre
record-name-used = En uso
record-name-evidence = Prueba
record-transliteration = Transliteración latina
record-born = Nacido/a
record-died = Fallecido/a
record-parents = Padres
record-siblings = Hermanos
record-children = Hijos
record-unknown-person = [Desconocido]
record-restricted-person = Reservada
record-restricted-title = Esta ficha no es visible para usted
record-absent-person-title = Nombrado en este árbol pero sin ficha propia
record-confidence = Certeza
record-source = Fuente
record-download = Descargar

## Acceso

access-restricted-title = No visible para usted
access-restricted-signed-in = La visibilidad de esta ficha está por encima de lo que su cuenta puede leer. Un administrador puede cambiar o la visibilidad de la ficha o su papel.
access-restricted-anonymous = Esta ficha no es pública. Inicie sesión para ver si su cuenta puede leerla.
access-role-title = No para su papel
access-role-admin = Esta es una página de administrador. Su cuenta puede crear y editar fichas, pero no gestionar cuentas, borrar fichas ni exportar el archivo.
access-role-write = Su cuenta puede leer este árbol pero no cambiarlo. Un administrador puede elevar su papel a colaborador.
access-scope-title = Fuera de su rama
access-scope-named = Su cuenta está limitada a una rama del árbol, y esta ficha atañe a alguien de fuera. Cada persona nombrada en una ficha tiene que estar dentro de su rama: si no, una familia con una pareja de fuera sería una manera de reescribir la filiación de esa persona.
access-scope-unnamed = Su cuenta está limitada a una rama del árbol, y esta ficha no nombra a nadie con quien contrastarla. Las fuentes y los lugares los editan las cuentas con acceso al árbol entero.

## Errores

error-not-found-title = No encontrado
error-not-found-detail = Esa página no existe aquí.
error-no-such-person-title = No hay tal persona
error-no-such-person-detail = Aquí no hay ninguna persona con ese identificador.
error-no-such-entity-title = No hay tal elemento
error-no-such-entity-detail = Aquí no hay ninguna ficha con ese identificador.
error-deleted-while-editing = Aquí no hay ninguna ficha con ese identificador. Puede que la borraran mientras usted la editaba.
error-no-such-file-title = No hay tal fichero
error-no-such-file-detail = Aquí no hay ningún documento con ese identificador, o el documento está registrado sin fichero — un documento citado nombra algo guardado en otro sitio.
error-not-an-image-title = No es una imagen
error-not-an-image-detail = Para este documento no hay miniatura, porque no es una imagen que esta versión sepa descodificar.
error-back = Atrás

## Inicio de sesión

login-title = Iniciar sesión
login-lede = Las cuentas las crea un administrador.
login-username = Nombre de usuario
login-password = Contraseña
login-submit = Iniciar sesión
login-wrong = Ese nombre de usuario y esa contraseña no coinciden.
login-token-wrong = Ese testigo no es correcto.
login-throttled = Demasiados intentos fallidos. Espere unos minutos y pruebe otra vez.
login-no-accounts-title = Esta instalación todavía no tiene cuentas.
login-no-accounts-detail = Aquí no hay página de configuración a propósito: el rato entre la puesta en marcha y el primer acceso es justo el momento en que una instalación está desprotegida, así que el primer administrador se crea desde la línea de órdenes.
login-no-accounts-note = Imprime una contraseña generada en stderr una sola vez y nunca más. Hasta entonces la única entrada es el testigo de emergencia de abajo.
login-emergency-summary = Acceso de emergencia
login-emergency-detail = El testigo compartido sigue abriendo una sesión de administrador y existe para una sola cosa: volver a entrar cuando el fichero .acl se ha perdido o todos los administradores están fuera. No es una cuenta: no tiene preferencias propias, y el diario de cambios lo anota como emergency-token en lugar de como persona. Su uso se registra como advertencia.
login-emergency-label = Testigo de emergencia
login-emergency-submit = Usar el testigo de emergencia
login-sign-in-prompt = Inicie sesión para llegar al panel de administración.

## Administración

admin-title = Administración
admin-lede = Se edita { $path } — { $total } elementos, { $files ->
        [one] un fichero adjunto
       *[other] { $files } ficheros adjuntos
    }, { $size } en disco. Cada cambio se escribe de una vez; un cambio rechazado deja el fichero intacto.
admin-entities = Elementos
admin-create = Crear
admin-new-kind = Nuevo: { $kind }
admin-operations = Operaciones
admin-validate = Comprobar
admin-deduplicate = Unir duplicados
admin-export = Exportar el archivo
admin-accounts = Cuentas
admin-roles-note = Comprobar, unir duplicados, exportar, borrar y gestionar cuentas es solo del administrador. Un colaborador llega a todas las demás páginas de aquí.
admin-dedup-confirm = Unir duplicados funde fichas y reescribe el archivo. ¿Continuar?
admin-recent-changes = Cambios recientes
admin-recent-note = Los últimos { $shown } de { $total ->
        [one] un cambio registrado
       *[other] { $total } cambios registrados
    }, desde { $path }.
admin-sessions-open = { $n ->
        [one] Una sesión abierta ahora mismo.
       *[other] { $n } sesiones abiertas ahora mismo.
    }
admin-no-changes-yet = Con esta aplicación todavía no se ha cambiado nada. Cada guardado a partir de ahora queda anotado en { $path }.
admin-last-validation = Última comprobación
admin-bundle-heavy = Este archivo pesa { $size }. Se carga entero al arrancar y se mantiene en memoria, así que a partir de unos { $warn } el sitio empieza a costar memoria de verdad y los reinicios se vuelven lentos. Eso le va bien a un archivo familiar, no a una mediateca: si los adjuntos crecen sin límite, guárdelos en un almacén de ficheros y que el archivo apunte a ellos.

admin-fields = Campos
admin-raw-json = JSON en bruto
admin-raw-json-help = El elemento entero, de modo que nada quede sin poder editarse: las listas como las parejas y los hijos de una familia, o la historia de fronteras de un lugar, viven justamente aquí. Este es el documento de partida; los campos de arriba se escriben después sobre las rutas que les pertenecen, así que edite un valor en un sitio o en el otro, no en los dos. Tiene que leerse como JSON o no se guarda nada.
admin-save = Guardar
admin-cancel = Cancelar
place-editor-title = Editar un lugar
place-add-detail = Completar este lugar
place-names = Nombres
place-name-primary = Principal
place-name-lang = Idioma
place-name-value = Nombre
place-names-hint = Una fila por cada nombre registrado. Un lugar administrado por tres imperios lleva tres nombres; el principal es el que se muestra en todas partes.
place-where = Ubicación
place-type = Tipo
place-region = Región
place-country-current = País actual
place-country-hint = ISO 3166-1 alfa-2, por ejemplo PL, FR, DE.
place-country-history = Historia de fronteras
place-history-country = Estado
place-history-from = Desde
place-history-until = Hasta
place-country-history-hint = Qué Estado tuvo este lugar y durante qué periodo. Importa en genealogía: un acta escrita en ruso en 1880 y otra escrita en polaco en 1930 pueden nombrar el mismo pueblo.
place-coordinates = Coordenadas
place-lat = Latitud
place-lon = Longitud
place-precision = Precisión
place-identifiers = Identificadores
place-wikidata = Wikidata
place-geonames = GeoNames
place-used-by = { $name } se usa en { $n ->
        [one] otro registro
       *[other] otros { $n } registros
    }.
place-error-no-name = Un lugar necesita al menos un nombre.
place-error-coords-pair = Latitud y longitud van juntas: indique ambas o ninguna.
place-error-coords-number = La latitud y la longitud deben ser números.
place-error-coords-range = La latitud va de -90 a 90 y la longitud de -180 a 180.
place-type-continent = continente
place-type-country = país
place-type-region = región
place-type-department = departamento
place-type-city = ciudad
place-type-village = pueblo
place-type-district = barrio
place-type-street = calle
place-type-building = edificio
place-type-farm = granja
place-type-island = isla
place-type-historical = histórico
place-type-unknown = desconocido
place-precision-exact = exacta
place-precision-building = edificio
place-precision-street = calle
place-precision-city_center = centro de la ciudad
place-precision-region_center = centro de la región
place-precision-country_center = centro del país
place-precision-approximate = aproximada

place-coordinates-hint = Escribirlas a mano es lo habitual. Muchos lugares registrados bajo una administración anterior no aparecen en una búsqueda moderna.
place-geocode-search = Buscar este nombre
place-geocode-hint = Envía el nombre, la región y el país al servicio de geocodificación, un lugar cada vez. No se guarda nada hasta que usted guarde.
place-geocode-off = La búsqueda de nombres está desactivada. Necesita una dirección de contacto con la que el servicio identifique esta instalación; arranque el servidor con --geocoder-contact para activarla.
place-geocode-query = Se buscó: { $q }
place-geocode-error = No se pudo contactar con el servicio de búsqueda. Los campos de coordenadas de arriba siguen funcionando.
place-geocode-none = No se encontró nada. Para una aldea registrada bajo administración rusa, prusiana o austriaca este es el resultado corriente; introduzca la posición a mano.
place-geocode-not-a-place = no es una población
place-geocode-use = Usar este
place-geocode-attribution = Resultados de OpenStreetMap mediante Nominatim, bajo la Open Database License.

place-paste = Pegar una posición
place-paste-placeholder = un enlace de mapa, o 52.0782795, 21.2508068
place-paste-read = Leerlo
place-paste-hint = Un enlace de Google Maps u OpenStreetMap, un URI geo:, un par de números, o grados-minutos-segundos como 52°04'41.8"N 21°15'02.9"E.
place-paste-read-ok = Leído en los campos de arriba. Compruébelo y guarde.
place-paste-unreadable = Esa no es una posición que se pueda leer aquí. Los campos de arriba siguen aceptando un par de números.

place-map-hint = Haga clic en el mapa para poner el punto, o arrastre el alfiler. Lo que vale son los campos de arriba.
place-map-clear = Quitar el punto
place-open-in-map = Buscar este lugar en OpenStreetMap y pegar aquí el enlace

person-tab-record = Ficha
person-tab-life = Vida
person-tab-media = Materiales
person-tab-tree = Árbol
person-tree-depth = { $n } generaciones a cada lado. El árbol entero está más abajo.
person-tree-alone = Esta ficha no nombra padres, parejas ni hijos, así que no hay forma alguna que dibujar a su alrededor.

record-no-evidence = A esta ficha no hay nada adjunto: ni fuente ni documento. Es el estado corriente de un archivo convertido, no un defecto suyo: el GEDCOM se lleva los hechos y deja atrás lo que los probaba.
record-no-evidence-signed-out = Inicie sesión para adjuntar algo.
admin-delete = Borrar
admin-not-set = — sin establecer —
admin-edit = Editar
admin-page-of = Página { $page } de { $pages }
admin-previous = Anterior
admin-next = Siguiente
admin-saved = Guardado como versión { $version } — { $summary }
admin-not-saved = No guardado
admin-created = Creado
admin-not-created = No creado
admin-deleted = Borrado
admin-not-deleted = No borrado — no se cambió nada
admin-what-changed = qué cambió
admin-field = Campo
admin-from = De
admin-to = A
admin-version = versión { $version }

## Cuentas

accounts-title = Cuentas
accounts-lede = Guardadas en { $path }, con permisos 600, junto al archivo y nunca dentro. Un archivo se copia, se envía y se publica; los resúmenes de las contraseñas viajando dentro harían de cada copia del árbol una copia de las credenciales.
accounts-existing = Existentes
accounts-username = Nombre de usuario
accounts-role = Papel
accounts-status = Estado
accounts-branch = Rama
accounts-last-seen = Último acceso
accounts-change = Cambiar
accounts-you = (usted)
accounts-active = activa
accounts-disabled = desactivada
accounts-never = nunca
accounts-whole-tree = árbol entero
accounts-roots = { $n ->
        [one] una raíz
       *[other] { $n } raíces
    }
accounts-add = Añadir una cuenta
accounts-no-registration = A propósito no hay ni registro por cuenta propia ni invitaciones. Para un archivo familiar basta con un administrador que conozca a todos, y eso retira del todo una superficie de abuso en lugar de tener que defenderla.
accounts-password-hint = Déjelo en blanco y se genera una y se muestra una sola vez. Al menos { $min } caracteres si la pone usted.
accounts-new-password-placeholder = nueva contraseña (en blanco = sin cambio)
accounts-email = Correo electrónico
accounts-optional = (opcional)
accounts-create = Crear la cuenta
accounts-role-viewer = lectura — lee las fichas públicas y las de la familia
accounts-role-contributor = colaborador — además crea, edita y sube ficheros
accounts-role-admin = administrador — además gestiona cuentas, borra y exporta
accounts-branch-hint = Limita lo que esta cuenta puede editar a esas personas, sus descendientes y sus cónyuges.
accounts-branch-reading = No limita lo que puede leer: eso lo gobierna la visibilidad de cada ficha, y las dos cosas se mantienen separadas a propósito.
accounts-branch-placeholder = un identificador de persona por línea
accounts-ids-in-bundle = Identificadores de personas en este árbol
accounts-emergency-warning = Ha entrado con el testigo de emergencia. Concede derechos de administrador durante esta sesión pero no es una cuenta: no tiene preferencias propias, y el diario anotará sus cambios como emergency-token en lugar de como persona. Créese abajo una cuenta de verdad y entre con ella.
accounts-created-with-password = Creada { $username }. Su contraseña es { $password } — se muestra una sola vez y se guarda solo como resumen Argon2id, así que pásela ahora.
accounts-created = Creada { $username }.
accounts-updated = Actualizada { $username }. Se ha cerrado cualquier sesión que tuviera abierta.
accounts-username-taken = Ese nombre de usuario ya está cogido.
accounts-pick-role = Elija un papel.
accounts-no-such = No existe esa cuenta.
accounts-last-admin = Es el único administrador activo. Ascienda antes a otra persona: una instalación sin administrador solo se recupera editando el fichero .acl o usando el testigo de emergencia.
accounts-not-saved = No guardado: { $error }

## Conflictos

conflict-title = Otra persona lo cambió antes
conflict-lede = { $who } guardó un cambio en este elemento ({ $kind }) a las { $when }, después de que usted lo abriera. Su edición no se ha guardado y no se ha sobrescrito nada.
conflict-no-merge = Aquí no se une nada de forma automática. Fundir las ediciones de dos personas produce una ficha que no eligió ninguna de las dos, y en genealogía dos editores en desacuerdo sobre una fecha suelen estar leyendo fuentes distintas — y eso es una pregunta para una persona, no para un programa. Compare las dos de abajo y decida.
conflict-versions = Usted partió de la versión { $expected }; la ficha está ahora en la versión { $current }.
conflict-both-changed = Lo cambiaron los dos
conflict-both-changed-detail = Estos campos los editaron los dos. Lo que guarde sustituirá lo que puso { $who }:
conflict-different-fields = Cambiaron campos distintos, así que nada del trabajo de { $who } está en discusión — pero volver a aplicar escribe de todos modos su elemento entero sobre el de la otra persona. Compruebe las dos columnas antes de guardar.
conflict-field-by-field = Campo por campo
conflict-theirs = En qué lo cambió { $who }
conflict-yours = En qué lo cambió usted
conflict-unchanged-by-you = sin cambiar por usted
conflict-unchanged-by-them = sin cambiar por ellos
conflict-nothing-differs = Ninguna de las dos versiones se aparta de aquella de la que partió usted en ningún campo de los que muestra esta página. El número de versión avanzó, así que alguien guardó la ficha sin cambiar nada de lo que contiene.
conflict-what-now = Y ahora
conflict-reapply = Volver a aplicar su versión sobre la de la otra persona
conflict-reapply-hint = Esta es su edición, llevada a la versión { $version }. Corríjala aquí para conservar lo que quiera del trabajo de { $who } y luego guarde. La versión de la otra persona se muestra abajo para copiar de ella.
conflict-save-over = Guardar esta sobre la suya
conflict-discard = Descartar la mía y empezar de nuevo
conflict-their-version = La versión de { $who }, tal como está ahora
conflict-history-of = Historial de este elemento ({ $kind })

## Importación

convert-title = Importar un fichero familiar
convert-submit = Importar
convert-result-title = Informe de importación
convert-download = Descargar el archivo

## Fechas

date-unknown = Fecha desconocida
date-not-recorded = Sin registrar
date-circa = hacia { $date }
date-between = entre { $from } y { $to }
date-before = antes de { $date }
date-after = después de { $date }
date-preserved = registrada como «{ $text }»
date-day-month-year = { $day } de { $month ->
        [1] enero
        [2] febrero
        [3] marzo
        [4] abril
        [5] mayo
        [6] junio
        [7] julio
        [8] agosto
        [9] septiembre
        [10] octubre
        [11] noviembre
        [12] diciembre
        *[other] { $month }
    } de { $year }
date-month-year = { $month ->
        [1] enero
        [2] febrero
        [3] marzo
        [4] abril
        [5] mayo
        [6] junio
        [7] julio
        [8] agosto
        [9] septiembre
        [10] octubre
        [11] noviembre
        [12] diciembre
        *[other] { $month }
    } de { $year }
date-decade = los años { $decade }
date-century = el siglo { $century }
date-quarter-century = el { $quarter ->
        [1] primer
        [2] segundo
        [3] tercer
       *[other] cuarto
    } cuarto del siglo { $century }

## Más páginas de error

error-back-to-start = Volver al principio
error-payload-missing-title = No hay tal fichero
error-payload-missing-detail = El contenido de ese documento no está en la caché.
error-payload-unopenable-detail = El contenido de ese documento no se pudo abrir.
error-no-such-document-detail = Aquí no hay ningún documento con ese identificador.
error-bad-preference-title = No es una de las opciones
error-bad-preference-detail = No es ni un idioma ni una apariencia que este sitio ofrezca. No se cambió nada.
error-unknown-kind-title = Clase desconocida
error-unknown-kind-detail = «{ $kind }» no es una clase de ficha. Este archivo contiene: { $kinds }.
error-io-title = No se pudo guardar
error-io-detail = { $error }. En el disco no se cambió nada.
error-upload-too-large = Ese fichero pasa del límite de { $mb } MB. No se guardó nada y el archivo queda igual.
error-upload-refused = El documento fue rechazado: { $reason }. El archivo queda igual.
error-back-to-person = Volver a la ficha
error-no-such-person-to-attach = Aquí no hay ninguna persona con ese identificador, así que no hay a qué adjuntar un documento.
error-upload-title = Esa subida no se guardó
error-download-expired-title = Esa descarga ha caducado
error-download-expired-detail = Una importación se guarda quince minutos y luego se descarta. Importe el fichero otra vez.
error-upload-none = No se subió ningún fichero. Elija uno primero.
error-upload-unsupported = Ese tipo de fichero el archivo no lo guarda. Se aceptan imágenes, PDF, texto sencillo, audio y vídeo; el tipo se lee de los propios bytes del fichero, así que renombrar un ejecutable no sirve. El SVG se rechaza sin más, porque un SVG puede llevar un script.
error-export-unreadable-title = No se pudo leer el archivo exportado
error-export-unreadable-detail = { $error }

## Página del árbol, continuación

tree-title-suffix = árbol
tree-back-to-focused = Volver a la vista alrededor de una persona
tree-show-all = Mostrar las { $n }
tree-width-notice = Esta vista mide { $width } píxeles de ancho: en una pantalla de 1500 píxeles son { $screens ->
        [one] una pantalla
       *[other] { $screens } pantallas
    } de desplazamiento horizontal.
tree-confidence-label = Certeza:
tree-band-certain = seguro
tree-band-high = alta
tree-band-medium = media
tree-band-low = hipotético
tree-counts = { $drawn } de { $total } personas · { $generations ->
        [one] una generación
       *[other] { $generations } generaciones
    }
tree-unplaced-count = { $n } sin sitio
tree-contradicts-title = Este árbol se contradice.
tree-contradicts-detail = Ninguna disposición de filas puede cumplirlo, así que el parentesco de abajo quedó fuera de la numeración de generaciones y alguna fila puede estar dibujada en el sitio equivocado. Corrija la de las dos fichas que esté mal.
tree-contradicts-pair = Registrados a la vez como pareja y como madre o padre e hijo:
tree-contradicts-more = { $n ->
        [one] Otra contradicción no aparece en la lista.
       *[other] Otras { $n } contradicciones no aparecen en la lista.
    }
tree-no-people = En este árbol todavía no hay nadie.
tree-no-people-cta = Importe un fichero familiar, o añada a la primera persona.
tree-nobody-selected = Para esa selección no hay a quién dibujar.
tree-nobody-selected-cta = Empiece por la vista por omisión.
tree-edge-union = Una unión registrada
tree-edge-parentage = Una filiación registrada

## Página de inicio

home-empty = Todavía no hay nada registrado. Importe un fichero familiar para traer un árbol ya existente, o añada a mano la primera persona.
home-count = { $total ->
        [one] Una ficha
       *[other] { $total } fichas
    }, en un solo fichero que es de la familia.
home-browse = Recorrer el árbol
home-convert = Importar un fichero familiar
home-unnamed-family = Este árbol familiar
home-in-this-tree = Lo que la familia ha registrado hasta ahora
home-showcase-title = Donde este árbol dice ya más que nombres y fechas
home-showcase-example = Ver un ejemplo →
home-nothing-title = Todavía no hay nada que mostrar.
home-nothing-detail = Importe un fichero familiar para traer un árbol ya existente, o empiece de cero y añada usted mismo a la primera persona.

## Tarjetas de muestra

showcase-links-title = { $n ->
        [one] Una relación fuera de la familia
       *[other] { $n } relaciones fuera de la familia
    }
showcase-links-detail = Padrinos, patronos, testigos y maestros, cada uno con sus propias fechas, su fuente y su grado de certeza.
showcase-occupations-title = { $n ->
        [one] Una ocupación con un principio y un final
       *[other] { $n } ocupaciones con un principio y un final
    }
showcase-occupations-detail = «Maestra, 1948-1978» conserva su duración y se dibuja como una barra a lo largo de los años, no como una sola línea con fecha.
showcase-uncertain-title = { $n ->
        [one] Una fecha dejada tan imprecisa como se dio
       *[other] { $n } fechas dejadas tan imprecisas como se dieron
    }
showcase-uncertain-detail = Hacia, antes, después y entre siguen siendo cuatro afirmaciones distintas. Una fecha que la fuente no supo fijar nunca se muestra como si la hubiera fijado.
showcase-preserved-title = { $n ->
        [one] Una fecha conservada en las palabras en que se escribió
       *[other] { $n } fechas conservadas en las palabras en que se escribieron
    }
showcase-preserved-detail = Una formulación que nadie supo leer como fecha queda exactamente como está escrita, en lugar de descartarse en silencio.
showcase-sources-title = { $n ->
        [one] Una fuente con su fiabilidad registrada
       *[other] { $n } fuentes con su fiabilidad registrada
    }
showcase-sources-detail = { $primary ->
        [one] Una fuente primaria.
       *[other] { $primary } primarias.
    } Cada hecho muestra en qué prueba se apoya y cuánta fuerza tiene esa prueba.
showcase-places-title = { $n ->
        [one] Un lugar cuyas fronteras se movieron
       *[other] { $n } lugares cuyas fronteras se movieron
    }
showcase-places-detail = Una ciudad puede pertenecer a estados distintos en épocas distintas, y la ficha dice cuál regía cuándo.

## Detalles de la ficha

record-also-recorded-as = registrado también como
record-borders-moved = Fronteras movidas:
record-display-name = nombre mostrado
record-read-as = leído como
record-note = Nota
record-living-yes = vive
record-deceased = fallecido/a
record-centre-tree-here = Centrar el árbol aquí
record-centre-tree-title = Mover el árbol para centrarlo en esta persona
record-open-full-page = Abrir la página entera ↗
record-open-full-title = Abrir la página independiente que se puede compartir
record-edit = Editar
panel-empty = Elija una ficha para ver aquí el documento completo de esa persona.
person-see-in-tree = Ver a esta persona en el árbol
person-visibility-inline = visibilidad:
person-age-at-death = murió a los { $n }
person-age-now = { $n } años
person-born-in = nacido en { $place }
person-died-in = murió en { $place }
person-children-count = { $n ->
        [one] un hijo
       *[other] { $n } hijos
    }
person-generations-below = { $n ->
        [one] una generación por debajo
       *[other] { $n } generaciones por debajo
    }
person-portrait-of = Fotografía de { $name }
person-no-portrait = Sin fotografía registrada

## Resultados de las operaciones

result-diagnostics = Avisos
result-diagnostics-note = Todos los avisos que devolvió la biblioteca, incluidas las advertencias que no detuvieron la operación. No se filtra ninguno.
result-no-diagnostics = La biblioteca no devolvió ningún aviso.
result-continue = Seguir
result-dashboard = Panel
person-sections-label = Secciones de esta página

## Vocabulary the structured editors offer

name-part-nasab = nasab (linaje)
name-part-laqab = laqab (epíteto)
name-part-kunya = kunya (teknónimo)
name-part-nisbah = nisbah (origen)
name-part-alias = alias
name-part-religious_name = nombre religioso
name-part-pen_name = seudónimo
name-type-pen_name = seudónimo
gender-U = Sin registrar

## Secciones de la ficha, detalles

record-notes-title = Que conste sobre esta ficha:
record-name = Nombre
record-type = Tipo
record-cause = Causa:
record-as = como
record-partner-not-recorded = Pareja sin registrar
record-union-from = Desde
record-union-at = en
record-union-until = hasta
record-occupation-from = desde
record-occupation-until = hasta
record-source-reliability = Fiabilidad
record-source-supports = Respalda
record-photographs = Fotografías
record-documents = Documentos
record-file = Fichero
record-status = Estado
record-size = Tamaño
record-absent-document = Nombrado por esta persona pero no guardado aquí.
record-no-file = sin fichero
record-attach-document = Adjuntar un documento
record-upload = Subir
record-upload-help = Hasta { $mb } MB por fichero. Los adjuntos se guardan junto al árbol y se vuelven a escribir en el archivo al exportar, así que una fotografía viaja con la familia a la que pertenece. La clase de fichero se lee de su propio contenido y no de su nombre: se aceptan imágenes, PDF, texto sencillo, audio y vídeo. El SVG se rechaza, porque un SVG puede llevar un script.
record-upload-help-short = Hasta { $mb } MB. El SVG se rechaza.
record-verbatim-note = Conservado tal como lo daba la ficha, porque ningún convertidor supo interpretarlo.
record-file-to-attach = Fichero que adjuntar
record-document-type = Tipo de documento
record-caption = Pie
record-caption-placeholder = Pie (opcional)
record-history-entry-meta = — { $at }
record-history-entry-version = , { $version }

## Clases de elemento

kind-person = persona
kind-family = familia
kind-event = hecho
kind-link = vínculo
kind-occupation = ocupación
kind-source = fuente
kind-place = lugar
kind-document = documento

kind-person-plural = { $n ->
        [one] persona
       *[other] personas
    }
kind-family-plural = { $n ->
        [one] familia
       *[other] familias
    }
kind-event-plural = { $n ->
        [one] hecho
       *[other] hechos
    }
kind-link-plural = { $n ->
        [one] vínculo
       *[other] vínculos
    }
kind-occupation-plural = { $n ->
        [one] ocupación
       *[other] ocupaciones
    }
kind-source-plural = { $n ->
        [one] fuente
       *[other] fuentes
    }
kind-place-plural = { $n ->
        [one] lugar
       *[other] lugares
    }
kind-document-plural = { $n ->
        [one] documento
       *[other] documentos
    }

## Listados

list-matching = { $total ->
        [one] Una coincidencia
       *[other] { $total } coincidencias
    }, { $per_page } por página.
list-filter-placeholder = Filtrar por nombre o identificador
list-filter = Filtrar
list-clear = Limpiar
list-summary = Descripción
list-id = Identificador
list-actions = Acciones
list-nothing = Aquí no hay nada.
list-nothing-matching = Aquí no hay nada que coincida con «{ $q }».
list-delete-confirm = ¿Borrar este elemento ({ $kind })? Elija qué pasa con los elementos que lo citan:
list-policy-reject = Rechazar
list-policy-reject-detail = — rechazar si algo sigue citándolo. No se pierde nada.
list-policy-cascade = En cascada
list-policy-cascade-detail = — borrarlo y quitar de verdad toda cita a él.
list-policy-orphan = Dejar huérfanos
list-policy-orphan-detail = — borrarlo pero conservar las fichas que lo citan, con la cita vaciada.

## Grado de detalle

completeness-dates-title = Las fechas según la forma que de verdad tienen
completeness-no-dates = Todavía no hay fechas registradas.
completeness-dates-note = Una fecha que alguien supo fijar al día y otra que alguien solo supo situar en una década son dos afirmaciones distintas, y las dos se conservan tal como se dieron. El texto que no se pudo leer como fecha se conserva palabra por palabra en lugar de descartarse.
completeness-shape-exact = exacta
completeness-shape-exact-note = un día de calendario entero
completeness-shape-approximate = aproximada
completeness-shape-approximate-note = hacia, o solo un año o una década
completeness-shape-ranged = acotada
completeness-shape-ranged-note = antes, después o entre
completeness-shape-preserved = literal
completeness-shape-preserved-note = texto no interpretable, conservado tal cual
completeness-shape-unknown = desconocida
completeness-shape-unknown-note = registrada como no sabida

## Página de importación

convert-page-title = Importar un fichero familiar
convert-lede = Traiga un árbol ya existente desde un fichero GEDCOM, la exportación que produce la mayoría de los programas de genealogía. Aquí no se guarda nada, y el árbol que este sitio ya muestra se queda exactamente como estaba.
convert-file-label = Fichero familiar (.ged)
convert-file-hint = Hasta { $mb } MB. Un árbol de 767 personas pesa unos 320 KB.
convert-confidence-label = Qué certeza tienen estos hechos, para empezar
convert-confidence-hint = El fichero que se importa no dice cuánta seguridad tenía nadie, así que cada hecho necesita un punto de partida. Póngalo bajo para un árbol reunido deprisa, más alto para uno trabajado sobre documentos. La lectura honrada de este número es «importado, y desde entonces nadie lo ha comprobado»: podrá subir o bajar cada hecho después, de uno en uno.
convert-lang-label = Idioma de los nombres de lugar
convert-lang-hint = Una etiqueta como en, fr o es.

## Informe de importación

convert-failed = La importación no salió adelante
convert-try-another = Probar con otro fichero
convert-converted = Importado { $filename }
convert-result-lede = { $total ->
        [one] Una ficha
       *[other] { $total } fichas
    }, { $size } KB. Todo entró con una certeza de { $confidence }, con los nombres de lugar leídos como { $lang }. El árbol que muestra este sitio no se tocó.
convert-produced = Qué pasó al otro lado
convert-skipped-title = { $n ->
        [one] Una entrada que no se pudo leer
       *[other] { $n } entradas que no se pudieron leer
    }
convert-skipped-note = Estas entradas no contenían nada que se pudiera traer.
convert-other-diagnostics = { $n ->
        [one] Otra cosa que conviene saber
       *[other] Otras { $n } cosas que conviene saber
    }
convert-clean = No quedó nada atrás: todas las entradas del fichero pasaron.
convert-download-title = Descarga
convert-download-named = Descargar { $name }
convert-download-note = Se guarda aquí quince minutos y luego se descarta, así que descárguelo ahora.
convert-another = Importar otro fichero
admin-history-on = el
admin-history-meta = — { $kind }, { $at }
admin-validation-counts = { $errors ->
        [one] Un error
       *[other] { $errors } errores
    }, { $warnings ->
        [one] una advertencia
       *[other] { $warnings } advertencias
    }, { $infos ->
        [one] una nota
       *[other] { $infos } notas
    }.
admin-warnings-never-block = Las advertencias nunca detienen nada: son información, no una barrera.
admin-validator-clean = La comprobación no informó de nada.
record-occupations-help-undated = Una ocupación se registra con principio y final, de modo que varias se comparen sobre una misma línea de tiempo. Este archivo tiene los nombres de los oficios pero no sus fechas — algo normal después de una importación, porque la mayoría de los ficheros familiares no tienen dónde guardarlas —, así que todavía no hay escala que dibujar.
record-occupations-help-axis = Una ocupación es un estado con duración, no un hecho en una sola fecha. Todos los tramos comparten un eje, { $from }–{ $to }.
admin-value-not-set = sin establecer
admin-validation-report = Informe de comprobación
admin-dedup-complete = Unión de duplicados terminada
admin-dedup-refused = Unión de duplicados rechazada
record-birth-order = orden de nacimiento
record-start-not-recorded = principio sin registrar
record-end-not-recorded = final sin registrar
record-document-no-file = El documento está registrado aquí, pero el fichero en sí no está
panel-selected-person = Persona elegida

## Franjas de generación

tree-band-generation = Generación { $g }
tree-band-people = { $n ->
        [one] una persona
       *[other] { $n } personas
    }
tree-band-unplaced = Sin sitio
tree-band-unplaced-note = { $n ->
        [one] una persona sin familia — se muestra en lugar de omitirla
       *[other] { $n } personas sin familia — se muestran en lugar de omitirlas
    }

## Vocabulario controlado

gender-M = Masculino
gender-F = Femenino
gender-NB = No binario
gender-unrecorded = Sin registrar

name-part-given_name = nombre de pila
name-part-family_name = apellido
name-part-patronymic = patronímico
name-part-matronymic = matronímico
name-part-middle_name = segundo nombre
name-part-nickname = apodo
name-part-prefix = prefijo
name-part-suffix = sufijo
name-part-particle = partícula
name-part-part = elemento

name-type-primary = principal
name-type-other = otro
name-type-alias = de uso
name-type-birth = de nacimiento
name-type-married = de casada
name-type-religious = religioso
name-type-transliteration = transliteración
name-type-nickname = apodo

## Anotaciones sobre la ficha

note-links = { $n ->
        [one] una relación fuera de la familia, con fechas y fuentes propias
       *[other] { $n } relaciones fuera de la familia, con fechas y fuentes propias
    }
note-occupations = { $n ->
        [one] un oficio registrado con principio y final
       *[other] { $n } oficios registrados con principio y final
    }
note-birth-imprecise = una fecha de nacimiento que la fuente no supo fijar, mostrada tal como está registrada
note-death-imprecise = una fecha de defunción que la fuente no supo fijar, mostrada tal como está registrada
note-names = { $n ->
        [one] un nombre registrado
       *[other] { $n } nombres registrados
    }
note-transliteration = un nombre en su propia escritura junto a su transliteración latina
note-witnessed = { $n ->
        [one] un hecho del que fue testigo y no protagonista
       *[other] { $n } hechos de los que fue testigo y no protagonista
    }

visibility-public = pública
visibility-members = familiares
visibility-contributors = colaboradores
visibility-private = reservada

## Descripciones de fila en los listados de administración

family-label-couple = { $children ->
        [0] { $a } y { $b }
        [one] { $a } y { $b } — un hijo
       *[other] { $a } y { $b } — { $children } hijos
    }
family-label-half = { $children ->
        [0] { $a } y { $unknown }
        [one] { $a } y { $unknown } — un hijo
       *[other] { $a } y { $unknown } — { $children } hijos
    }
family-label-children = { $others ->
        [0] { $first } — padres sin registrar
        [one] { $first } y un hermano — padres sin registrar
       *[other] { $first } y { $others } hermanos — padres sin registrar
    }
family-label-empty = Familia sin nadie registrado

event-label = { $category } — { $who }, { $date }
event-label-nobody = { $category } — { $date }
event-two-people = { $a } y { $b }
event-more-people = { $a } y { $b } y { $others ->
        [one] otro más
       *[other] otros { $others }
    }

link-label = { $label }: { $from } → { $to }
occupation-label = { $who } — { $title }
source-label = { $title } — { $reliability }
source-label-plain = { $title }
document-label = { $filename } — { $type }
document-label-untitled = { $type } sin título
list-unnamed = { $kind } sin nombre

## Vocabularios de la especificación en los listados

event-category-birth = Nacimiento
event-category-death = Defunción
event-category-marriage = Matrimonio
event-category-divorce = Divorcio
event-category-baptism = Bautismo
event-category-burial = Entierro
event-category-immigration = Inmigración
event-category-emigration = Emigración
event-category-census = Censo
event-category-residence = Residencia
event-category-military = Servicio militar
event-category-education = Estudios
event-category-other = Hecho

reliability-primary = fuente primaria
reliability-secondary = fuente secundaria
reliability-tertiary = fuente terciaria
reliability-recollection = testimonio oral
reliability-derivative = obra derivada
reliability-authored = obra de autor
reliability-oral = tradición oral
reliability-unknown = fiabilidad desconocida

document-type-photo = fotografía
document-type-certificate = acta
document-type-letter = carta
document-type-record = registro de archivo
document-type-newspaper = recorte de prensa
document-type-other = documento

## Dónde esta ficha podría decir más

completeness-title = Dónde este árbol podría decir más
completeness-intro = Qué está registrado y qué sigue en blanco.
completeness-import-title = Qué trajo la importación
completeness-import-intro = Contado sobre el fichero que acaba de subir. Una fila en blanco es algo que el fichero de origen no registraba, no algo que la importación haya perdido.

completeness-headline-full = Cada clase de detalle de abajo está registrada en alguna parte de este árbol.
completeness-headline-empty = { $total ->
        [one] La única clase de detalle de abajo no está registrada todavía en ninguna parte.
       *[other] Ninguna de las { $total } clases de detalle de abajo está registrada todavía.
    } Cada una es un sitio donde la ficha podría decir más.
completeness-headline-partial = { $carried ->
        [one] Una clase de detalle de abajo está registrada
       *[other] { $carried } clases de detalle de abajo están registradas
    }; { $empty ->
        [one] una sigue en blanco
       *[other] { $empty } siguen en blanco
    }.

completeness-metric-confidence = Qué certeza tiene cada hecho
completeness-metric-confidence-none = Ninguno de los { $slots } hechos de aquí dice qué certeza tiene. Una fecha leída en un acta y otra adivinada se parecen, hasta que dejan de parecerse.
completeness-metric-confidence-uniform = { $with } de { $slots } hechos llevan una puntuación, y todas son el mismo número ({ $modal }). Eso es lo que deja detrás una importación en bloque: un valor de relleno al que nadie ha vuelto. Ninguno se ha juzgado todavía uno por uno.
completeness-metric-confidence-some = { $with } de { $slots } hechos llevan una puntuación. { $modal_count } comparten un mismo valor ({ $modal }); { $assessed } se apartan de él y por tanto se han mirado de uno en uno.
completeness-metric-confidence-many = { $with } de { $slots } hechos llevan una puntuación, y { $assessed } de ellos se apartan del valor más frecuente ({ $modal }), a lo largo de { $distinct } niveles distintos. Este árbol registra una incertidumbre real y variada.

completeness-metric-parentage = Qué certeza tiene cada vínculo padre-hijo
completeness-metric-parentage-none = Ninguna filiación de aquí dice qué certeza tiene. Las adopciones, las líneas discutidas y las reconstrucciones a partir de una sola mención son justo los sitios donde una familia necesita registrar la duda — y el árbol dibuja un vínculo menos seguro con una línea más pálida.
completeness-metric-parentage-some = { $n ->
        [one] Una filiación lleva su propia puntuación
       *[other] { $n } filiaciones llevan su propia puntuación
    }, así que una línea hipotética es visiblemente más débil que una documentada.

completeness-metric-links = Relaciones más allá de la sangre y el matrimonio
completeness-metric-links-none = Padrinos, patronos, testigos, maestros, tutores. Todavía no hay ninguna registrada. Cada una puede llevar sus propias fechas, su fuente y su grado de certeza.
completeness-metric-links-some = { $n ->
        [one] Una registrada, con sus propias fechas, su fuente y su grado de certeza.
       *[other] { $n } registradas, cada una con sus propias fechas, su fuente y su grado de certeza.
    }

completeness-metric-occupations = Oficios registrados con principio y final
completeness-metric-occupations-none = No hay ocupaciones registradas. Un oficio ejercido treinta años dice más de una vida que una sola entrada con fecha.
completeness-metric-occupations-undated = { $total ->
        [one] Hay una ocupación registrada, sin fechas
       *[other] Hay { $total } ocupaciones registradas, sin fechas
    }. Añada un principio y un final y podrán compararse una junto a otra sobre una misma línea de tiempo.
completeness-metric-occupations-some = { $span } de { $total } tienen principio o final, así que pueden compararse una junto a otra sobre una misma línea de tiempo.

completeness-metric-sources = Fuentes con su fiabilidad valorada
completeness-metric-sources-none = No hay fuentes registradas. Decir de dónde salió un hecho es lo que permite a un pariente comprobarlo más tarde — o no estar de acuerdo y decir por qué.
completeness-metric-sources-some = { $graded } de { $total } dicen qué fuerza tienen, así que una afirmación que se apoya en una partida de nacimiento no es visiblemente lo mismo que otra que se apoya en un recuerdo.

completeness-what-is-recorded = Qué puede decir la ficha
completeness-in-this-tree = En este árbol
completeness-not-yet = todavía sin registrar

## Papeles de un participante en un hecho

role-spouse = cónyuge
role-spouse_1 = primer cónyuge
role-spouse_2 = segundo cónyuge
role-subject = persona del registro
role-participant = participante
role-witness = testigo
role-officiant = oficiante
role-informant = declarante
role-godparent = padrino o madrina

phys-no-source = sin fuente
phys-col-date = Cuándo
phys-col-source = Fuente
phys-col-confidence = Confianza
phys-col-note = Nota
phys-field-height-cm = Estatura
phys-field-weight-kg = Peso
phys-field-eye-colour = Color de ojos
phys-field-hair-colour = Color de pelo
phys-field-build = Complexión
phys-field-handedness = Lateralidad
phys-field-features = Señas particulares
phys-field-military = Servicio militar
phys-field-languages = Idiomas hablados
phys-field-blood-group = Grupo sanguíneo
phys-field-conditions = Dolencias conocidas
phys-field-operations = Operaciones y lesiones
phys-field-cause-of-death = Causa de la muerte
phys-field-religion = Religión o afiliación
phys-field-health-notes = Notas
admin-export-health-note = La exportación simple omite todas las categorías sensibles —salud y creencias, datos biométricos, datos genómicos y antecedentes penales— y también el perfil de comportamiento de cualquier persona viva, de modo que un archivo enviado a un pariente no contiene ninguna. Marque lo que debe llevar un archivo concreto; el propio archivo registra qué categorías se omitieron.
avatar-picker-title = Elegir una imagen
avatar-choose-link = Elegir imagen
avatar-choose = Qué imagen representa a esta persona
avatar-mode-auto = Que elija el programa
avatar-mode-auto-note = El primer retrato o, en su defecto, la primera imagen ligada a esta ficha.
avatar-mode-none = Mostrar las iniciales
avatar-mode-none-note = Para una ficha cuyas imágenes son documentos y no caras.
avatar-focal-hint = Pulse una imagen para elegirla y vuelva a pulsar en la parte que debe quedar en el encuadre: un avatar es cuadrado y la mayoría de los escaneos no.
avatar-no-images = Todavía no hay imágenes ligadas a esta ficha.
avatar-upload-title = Subir una imagen y usarla
avatar-upload-button = Subir y usar como imagen
avatar-not-available-title = Esa imagen no está disponible
avatar-not-available-detail = El archivo elegido no está ligado a esta persona, o no puede leerlo.

record-history-withheld = no se te muestra

## A death nobody wrote down
#
# GEDCOM cannot say "died, date unknown", so a converter marks those
# people living. These three say that the page worked it out rather than
# read it, which is a distinction this product does not blur.

record-life-status = Estado
record-presumed-deceased = fallecimiento presunto
record-presumed-short = presunto
record-presumed-why = No hay defunción registrada y el nacimiento fue hace más de { $years } años, así que este registro no puede ser correcto. El archivo no se modifica: esto es lo que deduce la página, no lo que dice la fuente.

## The identity editor

identity-editor-title = Nombres e identidad
identity-primary-name = El nombre que se muestra en todas partes
identity-primary-help = Lo que usan la ficha del árbol, el encabezado y todas las listas. Los otros nombres de abajo son los que una fuente usó en otro momento.
identity-display = Nombre
identity-display-latin = En alfabeto latino
identity-culture = Idioma
identity-direction = Dirección
identity-direction-ltr = de izquierda a derecha
identity-direction-rtl = de derecha a izquierda
identity-direction-auto = según el texto
identity-components = Partes del nombre
identity-components-help = Qué parte es el nombre de pila y cuál el apellido, en el orden en que se escriben. Un registro sin partes se muestra igual: las partes son lo que una búsqueda puede encontrar.
identity-part = Parte
identity-value = Texto
identity-other-names = Otros nombres
identity-other-help = Un apellido de casada, un nombre religioso, un nombre que usó un registro posterior. Cada uno lleva cuándo se usó y qué fuente lo dice.
identity-name-type = Tipo de nombre
identity-valid-from = En uso desde
identity-valid-until = En uso hasta
identity-about = Sobre la persona
identity-living-help = Esta es la marca que puso la fuente. La página presume aparte un fallecimiento cuando el nacimiento es demasiado antiguo, y esa presunción nunca cambia esta casilla ni el archivo.
identity-error-no-display = Un registro necesita un nombre con el que mostrarse. No se guardó nada.
editor-blank-to-remove = Borra el nombre para eliminar esta entrada.
# A BCP 47 tag, shown as an example in the language field.
identity-culture-placeholder = es
identity-edit-link = Editar nombres e identidad

## Union types, statuses and date precision, said out loud

union-type-marriage = matrimonio
union-type-civil_union = unión civil
union-type-cohabitation = convivencia
union-type-religious_only = unión religiosa
union-type-polygamous = polígamo
union-type-unknown = sin registrar
union-status-active = en curso
union-status-ended_by_death = terminada por fallecimiento
union-status-ended_by_divorce = terminada por divorcio
union-status-ended_by_separation = terminada por separación
union-status-annulled = anulada
union-status-unknown = sin registrar
union-status-ended = terminada
union-status-ended-by = terminada por { $reason }
union-reason-death_of_spouse = el fallecimiento de un cónyuge
precision-exact = al día
precision-year = al año
precision-month = al mes
precision-decade = a la década
precision-century = al siglo
precision-unknown = desconocida
record-precision = Precisión
record-approximate = Aproximado
record-place = Lugar

## The relationships editor

family-editor-title = Familia y relaciones
family-unions = Uniones
family-no-unions = No hay ninguna unión registrada para esta persona.
family-union-legend = Unión { $n }
family-writes-family = Guardar modifica el registro de familia #{ $id }, que ambas personas comparten. La página de la otra persona cambia también.
family-partners = Miembros de la pareja
family-partner = Pareja
family-role = Papel
family-children = Hijos
family-children-help = El orden de nacimiento es lo que afirma el propio registro. Dejado en blanco no afirma nada: un número tomado de la posición de la fila sería un hecho que nadie escribió.
family-child = Hijo
family-birth-order = Orden de nacimiento
family-the-union = La unión en sí
family-type = Tipo de unión
family-status = Estado
family-started = Inicio
family-ended = Fin
family-leave = Sacar a esta persona de esta unión
family-open-entity = Abrir el registro de familia
family-new-union = Una unión nueva
family-new-union-help = Esto crea un registro de familia nuevo con esta persona. La pareja es opcional: un progenitor que el registro nombra sin pareja alguna es una unión de uno.
family-create-union = Crear la unión
family-parents = Padres
family-no-parents = Esta persona no está registrada como hijo de ninguna familia.
family-child-of = Hijo de esta familia
family-detach-child = Sacar a esta persona de esta familia
family-attach-parents = Vincular a unos padres
family-attach-help = Elige la familia de la que esta persona es hijo. Se la añade a ese registro de familia, así que aparece también en las páginas de los padres.
family-the-family = La familia
family-attach = Vincular
family-error-last-partner = Una unión necesita al menos una persona. Elimina en su lugar el registro de familia, que pregunta qué hacer con todo lo que lo referencia.
family-error-no-family = No se eligió ninguna familia. No se guardó nada.
family-error-already-child = Esta persona ya es hijo de esa familia.
pick-error-empty = No se nombró a ninguna persona. No se guardó nada.
pick-error-not-found = No hay ninguna persona con ese nombre en este archivo. No se guardó nada.
pick-error-ambiguous = Más de una persona responde a eso. Elige una de la lista para que el registro diga cuál. No se guardó nada.

## Links and occupations

links-editor-title = Vínculos
links-editor-help = Las relaciones que no son de familia: un padrino, un patrón, un testigo, un regimiento. Cada una es un registro propio que nombra a dos personas, así que editarla aquí cambia también el otro registro.
links-none = No hay ningún vínculo registrado para esta persona.
links-new = Un vínculo nuevo
links-create = Crear el vínculo
links-remove = Eliminar este vínculo
links-other-end = El otro extremo
links-label = Qué es
links-label-reverse = En sentido inverso
links-category = Categoría
links-bidirectional = Se lee igual en ambos sentidos
links-from = Desde
links-until = Hasta
links-reversed = Este vínculo se creó desde el otro registro. Editarlo aquí cambia la misma entidad.
link-error-no-label = Un vínculo tiene que decir qué es. No se guardó nada.
occupations-editor-title = Ocupaciones
occupations-editor-help = Una ocupación es un periodo con un principio y un final, no un puesto. Cada una lleva sus propias fechas y su propia fuente.
occupations-none = No hay ninguna ocupación registrada para esta persona.
occupations-new = Una ocupación nueva
occupations-create = Crear la ocupación
occupations-remove = Eliminar esta ocupación
occupations-title = A qué se dedicaba
occupations-employer = Para quién
occupations-employer-place = Dónde estaban
occupations-from = Desde
occupations-until = Hasta
occupation-error-no-title = Una ocupación tiene que decir qué hacía alguien. No se guardó nada.
link-category-spiritual = espiritual
link-category-professional = profesional
link-category-social = social
link-category-legal = legal
link-category-medical = médica
link-category-educational = educativa
link-category-conflict = conflicto
link-category-other = otra
links-edit-link = Editar vínculos
occupations-edit-link = Editar ocupaciones
family-edit-link = Editar familia y relaciones

## Events and documents

events-editor-title = Sucesos
events-editor-help = Un suceso nombra a varias personas a la vez — una boda, un bautizo, un censo — así que cada uno es un registro propio y aparece en cada página que nombra.
events-none = Ningún suceso nombra a esta persona.
events-new = Un suceso nuevo
events-new-help = Esta persona se añade como sujeto si no nombras a nadie más. Un suceso sin nadie es sólo una fecha.
events-create = Crear el suceso
events-remove = Eliminar este suceso
events-category = Qué ocurrió
events-subcategory = Más concretamente
events-description = Descripción
events-participants = Quién estuvo
events-participants-help = Guardar modifica el registro del suceso, que también muestra cada otra persona nombrada.
events-who = Quién
event-error-no-category = Un suceso tiene que decir qué ocurrió. No se guardó nada.
documents-editor-title = Documentos
documents-editor-help = A qué archivos apunta este registro y qué es cada uno para él. Vaciar una fila desvincula el archivo: el documento y sus bytes siguen en el archivo general.
documents-attached = Adjuntos a este registro
documents-upload = Subir un archivo
documents-upload-help = Hasta { $mb } MB. El archivo se guarda en el archivo general y se adjunta a este registro.
documents-caption = Pie
documents-edit-link = Adjuntar y desvincular documentos
events-edit-link = Editar sucesos

## Presentation styles: density, never colour

prefs-style = Densidad
prefs-style-help = Cuánto espacio ocupa la página. Aparte del tema, que sólo trata del color, así que cualquier combinación es posible.
style-comfortable = Cómoda
style-comfortable-note = por defecto, con espacio para leer
style-compact = Compacta
style-compact-note = más registro por pantalla, para revisar varios
style-paper = Papel
style-paper-note = una tipografía con serifas y filetes en vez de tarjetas, para leer con calma o imprimir

## Sensitive classes

admin-export-choose = Incluir en esta exportación
scope-health = Salud y creencias
scope-biometrics = Datos biométricos
scope-genomics = Datos genómicos
scope-legal = Antecedentes penales
scope-behaviour = Perfiles de comportamiento de personas vivas
admin-export-with-chosen = Exportar con lo marcado

## Profile

pg-identity = Identidad y estado civil
pg-identity-intro = Quién era la persona según los documentos y qué anotaron los registros civiles.
pg-morphology = Morfología
pg-morphology-intro = El cuerpo tal como se midió y se describió.
pg-biometrics = Biometría
pg-biometrics-intro = La voz, las manos y los sentidos, y los patrones que permiten identificar a una persona.
pg-health = Salud
pg-health-intro = Enfermedades, tratamientos, mediciones y resultados.
pg-genomics = Genómica
pg-genomics-intro = Pruebas de ADN, haplogrupos, variantes y otros resultados moleculares.
pg-death = Defunción
pg-death-intro = Cómo, cuándo y dónde terminó una vida, y qué se hizo con el cuerpo.
pg-residence = Residencia y nacionalidad
pg-residence-intro = Dónde vivió la persona, qué Estados la consideraron nacional y qué lenguas hablaba.
pg-education = Educación y trabajo
pg-education-intro = Estudios, títulos, ingresos y bienes.
pg-military = Servicio militar y distinciones
pg-military-intro = Servicio, grados, unidades y distinciones.
pg-legal = Causas penales
pg-legal-intro = Procesos penales y su resultado.
pg-belief = Creencias y afiliaciones
pg-belief-intro = Religión, ritos, convicciones y afiliaciones.
pg-personality = Personalidad y comportamiento
pg-personality-intro = Temperamento, hábitos y aficiones, tal como los describen las fuentes.
pg-relationships = Relaciones
pg-relationships-intro = Padres, cónyuges, hijos y las demás personas de una vida.
pg-digital-legacy = Legado digital
pg-digital-legacy-intro = Escaneos, modelos, grabaciones y archivos que representan a una persona.
pa-identity-titles = Títulos
pa-identity-sex-at-birth = Sexo al nacer
pa-identity-gender-identity = Identidad de género
pa-birth-time = Hora de nacimiento
pa-birth-coordinates = Lugar de nacimiento en coordenadas
pa-civil-status-birth-certificate-number = Número del acta de nacimiento
pa-civil-status-register-entries = Inscripciones registrales
pa-civil-status-marginal-annotations = Notas marginales
pa-morphology-height = Estatura
pa-morphology-weight = Peso
pa-morphology-bmi = Índice de masa corporal
pa-morphology-body-composition = Composición corporal
pa-morphology-build = Complexión
pa-morphology-eye-colour = Color de ojos
pa-morphology-eye-shape = Forma de los ojos
pa-morphology-eye-spacing = Separación de los ojos
pa-morphology-hair-colour = Color natural del pelo
pa-morphology-hair-texture = Tipo de pelo
pa-morphology-hairline = Nacimiento del pelo
pa-morphology-facial-hair = Vello facial
pa-morphology-body-hair = Vello corporal
pa-morphology-skin-tone = Fototipo (Fitzpatrick)
pa-morphology-skin-undertone = Subtono de la piel
pa-morphology-freckles = Pecas
pa-morphology-pigmentation = Manchas de pigmentación
pa-morphology-scars = Cicatrices
pa-morphology-tattoos = Tatuajes
pa-morphology-moles = Lunares
pa-morphology-facial-asymmetries = Asimetrías faciales
pa-morphology-face-shape = Forma de la cara
pa-morphology-nose-shape = Forma de la nariz
pa-morphology-ear-shape = Forma de las orejas
pa-morphology-lip-shape = Forma de los labios
pa-morphology-dentition = Dentición
pa-morphology-malocclusion = Maloclusión (clase de Angle)
pa-morphology-posture = Postura
pa-morphology-gait = Forma de andar
pa-morphology-distinguishing-features = Señas particulares
pa-biometrics-fingerprints = Huellas dactilares
pa-biometrics-retinal-print = Patrón retiniano
pa-biometrics-voice-signature = Huella vocal
pa-biometrics-voice-frequency = Frecuencia fundamental de la voz
pa-biometrics-vocal-timbre = Timbre de voz
pa-biometrics-spoken-accent = Acento
pa-biometrics-speech-rate = Velocidad del habla
pa-biometrics-verbal-tics = Muletillas
pa-biometrics-frequent-vocabulary = Vocabulario frecuente
pa-biometrics-speech-register = Registro del habla
pa-biometrics-motor-tics = Tics motores
pa-biometrics-handedness = Lateralidad manual
pa-biometrics-hearing = Audición
pa-biometrics-visual-acuity = Agudeza visual
pa-biometrics-optical-correction = Corrección óptica
pa-health-blood-group = Grupo sanguíneo (AB0)
pa-health-rhesus = Factor Rh (RhD)
pa-health-blood-pressure = Tensión arterial
pa-health-resting-heart-rate = Frecuencia cardiaca en reposo
pa-health-respiratory-capacity = Función respiratoria
pa-health-conditions = Enfermedades
pa-health-surgeries = Antecedentes quirúrgicos
pa-health-injuries = Lesiones
pa-health-deformities = Deformidades
pa-health-amputations = Amputaciones
pa-health-prostheses = Prótesis
pa-health-implants = Implantes
pa-health-devices = Dispositivos implantados
pa-health-medications = Medicamentos
pa-health-allergies = Alergias
pa-health-vaccinations = Vacunas
pa-health-serology = Serología
pa-health-lab-results = Análisis de laboratorio
pa-health-deficiencies = Carencias
pa-health-sleep-disorders = Trastornos del sueño
pa-health-mental-health-assessments = Evaluaciones de salud mental
pa-genomics-autosomal-mapping = Prueba de ADN autosómico
pa-genomics-y-haplogroup = Haplogrupo del cromosoma Y
pa-genomics-mt-haplogroup = Haplogrupo mitocondrial
pa-genomics-whole-genome-sequencing = Secuenciación del genoma completo
pa-genomics-risk-variants = Variantes de riesgo
pa-genomics-hereditary-conditions = Enfermedades hereditarias
pa-genomics-predispositions = Predisposiciones
pa-genomics-epigenetic-markers = Marcadores epigenéticos
pa-genomics-epigenetic-age = Edad epigenética
pa-genomics-gut-microbiome = Microbioma intestinal
pa-genomics-skin-microbiome = Microbioma cutáneo
pa-genomics-toxicological-sensitivities = Sensibilidad a fármacos y tóxicos
pa-death-time = Hora de la defunción
pa-death-coordinates = Lugar de la defunción en coordenadas
pa-death-causes = Causas de la muerte
pa-death-contributing-factors = Factores contribuyentes
pa-death-autopsy = Autopsia
pa-death-disposition = Destino del cuerpo
pa-death-grave = Sepultura
pa-residence-addresses = Domicilios
pa-residence-nationality-of-origin = Nacionalidad de origen
pa-residence-acquired-nationalities = Nacionalidades adquiridas
pa-residence-mother-tongue = Lengua materna
pa-residence-spoken-languages = Lenguas habladas
pa-education-level = Nivel de estudios
pa-education-diplomas = Títulos y diplomas
pa-education-institutions = Escuelas e instituciones
pa-education-income = Ingresos
pa-education-real-estate = Bienes inmuebles
pa-military-distinctions = Distinciones
pa-military-citations = Menciones
pa-military-ranks = Grados
pa-military-units = Unidades
pa-military-service-numbers = Números de filiación
pa-legal-criminal-record = Antecedentes penales
pa-belief-religions = Religión
pa-belief-sacraments = Sacramentos y ritos
pa-belief-beliefs = Convicciones
pa-belief-political-leanings = Tendencia política
pa-belief-memberships = Afiliaciones
pa-personality-big-five = Puntuaciones Big Five
pa-personality-mbti = Tipo MBTI
pa-personality-introversion-extraversion = Introversión y extraversión
pa-personality-stress-tolerance = Tolerancia al estrés
pa-personality-decision-style = Estilo de decisión
pa-personality-interests = Intereses
pa-personality-hobbies = Aficiones
pa-personality-sports = Deportes
pa-personality-dietary-habits = Alimentación
pa-personality-dependencies = Adicciones
pa-digital-legacy-body-models = Modelos del cuerpo
pa-digital-legacy-skin-textures = Texturas de la piel
pa-digital-legacy-rigs = Esqueletos de animación
pa-digital-legacy-voice-corpora = Grabaciones para síntesis de voz
pa-digital-legacy-text-corpora = Escritos para un modelo de lenguaje
pa-digital-legacy-digital-traces = Huellas digitales en línea
pa-digital-legacy-carbon-footprint = Huella de carbono
pa-digital-legacy-behaviour-models = Modelos de comportamiento
pf-identity-titles-text = Título tal como figura
pf-identity-titles-kind = Tipo de título
pf-civil-status-marginal-annotations-text = Nota
pf-morphology-pigmentation-kind = Tipo de mancha
pf-biometrics-spoken-accent-description = Cómo se describe
pf-biometrics-optical-correction-kind = Corrección
pf-health-amputations-level = Nivel de la amputación
pf-health-prostheses-kind = Prótesis
pf-health-implants-kind = Implante
pf-health-devices-kind = Dispositivo
pf-health-allergies-type = Tipo de alergia
pf-health-vaccinations-status = Estado de vacunación
pf-health-sleep-disorders-category = Categoría del trastorno
pf-death-autopsy-kind = Autopsia
pf-education-institutions-name = Nombre de la institución
pf-military-distinctions-name = Nombre de la distinción
pf-military-distinctions-kind = Tipo de distinción
pf-military-citations-text = Texto de la mención
pf-military-ranks-category = Categoría del grado
pf-belief-political-leanings-position = Posición en el eje izquierda–derecha
pf-belief-memberships-kind = Tipo de organización
pf-digital-legacy-carbon-footprint-method = Método de estimación
pf-age-years = Edad en años
pf-agreeableness = Amabilidad
pf-allergen = Alérgeno
pf-amount = Importe
pf-analyte = Analito
pf-artefact-type = Tipo de artefacto
pf-autoimmune = Autoinmune
pf-body-region = Zona del cuerpo
pf-bone-percent = Hueso
pf-carrier-status = Estado de portador
pf-cause = Causa
pf-chronic = Crónica
pf-clock = Reloj
pf-condition = Enfermedad
pf-conferred-by = Concedida por
pf-congenital = Congénita
pf-conscientiousness = Responsabilidad
pf-consent = Consentimiento
pf-coordinates = Coordenadas
pf-corrected = Con corrección
pf-country = País
pf-court = Tribunal
pf-coverage = Cobertura
pf-currency = Moneda
pf-decimal = Agudeza (decimal)
pf-denomination = Confesión
pf-derived-from-id = Derivado de
pf-description = Descripción
pf-details = Detalles
pf-diagnosis = Diagnóstico
pf-diameter-mm = Diámetro
pf-diastolic = Diastólica
pf-diet = Dieta
pf-document-id = Documento
pf-dose = Dosis
pf-ear = Oído
pf-entry-number = Número de inscripción
pf-extraversion = Extraversión
pf-eye = Ojo
pf-fat-percent = Grasa
pf-fev1-fvc-ratio = Cociente FEV1/FVC
pf-fev1-litres = FEV1
pf-file-format = Formato de archivo
pf-findings = Hallazgos
pf-flag = Indicador
pf-format = Formato
pf-fracture = Fractura
pf-fvc-litres = FVC
pf-gene = Gen
pf-generator = Hecho con
pf-grade = Grado
pf-iccs-section = Sección del delito (ICCS)
pf-icd10-chapter = Capítulo de la CIE-10
pf-indication = Indicación
pf-inheritance = Herencia
pf-inscription = Inscripción
pf-institution = Institución
pf-instrument = Instrumento
pf-isced-level = Nivel CINE
pf-jurisdiction = Jurisdicción
pf-language = Lengua
pf-lat = Latitud
pf-level = Nivel
pf-lines = Dirección
pf-location = Ubicación
pf-lon = Longitud
pf-major = Haplogrupo principal
pf-marker = Marcador
pf-metaboliser-status = Fenotipo metabolizador
pf-method = Método
pf-mode = Forma de adquisición
pf-muscle-percent = Músculo
pf-neuroticism = Neuroticismo
pf-number = Número
pf-nutrient = Nutriente
pf-offence = Delito
pf-office = Oficina
pf-openness = Apertura
pf-organisation = Organización
pf-outcome = Resultado del proceso
pf-pace = Ritmo de envejecimiento
pf-page = Página
pf-panel = Perfil analítico
pf-party = Partido
pf-pathogen = Patógeno
pf-pattern = Patrón de consumo
pf-percentile = Percentil
pf-period = Periodicidad
pf-place-id = Lugar
pf-plot = Parcela
pf-polygenic-score = Puntuación poligénica
pf-postal-code = Código postal
pf-precision = Precisión
pf-prescription = Graduación
pf-proficiency = Nivel de dominio
pf-provider = Proveedor
pf-quintile = Quintil de ingresos
pf-rank = Grado
pf-rank-text = Grado tal como figura
pf-reaction = Reacción
pf-reference-build = Genoma de referencia
pf-reference-high = Valor de referencia superior
pf-reference-low = Valor de referencia inferior
pf-register-type = Tipo de inscripción
pf-result = Resultado
pf-role = Cargo
pf-sacrament = Sacramento o rito
pf-score = Puntuación
pf-sentence = Pena
pf-sequence = Lugar en la cadena causal
pf-service = Ejército
pf-severity = Gravedad
pf-shannon-diversity = Diversidad de Shannon
pf-shape = Forma
pf-significance = Significado clínico
pf-snp-count = SNP analizados
pf-sport = Deporte
pf-subclade = Subclado
pf-substance = Sustancia
pf-summary = Resumen
pf-systolic = Sistólica
pf-tenure = Régimen de tenencia
pf-test = Prueba
pf-threshold-db = Umbral auditivo
pf-title = Título
pf-tonnes-co2e-per-year = Emisiones
pf-tradition = Tradición
pf-tree-version = Versión del árbol
pf-unit = Unidad
pf-use = Uso
pf-variant = Variante
pf-volume = Tomo
pf-zygosity = Cigosidad
pu-cm = { $n } cm
pu-kg = { $n } kg
pu-kg-m2 = { $n } kg/m²
pu-percent = { $n } %
pu-mm = { $n } mm
pu-hz = { $n } Hz
pu-words-min = { $n } palabras/min
pu-db-hl = { $n } dB HL
pu-mmhg = { $n } mmHg
pu-bpm = { $n } lpm
pu-litres = { $n } l
pu-coverage = { $n }×
pu-years = { $n } años
pu-t-co2e-yr = { $n } t CO₂e al año
pv-sensitive-class-health = Salud y creencias
pv-sensitive-class-biometrics = Datos biométricos
pv-sensitive-class-genomics = Datos genómicos
pv-sensitive-class-legal = Antecedentes penales
pv-laterality-left = Izquierdo
pv-laterality-right = Derecho
pv-laterality-both = Ambos
pv-body-region-head = Cabeza
pv-body-region-face = Cara
pv-body-region-neck = Cuello
pv-body-region-left-shoulder = Hombro izquierdo
pv-body-region-right-shoulder = Hombro derecho
pv-body-region-left-arm = Brazo izquierdo
pv-body-region-right-arm = Brazo derecho
pv-body-region-left-hand = Mano izquierda
pv-body-region-right-hand = Mano derecha
pv-body-region-chest = Tórax
pv-body-region-abdomen = Abdomen
pv-body-region-upper-back = Parte alta de la espalda
pv-body-region-lower-back = Parte baja de la espalda
pv-body-region-pelvis = Pelvis y caderas
pv-body-region-left-leg = Pierna izquierda
pv-body-region-right-leg = Pierna derecha
pv-body-region-left-foot = Pie izquierdo
pv-body-region-right-foot = Pie derecho
pv-body-region-internal = Interna
pv-body-region-whole-body = Todo el cuerpo
pv-body-region-other = Otra zona
pv-artefact-type-mesh = Malla
pv-artefact-type-point-cloud = Nube de puntos
pv-artefact-type-skin-texture-map = Mapa de textura de la piel
pv-artefact-type-skeletal-rig = Esqueleto de animación
pv-artefact-type-voice-corpus = Corpus de voz
pv-artefact-type-text-corpus = Corpus de textos
pv-artefact-type-trace-archive = Archivo de actividad en línea
pv-artefact-type-behaviour-model = Modelo de comportamiento
pv-artefact-type-fingerprint-card = Ficha dactilar
pv-artefact-type-fingerprint-template = Plantilla de huella
pv-artefact-type-retinal-image = Imagen de retina
pv-artefact-type-voiceprint = Huella vocal
pv-consent-given = Otorgado
pv-consent-given-by-estate = Otorgado por los herederos
pv-consent-refused = Denegado
pv-consent-withdrawn = Retirado
pv-consent-not-asked = No solicitado
pv-consent-unknown = Desconocido
pv-sex-at-birth-female = Femenino
pv-sex-at-birth-male = Masculino
pv-sex-at-birth-intersex = Intersexual
pv-sex-at-birth-undetermined = Indeterminado
pv-sex-at-birth-unknown = Desconocido
pv-gender-identity-woman = Mujer
pv-gender-identity-man = Hombre
pv-gender-identity-non-binary = No binaria
pv-gender-identity-other = Otra
pv-gender-identity-undisclosed = No declarada
pv-gender-identity-unknown = Desconocida
pv-title-kind-nobility = Nobiliario
pv-title-kind-academic = Académico
pv-title-kind-professional = Profesional
pv-title-kind-religious = Religioso
pv-title-kind-military = Militar
pv-title-kind-civic = Honorífico
pv-title-kind-courtesy = De cortesía
pv-title-kind-other = Otro
pv-register-type-birth = Nacimiento
pv-register-type-baptism = Bautismo
pv-register-type-marriage = Matrimonio
pv-register-type-death = Defunción
pv-register-type-burial = Entierro
pv-register-type-divorce = Divorcio
pv-register-type-recognition = Reconocimiento de hijo
pv-register-type-legitimation = Legitimación
pv-register-type-adoption = Adopción
pv-register-type-name-change = Cambio de nombre
pv-register-type-other = Otro
pv-build-slight = Menuda
pv-build-slim = Delgada
pv-build-average = Media
pv-build-sturdy = Robusta
pv-build-stout = Corpulenta
pv-build-heavy = Gruesa
pv-eye-colour-light-blue = Azul claro
pv-eye-colour-blue = Azul
pv-eye-colour-dark-blue = Azul oscuro
pv-eye-colour-grey = Gris
pv-eye-colour-blue-grey = Gris azulado
pv-eye-colour-green = Verde
pv-eye-colour-grey-green = Gris verdoso
pv-eye-colour-hazel = Avellana
pv-eye-colour-amber = Ámbar
pv-eye-colour-light-brown = Castaño claro
pv-eye-colour-brown = Castaño
pv-eye-colour-dark-brown = Castaño oscuro
pv-eye-colour-black = Negro
pv-eye-colour-mixed = Mixto
pv-eye-colour-other = Otro
pv-eye-shape-almond = Almendrados
pv-eye-shape-round = Redondos
pv-eye-shape-hooded = Con párpado caído
pv-eye-shape-monolid = Sin pliegue palpebral
pv-eye-shape-deep-set = Hundidos
pv-eye-shape-protruding = Saltones
pv-eye-shape-upturned = Hacia arriba
pv-eye-shape-downturned = Hacia abajo
pv-eye-shape-other = Otra
pv-eye-spacing-close-set = Juntos
pv-eye-spacing-average = Normal
pv-eye-spacing-wide-set = Separados
pv-hair-colour-black = Negro
pv-hair-colour-dark-brown = Castaño oscuro
pv-hair-colour-brown = Castaño
pv-hair-colour-light-brown = Castaño claro
pv-hair-colour-auburn = Caoba
pv-hair-colour-red = Pelirrojo
pv-hair-colour-strawberry-blond = Rubio rojizo
pv-hair-colour-dark-blond = Rubio oscuro
pv-hair-colour-blond = Rubio
pv-hair-colour-light-blond = Rubio claro
pv-hair-colour-grey = Canoso
pv-hair-colour-white = Blanco
pv-hair-colour-none = Sin pelo
pv-hair-colour-other = Otro
pv-hair-texture-straight = Liso
pv-hair-texture-wavy = Ondulado
pv-hair-texture-curly = Rizado
pv-hair-texture-coily = Muy rizado
pv-hair-texture-other = Otro
pv-hairline-straight = Recta
pv-hairline-rounded = Redondeada
pv-hairline-widows-peak = En pico
pv-hairline-m-shaped = En M
pv-hairline-bell-shaped = En campana
pv-hairline-uneven = Irregular
pv-hairline-receding = Con entradas
pv-hairline-bald = Calvicie
pv-facial-hair-none = Ninguno
pv-facial-hair-stubble = Barba de pocos días
pv-facial-hair-moustache = Bigote
pv-facial-hair-goatee = Perilla
pv-facial-hair-full-beard = Barba poblada
pv-facial-hair-sideburns = Patillas
pv-facial-hair-other = Otro
pv-body-hair-none = Ninguno
pv-body-hair-sparse = Escaso
pv-body-hair-moderate = Moderado
pv-body-hair-dense = Abundante
pv-skin-tone-type-i = Tipo I — siempre se quema, nunca se broncea
pv-skin-tone-type-ii = Tipo II — suele quemarse, se broncea poco
pv-skin-tone-type-iii = Tipo III — a veces se quema, se broncea de forma uniforme
pv-skin-tone-type-iv = Tipo IV — rara vez se quema, se broncea bien
pv-skin-tone-type-v = Tipo V — casi nunca se quema
pv-skin-tone-type-vi = Tipo VI — nunca se quema
pv-skin-undertone-cool = Frío
pv-skin-undertone-neutral = Neutro
pv-skin-undertone-warm = Cálido
pv-skin-undertone-olive = Oliváceo
pv-freckles-none = Ninguna
pv-freckles-few = Pocas
pv-freckles-moderate = Moderadas
pv-freckles-many = Muchas
pv-pigmentation-mark-birthmark = Marca de nacimiento
pv-pigmentation-mark-port-wine-stain = Mancha en vino de Oporto
pv-pigmentation-mark-cafe-au-lait-spot = Mancha café con leche
pv-pigmentation-mark-depigmented-patch = Mancha despigmentada
pv-pigmentation-mark-hyperpigmented-patch = Mancha hiperpigmentada
pv-pigmentation-mark-other = Otra
pv-mole-shape-round = Redondo
pv-mole-shape-oval = Ovalado
pv-mole-shape-irregular = Irregular
pv-mole-shape-other = Otra
pv-face-shape-oval = Ovalada
pv-face-shape-round = Redonda
pv-face-shape-square = Cuadrada
pv-face-shape-oblong = Alargada
pv-face-shape-heart = De corazón
pv-face-shape-diamond = De diamante
pv-face-shape-triangular = Triangular
pv-nose-shape-straight = Recta
pv-nose-shape-aquiline = Aguileña
pv-nose-shape-snub = Chata
pv-nose-shape-upturned = Respingona
pv-nose-shape-flat = Aplastada
pv-nose-shape-broad = Ancha
pv-nose-shape-bulbous = Bulbosa
pv-nose-shape-crooked = Torcida
pv-nose-shape-other = Otra
pv-ear-shape-free-lobe = Lóbulos libres
pv-ear-shape-attached-lobe = Lóbulos pegados
pv-ear-shape-protruding = Salientes
pv-ear-shape-close-set = Pegadas
pv-ear-shape-pointed = Puntiagudas
pv-ear-shape-other = Otra
pv-lip-shape-thin = Finos
pv-lip-shape-medium = Medianos
pv-lip-shape-full = Carnosos
pv-lip-shape-bow-shaped = En arco
pv-lip-shape-wide = Anchos
pv-lip-shape-downturned = Con comisuras caídas
pv-lip-shape-other = Otra
pv-dentition-primary = Temporal
pv-dentition-mixed = Mixta
pv-dentition-permanent-complete = Permanente, completa
pv-dentition-permanent-partial-loss = Permanente, incompleta
pv-dentition-edentulous = Edéntula
pv-dentition-partial-denture = Prótesis parcial
pv-dentition-full-denture = Prótesis completa
pv-dentition-implants = Implantes dentales
pv-malocclusion-normal = Oclusión normal
pv-malocclusion-class-i = Clase I
pv-malocclusion-class-ii-division-1 = Clase II, división 1
pv-malocclusion-class-ii-division-2 = Clase II, división 2
pv-malocclusion-class-iii = Clase III
pv-posture-ideal = Correcta
pv-posture-kyphotic-lordotic = Cifolordótica
pv-posture-flat-back = Espalda plana
pv-posture-sway-back = Espalda arqueada
pv-posture-stooped = Encorvada
pv-posture-scoliotic = Escoliótica
pv-posture-other = Otra
pv-gait-brisk = Ágil
pv-gait-average = Normal
pv-gait-slow = Lenta
pv-gait-shuffling = Arrastrando los pies
pv-gait-limping = Cojeando
pv-gait-waddling = Balanceante
pv-gait-unsteady = Inestable
pv-gait-stiff = Rígida
pv-gait-other = Otra
pv-vocal-timbre-bright = Brillante
pv-vocal-timbre-dark = Oscuro
pv-vocal-timbre-warm = Cálido
pv-vocal-timbre-breathy = Aireado
pv-vocal-timbre-nasal = Nasal
pv-vocal-timbre-hoarse = Ronco
pv-vocal-timbre-resonant = Resonante
pv-vocal-timbre-thin = Fino
pv-vocal-timbre-other = Otro
pv-speech-register-frozen = Solemne
pv-speech-register-formal = Formal
pv-speech-register-consultative = Neutro
pv-speech-register-casual = Coloquial
pv-speech-register-intimate = Íntimo
pv-handedness-left = Zurdo
pv-handedness-right = Diestro
pv-handedness-ambidextrous = Ambidiestro
pv-handedness-mixed = Mixta
pv-handedness-unknown = Desconocida
pv-hearing-grade-normal = Normal
pv-hearing-grade-mild = Leve
pv-hearing-grade-moderate = Moderada
pv-hearing-grade-moderately-severe = Moderadamente grave
pv-hearing-grade-severe = Grave
pv-hearing-grade-profound = Profunda
pv-hearing-grade-complete = Total
pv-optical-correction-none = Ninguna
pv-optical-correction-glasses = Gafas
pv-optical-correction-contact-lenses = Lentillas
pv-optical-correction-glasses-and-contact-lenses = Gafas y lentillas
pv-optical-correction-refractive-surgery = Cirugía refractiva
pv-optical-correction-intraocular-lens = Lente intraocular
pv-optical-correction-other = Otra
pv-rhesus-positive = RhD positivo
pv-rhesus-negative = RhD negativo
pv-rhesus-weak-d = D débil
pv-rhesus-unknown = Desconocido
pv-icd10-chapter-infectious-parasitic = I Enfermedades infecciosas y parasitarias
pv-icd10-chapter-neoplasms = II Neoplasias
pv-icd10-chapter-blood-immune = III Sangre y sistema inmunitario
pv-icd10-chapter-endocrine-metabolic = IV Endocrinas, nutricionales y metabólicas
pv-icd10-chapter-mental-behavioural = V Trastornos mentales y del comportamiento
pv-icd10-chapter-nervous-system = VI Sistema nervioso
pv-icd10-chapter-eye-adnexa = VII Ojo y anexos
pv-icd10-chapter-ear-mastoid = VIII Oído y apófisis mastoides
pv-icd10-chapter-circulatory = IX Sistema circulatorio
pv-icd10-chapter-respiratory = X Sistema respiratorio
pv-icd10-chapter-digestive = XI Sistema digestivo
pv-icd10-chapter-skin = XII Piel y tejido subcutáneo
pv-icd10-chapter-musculoskeletal = XIII Sistema osteomuscular
pv-icd10-chapter-genitourinary = XIV Sistema genitourinario
pv-icd10-chapter-pregnancy-childbirth = XV Embarazo y parto
pv-icd10-chapter-perinatal = XVI Afecciones perinatales
pv-icd10-chapter-congenital = XVII Malformaciones congénitas
pv-icd10-chapter-ill-defined = XVIII Síntomas y causas mal definidas
pv-icd10-chapter-injury-poisoning = XIX Traumatismos y envenenamientos
pv-icd10-chapter-external-causes = XX Causas externas
pv-icd10-chapter-health-factors = XXI Factores que influyen en la salud
pv-icd10-chapter-special-purposes = XXII Códigos para propósitos especiales
pv-diagnosis-status-diagnosed = Diagnosticada
pv-diagnosis-status-suspected = Sospechada
pv-diagnosis-status-self-reported = Referida
pv-diagnosis-status-unknown = Desconocido
pv-prosthesis-kind-limb = Miembro
pv-prosthesis-kind-joint = Articular
pv-prosthesis-kind-ocular = Ocular
pv-prosthesis-kind-dental = Dental
pv-prosthesis-kind-auditory = Auditiva
pv-prosthesis-kind-breast = Mamaria
pv-prosthesis-kind-other = Otra
pv-implant-kind-orthopaedic = Ortopédico
pv-implant-kind-dental = Dental
pv-implant-kind-cochlear = Coclear
pv-implant-kind-breast = Mamario
pv-implant-kind-intraocular-lens = Lente intraocular
pv-implant-kind-contraceptive = Anticonceptivo
pv-implant-kind-cosmetic = Estético
pv-implant-kind-other = Otro
pv-device-kind-pacemaker = Marcapasos
pv-device-kind-implantable-defibrillator = Desfibrilador implantable
pv-device-kind-cardiac-resynchronisation = Dispositivo de resincronización
pv-device-kind-ventricular-assist = Asistencia ventricular
pv-device-kind-neurostimulator = Neuroestimulador
pv-device-kind-insulin-pump = Bomba de insulina
pv-device-kind-drug-port = Reservorio subcutáneo
pv-device-kind-shunt = Derivación
pv-device-kind-stent = Stent
pv-device-kind-other = Otro
pv-allergy-type-drug = Medicamento
pv-allergy-type-food = Alimento
pv-allergy-type-environmental = Ambiental
pv-allergy-type-insect-venom = Veneno de insecto
pv-allergy-type-latex = Látex
pv-allergy-type-other = Otro
pv-allergy-severity-mild = Leve
pv-allergy-severity-moderate = Moderada
pv-allergy-severity-severe = Grave
pv-allergy-severity-anaphylactic = Anafiláctica
pv-allergy-severity-unknown = Desconocida
pv-pathogen-diphtheria = Difteria
pv-pathogen-tetanus = Tétanos
pv-pathogen-pertussis = Tos ferina
pv-pathogen-poliomyelitis = Poliomielitis
pv-pathogen-measles = Sarampión
pv-pathogen-mumps = Paperas
pv-pathogen-rubella = Rubéola
pv-pathogen-varicella = Varicela
pv-pathogen-smallpox = Viruela
pv-pathogen-tuberculosis = Tuberculosis
pv-pathogen-hepatitis-a = Hepatitis A
pv-pathogen-hepatitis-b = Hepatitis B
pv-pathogen-hepatitis-c = Hepatitis C
pv-pathogen-haemophilus-influenzae-b = Haemophilus influenzae tipo b
pv-pathogen-pneumococcal = Neumococo
pv-pathogen-meningococcal = Meningococo
pv-pathogen-human-papillomavirus = Virus del papiloma humano
pv-pathogen-influenza = Gripe
pv-pathogen-covid-19 = COVID-19
pv-pathogen-rotavirus = Rotavirus
pv-pathogen-yellow-fever = Fiebre amarilla
pv-pathogen-typhoid = Fiebre tifoidea
pv-pathogen-cholera = Cólera
pv-pathogen-rabies = Rabia
pv-pathogen-japanese-encephalitis = Encefalitis japonesa
pv-pathogen-tick-borne-encephalitis = Encefalitis por garrapatas
pv-pathogen-hiv = VIH
pv-pathogen-syphilis = Sífilis
pv-pathogen-toxoplasmosis = Toxoplasmosis
pv-pathogen-cytomegalovirus = Citomegalovirus
pv-pathogen-epstein-barr = Virus de Epstein-Barr
pv-pathogen-other = Otro
pv-vaccination-status-vaccinated = Vacunado
pv-vaccination-status-partially-vaccinated = Parcialmente vacunado
pv-vaccination-status-unvaccinated = No vacunado
pv-vaccination-status-contraindicated = Contraindicada
pv-vaccination-status-unknown = Desconocido
pv-serology-result-positive = Positivo
pv-serology-result-negative = Negativo
pv-serology-result-equivocal = Dudoso
pv-serology-result-unknown = Desconocido
pv-lab-panel-basic-metabolic = Perfil metabólico básico
pv-lab-panel-lipid = Perfil lipídico
pv-lab-panel-liver = Perfil hepático
pv-lab-panel-renal = Perfil renal
pv-lab-panel-glycated-haemoglobin = Hemoglobina glicada
pv-lab-panel-iron = Metabolismo del hierro
pv-lab-analyte-sodium = Sodio
pv-lab-analyte-potassium = Potasio
pv-lab-analyte-chloride = Cloro
pv-lab-analyte-bicarbonate = Bicarbonato
pv-lab-analyte-urea = Urea
pv-lab-analyte-creatinine = Creatinina
pv-lab-analyte-glucose = Glucosa
pv-lab-analyte-calcium = Calcio
pv-lab-analyte-total-cholesterol = Colesterol total
pv-lab-analyte-ldl-cholesterol = Colesterol LDL
pv-lab-analyte-hdl-cholesterol = Colesterol HDL
pv-lab-analyte-triglycerides = Triglicéridos
pv-lab-analyte-non-hdl-cholesterol = Colesterol no HDL
pv-lab-analyte-alt = Alanina aminotransferasa (ALT)
pv-lab-analyte-ast = Aspartato aminotransferasa (AST)
pv-lab-analyte-alp = Fosfatasa alcalina (FA)
pv-lab-analyte-ggt = Gamma-glutamil transferasa (GGT)
pv-lab-analyte-total-bilirubin = Bilirrubina total
pv-lab-analyte-direct-bilirubin = Bilirrubina directa
pv-lab-analyte-albumin = Albúmina
pv-lab-analyte-total-protein = Proteínas totales
pv-lab-analyte-egfr = FG estimado
pv-lab-analyte-uric-acid = Ácido úrico
pv-lab-analyte-phosphate = Fosfato
pv-lab-analyte-urine-albumin-creatinine-ratio = Cociente albúmina/creatinina en orina
pv-lab-analyte-hba1c = HbA1c
pv-lab-analyte-serum-iron = Hierro sérico
pv-lab-analyte-ferritin = Ferritina
pv-lab-analyte-transferrin = Transferrina
pv-lab-analyte-transferrin-saturation = Saturación de transferrina
pv-lab-analyte-tibc = Capacidad total de fijación del hierro
pv-lab-flag-low = Bajo
pv-lab-flag-normal = Normal
pv-lab-flag-high = Alto
pv-lab-flag-critical-low = Críticamente bajo
pv-lab-flag-critical-high = Críticamente alto
pv-nutrient-vitamin-a = Vitamina A
pv-nutrient-thiamine = Tiamina (B1)
pv-nutrient-riboflavin = Riboflavina (B2)
pv-nutrient-niacin = Niacina (B3)
pv-nutrient-vitamin-b6 = Vitamina B6
pv-nutrient-folate = Folato (B9)
pv-nutrient-vitamin-b12 = Vitamina B12
pv-nutrient-vitamin-c = Vitamina C
pv-nutrient-vitamin-d = Vitamina D
pv-nutrient-vitamin-e = Vitamina E
pv-nutrient-vitamin-k = Vitamina K
pv-nutrient-iron = Hierro
pv-nutrient-zinc = Zinc
pv-nutrient-magnesium = Magnesio
pv-nutrient-calcium = Calcio
pv-nutrient-iodine = Yodo
pv-nutrient-selenium = Selenio
pv-nutrient-copper = Cobre
pv-nutrient-potassium = Potasio
pv-nutrient-phosphorus = Fósforo
pv-nutrient-other = Otro
pv-sleep-disorder-insomnia = Insomnio
pv-sleep-disorder-sleep-related-breathing = Trastorno respiratorio del sueño
pv-sleep-disorder-central-hypersomnolence = Hipersomnia central
pv-sleep-disorder-circadian-rhythm = Trastorno del ritmo circadiano
pv-sleep-disorder-parasomnia = Parasomnia
pv-sleep-disorder-sleep-related-movement = Trastorno del movimiento durante el sueño
pv-sleep-disorder-other = Otro
pv-assessment-instrument-phq-9 = PHQ-9
pv-assessment-instrument-gad-7 = GAD-7
pv-assessment-instrument-bdi-ii = BDI-II
pv-assessment-instrument-hads = HADS
pv-assessment-instrument-k10 = K10
pv-assessment-instrument-gds-15 = GDS-15
pv-assessment-instrument-mmse = MMSE
pv-assessment-instrument-moca = MoCA
pv-assessment-instrument-audit = AUDIT
pv-assessment-instrument-clinical-interview = Entrevista clínica
pv-assessment-instrument-other = Otro
pv-assessment-severity-none-minimal = Nula o mínima
pv-assessment-severity-mild = Leve
pv-assessment-severity-moderate = Moderada
pv-assessment-severity-moderately-severe = Moderadamente grave
pv-assessment-severity-severe = Grave
pv-reference-build-grch36 = GRCh36 (hg18)
pv-reference-build-grch37 = GRCh37 (hg19)
pv-reference-build-grch38 = GRCh38 (hg38)
pv-reference-build-t2t-chm13 = T2T-CHM13
pv-genomic-file-format-raw-microarray = Datos brutos de micromatriz
pv-genomic-file-format-fastq = FASTQ
pv-genomic-file-format-bam = BAM
pv-genomic-file-format-cram = CRAM
pv-genomic-file-format-vcf = VCF
pv-genomic-file-format-gvcf = gVCF
pv-genomic-file-format-other = Otro
pv-zygosity-heterozygous = Heterocigoto
pv-zygosity-homozygous = Homocigoto
pv-zygosity-hemizygous = Hemicigoto
pv-zygosity-compound-heterozygous = Heterocigoto compuesto
pv-clinical-significance-pathogenic = Patogénica
pv-clinical-significance-likely-pathogenic = Probablemente patogénica
pv-clinical-significance-uncertain-significance = Significado incierto
pv-clinical-significance-likely-benign = Probablemente benigna
pv-clinical-significance-benign = Benigna
pv-inheritance-pattern-autosomal-dominant = Autosómica dominante
pv-inheritance-pattern-autosomal-recessive = Autosómica recesiva
pv-inheritance-pattern-x-linked-dominant = Dominante ligada al X
pv-inheritance-pattern-x-linked-recessive = Recesiva ligada al X
pv-inheritance-pattern-y-linked = Ligada al Y
pv-inheritance-pattern-mitochondrial = Mitocondrial
pv-inheritance-pattern-multifactorial = Multifactorial
pv-inheritance-pattern-unknown = Desconocida
pv-carrier-status-affected = Afectado
pv-carrier-status-carrier = Portador
pv-carrier-status-not-carrier = No portador
pv-carrier-status-unknown = Desconocido
pv-epigenetic-clock-horvath = Horvath
pv-epigenetic-clock-hannum = Hannum
pv-epigenetic-clock-phenoage = PhenoAge
pv-epigenetic-clock-grimage = GrimAge
pv-epigenetic-clock-dunedinpace = DunedinPACE
pv-epigenetic-clock-other = Otro
pv-metaboliser-status-poor = Metabolizador lento
pv-metaboliser-status-intermediate = Metabolizador intermedio
pv-metaboliser-status-normal = Metabolizador normal
pv-metaboliser-status-rapid = Metabolizador rápido
pv-metaboliser-status-ultrarapid = Metabolizador ultrarrápido
pv-autopsy-not-performed = No realizada
pv-autopsy-clinical = Clínica
pv-autopsy-forensic = Forense
pv-autopsy-external-examination = Solo examen externo
pv-autopsy-unknown = Desconocido
pv-disposition-burial = Inhumación
pv-disposition-cremation = Cremación
pv-disposition-entombment = Sepultura en mausoleo
pv-disposition-burial-at-sea = Sepultura en el mar
pv-disposition-natural-burial = Entierro natural
pv-disposition-body-donation = Donación del cuerpo a la ciencia
pv-disposition-other = Otro
pv-disposition-unknown = Desconocido
pv-address-use-principal = Residencia habitual
pv-address-use-secondary = Segunda residencia
pv-address-use-temporary = Residencia temporal
pv-address-use-postal = Dirección postal
pv-address-use-other = Otro
pv-nationality-mode-descent = Por filiación
pv-nationality-mode-birth-in-territory = Por nacimiento en el territorio
pv-nationality-mode-naturalisation = Por naturalización
pv-nationality-mode-marriage = Por matrimonio
pv-nationality-mode-registration = Por opción
pv-nationality-mode-restoration = Por recuperación
pv-nationality-mode-state-succession = Por cambio de soberanía
pv-nationality-mode-other = Otra
pv-language-proficiency-a1 = A1 Acceso
pv-language-proficiency-a2 = A2 Plataforma
pv-language-proficiency-b1 = B1 Umbral
pv-language-proficiency-b2 = B2 Avanzado
pv-language-proficiency-c1 = C1 Dominio operativo eficaz
pv-language-proficiency-c2 = C2 Maestría
pv-language-proficiency-native = Lengua materna
pv-isced-level-isced-0 = 0 Educación infantil
pv-isced-level-isced-1 = 1 Primaria
pv-isced-level-isced-2 = 2 Secundaria inferior
pv-isced-level-isced-3 = 3 Secundaria superior
pv-isced-level-isced-4 = 4 Postsecundaria no terciaria
pv-isced-level-isced-5 = 5 Terciaria de ciclo corto
pv-isced-level-isced-6 = 6 Grado o equivalente
pv-isced-level-isced-7 = 7 Máster o equivalente
pv-isced-level-isced-8 = 8 Doctorado o equivalente
pv-income-quintile-q1 = Quinto más bajo
pv-income-quintile-q2 = Segundo quinto
pv-income-quintile-q3 = Quinto central
pv-income-quintile-q4 = Cuarto quinto
pv-income-quintile-q5 = Quinto más alto
pv-pay-period-hourly = Por hora
pv-pay-period-daily = Por día
pv-pay-period-weekly = Por semana
pv-pay-period-monthly = Por mes
pv-pay-period-annual = Por año
pv-tenure-owned = Propiedad
pv-tenure-co-owned = Copropiedad
pv-tenure-leasehold = Arrendamiento a largo plazo
pv-tenure-rented = Alquiler
pv-tenure-usufruct = Usufructo
pv-tenure-other = Otro
pv-distinction-kind-order = Orden
pv-distinction-kind-decoration = Condecoración
pv-distinction-kind-medal = Medalla
pv-distinction-kind-title = Título honorífico
pv-distinction-kind-other = Otra
pv-military-service-army = Ejército de Tierra
pv-military-service-navy = Armada
pv-military-service-air-force = Ejército del Aire
pv-military-service-marines = Infantería de Marina
pv-military-service-gendarmerie = Gendarmería
pv-military-service-border-guard = Guardia de fronteras
pv-military-service-national-guard = Guardia nacional
pv-military-service-other = Otro
pv-rank-category-enlisted = Tropa
pv-rank-category-non-commissioned = Suboficiales
pv-rank-category-warrant = Suboficiales mayores
pv-rank-category-officer-cadet = Cadetes
pv-rank-category-junior-officer = Oficiales
pv-rank-category-senior-officer = Jefes
pv-rank-category-general-officer = Oficiales generales
pv-iccs-section-acts-leading-to-death = 01 Actos que causan la muerte
pv-iccs-section-acts-causing-harm = 02 Actos que causan daño
pv-iccs-section-sexual-acts = 03 Actos lesivos de naturaleza sexual
pv-iccs-section-property-with-violence = 04 Contra la propiedad con violencia
pv-iccs-section-property-only = 05 Contra la propiedad
pv-iccs-section-controlled-substances = 06 Sustancias controladas
pv-iccs-section-fraud-deception-corruption = 07 Fraude, engaño o corrupción
pv-iccs-section-public-order-and-state = 08 Contra el orden público y el Estado
pv-iccs-section-public-safety-and-security = 09 Contra la seguridad pública
pv-iccs-section-natural-environment = 10 Contra el medio ambiente
pv-iccs-section-other-criminal-acts = 11 Otros delitos
pv-case-outcome-convicted = Condenado
pv-case-outcome-acquitted = Absuelto
pv-case-outcome-dismissed = Sobreseído
pv-case-outcome-conviction-quashed = Condena anulada
pv-case-outcome-pardoned = Indultado
pv-case-outcome-amnestied = Amnistiado
pv-case-outcome-expunged = Antecedentes cancelados
pv-case-outcome-pending = Pendiente
pv-case-outcome-unknown = Desconocido
pv-religion-buddhism = Budismo
pv-religion-christianity-catholic = Cristianismo: católico
pv-religion-christianity-orthodox = Cristianismo: ortodoxo
pv-religion-christianity-protestant = Cristianismo: protestante
pv-religion-christianity-other = Cristianismo: otro
pv-religion-hinduism = Hinduismo
pv-religion-islam-sunni = Islam: suní
pv-religion-islam-shia = Islam: chií
pv-religion-islam-other = Islam: otro
pv-religion-jainism = Jainismo
pv-religion-judaism = Judaísmo
pv-religion-sikhism = Sijismo
pv-religion-bahai = Fe bahaí
pv-religion-shinto = Sintoísmo
pv-religion-taoism = Taoísmo
pv-religion-zoroastrianism = Zoroastrismo
pv-religion-traditional = Religión tradicional o popular
pv-religion-other = Otra
pv-religion-none = Sin religión
pv-religion-unknown = Desconocida
pv-sacrament-baptism = Bautismo
pv-sacrament-confirmation = Confirmación
pv-sacrament-first-communion = Primera comunión
pv-sacrament-reconciliation = Confesión
pv-sacrament-anointing-of-the-sick = Unción de los enfermos
pv-sacrament-holy-orders = Orden sacerdotal
pv-sacrament-matrimony = Matrimonio
pv-sacrament-other-rite = Otro rito
pv-political-position-far-left = Extrema izquierda
pv-political-position-left = Izquierda
pv-political-position-centre-left = Centroizquierda
pv-political-position-centre = Centro
pv-political-position-centre-right = Centroderecha
pv-political-position-right = Derecha
pv-political-position-far-right = Extrema derecha
pv-political-position-apolitical = Apolítico
pv-political-position-other = Fuera de este eje
pv-political-position-unknown = Desconocida
pv-membership-kind-trade-union = Sindicato
pv-membership-kind-political-party = Partido político
pv-membership-kind-professional-body = Colegio profesional
pv-membership-kind-religious-order = Orden religiosa
pv-membership-kind-religious-association = Asociación religiosa
pv-membership-kind-fraternal-order = Cofradía
pv-membership-kind-veterans-association = Asociación de veteranos
pv-membership-kind-sports-club = Club deportivo
pv-membership-kind-cultural-association = Asociación cultural
pv-membership-kind-charitable-association = Asociación benéfica
pv-membership-kind-other = Otra
pv-personality-instrument-neo-pi-3 = NEO-PI-3
pv-personality-instrument-neo-ffi-3 = NEO-FFI-3
pv-personality-instrument-bfi-2 = BFI-2
pv-personality-instrument-ipip-neo-120 = IPIP-NEO-120
pv-personality-instrument-tipi = TIPI
pv-personality-instrument-hexaco-pi-r = HEXACO-PI-R
pv-personality-instrument-observer-rating = Valoración de alguien que la conoció
pv-personality-instrument-inferred = Deducido de documentos
pv-personality-instrument-other = Otro
pv-introversion-extraversion-strongly-introverted = Muy introvertido
pv-introversion-extraversion-introverted = Introvertido
pv-introversion-extraversion-ambiverted = Ambivertido
pv-introversion-extraversion-extraverted = Extravertido
pv-introversion-extraversion-strongly-extraverted = Muy extravertido
pv-stress-tolerance-very-low = Muy baja
pv-stress-tolerance-low = Baja
pv-stress-tolerance-moderate = Moderada
pv-stress-tolerance-high = Alta
pv-stress-tolerance-very-high = Muy alta
pv-decision-style-rational = Racional
pv-decision-style-intuitive = Intuitivo
pv-decision-style-dependent = Dependiente
pv-decision-style-avoidant = Evitativo
pv-decision-style-spontaneous = Espontáneo
pv-sport-level-recreational = Recreativo
pv-sport-level-amateur-competitive = Aficionado de competición
pv-sport-level-semi-professional = Semiprofesional
pv-sport-level-professional = Profesional
pv-diet-omnivore = Omnívora
pv-diet-flexitarian = Flexitariana
pv-diet-pescatarian = Pescetariana
pv-diet-vegetarian = Vegetariana
pv-diet-vegan = Vegana
pv-diet-other = Otra
pv-substance-tobacco = Tabaco y nicotina
pv-substance-alcohol = Alcohol
pv-substance-cannabis = Cannabis
pv-substance-opioids = Opioides
pv-substance-stimulants = Estimulantes
pv-substance-sedatives-hypnotics = Sedantes e hipnóticos
pv-substance-hallucinogens = Alucinógenos
pv-substance-inhalants = Inhalantes
pv-substance-gambling = Juego
pv-substance-gaming = Videojuegos
pv-substance-other = Otra
pv-use-pattern-occasional-use = Consumo ocasional
pv-use-pattern-regular-use = Consumo habitual
pv-use-pattern-harmful-use = Consumo perjudicial
pv-use-pattern-dependence = Dependencia
pv-use-pattern-in-remission = En remisión
pv-lineage-biological = Biológica
pv-lineage-adoptive = Adoptiva
pv-lineage-foster = De acogida
pv-lineage-step = Por afinidad
pv-lineage-guardianship = Tutela
pv-lineage-unknown = Desconocida
pv-link-relation-godparent = Padrino o madrina
pv-link-relation-godchild = Ahijado
pv-link-relation-witness = Testigo
pv-link-relation-officiant = Oficiante
pv-link-relation-business-partner = Socio
pv-link-relation-employer = Empleador
pv-link-relation-employee = Empleado
pv-link-relation-mentor = Mentor
pv-link-relation-apprentice = Aprendiz
pv-link-relation-close-friend = Amigo íntimo
pv-link-relation-neighbour = Vecino
pv-link-relation-guardian = Tutor
pv-link-relation-ward = Pupilo
pv-link-relation-other = Otra
pv-country-AD = Andorra
pv-country-AE = Emiratos Árabes Unidos
pv-country-AF = Afganistán
pv-country-AG = Antigua y Barbuda
pv-country-AI = Anguila
pv-country-AL = Albania
pv-country-AM = Armenia
pv-country-AO = Angola
pv-country-AQ = Antártida
pv-country-AR = Argentina
pv-country-AS = Samoa Americana
pv-country-AT = Austria
pv-country-AU = Australia
pv-country-AW = Aruba
pv-country-AX = Islas Aland
pv-country-AZ = Azerbaiyán
pv-country-BA = Bosnia y Herzegovina
pv-country-BB = Barbados
pv-country-BD = Bangladés
pv-country-BE = Bélgica
pv-country-BF = Burkina Faso
pv-country-BG = Bulgaria
pv-country-BH = Baréin
pv-country-BI = Burundi
pv-country-BJ = Benín
pv-country-BL = San Bartolomé
pv-country-BM = Bermudas
pv-country-BN = Brunéi
pv-country-BO = Bolivia
pv-country-BQ = Caribe neerlandés
pv-country-BR = Brasil
pv-country-BS = Bahamas
pv-country-BT = Bután
pv-country-BV = Isla Bouvet
pv-country-BW = Botsuana
pv-country-BY = Bielorrusia
pv-country-BZ = Belice
pv-country-CA = Canadá
pv-country-CC = Islas Cocos
pv-country-CD = República Democrática del Congo
pv-country-CF = República Centroafricana
pv-country-CG = Congo
pv-country-CH = Suiza
pv-country-CI = Côte d’Ivoire
pv-country-CK = Islas Cook
pv-country-CL = Chile
pv-country-CM = Camerún
pv-country-CN = China
pv-country-CO = Colombia
pv-country-CR = Costa Rica
pv-country-CU = Cuba
pv-country-CV = Cabo Verde
pv-country-CW = Curazao
pv-country-CX = Isla de Navidad
pv-country-CY = Chipre
pv-country-CZ = Chequia
pv-country-DE = Alemania
pv-country-DJ = Yibuti
pv-country-DK = Dinamarca
pv-country-DM = Dominica
pv-country-DO = República Dominicana
pv-country-DZ = Argelia
pv-country-EC = Ecuador
pv-country-EE = Estonia
pv-country-EG = Egipto
pv-country-EH = Sáhara Occidental
pv-country-ER = Eritrea
pv-country-ES = España
pv-country-ET = Etiopía
pv-country-FI = Finlandia
pv-country-FJ = Fiyi
pv-country-FK = Islas Malvinas
pv-country-FM = Micronesia
pv-country-FO = Islas Feroe
pv-country-FR = Francia
pv-country-GA = Gabón
pv-country-GB = Reino Unido
pv-country-GD = Granada
pv-country-GE = Georgia
pv-country-GF = Guayana Francesa
pv-country-GG = Guernesey
pv-country-GH = Ghana
pv-country-GI = Gibraltar
pv-country-GL = Groenlandia
pv-country-GM = Gambia
pv-country-GN = Guinea
pv-country-GP = Guadalupe
pv-country-GQ = Guinea Ecuatorial
pv-country-GR = Grecia
pv-country-GS = Islas Georgia del Sur y Sandwich del Sur
pv-country-GT = Guatemala
pv-country-GU = Guam
pv-country-GW = Guinea-Bisáu
pv-country-GY = Guyana
pv-country-HK = RAE de Hong Kong (China)
pv-country-HM = Islas Heard y McDonald
pv-country-HN = Honduras
pv-country-HR = Croacia
pv-country-HT = Haití
pv-country-HU = Hungría
pv-country-ID = Indonesia
pv-country-IE = Irlanda
pv-country-IL = Israel
pv-country-IM = Isla de Man
pv-country-IN = India
pv-country-IO = Territorio Británico del Océano Índico
pv-country-IQ = Irak
pv-country-IR = Irán
pv-country-IS = Islandia
pv-country-IT = Italia
pv-country-JE = Jersey
pv-country-JM = Jamaica
pv-country-JO = Jordania
pv-country-JP = Japón
pv-country-KE = Kenia
pv-country-KG = Kirguistán
pv-country-KH = Camboya
pv-country-KI = Kiribati
pv-country-KM = Comoras
pv-country-KN = San Cristóbal y Nieves
pv-country-KP = Corea del Norte
pv-country-KR = Corea del Sur
pv-country-KW = Kuwait
pv-country-KY = Islas Caimán
pv-country-KZ = Kazajistán
pv-country-LA = Laos
pv-country-LB = Líbano
pv-country-LC = Santa Lucía
pv-country-LI = Liechtenstein
pv-country-LK = Sri Lanka
pv-country-LR = Liberia
pv-country-LS = Lesoto
pv-country-LT = Lituania
pv-country-LU = Luxemburgo
pv-country-LV = Letonia
pv-country-LY = Libia
pv-country-MA = Marruecos
pv-country-MC = Mónaco
pv-country-MD = Moldavia
pv-country-ME = Montenegro
pv-country-MF = San Martín
pv-country-MG = Madagascar
pv-country-MH = Islas Marshall
pv-country-MK = Macedonia del Norte
pv-country-ML = Mali
pv-country-MM = Myanmar (Birmania)
pv-country-MN = Mongolia
pv-country-MO = RAE de Macao (China)
pv-country-MP = Islas Marianas del Norte
pv-country-MQ = Martinica
pv-country-MR = Mauritania
pv-country-MS = Montserrat
pv-country-MT = Malta
pv-country-MU = Mauricio
pv-country-MV = Maldivas
pv-country-MW = Malaui
pv-country-MX = México
pv-country-MY = Malasia
pv-country-MZ = Mozambique
pv-country-NA = Namibia
pv-country-NC = Nueva Caledonia
pv-country-NE = Níger
pv-country-NF = Isla Norfolk
pv-country-NG = Nigeria
pv-country-NI = Nicaragua
pv-country-NL = Países Bajos
pv-country-NO = Noruega
pv-country-NP = Nepal
pv-country-NR = Nauru
pv-country-NU = Niue
pv-country-NZ = Nueva Zelanda
pv-country-OM = Omán
pv-country-PA = Panamá
pv-country-PE = Perú
pv-country-PF = Polinesia Francesa
pv-country-PG = Papúa Nueva Guinea
pv-country-PH = Filipinas
pv-country-PK = Pakistán
pv-country-PL = Polonia
pv-country-PM = San Pedro y Miquelón
pv-country-PN = Islas Pitcairn
pv-country-PR = Puerto Rico
pv-country-PS = Territorios Palestinos
pv-country-PT = Portugal
pv-country-PW = Palaos
pv-country-PY = Paraguay
pv-country-QA = Catar
pv-country-RE = Reunión
pv-country-RO = Rumanía
pv-country-RS = Serbia
pv-country-RU = Rusia
pv-country-RW = Ruanda
pv-country-SA = Arabia Saudí
pv-country-SB = Islas Salomón
pv-country-SC = Seychelles
pv-country-SD = Sudán
pv-country-SE = Suecia
pv-country-SG = Singapur
pv-country-SH = Santa Elena
pv-country-SI = Eslovenia
pv-country-SJ = Svalbard y Jan Mayen
pv-country-SK = Eslovaquia
pv-country-SL = Sierra Leona
pv-country-SM = San Marino
pv-country-SN = Senegal
pv-country-SO = Somalia
pv-country-SR = Surinam
pv-country-SS = Sudán del Sur
pv-country-ST = Santo Tomé y Príncipe
pv-country-SV = El Salvador
pv-country-SX = Sint Maarten
pv-country-SY = Siria
pv-country-SZ = Esuatini
pv-country-TC = Islas Turcas y Caicos
pv-country-TD = Chad
pv-country-TF = Territorios Australes Franceses
pv-country-TG = Togo
pv-country-TH = Tailandia
pv-country-TJ = Tayikistán
pv-country-TK = Tokelau
pv-country-TL = Timor-Leste
pv-country-TM = Turkmenistán
pv-country-TN = Túnez
pv-country-TO = Tonga
pv-country-TR = Turquía
pv-country-TT = Trinidad y Tobago
pv-country-TV = Tuvalu
pv-country-TW = Taiwán
pv-country-TZ = Tanzania
pv-country-UA = Ucrania
pv-country-UG = Uganda
pv-country-UM = Islas menores alejadas de EE. UU.
pv-country-US = Estados Unidos
pv-country-UY = Uruguay
pv-country-UZ = Uzbekistán
pv-country-VA = Ciudad del Vaticano
pv-country-VC = San Vicente y las Granadinas
pv-country-VE = Venezuela
pv-country-VG = Islas Vírgenes Británicas
pv-country-VI = Islas Vírgenes de EE. UU.
pv-country-VN = Vietnam
pv-country-VU = Vanuatu
pv-country-WF = Wallis y Futuna
pv-country-WS = Samoa
pv-country-YE = Yemen
pv-country-YT = Mayotte
pv-country-ZA = Sudáfrica
pv-country-ZM = Zambia
pv-country-ZW = Zimbabue
pv-country-SU = Unión Soviética
pv-country-DD = Alemania Oriental
pv-country-YU = Yugoslavia
pv-country-CS = Checoslovaquia
pv-country-OT = Imperio otomano
lang-aa = Afar
lang-ab = Abjasio
lang-ae = Avéstico
lang-af = Afrikáans
lang-ak = Akan
lang-am = Amárico
lang-an = Aragonés
lang-ar = Árabe
lang-as = Asamés
lang-av = Avar
lang-ay = Aimara
lang-az = Azerbaiyano
lang-ba = Baskir
lang-be = Bielorruso
lang-bg = Búlgaro
lang-bi = Bislama
lang-bm = Bambara
lang-bn = Bengalí
lang-bo = Tibetano
lang-br = Bretón
lang-bs = Bosnio
lang-ca = Catalán
lang-ce = Checheno
lang-ch = Chamorro
lang-co = Corso
lang-cr = Cree
lang-cs = Checo
lang-cu = Eslavo eclesiástico
lang-cv = Chuvasio
lang-cy = Galés
lang-da = Danés
lang-de = Alemán
lang-dv = Divehi
lang-dz = Dzongkha
lang-ee = Ewé
lang-el = Griego
lang-en = Inglés
lang-eo = Esperanto
lang-es = Español
lang-et = Estonio
lang-eu = Euskera
lang-fa = Persa
lang-ff = Fula
lang-fi = Finés
lang-fj = Fiyiano
lang-fo = Feroés
lang-fr = Francés
lang-fy = Frisón occidental
lang-ga = Irlandés
lang-gd = Gaélico escocés
lang-gl = Gallego
lang-gn = Guaraní
lang-gu = Guyaratí
lang-gv = Manés
lang-ha = Hausa
lang-he = Hebreo
lang-hi = Hindi
lang-ho = Hiri motu
lang-hr = Croata
lang-ht = Criollo haitiano
lang-hu = Húngaro
lang-hy = Armenio
lang-hz = Herero
lang-ia = Interlingua
lang-id = Indonesio
lang-ie = Interlingue
lang-ig = Igbo
lang-ii = Yi de Sichuán
lang-ik = Inupiaq
lang-io = Ido
lang-is = Islandés
lang-it = Italiano
lang-iu = Inuktitut
lang-ja = Japonés
lang-jv = Javanés
lang-ka = Georgiano
lang-kg = Kongo
lang-ki = Kikuyu
lang-kj = Kuanyama
lang-kk = Kazajo
lang-kl = Groenlandés
lang-km = Jemer
lang-kn = Canarés
lang-ko = Coreano
lang-kr = Kanuri
lang-ks = Cachemir
lang-ku = Kurdo
lang-kv = Komi
lang-kw = Córnico
lang-ky = Kirguís
lang-la = Latín
lang-lb = Luxemburgués
lang-lg = Ganda
lang-li = Limburgués
lang-ln = Lingala
lang-lo = Lao
lang-lt = Lituano
lang-lu = Luba-katanga
lang-lv = Letón
lang-mg = Malgache
lang-mh = Marshalés
lang-mi = Maorí
lang-mk = Macedonio
lang-ml = Malayálam
lang-mn = Mongol
lang-mr = Maratí
lang-ms = Malayo
lang-mt = Maltés
lang-my = Birmano
lang-na = Nauruano
lang-nb = Noruego bokmal
lang-nd = Ndebele septentrional
lang-ne = Nepalí
lang-ng = Ndonga
lang-nl = Neerlandés
lang-nn = Noruego nynorsk
lang-no = Noruego
lang-nr = Ndebele meridional
lang-nv = Navajo
lang-ny = Nyanja
lang-oc = Occitano
lang-oj = Ojibwa
lang-om = Oromo
lang-or = Oriya
lang-os = Osético
lang-pa = Punyabí
lang-pi = Pali
lang-pl = Polaco
lang-ps = Pastún
lang-pt = Portugués
lang-qu = Quechua
lang-rm = Romanche
lang-rn = Kirundi
lang-ro = Rumano
lang-ru = Ruso
lang-rw = Kinyarwanda
lang-sa = Sánscrito
lang-sc = Sardo
lang-sd = Sindhi
lang-se = Sami septentrional
lang-sg = Sango
lang-sh = Serbocroata
lang-si = Cingalés
lang-sk = Eslovaco
lang-sl = Esloveno
lang-sm = Samoano
lang-sn = Shona
lang-so = Somalí
lang-sq = Albanés
lang-sr = Serbio
lang-ss = Suazi
lang-st = Sotho meridional
lang-su = Sundanés
lang-sv = Sueco
lang-sw = Suajili
lang-ta = Tamil
lang-te = Telugu
lang-tg = Tayiko
lang-th = Tailandés
lang-ti = Tigriña
lang-tk = Turcomano
lang-tl = Tagalo
lang-tn = Setsuana
lang-to = Tongano
lang-tr = Turco
lang-ts = Tsonga
lang-tt = Tártaro
lang-tw = Twi
lang-ty = Tahitiano
lang-ug = Uigur
lang-uk = Ucraniano
lang-ur = Urdu
lang-uz = Uzbeko
lang-ve = Venda
lang-vi = Vietnamita
lang-vo = Volapük
lang-wa = Valón
lang-wo = Wólof
lang-xh = Xhosa
lang-yi = Yidis
lang-yo = Yoruba
lang-za = Zhuang
lang-zh = Chino
lang-zu = Zulú
person-tab-profile = Perfil
profile-groups-label = Apartados del perfil
profile-group-withheld = Parte de este apartado no está visible para usted
profile-withheld = Registrado para esta persona y no visible para usted: { $classes }.
profile-empty = Todavía no hay nada registrado en este apartado.
profile-earlier = formulario anterior
profile-earlier-title = Registrado por una versión anterior de esta aplicación, en un campo que AXGF 1.1 no contempla. Se conserva tal como se escribió.
profile-other-names = { $n ->
        [one] y otro nombre
       *[other] y otros { $n } nombres
    }
profile-edit-group = Editar «{ $group }»
profile-summary-link = { $n ->
        [one] Un dato en el perfil
       *[other] { $n } datos en el perfil
    }
profile-from = desde
profile-until = hasta
profile-yes = Sí
profile-no = No
profile-value = Valor
profile-editor-title = Perfil
profile-problems = Parte de lo introducido no se pudo guardar. Cada problema se indica junto a su campo, y no se ha escrito nada.
profile-editor-withheld = Este apartado también contiene, para esta persona, datos de la categoría { $classes } que usted no puede leer. No se muestran aquí, y guardar este formulario los deja tal como están.
profile-living-class-note = Esta persona consta como viva. Lo que introduzca aquí en una categoría sensible solo lo verán los administradores.
profile-relationships-elsewhere = Los padres, cónyuges, hijos, padrinos y testigos no se guardan en esta persona. Son familias, vínculos y eventos que la nombran; por eso cada cambio aquí modifica también la ficha de las demás personas implicadas.
profile-documents-first = Un artefacto remite a un documento vinculado a esta persona. Adjunte primero el archivo.
profile-editor-nothing = No hay nada en este apartado que usted pueda editar.
profile-new-entry = Nueva entrada
profile-provenance = Fecha, fuente y fiabilidad
profile-from-date = Válido desde
profile-until-date = Válido hasta
profile-remove-entry = Eliminar esta entrada
profile-add-entry = Añadir otra entrada
profile-no-such-group-title = No existe ese apartado
profile-no-such-group-detail = El perfil no tiene ningún apartado con ese nombre.
profile-error-number = Un valor de este campo debe ser un número.
profile-error-integer = Un valor de este campo debe ser un número entero.
profile-error-range = Un número está fuera del intervalo permitido para este atributo.
profile-error-term = Un valor no está entre las opciones disponibles.
profile-error-required = A una entrada le falta un campo obligatorio.
profile-error-one-of = Una entrada necesita al menos uno de sus campos principales.
profile-error-confidence = La fiabilidad va de 0 a 1, por ejemplo 0,8.
profile-error-time = Una hora se escribe en horas y minutos, por ejemplo 05:40.
profile-error-currency = Una moneda se escribe con su código de tres letras, por ejemplo EUR.
profile-error-language = Una lengua se escribe con su código, por ejemplo es o zh-Hans.
profile-error-coordinates = Las coordenadas necesitan una latitud entre −90 y 90 y una longitud entre −180 y 180.
profile-error-rank-country = El grado pertenece a un país distinto del elegido.
record-unknown-place = [Lugar desconocido]
record-missing-document = [Documento que falta]

## Interface

confidence-certain = Certeza { $percent } % — prácticamente seguro
confidence-high = Certeza { $percent } % — bien documentado
confidence-medium = Certeza { $percent } % — verosímil pero sin confirmar
confidence-low = Certeza { $percent } % — conjetura
tree-edge-union-between = { $from } y { $to } — { $confidence }
tree-edge-parentage-of = { $from }, progenitor de { $to } — { $confidence }
record-note-biography = Biografía
record-note-birth-date-as-recorded = Fecha de nacimiento, tal como consta
record-note-death-date-as-recorded = Fecha de defunción, tal como consta
record-note-event-date-as-recorded = Fecha de «{ $event }», tal como consta
record-unknown-source = [Fuente desconocida]
record-untitled-source = [Fuente sin título]
record-unnamed = [Sin nombre]
record-untitled = [Sin título]
record-period-from = desde { $date }
record-period-until = hasta { $date }
record-dates-unrecorded = fechas sin registrar
record-link-unlabelled = vinculado con
record-link-reverse = { $label } (de)
record-place-worked-as = Trabajó como { $title }
record-place-married-to = Matrimonio con { $name }
record-place-married = Matrimonio
record-source-use-name = el nombre «{ $name }»
record-source-use-working-as = el oficio de { $title }
record-source-use-union-with = la unión con { $name }
record-source-use-union = la unión
record-lifespan-born = n. { $year }
record-lifespan-died = f. { $year }
size-bytes = { $n ->
        [one] { $n } byte
       *[other] { $n } bytes
    }
size-kb = { $n } KB
size-mb = { $n } MB
size-gb = { $n } GB
calendar-gregorian = gregoriano
calendar-julian = juliano
calendar-hebrew = hebreo
calendar-hijri = islámico
calendar-persian = persa
calendar-chinese = chino
calendar-ethiopian = etíope
calendar-japanese_era = eras japonesas
calendar-republican_french = republicano francés
calendar-roman = romano
diff-summary-none = no cambió ningún campo
diff-summary-one = cambió { $a }
diff-summary-two = cambió { $a } y { $b }
diff-summary-many = cambió { $a }, { $b } y { $n ->
        [one] otro campo más
       *[other] { $n } campos más
    }
diff-saved-none = ningún campo cambiado
diff-saved-one = { $a } cambiado
diff-saved-two = { $a } y { $b } cambiados
diff-saved-many = { $a }, { $b } y { $n ->
        [one] otro campo más cambiados
       *[other] { $n } campos más cambiados
    }
history-created = creó
history-deleted = eliminó
history-attached = adjuntó un archivo
admin-raw-json-unparsed = No se pudo leer el JSON en bruto ({ $error }). No se ha guardado nada.
conflict-someone = Alguien
conflict-unrecorded-time = (hora no registrada)
dedup-merged-persons = { $n ->
        [one] una persona fusionada
       *[other] { $n } personas fusionadas
    }
dedup-merged-families = { $n ->
        [one] una familia fusionada
       *[other] { $n } familias fusionadas
    }
dedup-manual-review = { $n ->
        [one] un caso pendiente de revisión por una persona
       *[other] { $n } casos pendientes de revisión por una persona
    }
dedup-nothing = Nada que señalar.
validate-errors = { $n ->
        [one] un error
       *[other] { $n } errores
    }
validate-warnings = { $n ->
        [one] un aviso
       *[other] { $n } avisos
    }
validate-notes = { $n ->
        [one] una nota
       *[other] { $n } notas
    }
validate-nothing = Nada que señalar.
list-separator = { ", " }
result-written = El archivo se ha escrito en el disco.
result-refused = La biblioteca rechazó esta operación. El archivo en el disco no ha cambiado.
convert-error-no-file = No se ha subido ningún archivo. Elija primero un archivo .ged.
convert-error-file-too-large = Este archivo pesa { $size } MB y el límite es de { $limit } MB. No se ha convertido nada.
convert-error-too-large = La subida supera el límite de { $limit } MB. No se ha convertido nada.
convert-error-unreadable = No se pudo leer la subida ({ $error }). No se ha convertido nada.
convert-error-not-gedcom = No parece un archivo GEDCOM: un archivo GEDCOM 5.5.1 empieza con una línea «0 HEAD». No se ha convertido nada.
convert-error-packaging = El archivo se convirtió pero no se pudo empaquetar ({ $error }).
completeness-fraction = { $part } de { $whole }
event-category-adoption = Adopción
event-category-migration = Migración
event-category-naturalization = Naturalización
event-category-incarceration = Encarcelamiento
event-category-name_change = Cambio de nombre
event-category-legal = Asunto judicial
event-category-religious = Hecho religioso
event-category-social = Hecho social
event-category-historical = Hecho histórico
precision-quarter_century = al cuarto de siglo
source-type-birth_certificate = partida de nacimiento
source-type-death_certificate = partida de defunción
source-type-marriage_certificate = partida de matrimonio
source-type-census = padrón
source-type-baptism_record = partida de bautismo
source-type-burial_record = partida de entierro
source-type-will = testamento
source-type-land_record = registro de la propiedad
source-type-military_record = expediente militar
source-type-immigration_record = registro de inmigración
source-type-naturalization = expediente de naturalización
source-type-passport = pasaporte
source-type-photograph = fotografía
source-type-letter = carta
source-type-diary = diario
source-type-newspaper = periódico
source-type-oral_tradition = tradición oral
source-type-dna = prueba de ADN
source-type-family_bible = Biblia familiar
source-type-gravestone = lápida
source-type-published_genealogy = genealogía publicada
source-type-other = otra fuente
source-status-verified = cotejada con el original
source-status-unverified = aún sin cotejar
source-status-lost = perdida
source-status-known_missing = se sabe que falta
document-type-birth_certificate = partida de nacimiento
document-type-death_certificate = partida de defunción
document-type-marriage_certificate = partida de matrimonio
document-type-census_page = hoja del padrón
document-type-baptism_record = partida de bautismo
document-type-military_record = expediente militar
document-type-will = testamento
document-type-land_record = registro de la propiedad
document-type-diary = diario
document-type-newspaper_clipping = recorte de prensa
document-type-gravestone_photo = fotografía de una lápida
document-type-family_tree_drawing = árbol genealógico dibujado
document-type-audio = grabación de sonido
document-type-video = grabación de vídeo
document-status-present = guardado aquí
document-status-referenced = citado, guardado en otro lugar
document-status-known_missing = se sabe que falta
document-status-lost = perdido
document-status-unknown = paradero desconocido
diag-unsupported_spec_version = El archivo declara una versión de AXGF que este programa no sabe leer.
diag-invalid_json = Algo que debería ser JSON no se puede leer.
diag-invalid_bundle_structure = El archivo no está organizado como exige AXGF.
diag-schema_validation_failed = Un registro no se ajusta al esquema de AXGF.
diag-dangling_reference = Un registro remite a otro que no está en el archivo.
diag-duplicate_entity_id = Dos registros comparten el mismo identificador.
diag-duplicate_unique_ref = Dos registros reclaman la misma referencia, que debería ser única.
diag-cycle_detected = Los vínculos familiares dan la vuelta en círculo: alguien sería su propio antepasado.
diag-chronology_conflict = Las fechas se contradicen, por ejemplo un hijo nacido antes que su progenitor.
diag-out_of_vocabulary = Un valor no está entre los términos que admite su lista.
diag-claim_inconsistent = Una afirmación se contradice a sí misma o a otra afirmación sobre lo mismo.
diag-spec_version_mismatch = La versión de AXGF que declara un registro no encaja con lo que contiene.
diag-unknown_attribute = Un registro lleva un atributo que AXGF no define.
diag-entity_not_found = El registro que se quiere cambiar no está en el archivo.
diag-entity_already_exists = Ya existe un registro con este identificador.
diag-unknown_entity_kind = Este no es un tipo de registro que exista en AXGF.
diag-delete_blocked_by_reference = El registro no se puede eliminar mientras otros remitan a él.
diag-manual_review_required = Tiene que revisarlo una persona; no se ha cambiado automáticamente.
diag-zip_read_error = No se pudo leer el fichero del archivo.
diag-zip_write_error = No se pudo escribir el fichero del archivo.
diag-payloads_external = Los ficheros adjuntos se guardan fuera de los datos del archivo.
diag-payload_source_failed = No se pudo leer un fichero adjunto.
diag-payload_sink_failed = No se pudo escribir un fichero adjunto.
diag-gedcom_parse_error = No se pudo entender una línea del archivo GEDCOM.
diag-gedcom_unrecognized_tag = El archivo GEDCOM usa una etiqueta que la importación no conoce, así que esa entrada no se ha traído.
diag-internal = Algo ha fallado dentro de la biblioteca.
field-person-display-name = Nombre mostrado
field-person-display-name-hint = El nombre que se muestra en todo el sitio.
field-person-gender = Género
field-person-living = Con vida
field-person-birth-date = Fecha de nacimiento
field-date-value-hint = Un año, año y mes o una fecha completa: 1923, 1923-04 o 1923-04-12. Déjelo vacío si nadie la sabe.
field-person-birth-precision = Precisión del nacimiento
field-precision-hint = Con qué precisión lo establece la fuente.
field-person-birth-circa = Nacimiento aproximado
field-circa-hint = Se muestra como «hacia 1923» y no como afirmación exacta.
field-person-birth-place = Identificador del lugar de nacimiento
field-person-birth-confidence = Certeza del nacimiento
field-person-confidence-hint = Cuán seguro está. Es lo que dibuja el sitio.
field-person-death-date = Fecha de defunción
field-person-death-precision = Precisión de la defunción
field-person-death-circa = Defunción aproximada
field-person-death-place = Identificador del lugar de defunción
field-person-death-confidence = Certeza de la defunción
field-person-death-cause = Causa de la muerte
field-person-bio = Biografía
field-notes = Notas
field-family-name = Nombre de la familia
field-description = Descripción
field-family-union-type = Tipo de unión
field-family-union-status = Estado de la unión
field-family-union-confidence = Certeza de la unión
field-family-union-confidence-hint = Fija con qué intensidad se dibuja en el árbol la línea entre los miembros de la pareja.
field-family-union-start = Inicio de la unión
field-family-union-end = Fin de la unión
field-family-notes-hint = La pareja y los hijos son listas: edítelos en el JSON en bruto de abajo o en la página de relaciones de la persona.
field-category = Categoría
field-required-hint = Obligatorio.
field-event-subcategory = Subcategoría
field-date = Fecha
field-event-date-hint = La exige el esquema.
field-precision = Precisión
field-circa = Aproximada
field-place-id = Identificador del lugar
field-confidence = Certeza
field-source-id = Identificador de la fuente
field-link-from-type = Tipo de origen
field-link-from-id = Identificador de origen
field-link-to-type = Tipo de destino
field-link-to-id = Identificador de destino
field-link-label = Nombre del vínculo
field-link-label-hint = Se lee en el sentido del vínculo: «padrino», «patrón», «testigo». Obligatorio.
field-link-label-reverse = Nombre en sentido inverso
field-link-label-reverse-hint = Cómo se lee desde el otro extremo: «ahijado», «empleado».
field-link-bidirectional = Se lee igual en ambos sentidos
field-valid-from = Válido desde
field-link-valid-from-hint = Cuándo empezó la relación.
field-valid-until = Válido hasta
field-link-confidence-hint = «Seguro al 85 %, según una carta familiar»: lo que GEDCOM no puede decir.
field-note = Nota
field-occupation-person-id = Identificador de la persona
field-occupation-title = Oficio
field-occupation-title-hint = Obligatorio, por ejemplo Maestra.
field-occupation-title-latin = Oficio (alfabeto latino)
field-occupation-employer = Patrón
field-occupation-from = Desde
field-occupation-from-hint = Un oficio es un periodo. Dar sus dos extremos es lo que permite dibujarlo como una barra.
field-occupation-until = Hasta
field-source-title = Título
field-source-type = Tipo de fuente
field-source-reliability = Fiabilidad
field-source-reliability-hint = Obligatorio. Se muestra como distintivo junto a cada hecho que se apoya en esta fuente.
field-source-status = Estado de la fuente
field-source-repository = Lugar de custodia
field-source-repository-reference = Signatura en el lugar de custodia
field-source-transcription = Transcripción
field-place-name = Nombre principal
field-place-name-lang = Idioma del nombre
field-place-name-lang-hint = Un código de idioma, por ejemplo en, fr o pl.
field-place-type = Tipo de lugar
field-place-region = Región
field-place-country-current = País actual
field-place-country-current-hint = La historia de sus fronteras es una lista: edítela en el JSON en bruto de abajo.
field-document-filename = Nombre del fichero
field-document-mime-type = Tipo de contenido
field-document-mime-type-hint = Obligatorio, por ejemplo image/jpeg.
field-document-type = Tipo de documento
field-document-status = Estado del fichero
field-document-url = Dirección web
field-document-caption = Pie
lang-zh-Hans = Chino simplificado
family-lineage = Filiación
links-relation = Tipo de vínculo
occupations-position = Cargo
field-link-relation = Tipo de vínculo
field-link-relation-hint = Uno de los vínculos que nombra AXGF 1.1. El nombre de arriba conserva las palabras del registro.
field-occupation-position = Cargo
field-occupation-position-hint = El cargo dentro del oficio: Directora, cuando el oficio es Maestra.
error-delete-changed-title = Cambiado desde que lo miró
error-delete-changed-detail = Este registro se guardó de nuevo después de mostrarse la página desde la que lo elimina; ahora es la versión { $version }. No se ha eliminado nada. Mírelo tal como está antes de volver a decidir.
error-delete-changed-look = Volver a mirarlo
documents-files = Ficheros adjuntos aquí
documents-files-help = Edite los detalles de un fichero o elimine el fichero en sí. Eliminar quita el documento y sus bytes del archivo, para todas las personas a las que está adjunto; para quitarlo solo de esta persona, vacíe su fila de arriba.
documents-edit-details = Editar detalles
documents-delete = Eliminar este fichero

## Charts

radar-section = Gráficos del registro
radar-section-help = Tres lecturas de lo que contiene este registro, cada eje de 0 a 100. Cada número se calcula a partir de los hechos indicados al lado, según reglas escritas en la documentación de la aplicación; no se guarda nada ni se adivina nada, y un eje sin nada que leer queda vacío en lugar de recibir una puntuación intermedia. Las marcas y las barras dicen lo seguro que es cada número: un punto relleno es prácticamente seguro, un punto con aro está bien documentado, un aro es verosímil, un aro discontinuo es una conjetura, y cuanto más larga la barra, menos seguro.
radar-physique = Físico
radar-mind = Temperamento y mente
radar-vitality = Salud y vitalidad
radar-axis-stature = Estatura
radar-axis-build = Complexión
radar-axis-lean-mass = Masa magra
radar-axis-posture = Postura
radar-axis-gait = Andar
radar-axis-dentition = Dientes
radar-axis-openness = Apertura
radar-axis-conscientiousness = Responsabilidad
radar-axis-extraversion = Extraversión
radar-axis-agreeableness = Amabilidad
radar-axis-stability = Estabilidad emocional
radar-axis-cognition = Cognición
radar-axis-circulation = Circulación
radar-axis-breathing = Respiración
radar-axis-metabolism = Metabolismo
radar-axis-illness = Ausencia de enfermedad
radar-axis-senses = Sentidos
radar-axis-rest = Sueño y ánimo
radar-folded-open = Mostrar este gráfico
radar-folded-why = Esta persona consta como viva. El retrato del temperamento de una persona viva permanece plegado hasta que alguien que puede leerlo pide verlo.
radar-empty = Todavía no hay nada en este registro que pueda leerse en este gráfico.
radar-table-caption = { $chart }: cada eje, su puntuación y de qué se ha leído
radar-col-axis = Eje
radar-col-score = Puntuación
radar-col-from = Leído de
radar-no-score = sin puntuación
radar-from-none = nada
record-link-outgoing = de esta persona
record-link-incoming = hacia esta persona
