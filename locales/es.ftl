# axgf-cms — textos de la interfaz, español.
#
# CALIDAD AUTOMÁTICA — no revisado por una persona hispanohablante nativa. El
# vocabulario genealógico en particular («union», «affiliation», «confidence»)
# tiene equivalentes establecidos que varían según la tradición archivística.
# Se agradecen las correcciones — véase CONTRIBUTING.md.
#
# REGLA: este archivo traduce solo la interfaz. Los nombres, lugares, notas y
# oficios provienen del archivo .axgf y permanecen en su propia lengua y
# escritura.

app-name = axgf-cms

nav-tree = Árbol
nav-admin = Administración
nav-sign-in = Iniciar sesión
nav-sign-out = Cerrar sesión

prefs-title = Idioma y apariencia
prefs-language = Idioma
prefs-language-note = Esto cambia solo la interfaz. Los nombres, lugares y notas se muestran siempre en su propia lengua y escritura.
prefs-theme = Apariencia
prefs-apply = Aplicar
prefs-reviewed = revisado
prefs-machine = automático, { $coverage } %

theme-light = Claro
theme-dark = Oscuro
theme-system = Seguir mi sistema
theme-high-contrast = Alto contraste
theme-sepia = Sepia
theme-deuteranopia = Deuteranopía
theme-protanopia = Protanopía
theme-tritanopia = Tritanopía
theme-colour-blind-note = seguro para daltonismo
theme-contrast-note = contraste máximo

tree-title-around = En torno a { $name }
tree-title-whole = El árbol completo
tree-lede-focused = { $ancestors ->
        [one] Un ascendiente
       *[other] { $ancestors } ascendientes
    }, { $descendants ->
        [one] un descendiente
       *[other] { $descendants } descendientes
    } y { $spouses ->
        [one] una pareja
       *[other] { $spouses } parejas
    }, { $depth } generaciones en cada sentido. Los mayores abajo. La opacidad de las líneas indica la certeza de la relación: una línea tenue es una afirmación de la que el registro no está seguro.
tree-lede-whole = Todas las personas del archivo. Los mayores abajo, los más jóvenes arriba. La opacidad de las líneas indica la certeza de la relación.
tree-filter-label = Filtrar las fichas visibles
tree-filter-placeholder = Escriba un nombre…
tree-centre-on = Centrar en
tree-depth = Generaciones en cada sentido
tree-show = Mostrar
tree-hidden-notice = { $n ->
        [one] Se muestra una persona sin sus datos
       *[other] Se muestran { $n } personas sin sus datos
    }
tree-hidden-because-role = , porque su visibilidad supera lo que su cuenta puede leer.
tree-hidden-because-anonymous = , porque no son públicas.
tree-hidden-sign-in = Inicie sesión si tiene una cuenta.
tree-restricted-card = La ficha de esta persona no es visible para usted
tree-empty = Este archivo no contiene a nadie que dibujar.
tree-unplaced = En ninguna familia registrada

record-identity = Identidad
record-life-events = Hechos vitales
record-family = Familia
record-other-relationships = Otras relaciones
record-occupations = Oficios
record-places = Lugares
record-sources-documents = Fuentes y documentos
record-notes = Notas
record-history = Historial
record-raw = Entidad en bruto
record-raw-summary-note = el JSON con el que se construyó esta página
record-sources-documents-help = Cada fuente nombra los hechos de esta página que se apoyan en ella, ordenadas según la solidez de la prueba.
record-notes-help = Notas sobre esta ficha, incluido el texto que ningún conversor supo interpretar y que se conservó literalmente en lugar de descartarse.
record-help-toggle = Lo que muestra esta sección

record-gender = Género
record-living = Con vida
record-visibility = Visibilidad
record-yes = sí
record-no = no
record-name-type = Tipo de nombre
record-name-used = Usado
record-name-evidence = Prueba
record-transliteration = Transliteración latina
record-born = Nacido/a
record-died = Fallecido/a
record-parents = Padres
record-siblings = Hermanos
record-children = Hijos
record-unknown-person = [Desconocido]
record-restricted-person = Privado
record-restricted-title = La ficha de esta persona no es visible para usted
record-absent-person-title = Mencionado por este archivo pero ausente de él
record-confidence = Certeza
record-source = Fuente
record-download = Descargar

access-restricted-title = No visible para usted
access-restricted-anonymous = Esta ficha no es pública. Inicie sesión para ver si su cuenta puede leerla.
access-role-title = No para su papel
access-role-write = Su cuenta puede leer este archivo pero no modificarlo. Un administrador puede elevar su papel a colaborador.
access-scope-title = Fuera de su rama

error-not-found-title = No encontrado
error-not-found-detail = Esa página no existe en este archivo.
error-no-such-person-title = No existe esa persona
error-no-such-person-detail = Este archivo no contiene ninguna persona con ese identificador.
error-no-such-entity-title = No existe esa entidad
error-no-such-entity-detail = Este archivo no contiene ninguna entidad con ese identificador.
error-deleted-while-editing = Este archivo no contiene ninguna entidad con ese identificador. Puede haberse eliminado mientras la editaba.
error-no-such-file-title = No existe ese archivo
error-not-an-image-title = No es una imagen
error-not-an-image-detail = No hay miniatura para este documento, porque no es una imagen que esta versión sepa descodificar.
error-back = Volver

login-title = Iniciar sesión
login-lede = Las cuentas las crea un administrador.
login-username = Nombre de usuario
login-password = Contraseña
login-submit = Iniciar sesión
login-wrong = Ese nombre de usuario y esa contraseña no coinciden.
login-token-wrong = Ese testigo no es correcto.
login-throttled = Demasiados intentos fallidos. Espere unos minutos e inténtelo de nuevo.
login-no-accounts-title = Esta instalación todavía no tiene cuentas.
login-emergency-summary = Acceso de emergencia
login-emergency-label = Testigo de emergencia
login-emergency-submit = Usar el testigo de emergencia
login-sign-in-prompt = Inicie sesión para acceder al panel de administración.

admin-title = Administración
admin-entities = Entidades
admin-create = Crear
admin-new-kind = Nuevo: { $kind }
admin-operations = Operaciones
admin-validate = Validar
admin-deduplicate = Eliminar duplicados
admin-export = Exportar el archivo
admin-accounts = Cuentas
admin-dedup-confirm = La eliminación de duplicados fusiona entidades y reescribe el archivo. ¿Continuar?
admin-recent-changes = Cambios recientes
admin-sessions-open = { $n ->
        [one] Una sesión abierta ahora mismo.
       *[other] { $n } sesiones abiertas ahora mismo.
    }
admin-no-changes-yet = Todavía no se ha cambiado nada mediante esta aplicación. Cada guardado a partir de ahora se registra en { $path }.
admin-last-validation = Última validación
admin-fields = Campos
admin-raw-json = JSON en bruto
admin-save = Guardar
admin-cancel = Cancelar
admin-delete = Eliminar
admin-not-set = — sin definir —
admin-edit = Editar
admin-page-of = Página { $page } de { $pages }
admin-previous = Anterior
admin-next = Siguiente
admin-saved = Guardado como versión { $version } — { $summary }
admin-not-saved = No guardado
admin-created = Creado
admin-not-created = No creado
admin-deleted = Eliminado
admin-not-deleted = No eliminado — el archivo queda igual
admin-what-changed = qué cambió
admin-field = Campo
admin-from = De
admin-to = A
admin-version = versión { $version }

accounts-title = Cuentas
accounts-existing = Existentes
accounts-username = Nombre de usuario
accounts-role = Papel
accounts-status = Estado
accounts-branch = Rama
accounts-last-seen = Última visita
accounts-change = Cambiar
accounts-you = (usted)
accounts-active = activa
accounts-disabled = desactivada
accounts-never = nunca
accounts-whole-tree = árbol completo
accounts-roots = { $n ->
        [one] una raíz
       *[other] { $n } raíces
    }
accounts-add = Añadir una cuenta
accounts-password-hint = Déjelo en blanco y se generará una que se mostrará una sola vez. Al menos { $min } caracteres si la define usted.
accounts-new-password-placeholder = contraseña nueva (en blanco = sin cambio)
accounts-email = Correo electrónico
accounts-optional = (opcional)
accounts-create = Crear la cuenta
accounts-role-viewer = lector — lee las fichas públicas y de miembros
accounts-role-contributor = colaborador — además crea, edita y sube archivos
accounts-role-admin = administrador — además gestiona cuentas, elimina y exporta
accounts-branch-placeholder = un identificador de persona por línea
accounts-ids-in-bundle = Identificadores de personas en este archivo
accounts-created = { $username } creada.
accounts-updated = { $username } actualizada. Se ha cerrado cualquier sesión que tuviera abierta.
accounts-username-taken = Ese nombre de usuario ya está ocupado.
accounts-pick-role = Elija un papel.
accounts-no-such = No existe esa cuenta.
accounts-not-saved = No guardado: { $error }

conflict-title = Otra persona lo cambió antes que usted
conflict-versions = Usted partió de la versión { $expected }; el archivo contiene ahora la versión { $current }.
conflict-both-changed = Ambos han cambiado esto
conflict-both-changed-detail = Estos campos fueron editados por los dos. Lo que guarde sustituirá lo que { $who } puso ahí:
conflict-field-by-field = Campo por campo
conflict-theirs = A qué lo cambió { $who }
conflict-yours = A qué lo cambió usted
conflict-unchanged-by-you = sin cambios por su parte
conflict-unchanged-by-them = sin cambios por su parte
conflict-what-now = Y ahora
conflict-reapply = Reaplicar su versión sobre la de ellos
conflict-save-over = Guardar esto sobre la suya
conflict-discard = Descartar la mía y empezar de nuevo
conflict-their-version = La versión de { $who }, tal como el archivo la contiene ahora
conflict-history-of = Historial de esta entidad ({ $kind })

## Dates

date-unknown = Fecha desconocida
date-not-recorded = No registrada
date-circa = hacia { $date }
date-between = entre { $from } y { $to }
date-before = antes de { $date }
date-after = después de { $date }
date-preserved = registrado como «{ $text }»
date-day-month-year = { $day } de { $month } de { $year }
date-month-year = { $month } de { $year }
date-decade = los años { $decade }
date-century = el siglo { $century }

month-1 = enero
month-2 = febrero
month-3 = marzo
month-4 = abril
month-5 = mayo
month-6 = junio
month-7 = julio
month-8 = agosto
month-9 = septiembre
month-10 = octubre
month-11 = noviembre
month-12 = diciembre
