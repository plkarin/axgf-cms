El AFG no se limita a responder: también puede llevar el árbol. Pídale con sus palabras qué hay que añadir o corregir. Propone los cambios en una tarjeta, y nada cambia antes de que pulse **Aplicar**. Cada capítulo describe un tipo de trabajo, con peticiones para usar tal cual o adaptar.

## 1. Antes de empezar

- **El archivo debe acompañar al proyecto.** En los ajustes del proyecto, marque «Dar al AFG el archivo de la familia en este proyecto». Sin él, el AFG no conoce las fichas ni sus identificadores, y se lo dirá.
- **Su papel decide lo que es posible.** Una cuenta de *lectura* puede preguntar al AFG, no modificar. Un *colaborador* modifica las fichas de su rama. Un *administrador* también puede eliminar a una persona.
- **La tarjeta «Cambios propuestos»** enumera con palabras sencillas lo que se hará. **Aplicar** lo ejecuta; **Descartar** lo deja de lado. Todo lo que contiene se aplica junto, o nada.
- **Todo queda registrado.** Cada cambio entra en el historial con su nombre, seguido de «AFG».

> ¿Puedes modificar el árbol desde este proyecto? ¿Qué me permite hacer mi cuenta?

## 2. Buscar y comprobar antes de modificar

Pida primero al AFG que compruebe lo que contiene el archivo: se corrige mejor sabiendo lo que ya hay.

> ¿Quiénes son los hijos de Pierre Dupont, nacido en 1890? Dame los identificadores de sus fichas.

> ¿Podría Marie Durand tener una ficha duplicada? Compara las fechas y los lugares.

> ¿Qué fichas de la rama Dupont no tienen ni fecha ni lugar de nacimiento?

## 3. Añadir personas

Diga quién es la persona y a quién se vincula. Un padre, una madre, un hermano, una pareja o un hijo nombrado con un nombre que nadie lleva todavía se crea al mismo tiempo.

> Añade a Jeanne Dupont, nacida el 3 de marzo de 1923 en Lyon, hija de Pierre Dupont y de Marie Durand.

> Añade los tres hijos de Jean Martin y Rose Petit: Paul (1950), Louise (1952) y André (hacia 1955), todos nacidos en Nantes.

> Añade a mi bisabuelo Joseph Ngoma, nacido hacia 1880 en el Congo y fallecido en 1952 en Kinsasa; su hijo es Albert Ngoma, que ya está en el árbol.

## 4. Corregir una ficha

Nombres, apellido, apodos, género, biografía, visibilidad: nombre la ficha y la corrección.

> El nombre de Jean Dupont (nacido en 1921) es en realidad Jean-Baptiste. Corrígelo.

> Añade el apodo «Abuela Lou» a Louise Martin.

> Redacta una breve biografía de Paul Martin a partir de lo que el archivo dice de él, y propónmela.

> La ficha de Rose Petit debe ser visible solo para los miembros, no para el público.

## 5. Nacimientos y fallecimientos

Una fecha se escribe 1890, 1890-06 o 1890-06-14; diga «hacia» cuando sea aproximada. Un lugar que el archivo no conoce se crea.

> Pierre Dupont nació hacia 1890 en Saint-Étienne.

> Marie Durand falleció el 12 de enero de 1975 en Lyon.

> La fecha de nacimiento de Louise Martin está mal: es el 2 de febrero de 1952, no 1950.

> Quita el lugar de fallecimiento de André Martin: no es seguro.

## 6. Uniones e hijos

Una unión reúne a uno o dos miembros, con su tipo (matrimonio, unión civil, convivencia, unión religiosa, polígama, desconocido), y la fecha y el lugar del matrimonio si lo hubo.

> Pierre Dupont y Marie Durand se casaron el 1 de mayo de 1920 en Lyon.

> Paul Martin también es hijo de Jean Martin y Rose Petit.

> Louise no es hija de esa pareja: sácala de la familia de Jean Martin y Rose Petit.

> Joseph Ngoma tuvo dos esposas, Marie Kanza y Thérèse Mbala: registra cada unión por separado, de tipo polígama.

## 7. Acontecimientos de la vida

Bautismo, entierro, emigración, servicio militar, censo, título, testamento… con las demás personas presentes y su papel (testigo, padrino o madrina, oficiante).

> Añade el bautismo de Jeanne Dupont el 10 de marzo de 1923 en Lyon, con Paul Martin como padrino.

> Jean Martin emigró a Canadá en 1956.

> Registra el servicio militar de Pierre Dupont de 1914 a 1918.

> El censo de 1936 muestra a la familia Dupont en Lyon: añádelo para Pierre y Marie.

## 8. Trabajar a partir de un documento

Adjunte la foto de un acta, un libro de familia en PDF o una carta. El AFG lee el documento, dice lo que encuentra y luego propone los cambios. Relea la tarjeta antes de aplicar: un documento antiguo se lee mal.

> Aquí está el acta de matrimonio de mis bisabuelos. Dime lo que lees y luego propón lo que hay que añadir al árbol.

> A partir de este libro de familia, añade los hijos que faltan y sus fechas.

> Compara esta carta con la ficha de Joseph Ngoma: ¿qué no concuerda?

## 9. Grandes trabajos, paso a paso

Una tarjeta puede contener hasta sesenta operaciones. Para un trabajo grande, avance por lotes: una rama, una generación o un documento cada vez.

> Completa los lugares de nacimiento de toda la descendencia de Pierre Dupont, una generación cada vez, empezando por sus hijos.

> Propón primero un plan de trabajo para poner en orden la rama Martin, sin modificar nada.

## 10. Eliminar y reparar

Un colaborador puede eliminar un acontecimiento, sacar a un hijo de una familia o deshacer una unión. Eliminar a una persona está reservado a los administradores: se retira de sus familias y acontecimientos, y luego se elimina su ficha. Para volver atrás, pida la operación inversa; el historial lo conserva todo.

> Elimina el censo de 1936 duplicado en la ficha de Pierre Dupont.

> La ficha «Jean Dupond», sin fechas ni familia, se creó por error: elimínala.

> Apliqué por error el matrimonio de Pierre Dupont y Marie Durand: deshaz esa unión y elimina el acontecimiento del matrimonio.

## 11. Desde ChatGPT, Claude u otra IA (MCP)

Las mismas operaciones están abiertas a las IA externas mediante el MCP, si un administrador lo ha abierto (Administración → AFG → MCP).

1. En la página **Mi cuenta**, sección «Tokens de acceso para una IA», cree un token; marque «Puede modificar el archivo, como esta cuenta» si la IA debe escribir. Cópielo: solo se muestra una vez.
2. Conecte su IA:
   - **Claude Desktop**: instale el conector comunitario `ax-genealogy.mcpb` (carpeta `mcp/ax-genealogy` del repositorio `axgf-cms`) y dele la dirección del sitio y el token.
   - **Claude Code**: `claude mcp add --transport http ax-genealogy https://su-sitio/mcp --header "Authorization: Bearer <token>"`
   - **ChatGPT** (conectores, modo desarrollador) o **`claude.ai`** (conector personalizado): dé la dirección con el token dentro, `https://su-sitio/mcp/t/<token>`, sin autenticación.
   - **Cursor** y los demás: la dirección `https://su-sitio/mcp` y la cabecera `Authorization: Bearer <token>`.
3. Revoque un token en cuanto deje de usarse, o si ha podido filtrarse.

> Con la herramienta ax-genealogy, busca a las personas llamadas Dupont y dime quién no tiene fecha de fallecimiento.

> Con ax-genealogy, añade a Jeanne Dupont, nacida en 1923 en Lyon, hija de Pierre Dupont y Marie Durand. Compruébalo antes con `check_operations`.

## 12. Buenas prácticas y límites

- **Una fuente para cada hecho.** Diga de dónde viene la información (acta, recuerdo, documento); el AFG puede añadirla a la biografía.
- **La duda se dice.** Prefiera «hacia 1890» a una fecha inventada, y pida al AFG que señale lo incierto.
- **Los vivos están protegidos.** Una persona viva no es pública por defecto. Piénselo antes de abrir una ficha al público.
- **Lo que el AFG aún no hace**: adjuntar fotos o documentos a las fichas, fusionar dos fichas que describen a la misma persona, escribir notas. Eso sigue en las páginas de administración, donde la fusión conserva todo lo que llevan ambas fichas.

> Antes de proponer nada, enumera lo que te parece incierto en lo que acabo de decirte.
