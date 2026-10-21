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
account-emergency = Token de emergencia
footer-credit = © 2026 · Diseñado y desarrollado por { $author }
footer-format-link = El formato AXGF en GitHub

## Preferencias

settings-title = Ajustes
edit-mode-label = Edición: lo que guarde cambia las fichas de la familia
display-menu-label = Idioma y presentación
display-settings = Tema y presentación…
settings-tabs-label = Secciones de los ajustes
settings-tab-theme = Tema
settings-tab-language = Idioma
settings-tab-appearance = Apariencia
settings-done = Listo
prefs-language = Idioma
prefs-theme = Tema
prefs-background = Fondo
prefs-background-on = Un velo suave de color detrás de la página
prefs-apply = Aplicar
busy-saving = Guardando…
busy-working = En curso…
prefs-reviewed = revisada
prefs-machine = automática, { $coverage } %
prefs-machine-complete = completa, aún sin revisar
prefs-machine-title = Traducida sin revisión por una persona de lengua materna. El vocabulario genealógico en particular puede estar equivocado: las palabras para una unión, un padrino o una fuente primaria cambian según la tradición archivística de cada país. Se agradecen las correcciones, y CONTRIBUTING.md dice por dónde empezar.

theme-canopee = Dosel
theme-canopee-note = oscuro, verde bosque · predeterminado
theme-fusion = Fusión
theme-fusion-note = oscuro, neón · un árbol vivo
theme-galaxy = Galaxia
theme-galaxy-note = espacio profundo · un sistema solar
theme-summit = Cumbre
theme-summit-note = oscuro, alba dorada · una ascensión

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
life-nothing-recorded = No consta ningún hecho, oficio ni lugar para esta persona. En un archivo convertido es lo corriente: GEDCOM lleva lo que alguien anotó, y la mayoría de las entradas son un nombre y una fecha.
life-add-first = Registrar el primer hecho
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
error-merged-title = Esta ficha ya no existe.
error-merged-detail = Se fusionó con { $name } el { $date }. Los enlaces a la dirección antigua llevan ahora a la ficha fusionada.
error-merged-open = Abrir la ficha fusionada
error-back-to-river = Volver al río
view-tree-back = Volver al árbol
view-galaxy-back = Volver a la galaxia
view-summit-back = Volver a la montaña
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
setup-title = Poner en marcha este archivo
setup-lede = Esta instalación aún no contiene ninguna familia. Elija cómo empieza. Aquí nunca se escribe nada encima de una familia: esta página se cierra en cuanto hay una.
setup-door-archive-title = A partir de un archivo AXGF
setup-door-archive-detail = Un fichero .axgf de otra instalación o de una copia de seguridad. Sus personas, fuentes y fotografías llegan tal como son.
setup-door-archive-file = Archivo (.axgf)
setup-door-archive-go = Poner en marcha con este archivo
setup-limit-gb = Hasta { $n } GB, escritos directamente en el disco.
setup-door-scratch-title = Desde cero
setup-door-scratch-detail = Un archivo vacío. Añada la primera persona y luego su familia, registro a registro.
setup-door-scratch-go = Añadir la primera persona
setup-door-gedcom-title = A partir de un fichero GEDCOM
setup-door-gedcom-detail = El fichero que exporta cualquier programa de genealogía: GEDCOM 5.5, 5.5.1, 5.5.5 o 7.0, en .ged o .gdz, con la codificación que sea. Verá cómo se leyó y qué pasa antes de que se guarde nada.
setup-door-gedcom-file = Fichero GEDCOM (.ged, .gdz)
setup-door-gedcom-go = Leer este fichero
setup-never-over = Para sustituir una familia ya presente, restaure una copia de seguridad desde la línea de órdenes (axgf-cms restore); ninguna página escribe encima.
setup-adopt-title = Hacer de esto la familia del archivo
setup-adopt-detail = Registros que pasarían a ser la familia de esta instalación: { $total }. No se guarda nada hasta que pulse el botón.
setup-adopt-go = Poner en marcha el archivo con esta familia
setup-adopt-back = Elegir otra entrada
setup-done-title = El archivo está en marcha
setup-done-detail = Registros en la familia de esta instalación ahora: { $total }.
setup-done-photos = Fotografías y documentos que llegaron con ella: { $n }.
setup-done-mismatches = Ficheros cuyo contenido no coincide con la huella registrada: { $n }. Se conservan; revíselos.
setup-done-river = Abrir el río
view-tree-open = Abrir el árbol
view-galaxy-open = Abrir la galaxia
view-summit-open = Abrir la montaña
setup-done-admin = Administración
setup-refused-title = Esto no pudo convertirse en el archivo
setup-refused-detail = No se escribió nada. Lo que lo impidió:
setup-closed-title = Este archivo ya está en marcha
setup-closed-detail = Contiene una familia, y ninguna página escribe encima. Para sustituirla, restaure una copia de seguridad desde la línea de órdenes (axgf-cms restore).
setup-not-archive = Esto no es un archivo .axgf.
setup-archive-too-large = Este archivo supera el tamaño permitido para un envío.
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
form-more-fields = { $n ->
        [one] Un campo más
        [many] { $n } campos más
       *[other] { $n } campos más
    }
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
person-tab-life = Eventos
person-tab-media = Medios
person-tab-tree = Árbol
person-tab-history = Historial
person-tree-depth = { $n } generaciones a cada lado. El árbol entero está más abajo.
person-tree-alone = Esta ficha no nombra padres, parejas ni hijos, así que no hay forma alguna que dibujar a su alrededor.

record-no-evidence = A esta ficha no hay nada adjunto: ni fuente ni documento. Es el estado corriente de un archivo convertido, no un defecto suyo: el GEDCOM se lleva los hechos y deja atrás lo que los probaba.
record-no-evidence-signed-out = Inicie sesión para adjuntar algo.
admin-delete = Borrar
admin-not-set = — sin establecer —
admin-edit = Editar
admin-page-of = Página { $page } de { $pages }
list-select-row = Seleccionar { $name }
list-kind-label = Tipo
list-select-page = Seleccionar todas las filas de esta página
list-selected = { $n ->
        [one] { $n } seleccionada
        [many] { $n } seleccionadas
       *[other] { $n } seleccionadas
    }
list-export = Exportar
list-escape = Esc para quitar la selección
bulk-confirm-title = { $n ->
        [one] ¿Eliminar { $n } ficha?
        [many] ¿Eliminar { $n } fichas?
       *[other] ¿Eliminar { $n } fichas?
    }
bulk-confirm-detail = Una ficha a la que otra todavía remite se rechaza, y entonces no se elimina nada.
bulk-confirm-go = Eliminarlas
bulk-deleted = { $n ->
        [one] { $n } ficha eliminada.
        [many] { $n } fichas eliminadas.
       *[other] { $n } fichas eliminadas.
    }
bulk-refused = No se eliminó nada: { $name } todavía se cita en otro lugar. Quite primero esa referencia y vuelva a intentarlo.
bulk-changed = { $n ->
        [one] { $n } ficha cambió desde que se mostró la lista, así que no se eliminó nada. Vuelva a cargar la lista e inténtelo de nuevo.
        [many] { $n } fichas cambiaron desde que se mostró la lista, así que no se eliminó nada. Vuelva a cargar la lista e inténtelo de nuevo.
       *[other] { $n } fichas cambiaron desde que se mostró la lista, así que no se eliminó nada. Vuelva a cargar la lista e inténtelo de nuevo.
    }
admin-range = { $from }–{ $to } de { $total }
list-filiation = Filiación
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
accounts-no-registration = Los visitantes no pueden abrir cuentas por sí mismos; el registro se fija arriba. Para la mayoría de las familias basta un administrador que conozca a todos.
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

home-empty = Aún no hay nada registrado.
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
home-nothing-admin = Esta instalación aún no contiene ninguna familia: póngala en marcha a partir de un archivo, de un fichero GEDCOM o desde cero.
home-nothing-visitor = Este archivo familiar se está preparando.
home-eyebrow = Archivo familiar
home-hero-1 = Cada vida es una fuente.
home-hero-2 = Cada familia, un río.
view-tree-hero-1 = Cada vida es una hoja.
view-tree-hero-2 = Cada familia, un árbol.
view-galaxy-hero-1 = Cada vida es una estrella.
view-galaxy-hero-2 = Cada familia, una galaxia.
view-summit-hero-1 = Cada vida es un paso.
view-summit-hero-2 = Cada familia, una ascensión.
home-cta-river = Explorar el río
view-tree-cta = Explorar el árbol
view-galaxy-cta = Explorar la galaxia
view-summit-cta = Subir la montaña
home-cta-tree = Ver el árbol
home-scroll = Desplácese para explorar
home-trust-title = Confianza en las filiaciones
home-trust-certain = prácticamente seguras
home-trust-high = bien fundamentadas
home-trust-medium = plausibles
home-trust-low = hipotéticas
home-trust-unrated = aún sin evaluar
home-showcase-eyebrow = Más allá de nombres y fechas
home-cta-title = Juntos, guardemos la memoria de la familia.
home-cta-detail = Cada nombre añadido y cada fuente comprobada hacen el río más fiel para quienes vendrán.
view-tree-cta-detail = Cada nombre añadido y cada fuente comprobada hacen el árbol más fiel para quienes vendrán.
view-galaxy-cta-detail = Cada nombre añadido y cada fuente comprobada hacen la galaxia más fiel para quienes vendrán.
view-summit-cta-detail = Cada nombre añadido y cada fuente verificada hacen la montaña más fiel para quienes vengan después.
home-cta-add = Añadir una persona

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
record-see-in-tree = Ver en el árbol
record-see-in-tree-title = Abrir la cuadrícula de fichas alrededor de esta persona
record-open-full-page = Abrir la página entera ↗
record-open-full-title = Abrir la página independiente que se puede compartir
record-edit = Editar
record-see-in-river = Ver en el río
view-tree-see = Ver en el árbol
view-galaxy-see = Ver en la galaxia
view-summit-see = Ver en la montaña
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
ai-badge = IA
ai-portraits-title = Retratos retocados por IA
ai-portraits-band = Retoques
ai-portraits-upload = Subir imágenes retocadas con IA
ai-portraits-manage = Gestionar las imágenes IA
ai-portraits-intro = Imágenes que una IA hizo a partir de las fotografías de esta persona: cómo podía ser. Aparecen junto a la cara en la ficha, marcadas «IA», y solo las ven los miembros con sesión iniciada. Nunca son la imagen de la persona, nunca están entre sus archivos, nunca son una prueba.
ai-portraits-current = En la ficha
ai-portraits-add = Añadir imágenes — sitio para { $n }
ai-portraits-files = Imágenes (PNG, JPEG, WebP o GIF), varias a la vez
ai-portraits-generator = Hechas con
ai-portraits-from = A partir de
ai-portraits-from-none = Sin indicar
ai-portraits-remove = Quitar
ai-portraits-alt = Retrato de { $name } retocado por una IA
ai-portraits-not-evidence = Hechas por una IA a partir de las fotografías: no son imágenes de la época.
ai-portraits-full = La ficha ya tiene sus tres imágenes IA. Quite una para añadir otra.
ai-portraits-too-many = Una ficha admite como máximo tres imágenes IA. Quite una antes de añadir otra.
ai-portraits-images-only = Un retrato IA tiene que ser una imagen: PNG, JPEG, WebP o GIF.
ai-portraits-stale = La ficha cambió mientras esta página estaba abierta. No se guardó nada: vuelva a abrirla y repita.
ai-portraits-back = Volver a las imágenes IA

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
identity-edit-link = Editar nombres e identidad

## Union types, statuses and date precision, said out loud

union-type-marriage = matrimonio
union-type-civil_union = unión civil
union-type-cohabitation = convivencia
union-type-religious_only = unión religiosa
union-type-polygamous = polígamo
union-type-unknown = sin registrar
union-role-spouse = cónyuge
union-role-partner = pareja
union-role-husband = marido
union-role-wife = esposa
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
profile-group-filled = { $n ->
        [one] { $n } relleno
        [many] { $n } rellenos
       *[other] { $n } rellenos
    }
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
diff-section-other = otro campo
diff-section-avatar = Foto de retrato
diff-section-partners = Pareja
diff-section-ai = Asistencia de IA
diff-section-birth = Nacimiento
diff-section-civil-status = Estado civil
diff-section-documents = Documentos adjuntos
diff-section-extensions = Datos de la aplicación
diff-section-children = Hijos
diff-section-union = Unión
diff-section-date = Fecha
diff-section-participants = Participantes
diff-section-from = Vinculado desde
diff-section-to = Vinculado a
diff-section-valid-from = Válido desde
diff-section-valid-until = Válido hasta
diff-section-visibility = Visibilidad
diff-section-employer = Empleador
diff-section-title-normalized = Título normalizado
diff-section-conflicts = Fuentes contradictorias
diff-section-dna = ADN
diff-section-document = Documento
diff-section-language = Idioma
diff-section-place = Lugar
diff-section-repository = Archivo depositario
diff-section-script = Escritura
diff-section-coordinates = Coordenadas
diff-section-country-history = Historial de países
diff-section-identifiers = Identificadores
diff-section-names = Nombres
diff-section-file = Fichero
diff-section-linked-to = Adjunto a
diff-section-ocr = Texto reconocido
diff-section-format-version = Versión del formato
history-created = creó
history-deleted = eliminó
history-merged = fusionó con otra ficha
history-setup = puso en marcha el archivo
history-attached = adjuntó un archivo
admin-raw-json-unparsed = No se pudo leer el JSON en bruto ({ $error }). No se ha guardado nada.
admin-raw-json-dangling = El JSON en bruto hace que { $field } apunte a { $target }, que no está en este archivo. Elíjalo con un campo de arriba o corrija el identificador. No se ha guardado nada.
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
record-union-duplicate = Una pareja, más de una ficha.
record-union-duplicate-detail = El paquete guarda fichas de Familia separadas para estas dos personas. Es un defecto de los datos, no una segunda unión.
record-union-duplicate-action = Fusionar duplicados
record-union-duplicate-confirm = ¿Deduplicar todo el paquete? Se fusionará cada par que la biblioteca pueda fusionar; el resto se informará.
dedup-pair-merged = El par en cuestión es ahora una sola ficha.
dedup-pair-refused = El par en cuestión no se fusionó: la biblioteca lo rechazó y lo dejó para que lo revise una persona.

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
gedcom-read-title = Cómo se leyó el archivo
gedcom-read-as = Leído como GEDCOM { $version }, en { $encoding }.
gedcom-read-as-unversioned = Leído como GEDCOM (su cabecera no indica versión), en { $encoding }.
gedcom-read-declared = Su cabecera dice { $declared }; sus bytes dicen otra cosa y se leyeron tal como son.
gedcom-read-gedzip = Un archivo GEDCOM 7 (.gdz): se leyó su conjunto de datos. Otros ficheros que contiene (fotografías, escaneos), no incorporados: { $n }.
gedcom-read-crs = Líneas terminadas al antiguo estilo Macintosh, reparadas: { $n }.
gedcom-read-voids = Referencias a @VOID@ — el «ningún registro» de GEDCOM 7 — apartadas en lugar de leerse como personas: { $n }.
gedcom-read-notes = Notas compartidas (SNOTE) leídas como notas: { $n }.
convert-error-no-dataset = Este zip no contiene gedcom.ged ni un único fichero .ged que leer en su lugar.
convert-error-refused = El conversor rechazó este fichero. Los diagnósticos de abajo dicen por qué. No se guardó nada.
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
radar-section-help = Tres lecturas de lo que contiene este registro, cada eje de 0 a 100. Cada número se calcula a partir de los hechos indicados al lado, según reglas escritas en la documentación de la aplicación; no se guarda nada ni se adivina nada, y un eje sin nada que leer queda vacío en lugar de recibir una puntuación intermedia.
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
# El aviso de funcionamiento en el panel de administración.
health-attention = Requiere atención:
health-standing-token = Sigue configurado un token de administración de emergencia. Entra por delante de cualquier cuenta y cualquier permiso — para eso sirve el día en que nadie puede iniciar sesión, y no para quedarse cuando alguien ya puede. Quítelo del archivo de entorno del servicio y reinícielo.
health-bundle-invalid = { $errors ->
        [one] Los datos familiares ya no pasan la validación: un error, indicado más abajo en «Validar».
        [many] Los datos familiares ya no pasan la validación: { $errors } errores, indicados más abajo en «Validar».
       *[other] Los datos familiares ya no pasan la validación: { $errors } errores, indicados más abajo en «Validar».
    }
health-disk-unknown = No se pudo leer el espacio libre de esta máquina, así que nada aquí puede prometer que el próximo guardado quepa.
health-disk-no-room-to-save = No se puede guardar: quedan { $free } libres y reconstruir este paquete necesita { $need }. No se ha perdido nada, y nada podrá cambiarse hasta que haya espacio.
health-disk-critical = El disco está libre al { $percent } % — quedan { $free }. Guardar dejará de funcionar en breve.
health-disk-low = El disco está libre al { $percent } % — quedan { $free }. Conviene atenderlo antes de que sea urgente.
health-backup-unconfigured = No se está respaldando nada. Un solo disco averiado se llevaría todos los registros.
health-backup-never = Hay un directorio de copias configurado, pero nunca se ha escrito ninguna copia en él.
health-backup-stale = { $days ->
        [one] La copia más reciente tiene un día. Debería escribirse una cada día.
        [many] La copia más reciente tiene { $days } días. Debería escribirse una cada día.
       *[other] La copia más reciente tiene { $days } días. Debería escribirse una cada día.
    }
health-cache-missing = { $missing ->
        [one] Falta uno de los { $declared } archivos adjuntos en la memoria intermedia y no podrá descargarse hasta que el próximo guardado lo reconstruya.
        [many] Faltan { $missing } de los { $declared } archivos adjuntos en la memoria intermedia y no podrán descargarse hasta que el próximo guardado los reconstruya.
       *[other] Faltan { $missing } de los { $declared } archivos adjuntos en la memoria intermedia y no podrán descargarse hasta que el próximo guardado los reconstruya.
    }

## River

nav-river = Río
nav-home = Resumen
river-aria = El río en torno a { $name }: ascendientes abajo, descendientes arriba.
river-band-living = VIVOS
river-circa = h.{" "}
river-trail = Recorrido
river-back = ← Atrás
river-range = Alcance
river-motion = Movimiento
river-travel = viaje
river-cut = corte
river-legend-attested = atestiguado
river-legend-inferred = deducido
river-legend-speculative = supuesto
river-legend-lost = fin del registro
river-legend-repeat = ×n alcanzado por n líneas, dibujado bajo cada una
river-legend-again = ↔ dibujado de nuevo junto a una pareja posterior
river-legend-discharge = descendientes
river-legend-direction = ↓ hacia la fuente · ↑ hacia los vivos
view-tree-direction = ↓ las raíces: los antepasados · ↑ la copa: los descendientes
view-galaxy-direction = ↓ los antepasados bajo el sol · ↑ los descendientes encima
view-summit-direction = ↓ los antepasados en el valle · ↑ los descendientes hacia la cumbre
river-legend-generation = una generación ≈ 29 años
river-zoom = Zoom
river-still = Detener el paisaje
view-tree-still = Detener el árbol
view-galaxy-still = Detener el cielo
view-summit-still = Detener la montaña
river-whole = Toda la familia
river-zoom-hint = Zoom: Ctrl + rueda, pellizcar o + / − · con zoom, arrastre para moverse
river-grid-link = Vista en cuadrícula
river-tab-sources = Fuentes
river-legend-title = Cómo leer el río
view-tree-name = Árbol vivo
view-tree-legend = Cómo leer el árbol
view-galaxy-name = Galaxia
view-galaxy-legend = Cómo leer la galaxia
view-summit-name = Ascensión
view-summit-legend = Cómo leer la montaña
hud-ancestors = Antepasados
hud-descendants = Descendientes
hud-generations = Generaciones
hud-living = Vivos
hud-attested = Vínculos comprobados

# Masthead search
search-label = Buscar una persona
search-short = Persona
date-qualifier = Calificador
date-qualifier-circa = hacia
search-title = Búsqueda
search-heading = Personas que coinciden con «{ $q }»
search-none = Nadie que puedas ver se llama así.
search-open-record = Abrir la ficha
search-hint = Intro lleva el río hasta esa persona; Mayús+Intro abre su ficha.
view-tree-search-hint = Intro hace crecer el árbol en torno a esa persona; Mayús+Intro abre su ficha.
view-galaxy-search-hint = Intro la convierte en el sol; Mayús+Intro abre su ficha.
view-summit-search-hint = Intro le lleva hasta esa persona; Mayús+Intro abre su ficha.
search-summary = { $n ->
        [one] { $n } persona
        [many] { $n } personas
       *[other] { $n } personas
    } · ↵ viajar · ⇧↵ abrir la ficha
search-empty-query = Escribe un nombre para buscar.

# Data quality (/quality)
nav-quality = Calidad de los datos
quality-title = Cosas que merecen una mirada
quality-none = Nada que mirar: todas las comprobaciones han pasado.
quality-intro = Aquí no se cambia nada por usted. Cada línea es un caso que merece una comprobación — a menudo hay una buena razón. Abra la ficha y decida, o márquelo como comprobado.
quality-groups-label = Tipos de hallazgo
quality-all = Todo
quality-show-label = Mostrar
quality-show-open-option = Por revisar
quality-download = Descargar
quality-group-names = Nombres
quality-group-relations = Parentescos
quality-group-dates = Fechas
quality-group-places = Lugares
quality-group-evidence = Pruebas
quality-export-csv = Descargar como CSV
quality-export-json = Descargar como JSON
quality-open = Abrir
quality-none-file = Nada que mirar en este archivo.
quality-show-list = Mostrar la lista
quality-after-import = No se ha cambiado nada en el archivo. Cuando sea el archivo del sitio, la página de calidad de los datos mostrará lo mismo con un enlace a cada ficha.
quality-rule-name-sentence = Una frase en un campo de nombre
quality-name-sentence = «{ $value }» parece más una frase que un nombre: si es una explicación, deje el nombre en el campo y pase el resto a una nota.
quality-rule-union-parent-child = Una pareja que también es padre e hijo
quality-union-parent-child = { $parent } y { $child } aparecen a la vez como pareja y como padre e hijo: conviene comprobar qué relación confirman las fuentes.
quality-rule-child-listed-twice = Un hijo anotado dos veces
quality-child-listed-twice = { $child } aparece dos veces entre los hijos de esta familia: si es el mismo hijo, una de las entradas puede quitarse.
quality-rule-duplicate-union = La misma pareja registrada dos veces
quality-duplicate-union = { $a } y { $b } tienen { $count } uniones registradas: si solo se casaron una vez, quizá sea una misma unión introducida dos veces.
quality-rule-partner-twice = Una persona a ambos lados de una unión
quality-partner-twice = { $person } aparece a ambos lados de esta unión: conviene comprobar quién era la otra pareja, o dejar ese lado vacío si nadie lo sabe.
quality-rule-foreign-surnames = Un hijo cuyo apellido no coincide con ninguno de los padres
quality-foreign-surnames = Un apellido que no coincide con el de ninguno de los padres: { $who }. Puede ser normal — un cambio de apellido, un hijastro, otra grafía —, pero merece una mirada. (Las hijas no se cuentan, porque el apellido de casada suele ser la razón.)
quality-rule-death-before-birth = Fallecimiento antes del nacimiento
quality-death-before-birth = { $person } tiene un fallecimiento en { $death }, antes de su nacimiento en { $birth }: quizá una de las fechas esté mal escrita; conviene comprobarlo en las fuentes.
quality-rule-child-before-parent = Un hijo nacido antes que uno de sus padres
quality-child-before-parent = { $child } (nacido en { $childborn }) parece haber nacido antes que { $parent } (nacido en { $parentborn }): conviene comprobar las fechas o la filiación.
quality-rule-parent-too-young = Un padre menor de doce años
quality-parent-too-young = { $parent } habría tenido { $age } años al nacer { $child }, lo cual es inusual: conviene comprobar las fechas o la filiación.
quality-rule-parent-too-old = Un padre mayor de sesenta y cinco años
quality-parent-too-old = { $parent } habría tenido { $age } años al nacer { $child }, lo cual es inusual: conviene comprobar las fechas o la filiación.
quality-rule-marriage-before-birth = Un matrimonio antes del nacimiento de un cónyuge
quality-marriage-before-birth = Este matrimonio está fechado en { $married }, antes del nacimiento de { $person } en { $born }: conviene comprobar la fecha del matrimonio.
quality-rule-isolated-record = Una ficha sin fecha ni parentescos
quality-isolated-record = { $person } no tiene ninguna fecha ni vínculo con nadie de la familia: conviene añadir cómo pertenece a ella, o borrar la ficha si se creó por error.
quality-rule-place-no-region = Un lugar sin región
quality-place-no-region = { $place } no tiene región: añadir una ayuda a distinguir los lugares con el mismo nombre.
quality-rule-region-free-text = Una región que parece una nota
quality-region-free-text = La región de { $place } dice «{ $value }», que parece más una nota que una región: conviene pasar la nota a la ficha que describe.
quality-rule-place-near-duplicate = Lugares que quizá sean el mismo
quality-place-near-duplicate = { $names } quizá sean el mismo lugar escrito de otra forma: si es así, pueden fusionarse en uno.
quality-rule-not-a-place = Una entrada que quizá no sea un lugar
quality-not-a-place = «{ $place }» está en la lista de lugares pero quizá no sea un lugar — empieza en minúscula, algo poco habitual en los nombres de lugar: conviene comprobar si es una nota sobre un lugar.
quality-rule-certain-without-source = Seguro, sin fuente
quality-certain-without-source = { $record } tiene un dato marcado como totalmente seguro ({ $claims }) sin fuente: conviene adjuntar la fuente o bajar la certeza.
quality-dismiss = Comprobado — normal en nuestra familia
quality-restore = Volver a mostrarlo
quality-note-label = Por qué (opcional)
quality-show-dismissed = Comprobados y conservados
quality-show-open = Volver a la lista abierta
quality-dismissed-by = Comprobado por { $by } el { $at }
quality-gone-title = Ese hallazgo ya no está
quality-gone-detail = La ficha ha cambiado desde que se abrió la lista y el hallazgo ya no es el mismo. Recargue la lista y vuelva a mirar.
quality-summary = { $n ->
    [one] { $n } cosa que merece una mirada
   *[other] { $n } cosas que merecen una mirada
}
quality-summary-file = { $n ->
    [one] { $n } cosa en este archivo merece una mirada
   *[other] { $n } cosas en este archivo merecen una mirada
}
quality-dismissed-summary = { $n ->
    [one] { $n } hallazgo comprobado y conservado
   *[other] { $n } hallazgos comprobados y conservados
}
quality-dismiss-all = Marcar todo como comprobado ({ $n })
quality-restore-all = Volver a mostrar todo ({ $n })
quality-bulk-conflict = Fichas cambiadas desde que se abrió la lista: { $n }. No se marcó nada; recargue la lista y vuelva a intentarlo.

## The record's editor (v0.4.2)

edit-tab-changed = modificado, aún sin guardar
edit-remove = Quitar
edit-add-partner = Añadir un cónyuge
edit-add-child = Añadir un hijo
edit-union-with = Unión con { $names }
edit-union-alone = Unión sin cónyuge registrado
edit-child-of = Hijo de { $names }
edit-new-family = Crear una familia
edit-cite-title = Lo que se apoya en una fuente
edit-attach-to = Citarla para
edit-add-source = Añadir una fuente
edit-error-source-title = Una fuente nueva necesita un título. No se ha guardado nada.
edit-detach-file = Quitar de esta ficha
edit-no-sources = Ningún dato propio de esta persona cita todavía una fuente.
edit-no-files = Todavía no hay ningún archivo adjunto a esta ficha.
edit-saved = { $n ->
        [one] Guardado: una ficha escrita.
        [many] Guardado: { $n } fichas escritas de una vez.
       *[other] Guardado: { $n } fichas escritas de una vez.
    }
edit-saved-none = Nada había cambiado, así que no se ha guardado nada.
edit-back-to-record = Volver a la ficha
edit-avatar = Foto de perfil
edit-avatar-frame = Encuadrar el rostro
edit-avatar-upload = Usar como foto de perfil
edit-stale = Otra persona ha modificado { $what } desde que se abrió esta página. No se ha guardado nada; todo lo que ha escrito sigue aquí.
edit-refused = No se ha guardado nada: { $reason }

# Administration console (v0.5.0, docs/adr/0011)
error-cross-site-title = Rechazado: enviado desde otro sitio
error-cross-site-detail = Este formulario venía de una página de otro sitio, así que no se ha tenido en cuenta. Si lo envió desde este sitio, recargue la página y vuelva a enviarlo.
admin-nav-label = Secciones de la administración
admin-nav-overview = Resumen
admin-nav-bundle = Archivo AXGF
admin-nav-infrastructure = Infraestructura
admin-nav-accounts = Cuentas
admin-nav-connectivity = Conectividad

# Administration console: the AXGF bundle (v0.5.0, docs/adr/0011)
bundle-title = Archivo AXGF
bundle-lede = El archivo de la familia tal como lo guarda este sitio: { $path }. A qué se ajusta, qué parece incorrecto y las dos maneras de llevarse una copia.
bundle-unset-title = Todavía no hay familia.
bundle-unset-detail = Esta instalación tiene un archivo vacío, así que aún no hay nada que comprobar. Póngala en marcha desde un archivo, un fichero GEDCOM o desde cero.
bundle-kpi-conformity = Conformidad
bundle-verdict-ok = Conforme
bundle-verdict-warn = A vigilar
bundle-verdict-fail = No conforme
findings-tally = En verde: { $ok } · Avisos: { $warn } · Críticos: { $fail }
bundle-kpi-entities = Entidades
bundle-kpi-entities-note = { $persons ->
    [one] 1 persona
    *[other] { $persons } personas
    } · { $files ->
    [one] 1 archivo adjunto
    *[other] { $files } archivos adjuntos
    }
bundle-kpi-size = Tamaño en disco
bundle-kpi-members = { $n ->
    [one] 1 fichero en el archivo
    *[other] { $n } ficheros en el archivo
    }
bundle-kpi-format = Formato
bundle-kpi-format-note = Esta versión lee y escribe AXGF { $current }.
bundle-conformity = Conformidad
bundle-conformity-intro = Nueve comprobaciones sobre el archivo tal como está ahora; ninguna lo modifica. Abra una línea para ver qué hacer.
bundle-area-format = Formato
bundle-area-integrity = Integridad
bundle-area-privacy = Privacidad
bundle-area-content = Contenido
level-ok = En verde
level-warn = Aviso
level-fail = Crítico
findings-what-to-do = Qué hacer
findings-go = Ir allí →
bundle-anomalies = Anomalías
bundle-anomalies-intro = Lo que encuentra la página de calidad de los datos, regla por regla: aún abierto y ya marcado como revisado.
bundle-anomalies-rule = Regla
bundle-anomalies-open = Abiertas
bundle-anomalies-checked = Revisadas
bundle-anomalies-none = La página de calidad de los datos no encuentra nada que revisar.
bundle-diagnostics = Diagnósticos del validador
bundle-facts = Características
bundle-fact-path = Fichero
bundle-fact-size = Tamaño
bundle-fact-spec = Formato
bundle-fact-schema = Esquema incluido
bundle-fact-none = ninguno
bundle-fact-created = Creado
bundle-fact-updated = Última escritura
bundle-fact-generator = Escrito por
bundle-fact-gedcom = Convertido desde
bundle-export = Exportar
bundle-export-raw = El archivo tal cual (.axgf)
bundle-export-raw-go = Descargar el archivo
bundle-export-sealed = Cifrado para una persona (.axgf.gpg)
bundle-export-sealed-intro = El mismo archivo, como mensaje OpenPGP que solo puede abrir quien tiene la clave privada. Pegue su clave pública: el texto que empieza por «-----BEGIN PGP PUBLIC KEY BLOCK-----».
bundle-export-sealed-go = Descargar cifrado
seal-recipient = Para
seal-fingerprint = Huella
seal-algorithm = Algoritmos
seal-fips-yes = dentro del conjunto aprobado por FIPS
seal-fips-no = fuera del conjunto aprobado por FIPS
seal-expires = Caducidad de la clave
seal-never = nunca
seal-key-label = La clave pública del destinatario
seal-key-hint = Una clave RSA de al menos 2048 bits, o una clave de curva elíptica. Compruebe la huella con su titular antes de enviar nada.
seal-remember = Guardar esta clave para la próxima vez
seal-check = Solo leer la clave
seal-fips-note = El archivo se cifra con AES-256 para la clave del destinatario. Los algoritmos aprobados por FIPS son RSA de 3072 bits o más y las curvas NIST P-256, P-384 y P-521: pida P-384. La biblioteca usada aquí no es en sí un módulo validado FIPS 140-3.
seal-decrypt-note = El destinatario lo abre con GnuPG:
seal-forget = Olvidar la clave guardada
seal-key-kept = La clave { $fingerprint } se guarda para la próxima exportación cifrada.
seal-error-no-key = Pegue primero la clave pública del destinatario.
seal-error-not-a-key = Eso no es una clave pública OpenPGP. Pegue el bloque entero, de su línea BEGIN a su línea END, y nunca una clave privada.
seal-error-bad-signatures = Las firmas propias de esa clave no se verifican: puede haber sido alterada. Pida una copia nueva a su titular.
seal-error-expired = Esa clave, o todas las suyas que pueden cifrar, han caducado. Pida a su titular que la prolongue o que envíe una nueva.
seal-error-revoked = Su titular ha revocado esa clave. Pídale la nueva.
seal-error-no-encryption-key = Esa clave puede firmar pero no cifrar: no tiene subclave de cifrado. Pida una a su titular (gpg --quick-add-key … encr).
seal-error-too-weak = Esa clave RSA tiene { $bits } bits; una exportación necesita al menos { $min }.
seal-error-unsupported = El algoritmo de esa clave no es de los que usa la exportación: solo claves RSA o de curva elíptica.
seal-error-io = No se pudo hacer la copia cifrada: { $error }
date-time-utc = { $date }, { $time } UTC

# Administration console: the bundle's conformity (v0.5.0, docs/adr/0011)
bundle-check-format-title = Versión del formato
bundle-check-format-ok = AXGF { $version }, la versión que esta compilación lee y escribe.
bundle-check-format-missing = El manifiesto no indica ninguna versión de AXGF.
bundle-check-format-missing-fix = El archivo no lo escribió una herramienta AXGF, o está dañado. Restaure la última copia de seguridad que validaba (axgf-cms restore).
bundle-check-format-older = Escrito en AXGF { $version }; el próximo guardado lo escribirá en AXGF { $current }.
bundle-check-format-older-fix = Nada que hacer: la próxima edición lo reescribe. Guarde una copia del fichero antiguo si otro programa solo lee la { $version }.
bundle-check-format-newer = Escrito en AXGF { $version }, más reciente que la { $current } de esta compilación: guardar podría perder lo que no entiende.
bundle-check-format-newer-fix = Actualice axgf-cms a una versión que lea AXGF { $version } antes de que nadie edite (Infraestructura, Versión).
bundle-check-manifest-title = Los recuentos del manifiesto
bundle-check-manifest-ok = Los recuentos del manifiesto coinciden con lo que contiene el archivo.
bundle-check-manifest-drift = El manifiesto declara { $declared } donde el archivo contiene { $actual } ({ $collection }).
bundle-check-manifest-fix = No se pierde nada: los recuentos se reescriben en el próximo guardado. Si el archivo viene de otro lugar, compruebe que se copió entero.
bundle-check-validation-title = Validación
bundle-check-validation-ok = El validador de la biblioteca no encuentra nada incorrecto.
bundle-check-validation-errors = { $errors ->
    [one] 1 error
    *[other] { $errors } errores
    }: el archivo no valida.
bundle-check-validation-errors-fix = Cada diagnóstico de abajo nombra el registro y el campo. Corrija el registro o restaure la última copia de seguridad que validaba.
bundle-check-validation-warnings = { $warnings ->
    [one] 1 aviso
    *[other] { $warnings } avisos
    }. Nunca bloquean un guardado; cada uno indica que algo puede estar mal.
bundle-check-validation-warnings-fix = Léalos abajo. La mayoría son referencias a registros que no están en el archivo, que una importación GEDCOM suele dejar.
bundle-check-payloads-title = Archivos adjuntos
bundle-check-payloads-none = Ningún archivo adjunto.
bundle-check-payloads-ok = { $declared ->
    [one] El archivo está presente.
    *[other] Los { $declared } archivos están presentes.
    }
bundle-check-payloads-missing = Faltan { $missing } de { $declared } archivos adjuntos en la caché.
bundle-check-payloads-fix = El próximo guardado reconstruye la caché a partir del archivo. Si faltan en el propio archivo, restáurelos desde una copia de seguridad.
bundle-check-binding-title = Cuentas y archivo
bundle-check-binding-ok = El fichero de cuentas se creó para esta familia.
bundle-check-binding-mismatch = El fichero de cuentas se creó para { $expected }; este archivo es { $found }.
bundle-check-binding-fix = Si el archivo se restauró o se renombró a propósito, nada que hacer. Si no, las cuentas de una familia se aplican al árbol de otra: vuelva a poner el .acl correcto a su lado.
bundle-check-living-title = Los vivos, en público
bundle-check-living-ok = Nadie registrado como vivo es visible para los visitantes.
bundle-check-living-public = { $n ->
    [one] 1 persona registrada como viva está marcada
    *[other] { $n } personas registradas como vivas están marcadas
    } como públicas: cualquier visitante lee su ficha.
bundle-check-living-fix = Salvo que lo hayan aceptado, ponga su visibilidad en miembros (Editar, Identidad).
bundle-check-special-title = Categorías especiales
bundle-check-special-none = Ningún registro contiene datos de salud, biométricos, genómicos, penales o de comportamiento.
bundle-check-special-some = { $n ->
    [one] 1 registro contiene
    *[other] { $n } registros contienen
    } datos de salud, biométricos, genómicos, penales o de comportamiento. De los vivos solo los ven los administradores, diga lo que diga el registro.
bundle-check-quality-title = Calidad de los datos
bundle-check-quality-ok = Nada que revisar.
bundle-check-quality-open = { $n ->
    [one] 1 cosa que revisar
    *[other] { $n } cosas que revisar
    }, en { $groups ->
    [one] 1 grupo
    *[other] { $groups } grupos
    }.
bundle-check-quality-fix = Abra la página de calidad de los datos: cada línea nombra el registro y lo que parece mal. Corríjalo o márquelo como revisado.
bundle-check-size-title = Tamaño
bundle-check-size-ok = { $size }, por debajo de los { $warn } a partir de los que esta página avisa.
bundle-check-size-heavy = { $size }, por encima de { $warn }.
bundle-check-size-fix = El archivo es de la familia y puede crecer. Vigile el disco y las copias de seguridad (Infraestructura, Volúmenes) y suba --size-warn-mb si este tamaño es el esperado.
seal-algorithm-value = { $key } · AES-256
seal-key-placeholder = -----BEGIN PGP PUBLIC KEY BLOCK-----

# Administration console: the infrastructure's checks (v0.5.0, docs/adr/0011 §3)
infra-area-security = Seguridad
infra-area-access = Acceso
infra-area-data = Datos
infra-area-capacity = Capacidad
infra-area-release = Versión
infra-area-platform = Plataforma
infra-verdict-ok = En buen estado
infra-verdict-warn = Requiere atención
infra-verdict-fail = Crítico
duration-days = { $n ->
    [one] { $n } día
    [many] { $n } días
    *[other] { $n } días
    }
duration-hours = { $n ->
    [one] { $n } hora
    [many] { $n } horas
    *[other] { $n } horas
    }
duration-minutes = { $n ->
    [one] { $n } minuto
    [many] { $n } minutos
    *[other] { $n } minutos
    }
infra-transport-title = Cómo se llega al sitio
infra-transport-ok = Esta página llegó por HTTPS.
infra-transport-exposed = Esta página llegó sin cifrar, y el servidor escucha en { $bind }, más allá de esta máquina: contraseñas y cookies de sesión cruzan la red en claro.
infra-transport-exposed-fix = Ponga delante un proxy inverso HTTPS (Caddy, nginx, HAProxy) y vuelva a ligar el servicio a 127.0.0.1: ejecute de nuevo bootstrap.sh con --bind 127.0.0.1:8080, o quite el drop-in que lo amplió.
infra-transport-plain = Esta página llegó sin cifrar, de un servidor que escucha en { $bind }. Eso vale en esta máquina sola, y solo ahí.
infra-transport-plain-fix = Acceda al sitio por su dirección HTTPS. Si un proxy termina el TLS, haga que envíe X-Forwarded-Proto: https para que este servidor lo sepa.
infra-forwarded-title = Direcciones reenviadas
infra-forwarded-ok = El servidor solo escucha en esta máquina, así que la dirección del cliente que reenvía un proxy es fiable para frenar los inicios de sesión fallidos repetidos.
infra-forwarded-exposed = El servidor es accesible directamente y confía en la cabecera X-Forwarded-For: un cliente que la ponga él mismo esquiva el freno tras inicios de sesión fallidos.
infra-forwarded-fix = Ligue el servicio a 127.0.0.1 detrás del proxy, o filtre el puerto para que solo el proxy llegue a él.
infra-cross-site-title = Envíos desde otros sitios
infra-cross-site-ok = Ningún formulario se envió aquí desde otro sitio en el último día.
infra-cross-site-refused = { $n ->
    [one] En el último día se rechazó un formulario enviado desde otro sitio.
    [many] En el último día se rechazaron { $n } formularios enviados desde otros sitios.
    *[other] En el último día se rechazaron { $n } formularios enviados desde otros sitios.
    }
infra-cross-site-fix = No pasó nada. Si los rechazos son suyos —el mismo sitio por una segunda dirección—, fije la dirección pública en Conectividad o use solo esa.
infra-sso-title = Servicios de inicio de sesión
infra-sso-none = No hay ningún servicio de inicio de sesión activado; las cuentas entran con contraseña.
infra-sso-ok = { $n ->
    [one] Hay un servicio de inicio de sesión activado, y la dirección pública es HTTPS.
    [many] Hay { $n } servicios de inicio de sesión activados, y la dirección pública es HTTPS.
    *[other] Hay { $n } servicios de inicio de sesión activados, y la dirección pública es HTTPS.
    }
infra-sso-no-https = Hay un servicio de inicio de sesión activado, pero la dirección pública no es HTTPS: los proveedores se niegan a enviar su respuesta a una dirección sin cifrar, y el inicio de sesión falla.
infra-sso-fix = En Conectividad, ponga como dirección pública la dirección https:// que usan los lectores.
infra-admins-title = Administradores
infra-admins-none = Ninguna cuenta activa puede administrar este sitio. Solo puede el token de emergencia, y solo mientras esté definido.
infra-admins-none-fix = Entre con el token de emergencia, cree un administrador en Cuentas y luego quite el token.
infra-admins-one = Un solo administrador. Si esa persona no está o pierde la contraseña, nadie puede gestionar el sitio.
infra-admins-one-fix = Dé el rol de administrador a una segunda persona de confianza, en Cuentas.
infra-admins-ok = { $n ->
    [one] { $n } administrador puede entrar.
    [many] { $n } administradores pueden entrar.
    *[other] { $n } administradores pueden entrar.
    }
infra-token-title = Token de emergencia
infra-token-generated = No hay ningún token de emergencia definido.
infra-token-only-way = Hay un token de emergencia definido y, sin cuenta de administrador, es la única entrada. Consérvelo hasta que exista un administrador.
infra-token-standing = Sigue definido un token de emergencia. Entra por encima de toda cuenta y todo permiso.
infra-token-fix = Quite AXGF_CMS_ADMIN_TOKEN del archivo de entorno del servicio y reinicie el servicio.
infra-registration-title = Registro de visitantes
infra-registration-closed = Los visitantes no pueden crear cuentas; solo un administrador puede.
infra-registration-secret = Los visitantes pueden crear una cuenta de miembro si conocen el secreto de registro.
infra-registration-open = Cualquier visitante puede crear una cuenta de miembro.
infra-registration-open-fix = Eso sirve para un sitio pensado para ser abierto. Si no, exija el secreto de registro, o cierre el registro, en Cuentas.
infra-failures-title = Inicios de sesión fallidos
infra-failures-ok = { $n ->
    [one] { $n } inicio de sesión fallido en el último día.
    [many] { $n } inicios de sesión fallidos en el último día.
    *[other] { $n } inicios de sesión fallidos en el último día.
    }
infra-failures-many = { $n ->
    [one] { $n } inicio de sesión fallido en el último día: puede que alguien esté adivinando contraseñas.
    [many] { $n } inicios de sesión fallidos en el último día: puede que alguien esté adivinando contraseñas.
    *[other] { $n } inicios de sesión fallidos en el último día: puede que alguien esté adivinando contraseñas.
    }
infra-failures-fix = Cada dirección ya se frena tras fallos repetidos. Si la cifra sigue subiendo, busque en el registro del proxy de dónde vienen los intentos y bloquéelos allí, y asegúrese de que cada administrador tenga una contraseña larga o entre por un servicio de inicio de sesión.
infra-health-bundle-title = Datos de la familia
infra-health-bundle-ok = El archivo se abre y supera la validación.
infra-health-bundle-fix = Abra la página Archivo AXGF para ver los errores y corregirlos, o restaure la última copia de seguridad que validaba (axgf-cms restore).
infra-health-cache-title = Caché de adjuntos
infra-health-cache-ok = Cada archivo adjunto está en la caché.
infra-health-cache-fix = Guarde cualquier ficha una vez: el guardado reconstruye la caché. Los archivos que sigan faltando después faltan en el propio archivo.
infra-health-backup-title = Copias de seguridad
infra-health-backup-ok = { $count ->
    [one] Existe una copia reciente; se conserva { $count } archivo.
    [many] Existe una copia reciente; se conservan { $count } archivos.
    *[other] Existe una copia reciente; se conservan { $count } archivos.
    }
infra-health-backup-fix = Sin directorio de copias, ejecute de nuevo bootstrap.sh con --backup-dir. Con uno, revise el temporizador (systemctl status axgf-cms-backup.timer) y su registro (journalctl -u axgf-cms-backup).
infra-disk-title = Espacio en disco
infra-disk-ok = { $free } libres de { $total } ({ $percent } % libre).
infra-disk-fix = Libere espacio en la partición que contiene el directorio de datos —copias antiguas guardadas también en otro sitio, registros, archivos de otros programas— o traslade los datos a un disco mayor.
infra-growth-title = Crecimiento
infra-growth-ok = { $days ->
    [one] Crece unos { $rate } al mes; hay sitio para { $days } día más a este ritmo.
    [many] Crece unos { $rate } al mes; hay sitio para { $days } días más a este ritmo.
    *[other] Crece unos { $rate } al mes; hay sitio para { $days } días más a este ritmo.
    }
infra-growth-soon = { $days ->
    [one] Crece unos { $rate } al mes: a este ritmo el disco se llena en { $days } día.
    [many] Crece unos { $rate } al mes: a este ritmo el disco se llena en { $days } días.
    *[other] Crece unos { $rate } al mes: a este ritmo el disco se llena en { $days } días.
    }
infra-growth-fix = Planifique el espacio ya: un disco mayor, menos copias en el mismo disco, o las copias en otro.
infra-growth-flat = Las copias no han crecido; no hay nada que planificar.
infra-growth-unknown = Aún no hay copias suficientes para medir el crecimiento: hace falta al menos un día entre la más antigua y la más reciente.
infra-memory-title = Memoria
infra-memory-ok = { $used } usados del límite de { $max } del servicio ({ $percent } %).
infra-memory-high = { $used } usados del límite de { $max } del servicio ({ $percent } %): cerca de donde el sistema detiene el servicio.
infra-memory-fix = Suba MemoryMax= en un drop-in para axgf-cms.service, o busque qué retiene la memoria —una importación muy grande, un adjunto grande— y reinicie el servicio.
infra-memory-unlimited = { $used } en uso, sin límite fijado para el servicio.
infra-version-title = Versión
infra-version-ok = v{ $current } es la versión publicada más reciente.
infra-version-behind = Se ejecuta v{ $current }; { $latest } está publicada.
infra-version-behind-fix = Lea las notas de la versión y actualice desde la sección Versión de abajo. Primero se hace una copia, y la versión anterior vuelve sola si la nueva no arranca.
infra-version-unknown = No se sabe si existe una versión más reciente: la lista de versiones no se ha obtenido.
infra-version-unknown-fix = Use «Buscar versiones nuevas» abajo. El aislamiento de producción no puede llegar a GitHub por sí mismo; una vez instalado el asistente de actualización, él obtiene la lista cada día.
infra-version-stale = { $days ->
    [one] La lista de versiones se obtuvo por última vez hace { $days } día.
    [many] La lista de versiones se obtuvo por última vez hace { $days } días.
    *[other] La lista de versiones se obtuvo por última vez hace { $days } días.
    }
infra-upgrades-title = Actualizaciones
infra-upgrades-ok = El asistente de actualización está instalado: el botón de abajo puede cambiar de versión.
infra-upgrades-manual = El asistente de actualización no está instalado, así que la versión se cambia a mano en el servidor.
infra-upgrades-fix = Ejecute una vez el bootstrap.sh actual como root: instala el asistente junto con el servicio.
infra-sandbox-title = Aislamiento
infra-sandbox-ok = El proceso se ejecuta con no-new-privileges y un filtro de llamadas al sistema.
infra-sandbox-unknown = Este sistema no informa del aislamiento del proceso.
infra-sandbox-off = El proceso se ejecuta sin parte de su aislamiento: no-new-privileges o el filtro de llamadas al sistema está desactivado.
infra-sandbox-fix = Ejecute el servicio desde la unidad que escribe bootstrap.sh, sin drop-ins que la relajen. En una máquina de desarrollo es lo esperado.
infra-clock-title = Reloj
infra-clock-ok = El reloj del sistema marca { $year }, lo cual es plausible.
infra-clock-wrong = El reloj del sistema marca { $year }: fechas, antigüedad de las copias y sesiones son todas erróneas.
infra-clock-fix = Active la sincronización horaria: timedatectl set-ntp true.

# Administration console: the Infrastructure page (v0.5.0, docs/adr/0011)
infra-title = Infraestructura
infra-lede = La máquina en la que funciona este sitio, comprobada cada vez que se abre esta página: cómo se llega a ella, quién tiene las llaves, si los datos están a salvo y aún caben, qué versión se ejecuta y el aislamiento que la rodea.
infra-kpi-health = Estado
infra-kpi-version = Versión
infra-kpi-version-newer = { $version } disponible
infra-kpi-version-current = Al día
infra-kpi-version-unknown = Aún sin comprobar
infra-kpi-disk = Espacio libre
infra-kpi-disk-note = { $percent } % de { $total } usado
infra-kpi-compute = Máquina
infra-kpi-cpus = { $n ->
    [one] { $n } núcleo
    [many] { $n } núcleos
    *[other] { $n } núcleos
    }
infra-kpi-load = Carga { $load }
infra-kpi-rss = { $size } en uso
infra-unknown = Desconocido
infra-health = Estado
infra-health-intro = Verde está bien, un aviso merece un vistazo, crítico requiere atención ya. Cada hallazgo se abre en lo que hay que hacer.
infra-json = Las mismas comprobaciones en JSON, para una monitorización
infra-version = Versión
infra-version-running = En ejecución
infra-version-latest = Última publicada
infra-version-up-to-date = al día
infra-version-checked = Lista obtenida
infra-version-never = nunca
infra-source-direct = por este servidor
infra-source-helper = por el asistente de actualización
infra-version-token = Repositorio privado
infra-token-env-set = Hay un token de GitHub definido para listar y descargar versiones.
infra-token-env-unset = Sin token. Si el repositorio es privado, ponga uno en { $name }, legible solo por root: el asistente de actualización lo lee ahí.
infra-releases-error = El último intento de obtener la lista falló:
infra-releases-refresh = Buscar versiones nuevas
infra-releases-asked = Se ha pedido al asistente de actualización una lista nueva. Recargue esta página en un minuto.
infra-releases-not-asked = No se pudo escribir la solicitud: { $error }
infra-upgrade-status-running = Cambiando de { $from } a { $to }…
infra-upgrade-status-succeeded = Cambiada de { $from } a { $to }.
infra-upgrade-status-failed = El cambio de { $from } a { $to } falló; sigue ejecutándose { $from }.
infra-upgrade-status-refused = El asistente de actualización rechazó el cambio a { $to }.
infra-upgrade-log = Salida del asistente
infra-upgrade-pending = { $by } pidió { $version } el { $at }; el asistente de actualización aún no lo ha recogido.
infra-releases = Versiones publicadas
infra-releases-tag = Versión
infra-releases-published = Publicada
infra-releases-relation = Respecto a esta
infra-releases-notes = Notas de la versión
infra-releases-open = Leer
infra-relation-newer = más reciente
infra-relation-current = esta
infra-relation-older = más antigua
infra-prerelease = versión candidata
infra-releases-none = Todavía no hay lista de versiones.
infra-upgrade = Cambiar de versión
infra-upgrade-intro = El botón se lo pide al asistente de actualización, que se ejecuta como root fuera del aislamiento de este sitio. Hace una copia, instala la versión tras verificar su suma de control, reinicia el servicio y lo comprueba; si la versión nueva no responde, se repone la anterior.
infra-upgrade-version = Versión que instalar
infra-upgrade-confirm = Para volver a una versión anterior, escriba de nuevo su etiqueta
infra-upgrade-confirm-hint = No hace falta para una versión más reciente. Una versión anterior puede no leer datos escritos por una más reciente: lea antes sus notas.
infra-upgrade-go = Instalar esta versión
infra-upgrade-safety = El sitio no está disponible unos segundos mientras el servicio se reinicia.
infra-upgrade-manual = El asistente de actualización no está instalado en este servidor, así que esta página no puede cambiar de versión. Como root, en el servidor:
infra-upgrade-manual-helper = Ejecutar una vez el bootstrap.sh actual instala el asistente, y desde entonces el botón funciona aquí.
infra-upgrade-asked = Se ha pedido { $version }. El asistente de actualización lo recoge en pocos segundos; esta página muestra cómo ha ido.
infra-upgrade-no-helper = El asistente de actualización no está instalado, así que { $version } no se puede instalar desde aquí.
infra-upgrade-unknown-version = { $version } no está en la lista de versiones publicadas.
infra-upgrade-same-version = { $version } es la versión que ya se ejecuta.
infra-upgrade-confirm-older = { $version } es anterior a la versión en ejecución. Escriba su etiqueta en el campo de confirmación para volver a ella.
infra-volumes = Almacenamiento
infra-volumes-intro = La partición que contiene el directorio de datos ({ $path }) y lo que este sitio guarda en ella.
infra-volumes-partition = Partición
infra-of = { $used } de { $total }
infra-volumes-ours = Este sitio
infra-volumes-breakdown = Lo que guarda este sitio
infra-part-bundle = Archivo de la familia
infra-part-cache = Caché de adjuntos
infra-part-backups = Copias de seguridad
infra-part-binary = Programa
infra-part-journal = Diario de cambios
infra-part-accounts = Cuentas y ajustes
infra-files = { $n ->
    [one] { $n } archivo
    [many] { $n } archivos
    *[other] { $n } archivos
    }
infra-growth = Crecimiento
infra-growth-month = Al mes
infra-growth-not-enough = no hay copias suficientes para saberlo
infra-growth-left = Espacio restante a este ritmo
infra-backups = Copias de seguridad
infra-backups-dir = Directorio
infra-backups-count = Archivos
infra-backups-newest = La más reciente
infra-backups-oldest = La más antigua
infra-backups-room = Espacio donde se guardan
infra-free-of = { $free } libres de { $total }
infra-paths = Rutas
infra-path-data = Directorio de datos
infra-path-binary = Programa
infra-compute = Cómputo
infra-compute-hidden = El aislamiento del servicio le oculta las cifras de la propia máquina; lo que se muestra aquí es lo que el proceso puede ver.
infra-compute-memory = Memoria de la máquina
infra-available-of = { $available } libres de { $total }
infra-compute-ceiling = Memoria del servicio
infra-compute-no-ceiling = { $used }, sin límite
infra-compute-os = Sistema
infra-compute-kernel = Núcleo del sistema
infra-compute-arch = Arquitectura
infra-compute-cpu = Procesador
infra-compute-cpus = Núcleos
infra-compute-load = Carga (1, 5, 15 min)
infra-compute-rss = Este proceso
infra-compute-threads = Hilos
infra-compute-files = Archivos abiertos
infra-compute-uptime = Servicio en marcha desde hace
infra-compute-sys-uptime = Máquina en marcha desde hace
infra-compute-sandbox = Aislamiento
infra-compute-nnp-on = sin privilegios nuevos
infra-compute-nnp-off = se pueden obtener privilegios
infra-compute-seccomp-on = llamadas al sistema filtradas
infra-compute-seccomp-off = llamadas al sistema sin filtrar
infra-activity = Actividad
infra-activity-intro = Contado sobre { $span }: el servicio guarda un día y vuelve a cero cada vez que se reinicia.
infra-activity-sessions = Sesiones abiertas
infra-activity-signed-in = Inicios de sesión
infra-activity-failed = Inicios de sesión fallidos
infra-activity-registered = Cuentas creadas
infra-activity-cross-site = Envíos de otros sitios rechazados

# Sign-in services: what the reader is told when one fails (v0.5.0, docs/adr/0012)
sso-error-not-configured = Este servicio de inicio de sesión no está configurado en este sitio.
sso-error-no-public-url = Entrar mediante un servicio requiere la dirección pública de este sitio, que aún no está definida. Un administrador la fija en Conectividad.
sso-error-unreachable = No se pudo contactar con el servicio de inicio de sesión. Inténtelo de nuevo en un momento o entre de otra forma.
sso-error-refused = El servicio de inicio de sesión no le dejó entrar, o el inicio de sesión se canceló allí.
sso-error-stale = Ese inicio de sesión había caducado o se empezó en otro navegador. Vuelva a empezar.
sso-error-invalid = La respuesta del servicio de inicio de sesión no superó las comprobaciones de este sitio, así que no se usó. Vuelva a empezar; si se repite, avise a un administrador.
infra-upgrade-already-pending = { $version } ya está esperando al asistente de actualización. Espere a que termine antes de pedir otra versión.

# Administration console: Connectivity, sign-in services and registration (v0.5.0, docs/adr/0012)
connectivity-title = Conectividad
connectivity-lede = Cómo llegan los lectores a este sitio y cómo entran: la dirección pública, los servicios de inicio de sesión y las reglas que sigue cada inicio de sesión, incluidos los de los administradores.
connectivity-kpi-address = Dirección pública
connectivity-address-unset = Sin definir
connectivity-address-https = HTTPS
connectivity-address-plain = Sin cifrar
connectivity-address-unset-note = Todo servicio de inicio de sesión la necesita
connectivity-kpi-services = Servicios de inicio de sesión
connectivity-kpi-services-note = { $n ->
    [one] listo para ofrecer
    [many] listos para ofrecer
    *[other] listos para ofrecer
    }
connectivity-kpi-passwords = Contraseñas
connectivity-policy-short-everyone = Todos
connectivity-policy-short-non-admins = Salvo administradores
connectivity-policy-short-nobody = Nadie
connectivity-policy-note-everyone = Cada cuenta puede entrar con su contraseña, y también con un servicio vinculado.
connectivity-policy-note-non-admins = Los administradores solo entran mediante un servicio; los demás pueden seguir usando contraseña. Una contraseña de administrador robada ya no abre nada.
connectivity-policy-note-nobody = Todos entran mediante un servicio. Una cuenta sin servicio vinculado ni dirección con la que vincularlo no puede entrar.
connectivity-kpi-admins-linked = { $n ->
    [one] { $n } administrador con un servicio vinculado
    [many] { $n } administradores con un servicio vinculado
    *[other] { $n } administradores con un servicio vinculado
    }
connectivity-kpi-registration = Registro
connectivity-kpi-registration-go = Se fija en Cuentas →
registration-short-closed = Solo administradores
registration-short-secret = Con el secreto de la familia
registration-short-open = Abierto a visitantes
connectivity-address = Dirección pública
connectivity-address-intro = La dirección que escriben los lectores, tal como la escriben: https://family.example. Los servicios de inicio de sesión devuelven a los lectores a ella, así que debe ser la dirección pública, no aquella en la que escucha este servidor.
connectivity-address-label = Dirección pública
connectivity-address-hint = Déjela vacía mientras nadie entre mediante un servicio.
connectivity-address-invalid = No es una dirección http o https con nombre de host.
connectivity-address-saved = La dirección pública está guardada.
connectivity-policy = Reglas de inicio de sesión
connectivity-policy-intro = Se aplican a todas las cuentas, incluidas las de administrador. Se rechaza una regla que dejaría a ningún administrador capaz de entrar.
connectivity-policy-passwords = Quién puede entrar con contraseña
connectivity-policy-without-way = { $n ->
    [one] { $n } cuenta activa no tiene servicio vinculado ni dirección: con «Nadie» no podría entrar.
    [many] { $n } cuentas activas no tienen servicio vinculado ni dirección: con «Nadie» no podrían entrar.
    *[other] { $n } cuentas activas no tienen servicio vinculado ni dirección: con «Nadie» no podrían entrar.
    }
connectivity-link-by-email = Vincular un servicio a la cuenta con la misma dirección verificada
connectivity-link-by-email-note = Así obtiene un familiar un inicio de sesión de un clic sin registro: cree su cuenta con su dirección, y su primer inicio de sesión mediante un servicio que haya verificado esa dirección la vincula. Solo se compara una dirección escrita por un administrador o verificada por un servicio.
connectivity-domains = Dominios autorizados a abrir una cuenta
connectivity-domains-hint = Uno por línea, como family.example. Vacío: cualquiera. Si se definen, solo una dirección verificada por un servicio en uno de estos dominios puede abrir una cuenta, y no se ofrece el formulario de registro.
connectivity-domain-invalid = «{ $domain }» no es un nombre de dominio.
connectivity-policy-unknown = Elija quién puede entrar con contraseña.
connectivity-policy-lockout = Rechazado: con este cambio ningún administrador activo podría entrar. Vincule primero un servicio de inicio de sesión a la cuenta de un administrador, entrando una vez con él, o mantenga las contraseñas para los administradores.
connectivity-policy-saved = Las reglas de inicio de sesión están guardadas.
connectivity-not-saved = No guardado: { $error }
connectivity-services = Servicios de inicio de sesión
connectivity-services-intro = Registre este sitio en la consola del servicio con la dirección de redirección indicada y pegue aquí el ID de cliente y el secreto. Los visitantes verán entonces «Continuar con …» en las páginas de inicio de sesión y de registro.
connectivity-outbound-note = Entrar mediante un servicio exige que este servidor llegue al servicio. La unidad que escribe bootstrap.sh rechaza las conexiones salientes (IPAddressDeny=any): permítalas con sudo systemctl edit axgf-cms añadiendo IPAddressAllow=any bajo [Service], y reinicie. El botón de prueba dice si lo consigue.
connectivity-service-ready = Listo
connectivity-service-incomplete = Incompleto
connectivity-service-off = Desactivado
connectivity-service-enable = Ofrecer este servicio en las páginas de inicio de sesión y registro
connectivity-service-issuer = Emisor
connectivity-service-issuer-hint = La dirección del proveedor, como https://auth.family.example/realms/family; su configuración se lee de /.well-known/openid-configuration bajo ella.
connectivity-service-label = Texto del botón
connectivity-service-tenant = Directorio (inquilino)
connectivity-service-tenant-hint = common para cualquier cuenta de Microsoft, organizations para cuentas profesionales y educativas, consumers para las personales, o el ID de su directorio.
connectivity-service-client-id = ID de cliente
connectivity-service-secret = Secreto de cliente
connectivity-service-secret-set = Hay un secreto definido. Deje esto vacío para conservarlo.
connectivity-service-secret-unset = Aún no hay secreto definido.
connectivity-service-secret-clear = Quitar el secreto
connectivity-service-redirect = Dirección de redirección que registrar
connectivity-service-redirect-needs-address = Defina primero la dirección pública
connectivity-service-console = Dónde registrarlo
connectivity-service-console-open = Consola de { $name }
connectivity-service-test = Probar la conexión
connectivity-issuer-invalid = El emisor debe ser una dirección https://.
connectivity-tenant-invalid = Un directorio es un nombre o un ID: letras, cifras, puntos y guiones.
connectivity-provider-incomplete = { $name } no está completo: necesita un ID de cliente y un secreto de cliente, y un emisor si es un proveedor genérico.
connectivity-provider-saved = { $name } está guardado.
connectivity-test-ok = { $name } respondió: { $answer }.
connectivity-test-failed = No se pudo llegar a { $name } desde este servidor: { $detail }
registration-title = Registro
registration-intro = Si un visitante puede abrir una cuenta aquí por sí mismo. Esa cuenta es de miembro: lee lo que leen los miembros y no cambia nada. Darle otro rol lo decide un administrador, más abajo.
registration-who = Quién puede abrir una cuenta
registration-note-closed = Solo un administrador crea cuentas, en esta página.
registration-note-secret = Un visitante que conozca el secreto de la familia puede abrir una cuenta de miembro. Comparta el secreto como compartiría el código de una puerta.
registration-note-open = Cualquier visitante puede abrir una cuenta de miembro. Los miembros leen las fichas reservadas a los miembros.
registration-rotate = Generar un secreto nuevo
registration-secret = El secreto de la familia
registration-secret-note = Quien lo tenga puede abrir una cuenta mientras este modo esté activo. Generar uno nuevo deja fuera el anterior; las cuentas ya abiertas se mantienen.
registration-link = Página de registro
registration-quick = Accesos rápidos
registration-quick-none = Ningún servicio de inicio de sesión está listo: configure uno en Conectividad
registration-changed = Último cambio por { $by }, { $at }.
registration-unknown = Elija quién puede abrir una cuenta.
accounts-kpi-total = Cuentas activas
accounts-kpi-total-note = { $disabled ->
    [one] { $disabled } desactivada
    [many] { $disabled } desactivadas
    *[other] { $disabled } desactivadas
    }
accounts-kpi-admins = Administradores
accounts-kpi-admins-few = Añada un segundo
accounts-kpi-admins-ok = Más de uno, como debe ser
accounts-kpi-linked = Vinculadas a un servicio
accounts-kpi-linked-note = { $n ->
    [one] { $n } servicio listo
    [many] { $n } servicios listos
    *[other] { $n } servicios listos
    }
accounts-kpi-registered = { $n ->
    [one] { $n } abierta por un visitante
    [many] { $n } abiertas por visitantes
    *[other] { $n } abiertas por visitantes
    }
accounts-roles = Roles
accounts-sign-in = Entra con
accounts-sign-in-password = contraseña
accounts-sign-in-google = Google
accounts-sign-in-microsoft = Microsoft
accounts-sign-in-github = GitHub
accounts-sign-in-oidc = inicio de sesión único
accounts-sign-in-none = aún sin forma de entrar
accounts-created-via-register = abierta por el visitante
accounts-created-via-sso = abierta mediante un servicio
accounts-add-also-register = Los visitantes también pueden abrir cuentas de miembro por sí mismos, como se fijó arriba. Una cuenta creada aquí puede tener cualquier rol.
login-password-not-allowed = Las reglas de inicio de sesión de este sitio no permiten a esta cuenta usar contraseña: entre mediante un servicio.
login-register = ¿Aún sin cuenta? Cree una
register-title = Crear una cuenta
register-lede = Una cuenta de miembro lee las fichas reservadas a los miembros de la familia. Lo más rápido es una cuenta que ya tenga.
register-closed = Aquí las cuentas las crea un administrador. Pídale la suya.
register-have-account = ¿Ya tiene cuenta? Entre
register-or = o
register-secret = El secreto de la familia
register-secret-hint = Se lo dio un administrador, o alguien de la familia.
register-secret-hint-services = Escríbalo antes de elegir un servicio, o el paso siguiente se lo pedirá.
register-domains = Solo las direcciones de { $domains } pueden abrir una cuenta.
register-username-hint = De 2 a 64 letras, cifras, puntos, guiones o guiones bajos.
register-submit = Crear la cuenta
register-member-note = No se pide ninguna dirección: no se le envía nada, y nadie puede apropiarse de la suya.
register-form-closed = Este sitio no abre cuentas con este formulario.
register-throttled = Demasiados intentos desde aquí. Espere un cuarto de hora y vuelva a intentarlo.
register-secret-wrong = Ese no es el secreto de la familia.
register-username-invalid = Un nombre de usuario tiene de 2 a 64 letras, cifras, puntos, guiones o guiones bajos.
register-password-short = La contraseña necesita al menos { $min } caracteres.
register-username-taken = Ese nombre de usuario está ocupado. Elija otro.
register-not-saved = No se pudo guardar la cuenta. Inténtelo de nuevo en un momento.
register-finish-title = Un paso más
register-finish-lede = El servicio confirmó quién es usted. Para abrir una cuenta aquí, escriba el secreto de la familia.
register-finish-lede-email = El servicio le confirmó como { $email }. Para abrir una cuenta aquí, escriba el secreto de la familia.
sso-continue = Continuar
sso-continue-with = Continuar con { $name }
sso-error-title = El inicio de sesión no se completó
sso-account-disabled = La cuenta vinculada a este inicio de sesión está desactivada. Consulte a un administrador.
sso-no-account = Ninguna cuenta de aquí corresponde a este inicio de sesión, y este sitio no abre cuentas por sí solo. Pida a un administrador que cree la suya con esta dirección: su próximo inicio de sesión se vinculará a ella.
sso-domain-refused = Aquí solo se pueden abrir cuentas con una dirección de los dominios de la familia, confirmada por el servicio.

# Administration console: the overview's tiles (v0.5.0, docs/adr/0011)
overview-accounts-note = { $admins ->
    [one] { $admins } administrador
    [many] { $admins } administradores
    *[other] { $admins } administradores
    }
register-password-hint = Al menos { $min } caracteres. Una frase de varias palabras se recuerda mejor que un revoltijo de signos.

# Clarity: the light theme and its funnel (v0.5.0, docs/adr/0013)
theme-clarity = Claridad
theme-clarity-note = claro, blanco y naranja · un embudo
view-funnel-back = Volver al embudo
view-funnel-open = Abrir el embudo
view-funnel-hero-1 = Cada generación es un anillo.
view-funnel-hero-2 = Cada familia, un embudo.
view-funnel-cta = Explorar el embudo
view-funnel-cta-detail = Cada nombre añadido y cada fuente comprobada hacen el embudo más fiel para quienes vengan después.
view-funnel-see = Ver en el embudo
view-funnel-direction = ↓ los más antiguos en el extremo estrecho · ↑ los más recientes en la boca
view-funnel-still = Detener el embudo
view-funnel-name = Embudo
view-funnel-legend = Cómo leer el embudo
view-funnel-search-hint = Intro los trae al frente; Mayús+Intro abre su ficha.
