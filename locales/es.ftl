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

## Resultados de las operaciones

result-diagnostics = Avisos
result-diagnostics-note = Todos los avisos que devolvió la biblioteca, incluidas las advertencias que no detuvieron la operación. No se filtra ninguno.
result-no-diagnostics = La biblioteca no devolvió ningún aviso.
result-continue = Seguir
result-dashboard = Panel
person-sections-label = Secciones de esta página

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
record-doc-photo = foto
record-doc-certificate = acta
record-doc-letter = carta
record-doc-record = registro
record-doc-newspaper = periódico
record-doc-other = otro
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
